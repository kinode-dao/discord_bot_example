use std::str::FromStr;

use nectar_process_lib::{await_message, print_to_terminal, spawn, Message, Address, OnExit, ProcessId, Request, SendError, our_capabilities};

use discord_api::{GatewayReceiveEvent, HttpApiCall, MessagesCall, DiscordApiRequest, BotId};

wit_bindgen::generate!({
    path: "wit",
    world: "process",
    exports: {
        world: Component,
    },
});

fn handle_message(our: &Address, discord_api_id: &ProcessId, bot: &BotId) -> anyhow::Result<()> {
    let message = await_message().unwrap();

    match message {
        Message::Response { body, .. } => {
            let response = String::from_utf8(body).unwrap();
            print_to_terminal(0, &format!("bot_testing: Response {:?}", response));

            // Handle responses to HTTP requests
        }
        Message::Request {
            ref source,
            ref body,
            ..
        } => {
            print_to_terminal(0, &format!("bot_testing: Request"));

            // Handle Discord API events
            if let Ok(event) = serde_json::from_slice::<GatewayReceiveEvent>(&body) {
                print_to_terminal(0, &format!("bot_testing: Discord API Event: {:?}", event));

                match event {
                    GatewayReceiveEvent::MessageCreate(message) => {
                        print_to_terminal(0, &format!("2: {:?}", message.content.contains("/twitter.com")));
                        // Check if message.content includes a twitter link, if so delete the message and rewrite to vxtwitter
                        if message.content.contains("/twitter.com") {
                            print_to_terminal(0, &format!("3"));

                            let delete = HttpApiCall::Messages(MessagesCall::Delete { channel_id: message.channel_id.clone(), message_id: message.id });
                            let _ = Request::new()
                                .target((our.node.as_ref(), discord_api_id.clone()))
                                .body(serde_json::to_vec(& DiscordApiRequest::Http { bot: bot.clone(), call: delete }).unwrap())
                                .send_and_await_response(5)?;

                            print_to_terminal(0, &format!("4"));
                            let new_copy = message.content.replace("twitter.com", "vxtwitter.com");
                            let new_content = format!("<@{}> shared a link:\n{}", message.author.id, new_copy);

                            let create = HttpApiCall::Messages(MessagesCall::Create { channel_id: message.channel_id, content: new_content });
                            Request::new()
                                .target((our.node.as_ref(), discord_api_id.clone()))
                                .body(serde_json::to_vec(& DiscordApiRequest::Http { bot: bot.clone(), call: create }).unwrap())
                                .expects_response(5)
                                .send()?;
                            print_to_terminal(0, &format!("5"));
                        }
                    }
                    _ => {}
                }
            }
        }
    }
    Ok(())
}

struct Component;
impl Guest for Component {
    fn init(our: String) {
        print_to_terminal(0, "bot_testing: begin");

        let our = Address::from_str(&our).unwrap();

        let bot_token =
            "".to_string();
        // let intents = 9216;
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
        vec![ProcessId::new(Some("http_client"), "sys", "nectar"), ProcessId::new(Some("timer"), "sys", "nectar")],
        false, // not public
    ) else {
        return Err(anyhow::anyhow!("failed to spawn discord_api!"));
    };

    print_to_terminal(
        0,
        &format!(
            "bot_testing: discord_api_process_id: {}",
            discord_api_process_id
        ),
    );

    let result = Request::new()
        .target((our.node.as_ref(), discord_api_process_id.clone()))
        .body(serde_json::to_vec(&DiscordApiRequest::Connect(bot.clone())).unwrap())
        .send_and_await_response(10)?;

    Ok((discord_api_process_id, result))
}
