# silent-stream-detector

Monitors two Yellowstone gRPC streams (`blocks_meta` and `transactions`) and fires a Slack alert when either stream goes silent for too long.

## How it works

```
main.py
  ├── blocks_meta task   → subscribes to block metadata events
  ├── transactions task  → subscribes to filtered transaction events
  └── idle_monitor task  → checks every 30s if either stream has gone silent
```

Each stream task:
- Connects to the Shyft gRPC endpoint with TLS + `x-token` auth
- Touches a `LastSeen` timestamp on every message received
- Logs a heartbeat every 30s showing when the last message arrived
- Auto-reconnects on any disconnect with a 3s backoff

The `idle_monitor` fires a Slack webhook alert if a stream's last message was more than `IDLE_TIMEOUT_SECS` ago.

## Prerequisites

- Python 3.11+
- A Solana Yellowstone gRPC endpoint and x token (like from [shyft.to](https://shyft.to))
- (Optional) A Slack incoming webhook URL for alerts

## Running the project

**1. Clone and enter the directory**
```bash
git clone <repo-url>
cd general-grpc-examples/Python/silent-stream-detector-python
```

**2. Create and activate a virtual environment**
```bash
python3 -m venv .venv
source .venv/bin/activate      # Windows: .venv\Scripts\activate
```

**3. Install dependencies**
```bash
pip install -r requirements.txt
```

**4. Generate protobuf stubs** (only needed once)
```bash
bash generate_proto.sh
```

**5. Configure environment** — copy the example and fill in your values:
```bash
cp .env.example .env
```

Then edit `.env`:
```env
# Required
GRPC_ENDPOINT=https://grpc.fra.shyft.to:443

# Optional
GRPC_X_TOKEN=your-api-token
ACCOUNT_INCLUDE=pubkey1,pubkey2      # comma-separated, filters the transactions stream
IDLE_TIMEOUT_SECS=120                # seconds before an idle alert fires (default: 120)
SLACK_WEBHOOK_URL=https://hooks.slack.com/services/XXX/YYY/ZZZ
```

**6. Run**
```bash
python main.py
```

Stop with `Ctrl+C` — it shuts down gracefully.

## Log output

| Event | Example |
|---|---|
| Connecting | `[blocks_meta] connecting to https://...` |
| Connected | `[blocks_meta] connected (commitment=CONFIRMED)` |
| Heartbeat (every 30s) | `[blocks_meta] connected — last message 12s ago` |
| Disconnect | `[blocks_meta] disconnected — <reason>` |
| Reconnect | `[blocks_meta] reconnecting in 3s (attempt 2)...` |
| Idle alert | `stream idle  name=transactions  idle_secs=130` |

## Project structure

| File | Purpose |
|---|---|
| `main.py` | Entry point, wires up tasks and signal handling |
| `config.py` | Loads config from environment variables |
| `channel.py` | Creates the authenticated gRPC channel |
| `blocks_meta.py` | Subscribes to the `blocks_meta` stream |
| `transactions.py` | Subscribes to the `transactions` stream |
| `monitor.py` | Idle detection and Slack alert logic |
| `slack.py` | Sends Slack webhook messages |
| `generate_proto.sh` | Generates Python stubs from `.proto` files |
