use futures_util::{SinkExt, StreamExt};
use serde_json::{json, Value};
use tokio_tungstenite::{
    connect_async,
    tungstenite::{client::IntoClientRequest, Message},
};

// Solana WebSocket endpoint — explicit "/" path required before query string
const WS_URL: &str = "wss://rpc.shyft.to/?api_key=random";

// A well-known devnet account (the System Program)
const ACCOUNT_PUBKEY: &str = "58oQChx4yWmvKdwLLZzBi4ChoCc2fqCUWBkwMihLYQo2";

#[tokio::main]
async fn main() {
    println!("Connecting to Solana WebSocket: {WS_URL}");

    let mut request = WS_URL
        .into_client_request()
        .expect("Invalid WebSocket URL");

    // Some RPC proxies (nginx) require an explicit Host header
    request.headers_mut().insert(
        "Host",
        "rpc.shyft.to".parse().unwrap(),
    );

    let (ws_stream, _) = connect_async(request)
        .await
        .expect("Failed to connect to WebSocket");

    println!("Connected.");

    let (mut write, mut read) = ws_stream.split();

    // Send accountSubscribe request (Solana JSON-RPC over WS)
    let subscribe_msg = json!({
        "jsonrpc": "2.0",
        "id": 1,
        "method": "accountSubscribe",
        "params": [
            ACCOUNT_PUBKEY,
            {
                "encoding": "base64",
                "commitment": "finalized"
            }
        ]
    });

    write
        .send(Message::Text(subscribe_msg.to_string().into()))
        .await
        .expect("Failed to send subscribe message");

    println!("Subscribed to account: {ACCOUNT_PUBKEY}");
    println!("Waiting for notifications...\n");

    // Read incoming messages
    while let Some(msg) = read.next().await {
        match msg {
            Ok(Message::Text(text)) => {
                let parsed: Value = serde_json::from_str(&text).unwrap_or(Value::Null);

                // Subscription confirmation
                if parsed["id"] == 1 {
                    let sub_id = &parsed["result"];
                    println!("[Subscription confirmed] subscription id: {sub_id}");
                    continue;
                }

                // Account notification
                if parsed["method"] == "accountNotification" {
                    let params = &parsed["params"];
                    let sub_id = &params["subscription"];
                    let slot = &params["result"]["context"]["slot"];
                    let lamports = &params["result"]["value"]["lamports"];
                    let owner = &params["result"]["value"]["owner"];
                    let executable = &params["result"]["value"]["executable"];

                    println!("--- Account Update ---");
                    println!("  Subscription : {sub_id}");
                    println!("  Slot         : {slot}");
                    println!("  Lamports     : {lamports}");
                    println!("  Owner        : {owner}");
                    println!("  Executable   : {executable}");
                    println!("  Raw          : {params}\n");
                    continue;
                }

                // Anything else
                println!("[Message] {parsed}");
            }
            Ok(Message::Ping(data)) => {
                write
                    .send(Message::Pong(data))
                    .await
                    .expect("Failed to send pong");
            }
            Ok(Message::Close(frame)) => {
                println!("Connection closed: {frame:?}");
                break;
            }
            Ok(_) => {}
            Err(e) => {
                eprintln!("WebSocket error: {e}");
                break;
            }
        }
    }
}
