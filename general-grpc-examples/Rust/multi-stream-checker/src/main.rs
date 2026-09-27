mod config;
mod grpc_stream;
mod latency;
mod rpc_poller;
mod slack;
mod types;

use std::time::Duration;

use anyhow::Result;
use tokio::sync::mpsc;
use tracing::{info, warn};
use tracing_subscriber::{fmt, prelude::*, EnvFilter};

use config::Config;
use latency::{LatencyTracker, LogOptions};
use types::{StreamEvent, StreamId};

/// Consecutive `[STATS] n=0` reports for a stream before it's treated as
/// worth a health check — one alone is normal for a quiet account, three in a
/// row is not.
const ZERO_SAMPLE_STREAK_ALERT: u32 = 3;

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
    let s2_summary = match &cfg.account_include_2 {
        Some(accounts) => format!("{} account(s) @ {:?}", accounts.len(), cfg.commitment_2),
        None => "disabled (ACCOUNT_INCLUDE_2 not set)".to_owned(),
    };
    info!(
        "region={}   S1: {} account(s) @ {:?}   S2: {s2_summary}   blocks_meta @ {:?}   buffer={} slots   \
         slack_alerts={}   log_transactions={}   log_blocks_meta={}   stats_interval={}s",
        cfg.region,
        cfg.account_include_1.len(),
        cfg.commitment_1,
        cfg.commitment_blocks_meta,
        cfg.latency_buffer_slots,
        cfg.slack_webhook_url.is_some(),
        cfg.log_transactions,
        cfg.log_blocks_meta,
        cfg.stats_interval_secs,
    );

    // A blocks_meta stream lagging behind the transaction streams means every
    // transaction waits in the buffer for its block time. That is correct, but
    // `finalized` trails `processed` by roughly 32 slots, so a buffer smaller
    // than that would evict transactions before they could ever be timed.
    let tx_commitment_max = match cfg.stream_2_enabled() {
        true => cfg.commitment_1.max(cfg.commitment_2),
        false => cfg.commitment_1,
    };
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

    if cfg.rpc_poller_enabled() {
        tokio::spawn(rpc_poller::run(
            cfg.region.clone(),
            cfg.slack_webhook_url.clone(),
            cfg.rpc_url.clone().expect("rpc_poller_enabled implies rpc_url is set"),
            cfg.rpc_poll_interval_secs,
            cfg.rpc_commitment.clone(),
        ));
    }

    // ── Event loop ────────────────────────────────────────────────────────────
    // Single-threaded ownership of the tracker: every event is joined here, so
    // no locking is needed and the ordering of arrivals is preserved.
    info!("Event loop started");
    let mut tracker = LatencyTracker::new(
        cfg.latency_buffer_slots,
        LogOptions {
            log_transactions: cfg.log_transactions,
            log_signatures: cfg.log_signatures,
            log_blocks_meta: cfg.log_blocks_meta,
        },
        cfg.stream_2_enabled(),
    );

    if cfg.stats_interval_secs > 0 {
        info!("Printing p50/p95/p99 latency every {}s", cfg.stats_interval_secs);
        let mut ticker = tokio::time::interval(Duration::from_secs(cfg.stats_interval_secs));
        ticker.set_missed_tick_behavior(tokio::time::MissedTickBehavior::Delay);
        // The first tick fires immediately; skip it so the first report reflects
        // a full interval of samples rather than firing the moment we start.
        ticker.tick().await;

        // Consecutive zero-sample streaks per stream, used to trigger a grpc
        // health check — see `check_zero_streak`.
        let mut zero_streak = [0u32; 2];

        loop {
            tokio::select! {
                event = event_rx.recv() => match event {
                    Some(event) => apply_event(&mut tracker, event),
                    None => break,
                },
                _ = ticker.tick() => {
                    // Sample counts must be read before `stats_lines()`, which
                    // drains them.
                    let (s1_count, s2_count) = tracker.sample_counts();

                    for line in tracker.stats_lines() {
                        info!("{line}");
                    }

                    check_zero_streak(&cfg, StreamId::One, s1_count, &mut zero_streak[0]).await;
                    if cfg.stream_2_enabled() {
                        check_zero_streak(&cfg, StreamId::Two, s2_count, &mut zero_streak[1]).await;
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

/// Track a stream's consecutive zero-sample streak across `[STATS]` reports.
/// On the third report in a row with nothing recorded, run a unary `get_slot`
/// health check against the grpc endpoint and alert with the outcome — a
/// healthy response means the endpoint is up but this stream specifically has
/// gone quiet, an error means the endpoint itself looks dead. The streak
/// resets either way, so the next alert only fires after another full streak.
async fn check_zero_streak(cfg: &Config, stream: StreamId, sample_count: usize, streak: &mut u32) {
    if sample_count > 0 {
        *streak = 0;
        return;
    }

    *streak += 1;
    if *streak < ZERO_SAMPLE_STREAK_ALERT {
        return;
    }
    *streak = 0;

    let quiet_secs = ZERO_SAMPLE_STREAK_ALERT as u64 * cfg.stats_interval_secs;
    warn!(
        "[{}] [{stream}] no samples for {ZERO_SAMPLE_STREAK_ALERT} consecutive stats reports (~{quiet_secs}s) — checking grpc health",
        cfg.region
    );

    let text = match grpc_stream::health_check(cfg).await {
        Ok(slot) => format!(
            ":warning: *[{}] [{stream}]* no transactions for {ZERO_SAMPLE_STREAK_ALERT} consecutive stats reports (~{quiet_secs}s). \
             grpc endpoint is responsive (get_slot={slot}) — likely no matching activity, not a dead stream.",
            cfg.region
        ),
        Err(e) => format!(
            ":red_circle: *[{}] [{stream}]* no transactions for {ZERO_SAMPLE_STREAK_ALERT} consecutive stats reports (~{quiet_secs}s), \
             AND the grpc health check failed: {e:#} — endpoint appears dead.",
            cfg.region
        ),
    };
    slack::report(&cfg.slack_webhook_url, &text).await;
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
