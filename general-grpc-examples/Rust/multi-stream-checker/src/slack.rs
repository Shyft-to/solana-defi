use anyhow::Result;
use tracing::warn;

/// Post a message to a Slack incoming webhook.
async fn send_alert(webhook_url: &str, text: &str) -> Result<()> {
    let payload = serde_json::json!({ "text": text });

    reqwest::Client::new()
        .post(webhook_url)
        .json(&payload)
        .send()
        .await?
        .error_for_status()?;

    Ok(())
}

/// Post to Slack when a webhook is configured; a no-op otherwise.
///
/// Callers always log the event separately — this only adds the Slack copy, so
/// disconnects remain visible in the console log with or without a webhook set.
/// A failed post is itself only logged, never propagated, so a broken webhook
/// can't take down a stream reader.
pub async fn report(webhook_url: &Option<String>, text: &str) {
    if let Some(url) = webhook_url {
        if let Err(e) = send_alert(url, text).await {
            warn!("slack send failed: {e:#}");
        }
    }
}
