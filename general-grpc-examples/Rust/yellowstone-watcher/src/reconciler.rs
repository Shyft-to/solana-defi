use std::collections::HashMap;
use std::collections::HashSet;
use std::time::Duration;

use anyhow::{Context, Result};
use solana_client::{
    nonblocking::rpc_client::RpcClient,
    rpc_client::GetConfirmedSignaturesForAddress2Config,
    rpc_config::RpcBlockConfig,
};
use solana_sdk::{
    commitment_config::{CommitmentConfig, CommitmentLevel},
    pubkey::Pubkey,
    signature::Signature,
};
use solana_transaction_status::{
    EncodedTransaction, TransactionDetails, UiMessage, UiTransactionEncoding,
};
use tracing::{debug, error, warn};

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

    // Wait before querying the RPC so the node has time to index the block.
    if cfg.rpc_delay_secs > 0 {
        tokio::time::sleep(Duration::from_secs(cfg.rpc_delay_secs)).await;
    }

    // sig -> block_time_ms from RPC
    let rpc_map = fetch_rpc_signatures(client, cfg, slot).await?;
    let rpc_set: HashSet<String> = rpc_map.keys().cloned().collect();

    let missed: Vec<String> = rpc_set.difference(grpc_set).cloned().collect();
    let extra: Vec<String> = grpc_set.difference(&rpc_set).cloned().collect();

    // Compute per-signature latency for transactions seen by both sides.
    let mut latencies: Vec<(String, i64)> = grpc_set
        .intersection(&rpc_set)
        .filter_map(|sig| {
            let received_at = *slot_data.received_at_ms.get(sig)?;
            let block_time_ms = rpc_map.get(sig)?.as_ref()?;
            Some((sig.clone(), received_at as i64 - block_time_ms))
        })
        .collect();
    latencies.sort_by_key(|(_, lat)| *lat);

    let (latency_min_ms, latency_max_ms, latency_avg_ms) = if latencies.is_empty() {
        (None, None, None)
    } else {
        let min = latencies.first().map(|(_, l)| *l);
        let max = latencies.last().map(|(_, l)| *l);
        let avg = Some(latencies.iter().map(|(_, l)| l).sum::<i64>() / latencies.len() as i64);
        (min, max, avg)
    };

    let report = ReconcileReport {
        slot,
        grpc_count: grpc_set.len(),
        rpc_count: rpc_set.len(),
        missed,
        extra,
        latencies,
        latency_min_ms,
        latency_max_ms,
        latency_avg_ms,
    };

    Ok(report)
}

/// Fetch all relevant transaction signatures for `slot` from the RPC.
///
/// Delegates to `getBlock` or `getSignaturesForAddress` depending on `cfg.use_get_block`.
/// Returns a map of signature → block_time_ms (None if block_time unavailable).
async fn fetch_rpc_signatures(
    client: &RpcClient,
    cfg: &Config,
    slot: u64,
) -> Result<HashMap<String, Option<i64>>> {
    if cfg.use_get_block {
        return fetch_via_get_block(client, cfg, slot).await;
    }

    let mut combined: HashMap<String, Option<i64>> = HashMap::new();

    for addr_str in &cfg.account_include {
        match addr_str.parse::<Pubkey>() {
            Ok(pubkey) => {
                match fetch_via_signatures_for_address(client, &pubkey, slot, cfg.rpc_signatures_limit).await {
                    Ok(sigs) => {
                        debug!(
                            "getSignaturesForAddress({addr_str}, slot={slot}) → {} sigs",
                            sigs.len()
                        );
                        combined.extend(sigs);
                    }
                    Err(e) => {
                        error!("getSignaturesForAddress failed for address={addr_str} slot={slot}: {e:#}");
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

/// Fetch ALL transaction signatures touching `pubkey` in exactly `slot`.
///
/// Paginates through pages (newest-first) until the results go older than
/// `slot`, guaranteeing no transactions are missed due to the page size limit.
///
/// Returns a map of signature → block_time in milliseconds (None if unavailable).
async fn fetch_via_signatures_for_address(
    client: &RpcClient,
    pubkey: &Pubkey,
    slot: u64,
    page_size: usize,
) -> Result<HashMap<String, Option<i64>>> {
    let commitment = Some(CommitmentConfig {
        commitment: CommitmentLevel::Confirmed,
    });

    let mut sigs: HashMap<String, Option<i64>> = HashMap::new();
    let mut before: Option<Signature> = None;

    loop {
        let config = GetConfirmedSignaturesForAddress2Config {
            before,
            until: None,
            limit: Some(page_size),
            commitment,
        };

        let page = client
            .get_signatures_for_address_with_config(pubkey, config)
            .await
            .context("getSignaturesForAddress RPC call failed")?;

        if page.is_empty() {
            break;
        }

        let mut passed_target = false;

        for item in &page {
            if item.slot == slot && item.err.is_none() {
                // block_time is Unix seconds — convert to milliseconds
                let block_time_ms = item.block_time.map(|t| t * 1000);
                sigs.insert(item.signature.clone(), block_time_ms);
            }
            if item.slot < slot {
                passed_target = true;
                break;
            }
        }

        if passed_target {
            break;
        }

        before = page
            .last()
            .and_then(|item| item.signature.parse::<Signature>().ok());

        if before.is_none() {
            break;
        }
    }

    Ok(sigs)
}

/// Fetch transaction signatures for `slot` using `getBlock`, then filter down
/// to only the transactions that touch at least one watched account.
///
/// Mirrors the gRPC filters: failed transactions and vote transactions are excluded.
async fn fetch_via_get_block(
    client: &RpcClient,
    cfg: &Config,
    slot: u64,
) -> Result<HashMap<String, Option<i64>>> {
    let block = client
        .get_block_with_config(
            slot,
            RpcBlockConfig {
                encoding: Some(UiTransactionEncoding::JsonParsed),
                transaction_details: Some(TransactionDetails::Full),
                rewards: Some(false),
                commitment: Some(CommitmentConfig {
                    commitment: CommitmentLevel::Confirmed,
                }),
                max_supported_transaction_version: Some(0),
            },
        )
        .await
        .context("getBlock RPC call failed")?;

    let block_time_ms = block.block_time.map(|t| t * 1000);
    let account_set: HashSet<&str> = cfg.account_include.iter().map(|s| s.as_str()).collect();

    const VOTE_PROGRAM: &str = "Vote111111111111111111111111111111111111111";
    let mut result = HashMap::new();

    for tx_with_meta in block.transactions.unwrap_or_default() {
        // Mirror gRPC filter: failed=false
        if tx_with_meta
            .meta
            .as_ref()
            .and_then(|m| m.err.as_ref())
            .is_some()
        {
            continue;
        }

        let ui_tx = match &tx_with_meta.transaction {
            EncodedTransaction::Json(tx) => tx,
            _ => continue,
        };

        let sig = match ui_tx.signatures.first() {
            Some(s) => s.clone(),
            None => continue,
        };

        let accounts: Vec<String> = match &ui_tx.message {
            UiMessage::Parsed(m) => m.account_keys.iter().map(|k| k.pubkey.clone()).collect(),
            UiMessage::Raw(m) => m.account_keys.clone(),
        };

        // Mirror gRPC filter: vote=false
        if accounts.iter().any(|a| a == VOTE_PROGRAM) {
            continue;
        }

        // Only keep transactions touching a watched account
        if !accounts.iter().any(|a| account_set.contains(a.as_str())) {
            continue;
        }

        result.insert(sig, block_time_ms);
    }

    Ok(result)
}
