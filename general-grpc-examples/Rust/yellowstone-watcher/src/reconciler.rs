use std::collections::HashSet;

use anyhow::{Context, Result};
use solana_client::{
    nonblocking::rpc_client::RpcClient,
    rpc_client::GetConfirmedSignaturesForAddress2Config,
};
use solana_sdk::{
    commitment_config::{CommitmentConfig, CommitmentLevel},
    pubkey::Pubkey,
};
use tracing::{debug, warn};

use crate::{
    config::Config,
    types::{ReconcileReport, SlotData},
};

/// Verify that the `grpc_sigs` set matches what the RPC reports for `slot`.
///
/// Uses `getSignaturesForAddress` for every watched account, filtered to the slot.
pub async fn reconcile(
    client: &RpcClient,
    cfg: &Config,
    slot_data: &SlotData,
) -> Result<ReconcileReport> {
    let slot = slot_data.slot;
    let grpc_set: &HashSet<String> = &slot_data.grpc_signatures;

    let rpc_set = fetch_rpc_signatures(client, cfg, slot).await?;

    let missed: Vec<String> = rpc_set
        .difference(grpc_set)
        .cloned()
        .collect();

    let extra: Vec<String> = grpc_set
        .difference(&rpc_set)
        .cloned()
        .collect();

    let report = ReconcileReport {
        slot,
        grpc_count: grpc_set.len(),
        rpc_count: rpc_set.len(),
        missed,
        extra,
    };

    Ok(report)
}

/// Fetch all relevant transaction signatures for `slot` from the RPC.
///
/// Returns a [`HashSet`] of base-58 signature strings.
async fn fetch_rpc_signatures(
    client: &RpcClient,
    cfg: &Config,
    slot: u64,
) -> Result<HashSet<String>> {
    let mut combined: HashSet<String> = HashSet::new();

    for addr_str in &cfg.account_include {
        match addr_str.parse::<Pubkey>() {
            Ok(pubkey) => {
                match fetch_via_signatures_for_address(
                    client,
                    &pubkey,
                    slot,
                    cfg.rpc_signatures_limit,
                )
                .await
                {
                    Ok(sigs) => {
                        debug!(
                            "getSignaturesForAddress({addr_str}, slot={slot}) → {} sigs",
                            sigs.len()
                        );
                        combined.extend(sigs);
                    }
                    Err(e) => {
                        warn!("getSignaturesForAddress({addr_str}, slot={slot}) failed: {e}");
                    }
                }
            }
            Err(e) => {
                warn!("Invalid pubkey in account_include ({addr_str}): {e}");
            }
        }
    }

    Ok(combined)
}

/// Fetch transaction signatures touching `pubkey` in exactly `slot`.
async fn fetch_via_signatures_for_address(
    client: &RpcClient,
    pubkey: &Pubkey,
    slot: u64,
    limit: usize,
) -> Result<Vec<String>> {
    let config = GetConfirmedSignaturesForAddress2Config {
        before: None,
        until: None,
        limit: Some(limit),
        commitment: Some(CommitmentConfig {
            commitment: CommitmentLevel::Confirmed,
        }),
    };

    let results = client
        .get_signatures_for_address_with_config(pubkey, config)
        .await
        .context("getSignaturesForAddress RPC call failed")?;

    // Filter to the exact slot and exclude failed transactions
    let sigs: Vec<String> = results
        .into_iter()
        .filter(|s| s.slot == slot && s.err.is_none())
        .map(|s| s.signature)
        .collect();

    Ok(sigs)
}
