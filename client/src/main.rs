use std::{env, time::Duration};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();
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
    let mut exceeded = false;
    loop {
        interval.tick().await;
        let response = reqwest::get("https://discord.com/api/v10/invites/discord-developers").await?;
        let body = response.json::<serde_json::Value>().await?;
        if let Some(message) = body.get("message") {
            if message.as_str().unwrap().starts_with("You are being blocked") && !exceeded {
                exceeded = true;
                log::error!("Rate limit exceeded: {}", message);
                let client = reqwest::Client::new();
                client
                    .post(format!("{}/ratelimit", env::var("API_URL").unwrap()))
                    .header("Authorization", env::var("API_KEY").unwrap())
                    .json(&serde_json::json!({ "node": env::var("NODE_NAME").unwrap() }))
                    .send()
                    .await?;
            } else {
                if exceeded {
                    log::error!("Rate limit still exceeded: {}", message);
                } else {
                    log::debug!("Rate limit not exceeded");
                }
            }
        } else {
            exceeded = false;
            log::debug!("Rate limit not exceeded");
        }
    }
}
