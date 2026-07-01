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
                    println!("  gRPC (slot {grpc_slot}): {}", fmt_bytes(&grpc_bytes));
                    println!("  RPC  (slot {context_slot}): {}", fmt_bytes(&rpc_bytes));
                }
            }
        }
    }
}

fn fmt_bytes(bytes: &[u8]) -> String {
    let preview: String = bytes[..bytes.len().min(64)]
        .iter()
        .map(|b| format!("{b:02x}"))
        .collect();
    if bytes.len() > 64 {
        format!("{preview}… ({} bytes total)", bytes.len())
    } else {
        format!("{preview} ({} bytes)", bytes.len())
    }
}
