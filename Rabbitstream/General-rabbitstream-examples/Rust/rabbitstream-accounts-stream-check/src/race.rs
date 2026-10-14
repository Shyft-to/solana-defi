//! Correlates two independent Yellowstone gRPC streams — a transaction
//! stream and an account-update stream — that should both eventually report
//! the same signature, and measures which one sees it first.
//!
//! Each stream calls [`RaceTracker::record_tx`] / [`RaceTracker::record_account`]
//! with the signature and the `Instant` it arrived. The first call for a given
//! signature opens a pending entry; the second call (from the other stream)
//! closes it out and records who won and by how much. A signature that only
//! ever shows up on one side is finalized as that stream's exclusive miss,
//! either by [`RaceTracker::sweep`] once it has waited longer than the match
//! timeout, or by [`RaceTracker::flush_all`] at run end.

use std::{
    collections::{HashMap, HashSet},
    sync::{
        atomic::{AtomicU64, Ordering},
        Mutex,
    },
    time::{Duration, Instant, SystemTime},
};

/// Which stream reported a given signature first.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Winner {
    TxFirst,
    AccountFirst,
    /// Both arrivals landed at the same `Instant` — vanishingly rare, but
    /// possible depending on timer resolution.
    Tie,
}

struct MatchResult {
    winner: Winner,
    delta: Duration,
}

/// A signature seen on one stream so far, waiting on the other.
struct Pending {
    tx_at: Option<Instant>,
    account_at: Option<Instant>,
    first_seen: Instant,
}

#[derive(Default)]
struct State {
    pending: HashMap<String, Pending>,
    /// Signatures whose race has already been settled — matched, or
    /// already counted as a stream-exclusive miss. A watched `owner`
    /// program commonly has one transaction write several of its accounts,
    /// so the account stream can emit multiple `SubscribeUpdateAccount`
    /// messages carrying the same `txn_signature`. Once a signature is
    /// settled, any further update for it (from either stream) is a
    /// duplicate, not a new race — without this guard it would reopen a
    /// phantom `pending` entry that can never complete (its one-and-only
    /// counterpart on the other stream already fired), and that phantom
    /// would eventually be misclassified as a stream-exclusive miss even
    /// though the signature was, in fact, seen and matched on both streams.
    resolved: HashSet<String>,
}

pub struct RaceTracker {
    state: Mutex<State>,
    matched: Mutex<Vec<MatchResult>>,
    tx_only: AtomicU64,
    account_only: AtomicU64,
    tx_only_sigs: Mutex<Vec<(String, SystemTime)>>,
    account_only_sigs: Mutex<Vec<(String, SystemTime)>>,
    warmup_skipped: AtomicU64,
    // Anchor pair (captured together, at construction) used to convert a
    // `Pending::first_seen` `Instant` — monotonic, not a calendar time — into
    // a wall-clock `SystemTime` for display, without threading a second
    // timestamp through every `record_tx`/`record_account` call.
    run_start_instant: Instant,
    run_start_wall: SystemTime,
    // Signatures first seen before this instant that only ever show up on
    // one stream are startup artifacts, not genuine misses — see `record`.
    warmup_until: Instant,
}

impl RaceTracker {
    /// `warmup` is how long after construction to treat a one-sided
    /// signature as connection-startup skew rather than a genuine miss: the
    /// two streams connect independently and neither backfills, so whichever
    /// one finishes subscribing first can briefly see chain activity the
    /// other — not yet listening — can never be shown. Real matches inside
    /// the window still count normally; only the *miss* classification is
    /// suppressed.
    pub fn new(warmup: Duration) -> Self {
        let run_start_instant = Instant::now();
        Self {
            state: Mutex::new(State::default()),
            matched: Mutex::new(Vec::new()),
            tx_only: AtomicU64::new(0),
            account_only: AtomicU64::new(0),
            tx_only_sigs: Mutex::new(Vec::new()),
            account_only_sigs: Mutex::new(Vec::new()),
            warmup_skipped: AtomicU64::new(0),
            run_start_instant,
            run_start_wall: SystemTime::now(),
            warmup_until: run_start_instant + warmup,
        }
    }

    /// Convert a `Pending::first_seen` `Instant` to the wall-clock moment it
    /// corresponds to, via the anchor pair captured at construction.
    fn to_wall(&self, at: Instant) -> SystemTime {
        match at.checked_duration_since(self.run_start_instant) {
            Some(elapsed) => self.run_start_wall + elapsed,
            None => self.run_start_wall - self.run_start_instant.duration_since(at),
        }
    }

    pub fn record_tx(&self, sig: &str, at: Instant) {
        self.record(sig, at, true);
    }

    pub fn record_account(&self, sig: &str, at: Instant) {
        self.record(sig, at, false);
    }

    fn record(&self, sig: &str, at: Instant, from_tx: bool) {
        let finalized = {
            let mut state = self.state.lock().unwrap();

            // Already settled — a duplicate update for a signature whose
            // race is over. See `State::resolved` for why this matters.
            if state.resolved.contains(sig) {
                return;
            }

            let entry = state.pending.entry(sig.to_owned()).or_insert_with(|| Pending {
                tx_at: None,
                account_at: None,
                first_seen: at,
            });

            // Keep the earliest arrival if a stream ever redelivers a signature.
            let slot = if from_tx { &mut entry.tx_at } else { &mut entry.account_at };
            if slot.is_none() {
                *slot = Some(at);
            }

            match (entry.tx_at, entry.account_at) {
                (Some(tx_at), Some(account_at)) => {
                    state.pending.remove(sig);
                    state.resolved.insert(sig.to_owned());
                    Some(finalize(tx_at, account_at))
                }
                _ => None,
            }
        };

        if let Some(result) = finalized {
            self.matched.lock().unwrap().push(result);
        }
    }

    /// Finalize any pending signature that has waited longer than `timeout`
    /// without a match on the other stream, counting it as that stream's
    /// exclusive miss.
    pub fn sweep(&self, timeout: Duration) {
        let now = Instant::now();
        let mut state = self.state.lock().unwrap();
        let stale_sigs: Vec<String> = state
            .pending
            .iter()
            .filter(|(_, p)| now.duration_since(p.first_seen) >= timeout)
            .map(|(sig, _)| sig.clone())
            .collect();

        let mut stale = Vec::with_capacity(stale_sigs.len());
        for sig in stale_sigs {
            if let Some(entry) = state.pending.remove(&sig) {
                state.resolved.insert(sig.clone());
                stale.push((sig, entry));
            }
        }
        drop(state);

        for (sig, entry) in &stale {
            self.count_miss(sig, entry);
        }
    }

    /// At run end: finalize every remaining pending signature immediately,
    /// regardless of how long it has been waiting.
    pub fn flush_all(&self) {
        let mut state = self.state.lock().unwrap();
        let stale: Vec<(String, Pending)> = state.pending.drain().collect();
        for (sig, _) in &stale {
            state.resolved.insert(sig.clone());
        }
        drop(state);

        for (sig, entry) in &stale {
            self.count_miss(sig, entry);
        }
    }

    fn count_miss(&self, sig: &str, entry: &Pending) {
        // A one-sided signature that first showed up during the warm-up
        // window is more likely a startup connection-skew artifact (one
        // stream subscribed before the other) than a genuine miss — don't
        // hold it against either stream.
        if entry.first_seen < self.warmup_until {
            self.warmup_skipped.fetch_add(1, Ordering::Relaxed);
            return;
        }

        // `first_seen` is set when the entry is first opened, i.e. by
        // whichever side actually arrived — so for a one-sided entry it's
        // exactly that side's arrival time.
        let at = self.to_wall(entry.first_seen);
        match (entry.tx_at, entry.account_at) {
            (Some(_), None) => {
                self.tx_only.fetch_add(1, Ordering::Relaxed);
                self.tx_only_sigs.lock().unwrap().push((sig.to_owned(), at));
            }
            (None, Some(_)) => {
                self.account_only.fetch_add(1, Ordering::Relaxed);
                self.account_only_sigs.lock().unwrap().push((sig.to_owned(), at));
            }
            // Both-set entries are finalized immediately in `record`, so a
            // pending entry reaching here is always one-sided.
            _ => {}
        }
    }

    pub fn summary(&self) -> RaceSummary {
        let matched = self.matched.lock().unwrap();
        let total = matched.len();
        let pct = |n: usize| {
            if total > 0 {
                n as f64 / total as f64 * 100.0
            } else {
                0.0
            }
        };

        // Lead-time percentiles are computed per winner: "when the tx stream
        // won, how far ahead was it" and "when the account stream won, how
        // far ahead was it" are different distributions, not one pooled one.
        let mut tx_deltas_ms: Vec<f64> = Vec::new();
        let mut account_deltas_ms: Vec<f64> = Vec::new();
        let mut ties = 0usize;

        for m in matched.iter() {
            let ms = m.delta.as_secs_f64() * 1000.0;
            match m.winner {
                Winner::TxFirst => tx_deltas_ms.push(ms),
                Winner::AccountFirst => account_deltas_ms.push(ms),
                Winner::Tie => ties += 1,
            }
        }
        tx_deltas_ms.sort_by(|a, b| a.partial_cmp(b).unwrap());
        account_deltas_ms.sort_by(|a, b| a.partial_cmp(b).unwrap());

        RaceSummary {
            matched: total,
            ties,
            tx_only: self.tx_only.load(Ordering::Relaxed),
            account_only: self.account_only.load(Ordering::Relaxed),
            tx: StreamRace {
                ahead: tx_deltas_ms.len(),
                ahead_pct: pct(tx_deltas_ms.len()),
                p50_ms: percentile(&tx_deltas_ms, 50.0),
                p95_ms: percentile(&tx_deltas_ms, 95.0),
                p99_ms: percentile(&tx_deltas_ms, 99.0),
            },
            account: StreamRace {
                ahead: account_deltas_ms.len(),
                ahead_pct: pct(account_deltas_ms.len()),
                p50_ms: percentile(&account_deltas_ms, 50.0),
                p95_ms: percentile(&account_deltas_ms, 95.0),
                p99_ms: percentile(&account_deltas_ms, 99.0),
            },
            tx_only_sigs: self.tx_only_sigs.lock().unwrap().clone(),
            account_only_sigs: self.account_only_sigs.lock().unwrap().clone(),
            warmup_skipped: self.warmup_skipped.load(Ordering::Relaxed),
        }
    }
}

fn finalize(tx_at: Instant, account_at: Instant) -> MatchResult {
    if tx_at < account_at {
        MatchResult {
            winner: Winner::TxFirst,
            delta: account_at - tx_at,
        }
    } else if account_at < tx_at {
        MatchResult {
            winner: Winner::AccountFirst,
            delta: tx_at - account_at,
        }
    } else {
        MatchResult {
            winner: Winner::Tie,
            delta: Duration::ZERO,
        }
    }
}

/// Linear-interpolation percentile over an already-sorted slice (in ms).
fn percentile(sorted: &[f64], p: f64) -> f64 {
    match sorted.len() {
        0 => 0.0,
        1 => sorted[0],
        len => {
            let rank = (p / 100.0) * (len - 1) as f64;
            let lo = rank.floor() as usize;
            let hi = rank.ceil() as usize;
            if lo == hi {
                sorted[lo]
            } else {
                sorted[lo] + (sorted[hi] - sorted[lo]) * (rank - lo as f64)
            }
        }
    }
}

/// How one stream fared in the arrival race: how often it was the first to
/// report a signature, and — restricted to just the races it won — how far
/// ahead of the other stream it was.
pub struct StreamRace {
    pub ahead: usize,
    pub ahead_pct: f64,
    pub p50_ms: f64,
    pub p95_ms: f64,
    pub p99_ms: f64,
}

pub struct RaceSummary {
    pub matched: usize,
    pub ties: usize,
    pub tx_only: u64,
    pub account_only: u64,
    pub tx: StreamRace,
    pub account: StreamRace,
    /// Signatures seen only on the tx stream, with the wall-clock time each
    /// was first seen — the account stream never reported a matching update
    /// for them. Tracked for symmetry; not currently printed anywhere.
    #[allow(dead_code)]
    pub tx_only_sigs: Vec<(String, SystemTime)>,
    /// Signatures seen only on the account stream, with the wall-clock time
    /// each was first seen — the tx stream never reported them (this is
    /// what "accounts saw it, tx feed missed it" means).
    pub account_only_sigs: Vec<(String, SystemTime)>,
    /// One-sided signatures first seen during the warm-up window — excluded
    /// from `tx_only`/`account_only` as likely startup connection skew
    /// rather than genuine misses.
    pub warmup_skipped: u64,
}

#[cfg(test)]
mod tests {
    use super::*;

    /// One transaction that writes several accounts under a watched `owner`
    /// program produces one tx-stream message but multiple account-stream
    /// messages, all sharing the same signature. The extra account updates
    /// must not reopen the signature as a fresh (and unwinnable) race after
    /// it has already matched.
    #[test]
    fn duplicate_account_update_after_match_is_not_a_miss() {
        let tracker = RaceTracker::new(Duration::ZERO);
        let sig = "dupe-sig";
        let t0 = Instant::now();

        tracker.record_account(sig, t0);
        tracker.record_tx(sig, t0 + Duration::from_millis(5));
        // A second account write for the same transaction, arriving after
        // the race above already resolved.
        tracker.record_account(sig, t0 + Duration::from_millis(50));

        // Nothing left pending to time out later.
        tracker.sweep(Duration::from_secs(0));
        tracker.flush_all();

        let summary = tracker.summary();
        assert_eq!(summary.matched, 1);
        assert_eq!(summary.tx_only, 0);
        assert_eq!(summary.account_only, 0);
        assert!(summary.account_only_sigs.is_empty());
    }

    #[test]
    fn genuine_account_only_miss_is_still_reported() {
        let tracker = RaceTracker::new(Duration::ZERO);
        tracker.record_account("orphan-sig", Instant::now());
        tracker.flush_all();

        let summary = tracker.summary();
        assert_eq!(summary.account_only, 1);
        assert_eq!(summary.account_only_sigs.len(), 1);
        assert_eq!(summary.account_only_sigs[0].0, "orphan-sig");
    }

    /// A one-sided signature seen while one stream hasn't finished
    /// connecting yet is startup skew, not a genuine miss: it must not be
    /// counted in `tx_only`/`account_only`, but should show up in
    /// `warmup_skipped` instead.
    #[test]
    fn miss_during_warmup_is_skipped_not_counted() {
        let tracker = RaceTracker::new(Duration::from_secs(3600));
        tracker.record_account("startup-skew-sig", Instant::now());
        tracker.flush_all();

        let summary = tracker.summary();
        assert_eq!(summary.account_only, 0);
        assert!(summary.account_only_sigs.is_empty());
        assert_eq!(summary.warmup_skipped, 1);
    }
}
