use dashmap::DashMap;
use std::sync::Arc;
use tracing::{debug, info};

use crate::types::SlotData;

/// Thread-safe store for in-flight slot data.
///
/// Keys are slot numbers.  Entries are created on first transaction or slot
/// event and removed after reconciliation.
#[derive(Clone, Default)]
pub struct SlotTracker {
    slots: Arc<DashMap<u64, SlotData>>,
}

impl SlotTracker {
    pub fn new() -> Self {
        Self {
            slots: Arc::new(DashMap::new()),
        }
    }

    /// Record that a transaction with `signature` was seen in `slot`.
    pub fn record_transaction(&self, slot: u64, signature: String) {
        let mut entry = self.slots.entry(slot).or_insert_with(|| SlotData {
            slot,
            ..Default::default()
        });
        entry.grpc_signatures.insert(signature);
        debug!("tracker: slot={slot} sigs={}", entry.grpc_signatures.len());
    }

    /// Take and remove slot data for reconciliation.
    /// Returns `None` if we never saw any transactions for this slot
    /// (that is fine — nothing to reconcile).
    pub fn take(&self, slot: u64) -> Option<SlotData> {
        self.slots.remove(&slot).map(|(_, v)| v)
    }

    /// Number of slots currently tracked (diagnostic).
    pub fn len(&self) -> usize {
        self.slots.len()
    }

    /// Log a snapshot of every tracked slot and its signature count.
    pub fn dump(&self) {
        let mut entries: Vec<(u64, usize)> = self
            .slots
            .iter()
            .map(|e| (*e.key(), e.value().grpc_signatures.len()))
            .collect();
        entries.sort_by_key(|(slot, _)| *slot);
        info!("Buffering {} slot(s) waiting for reconciliation:", entries.len());
        for (slot, count) in entries {
            info!("  Slot {slot} — {count} transaction(s) collected so far");
        }
    }
}
