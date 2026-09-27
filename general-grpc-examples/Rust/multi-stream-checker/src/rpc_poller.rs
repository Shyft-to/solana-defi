use std::time::Duration;

use anyhow::{Context, Result};
use serde_json::{json, Value};
use tracing::{info, warn};

use crate::slack;

/// Poll `rpc_url` for `getSlot` every `interval_secs`, printing each slot as
/// it arrives. Runs forever; a request error is logged on every failed tick
/// but only posted to Slack on the transition into and out of a failing
/// state — otherwise a downed RPC polled every second or so would flood the
/// channel with one alert per tick.
pub async fn run(
    region: String,
    slack_webhook_url: Option<String>,
    rpc_url: String,
    interval_secs: u64,
    commitment: String,
) {
    info!(
        "[{region}] [rpc-poller] polling {rpc_url} for getSlot (commitment={commitment}) every {interval_secs}s"
    );

    let client = reqwest::Client::new();
    let mut ticker = tokio::time::interval(Duration::from_secs(interval_secs));
    ticker.set_missed_tick_behavior(tokio::time::MissedTickBehavior::Delay);

    let mut failing = false;

    loop {
        ticker.tick().await;

        match get_slot(&client, &rpc_url, &commitment).await {
            Ok(slot) => {
                info!("[{region}] [rpc-poller] slot={slot}");
                if failing {
                    failing = false;
                    slack::report(
                        &slack_webhook_url,
                        &format!(":white_check_mark: *[{region}] [rpc-poller]* getSlot recovered (slot={slot})"),
                    )
                    .await;
                }
            }
            Err(e) => {
                warn!("[{region}] [rpc-poller] getSlot failed: {e:#}");
                if !failing {
                    failing = true;
                    slack::report(
                        &slack_webhook_url,
                        &format!(":red_circle: *[{region}] [rpc-poller]* getSlot failed: {e:#}"),
                    )
                    .await;
                }
            }
        }
    }
}

/// Call the `getSlot` JSON-RPC method against `rpc_url` at `commitment`.
async fn get_slot(client: &reqwest::Client, rpc_url: &str, commitment: &str) -> Result<u64> {
    let payload = json!({
        "jsonrpc": "2.0",
        "id": 1,
        "method": "getSlot",
        "params": [{ "commitment": commitment }],
    });

    let resp: Value = client
        .post(rpc_url)
        .json(&payload)
        .send()
        .await
        .context("request failed")?
        .error_for_status()
        .context("non-2xx response")?
        .json()
        .await
        .context("failed to parse response body")?;

    if let Some(err) = resp.get("error") {
        anyhow::bail!("rpc error: {err}");
    }

    resp.get("result")
        .and_then(Value::as_u64)
        .context("response missing `result` slot")
}
