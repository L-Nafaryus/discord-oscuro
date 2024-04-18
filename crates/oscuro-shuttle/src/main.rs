use oscuro_core::{client, config::Config, AppState};

#[shuttle_runtime::main]
async fn main(
    #[shuttle_runtime::Secrets] secrets: shuttle_runtime::SecretStore,
) -> shuttle_serenity::ShuttleSerenity {
    let token = secrets
        .get("discord_token")
        .expect("Variable 'DISCORD_TOKEN' must be set");

    let state = AppState {
        config: Config {
            discord_token: token,
        },
    };

    let client = client(state).await.expect("Failed to create client");

    Ok(client.into())
}
