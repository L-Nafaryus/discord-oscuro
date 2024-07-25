pub mod commands;

use crate::config::Config;
use crate::errors::BoxedError;
use std::sync::Arc;

use poise::serenity_prelude::{
    self as serenity,
    builder::{CreateEmbed, CreateMessage},
    model::id::ChannelId,
    prelude::TypeMapKey,
    //Client,
};
use serenity::GatewayIntents;

use teloxide::prelude::*;
use teloxide::types::Recipient;

use crate::telegram;

#[derive(Debug, Clone)]
pub struct BotState {
    pub config: Config,
    pub telegram_agent: Option<telegram::Client>,
}

impl TypeMapKey for BotState {
    type Value = BotState;
}

type Context<'a> = poise::Context<'a, BotState, BoxedError>;

pub struct Client {
    client: serenity::Client,
}

impl Client {
    pub async fn new(config: Config) -> Result<Self, BoxedError> {
        let telegram_agent = if config.clone().telegram_token.is_some() {
            Some(telegram::Client::new(config.clone()))
        } else {
            None
        };

        let state = BotState {
            config: config.clone(),
            telegram_agent: telegram_agent,
        };

        let intents = GatewayIntents::GUILDS
            | GatewayIntents::GUILD_MESSAGES
            | GatewayIntents::MESSAGE_CONTENT;
        let state_copy = state.clone();
        let framework = poise::Framework::builder()
            .options(poise::FrameworkOptions {
                commands: vec![
                    commands::register(),
                    commands::age(),
                    commands::dice(),
                    commands::status(),
                ],
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

        let client =
            serenity::ClientBuilder::new(state.clone().config.discord_token.unwrap(), intents)
                .framework(framework)
                .await?;

        {
            let mut data = client.data.write().await;
            data.insert::<BotState>(state);
        }

        Ok(Self { client })
    }

    pub async fn start(&mut self) -> Result<(), BoxedError> {
        self.client.start().await;
        Ok(())
    }

    pub async fn send(&self, chat_id: u64, msg: String) -> Result<(), BoxedError> {
        let builder = CreateMessage::new().content(msg);
        let message = ChannelId::new(chat_id)
            .send_message(&self.client.http, builder)
            .await?;
        Ok(())
    }
}

async fn event_handler(
    ctx: &serenity::Context,
    event: &serenity::FullEvent,
    _framework: poise::FrameworkContext<'_, BotState, BoxedError>,
    _state: &BotState,
) -> Result<(), BoxedError> {
    match event {
        serenity::FullEvent::Ready { data_about_bot, .. } => {
            tracing::info!("discord: Logged in as {}", data_about_bot.user.name);

            // We can use ChannelId directly to send a message to a specific channel; in this case, the
            // message would be sent to the #testing channel on the discord server.
            /*let embed = CreateEmbed::new().title("System Resource Load").field(
                "CPU Load Average",
                format!("{:.2}%", 10.0),
                false,
            );
            let builder = CreateMessage::new().embed(embed);
            let message = ChannelId::new(1145642256443904002)
                .send_message(&ctx, builder)
                .await;
            if let Err(why) = message {
                eprintln!("Error sending message: {why:?}");
            };*/
        }
        serenity::FullEvent::Message { new_message } => {
            let mut data = ctx.data.write().await;
            let state = data.get_mut::<BotState>().unwrap();
            println!("{:?}", new_message);

            let author = new_message
                .author
                .global_name
                .clone()
                .or(Some(new_message.author.name.clone()))
                .unwrap();

            if let Some(agent) = &state.telegram_agent {
                agent
                    .send(-4221527632, format!("{}: {}", author, new_message.content))
                    .await;
            }
        }
        _ => {}
    }
    Ok(())
}
