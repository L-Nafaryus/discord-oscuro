use teloxide::prelude::*;
use teloxide::types::Recipient;
use teloxide::utils::command::BotCommands;

use crate::config::Config;
use crate::errors;
use rand::Rng;

async fn main() {
    let bot = Bot::from_env();

    /*let http = Http::new("");
        let webhook = Webhook::from_url(&http, "https://discord.com/api/webhooks/1259860143579987999/whI0ozB5uc17Wdzkb2-HSrVGi8h_MyR2_4eyCsGuGpQN4KcjMhq7rfQH1JIdbD1HNaW_")
            .await
            .expect("Replace the webhook with your own");

        let builder = ExecuteWebhook::new().content("hello there").username("Webhook test");
        webhook.execute(&http, false, builder).await.expect("Could not execute webhook.");
    */
    teloxide::repl(bot, |bot: Bot, msg: Message| async move {
        bot.send_dice(msg.chat.id).await?;
        Ok(())
    })
    .await;

    /*bot.send_message(Recipient::Id(ChatId(-4221527632)), "Heya!")
    .await
    .expect("err");*/
}

#[derive(Clone, Debug)]
pub struct Client {
    bot: Bot,
}

impl Client {
    pub fn new(config: Config) -> Self {
        Self {
            bot: Bot::new(config.telegram_token.unwrap()),
        }
    }

    pub async fn start(&self) -> Result<(), errors::BoxedError> {
        Command::repl(self.bot.clone(), event_handler).await;
        Ok(())
    }

    pub async fn send(&self, chat_id: i64, msg: String) -> ResponseResult<()> {
        self.bot
            .send_message(Recipient::Id(ChatId(chat_id)), msg)
            .await?;
        Ok(())
    }
}

#[derive(BotCommands, Clone)]
#[command(rename_rule = "lowercase")]
pub enum Command {
    #[command()]
    Dice,
}

async fn event_handler(bot: Bot, msg: Message, cmd: Command) -> ResponseResult<()> {
    match cmd {
        Command::Dice => {
            let number = {
                let mut rng = rand::thread_rng();
                rng.gen_range(1..21)
            };

            let response = format!("{} throws {}.", "test", number);
            let response = match number {
                20 => format!("{} Critical success.", response),
                1 => format!("{} Critical failure.", response),
                _ => response,
            };
            // -4221527632

            bot.send_message(Recipient::Id(msg.chat.id), response)
                .await?;
        }
    };

    Ok(())
}
