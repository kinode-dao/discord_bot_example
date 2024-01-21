use discord_api::{
    ApplicationCommandOption, ApplicationCommandOptionType, ApplicationCommandType, BotId,
    CommandsCall, DiscordApiRequest, GatewayReceiveEvent, HttpApiCall, InteractionCallbackData,
    InteractionData, InteractionsCall, NewApplicationCommand,
};
use kinode_process_lib::{
    await_message, call_init, println, Address, Message, ProcessId, Request, SendError,
};

wit_bindgen::generate!({
    path: "wit",
    world: "process",
    exports: {
        world: Component,
    },
});

const BOT_APPLICATION_ID: &str = include_str!("../.bot_application_id");
const BOT_TOKEN: &str = include_str!("../.bot_token");
call_init!(init);

fn init(our: Address) {
    let intents = 11264; // read, send, and manage messages
    let bot = BotId::new(BOT_TOKEN.to_string(), intents);

    // Spawn the API process
    let result = match init_discord_api(&our, &bot) {
        Ok(result) => result,
        Err(e) => {
            println!("my_bot: error initiating bot: {e:?}");
            panic!();
        }
    };

    if let Err(e) = result {
        println!("my_bot: error initiating bot: {e:?}");
        panic!();
    }

    println!("{our}: discord_api spawned");

    //
    // Register all the commands the bot will handle
    //
    let command = HttpApiCall::Commands(CommandsCall::CreateApplicationCommand {
        application_id: BOT_APPLICATION_ID.to_string(),
        command: NewApplicationCommand {
            name: "pki".to_string(),
            description: Some("Check the PKI entry for a node ID".to_string()),
            command_type: Some(ApplicationCommandType::ChatInput.as_u8()),
            options: Some(vec![ApplicationCommandOption {
                option_type: ApplicationCommandOptionType::String.as_u8(),
                name: "name".to_string(),
                description: "The node ID".to_string(),
                name_localizations: None,
                description_localizations: None,
                required: Some(true),
            }]),
        },
    });

    let discord_api_id = ProcessId::new(Some("discord_api_runner"), our.package(), our.publisher());

    Request::new()
        .target((our.node.as_ref(), discord_api_id.clone()))
        .body(
            serde_json::to_vec(&DiscordApiRequest::Http {
                bot: bot.clone(),
                call: command,
            })
            .unwrap(),
        )
        .expects_response(5)
        .send()
        .expect("failed to trigger child process");

    loop {
        match handle_message(&our, &discord_api_id, &bot) {
            Ok(()) => {}
            Err(e) => {
                println!("my_bot: error: {e:?}");
            }
        };
    }
}

fn handle_message(our: &Address, discord_api_id: &ProcessId, bot: &BotId) -> anyhow::Result<()> {
    // We currently don't do anything with Responses.
    // If we did, we could match on await_message() and handle the Response type.
    if let Message::Request { ref body, .. } = await_message()? {
        // Handle Discord API events
        // Can handle any of their abundant events here, depending on your bot's perms...
        let Ok(event) = serde_json::from_slice::<GatewayReceiveEvent>(&body) else {
            return Ok(())
        };
        match event {
            GatewayReceiveEvent::InteractionCreate(interaction) => {
                // Handle interactions with bot commands
                let Some(data) = interaction.data else {
                    return Ok(())
                };
                match data.name.as_str() {
                    // Handle the /pki command we registered
                    "pki" => {
                        let _ = respond_with_pki_entry(
                            &our,
                            &bot,
                            &discord_api_id,
                            interaction.id,
                            interaction.token,
                            data,
                        );
                    }
                    _ => {}
                }
            }
            _ => {}
        }
    }
    Ok(())
}

fn respond_with_pki_entry(
    our: &Address,
    bot: &BotId,
    discord_api_id: &ProcessId,
    interaction_id: String,
    interaction_token: String,
    data: InteractionData,
) -> anyhow::Result<()> {
    let options = data.options.unwrap_or(vec![]);

    let content: String = match options.first() {
        Some(option) => {
            // Get the node name
            let node_id = option.value.as_str().unwrap_or_default().to_string();

            // Get the PKI data from `net:distro:sys`
            let Message::Response { body, .. } =
                Request::to((our.node(), ("net", "distro", "sys")))
                    .body(node_id.as_bytes())
                    .send_and_await_response(5)?? else {
                        return Ok(())
                    };

            String::from_utf8(body).unwrap_or_default()
        }
        None => "No name given.".to_string(),
    };

    println!("{our}: responding to command with: {}", content);

    // Generate the response using the Discord API
    let call = HttpApiCall::Interactions(InteractionsCall::CreateInteractionResponse {
        interaction_id,
        interaction_token,
        interaction_type: 4, // ChannelMessageWithSource
        data: Some(InteractionCallbackData {
            tts: None,
            content: Some(content),
            embeds: None,
            allowed_mentions: None,
            flags: None,
            components: None,
            attachments: None,
        }),
    });

    // Send the response to the Discord API
    Request::new()
        .target((our.node.as_ref(), discord_api_id.clone()))
        .body(serde_json::to_vec(&DiscordApiRequest::Http {
            bot: bot.clone(),
            call,
        })?)
        .expects_response(5)
        .send()
}

fn init_discord_api(
    our: &Address,
    bot_id: &BotId,
) -> Result<Result<Message, SendError>, anyhow::Error> {
    Request::new()
        .target((
            our.node.as_ref(),
            ProcessId::new(Some("discord_api_runner"), our.package(), our.publisher()),
        ))
        .body(serde_json::to_vec(&DiscordApiRequest::Connect(
            bot_id.clone(),
        ))?)
        .send_and_await_response(5)
}
