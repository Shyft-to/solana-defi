# rabbitstream-streamer

Streams Solana transactions via [Yellowstone gRPC](https://github.com/rpcpool/yellowstone-grpc) and prints signatures with throughput stats.

## Setup

**1. Copy the example env file and fill in your values:**

```bash
cp .env.example .env
```

```env
ENDPOINT=https://your-endpoint.solana-mainnet.example.com:10000
X_TOKEN=your_x_token_here
ACCOUNT_INCLUDE=6EF8rrecthR5Dkzon8Nwu78hRvfCKubJ14M5uBEwF6P,pAMMBay6oceH9fJKBRHGP5D4bD4sWpmSwMn52FMfXEA
LOG_SIG=true
STATS_INTERVAL_SECS=5
RUN_DURATION_MINS=60
```

| Variable | Description | Default |
|---|---|---|
| `ENDPOINT` | Yellowstone/Rabbitstream gRPC endpoint (must include port) | required |
| `X_TOKEN` | Auth token for the endpoint | required |
| `ACCOUNT_INCLUDE` | Comma-separated program/account addresses to filter on | required |
| `LOG_SIG` | `true` to print each transaction signature, `false` to suppress | `true` |
| `STATS_INTERVAL_SECS` | How often to print throughput stats (seconds) | `5` |
| `RUN_DURATION_MINS` | Auto-stop after this many minutes, omit to run forever | — |

**2. Run:**

```bash
cargo run
```

You can also pass values directly as flags (these override `.env`):

```bash
cargo run -- \
  --endpoint <URL> \
  --x-token <TOKEN> \
  --account-include <ADDR1>,<ADDR2> \
  --log-sig true \
  --stats-interval-secs 1 \
  --run-duration-mins 60
```

## Output

Each received transaction prints its signature (when `LOG_SIG=true`):

```
[INFO  rabbitstream_streamer] 2TNkmwSZ...
```

Every `STATS_INTERVAL_SECS` seconds a throughput line is printed:

```
[INFO  rabbitstream_streamer] -----> throughput: 12.4 tx/s | total transactions: 62 <------
```

When `RUN_DURATION_MINS` is set, a summary and ASCII chart are printed on exit:

```
========== Run Summary ==========
  Duration : 3600s
  Avg tx/s : 14.2
  Peak tx/s: 31.0
  Min  tx/s: 4.5
=================================

Throughput over time (tx/s)
...
```

The streamer automatically reconnects on errors or if no messages are received for 30 seconds.
