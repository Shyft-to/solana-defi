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

/// Compare gRPC-observed signatures for a slot against the RPC ground truth.
///
/// Produces a report with:
///   - `missed`: signatures the RPC confirms but gRPC never delivered (data loss)
///   - `extra`:  signatures gRPC delivered but RPC doesn't recognise (timing edge case)
pub async fn reconcile(
    client: &RpcClient,
    cfg: &Config,
    slot_data: &SlotData,
) -> Result<ReconcileReport> {
    let slot = slot_data.slot;

    // Signatures collected from the gRPC stream for this slot.
    let grpc_set: &HashSet<String> = &slot_data.grpc_signatures;

    // Signatures fetched from the RPC for the same slot (ground truth).
    let rpc_set = fetch_rpc_signatures(client, cfg, slot).await?;

    // In RPC but not in gRPC → gRPC missed these transactions.
    let missed: Vec<String> = rpc_set
        .difference(grpc_set)
        .cloned()
        .collect();

    // In gRPC but not in RPC → gRPC delivered something RPC doesn't confirm yet.
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

/// Fetch the RPC's view of signatures for `slot`, using whichever strategy is configured.
///
/// Returns a `HashSet` of base-58 signature strings for non-failed, non-vote
/// transactions that touch at least one watched account.
async fn fetch_rpc_signatures(
    client: &RpcClient,
    cfg: &Config,
    slot: u64,
) -> Result<HashSet<String>> {
    // USE_GET_BLOCK=true → fetch the full block and filter locally.
    // USE_GET_BLOCK=false (default) → query per watched account via getSignaturesForAddress.
    if cfg.use_get_block {
        return fetch_via_get_block(client, cfg, slot).await;
    }

    // One getSignaturesForAddress call per watched account, results unioned.
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

/// Query `getSignaturesForAddress` for a single account, collecting only
/// signatures that landed in exactly `slot` and succeeded (no error).
///
/// Paginates newest-first until results go older than the target slot,
/// so no transactions are missed even if the page size limit is hit.
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
    // Pagination cursor: the RPC returns signatures older than this one on the next page.
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
            // Only collect successful transactions in the exact target slot.
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

        // Advance the cursor to just before the oldest signature on this page.
        before = page
            .last()
            .and_then(|item| item.signature.parse::<Signature>().ok());

        if before.is_none() {
            break;
        }
    }

    Ok(sigs)
}

/// Fetch the entire block for `slot` via `getBlock`, then filter down to
/// signatures of transactions that touch at least one watched account.
///
/// This is the alternative to `getSignaturesForAddress` — it fetches more data
/// (the full block) but makes a single RPC call regardless of how many accounts
/// are being watched.
async fn fetch_via_get_block(
    client: &RpcClient,
    cfg: &Config,
    slot: u64,
) -> Result<HashSet<String>> {
    // Fetch the full block with all transaction details in JSON-parsed form.
    // JsonParsed gives us human-readable account keys (base-58 strings) without
    // needing to decode raw bytes ourselves.
    let block = client
        .get_block_with_config(
            slot,
            RpcBlockConfig {
                encoding: Some(UiTransactionEncoding::JsonParsed),
                transaction_details: Some(TransactionDetails::Full),
                rewards: Some(false), // we don't need staking reward data
                commitment: Some(CommitmentConfig {
                    commitment: CommitmentLevel::Confirmed,
                }),
                max_supported_transaction_version: Some(0), // include versioned (v0) transactions
            },
        )
        .await
        .context("getBlock RPC call failed")?;

    // Build a fast lookup set from the watched account list.
    let account_set: HashSet<&str> = cfg.account_include.iter().map(|s| s.as_str()).collect();

    // The on-chain address of the Solana vote program — used to detect and skip validator votes.
    const VOTE_PROGRAM: &str = "Vote111111111111111111111111111111111111111";

    let mut result = HashSet::new();

    for tx_with_meta in block.transactions.unwrap_or_default() {
        // Skip failed transactions — mirrors gRPC filter `failed: Some(false)`.
        if tx_with_meta
            .meta
            .as_ref()
            .and_then(|m| m.err.as_ref())
            .is_some()
        {
            continue;
        }

        // We requested JsonParsed encoding, so every transaction comes back as
        // EncodedTransaction::Json. The wildcard arm is a safety fallback.
        let ui_tx = match &tx_with_meta.transaction {
            EncodedTransaction::Json(tx) => tx,
            _ => continue,
        };

        // The first entry in `signatures` is always the transaction's primary signature.
        let sig = match ui_tx.signatures.first() {
            Some(s) => s.clone(),
            None => continue,
        };

        // Extract all account keys involved in this transaction.
        // JsonParsed returns Parsed (with structured fields) for known programs,
        // or Raw (plain strings) when the program isn't recognised.
        let accounts: Vec<String> = match &ui_tx.message {
            UiMessage::Parsed(m) => m.account_keys.iter().map(|k| k.pubkey.clone()).collect(),
            UiMessage::Raw(m) => m.account_keys.clone(),
        };

        // Skip vote transactions — mirrors gRPC filter `vote: Some(false)`.
        // Validator vote transactions always include the vote program as an account.
        if accounts.iter().any(|a| a == VOTE_PROGRAM) {
            continue;
        }

        // Only keep transactions where at least one account matches our watch list.
        // This narrows the full block (potentially thousands of transactions) down
        // to only those relevant to the accounts we subscribed to via gRPC.
        if !accounts.iter().any(|a| account_set.contains(a.as_str())) {
            continue;
        }

        result.insert(sig);
    }

    Ok(result)
}
