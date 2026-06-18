use std::collections::HashMap;
use std::time::{Duration, Instant};

use anyhow::{Context, Result};
use futures::{SinkExt, StreamExt};
use tokio::sync::mpsc;
use tracing::{debug, error, info, warn};
use tonic::transport::ClientTlsConfig;
use yellowstone_grpc_client::GeyserGrpcClient;
use yellowstone_grpc_proto::prelude::{
    CommitmentLevel, SubscribeRequest, SubscribeRequestFilterAccounts,
    SubscribeRequestPing, SubscribeUpdate,
};

use crate::config::{Config, StreamGroup};

/// How often to send a ping to the server to keep the connection alive.
const PING_INTERVAL: Duration = Duration::from_secs(15);

#[derive(Debug)]
pub enum StreamEvent {
    AccountUpdate {
        stream: String,
        slot: u64,
        pubkey: String,
        txn_signature: Option<String>,
    },
}

/// Spawn one background reader task per configured stream group.
pub fn spawn_grpc_readers(cfg: Config, tx: mpsc::Sender<StreamEvent>) {
    for group in cfg.streams.clone() {
        let cfg = cfg.clone();
        let tx = tx.clone();
        tokio::spawn(async move {
            let mut attempt: u32 = 0;
            loop {
                attempt += 1;
                info!("[{}] connect attempt #{attempt}", group.name);
                let session_start = Instant::now();

                match run_stream(&cfg, &group, tx.clone()).await {
                    Ok(()) => {
                        info!(
                            "[{}] stream closed cleanly after {:.1}s — reconnecting …",
                            group.name,
                            session_start.elapsed().as_secs_f64()
                        );
                    }
                    Err(e) => {
                        error!(
                            "[{}] disconnected after {:.1}s: {e:#} — reconnecting in 3 s …",
                            group.name,
                            session_start.elapsed().as_secs_f64()
                        );
                        tokio::time::sleep(Duration::from_secs(3)).await;
                    }
                }
            }
        });
    }
}

async fn run_stream(
    cfg: &Config,
    group: &StreamGroup,
    tx: mpsc::Sender<StreamEvent>,
) -> Result<()> {
    info!("[{}] connecting to {}", group.name, cfg.grpc_endpoint);

    let mut client = GeyserGrpcClient::build_from_shared(cfg.grpc_endpoint.clone())?
        .x_token(cfg.grpc_x_token.clone())?
        .connect_timeout(Duration::from_secs(20))
        .timeout(Duration::from_secs(20))
        .tls_config(ClientTlsConfig::new().with_native_roots())?
        .max_decoding_message_size(1024 * 1024 * 1024)
        .connect()
        .await
        .context("Failed to connect to Yellowstone gRPC")?;

    info!(
        "[{}] connected — subscribing to {} accounts",
        group.name,
        group.accounts.len()
    );

    let request = build_subscribe_request(&group.accounts);
    let (mut sink, mut stream) = client
        .subscribe_with_request(Some(request))
        .await
        .context("subscribe_with_request failed")?;

    info!(
        "[{}] subscribed — streaming account updates (ping every {}s)",
        group.name,
        PING_INTERVAL.as_secs()
    );

    let mut ping_id: i32 = 0;
    let mut ping_ticker = tokio::time::interval(PING_INTERVAL);
    ping_ticker.tick().await; // discard the immediate first tick

    loop {
        tokio::select! {
            _ = ping_ticker.tick() => {
                ping_id += 1;
                let ping = SubscribeRequest {
                    ping: Some(SubscribeRequestPing { id: ping_id }),
                    ..Default::default()
                };
                if let Err(e) = sink.send(ping).await {
                    warn!("[{}] failed to send ping id={ping_id}: {e}", group.name);
                } else {
                    info!("[{}] ping sent id={ping_id}", group.name);
                }
            }

            msg = stream.next() => {
                match msg {
                    Some(Ok(update)) => {
                        handle_update(update, &group.name, &tx).await?;
                    }
                    Some(Err(status)) => {
                        return Err(anyhow::anyhow!("stream error: {status}"));
                    }
                    None => {
                        info!("[{}] server closed the stream", group.name);
                        return Ok(());
                    }
                }
            }
        }
    }
}

fn build_subscribe_request(accounts: &[String]) -> SubscribeRequest {
    let acct_filter = SubscribeRequestFilterAccounts {
        account: accounts.to_vec(),
        owner: vec![],
        filters: vec![],
        nonempty_txn_signature: Some(true),
    };

    SubscribeRequest {
        accounts: {
            let mut m = HashMap::new();
            m.insert("pool_vaults".to_owned(), acct_filter);
            m
        },
        slots: HashMap::new(),
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
    stream_name: &str,
    tx: &mpsc::Sender<StreamEvent>,
) -> Result<()> {
    use yellowstone_grpc_proto::prelude::subscribe_update::UpdateOneof;

    let inner = match update.update_oneof {
        Some(v) => v,
        None => return Ok(()),
    };

    match inner {
        UpdateOneof::Account(acct_update) => {
            let slot = acct_update.slot;
            let info = match acct_update.account {
                Some(a) => a,
                None => return Ok(()),
            };

            let pubkey = bs58::encode(&info.pubkey).into_string();
            let txn_signature = info
                .txn_signature
                .as_deref()
                .filter(|b| !b.is_empty())
                .map(|b| bs58::encode(b).into_string());

            debug!(
                "[{stream_name}] account  slot={slot}  pubkey={pubkey}  sig={:?}",
                txn_signature
            );

            tx.send(StreamEvent::AccountUpdate {
                stream: stream_name.to_owned(),
                slot,
                pubkey,
                txn_signature,
            })
            .await
            .ok();
        }

        UpdateOneof::Pong(pong) => {
            info!("[{stream_name}] pong received id={}", pong.id);
        }

        _ => {}
    }

    Ok(())
}
