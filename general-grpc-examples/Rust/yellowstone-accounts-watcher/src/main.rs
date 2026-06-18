mod config;
mod grpc_stream;

use anyhow::Result;
use tokio::sync::mpsc;
use tracing::info;
use tracing_subscriber::{fmt, prelude::*, EnvFilter};

use config::Config;
use grpc_stream::{spawn_grpc_readers, StreamEvent};

#[tokio::main]
async fn main() -> Result<()> {
    dotenvy::dotenv().ok();

    rustls::crypto::ring::default_provider()
        .install_default()
        .expect("Failed to install rustls crypto provider");

    tracing_subscriber::registry()
        .with(EnvFilter::try_from_default_env().unwrap_or_else(|_| "info".into()))
        .with(fmt::layer())
        .init();

    let cfg = Config::from_env()?;
    for s in &cfg.streams {
        info!("[{}] watching {} accounts", s.name, s.accounts.len());
    }

    let (event_tx, mut event_rx) = mpsc::channel::<StreamEvent>(65_536);
    spawn_grpc_readers(cfg, event_tx);

    while let Some(event) = event_rx.recv().await {
        match event {
            StreamEvent::AccountUpdate {
                stream,
                slot,
                pubkey,
                txn_signature,
            } => {
                let sig = txn_signature.as_deref().unwrap_or("<no-sig>");
                info!("[{stream}]  slot={slot}  account={pubkey}  sig={sig}");
            }
        }
    }

    info!("Event channel closed — exiting");
    Ok(())
}
