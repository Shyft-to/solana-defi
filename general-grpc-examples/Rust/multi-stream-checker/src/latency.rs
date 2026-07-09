use std::collections::BTreeMap;

use tracing::{info, warn};

use crate::types::StreamId;

/// A transaction seen on a stream whose slot has no `block_time` yet.
#[derive(Debug)]
struct PendingTx {
    stream: StreamId,
    signature: String,
    recv_ms: i64,
}

/// Controls what per-transaction output `LatencyTracker` prints. Sampling for
/// the periodic `[STATS]` report is unaffected by either field — both keep
/// recording latencies even when the per-transaction line itself is off.
#[derive(Debug, Clone, Copy)]
pub struct LogOptions {
    /// Print a line for every resolved transaction. Off leaves only the
    /// periodic `[STATS]` report (if that is also enabled).
    pub log_transactions: bool,
    /// Include the transaction signature on that line. Ignored when
    /// `log_transactions` is off.
    pub log_signatures: bool,
}

impl Default for LogOptions {
    fn default() -> Self {
        Self {
            log_transactions: true,
            log_signatures: true,
        }
    }
}

/// Joins transaction arrivals against block execution timestamps.
///
/// The two sources race: for a given slot, blocks_meta may land before or after
/// the transactions of that slot. Both orders are handled —
///
///   * tx first  → buffer it, emit when the slot's blocks_meta arrives
///   * meta first → the block time is already known, emit immediately
///
/// Slots more than `buffer_slots` behind the highest slot seen are evicted, so
/// memory stays bounded even when a blocks_meta update never arrives.
pub struct LatencyTracker {
    /// slot → block execution time, Unix milliseconds.
    block_time_ms: BTreeMap<u64, i64>,
    /// slot → transactions still waiting on that slot's block time.
    pending: BTreeMap<u64, Vec<PendingTx>>,
    highest_slot: u64,
    buffer_slots: u64,
    log_opts: LogOptions,
    /// Latencies (ms) recorded on stream 1 since the last `stats_lines` call.
    samples_s1: Vec<i64>,
    /// Latencies (ms) recorded on stream 2 since the last `stats_lines` call.
    samples_s2: Vec<i64>,
}

impl LatencyTracker {
    pub fn new(buffer_slots: u64, log_opts: LogOptions) -> Self {
        Self {
            block_time_ms: BTreeMap::new(),
            pending: BTreeMap::new(),
            highest_slot: 0,
            buffer_slots,
            log_opts,
            samples_s1: Vec::new(),
            samples_s2: Vec::new(),
        }
    }

    /// Record a transaction arrival. Emits immediately if the slot's block time
    /// is already known, otherwise buffers until blocks_meta catches up.
    pub fn on_transaction(&mut self, stream: StreamId, slot: u64, signature: String, recv_ms: i64) {
        match self.block_time_ms.get(&slot) {
            Some(&block_time_ms) => self.record_and_emit(stream, slot, &signature, recv_ms, block_time_ms),
            None => self.pending.entry(slot).or_default().push(PendingTx {
                stream,
                signature,
                recv_ms,
            }),
        }
        self.advance(slot);
    }

    /// Record a slot's block time and flush every transaction buffered for it.
    ///
    /// A slot with no `block_time` still has to release its buffered
    /// transactions — otherwise they sit until eviction and are reported as
    /// unresolved, which would misread as a stream fault.
    pub fn on_block_meta(&mut self, slot: u64, block_time_ms: Option<i64>) {
        match block_time_ms {
            Some(block_time_ms) => {
                self.block_time_ms.insert(slot, block_time_ms);
                for p in self.pending.remove(&slot).unwrap_or_default() {
                    self.record_and_emit(p.stream, slot, &p.signature, p.recv_ms, block_time_ms);
                }
            }
            None => {
                let dropped = self.pending.remove(&slot).unwrap_or_default().len();
                warn!("slot={slot} blocks_meta carried no block_time — {dropped} transaction(s) cannot be timed");
            }
        }
        self.advance(slot);
    }

    /// Add a resolved transaction's latency to that stream's percentile sample,
    /// then print its line — unless `log_transactions` is off, in which case
    /// the sample is still recorded but nothing is printed. The `[STATS]`
    /// report must see every transaction regardless of what is on screen.
    fn record_and_emit(&mut self, stream: StreamId, slot: u64, signature: &str, recv_ms: i64, block_time_ms: i64) {
        match stream {
            StreamId::One => self.samples_s1.push(recv_ms - block_time_ms),
            StreamId::Two => self.samples_s2.push(recv_ms - block_time_ms),
        }
        if self.log_opts.log_transactions {
            emit(stream, slot, signature, recv_ms, block_time_ms, self.log_opts.log_signatures);
        }
    }

    /// Build the periodic `[STATS]` lines for both streams — p50/p95/p99 over
    /// every latency recorded since the previous call — then clear the
    /// samples, so each report covers only its own interval, not a running
    /// total since startup.
    pub fn stats_lines(&mut self) -> [String; 2] {
        [
            format_stats(StreamId::One, std::mem::take(&mut self.samples_s1)),
            format_stats(StreamId::Two, std::mem::take(&mut self.samples_s2)),
        ]
    }

    /// Track the chain head and evict slots that have fallen too far behind it.
    fn advance(&mut self, slot: u64) {
        self.highest_slot = self.highest_slot.max(slot);
        let cutoff = self.highest_slot.saturating_sub(self.buffer_slots);

        // `split_off` keeps [cutoff, ..] and hands back the older half, which we drop.
        let stale_meta = self.block_time_ms.split_off(&cutoff);
        self.block_time_ms = stale_meta;

        let fresh_pending = self.pending.split_off(&cutoff);
        let stale_pending = std::mem::replace(&mut self.pending, fresh_pending);

        for (slot, txs) in stale_pending {
            warn!(
                "slot={slot} evicted after {} slots with no blocks_meta — {} transaction(s) never timed",
                self.buffer_slots,
                txs.len()
            );
        }
    }
}

/// Print one line per transaction: arrival, block execution time, and the gap.
/// The signature is included only when `log_signatures` is set — it is the
/// bulk of the line's width and is rarely needed once the latency numbers
/// themselves are what's being watched.
fn emit(
    stream: StreamId,
    slot: u64,
    signature: &str,
    recv_ms: i64,
    block_time_ms: i64,
    log_signatures: bool,
) {
    info!("{}", format_line(stream, slot, signature, recv_ms, block_time_ms, log_signatures));
}

fn format_line(
    stream: StreamId,
    slot: u64,
    signature: &str,
    recv_ms: i64,
    block_time_ms: i64,
    log_signatures: bool,
) -> String {
    let latency_ms = recv_ms - block_time_ms;
    let recv = fmt_unix_ms(recv_ms);
    let b_time = fmt_unix_ms(block_time_ms);

    if log_signatures {
        format!("[{}] slot={slot} sig={signature} recv={recv} b_time={b_time} latency={latency_ms:+}ms", stream.tag())
    } else {
        format!("[{}] slot={slot} recv={recv} b_time={b_time} latency={latency_ms:+}ms", stream.tag())
    }
}

/// Build one `[STATS]` line for a stream: sample count plus p50/p95/p99
/// latency over `samples`. `samples` need not be sorted going in.
fn format_stats(stream: StreamId, mut samples: Vec<i64>) -> String {
    if samples.is_empty() {
        return format!("[STATS] {} n=0 (no samples since last report)", stream.tag());
    }

    samples.sort_unstable();
    format!(
        "[STATS] {} n={} p50={:+}ms p95={:+}ms p99={:+}ms",
        stream.tag(),
        samples.len(),
        percentile(&samples, 50.0),
        percentile(&samples, 95.0),
        percentile(&samples, 99.0),
    )
}

/// Nearest-rank percentile: `sorted` must already be ascending. `p` is 0–100.
///
/// rank = ceil(p/100 * n), 1-indexed and clamped to the sample range — the
/// standard nearest-rank method, e.g. p99 of 100 samples is the 99th smallest.
fn percentile(sorted: &[i64], p: f64) -> i64 {
    let n = sorted.len();
    let rank = ((p / 100.0) * n as f64).ceil() as usize;
    let idx = rank.saturating_sub(1).min(n - 1);
    sorted[idx]
}

/// Format Unix milliseconds as `YYYY-MM-DDTHH:MM:SS.mmmZ`, always three
/// fractional digits (zero-padded).
fn fmt_unix_ms(unix_ms: i64) -> String {
    match chrono::DateTime::from_timestamp_millis(unix_ms) {
        Some(dt) => dt.format("%Y-%m-%dT%H:%M:%S%.3fZ").to_string(),
        None => format!("<invalid timestamp {unix_ms}>"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn formats_milliseconds_with_zero_padding() {
        // Whole second → .000, not .0
        assert_eq!(fmt_unix_ms(1_752_000_000_000), "2025-07-08T18:40:00.000Z");
        // 7 ms past the second → .007, not .7
        assert_eq!(fmt_unix_ms(1_752_000_000_007), "2025-07-08T18:40:00.007Z");
        assert_eq!(fmt_unix_ms(1_752_000_000_070), "2025-07-08T18:40:00.070Z");
    }

    #[test]
    fn transaction_before_block_meta_is_buffered_then_flushed() {
        let mut t = LatencyTracker::new(300, LogOptions::default());
        t.on_transaction(StreamId::One, 100, "sig".into(), 1_000);
        assert_eq!(t.pending.get(&100).map(Vec::len), Some(1));

        t.on_block_meta(100, Some(900));
        assert!(t.pending.get(&100).is_none(), "flushed on block_meta");
    }

    #[test]
    fn transaction_after_block_meta_emits_without_buffering() {
        let mut t = LatencyTracker::new(300, LogOptions::default());
        t.on_block_meta(100, Some(900));
        t.on_transaction(StreamId::One, 100, "sig".into(), 1_000);
        assert!(t.pending.is_empty());
    }

    #[test]
    fn missing_block_time_releases_buffered_transactions() {
        let mut t = LatencyTracker::new(300, LogOptions::default());
        t.on_transaction(StreamId::Two, 100, "sig".into(), 1_000);
        t.on_block_meta(100, None);
        assert!(t.pending.is_empty(), "must not leak until eviction");
    }

    #[test]
    fn format_line_is_one_line_with_short_labels() {
        let line = format_line(StreamId::One, 340_123_456, "5xK9aB2c", 1_752_000_000_187, 1_752_000_000_000, true);
        assert_eq!(
            line,
            "[S1] slot=340123456 sig=5xK9aB2c recv=2025-07-08T18:40:00.187Z b_time=2025-07-08T18:40:00.000Z latency=+187ms"
        );
        assert!(!line.contains('\n'), "must be a single line");
    }

    #[test]
    fn format_line_omits_signature_when_disabled() {
        let line = format_line(StreamId::Two, 340_123_456, "5xK9aB2c", 1_752_000_000_187, 1_752_000_000_000, false);
        assert_eq!(
            line,
            "[S2] slot=340123456 recv=2025-07-08T18:40:00.187Z b_time=2025-07-08T18:40:00.000Z latency=+187ms"
        );
        assert!(!line.contains("sig="), "signature must be absent");
    }

    #[test]
    fn percentile_matches_nearest_rank_method() {
        let sorted: Vec<i64> = (1..=10).collect(); // [1..10]
        assert_eq!(percentile(&sorted, 50.0), 5); // ceil(0.5*10)=5 -> sorted[4]
        assert_eq!(percentile(&sorted, 95.0), 10); // ceil(0.95*10)=10 -> sorted[9]
        assert_eq!(percentile(&sorted, 99.0), 10);
        assert_eq!(percentile(&[42], 99.0), 42); // single sample: every percentile is it
    }

    #[test]
    fn sampling_continues_when_log_transactions_is_off() {
        // The per-transaction line is what LOG_TRANSACTIONS=false is meant to
        // silence — the [STATS] report must be unaffected by it.
        let opts = LogOptions {
            log_transactions: false,
            log_signatures: true,
        };
        let mut t = LatencyTracker::new(300, opts);
        t.on_block_meta(100, Some(1_000));
        t.on_transaction(StreamId::One, 100, "sig".into(), 1_150);

        let [s1, _] = t.stats_lines();
        assert_eq!(s1, "[STATS] S1 n=1 p50=+150ms p95=+150ms p99=+150ms");
    }

    #[test]
    fn stats_lines_report_and_then_reset() {
        let mut t = LatencyTracker::new(300, LogOptions::default());
        for recv in [1_100, 1_200, 1_300] {
            t.on_block_meta(100, Some(1_000));
            t.on_transaction(StreamId::One, 100, "sig".into(), recv);
        }
        t.on_block_meta(200, Some(2_000));
        t.on_transaction(StreamId::Two, 200, "sig".into(), 2_050);

        let [s1, s2] = t.stats_lines();
        assert_eq!(s1, "[STATS] S1 n=3 p50=+200ms p95=+300ms p99=+300ms");
        assert_eq!(s2, "[STATS] S2 n=1 p50=+50ms p95=+50ms p99=+50ms");

        // A second call with no new activity must report zero, not the same
        // samples again — otherwise every report after a quiet stream would
        // repeat stale numbers forever.
        let [s1_again, s2_again] = t.stats_lines();
        assert_eq!(s1_again, "[STATS] S1 n=0 (no samples since last report)");
        assert_eq!(s2_again, "[STATS] S2 n=0 (no samples since last report)");
    }

    #[test]
    fn stale_slots_are_evicted_once_the_head_moves_on() {
        let mut t = LatencyTracker::new(10, LogOptions::default());
        t.on_transaction(StreamId::One, 100, "old".into(), 1_000);
        t.on_block_meta(100, Some(900));

        // Head jumps well past the buffer window.
        t.on_transaction(StreamId::One, 200, "new".into(), 2_000);

        assert!(t.block_time_ms.get(&100).is_none(), "old meta evicted");
        assert!(t.block_time_ms.is_empty() || t.block_time_ms.contains_key(&200));
        assert_eq!(t.pending.get(&200).map(Vec::len), Some(1), "head slot kept");
    }
}
