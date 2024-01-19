extern crate regex;
use discord_api::{
    ApplicationCommandOption, ApplicationCommandOptionType, ApplicationCommandType, BotId,
    CommandsCall, DiscordApiRequest, GatewayReceiveEvent, HttpApiCall, InteractionCallbackData,
    InteractionData, InteractionsCall, MessagesCall, NewApplicationCommand,
};
use kinode::process::standard::get_blob;
use kinode_process_lib::{
    await_message,
    http::{HttpClientAction, OutgoingHttpRequest},
    our_capabilities, print_to_terminal, spawn, Address, Message, OnExit, ProcessId, Request,
    SendError,
};
use regex::Regex;
use std::{collections::HashMap, str::FromStr};

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

fn handle_message(our: &Address, discord_api_id: &ProcessId, bot: &BotId) -> anyhow::Result<()> {
    let message = await_message()?;

    match message {
        Message::Response { body, .. } => {
            // Handle responses to Discord API HTTP requests here
            let response = String::from_utf8(body)?;
            print_to_terminal(0, &format!("my_bot: Response {:?}", response));

            match get_blob() {
                Some(blob) => {
                    print_to_terminal(
                        0,
                        &format!("my_bot: Blob {:?}", String::from_utf8(blob.bytes)?),
                    );
                }
                _ => {}
            };
        }
        Message::Request {
            ref source,
            ref body,
            ..
        } => {
            // Handle Discord API events
            print_to_terminal(0, &format!("my_bot: Request"));

            if let Ok(event) = serde_json::from_slice::<GatewayReceiveEvent>(&body) {
                print_to_terminal(0, &format!("my_bot: Discord API Event: {:?}", event));

                match event {
                    GatewayReceiveEvent::MessageCreate(message) => {
                        let Some(content) = message.content else {
                            return Ok(());
                        };

                        // Check if message.content includes a twitter link, if so delete the message and rewrite to vxtwitter with attribution
                        let re = Regex::new(r"https?://(twitter\.com|x\.com)/\S+").unwrap();
                        if re.is_match(&content) {
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
                            let new_copy = content
                                .replace("twitter.com", "vxtwitter.com")
                                .replace("x.com", "vxtwitter.com");
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
                    GatewayReceiveEvent::InteractionCreate(interaction) => {
                        // Handle interactions with bot commands
                        match interaction.data {
                            Some(data) => match data.name.as_str() {
                                // Handle the /price command
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
                            },
                            _ => {}
                        }
                    }
                    _ => {}
                }
            }
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

    let content = match options.first() {
        Some(option) => {
            // Get the token symbol
            let symbol = option
                .value
                .as_str()
                .unwrap_or("")
                .to_string()
                .to_uppercase();

            let mut headers = HashMap::new();
            headers.insert("Accepts".to_string(), "application/json".to_string());
            headers.insert(
                "X-CMC_PRO_API_KEY".to_string(),
                COINMARKETCAP_API_KEY.to_string(),
            );
            // Get the price from the CoinMarketCap API
            let _ = Request::new()
                .target((our.node.as_ref(), ProcessId::new(Some("http_client"), "distro", "sys")))
                .body(serde_json::to_vec(
                    &HttpClientAction::Http(OutgoingHttpRequest {
                        method: "GET".to_string(),
                        url: format!("https://pro-api.coinmarketcap.com/v1/cryptocurrency/quotes/latest?symbol={}&convert=USD", symbol),
                        headers,
                        version: None,
                    })
                )?)
                .send_and_await_response(3)?;

            // Get the blob, parse and generate the response content
            match get_blob() {
                Some(response_data) => {
                    let price_data =
                        serde_json::from_slice::<serde_json::Value>(&response_data.bytes)?;
                    match price_data["data"][&symbol]["quote"]["USD"]["price"].as_f64() {
                        Some(price) => Some(
                            format!("The current price of {} is ${}", symbol, price).to_string(),
                        ),
                        None => Some(
                            "Cryptocurrency not found or API response format has changed."
                                .to_string(),
                        ),
                    }
                }
                None => Some("Unable to get token price.".to_string()),
            }
        }
        None => Some("No symbol given.".to_string()),
    };

    // Generate the response using the Discord API
    let command = HttpApiCall::Interactions(InteractionsCall::CreateInteractionResponse {
        interaction_id,
        interaction_token,
        interaction_type: 4, // ChannelMessageWithSource
        data: Some(InteractionCallbackData {
            tts: None,
            content,
            embeds: None,
            allowed_mentions: None,
            flags: None,
            components: None,
            attachments: None,
        }),
    });

    // Send the response to the Discord API
    match serde_json::to_vec(&DiscordApiRequest::Http {
        bot: bot.clone(),
        call: command,
    }) {
        Ok(body) => {
            let _ = Request::new()
                .target((our.node.as_ref(), discord_api_id.clone()))
                .body(body)
                .expects_response(5)
                .send();
        }
        _ => {}
    };

    Ok(())
}

fn init_discord_api(
    our: &Address,
    bot: &BotId,
) -> Result<(ProcessId, Result<Message, SendError>), anyhow::Error> {
    let new_process_id: u64 = rand::random();

    let Ok(discord_api_process_id) = spawn(
        Some(new_process_id.to_string().as_str()),
        &format!("{}/pkg/discord_api_runner.wasm", our.package_id()),
        OnExit::Restart,
        our_capabilities(),
        vec![
            ProcessId::new(Some("http_client"), "distro", "sys"),
            ProcessId::new(Some("timer"), "distro", "sys"),
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
        print_to_terminal(0, "my_bot: begin");

        let our = Address::from_str(&our).unwrap();

        // ADD YOUR BOT TOKEN HERE (do not commit this token to git)
        let bot_token = BOT_TOKEN.to_string();
        // let intents = 520; // 512 Read + 8 Admin
        let intents = 8704; // 512 Read + 8192 Manage Messages

        let bot = BotId::new(bot_token, intents);
        // Spawn the API process
        let (discord_api_id, result) = match init_discord_api(&our, &bot) {
            Ok((id, result)) => (id, result),
            Err(e) => {
                print_to_terminal(
                    0,
                    format!("my_bot: error initiating bot: {:?}", e,).as_str(),
                );
                return;
            }
        };

        match result {
            Ok(_) => {
                print_to_terminal(0, "my_bot: discord_api spawned");

                // Register bot commands
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

                match serde_json::to_vec(&DiscordApiRequest::Http {
                    bot: bot.clone(),
                    call: command,
                }) {
                    Ok(body) => {
                        let _ = Request::new()
                            .target((our.node.as_ref(), discord_api_id.clone()))
                            .body(body)
                            .expects_response(5)
                            .send();
                    }
                    _ => {}
                }
            }
            Err(e) => {
                print_to_terminal(0, format!("my_bot: spawn error: {:?}", e,).as_str());
            }
        }

        loop {
            match handle_message(&our, &discord_api_id, &bot) {
                Ok(()) => {}
                Err(e) => {
                    print_to_terminal(0, format!("my_bot: error: {:?}", e,).as_str());
                }
            };
        }
    }
}
