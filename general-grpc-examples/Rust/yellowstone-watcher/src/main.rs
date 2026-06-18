mod config;
mod grpc_stream;
mod reconciler;
mod slack;
mod slot_tracker;
mod types;

use anyhow::Result;
use solana_client::nonblocking::rpc_client::RpcClient;
use solana_sdk::commitment_config::{CommitmentConfig, CommitmentLevel};
use tokio::sync::mpsc;
use tracing::{debug, error, info, warn};
use tracing_subscriber::{fmt, prelude::*, EnvFilter};

use config::Config;
use grpc_stream::{spawn_grpc_reader, StreamEvent};
use slot_tracker::SlotTracker;

#[tokio::main]
async fn main() -> Result<()> {
    // ── Load .env if present ─────────────────────────────────────────────────
    dotenvy::dotenv().ok();

    // ── Logging ───────────────────────────────────────────────────────────────
    tracing_subscriber::registry()
        .with(EnvFilter::try_from_default_env().unwrap_or_else(|_| "info".into()))
        .with(fmt::layer())
        .init();

    // ── Config ────────────────────────────────────────────────────────────────
    let cfg = Config::from_env()?;
    info!(
        "Watching {} accounts  lag={} slots",
        cfg.account_include.len(),
        cfg.reconcile_lag_slots
    );

    // ── Shared state ─────────────────────────────────────────────────────────
    let tracker = SlotTracker::new();

    // ── RPC client (for reconciliation) ──────────────────────────────────────
    let rpc = RpcClient::new_with_commitment(
        cfg.rpc_endpoint.clone(),
        CommitmentConfig {
            commitment: CommitmentLevel::Confirmed,
        },
    );

    // ── gRPC reader ───────────────────────────────────────────────────────────
    // Generous buffer — bursts of transactions arrive in batches.
    let (event_tx, mut event_rx) = mpsc::channel::<StreamEvent>(65_536);
    spawn_grpc_reader(cfg.clone(), event_tx);

    // ── Event loop ────────────────────────────────────────────────────────────
    info!("Event loop started");

    // Highest slot number seen in any transaction so far.
    // Used to measure the reconciliation lag without a slot subscription.
    let mut highest_slot_seen: u64 = 0;
    // Slots waiting for lag to elapse before reconciliation.
    // Stored as plain slot numbers — ready when highest_slot_seen >= slot + lag.
    let mut pending_reconcile: Vec<u64> = Vec::new();

    while let Some(event) = event_rx.recv().await {
        match event {
            StreamEvent::Transaction { slot, signature } => {
                info!("TX  slot={slot}  sig={signature}");
                tracker.record_transaction(slot, signature);

                if slot > highest_slot_seen {
                    highest_slot_seen = slot;
                    debug!(
                        "Chain advanced to slot {highest_slot_seen} — currently buffering {} slots waiting for verification",
                        tracker.len()
                    );
                    tracker.dump();
                }

                // Enqueue slot for reconciliation the first time we see it.
                if !pending_reconcile.contains(&slot) {
                    pending_reconcile.push(slot);
                }

                // Drain slots that have waited at least `lag` slots.
                let lag = cfg.reconcile_lag_slots;
                let ready: Vec<u64> = pending_reconcile
                    .iter()
                    .filter(|&&s| highest_slot_seen >= s + lag)
                    .copied()
                    .collect();

                pending_reconcile.retain(|&s| highest_slot_seen < s + lag);

                for target_slot in ready {
                    let slot_data = match tracker.take(target_slot) {
                        Some(d) => d,
                        None => {
                            debug!("Slot {target_slot} had no transactions for watched accounts — nothing to reconcile");
                            continue;
                        }
                    };

                    match reconciler::reconcile(&rpc, &cfg, &slot_data).await {
                        Ok(report) => {
                            if report.is_clean() {
                                info!(
                                    "----> Slot {} verfied cleanly all matched — {} transactions matched between gRPC stream and RPC <----",
                                    report.slot, report.grpc_count
                                );
                            } else {
                                if !report.missed.is_empty() {
                                    error!(
                                        "Slot {} is missing {} transaction(s) that the RPC confirmed but the gRPC stream never delivered — possible data loss:",
                                        report.slot,
                                        report.missed.len()
                                    );
                                    for sig in &report.missed {
                                        error!("  {sig}");
                                    }
                                    if let Some(url) = &cfg.slack_webhook_url {
                                        slack::notify_missed(url, report.slot, &report.missed).await;
                                    }
                                }
                                if !report.extra.is_empty() {
                                    warn!(
                                        "Slot {} has {} transaction(s) seen via gRPC that the RPC does not recognise — may be a timing issue:",
                                        report.slot,
                                        report.extra.len()
                                    );
                                    for sig in &report.extra {
                                        warn!("  {sig}");
                                    }
                                    if let Some(url) = &cfg.slack_webhook_url {
                                        slack::notify_extra(url, report.slot, &report.extra).await;
                                    }
                                }
                            }
                        }
                        Err(e) => {
                            error!("Could not reconcile slot {target_slot} — RPC call failed: {e:#}");
                        }
                    }
                }
            }
        }
    }

    warn!("Event channel closed — exiting");
    Ok(())
}
