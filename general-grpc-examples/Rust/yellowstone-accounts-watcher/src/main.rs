mod config;
mod stream;
mod verifier;

use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;

use anyhow::Result;
use dashmap::DashMap;
use tokio::sync::mpsc;
use tracing::info;
use tracing_subscriber::{fmt, prelude::*, EnvFilter};

use config::Config;
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
        "verifier started — targets={} rpc={}",
        cfg.target_pubkeys.join(", "),
        cfg.rpc_endpoint
    );

    // Shared map: (slot, pubkey) → list of txn signatures delivered by gRPC for that pubkey.
    let updates: Arc<DashMap<(u64, String), Vec<String>>> = Arc::new(DashMap::new());

    // Tracks the earliest slot for which we received a gRPC account update.
    // Initialised to u64::MAX ("not seen anything yet"); the account-update task
    // drives it down to the real first slot via fetch_min.
    let start_slot: Arc<AtomicU64> = Arc::new(AtomicU64::new(u64::MAX));

    // ── Thread 1: account updates ────────────────────────────────────────────
    // Receives every account-update event from the stream, prints it, and
    // records the signature in the shared slot map.
    let (account_tx, mut account_rx) = mpsc::channel::<AccountUpdate>(65_536);
    let updates_for_acct = updates.clone();
    let start_slot_for_acct = start_slot.clone();
    let account_handle = tokio::spawn(async move {
        while let Some(AccountUpdate { slot, pubkey, txn_signature }) = account_rx.recv().await {
            // Record the earliest slot we've seen so the verifier can ignore
            // slots that finalized before our subscription was established.
            start_slot_for_acct.fetch_min(slot, Ordering::Relaxed);

            let sig = txn_signature.clone().unwrap_or_default();
            info!("ACCOUNT UPDATE | slot={slot} pubkey={pubkey} sig={sig}");
            updates_for_acct
                .entry((slot, pubkey))
                .or_default()
                .push(sig);
        }
        info!("account-update channel closed");
    });

    // ── Thread 2: slot verification ──────────────────────────────────────────
    // Receives finalized slot numbers and cross-checks the gRPC-observed
    // signatures against the on-chain block via JSON-RPC.
    let (slot_tx, mut slot_rx) = mpsc::channel::<u64>(65_536);
    let verifier = Arc::new(Verifier::new(
        &cfg.rpc_endpoint,
        cfg.target_pubkeys.clone(),
        updates.clone(),
        start_slot.clone(),
        cfg.sol_bal_check,
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

    spawn_stream(cfg, account_tx, slot_tx);

    tokio::try_join!(account_handle, slot_handle)?;
    info!("both tasks exited — shutting down");
    Ok(())
}
