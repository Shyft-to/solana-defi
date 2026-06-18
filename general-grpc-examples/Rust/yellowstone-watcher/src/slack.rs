use tracing::warn;

/// Post a notification about missed transactions (gRPC gap detected) to Slack.
pub async fn notify_missed(webhook_url: &str, slot: u64, sigs: &[String]) {
    let lines: Vec<String> = sigs.iter().map(|s| format!("• `{s}`")).collect();
    let text = format!(
        "🚨 *MISSED* — Slot {slot} — {} transaction(s) confirmed by RPC but never delivered by gRPC:\n{}",
        sigs.len(),
        lines.join("\n")
    );
    post(webhook_url, &text).await;
}

/// Post a notification about extra transactions (gRPC delivered what RPC doesn't confirm) to Slack.
pub async fn notify_extra(webhook_url: &str, slot: u64, sigs: &[String]) {
    let lines: Vec<String> = sigs.iter().map(|s| format!("• `{s}`")).collect();
    let text = format!(
        "⚠️ *EXTRA* — Slot {slot} — {} transaction(s) delivered by gRPC but not recognised by RPC:\n{}",
        sigs.len(),
        lines.join("\n")
    );
    post(webhook_url, &text).await;
}

/// Send a plain-text payload to a Slack incoming webhook URL.
async fn post(webhook_url: &str, text: &str) {
    let payload = serde_json::json!({ "text": text });
    match reqwest::Client::new()
        .post(webhook_url)
        .json(&payload)
        .send()
        .await
    {
        Ok(resp) if resp.status().is_success() => {}
        Ok(resp) => warn!("Slack webhook returned unexpected status: {}", resp.status()),
        Err(e) => warn!("Slack webhook request failed: {e}"),
    }
}
