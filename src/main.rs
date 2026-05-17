use anyhow::{Context, Result};
use assistant_maxxing::{bot::BotApp, config::Config};

#[tokio::main]
async fn main() -> Result<()> {
    dotenvy::dotenv().ok();
    tracing_subscriber::fmt::init();

    let config = Config::from_env().context("failed to load configuration")?;
    BotApp::new(config)
        .run()
        .await
        .context("bot runtime failed")?;

    Ok(())
}
