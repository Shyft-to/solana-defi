use std::time::Duration;

use solana_client::nonblocking::rpc_client::RpcClient;
use solana_sdk::commitment_config::CommitmentConfig;
use solana_sdk::pubkey::Pubkey;
use tracing::warn;

use crate::config::Commitment;
use crate::fetcher::GrpcDataMap;

// Extra delay after SLOT_FINALIZED before calling getAccountInfo, to let the RPC node catch up.
const RPC_LAG_SECS: u64 = 2;

pub struct Verifier {
    rpc: RpcClient,
    rpc_commitment: Commitment,
    pubkey: Pubkey,
    pubkey_str: String,
    grpc_data: GrpcDataMap,
}

impl Verifier {
    pub fn new(
        rpc_url: &str,
        rpc_commitment: Commitment,
        target_pubkey: String,
        grpc_data: GrpcDataMap,
    ) -> Self {
        let commitment_config = match rpc_commitment {
            Commitment::Confirmed => CommitmentConfig::confirmed(),
            Commitment::Finalized => CommitmentConfig::finalized(),
        };
        Self {
            rpc: RpcClient::new_with_commitment(rpc_url.to_owned(), commitment_config),
            rpc_commitment,
            pubkey: target_pubkey.parse().expect("TARGET_PUBKEY is not a valid base58 pubkey"),
            pubkey_str: target_pubkey,
            grpc_data,
        }
    }

    pub async fn verify_slot(&self, slot: u64) {
        tokio::time::sleep(Duration::from_secs(RPC_LAG_SECS)).await;

        let commitment_config = match self.rpc_commitment {
            Commitment::Confirmed => CommitmentConfig::confirmed(),
            Commitment::Finalized => CommitmentConfig::finalized(),
        };

        let resp = match self.rpc.get_account_with_commitment(&self.pubkey, commitment_config).await {
            Ok(r) => r,
            Err(e) => {
                warn!("SLOT {slot} | getAccountInfo failed: {e:#}");
                return;
            }
        };

        let context_slot = resp.context.slot;
        let rpc_bytes: Vec<u8> = resp.value.map(|a| a.data).unwrap_or_default();

        // Find the most recent gRPC entry for this pubkey at or before context_slot.
        // With 200 entries max, the linear scan is negligible.
        let latest_grpc = self
            .grpc_data
            .iter()
            .filter(|e| e.key().0 <= context_slot && e.key().1 == self.pubkey_str)
            .max_by_key(|e| e.key().0)
            .map(|e| (e.key().0, e.value().clone()));

        match latest_grpc {
            None => {
                warn!(
                    "SLOT {slot} | context_slot={context_slot} | no gRPC data in map yet — skipping"
                );
            }
            Some((grpc_slot, grpc_bytes)) => {
                if grpc_bytes.as_slice() == rpc_bytes.as_slice() {
                    if grpc_slot == context_slot {
                        println!(
                            "SLOT {slot} | context_slot={context_slot} | gRPC update present for this slot, data matches RPC | OK"
                        );
                    } else {
                        println!(
                            "SLOT {slot} | context_slot={context_slot} | data matches RPC | last update received at slot {grpc_slot} | OK"
                        );
                    }
                } else {
                    println!(
                        "SLOT {slot} | context_slot={context_slot} | MISS — gRPC data (from slot {grpc_slot}) does not match RPC"
                    );
                    println!("  gRPC (slot {grpc_slot}) — {} bytes:", grpc_bytes.len());
                    print_hex_dump(&grpc_bytes, "    ");
                    println!("  RPC  (slot {context_slot}) — {} bytes:", rpc_bytes.len());
                    print_hex_dump(&rpc_bytes, "    ");
                    print_diff(&grpc_bytes, &rpc_bytes);
                }
            }
        }
    }
}

/// Print a full hex dump, 16 bytes per line with offset prefix.
fn print_hex_dump(bytes: &[u8], indent: &str) {
    for (i, chunk) in bytes.chunks(16).enumerate() {
        let offset = i * 16;
        let hex: String = chunk
            .iter()
            .map(|b| format!("{b:02x}"))
            .collect::<Vec<_>>()
            .join(" ");
        println!("{indent}{offset:04x}  {hex}");
    }
}

/// Find contiguous ranges of differing bytes and print them.
fn print_diff(a: &[u8], b: &[u8]) {
    let len = a.len().max(b.len());
    let mut ranges: Vec<(usize, usize)> = Vec::new();
    let mut in_diff = false;
    let mut start = 0;

    for i in 0..len {
        let byte_a = a.get(i).copied();
        let byte_b = b.get(i).copied();
        if byte_a != byte_b {
            if !in_diff {
                start = i;
                in_diff = true;
            }
        } else if in_diff {
            ranges.push((start, i));
            in_diff = false;
        }
    }
    if in_diff {
        ranges.push((start, len));
    }

    if ranges.is_empty() {
        return;
    }

    println!(
        "  Diff — {} changed range(s), {} byte(s) total:",
        ranges.len(),
        ranges.iter().map(|(s, e)| e - s).sum::<usize>()
    );
    for (start, end) in ranges {
        let grpc_chunk: String = a[start..end.min(a.len())]
            .iter()
            .map(|b| format!("{b:02x}"))
            .collect::<Vec<_>>()
            .join(" ");
        let rpc_chunk: String = b[start..end.min(b.len())]
            .iter()
            .map(|b| format!("{b:02x}"))
            .collect::<Vec<_>>()
            .join(" ");
        println!(
            "    offset 0x{start:04x}-0x{:04x} ({} byte(s))",
            end - 1,
            end - start
        );
        println!("      gRPC: {grpc_chunk}");
        println!("      RPC : {rpc_chunk}");
    }
}
