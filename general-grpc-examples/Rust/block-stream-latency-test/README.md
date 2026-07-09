# block streamer + latency

Streams full Solana blocks from a Yellowstone gRPC endpoint, logs each block's number, and measures how long after production each block arrived (by comparing against the RPC `getBlock` time). Errors are logged and — if a Slack webhook is configured — posted to Slack; otherwise everything is log-only.

---

## How to run

**1. Copy the env file and fill in your values**

```bash
cp .env.example .env
```

| Variable | Required | Description |
|---|---|---|
| `GRPC_ENDPOINT` | Yes | Yellowstone gRPC endpoint URL (the block stream) |
| `GRPC_X_TOKEN` | No | Auth token for the gRPC endpoint |
| `RPC_URL` | No | Solana **JSON-RPC** endpoint for `getBlock` latency checks — omit to disable latency |
| `LATENCY_LAG_SLOTS` | No | Slots to wait before calling `getBlock` (default: `32`, ~14s) |
| `LATENCY_SAMPLE_EVERY` | No | Measure latency every Nth block (default: `1`) |
| `SLACK_WEBHOOK_URL` | No | Slack incoming webhook — omit to log-only |

**2. Build and run**

```bash
cargo run --release
```

Stop with `Ctrl-C`. Override log level with `RUST_LOG` (e.g. `RUST_LOG=debug cargo run --release`).

---

## How it works

### Block stream

Opens a single gRPC connection and subscribes to **full blocks** (`SubscribeRequestFilterBlocks` with `include_transactions: true`) at **`FINALIZED`** commitment. On each block it logs:

```
block received  slot=… block_height=… txns=…
```

so you can confirm blocks are arriving continuously and in order.

The runner loops forever: on any disconnect (clean close or error) it logs a warning, reports to Slack (if configured), waits 3s, and reconnects — logging/reporting again on success.

### Block latency measurement

When `RPC_URL` is set, each block triggers (sampled by `LATENCY_SAMPLE_EVERY`) a background task:

```
block arrives on stream (slot S)  →  record arrival wall-clock time (ms)
wait LATENCY_LAG_SLOTS slots (lets the RPC node index the slot)
call getBlock(S)                  →  read its on-chain blockTime
latency_ms = arrival_time_ms − blockTime × 1000
```

Logged as:

```
block stream latency (arrival − getBlock.blockTime)  slot=… block_time=… arrival_unix_ms=… latency_ms=…
```

`getBlock` is a standard Solana **JSON-RPC** call — a different transport from the gRPC stream — so `RPC_URL` points at a regular Solana RPC endpoint, not the gRPC one. The check runs off the stream loop, so it never blocks block ingestion. `getBlock` failures are logged and sent to Slack (if configured).

**Two things to know about the number:**

- **It's coarse (±~1s).** `blockTime` is a Unix timestamp in *whole seconds* (a stake-weighted median of validators' vote timestamps). The `× 1000` only converts units — it adds no sub-second precision — so the trailing millisecond digits come solely from the arrival side.
- **At `FINALIZED` it's dominated by finalization lag.** A block is only streamed after it finalizes (~32 slots, ~14s), so `latency_ms` mostly measures finalization delay, not transport speed. For transport latency you'd stream at a lower commitment (`PROCESSED`/`CONFIRMED`).

### Reporting

Every notable event — disconnects, reconnects, `getBlock` failures — is logged. If `SLACK_WEBHOOK_URL` is set, the same events are also POSTed to the webhook. With no webhook, the tool is log-only.
