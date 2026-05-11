use std::collections::HashSet;

/// A single slot worth of data collected from the gRPC stream.
#[derive(Debug, Default)]
pub struct SlotData {
    pub slot: u64,
    /// Signatures seen via the gRPC transaction stream for this slot.
    pub grpc_signatures: HashSet<String>,
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
}

impl ReconcileReport {
    pub fn is_clean(&self) -> bool {
        self.missed.is_empty() && self.extra.is_empty()
    }
}
