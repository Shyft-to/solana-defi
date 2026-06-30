use std::collections::HashSet;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;
use std::time::Duration;

use dashmap::DashMap;
use solana_client::nonblocking::rpc_client::RpcClient;
use solana_client::rpc_config::RpcBlockConfig;
use solana_sdk::commitment_config::CommitmentConfig;
use solana_transaction_status::{
    option_serializer::OptionSerializer, EncodedTransaction, TransactionDetails, UiMessage,
    UiRawMessage, UiTransactionEncoding,
};
use tracing::warn;

use crate::fetcher::AccountStateMap;

const VOTE_PROGRAM_ID: &str = "Vote111111111111111111111111111111111111111111";

// Extra delay after SLOT_FINALIZED before calling getBlock, to let the RPC node catch up.
const RPC_LAG_SECS: u64 = 2;

pub struct Verifier {
    rpc: RpcClient,
    target_pubkeys: Vec<String>,
    // (slot, pubkey) → list of txn signatures delivered by gRPC for that pubkey in that slot
    updates: Arc<DashMap<(u64, String), Vec<String>>>,
    // earliest slot for which we received a gRPC account update
    start_slot: Arc<AtomicU64>,
    // (slot, pubkey) → account data snapshot taken when that slot finalized
    account_states: AccountStateMap,
}

impl Verifier {
    pub fn new(
        rpc_url: &str,
        target_pubkeys: Vec<String>,
        updates: Arc<DashMap<(u64, String), Vec<String>>>,
        start_slot: Arc<AtomicU64>,
        account_states: AccountStateMap,
    ) -> Self {
        Self {
            rpc: RpcClient::new_with_commitment(rpc_url.to_owned(), CommitmentConfig::finalized()),
            target_pubkeys,
            updates,
            start_slot,
            account_states,
        }
    }

    pub async fn verify_slot(&self, slot: u64) {
        // Ignore slots that finalized before our gRPC subscription was established.
        // start_slot stays at u64::MAX until the first account update arrives.
        let start = self.start_slot.load(Ordering::Relaxed);
        if start == u64::MAX || slot < start {
            return;
        }

        tokio::time::sleep(Duration::from_secs(RPC_LAG_SECS)).await;

        let config = RpcBlockConfig {
            encoding: Some(UiTransactionEncoding::Json),
            transaction_details: Some(TransactionDetails::Full),
            rewards: Some(false),
            commitment: Some(CommitmentConfig::finalized()),
            max_supported_transaction_version: Some(0),
        };

        // Single getBlock call for the slot — reused across all watched pubkeys.
        let txns = match self.rpc.get_block_with_config(slot, config).await {
            Ok(b) => b.transactions.unwrap_or_default(),
            Err(e) => {
                // Log only if at least one pubkey had gRPC activity for this slot.
                let any_grpc = self
                    .target_pubkeys
                    .iter()
                    .any(|pk| self.updates.contains_key(&(slot, pk.clone())));
                if any_grpc {
                    warn!("SLOT {slot} | getBlock failed: {e:#}");
                }
                for pk in &self.target_pubkeys {
                    self.updates.remove(&(slot, pk.clone()));
                }
                return;
            }
        };

        // Check each watched pubkey independently against the same block.
        for pubkey in &self.target_pubkeys {
            let grpc_sigs: HashSet<String> = self
                .updates
                .get(&(slot, pubkey.clone()))
                .map(|v| v.iter().filter(|s| !s.is_empty()).cloned().collect())
                .unwrap_or_default();
            let grpc_count = grpc_sigs.len();

            let expected_sigs = collect_writable_sigs(&txns, pubkey);
            let expected_count = expected_sigs.len();

            // No activity for this pubkey in this slot — skip silently.
            if grpc_count == 0 && expected_count == 0 {
                self.updates.remove(&(slot, pubkey.clone()));
                continue;
            }

            if grpc_count < expected_count {
                let delta = expected_count - grpc_count;
                println!(
                    "SLOT {slot} | pubkey {pubkey} | gRPC: {grpc_count} | rpc_writable: {expected_count} | NO_GRPC_UPDATE (delta: {delta})"
                );
                for sig in &expected_sigs {
                    if !grpc_sigs.contains(sig) {
                        println!(
                            "  pubkey {pubkey} | TXN {sig} — in writable set, account may not have been modified. No update from gRPC."
                        );
                    }
                }
                self.compare_account_states(slot, pubkey).await;
            } else {
                println!(
                    "SLOT {slot} | pubkey {pubkey} | gRPC: {grpc_count} | rpc_writable: {expected_count} | OK"
                );
            }

            self.updates.remove(&(slot, pubkey.clone()));
        }
    }

    /// Compares the account data snapshot at `slot - 1` against `slot`.
    /// Called only on NO_GRPC_UPDATE to determine whether the account was
    /// actually modified (real delivery gap) or just listed as writable without
    /// a state change (expected no-op from Yellowstone's perspective).
    ///
    /// Polls for up to 5 s to let the AccountFetcher's RPC call complete.
    async fn compare_account_states(&self, slot: u64, pubkey: &str) {
        // Wait for the fetcher to populate data for this slot (max 5s, 500ms steps).
        for _ in 0..10 {
            if self.account_states.contains_key(&(slot, pubkey.to_owned())) {
                break;
            }
            tokio::time::sleep(Duration::from_millis(500)).await;
        }

        let prev = self.account_states.get(&(slot.saturating_sub(1), pubkey.to_owned()));
        let curr = self.account_states.get(&(slot, pubkey.to_owned()));

        match (prev, curr) {
            (Some(p), Some(c)) => {
                if p.value() != c.value() {
                    println!(
                        "  ERROR pubkey {pubkey} | account data CHANGED between slot {} and {slot} — likely a real gRPC delivery gap",
                        slot.saturating_sub(1)
                    );
                } else {
                    println!(
                        "  INFO  pubkey {pubkey} | account data UNCHANGED in slot {slot} — writable-but-no-modify, no real gap"
                    );
                }
            }
            _ => {
                println!(
                    "  WARN  pubkey {pubkey} | account state snapshot not available for slot {slot} after 5s — cannot compare"
                );
            }
        }
    }
}

// Collect the primary signature of every transaction in the block that:
//   - succeeded (no error)
//   - is not a vote transaction
//   - has `target` as a writable account
fn collect_writable_sigs(
    txns: &[solana_transaction_status::EncodedTransactionWithStatusMeta],
    target: &str,
) -> Vec<String> {
    let mut sigs = Vec::new();

    for tx_with_meta in txns {
        let meta = match &tx_with_meta.meta {
            Some(m) => m,
            None => continue,
        };

        if meta.err.is_some() {
            continue;
        }

        let (sig, raw) = match &tx_with_meta.transaction {
            EncodedTransaction::Json(ui_tx) => {
                let sig = match ui_tx.signatures.first() {
                    Some(s) => s.clone(),
                    None => continue,
                };
                let raw = match &ui_tx.message {
                    UiMessage::Raw(r) => r,
                    _ => continue,
                };
                (sig, raw)
            }
            _ => continue,
        };

        if raw.account_keys.iter().any(|k| k == VOTE_PROGRAM_ID) {
            continue;
        }

        let loaded_writable = match &meta.loaded_addresses {
            OptionSerializer::Some(la) => la.writable.clone(),
            _ => vec![],
        };

        if !is_writable(target, raw, &loaded_writable) {
            continue;
        }

        sigs.push(sig);
    }

    sigs
}

// Returns true if `target` appears in the writable account set of the transaction.
//
// Writable set derivation (per Solana message header spec):
//   writable signers:     account_keys[0 .. nrs - nrsa]
//   writable non-signers: account_keys[nrs .. n - nrua]
//   loaded (ALT) writable: passed in separately from meta.loadedAddresses.writable
fn is_writable(target: &str, raw: &UiRawMessage, loaded_writable: &[String]) -> bool {
    let n = raw.account_keys.len();
    let nrs = raw.header.num_required_signatures as usize;
    let nrsa = raw.header.num_readonly_signed_accounts as usize;
    let nrua = raw.header.num_readonly_unsigned_accounts as usize;

    // writable signers: [0, nrs - nrsa)
    let ws_end = nrs.saturating_sub(nrsa).min(n);
    if raw.account_keys[..ws_end].iter().any(|k| k == target) {
        return true;
    }

    // writable non-signers: [nrs, n - nrua)
    let wns_start = nrs.min(n);
    let wns_end = n.saturating_sub(nrua);
    if wns_start < wns_end && raw.account_keys[wns_start..wns_end].iter().any(|k| k == target) {
        return true;
    }

    // loaded (ALT) writable accounts
    loaded_writable.iter().any(|k| k == target)
}
