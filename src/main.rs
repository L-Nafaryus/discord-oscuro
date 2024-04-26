use oscuro_core::{client, config::Config, AppState};
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt()
        .with_target(false)
        .compact()
        .init();

    let mut config = match Config::open(Config::data_dir()?.join("config.toml").as_path()) {
        Ok(config) => config,
        Err(_) => Config::default(),
    };

    if let Ok(token) = env::var("DISCORD_TOKEN") {
        config.discord_token = token;
    };

    if config.discord_token.is_empty() {
        tracing::error!("Missing discord token");
    }

    let state = AppState { config };

    client(state)
        .await
        .expect("Failed to create client")
        .start()
        .await
        .expect("Failed to start client");

    Ok(())
}
