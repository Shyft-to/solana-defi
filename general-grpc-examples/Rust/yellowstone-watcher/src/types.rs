use std::collections::{HashMap, HashSet};

/// A single slot worth of data collected from the gRPC stream.
#[derive(Debug, Default)]
pub struct SlotData {
    pub slot: u64,
    /// Signatures seen via the gRPC transaction stream for this slot.
    pub grpc_signatures: HashSet<String>,
    /// Unix millisecond timestamp of when each signature was received from gRPC.
    pub received_at_ms: HashMap<String, u64>,
}

/// Outcome of a reconciliation run for one slot.
#[derive(Debug)]
pub struct ReconcileReport {
    pub slot: u64,
    pub grpc_count: usize,
    pub rpc_count: usize,
    /// Signatures present in RPC result but absent from the gRPC stream.
    pub missed: Vec<String>,
    /// Signatures seen by gRPC but not (yet?) confirmed by RPC — unusual; logged as warnings.
    pub extra: Vec<String>,
    /// Per-signature latency: (signature, latency_ms) for every matched transaction.
    /// Latency = time gRPC delivered it − block production time reported by RPC.
    pub latencies: Vec<(String, i64)>,
    /// Latency stats derived from `latencies`. None if block_time was unavailable.
    pub latency_min_ms: Option<i64>,
    pub latency_max_ms: Option<i64>,
    pub latency_avg_ms: Option<i64>,
}

impl ReconcileReport {
    pub fn is_clean(&self) -> bool {
        self.missed.is_empty() && self.extra.is_empty()
    }
}
