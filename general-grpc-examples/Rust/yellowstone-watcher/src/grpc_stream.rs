use std::collections::HashMap;

use anyhow::{Context, Result};
use futures::StreamExt;
use tokio::sync::mpsc;
use tracing::{debug, error, info, warn};
use std::time::Duration;
use tonic::transport::ClientTlsConfig;
use yellowstone_grpc_client::GeyserGrpcClient;
use yellowstone_grpc_proto::prelude::{
    CommitmentLevel, SubscribeRequest, SubscribeRequestFilterTransactions, SubscribeUpdate,
};

use crate::config::Config;

/// Events emitted by the gRPC reader task.
#[derive(Debug)]
pub enum StreamEvent {
    /// A non-vote, non-failed transaction was observed.
    Transaction {
        slot: u64,
        index: u64,
        signature: String,
        /// Unix timestamp in milliseconds of when this event was received.
        received_at_ms: u64,
    },
}

/// Spawn a background task that connects to Yellowstone and forwards
/// [`StreamEvent`]s over `tx`.  The task reconnects automatically on error.
pub fn spawn_grpc_reader(cfg: Config, tx: mpsc::Sender<StreamEvent>) {
    tokio::spawn(async move {
        loop {
            match run_stream(&cfg, tx.clone()).await {
                Ok(()) => {
                    info!("gRPC stream ended cleanly; reconnecting …");
                }
                Err(e) => {
                    error!("gRPC stream error: {e:#}; reconnecting in 3 s …");
                    tokio::time::sleep(std::time::Duration::from_secs(3)).await;
                }
            }
        }
    });
}

/// Connect, subscribe, and forward events until the stream closes or errors.
async fn run_stream(cfg: &Config, tx: mpsc::Sender<StreamEvent>) -> Result<()> {
    info!("Connecting to Yellowstone gRPC at {}", cfg.grpc_endpoint);

    let mut client = GeyserGrpcClient::build_from_shared(cfg.grpc_endpoint.clone())?
        .x_token(cfg.grpc_x_token.clone())?
        .connect_timeout(Duration::from_secs(20))
        .timeout(Duration::from_secs(20))
        .tls_config(ClientTlsConfig::new().with_native_roots())?
        .max_decoding_message_size(1024 * 1024 * 1024)
        .connect()
        .await
        .context("Failed to connect to Yellowstone gRPC")?;

    info!("Connected. Sending subscribe request …");

    let request = build_subscribe_request(cfg);
    print!("request--: {:?}", request);
    let (mut _sink, mut stream) = client
        .subscribe_with_request(Some(request))
        .await
        .context("subscribe_with_request failed")?;

    info!("Subscribed. Streaming …");

    while let Some(msg) = stream.next().await {
        match msg {
            Ok(update) => {
                if let Err(e) = handle_update(update, &tx).await {
                    warn!("Failed to forward event: {e}");
                }
            }
            Err(status) => {
                return Err(anyhow::anyhow!("stream error: {status}"));
            }
        }
    }

    Ok(())
}

/// Build the [`SubscribeRequest`] — single transactions filter to stay within
/// the 1-filter limit enforced by the Shyft endpoint.
fn build_subscribe_request(cfg: &Config) -> SubscribeRequest {
    let tx_filter = SubscribeRequestFilterTransactions {
        vote: Some(false),
        failed: Some(false),
        signature: None,
        account_include: cfg.account_include.clone(),
        account_exclude: vec![],
        account_required: vec![],
    };
    
    SubscribeRequest {
        accounts: HashMap::new(),
        slots: HashMap::new(),
        transactions: {
            let mut m = HashMap::new();
            m.insert("txn".to_owned(), tx_filter);
            m
        },
        transactions_status: HashMap::new(),
        blocks: HashMap::new(),
        blocks_meta: HashMap::new(),
        entry: HashMap::new(),
        accounts_data_slice: vec![],
        commitment: Some(CommitmentLevel::Confirmed as i32),
        ping: None,
        from_slot: None,
    }
}

/// Decode one [`SubscribeUpdate`] and forward the relevant event.
async fn handle_update(update: SubscribeUpdate, tx: &mpsc::Sender<StreamEvent>) -> Result<()> {
    use yellowstone_grpc_proto::prelude::subscribe_update::UpdateOneof;

    let inner = match update.update_oneof {
        Some(v) => v,
        None => return Ok(()),
    };

    match inner {
        // ── Transaction ──────────────────────────────────────────────────────
        UpdateOneof::Transaction(tx_update) => {
            let slot = tx_update.slot;
            let info = match tx_update.transaction {
                Some(t) => t,
                None => return Ok(()),
            };

            // Signature is the first 64 bytes of the first signature field
            let sig = signature_from_bytes(&info.signature);
            let index = info.index;
            let received_at_ms = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_millis() as u64;
            debug!("gRPC tx  slot={slot}  index={index}  sig={sig}");

            tx.send(StreamEvent::Transaction { slot, index, signature: sig, received_at_ms })
                .await
                .ok();
        }

        _ => {}
    }

    Ok(())
}

/// Convert raw signature bytes to a base-58 string.
fn signature_from_bytes(bytes: &[u8]) -> String {
    bs58::encode(bytes).into_string()
}
