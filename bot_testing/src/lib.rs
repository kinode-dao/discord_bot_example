use discord_api::{BotId, DiscordApiRequest, GatewayReceiveEvent, HttpApiCall, MessagesCall};
use nectar_process_lib::{
    await_message, our_capabilities, print_to_terminal, spawn, Address, Message, OnExit, ProcessId,
    Request, SendError,
};
use std::str::FromStr;

wit_bindgen::generate!({
    path: "wit",
    world: "process",
    exports: {
        world: Component,
    },
});

fn handle_message(our: &Address, discord_api_id: &ProcessId, bot: &BotId) -> anyhow::Result<()> {
    let message = await_message()?;

    match message {
        Message::Response { body, .. } => {
            // Handle responses to Discord API HTTP requests here
            let response = String::from_utf8(body)?;
            print_to_terminal(0, &format!("bot_testing: Response {:?}", response));
        }
        Message::Request {
            ref source,
            ref body,
            ..
        } => {
            // Handle Discord API events
            print_to_terminal(0, &format!("bot_testing: Request"));

            if let Ok(event) = serde_json::from_slice::<GatewayReceiveEvent>(&body) {
                print_to_terminal(0, &format!("bot_testing: Discord API Event: {:?}", event));

                match event {
                    GatewayReceiveEvent::MessageCreate(message) => {
                        let Some(content) = message.content else {
                            return Ok(());
                        };

                        // Check if message.content includes a twitter link, if so delete the message and rewrite to vxtwitter with attribution
                        if content.contains("/twitter.com") {
                            let delete = HttpApiCall::Messages(MessagesCall::Delete {
                                channel_id: message.channel_id.clone(),
                                message_id: message.id,
                            });
                            let _ = Request::new()
                                .target((our.node.as_ref(), discord_api_id.clone()))
                                .body(serde_json::to_vec(&DiscordApiRequest::Http {
                                    bot: bot.clone(),
                                    call: delete,
                                })?)
                                .send_and_await_response(5)?;

                            let Some(author) = message.author else {
                                return Ok(());
                            };
                            let new_copy = content.replace("twitter.com", "vxtwitter.com");
                            let new_content =
                                format!("<@{}> shared a link:\n{}", author.id, new_copy);

                            let create = HttpApiCall::Messages(MessagesCall::Create {
                                channel_id: message.channel_id,
                                content: new_content,
                            });
                            Request::new()
                                .target((our.node.as_ref(), discord_api_id.clone()))
                                .body(serde_json::to_vec(&DiscordApiRequest::Http {
                                    bot: bot.clone(),
                                    call: create,
                                })?)
                                .expects_response(5)
                                .send()?;
                        }
                    }
                    _ => {}
                }
            }
        }
    }
    Ok(())
}

fn init_discord_api(
    our: &Address,
    bot: &BotId,
) -> Result<(ProcessId, Result<Message, SendError>), anyhow::Error> {
    let new_process_id: u64 = rand::random();

    let Ok(discord_api_process_id) = spawn(
        Some(new_process_id.to_string().as_str()),
        &format!("{}/pkg/discord_api.wasm", our.package_id()),
        OnExit::Restart,
        our_capabilities(),
        vec![
            ProcessId::new(Some("http_client"), "sys", "nectar"),
            ProcessId::new(Some("timer"), "sys", "nectar"),
        ],
        false, // not public
    ) else {
        return Err(anyhow::anyhow!("failed to spawn discord_api!"));
    };

    let result = Request::new()
        .target((our.node.as_ref(), discord_api_process_id.clone()))
        .body(serde_json::to_vec(&DiscordApiRequest::Connect(
            bot.clone(),
        ))?)
        .send_and_await_response(10)?;

    Ok((discord_api_process_id, result))
}

struct Component;
impl Guest for Component {
    fn init(our: String) {
        print_to_terminal(0, "bot_testing: begin");

        let our = Address::from_str(&our).unwrap();

        // ADD YOUR BOT TOKEN HERE (do not commit this token to git)
        let bot_token = "".to_string();
        // let intents = 520; // 512 Read + 8 Admin
        let intents = 8704; // 512 Read + 8192 Manage Messages

        let bot = BotId::new(bot_token, intents);
        // Spawn the API process
        let (discord_api_id, result) = init_discord_api(&our, &bot).unwrap();

        match result {
            Ok(_) => {
                print_to_terminal(0, "bot_testing: discord_api spawned");
            }
            Err(e) => {
                print_to_terminal(0, format!("bot_testing: error: {:?}", e,).as_str());
            }
        }

        loop {
            match handle_message(&our, &discord_api_id, &bot) {
                Ok(()) => {}
                Err(e) => {
                    print_to_terminal(0, format!("bot_testing: error: {:?}", e,).as_str());
                }
            };
        }
    }
}
