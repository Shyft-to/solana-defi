use std::time::Duration;

use anyhow::{bail, Result};
use serde_json::json;
use tracing::{info, warn};

use crate::slack;

/// Records the wall-clock arrival of a streamed block, waits `delay` (so the RPC
/// node has definitely indexed the slot), then calls `getBlock` and compares the
/// block's on-chain `blockTime` against when we received it on the stream.
///
/// `latency_ms = arrival_unix_ms − blockTime*1000` — how long after the block was
/// produced we received it via the gRPC stream. Runs in its own task so it never
/// blocks the stream loop. `getBlock` failures are reported to the log and to
/// Slack (when a webhook is configured).
///
/// Note: `blockTime` is second-resolution, so the sub-second digits of
/// `latency_ms` come only from the arrival side — treat it as coarse (±1s).
pub fn spawn_measure(
    rpc_url: String,
    slack_webhook_url: Option<String>,
    slot: u64,
    arrival_unix_ms: u128,
    delay: Duration,
) {
    tokio::spawn(async move {
        tokio::time::sleep(delay).await;

        match get_block_time(&rpc_url, slot).await {
            Ok(Some(block_time)) => {
                let latency_ms = arrival_unix_ms as i128 - (block_time as i128) * 1000;
                info!(
                    slot,
                    block_time,
                    arrival_unix_ms = arrival_unix_ms as u64,
                    latency_ms = latency_ms as i64,
                    "block stream latency (arrival − getBlock.blockTime)"
                );
            }
            Ok(None) => warn!(slot, "getBlock returned null blockTime — skipping latency"),
            Err(e) => {
                warn!(slot, "getBlock failed: {e:#}");
                slack::report(
                    &slack_webhook_url,
                    &format!(":red_circle: *getBlock* failed for slot {slot}: {e:#}"),
                )
                .await;
            }
        }
    });
}

/// Calls the Solana JSON-RPC `getBlock` for `slot` and returns its `blockTime`
/// (Unix seconds). The request is made as light as possible — no transactions,
/// no rewards — since we only need the timestamp.
async fn get_block_time(rpc_url: &str, slot: u64) -> Result<Option<i64>> {
    let payload = json!({
        "jsonrpc": "2.0",
        "id": 1,
        "method": "getBlock",
        "params": [
            slot,
            {
                "commitment": "finalized",
                "transactionDetails": "none",
                "rewards": false,
                "maxSupportedTransactionVersion": 0
            }
        ]
    });

    let resp: serde_json::Value = reqwest::Client::new()
        .post(rpc_url)
        .json(&payload)
        .send()
        .await?
        .error_for_status()?
        .json()
        .await?;

    if let Some(err) = resp.get("error") {
        bail!("rpc error: {err}");
    }

    Ok(resp["result"]["blockTime"].as_i64())
}
