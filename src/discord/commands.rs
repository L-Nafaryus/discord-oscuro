use async_process::Command;
use poise::serenity_prelude as serenity;
use rand::Rng;
use std::collections::HashMap;
use std::str;

use super::Context;
use crate::errors::BoxedError;

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

#[poise::command(slash_command, prefix_command)]
pub async fn dice(ctx: Context<'_>) -> Result<(), BoxedError> {
    let number = {
        let mut rng = rand::thread_rng();
        rng.gen_range(1..21)
    };

    let response = format!("{} throws {}.", ctx.author(), number);
    let response = match number {
        20 => format!("{} Critical success.", response),
        1 => format!("{} Critical failure.", response),
        _ => response,
    };

    ctx.say(response).await?;
    Ok(())
}

#[derive(Debug, poise::ChoiceParameter)]
pub enum ServiceChoice {
    #[name = "Elnafo VCS"]
    ElnafoVcs,
    #[name = "Elnafo Mail"]
    ElnafoMail,
}

#[poise::command(slash_command, prefix_command)]
pub async fn status(
    ctx: Context<'_>,
    #[description = "Check service status"] service: ServiceChoice,
) -> Result<(), BoxedError> {
    let mut systemctl = Command::new("systemctl");
    let service_info = match service {
        ServiceChoice::ElnafoVcs => systemctl.arg("show").arg("gitea.service"),
        ServiceChoice::ElnafoMail => systemctl.arg("show").arg("acpid.service"),
    };
    let output = service_info.output().await?;

    let mut data: HashMap<&str, &str> = HashMap::new();

    for line in str::from_utf8(&output.stdout)?.lines() {
        let kv: Vec<&str> = line.split('=').collect();
        data.insert(kv[0], kv[1]);
    }
    println!("{:?} {:?}", data["LoadState"], data["SubState"]);

    if data["LoadState"] == "loaded" && data["SubState"] == "running" {
        ctx.say(format!(
            "{:?} is up and running for {}",
            service, data["ExecMainStartTimestamp"]
        ))
        .await?;
    } else {
        ctx.say(format!("{:?} is dead", service)).await?;
    }

    Ok(())
}
