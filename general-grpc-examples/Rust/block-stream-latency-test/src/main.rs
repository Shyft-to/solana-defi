use std::sync::Arc;

use anyhow::Result;
use tracing::info;

mod blocks;
mod config;
mod latency;
mod slack;

use config::Config;

#[tokio::main]
async fn main() -> Result<()> {
    dotenvy::dotenv().ok();

    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info")),
        )
        .compact()
        .init();

    let cfg = Arc::new(Config::from_env()?);

    info!(
        endpoint = %cfg.grpc_endpoint,
        rpc_latency = cfg.rpc_url.is_some(),
        slack = cfg.slack_webhook_url.is_some(),
        "block streamer starting"
    );

    blocks::spawn(cfg);

    tokio::signal::ctrl_c().await?;
    info!("shutting down");

    Ok(())
}
