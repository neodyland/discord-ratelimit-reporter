use std::time::Duration;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::builder()
        .filter_module("discord_ratelimit_reporter", {
            if cfg!(debug_assertions) {
                log::LevelFilter::Debug
            } else {
                log::LevelFilter::Info
            }
        })
        .init();
    let mut interval = tokio::time::interval(Duration::from_secs(60));
    loop {
        interval.tick().await;
        let response = reqwest::get("https://discord.com/api/v10/invites/discord-developers").await?;
        let body = response.json::<serde_json::Value>().await?;
        if let Some(message) = body.get("message") {
            if message.as_str().unwrap().starts_with("You are being blocked") {
                log::error!("Rate limit exceeded: {}", message);
            } else {
                log::debug!("Rate limit not exceeded");
            }
        } else {
            log::debug!("Rate limit not exceeded");
        }
    }
}
