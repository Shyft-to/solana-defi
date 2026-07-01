mod config;
mod fetcher;
mod stream;
mod verifier;

use std::sync::Arc;

use anyhow::Result;
use dashmap::DashMap;
use tokio::sync::mpsc;
use tracing::info;
use tracing_subscriber::{fmt, prelude::*, EnvFilter};

use config::Config;
use fetcher::GrpcDataMap;
use stream::{spawn_stream, AccountUpdate};
use verifier::Verifier;


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
    info!(
        "verifier started — target={} rpc={} grpc_commitment={} rpc_commitment={} retain_slots={}",
        cfg.target_pubkey,
        cfg.rpc_endpoint,
        cfg.grpc_commitment,
        cfg.rpc_commitment,
        cfg.grpc_data_retain_slots,
    );

    // (slot, pubkey) → latest account data bytes delivered by gRPC for that slot.
    let grpc_data: GrpcDataMap = Arc::new(DashMap::new());

    // ── Task 1: account updates ──────────────────────────────────────────────
    // Receives every gRPC account update, prints it, and stores the raw data
    // in grpc_data. Evicts entries older than grpc_data_retain_slots.
    let (account_tx, mut account_rx) = mpsc::channel::<AccountUpdate>(65_536);
    let grpc_data_for_acct = grpc_data.clone();
    let retain_slots = cfg.grpc_data_retain_slots;
    let account_handle = tokio::spawn(async move {
        while let Some(AccountUpdate { slot, pubkey, txn_signature, data }) = account_rx.recv().await {
            let sig = txn_signature.unwrap_or_default();
            let preview: String = data[..data.len().min(64)]
                .iter()
                .map(|b| format!("{b:02x}"))
                .collect();
            let data_str = if data.len() > 64 {
                format!("{preview}… ({} bytes)", data.len())
            } else {
                format!("{preview} ({} bytes)", data.len())
            };
            println!("ACCOUNT UPDATE | slot={slot} | sig={sig} | data={data_str}");

            let stale = slot.saturating_sub(retain_slots);
            grpc_data_for_acct.remove(&(stale, pubkey.clone()));
            grpc_data_for_acct.insert((slot, pubkey), data);
        }
        info!("account-update channel closed");
    });

    // ── Task 2: slot verifier ────────────────────────────────────────────────
    // On every finalized slot, calls getAccountInfo and compares the RPC state
    // (at the returned context slot) against the latest gRPC-delivered data.
    let (slot_tx, mut slot_rx) = mpsc::channel::<u64>(65_536);
    let verifier = Arc::new(Verifier::new(
        &cfg.rpc_endpoint,
        cfg.rpc_commitment,
        cfg.target_pubkey.clone(),
        grpc_data.clone(),
    ));
    let slot_handle = tokio::spawn(async move {
        while let Some(slot) = slot_rx.recv().await {
            let v = verifier.clone();
            tokio::spawn(async move {
                v.verify_slot(slot).await;
            });
        }
        info!("slot-verifier channel closed");
    });

    // fetch_slot_tx is required by spawn_stream's signature; we don't use it here.
    let (fetch_slot_tx, mut fetch_slot_rx) = mpsc::channel::<u64>(65_536);
    tokio::spawn(async move { while fetch_slot_rx.recv().await.is_some() {} });

    spawn_stream(cfg, account_tx, slot_tx, fetch_slot_tx);

    tokio::try_join!(account_handle, slot_handle)?;
    info!("all tasks exited — shutting down");
    Ok(())
}
