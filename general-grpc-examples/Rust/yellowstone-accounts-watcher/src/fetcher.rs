use std::sync::Arc;

use dashmap::DashMap;

/// (slot, pubkey) → raw account data bytes as delivered by the gRPC stream.
/// The latest gRPC update for a given slot wins (overwrites on multiple updates).
pub type GrpcDataMap = Arc<DashMap<(u64, String), Vec<u8>>>;
