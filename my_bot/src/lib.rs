use discord_api::{
    ApplicationCommandOption, ApplicationCommandOptionType, ApplicationCommandType, BotId,
    CommandsCall, DiscordApiRequest, GatewayReceiveEvent, HttpApiCall, InteractionCallbackData,
    InteractionData, InteractionsCall, NewApplicationCommand,
};
use kinode_process_lib::{
    await_message, call_init, get_blob,
    http::{HttpClientAction, OutgoingHttpRequest},
    our_capabilities, println, spawn, Address, Message, OnExit, ProcessId, Request, SendError,
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
const COINMARKETCAP_API_KEY: &str = include_str!("../.coinmarketcap_api_key");
const COINMARKETCAP_URL: &str = "https://pro-api.coinmarketcap.com/v1/cryptocurrency/quotes/latest";

call_init!(init);

fn init(our: Address) {
    // let intents = 520; // 512 Read + 8 Admin
    let intents = 8704; // 512 Read + 8192 Manage Messages
    let bot = BotId::new(BOT_TOKEN.to_string(), intents);

    // Spawn the API process
    let (discord_api_id, result) = match init_discord_api(&our, &bot) {
        Ok((id, result)) => (id, result),
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
            name: "price".to_string(),
            description: Some("Check the price of a token".to_string()),
            command_type: Some(ApplicationCommandType::ChatInput.as_u8()),
            options: Some(vec![ApplicationCommandOption {
                option_type: ApplicationCommandOptionType::String.as_u8(),
                name: "token".to_string(),
                description: "The token to check the price of".to_string(),
                name_localizations: None,
                description_localizations: None,
                required: Some(true),
            }]),
        },
    });

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
                    // Handle the /price command we registered
                    "price" => {
                        let _ = respond_with_price(
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

fn respond_with_price(
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
            // Get the token symbol
            let symbol = option
                .value
                .as_str()
                .unwrap_or_default()
                .to_string()
                .to_uppercase();

            // Get the price from the CoinMarketCap API
            Request::new()
                .target((
                    our.node.as_ref(),
                    ProcessId::new(Some("http_client"), "distro", "sys"),
                ))
                .body(serde_json::to_vec(&HttpClientAction::Http(
                    OutgoingHttpRequest {
                        method: "GET".to_string(),
                        url: format!("{COINMARKETCAP_URL}?symbol={symbol}&convert=USD"),
                        headers: std::collections::HashMap::from([
                            ("Accepts".to_string(), "application/json".to_string()),
                            (
                                "X-CMC_PRO_API_KEY".to_string(),
                                COINMARKETCAP_API_KEY.to_string(),
                            ),
                        ]),
                        version: None,
                    },
                ))?)
                .send_and_await_response(3)??;

            // Get the blob from the response, parse and generate the response content
            match get_blob() {
                Some(response_data) => {
                    let price_data =
                        serde_json::from_slice::<serde_json::Value>(&response_data.bytes)?;
                    match price_data["data"][&symbol]["quote"]["USD"]["price"].as_f64() {
                        Some(price) => format!("The current price of {symbol} is ${price}"),
                        None => "Cryptocurrency not found or API response format has changed."
                            .to_string(),
                    }
                }
                None => "Unable to get token price.".to_string(),
            }
        }
        None => "No symbol given.".to_string(),
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
) -> Result<(ProcessId, Result<Message, SendError>), anyhow::Error> {
    let Ok(discord_api_process_id) = spawn(
        None,
        &format!("{}/pkg/discord_api_runner.wasm", our.package_id()),
        OnExit::None,
        our_capabilities(), // give the bot all our capabilities
        vec![
            ProcessId::new(Some("http_client"), "distro", "sys"),
            ProcessId::new(Some("timer"), "distro", "sys"),
        ],
        false, // not public
    ) else {
        return Err(anyhow::anyhow!("failed to spawn discord_api!"));
    };

    Ok((
        discord_api_process_id.clone(),
        Request::new()
            .target((our.node.as_ref(), discord_api_process_id))
            .body(serde_json::to_vec(&DiscordApiRequest::Connect(
                bot_id.clone(),
            ))?)
            .send_and_await_response(5)?,
    ))
}
