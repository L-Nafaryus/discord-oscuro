mod config;
mod discord;
mod errors;
mod telegram;

use config::Config;

use tokio::signal;
use tokio::task::JoinSet;

#[tokio::main]
async fn main() -> Result<(), errors::BoxedError> {
    tracing_subscriber::fmt()
        .with_target(false)
        .compact()
        .init();

    tracing::info!("Working directory: {:?}", Config::data_dir()?);

    let config = match Config::open(Config::data_dir()?.join("config.toml").as_path()) {
        Ok(config) => config,
        Err(err) => {
            tracing::debug!("{}", err);
            tracing::info!("Using default configuration");
            Config::default()
        }
    }
    .with_env();

    let mut runset = JoinSet::new();

    if !config.clone().discord_token.is_some() {
        tracing::warn!("Missing discord token");
    } else {
        let mut discord_client = discord::Client::new(config.clone())
            .await
            .expect("Failed to create discord client");

        runset.spawn(async move {

            let res = discord_client.start().await;
            if let Err(err) = res {
                tracing::error!("{}", err);
            }
        });
    }

    if !config.clone().telegram_token.is_some() {
        tracing::warn!("Missing telegram token");
    } else {
        let telegram_client = telegram::Client::new(config);

        runset.spawn(async move {
            let res = telegram_client.start().await;
            if let Err(err) = res {
                tracing::error!("{}", err);
            }
        });
    }

    while let Some(res) = runset.join_next().await {
        if let Err(err) = res {
            tracing::error!("{}", err);
        }
    }

    match signal::ctrl_c().await {
        Ok(()) => {}
        Err(err) => {
            eprintln!("Unable to listen for shutdown signal: {}", err);
            // we also shut down in case of error
        }
    }

    Ok(())
}
