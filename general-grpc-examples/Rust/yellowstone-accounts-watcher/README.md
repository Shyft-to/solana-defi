# yellowstone-accounts-watcher

Streams Solana account updates from a Yellowstone gRPC node and logs each update alongside the transaction that caused it. Designed to verify that account updates are delivered reliably and to observe connection health over time.

## Build & Run

```bash
cp .env.example .env   # fill in your values

cargo run
```

### Release build

```bash
cargo build --release
./target/release/yellowstone-watcher
```

---

## Configuration
The watcher supports multi-stream monitoring. You can define up to three distinct account sets, each running in its own dedicated gRPC stream. If an `ACCOUNT_INCLUDE` variable is left empty, that specific stream will not be spawned.

| Variable | Required | Default | Description |
|---|---|---|---|
| `GRPC_ENDPOINT` | ✓ | — | Yellowstone gRPC URL |
| `GRPC_X_TOKEN` | | — | Auth token (omit if not required) |
| `ACCOUNT_INCLUDE_1` | ✓ | — | Pubkeys for Stream 1 (comma-separated) |
| `ACCOUNT_INCLUDE_2` |  | — | Pubkeys for Stream 2 (comma-separated)|
| `ACCOUNT_INCLUDE_3` |  | — | Pubkeys for Stream 3 (comma-separated) |
| `RUST_LOG` | | `info` | Tracing filter (`info`, `debug`, `trace`) |

---

## How it works

1. Multi-Stream Architecture: The service independently initializes up to 3 separate gRPC streams based on the provided ACCOUNT_INCLUDE_X environment variables. If a variable is unset, that stream is ignored.

2. Subscription Logic: Subscribes to account updates for specified pubkeys at Confirmed commitment.

3. Filtering: Only transaction-triggered updates are delivered (nonempty_txn_signature=true), so startup snapshot writes are automatically filtered out.

4. Heartbeat: To maintain connection health, a ping is sent to the server every 15 seconds via the gRPC sink. The server responds with a pong, which is logged at the debug level.

5. Resilience: On any stream error or clean server close, the session duration is logged and a reconnect is attempted after 3 seconds. The service allows the connection to persist as long as the server keeps it open.

---

## Log output

```
# Startup
INFO  Watching 30 accounts
INFO  gRPC connect attempt #1
INFO  Connecting to https://grpc.ams.shyft.to
INFO  Connected — subscribing to 30 accounts
INFO  Subscribed — streaming account updates (ping every 15s)

# Account update received
INFO  ACCOUNT  slot=345123456  account=DNfuF1…  sig=3xQr…

# Ping / pong (debug level — only visible with RUST_LOG=debug)
DEBUG Ping sent id=1
DEBUG Pong received id=1

# Server closed the stream cleanly
INFO  Server closed the stream
INFO  gRPC stream closed cleanly after 182.3s — reconnecting …
INFO  gRPC connect attempt #2
INFO  Connecting to https://grpc.ams.shyft.to
...

# Stream error / disconnection
ERROR gRPC disconnected after 45.7s: stream error: … — reconnecting in 3 s …
INFO  gRPC connect attempt #2
INFO  Connecting to https://grpc.ams.shyft.to
...
```
