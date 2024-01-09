use uqbar_process_lib::*;

mod types;
use types::{ConnectGateway, GatewayReceiveEvent};

wit_bindgen::generate!({
    path: "wit",
    world: "process",
    exports: {
        world: Component,
    },
});

fn handle_message (our: &Address) -> anyhow::Result<()> {
    let message = await_message().unwrap();

    match message {
        Message::Response { context, .. } => {
            print_to_terminal(0, &format!("bot_testing: Response"));

            // Handle responses to HTTP requests
        },
        Message::Request { ref source, ref ipc, .. } => {
            print_to_terminal(0, &format!("bot_testing: Request"));

            // Handle Discord API events
            if let Ok(event) = serde_json::from_slice::<GatewayReceiveEvent>(&ipc) {
                print_to_terminal(0, &format!("bot_testing: Discord API Event: {:?}", event));
            }
        },
    }
    Ok(())
}

struct Component;
impl Guest for Component {
    fn init(our: String) {
        print_to_terminal(0, "bot_testing: begin");

        let our = Address::from_str(&our).unwrap();
        let bot_token = "MTE5MjEzNDg0ODY5ODEzMDU1Mw.G4xfrT.go7Sxk0zzyreJx1OrOiHgVnULCHofivI23eeCE".to_string();
        let intents = 512; // Guild messages

        // Spawn the API process
        match init_discord_api(&our, bot_token, 512).unwrap() {
            Ok(_) => {
                print_to_terminal(0, "bot_testing: discord_api spawned");
            },
            Err(e) => {
                print_to_terminal(0, format!(
                    "bot_testing: error: {:?}",
                    e,
                ).as_str());
            },
        }

        loop {
            match handle_message(&our) {
                Ok(()) => {},
                Err(e) => {
                    print_to_terminal(0, format!(
                        "bot_testing: error: {:?}",
                        e,
                    ).as_str());
                },
            };
        }
    }
}

fn init_discord_api(our: &Address, bot_token: String, intents: u128) -> Result<Result<uqbar_process_lib::Message, uqbar_process_lib::SendError>, anyhow::Error> {
    let new_process_id: u64 = rand::random();

    let Ok(discord_api_process_id) = spawn(
        Some(new_process_id.to_string().as_str()),
        &format!("{}/pkg/discord_api.wasm", our.package_id()),
        OnExit::Restart,
        &Capabilities::All,
        true, // not public
    ) else {
        return Err(anyhow::anyhow!("failed to spawn discord_api!"));
    };

    print_to_terminal(0, &format!("bot_testing: discord_api_process_id: {}", discord_api_process_id));

    Request::new()
        .target((our.node.as_ref(), discord_api_process_id))
        .ipc(
            serde_json::to_vec(&ConnectGateway {
                bot_token,
                intents,
            })
            .unwrap(),
        )
        .send_and_await_response(10)
}
