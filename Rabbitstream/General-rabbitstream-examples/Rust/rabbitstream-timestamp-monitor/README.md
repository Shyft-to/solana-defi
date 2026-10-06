# rabbitstream-timestamp-monitor

Subscribes to Solana transactions via [Yellowstone gRPC](https://github.com/rpcpool/yellowstone-grpc)
and measures the latency between the geyser plugin's `created_at` timestamp and the
moment each update is received locally. Auto-reconnects on errors/idle timeouts, can
auto-stop after a fixed number of transactions, and prints a p50/p95/p99/avg latency
report on exit. Optionally posts disconnect alerts to Slack.

``` 
Note: This also works with Rabbitstream endpoints. 
```


## Setup

**1. Copy the example env file and fill in your values:**

```bash
cp .env.example .env
```

```env
ENDPOINT=https://your-endpoint.solana-mainnet.example.com
X_TOKEN=your_x_token_here
ACCOUNT_INCLUDE=6EF8rrecthR5Dkzon8Nwu78hRvfCKubJ14M5uBEwF6P,pAMMBay6oceH9fJKBRHGP5D4bD4sWpmSwMn52FMfXEA
LOG_SIG=true
SHOW_TIMESTAMPS=false
STATS_INTERVAL_SECS=5
MAX_TRANSACTIONS=1000
REGION=us-east-1
SLACK_URL=
```

All values are read from `.env` at startup (via [dotenvy](https://docs.rs/dotenvy)) and
map 1:1 onto CLI flags of the same name.

| Variable | Flag | Description | Default |
|---|---|---|---|
| `ENDPOINT` | `--endpoint` | Yellowstone/Rabbitstream gRPC endpoint | required |
| `X_TOKEN` | `--x-token` | Auth token for the endpoint | required |
| `ACCOUNT_INCLUDE` | `--account-include` | Comma-separated program/account addresses to filter transactions on | required |
| `LOG_SIG` | `--log-sig` | `true` to print each transaction's signature and its per-transaction latency line; `false` to suppress both (the summary/report still use every sample either way) | `true` |
| `SHOW_TIMESTAMPS` | `--show-timestamps` | `true` to also print the `created_at`/`received_at` timestamps (RFC3339, ms precision) right before each latency line. Only takes effect when `LOG_SIG=true` | `false` |
| `MAX_TRANSACTIONS` | `--max-transactions` | Stop after receiving this many transactions, gracefully close the stream, and print the latency report. Omit or set `0` to run indefinitely | — (run forever) |
| `STATS_INTERVAL_SECS` | `--stats-interval-secs` | How often the throughput stats window rolls over (seconds). Currently unused — see [Known limitations](#known-limitations) | `5` |
| `REGION` | `--region` | Free-form tag prefixed on log lines and Slack alerts (e.g. which deployment/PoP this instance is watching) | `unknown` |
| `SLACK_URL` | `--slack-url` | Slack incoming webhook URL. When set, disconnect/reconnect events are also posted there; a failed post only logs a warning, it never stops the stream | — (console-only) |

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
  --max-transactions 1000
```

## What it does

1. Connects to `ENDPOINT` and subscribes to non-vote, non-failed transactions touching
   any address in `ACCOUNT_INCLUDE`.
2. For every transaction update, computes `received_at - created_at`:
   - `received_at` is captured the instant the update comes off the gRPC stream.
   - `created_at` is the timestamp the geyser plugin stamped on the `SubscribeUpdate`
     envelope (a sibling of the transaction payload, not nested inside it).
   - The resulting latency (ms) is recorded for the final report on every transaction,
     regardless of `LOG_SIG`.
3. Answers `Ping` updates with a `Pong` to keep the stream alive, and reconnects (with a
   constant 2s backoff) if the stream errors, closes, or goes idle for 30s.
4. If `MAX_TRANSACTIONS` (> 0) is set, once that many transactions have been received it:
   - closes the client → server request sink (`sink.close()`),
   - drops the server → client stream, which makes tonic/h2 send `RST_STREAM` and cancel
     the call — there's no explicit `.cancel()` method on this client, dropping an
     unfinished `Streaming<T>` *is* the cancellation,
   - stops the retry loop and moves on to printing the report.
5. On exit (`MAX_TRANSACTIONS` reached, or the process is stopped) it prints the run
   summary and the latency report described below.

## Output

Each received transaction prints its signature, when `LOG_SIG=true`:

```
[INFO  rabbitstream_stream_monitor] 2TNkmwSZ...
[INFO  rabbitstream_stream_monitor] [rabbit-fra] created_at -> received_at latency: 42.13ms
```

Add `SHOW_TIMESTAMPS=true` to also print the raw timestamps right before that line:

```
[INFO  rabbitstream_stream_monitor] 2TNkmwSZ...
[INFO  rabbitstream_stream_monitor] [rabbit-fra] created_at: 2026-09-12T10:15:32.100Z | received_at: 2026-09-12T10:15:32.142Z
[INFO  rabbitstream_stream_monitor] [rabbit-fra] created_at -> received_at latency: 42.13ms
```

While `MAX_TRANSACTIONS` is set, progress is logged roughly every 5% regardless of `LOG_SIG`:

```
[INFO  rabbitstream_stream_monitor] [rabbit-fra] Progress: 500/1000 (50.0%)
[INFO  rabbitstream_stream_monitor] [rabbit-fra] Progress: 1000/1000 (100.0%)
[INFO  rabbitstream_stream_monitor] [rabbit-fra] Reached 1000 transactions — closing stream
```

When the run ends, a latency report table is printed:

```
===== Latency Report [rabbit-fra] — 1000 samples =====
+---------+--------------+
| Metric  | Latency (ms) |
+---------+--------------+
| Min     |        12.34 |
| Average |        45.67 |
| P50     |        40.00 |
| P95     |        98.20 |
| P99     |       150.10 |
| Max     |       210.55 |
+---------+--------------+
```

If a transaction's `created_at` is missing/unparseable, or the local clock appears to be
*behind* the geyser node's (negative latency), that sample is skipped from the report and
a `warn!` is logged instead — these warnings print regardless of `LOG_SIG`.

The monitor automatically reconnects on stream errors, a closed stream, or 30s of silence,
notifying `SLACK_URL` (if set) on every disconnect.

## Known limitations

- The `Run Summary` (throughput tx/s block) and `STATS_INTERVAL_SECS` are currently
  dead — the code that populates them is commented out in `src/main.rs`, so
  `print_summary` silently prints nothing. Only the latency report is currently live.
- `MAX_TRANSACTIONS` counts transactions as `sink.close()`/`drop(stream)` see them on the
  *current* connection; a mid-run reconnect does not reset the counter (it's shared
  across reconnect attempts), but the process only stops once the target is hit while
  connected.
