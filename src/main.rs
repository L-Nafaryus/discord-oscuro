use oscuro_core::{client, config::Config, AppState};
use std::env;

#[tokio::main]
async fn main() {
    let token = env::var("DISCORD_TOKEN").expect("Variable 'DISCORD_TOKEN' must be set");
    let state = AppState {
        config: Config {
            discord_token: token,
        },
    };

    client(state)
        .await
        .expect("Failed to create client")
        .start()
        .await
        .expect("Failed to start client");
}
