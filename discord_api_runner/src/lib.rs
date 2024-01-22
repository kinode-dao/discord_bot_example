use discord_api::{
    parse_gateway_blob, Bot, BotId, Bots, DiscordApiRequest, GatewayIdentifyProperties,
    GatewayReceiveEvent, GatewaySendEvent, WsChannels, DISCORD_GATEWAY,
};
use kinode_process_lib::{
    await_message, get_blob, get_state,
    http::{
        close_ws_connection, open_ws_connection_and_await, send_ws_client_push, HttpClientAction,
        HttpClientRequest, OutgoingHttpRequest, WsMessageType,
    },
    print_to_terminal,
    timer::set_timer,
    Address, LazyLoadBlob, Message, Request, Response,
};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, str::FromStr};

wit_bindgen::generate!({
    path: "wit",
    world: "process",
    exports: {
        world: Component,
    },
});

#[derive(Serialize, Deserialize, Debug)]
struct State {
    bots: Bots,
    channels: WsChannels,
}

#[derive(Serialize, Deserialize, Debug)]
struct Heartbeat {
    bot: BotId,
}

fn connect_gateway(our: &Address, ws_client_channel: &u32) -> anyhow::Result<()> {
    match open_ws_connection_and_await(
        our.node.clone(),
        DISCORD_GATEWAY.to_string(),
        None,
        *ws_client_channel,
    ) {
        Ok(result) => match result {
            Ok(_) => {
                print_to_terminal(1, "discord_api: ws: connected");
            }
            Err(_) => {
                print_to_terminal(1, "discord_api: ws: error connecting");
            }
        },
        Err(_) => {}
    }

    Response::new().body(vec![]).send()?;

    Ok(())
}

fn handle_gateway_event(
    our: &Address,
    event: GatewayReceiveEvent,
    bot: &mut Bot,
) -> anyhow::Result<()> {
    // Handle all events that have to do with the gateway connection
    // Forward all other events to the parent process
    match event {
        GatewayReceiveEvent::Hello(hello) => {
            let send_event = GatewaySendEvent::Identify {
                token: bot.token.clone(),
                intents: bot.intents.clone(),
                properties: GatewayIdentifyProperties {
                    os: "kinode".to_string(),
                    browser: "kinode".to_string(),
                    device: "kinode".to_string(),
                },
                compress: None,
                large_threshold: None,
                shard: None,
                presence: None,
                guild_subscriptions: None,
            };

            bot.heartbeat_interval = hello.heartbeat_interval;
            discord_heartbeat_tick(
                bot.heartbeat_interval - 1000,
                BotId::new(bot.token.clone(), bot.intents),
            );

            send_ws_client_push(
                our.node.clone(),
                bot.ws_client_channel,
                WsMessageType::Text,
                LazyLoadBlob {
                    mime: None,
                    bytes: GatewaySendEvent::Heartbeat { seq: None }.to_json_bytes(),
                },
            )?;

            send_ws_client_push(
                our.node.clone(),
                bot.ws_client_channel,
                WsMessageType::Text,
                LazyLoadBlob {
                    mime: None,
                    bytes: send_event.to_json_bytes(),
                },
            )?;
        }
        GatewayReceiveEvent::Ready(ready) => {
            print_to_terminal(1, &format!("discord_api: READY {:?}", ready));
            bot.session_id = ready.session_id.clone();
            bot.gateway_connection_open = true;

            Request::new()
                .target(bot.parent.clone())
                .body(serde_json::json!(ready).to_string().into_bytes())
                .send()?;

            // set_state(&serde_json::to_vec(&load_state())?);
        }
        GatewayReceiveEvent::Reconnect => {
            print_to_terminal(1, &format!("discord_api: RECONNECT"));
            let send_event = GatewaySendEvent::Resume {
                token: bot.token.clone(),
                session_id: bot.session_id.clone(),
                seq: bot.heartbeat_sequence,
            };

            send_ws_client_push(
                our.node.clone(),
                bot.ws_client_channel,
                WsMessageType::Text,
                LazyLoadBlob {
                    mime: None,
                    bytes: send_event.to_json_bytes(),
                },
            )?;
        }
        GatewayReceiveEvent::InvalidSession(resumable) => {
            // print_to_terminal(
            //     0,
            //     &format!("discord_api: INVALID SESSION, resumable: {:?}", resumable),
            // );

            if resumable {
                let send_event = GatewaySendEvent::Resume {
                    token: bot.token.clone(),
                    session_id: bot.session_id.clone(),
                    seq: bot.heartbeat_sequence,
                };

                send_ws_client_push(
                    our.node.clone(),
                    bot.ws_client_channel,
                    WsMessageType::Text,
                    LazyLoadBlob {
                        mime: None,
                        bytes: send_event.to_json_bytes(),
                    },
                )?;
            }
        }
        _ => {
            print_to_terminal(1, &format!("discord_api: OTHER EVENT: {:?}", event));
            // Pass all the others to the parent process
            Request::new()
                .target(bot.parent.clone())
                .body(serde_json::json!(event).to_string().into_bytes())
                .send()?;
        }
    }

    Ok(())
}

fn handle_message(our: &Address, state: &mut State) -> anyhow::Result<()> {
    let message = await_message()?;

    match message {
        Message::Response { context, .. } => {
            if let Some(context) = context {
                if let Ok(context) = serde_json::from_slice::<Heartbeat>(&context) {
                    if let Some(bot) = state.bots.get(&context.bot) {
                        if bot.gateway_connection_open {
                            match send_ws_client_push(
                                our.node.clone(),
                                bot.ws_client_channel,
                                WsMessageType::Text,
                                LazyLoadBlob {
                                    mime: None,
                                    bytes: GatewaySendEvent::Heartbeat {
                                        seq: Some(bot.heartbeat_sequence),
                                    }
                                    .to_json_bytes(),
                                },
                            ) {
                                Ok(_) => {
                                    discord_heartbeat_tick(bot.heartbeat_interval, context.bot)
                                }
                                Err(_e) => {
                                    // print_to_terminal(
                                    //     0,
                                    //     &format!("discord_api: error sending heartbeat: {:?}", e),
                                    // );
                                }
                            }

                            return Ok(());
                        }
                    }
                }
            }
        }
        Message::Request {
            ref source,
            ref body,
            ..
        } => {
            print_to_terminal(2, &format!("request: {:?}", String::from_utf8_lossy(body)));
            // Handle requests with body of type DiscordApiRequest
            if let Ok(api_req) = serde_json::from_slice::<DiscordApiRequest>(&body) {
                // print_to_terminal(0, &format!("discord_api: Request: {:?}", api_req));
                match api_req {
                    DiscordApiRequest::Connect(bot_id) => {
                        let ws_client_channel = state.bots.len() as u32;
                        let bot = Bot {
                            parent: source.clone(),
                            token: bot_id.token.clone(),
                            intents: bot_id.intents.clone(),
                            gateway_connection_open: false,
                            heartbeat_interval: 0,
                            heartbeat_sequence: 0,
                            session_id: "".to_string(),
                            ws_client_channel,
                        };

                        state
                            .bots
                            .insert(BotId::new(bot.token.clone(), bot.intents), bot);
                        state.channels.insert(ws_client_channel, bot_id);

                        // set_state(&serde_json::to_vec(state)?);
                        connect_gateway(our, &ws_client_channel)?;
                    }
                    DiscordApiRequest::Disconnect(bot_id) => {
                        if let Some(bot) = state.bots.get_mut(&bot_id) {
                            bot.gateway_connection_open = false;
                            bot.heartbeat_interval = 0;
                            bot.heartbeat_sequence = 0;
                            bot.session_id = "".to_string();

                            // Send a close message to http_client
                            close_ws_connection(our.node.clone(), bot.ws_client_channel)?;

                            state.bots.remove(&bot_id);
                            // set_state(&serde_json::to_vec(state)?);
                        }
                    }
                    DiscordApiRequest::Gateway { .. } => {
                        // Send a gateway event as a Gateway request via websocket through http_client
                    }
                    DiscordApiRequest::Http { bot, call } => {
                        // Send an http request to http_client
                        let (url, method, http_body) = call.to_request();
                        let mut headers = HashMap::new();
                        headers.insert("Authorization".to_string(), format!("Bot {}", bot.token));
                        headers.insert("Content-Type".to_string(), "application/json".to_string());
                        headers.insert(
                            "User-Agent".to_string(),
                            format!("DiscordBot ({}, {})", "https://kinode.network", "1.0"),
                        );

                        let http_req = OutgoingHttpRequest {
                            method: method.to_string(),
                            version: None,
                            url: url.to_string(),
                            headers,
                        };

                        let _ = Request::new()
                            .target(("our", "http_client", "distro", "sys"))
                            .inherit(true) // Send response to the process that requested
                            .body(serde_json::to_vec(&HttpClientAction::Http(http_req))?)
                            .blob_bytes(http_body)
                            .send()?;
                    }
                }

                return Ok(());
            }

            // Handle incoming WebSocket messages from http_client
            if let Ok(ws_message) = serde_json::from_slice::<HttpClientRequest>(&body) {
                match ws_message {
                    // Handle an incoming message from Discord Gateway API (via http_client)
                    HttpClientRequest::WebSocketPush { channel_id, .. } => {
                        let Some(blob) = get_blob() else {
                            print_to_terminal(1, "discord_api: ws push: no blob");
                            return Ok(());
                        };

                        let Some(bot_id) = state.channels.get(&channel_id) else {
                            print_to_terminal(1, "discord_api: ws push: no bot_id");
                            return Ok(());
                        };

                        let Some(bot) = state.bots.get_mut(&bot_id) else {
                            print_to_terminal(1, "discord_api: ws push: no bot");
                            return Ok(());
                        };

                        // Handle Gateway events
                        match parse_gateway_blob(&blob.bytes) {
                            Ok((event, seq)) => {
                                if let Some(seq) = seq {
                                    bot.heartbeat_sequence = seq;
                                }

                                handle_gateway_event(our, event, bot)?;
                                // set_state(&serde_json::to_vec(state)?);
                            }
                            Err(e) => {
                                print_to_terminal(
                                    1,
                                    &format!("discord_api: ws push: unable to parse blob: {:?}", e),
                                );
                            }
                        }
                    }
                    HttpClientRequest::WebSocketClose { channel_id } => {
                        print_to_terminal(0, "discord_api: ws close");
                        let Some(bot_id) = state.channels.get(&channel_id) else {
                            // print_to_terminal(0, "discord_api: ws push: no bot_id");
                            return Ok(());
                        };

                        let Some(bot) = state.bots.get_mut(&bot_id) else {
                            // print_to_terminal(0, "discord_api: ws push: no bot");
                            return Ok(());
                        };

                        // Reopen connection if closed, also clear current timers and set_state again
                        bot.gateway_connection_open = false;
                        bot.heartbeat_interval = 0;
                        bot.heartbeat_sequence = 0;
                        bot.session_id = "".to_string();

                        connect_gateway(our, &bot.ws_client_channel)?;

                        // set_state(&serde_json::to_vec(state)?);
                    }
                }
            } else {
                // print_to_terminal(0, &format!("discord_api: Request: {:?}", message));
            }
        }
    }
    Ok(())
}

fn discord_heartbeat_tick(interval: u64, bot: BotId) {
    set_timer(
        interval,
        Some(serde_json::to_vec(&Heartbeat { bot }).unwrap()),
    );
}

fn load_state() -> State {
    match get_state() {
        Some(state) => match serde_json::from_slice::<State>(&state) {
            Ok(state) => state,
            Err(_) => State {
                bots: HashMap::new(),
                channels: HashMap::new(),
            },
        },
        None => State {
            bots: HashMap::new(),
            channels: HashMap::new(),
        },
    }
}

struct Component;
impl Guest for Component {
    fn init(our: String) {
        print_to_terminal(0, "discord_api: begin");

        let our = Address::from_str(&our).unwrap();
        let mut state = load_state();

        loop {
            match handle_message(&our, &mut state) {
                Ok(()) => {}
                Err(e) => {
                    print_to_terminal(0, format!("discord_api: error: {:?}", e,).as_str());
                }
            };
        }
    }
}
