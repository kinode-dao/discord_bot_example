use nectar_process_lib::{
    await_message, get_payload,
    http::{
        open_ws_connection_and_await, send_ws_client_push, WebSocketClientAction, WsMessageType,
    },
    print_to_terminal,
    timer::set_timer,
    Address, Message, Payload, Response, Request,
};

mod types;
use types::*;
mod http_api;
use http_api::*;
mod gateway_api;
use gateway_api::*;

wit_bindgen::generate!({
    path: "wit",
    world: "process",
    exports: {
        world: Component,
    },
});

fn connect_gateway(our: &Address, ws_client_channel: &u32) -> anyhow::Result<()> {
    match open_ws_connection_and_await(
        our.node.clone(),
        DISCORD_GATEWAY.to_string(),
        None,
        *ws_client_channel,
    ) {
        Ok(result) => match result {
            Ok(_) => {
                print_to_terminal(0, "discord_api: ws: connected");
            }
            Err(_) => {
                print_to_terminal(0, "discord_api: ws: error connecting");
            }
        },
        Err(_) => {}
    }

    Response::new().body(vec![]).send()?;

    Ok(())
}

fn handle_gateway_event(
    our: &Address,
    parent: &Option<Address>,
    gateway_connection_open: &mut bool,
    bot_token: &mut String,
    intents: &mut u128,
    ws_client_channel: &u32,
    heartbeat_interval: &mut u64,
    heartbeat_sequence: &mut u64,
    session_id: &mut String,
    event: GatewayReceiveEvent,
) -> anyhow::Result<()> {
    match event {
        GatewayReceiveEvent::Hello(hello) => {
            let send_event = GatewaySendEvent::Identify {
                token: bot_token.clone(),
                intents: intents.clone(),
                properties: GatewayIdentifyProperties {
                    os: "nectar".to_string(),
                    browser: "nectar".to_string(),
                    device: "nectar".to_string(),
                },
                compress: None,
                large_threshold: None,
                shard: None,
                presence: None,
                guild_subscriptions: None,
            };

            *heartbeat_interval = hello.heartbeat_interval;
            discord_heartbeat_tick(*heartbeat_interval - 1000);

            send_ws_client_push(
                our.node.clone(),
                *ws_client_channel,
                WsMessageType::Text,
                Payload {
                    mime: None,
                    bytes: GatewaySendEvent::Heartbeat {
                        seq: None,
                    }
                    .to_json_bytes(),
                },
            )?;

            send_ws_client_push(
                our.node.clone(),
                *ws_client_channel,
                WsMessageType::Text,
                Payload {
                    mime: None,
                    bytes: send_event.to_json_bytes(),
                },
            )?;
        }
        GatewayReceiveEvent::Ready(ready) => {
            print_to_terminal(0, &format!("discord_api: READY {:?}", ready));
            *session_id = ready.session_id.clone();
            *gateway_connection_open = true;
            if let Some(parent) = parent {
                Request::new()
                    .target(parent.clone())
                    .body(serde_json::json!(ready).to_string().into_bytes())
                    .send()?;
            }
        }
        GatewayReceiveEvent::Reconnect => {
            print_to_terminal(0, &format!("discord_api: RECONNECT"));
            let send_event = GatewaySendEvent::Resume {
                token: bot_token.clone(),
                session_id: session_id.clone(),
                seq: *heartbeat_sequence,
            };

            send_ws_client_push(
                our.node.clone(),
                *ws_client_channel,
                WsMessageType::Text,
                Payload {
                    mime: None,
                    bytes: send_event.to_json_bytes(),
                },
            )?;
        }
        GatewayReceiveEvent::InvalidSession(resumable) => {
            print_to_terminal(0, &format!("discord_api: INVALID SESSION, resumable: {:?}", resumable));

            if resumable {
                let send_event = GatewaySendEvent::Resume {
                    token: bot_token.clone(),
                    session_id: session_id.clone(),
                    seq: *heartbeat_sequence,
                };

                send_ws_client_push(
                    our.node.clone(),
                    *ws_client_channel,
                    WsMessageType::Text,
                    Payload {
                        mime: None,
                        bytes: send_event.to_json_bytes(),
                    },
                )?;
            }
        }
        _ => {
            print_to_terminal(0, &format!("discord_api: OTHER EVENT: {:?}", event));
            // Pass all the others to the parent process
            if let Some(parent) = parent {
                Request::new()
                    .target(parent.clone())
                    .body(serde_json::json!(event).to_string().into_bytes())
                    .send()?;
            }
        }
    }

    Ok(())
}

fn handle_message(
    our: &Address,
    parent: &mut Option<Address>,
    gateway_connection_open: &mut bool,
    bot_token: &mut String,
    intents: &mut u128,
    heartbeat_interval: &mut u64,
    heartbeat_sequence: &mut u64,
    ws_client_channel: &u32,
    session_id: &mut String,
) -> anyhow::Result<()> {
    let message = await_message().unwrap();

    match message {
        Message::Response { context, .. } => {
            print_to_terminal(0, &format!("discord_api: Response"));

            if let Some(context) = context {
                let Ok(context) = String::from_utf8(context.clone()) else {
                    print_to_terminal(0, "discord_api: Response: context is not a String");
                    return Ok(());
                };

                if context == "discord_heartbeat" && *gateway_connection_open {
                    match send_ws_client_push(
                        our.node.clone(),
                        *ws_client_channel,
                        WsMessageType::Text,
                        Payload {
                            mime: None,
                            bytes: GatewaySendEvent::Heartbeat {
                                seq: Some(*heartbeat_sequence),
                            }
                            .to_json_bytes(),
                        },
                    ) {
                        Ok(_) => discord_heartbeat_tick(*heartbeat_interval),
                        Err(e) => {
                            print_to_terminal(
                                0,
                                &format!("discord_api: error sending heartbeat: {:?}", e),
                            );
                        }
                    }

                    return Ok(());
                }
            }
        }
        Message::Request {
            ref source,
            ref body,
            ..
        } => {
            // Handle the initial request to connect to the gateway
            if let Ok(connect) = serde_json::from_slice::<ConnectGateway>(&body) {
                *bot_token = connect.bot_token.clone();
                *intents = connect.intents.clone();
                *parent = Some(source.clone());

                connect_gateway(our, ws_client_channel)?;

                return Ok(());
            }

            // Handle
            if let Ok(ws_message) = serde_json::from_slice::<WebSocketClientAction>(&body) {
                match ws_message {
                    // Handle an incoming message from Discord Gateway API (via http_client)
                    WebSocketClientAction::Push { .. } => {
                        let Some(payload) = get_payload() else {
                            print_to_terminal(0, "discord_api: ws push: no payload");
                            return Ok(());
                        };

                        // Handle Gateway events
                        match parse_gateway_payload(&payload.bytes, heartbeat_sequence) {
                            Ok(event) => {
                                handle_gateway_event(
                                    our,
                                    parent,
                                    gateway_connection_open,
                                    bot_token,
                                    intents,
                                    ws_client_channel,
                                    heartbeat_interval,
                                    heartbeat_sequence,
                                    session_id,
                                    event,
                                )?;
                            }
                            Err(e) => {
                                print_to_terminal(
                                    0,
                                    &format!(
                                        "discord_api: ws push: unable to parse payload: {:?}",
                                        e
                                    ),
                                );
                            }
                        }
                    }
                    WebSocketClientAction::Close { .. } => {
                        print_to_terminal(0, "discord_api: ws close");
                        *gateway_connection_open = false;
                        // TODO: reopen connection if closed, also clear current timers
                    }
                    WebSocketClientAction::Open { .. } => {} // This request type is only sent to http_client
                    WebSocketClientAction::Response { .. } => {} // This request type only comes back as a response
                }
            } else {
                print_to_terminal(0, &format!("discord_api: Request: {:?}", message));
            }
        }
    }
    Ok(())
}

struct Component;
impl Guest for Component {
    fn init(our: String) {
        print_to_terminal(0, "discord_api: begin");

        let our = Address::from_str(&our).unwrap();
        let mut parent: Option<Address> = None;
        let mut gateway_connection_open = false;
        let mut bot_token = String::new();
        let mut heartbeat_interval = 0;
        let mut heartbeat_sequence = 0;
        let mut intents: u128 = 74;
        let mut session_id = String::new();
        let ws_client_channel = 0;

        loop {
            match handle_message(
                &our,
                &mut parent,
                &mut gateway_connection_open,
                &mut bot_token,
                &mut intents,
                &mut heartbeat_interval,
                &mut heartbeat_sequence,
                &ws_client_channel,
                &mut session_id,
            ) {
                Ok(()) => {}
                Err(e) => {
                    print_to_terminal(0, format!("discord_api: error: {:?}", e,).as_str());
                }
            };
        }
    }
}

fn discord_heartbeat_tick(interval: u64) {
    set_timer(interval, Some("discord_heartbeat".to_string().into_bytes()));
}
