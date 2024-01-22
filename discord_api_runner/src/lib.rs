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

fn handle_message(our: &Address, state: &mut State) -> anyhow::Result<()> {
    let message = await_message()?;

    match message {
        Message::Request {
            ref source,
            ref body,
            ..
        } => {
            if let Ok(api_req) = serde_json::from_slice::<DiscordApiRequest>(&body) {
                // Handle requests with body of type DiscordApiRequest
                handle_api_request(our, source, api_req, state)?;
            } else if let Ok(ws_message) = serde_json::from_slice::<HttpClientRequest>(&body) {
                // Handle incoming WebSocket messages from http_client
                // These will be Gateway events or a websocket close
                handle_websocket_client_message(our, ws_message, state)?;
            } else {
                print_to_terminal(1, &format!("discord_api: unknown request: {:?}", message));
            }
        }
        Message::Response { context, .. } => {
            // Handle timer responses with a context of type Heartbeat
            // Used to maintain the Discord Gateway API connection
            maintain_heartbeat(our, context, state)?;
        }
    }
    Ok(())
}

fn handle_api_request(
    our: &Address,
    source: &Address,
    api_req: DiscordApiRequest,
    state: &mut State,
) -> anyhow::Result<()> {
    match api_req {
        // Connect a bot to the Discord Gateway API
        // Comes from the parent process
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
        // Disconnect a bot from the Discord Gateway API
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
        // Send a Gateway event to the Discord Gateway API
        DiscordApiRequest::Gateway { .. } => {
            // Send a gateway event as a Gateway request via websocket through http_client
        }
        // Send an http request to the Discord HTTP API
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

    Ok(())
}

fn handle_websocket_client_message(
    our: &Address,
    ws_message: HttpClientRequest,
    state: &mut State,
) -> anyhow::Result<()> {
    match ws_message {
        // Handle an incoming message from Discord Gateway API (via http_client)
        HttpClientRequest::WebSocketPush { channel_id, .. } => {
            let Some(blob) = get_blob() else {
                // print_to_terminal(0, "discord_api: ws push: no blob");
                return Ok(());
            };

            let Some(bot_id) = state.channels.get(&channel_id) else {
                // print_to_terminal(0, "discord_api: ws push: no bot_id");
                return Ok(());
            };

            let Some(bot) = state.bots.get_mut(&bot_id) else {
                // print_to_terminal(0, "discord_api: ws push: no bot");
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
                Err(_e) => {
                    // print_to_terminal(
                    //     0,
                    //     &format!("discord_api: ws push: unable to parse blob: {:?}", e),
                    // );
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

    Ok(())
}

// Connect to the Discord Gateway API
// Sent when a bot is connected with a DiscordApiRequest::Connect
fn connect_gateway(our: &Address, ws_client_channel: &u32) -> anyhow::Result<()> {
    match open_ws_connection_and_await(
        our.node.clone(),
        DISCORD_GATEWAY.to_string(),
        None,
        *ws_client_channel,
    ) {
        Ok(result) => match result {
            Ok(_) => {
                // print_to_terminal(0, "discord_api: ws: connected");
            }
            Err(_) => {
                // print_to_terminal(0, "discord_api: ws: error connecting");
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
            // print_to_terminal(0, &format!("discord_api: OTHER EVENT: {:?}", event));
            // Pass all the others to the parent process
            Request::new()
                .target(bot.parent.clone())
                .body(serde_json::json!(event).to_string().into_bytes())
                .send()?;
        }
    }

    Ok(())
}

fn maintain_heartbeat(our: &Address, context: Option<Vec<u8>>, state: &mut State) -> anyhow::Result<()> {
    let Some(context) = context else {
        return Ok(()); // No context
    };

    let heartbeat = serde_json::from_slice::<Heartbeat>(&context)?;

    let Some(bot) = state.bots.get(&heartbeat.bot) else {
        return Ok(()); // Bot does not exist
    };

    if bot.gateway_connection_open {
        send_ws_client_push(
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
        )?;

        discord_heartbeat_tick(bot.heartbeat_interval, heartbeat.bot);
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
