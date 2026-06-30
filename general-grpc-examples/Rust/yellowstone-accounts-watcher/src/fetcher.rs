use std::str::FromStr;
use std::sync::Arc;

use dashmap::DashMap;
use solana_client::nonblocking::rpc_client::RpcClient;
use solana_sdk::commitment_config::CommitmentConfig;
use solana_sdk::pubkey::Pubkey;
use tokio::sync::mpsc;
use tracing::{info, warn};

/// (slot, pubkey) → account data bytes at that slot (None if account does not exist).
pub type AccountStateMap = Arc<DashMap<(u64, String), Option<Vec<u8>>>>;

pub struct AccountFetcher {
    rpc: RpcClient,
    pubkeys: Vec<Pubkey>,
    pubkey_strs: Vec<String>,
    pub states: AccountStateMap,
}

impl AccountFetcher {
    pub fn new(rpc_url: &str, target_pubkeys: Vec<String>, states: AccountStateMap) -> Self {
        let pubkeys = target_pubkeys
            .iter()
            .filter_map(|s| Pubkey::from_str(s).ok())
            .collect();
        Self {
            rpc: RpcClient::new_with_commitment(rpc_url.to_owned(), CommitmentConfig::finalized()),
            pubkeys,
            pubkey_strs: target_pubkeys,
            states,
        }
    }

    /// Runs in its own tokio task. For every finalized slot received it spawns a
    /// sub-task that snapshots all watched accounts via getMultipleAccounts.
    pub async fn run(self, mut slot_rx: mpsc::Receiver<u64>) {
        let this = Arc::new(self);
        while let Some(slot) = slot_rx.recv().await {
            let f = this.clone();
            tokio::spawn(async move {
                f.fetch_slot(slot).await;
                // Keep only the last two slots in memory so the map doesn't grow unboundedly.
                let stale = slot.saturating_sub(2);
                for pk in &f.pubkey_strs {
                    f.states.remove(&(stale, pk.clone()));
                }
            });
        }
        info!("account-fetcher channel closed");
    }

    async fn fetch_slot(&self, slot: u64) {
        match self.rpc.get_multiple_accounts(&self.pubkeys).await {
            Ok(accounts) => {
                for (pk_str, acct) in self.pubkey_strs.iter().zip(accounts.iter()) {
                    self.states
                        .insert((slot, pk_str.clone()), acct.as_ref().map(|a| a.data.clone()));
                }
            }
            Err(e) => warn!("slot {slot} | getMultipleAccounts failed: {e:#}"),
        }
    }
}
