use poise::serenity_prelude as serenity;

use super::errors::BoxedError;
use super::Context;

#[poise::command(prefix_command)]
pub async fn register(ctx: Context<'_>) -> Result<(), BoxedError> {
    poise::builtins::register_application_commands_buttons(ctx).await?;
    Ok(())
}

#[poise::command(slash_command, prefix_command)]
pub async fn age(
    ctx: Context<'_>,
    #[description = "Ooph user"] user: Option<serenity::User>,
) -> Result<(), BoxedError> {
    let u = user.as_ref().unwrap_or_else(|| ctx.author());
    let response = format!("{}'s account was created at {}", u.name, u.created_at());
    ctx.say(response).await?;
    Ok(())
}
