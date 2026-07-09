use std::time::{Duration, SystemTime, UNIX_EPOCH};

use anyhow::{Context, Result};
use futures::StreamExt;
use tokio::sync::mpsc;
use tonic::transport::ClientTlsConfig;
use tracing::{debug, error, info, warn};
use yellowstone_grpc_client::GeyserGrpcClient;
use yellowstone_grpc_proto::prelude::{
    subscribe_update::UpdateOneof, CommitmentLevel, SubscribeRequest,
    SubscribeRequestFilterBlocksMeta, SubscribeRequestFilterTransactions, SubscribeUpdate,
};

use crate::config::Config;
use crate::slack;
use crate::types::{StreamEvent, StreamId};

/// Reconnect backoff after a stream error or clean close.
const RECONNECT_DELAY: Duration = Duration::from_secs(3);

/// What a reader task is subscribed to. Each variant becomes its own gRPC
/// connection — Yellowstone endpoints commonly cap a subscription at one
/// filter, and separate connections keep the streams independent so a stall on
/// one does not delay the others.
#[derive(Debug, Clone)]
enum Subscription {
    /// A transaction stream with its own account_include list.
    Transactions {
        stream: StreamId,
        account_include: Vec<String>,
        commitment: CommitmentLevel,
    },
    /// The blocks_meta stream, source of on-chain block timestamps.
    BlocksMeta { commitment: CommitmentLevel },
}

impl Subscription {
    /// Label used in connection/reconnect logs.
    fn label(&self) -> String {
        match self {
            Subscription::Transactions { stream, .. } => format!("tx-{}", stream.tag()),
            Subscription::BlocksMeta { .. } => "blocks-meta".to_owned(),
        }
    }

    fn commitment(&self) -> CommitmentLevel {
        match self {
            Subscription::Transactions { commitment, .. }
            | Subscription::BlocksMeta { commitment } => *commitment,
        }
    }
}

/// Spawn all three reader tasks: two transaction streams and one blocks_meta
/// stream. Each is a separate gRPC connection and a separate `tokio::spawn`
/// task, so tokio's multi-threaded runtime runs them concurrently across
/// worker threads — a stall or reconnect on one never blocks the others. All
/// three forward onto the same `tx` channel; the consumer demultiplexes on
/// [`StreamEvent`].
pub fn spawn_all(cfg: Config, tx: mpsc::Sender<StreamEvent>) {
    let subs = [
        Subscription::Transactions {
            stream: StreamId::One,
            account_include: cfg.account_include_1.clone(),
            commitment: cfg.commitment_1,
        },
        Subscription::Transactions {
            stream: StreamId::Two,
            account_include: cfg.account_include_2.clone(),
            commitment: cfg.commitment_2,
        },
        Subscription::BlocksMeta {
            commitment: cfg.commitment_blocks_meta,
        },
    ];

    for sub in subs {
        spawn_reader(cfg.clone(), sub, tx.clone());
    }
}

/// Run one subscription forever, reconnecting on error or clean close.
///
/// Every disconnect is logged locally regardless of Slack configuration; when
/// `SLACK_WEBHOOK_URL` is set, the same event is additionally posted there —
/// `slack::report` is a no-op when it isn't.
fn spawn_reader(cfg: Config, sub: Subscription, tx: mpsc::Sender<StreamEvent>) {
    tokio::spawn(async move {
        let label = sub.label();
        loop {
            match run_stream(&cfg, &sub, &tx).await {
                Ok(()) => {
                    warn!("[{label}] stream ended cleanly; reconnecting …");
                    slack::report(
                        &cfg.slack_webhook_url,
                        &format!(":warning: *[{label}]* stream disconnected (clean close); reconnecting in 3s …"),
                    )
                    .await;
                }
                Err(e) => {
                    error!("[{label}] stream error: {e:#}; reconnecting in 3 s …");
                    slack::report(
                        &cfg.slack_webhook_url,
                        &format!(":red_circle: *[{label}]* stream disconnected: {e:#}; reconnecting in 3s …"),
                    )
                    .await;
                }
            }
            tokio::time::sleep(RECONNECT_DELAY).await;
        }
    });
}

/// Connect, subscribe, and forward events until the stream closes or errors.
async fn run_stream(
    cfg: &Config,
    sub: &Subscription,
    tx: &mpsc::Sender<StreamEvent>,
) -> Result<()> {
    let label = sub.label();
    info!("[{label}] connecting to {}", cfg.grpc_endpoint);

    let mut client = GeyserGrpcClient::build_from_shared(cfg.grpc_endpoint.clone())?
        .x_token(cfg.grpc_x_token.clone())?
        .connect_timeout(Duration::from_secs(20))
        // No per-request timeout: a transactions filter can legitimately sit
        // idle for longer than any fixed timeout while its accounts are quiet.
        .tls_config(ClientTlsConfig::new().with_native_roots())?
        .max_decoding_message_size(1024 * 1024 * 1024)
        .connect()
        .await
        .with_context(|| format!("[{label}] failed to connect to Yellowstone gRPC"))?;

    let (mut _sink, mut stream) = client
        .subscribe_with_request(Some(build_subscribe_request(sub)))
        .await
        .with_context(|| format!("[{label}] subscribe_with_request failed"))?;

    info!(
        "[{label}] subscribed at commitment={:?}; streaming …",
        sub.commitment()
    );

    while let Some(msg) = stream.next().await {
        let update = msg.map_err(|status| anyhow::anyhow!("[{label}] stream error: {status}"))?;

        // Stamp arrival before doing anything else with the update.
        let recv_ms = unix_now_ms();

        if let Some(event) = decode(update, sub, recv_ms) {
            // A full channel means the consumer has fallen behind. Blocking here
            // applies backpressure to the stream rather than silently dropping a
            // transaction and reporting a bogus latency gap for it later.
            if tx.send(event).await.is_err() {
                warn!("[{label}] event channel closed; stopping reader");
                return Ok(());
            }
        }
    }

    Ok(())
}

/// Build the [`SubscribeRequest`] for one subscription — exactly one filter each.
fn build_subscribe_request(sub: &Subscription) -> SubscribeRequest {
    let mut req = SubscribeRequest {
        commitment: Some(sub.commitment() as i32),
        ..Default::default()
    };

    match sub {
        Subscription::Transactions {
            account_include, ..
        } => {
            req.transactions.insert(
                "txn".to_owned(),
                SubscribeRequestFilterTransactions {
                    vote: Some(false),
                    failed: Some(false),
                    signature: None,
                    account_include: account_include.clone(),
                    account_exclude: vec![],
                    account_required: vec![],
                    ..Default::default()
                },
            );
        }
        Subscription::BlocksMeta { .. } => {
            req.blocks_meta
                .insert("meta".to_owned(), SubscribeRequestFilterBlocksMeta {});
        }
    }

    req
}

/// Decode one [`SubscribeUpdate`] into a [`StreamEvent`], or `None` for
/// control messages (pings, pongs) and updates we did not subscribe to.
fn decode(update: SubscribeUpdate, sub: &Subscription, recv_ms: i64) -> Option<StreamEvent> {
    match (update.update_oneof?, sub) {
        (UpdateOneof::Transaction(u), Subscription::Transactions { stream, .. }) => {
            let info = u.transaction?;
            let signature = bs58::encode(&info.signature).into_string();
            debug!("[{}] tx slot={} sig={signature}", stream.tag(), u.slot);

            Some(StreamEvent::Transaction {
                stream: *stream,
                slot: u.slot,
                signature,
                recv_ms,
            })
        }

        (UpdateOneof::BlockMeta(meta), Subscription::BlocksMeta { .. }) => {
            // `block_time` is the validator's estimate of when the block was
            // executed, in whole Unix *seconds*. Scale to milliseconds so it can
            // be diffed against transaction arrival timestamps directly — the
            // sub-second digits are always zero (see README on precision).
            let block_time_ms = meta.block_time.map(|t| t.timestamp * 1_000);
            debug!("[blocks-meta] slot={} block_time_ms={block_time_ms:?}", meta.slot);

            Some(StreamEvent::BlockMeta {
                slot: meta.slot,
                block_time_ms,
            })
        }

        _ => None,
    }
}

/// Current wall clock as Unix milliseconds.
fn unix_now_ms() -> i64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis() as i64
}
