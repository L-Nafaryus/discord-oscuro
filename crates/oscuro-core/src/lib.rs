pub mod commands;
pub mod config;
pub mod errors;

use errors::BoxedError;
use poise::serenity_prelude::{self as serenity, prelude::TypeMapKey, Client};

#[derive(Debug, Clone)]
pub struct AppState {
    pub config: config::Config,
}

impl TypeMapKey for AppState {
    type Value = AppState;
}

type Context<'a> = poise::Context<'a, AppState, BoxedError>;

pub async fn client(state: AppState) -> Result<Client, BoxedError> {
    let intents = serenity::GatewayIntents::non_privileged();
    let state_copy = state.clone();
    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![commands::register(), commands::age()],
            event_handler: |ctx, event, framework, data| {
                Box::pin(event_handler(ctx, event, framework, data))
            },
            ..Default::default()
        })
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(state_copy)
            })
        })
        .build();

    let client = serenity::ClientBuilder::new(state.clone().config.discord_token, intents)
        .framework(framework)
        .await?;

    {
        let mut data = client.data.write().await;
        data.insert::<AppState>(state);
    }

    Ok(client)
}

async fn event_handler(
    _ctx: &serenity::Context,
    event: &serenity::FullEvent,
    _framework: poise::FrameworkContext<'_, AppState, BoxedError>,
    _state: &AppState,
) -> Result<(), BoxedError> {
    match event {
        serenity::FullEvent::Ready { data_about_bot, .. } => {
            println!("Logged in as {}", data_about_bot.user.name);
        }
        _ => {}
    }
    Ok(())
}
