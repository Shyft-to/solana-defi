use std::collections::HashMap;
use std::time::{Duration, Instant};

use anyhow::{Context, Result};
use futures::{SinkExt, StreamExt};
use tokio::sync::mpsc;
use tracing::{error, info, warn};
use tonic::transport::ClientTlsConfig;
use yellowstone_grpc_client::GeyserGrpcClient;
use yellowstone_grpc_proto::prelude::{
    subscribe_update::UpdateOneof, CommitmentLevel, SlotStatus, SubscribeRequest,
    SubscribeRequestFilterAccounts, SubscribeRequestFilterSlots, SubscribeRequestPing,
    SubscribeUpdate,
};

use crate::config::Config;

const PING_INTERVAL: Duration = Duration::from_secs(15);

#[derive(Debug)]
pub struct AccountUpdate {
    pub slot: u64,
    pub pubkey: String,
    pub txn_signature: Option<String>,
}

pub fn spawn_stream(
    cfg: Config,
    account_tx: mpsc::Sender<AccountUpdate>,
    slot_tx: mpsc::Sender<u64>,
    fetch_slot_tx: mpsc::Sender<u64>,
) {
    tokio::spawn(async move {
        let mut attempt: u32 = 0;
        loop {
            attempt += 1;
            info!("gRPC connect attempt #{attempt}");
            let t = Instant::now();
            match run_stream(&cfg, account_tx.clone(), slot_tx.clone(), fetch_slot_tx.clone()).await {
                Ok(()) => {
                    info!(
                        "stream closed cleanly after {:.1}s — reconnecting",
                        t.elapsed().as_secs_f64()
                    );
                }
                Err(e) => {
                    error!(
                        "stream error after {:.1}s: {e:#} — reconnecting in 3s",
                        t.elapsed().as_secs_f64()
                    );
                    tokio::time::sleep(Duration::from_secs(3)).await;
                }
            }
        }
    });
}

async fn run_stream(
    cfg: &Config,
    account_tx: mpsc::Sender<AccountUpdate>,
    slot_tx: mpsc::Sender<u64>,
    fetch_slot_tx: mpsc::Sender<u64>,
) -> Result<()> {
    let mut client = GeyserGrpcClient::build_from_shared(cfg.grpc_endpoint.clone())?
        .x_token(cfg.grpc_x_token.clone())?
        .connect_timeout(Duration::from_secs(20))
        .timeout(Duration::from_secs(20))
        .tls_config(ClientTlsConfig::new().with_native_roots())?
        .max_decoding_message_size(1024 * 1024 * 1024)
        .connect()
        .await
        .context("failed to connect to Yellowstone gRPC")?;

    let request = build_request(&cfg.target_pubkeys);
    let (mut sink, mut stream) = client
        .subscribe_with_request(Some(request))
        .await
        .context("subscribe_with_request failed")?;

    info!(
        "subscribed — watching {} account(s) + slot finalizations",
        cfg.target_pubkeys.len()
    );

    let mut ping_id: i32 = 0;
    let mut ticker = tokio::time::interval(PING_INTERVAL);
    ticker.tick().await;

    loop {
        tokio::select! {
            _ = ticker.tick() => {
                ping_id += 1;
                let ping = SubscribeRequest {
                    ping: Some(SubscribeRequestPing { id: ping_id }),
                    ..Default::default()
                };
                if let Err(e) = sink.send(ping).await {
                    warn!("ping failed: {e}");
                }
            }

            msg = stream.next() => {
                match msg {
                    Some(Ok(update)) => handle_update(update, &account_tx, &slot_tx, &fetch_slot_tx).await?,
                    Some(Err(s)) => return Err(anyhow::anyhow!("stream error: {s}")),
                    None => {
                        info!("server closed stream");
                        return Ok(());
                    }
                }
            }
        }
    }
}

fn build_request(target_pubkeys: &[String]) -> SubscribeRequest {
    let acct_filter = SubscribeRequestFilterAccounts {
        account: target_pubkeys.to_vec(),
        owner: vec![],
        filters: vec![],
        ..Default::default()
    };

    let slot_filter = SubscribeRequestFilterSlots {
        filter_by_commitment: None,
        interslot_updates: None,
    };

    SubscribeRequest {
        accounts: {
            let mut m = HashMap::new();
            m.insert("pumpFun".to_owned(), acct_filter);
            m
        },
        slots: {
            let mut n = HashMap::new();
            n.insert("all".to_owned(), slot_filter);
            n
        },
        transactions: HashMap::new(),
        transactions_status: HashMap::new(),
        blocks: HashMap::new(),
        blocks_meta: HashMap::new(),
        entry: HashMap::new(),
        accounts_data_slice: vec![],
        commitment: Some(CommitmentLevel::Confirmed as i32),
        ping: None,
        from_slot: None,
    }
}

async fn handle_update(
    update: SubscribeUpdate,
    account_tx: &mpsc::Sender<AccountUpdate>,
    slot_tx: &mpsc::Sender<u64>,
    fetch_slot_tx: &mpsc::Sender<u64>,
) -> Result<()> {
    match update.update_oneof {
        Some(UpdateOneof::Account(acct)) => {
            let slot = acct.slot;
            let info = match acct.account {
                Some(a) => a,
                None => return Ok(()),
            };
            let pubkey = bs58::encode(&info.pubkey).into_string();
            let sig = info
                .txn_signature
                .as_deref()
                .filter(|b| !b.is_empty())
                .map(|b| bs58::encode(b).into_string());
            account_tx
                .send(AccountUpdate { slot, pubkey, txn_signature: sig })
                .await
                .ok();
        }
        Some(UpdateOneof::Slot(slot_update)) => {
            if slot_update.status == SlotStatus::SlotFinalized as i32 {
                slot_tx.send(slot_update.slot).await.ok();
                fetch_slot_tx.send(slot_update.slot).await.ok();
            }
        }
        Some(UpdateOneof::Pong(_)) => {}
        _ => {}
    }
    Ok(())
}
