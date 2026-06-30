use std::str::FromStr;
use std::sync::Arc;

use dashmap::DashMap;
use solana_client::nonblocking::rpc_client::RpcClient;
use solana_sdk::commitment_config::CommitmentConfig;
use solana_sdk::pubkey::Pubkey;
use tokio::sync::mpsc;
use tracing::{info, warn};

use crate::config::Commitment;

/// (slot, pubkey) → account data bytes at that slot (None if account does not exist).
pub type AccountStateMap = Arc<DashMap<(u64, String), Option<Vec<u8>>>>;

pub struct AccountFetcher {
    rpc: RpcClient,
    pubkey: Pubkey,
    pubkey_str: String,
    pub states: AccountStateMap,
}

impl AccountFetcher {
    pub fn new(
        rpc_url: &str,
        target_pubkey: String,
        states: AccountStateMap,
        rpc_commitment: Commitment,
    ) -> Self {
        let commitment_config = match rpc_commitment {
            Commitment::Confirmed => CommitmentConfig::confirmed(),
            Commitment::Finalized => CommitmentConfig::finalized(),
        };
        let pubkey = Pubkey::from_str(&target_pubkey)
            .expect("TARGET_PUBKEY is not a valid base58 pubkey");
        Self {
            rpc: RpcClient::new_with_commitment(rpc_url.to_owned(), commitment_config),
            pubkey,
            pubkey_str: target_pubkey,
            states,
        }
    }

    /// Runs in its own tokio task. For every finalized slot received it spawns a
    /// sub-task that snapshots the watched account via getAccountInfo.
    pub async fn run(self, mut slot_rx: mpsc::Receiver<u64>) {
        let this = Arc::new(self);
        while let Some(slot) = slot_rx.recv().await {
            let f = this.clone();
            tokio::spawn(async move {
                f.fetch_slot(slot).await;
                // Keep the last 50 slots so the comparison worker has time to read them.
                let stale = slot.saturating_sub(50);
                f.states.remove(&(stale, f.pubkey_str.clone()));
            });
        }
        info!("account-fetcher channel closed");
    }

    async fn fetch_slot(&self, slot: u64) {
        match self.rpc.get_account(&self.pubkey).await {
            Ok(account) => {
                self.states.insert((slot, self.pubkey_str.clone()), Some(account.data));
            }
            Err(e) => {
                warn!("slot {slot} | getAccountInfo failed: {e:#}");
                // Store None so comparison worker can still proceed.
                self.states.insert((slot, self.pubkey_str.clone()), None);
            }
        }
    }
}
