mod config;
mod grpc_stream;
mod latency;
mod slack;
mod types;

use std::time::Duration;

use anyhow::Result;
use tokio::sync::mpsc;
use tracing::{info, warn};
use tracing_subscriber::{fmt, prelude::*, EnvFilter};

use config::Config;
use latency::{LatencyTracker, LogOptions};
use types::StreamEvent;

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
        "S1: {} account(s) @ {:?}   S2: {} account(s) @ {:?}   blocks_meta @ {:?}   buffer={} slots   \
         slack_alerts={}   log_transactions={}   stats_interval={}s",
        cfg.account_include_1.len(),
        cfg.commitment_1,
        cfg.account_include_2.len(),
        cfg.commitment_2,
        cfg.commitment_blocks_meta,
        cfg.latency_buffer_slots,
        cfg.slack_webhook_url.is_some(),
        cfg.log_transactions,
        cfg.stats_interval_secs,
    );

    // A blocks_meta stream lagging behind the transaction streams means every
    // transaction waits in the buffer for its block time. That is correct, but
    // `finalized` trails `processed` by roughly 32 slots, so a buffer smaller
    // than that would evict transactions before they could ever be timed.
    let tx_commitment_max = cfg.commitment_1.max(cfg.commitment_2);
    if cfg.commitment_blocks_meta > tx_commitment_max && cfg.latency_buffer_slots < 64 {
        warn!(
            "blocks_meta commitment ({:?}) is stricter than both tx streams — it will lag them. \
             LATENCY_BUFFER_SLOTS={} is likely too small; transactions may be evicted before timing.",
            cfg.commitment_blocks_meta, cfg.latency_buffer_slots,
        );
    }

    // Both output paths off means the process runs and reconnects but never
    // prints a single latency number — almost certainly a misconfiguration
    // rather than the intent.
    if !cfg.log_transactions && cfg.stats_interval_secs == 0 {
        warn!(
            "LOG_TRANSACTIONS=false and STATS_INTERVAL_SECS=0 — no latency output of any kind will be printed"
        );
    }

    // ── Streams ───────────────────────────────────────────────────────────────
    // Two transaction streams plus blocks_meta, each its own gRPC connection and
    // its own tokio task — tokio's multi-threaded runtime schedules them across
    // OS threads/cores concurrently. Generous buffer: transactions arrive in bursts.
    let (event_tx, mut event_rx) = mpsc::channel::<StreamEvent>(65_536);
    grpc_stream::spawn_all(cfg.clone(), event_tx);

    // ── Event loop ────────────────────────────────────────────────────────────
    // Single-threaded ownership of the tracker: every event is joined here, so
    // no locking is needed and the ordering of arrivals is preserved.
    info!("Event loop started");
    let mut tracker = LatencyTracker::new(
        cfg.latency_buffer_slots,
        LogOptions {
            log_transactions: cfg.log_transactions,
            log_signatures: cfg.log_signatures,
        },
    );

    if cfg.stats_interval_secs > 0 {
        info!("Printing p50/p95/p99 latency every {}s", cfg.stats_interval_secs);
        let mut ticker = tokio::time::interval(Duration::from_secs(cfg.stats_interval_secs));
        ticker.set_missed_tick_behavior(tokio::time::MissedTickBehavior::Delay);
        // The first tick fires immediately; skip it so the first report reflects
        // a full interval of samples rather than firing the moment we start.
        ticker.tick().await;

        loop {
            tokio::select! {
                event = event_rx.recv() => match event {
                    Some(event) => apply_event(&mut tracker, event),
                    None => break,
                },
                _ = ticker.tick() => {
                    for line in tracker.stats_lines() {
                        info!("{line}");
                    }
                }
            }
        }
    } else {
        while let Some(event) = event_rx.recv().await {
            apply_event(&mut tracker, event);
        }
    }

    warn!("Event channel closed — exiting");
    Ok(())
}

fn apply_event(tracker: &mut LatencyTracker, event: StreamEvent) {
    match event {
        StreamEvent::Transaction {
            stream,
            slot,
            signature,
            recv_ms,
        } => tracker.on_transaction(stream, slot, signature, recv_ms),

        StreamEvent::BlockMeta {
            slot,
            block_time_ms,
        } => tracker.on_block_meta(slot, block_time_ms),
    }
}
