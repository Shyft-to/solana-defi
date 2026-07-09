use anyhow::Result;
use tracing::warn;

/// Posts a message to a Slack incoming webhook.
pub async fn send_alert(webhook_url: &str, text: &str) -> Result<()> {
    let payload = serde_json::json!({ "text": text });

    reqwest::Client::new()
        .post(webhook_url)
        .json(&payload)
        .send()
        .await?
        .error_for_status()?;

    Ok(())
}

/// Posts to Slack when a webhook is configured, logging a warning if the post
/// itself fails. A no-op when no webhook is set — callers log separately, so
/// events are always visible in the log and additionally sent to Slack if present.
pub async fn report(webhook_url: &Option<String>, text: &str) {
    if let Some(url) = webhook_url {
        if let Err(e) = send_alert(url, text).await {
            warn!("slack send failed: {e:#}");
        }
    }
}
