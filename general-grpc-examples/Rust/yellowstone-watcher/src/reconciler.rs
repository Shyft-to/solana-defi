use std::collections::HashSet;

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
/// Delegates to `getBlock` or `getSignaturesForAddress` depending on `cfg.use_get_block`.
async fn fetch_rpc_signatures(
    client: &RpcClient,
    cfg: &Config,
    slot: u64,
) -> Result<HashSet<String>> {
    if cfg.use_get_block {
        return fetch_via_get_block(client, cfg, slot).await;
    }

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

/// Fetch ALL transaction signatures touching `pubkey` in exactly `slot`.
///
/// Paginates through pages (newest-first) until the results go older than
/// `slot`, guaranteeing no transactions are missed due to the page size limit.
async fn fetch_via_signatures_for_address(
    client: &RpcClient,
    pubkey: &Pubkey,
    slot: u64,
    page_size: usize,
) -> Result<Vec<String>> {
    let commitment = Some(CommitmentConfig {
        commitment: CommitmentLevel::Confirmed,
    });

    let mut sigs: Vec<String> = Vec::new();
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
                sigs.push(item.signature.clone());
            }
            // Results are newest-first. Once we see a slot older than the
            // target we have scrolled past it — no need to fetch more pages.
            if item.slot < slot {
                passed_target = true;
                break;
            }
        }

        if passed_target {
            break;
        }

        // Prepare the cursor for the next page: start just before the oldest
        // signature returned on this page.
        before = page
            .last()
            .and_then(|item| item.signature.parse::<Signature>().ok());

        if before.is_none() {
            break;
        }
    }

    Ok(sigs)
}

/// Fetch transaction signatures for `slot` using `getBlock`, filtered to
/// transactions that touch at least one watched account.
///
/// Mirrors the gRPC filters: failed transactions and vote transactions are excluded.
async fn fetch_via_get_block(
    client: &RpcClient,
    cfg: &Config,
    slot: u64,
) -> Result<HashSet<String>> {
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

    let account_set: HashSet<&str> = cfg.account_include.iter().map(|s| s.as_str()).collect();
    const VOTE_PROGRAM: &str = "Vote111111111111111111111111111111111111111";
    let mut result = HashSet::new();

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

        // Only include transactions touching a watched account
        if !accounts.iter().any(|a| account_set.contains(a.as_str())) {
            continue;
        }

        result.insert(sig);
    }

    Ok(result)
}