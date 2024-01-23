use discord_api::{load_state, handle_message};
use kinode_process_lib::{print_to_terminal, Address};
use std::str::FromStr;

wit_bindgen::generate!({
    path: "wit",
    world: "process",
    exports: {
        world: Component,
    },
});

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
