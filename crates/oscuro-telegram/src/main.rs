use serenity::builder::ExecuteWebhook;
use serenity::http::Http;
use serenity::model::webhook::Webhook;
use teloxide::prelude::*;
use teloxide::types::Recipient;

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    log::info!("Starting throw dice bot...");

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
