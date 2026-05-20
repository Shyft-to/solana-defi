# websocket-checker

A minimal Rust tool that connects to a Solana WebSocket RPC endpoint and streams live account update notifications to stdout.

## What it does

1. Opens a WebSocket connection to a Solana RPC node
2. Sends an `accountSubscribe` JSON-RPC request for a given account pubkey
3. Logs every `accountNotification` event — slot, lamports, owner, and executable flag
4. Responds to server pings to keep the connection alive

## Requirements

- Rust 1.75+ (uses `tokio` async runtime)
- A Solana RPC WebSocket URL (mainnet, devnet, or a provider like Shyft)

## Configuration

Edit the two constants at the top of `src/main.rs`:

```rust
const WS_URL: &str = "wss://rpc.shyft.to/?api_key=YOUR_API_KEY";
const ACCOUNT_PUBKEY: &str = "YOUR_ACCOUNT_PUBKEY";
```

> **Note:** The path `/` before `?` in the URL is required. Some nginx-based RPC proxies return `400 Bad Request` without it.

## Usage

```bash
cargo run
```

Example output:

```
Connecting to Solana WebSocket: wss://rpc.shyft.to/?api_key=...
Connected.
Subscribed to account: 58oQChx4yWmvKdwLLZzBi4ChoCc2fqCUWBkwMihLYQo2
Waiting for notifications...

[Subscription confirmed] subscription id: 123456

--- Account Update ---
  Subscription : 123456
  Slot         : 287401234
  Lamports     : 1000000000
  Owner        : "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"
  Executable   : false
  Raw          : { ... }
```

## Dependencies

| Crate | Purpose |
|---|---|
| `tokio` | Async runtime |
| `tokio-tungstenite` | WebSocket client |
| `futures-util` | Stream/sink combinators |
| `serde_json` | JSON serialization |
