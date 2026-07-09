use std::collections::HashMap;
use std::sync::Arc;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use anyhow::{Context, Result};
use futures::StreamExt;
use tracing::{info, warn};
use yellowstone_grpc_client::GeyserGrpcClient;
use yellowstone_grpc_proto::prelude::{
    subscribe_update::UpdateOneof, CommitmentLevel, SubscribeRequestFilterBlocks,
};

use crate::{config::Config, latency, slack};

/// Streams full blocks (with transactions) at FINALIZED commitment, logging each
/// block's number and firing a sampled `getBlock` latency check. Reconnects
/// indefinitely, reporting every disconnect to the log and to Slack (if set).
pub fn spawn(cfg: Arc<Config>) {
    tokio::spawn(async move {
        let mut is_reconnect = false;
        loop {
            match run(&cfg, is_reconnect).await {
                Ok(()) => {
                    warn!("blocks stream closed; reconnecting in 3s …");
                    slack::report(
                        &cfg.slack_webhook_url,
                        ":red_circle: *blocks* stream disconnected (clean close); reconnecting …",
                    )
                    .await;
                }
                Err(e) => {
                    warn!("blocks stream dropped ({e:#}); reconnecting in 3s …");
                    slack::report(
                        &cfg.slack_webhook_url,
                        &format!(":red_circle: *blocks* stream disconnected: {e:#}"),
                    )
                    .await;
                }
            }
            tokio::time::sleep(Duration::from_secs(3)).await;
            is_reconnect = true;
        }
    });
}

async fn run(cfg: &Config, is_reconnect: bool) -> Result<()> {
    info!("blocks connecting to {}", cfg.grpc_endpoint);

    let mut client = GeyserGrpcClient::connect_with_timeout(
        cfg.grpc_endpoint.clone(),
        cfg.grpc_x_token.clone(),
        None, // auto-applies TLS when endpoint scheme is https://
        Some(Duration::from_secs(20)),
        None, // no per-request timeout — avoids killing idle streams
        false,
    )
    .await
    .context("blocks: connect failed")?;

    info!("blocks stream connected (commitment=FINALIZED)");
    if is_reconnect {
        slack::report(
            &cfg.slack_webhook_url,
            ":large_green_circle: *blocks* stream reconnected successfully",
        )
        .await;
    }

    let mut blocks_filter = HashMap::new();
    blocks_filter.insert(
        "all".to_owned(),
        SubscribeRequestFilterBlocks {
            account_include: vec![],
            include_transactions: Some(true),
            include_accounts: Some(false),
            include_entries: Some(false),
        },
    );

    let mut stream = client
        .subscribe_once(
            HashMap::new(),
            HashMap::new(),
            HashMap::new(),
            HashMap::new(),
            blocks_filter,
            HashMap::new(),
            Some(CommitmentLevel::Finalized),
            vec![],
        )
        .await
        .context("blocks: subscribe failed")?;

    while let Some(msg) = stream.next().await {
        match msg {
            Ok(update) => {
                if let Some(UpdateOneof::Block(block)) = update.update_oneof {
                    let block_height = block.block_height.map(|h| h.block_height);
                    info!(
                        slot = block.slot,
                        block_height,
                        txns = block.executed_transaction_count,
                        "block received"
                    );

                    // Sampled block-arrival latency vs the block's on-chain
                    // blockTime (fetched via getBlock after a slot lag).
                    if let Some(rpc_url) = &cfg.rpc_url {
                        if block.slot % cfg.latency_sample_every == 0 {
                            let delay = Duration::from_millis(cfg.latency_lag_slots * 450);
                            latency::spawn_measure(
                                rpc_url.clone(),
                                cfg.slack_webhook_url.clone(),
                                block.slot,
                                unix_now_ms(),
                                delay,
                            );
                        }
                    }
                }
                // pings and other control messages are intentionally ignored
            }
            Err(status) => {
                return Err(anyhow::anyhow!("blocks: stream error: {status}"));
            }
        }
    }

    Ok(())
}

fn unix_now_ms() -> u128 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis()
}
