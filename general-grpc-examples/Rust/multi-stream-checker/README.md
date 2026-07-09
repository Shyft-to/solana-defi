# multi-stream-checker

Measures how long after a block is executed on-chain each of two Yellowstone
gRPC transaction streams delivers its transactions.

Three subscriptions run concurrently against the same endpoint:

| Stream | Filter | Purpose |
|---|---|---|
| `tx-S1` | `transactions`, `account_include = ACCOUNT_INCLUDE_1` | transaction arrivals |
| `tx-S2` | `transactions`, `account_include = ACCOUNT_INCLUDE_2` | transaction arrivals |
| `blocks-meta` | `blocks_meta` | the block's on-chain execution timestamp |

Each is its own gRPC connection — Yellowstone endpoints commonly cap a
subscription at one filter, and independent connections keep a stall on one
stream from delaying the others. All three feed a single channel; one event
loop joins them.

Both transaction filters use `vote = false`, `failed = false`.

## Commitment

`COMMITMENT` (`processed` | `confirmed` | `finalized`, default `confirmed`) sets
the level for all three streams. `COMMITMENT_1`, `COMMITMENT_2`, and
`COMMITMENT_BLOCKS_META` override it per stream. An unrecognised value fails at
startup rather than silently falling back.

Per-stream overrides make a second comparison possible: point `ACCOUNT_INCLUDE_1`
and `ACCOUNT_INCLUDE_2` at the *same* accounts, set `COMMITMENT_1=processed` and
`COMMITMENT_2=confirmed`, and each transaction prints twice — once per level,
both against the same `block_time`. The gap between the two is the cost of
waiting for confirmation.

**Mind the buffer when blocks_meta is the strictest stream.** Block times only
arrive at that stream's commitment, so `COMMITMENT_BLOCKS_META=finalized` with
`COMMITMENT=processed` makes every transaction wait ~32 slots in the buffer for
its block time. That works, but `LATENCY_BUFFER_SLOTS` must exceed the lag or
transactions get evicted before they can be timed — startup warns if the buffer
looks too small for the configured levels. Note that the commitment level does
**not** change `block_time` itself, only when you learn it; latency figures stay
comparable across levels.

## Latency

For each transaction:

```
latency_ms = tx_recv_ms − block_time_ms
```

`tx_recv_ms` is stamped the instant the update is pulled off the gRPC stream,
before any internal channel hop. `block_time_ms` comes from the `blocks_meta`
update for that transaction's slot.

Transactions and blocks_meta race, and either can win for a given slot. Both
orders produce the same latency:

- **tx first** — no block time for the slot yet, so the transaction is buffered
  and emitted when that slot's blocks_meta lands.
- **blocks_meta first** — the block time is already known, so the transaction is
  emitted immediately, with no buffering. The slot's block time stays cached
  until eviction, so *any* number of later transactions for that slot resolve
  instantly.

Which order dominates depends on the configured commitment levels. Slots that
fall more than `LATENCY_BUFFER_SLOTS` behind the chain head are evicted and
logged, so memory stays bounded when a blocks_meta update never arrives at all.

### Precision caveat

**`block_time` is whole Unix seconds.** Yellowstone reports it as an `int64`
second count, so this tool scales it by 1000 and formats it with three zeroed
fractional digits (`…:22.000Z`). Every millisecond digit in `latency_ms` comes
from the arrival side alone.

Two consequences:

- An individual `latency_ms` value carries up to ~1s of quantisation error, and
  **can be negative** when the validator's block-time estimate rounds forward.
- The *difference* between S1 and S2 for the same slot is exact — both are
  measured against the same `block_time`, so the error cancels. Comparing the
  two streams is what this tool is for; treat the absolute number as coarse.

## Output

One line per transaction, tagged with the stream that delivered it:

```
[S1] slot=340123456 sig=5xK9aB2c recv=2026-07-08T11:04:22.187Z b_time=2026-07-08T11:04:22.000Z latency=+187ms
[S2] slot=340123456 sig=3jQm7Ffd recv=2026-07-08T11:04:22.243Z b_time=2026-07-08T11:04:22.000Z latency=+243ms
```

A transaction touching accounts in both lists is delivered on both streams and
prints twice, once per tag — that is the comparison case.

Set `LOG_SIGNATURES=false` to drop `sig=...` from the line:

```
[S1] slot=340123456 recv=2026-07-08T11:04:22.187Z b_time=2026-07-08T11:04:22.000Z latency=+187ms
```

Set `LOG_TRANSACTIONS=false` to drop this line entirely — for when only the
periodic `[STATS]` summary below is wanted. Percentile sampling keeps running
either way; only the per-transaction print is gated. `LOG_SIGNATURES` is
ignored in this mode, since there is no line left to put a signature on.

If both `LOG_TRANSACTIONS=false` and `STATS_INTERVAL_SECS=0`, nothing is ever
printed — startup logs a warning for that combination, since it is almost
certainly not what was intended.

### Periodic stats

Every `STATS_INTERVAL_SECS` (default `10`), one `[STATS]` line per stream
reports p50/p95/p99 latency over the samples seen since the previous report —
not a running total since startup, so each line reflects that interval alone:

```
[STATS] S1 n=142 p50=+185ms p95=+320ms p99=+410ms
[STATS] S2 n=138 p50=+201ms p95=+350ms p99=+430ms
```

A stream with no resolved transactions in the interval reports `n=0 (no
samples since last report)` instead of stale numbers. Set
`STATS_INTERVAL_SECS=0` to disable this entirely.

## Running

```bash
cp .env.example .env      # then fill in GRPC_ENDPOINT and the two account lists
cargo run --release
```

## Configuration

| Variable | Required | Default | Meaning |
|---|---|---|---|
| `GRPC_ENDPOINT` | yes | — | Yellowstone gRPC endpoint |
| `ACCOUNT_INCLUDE_1` | yes | — | comma-separated accounts for stream 1 |
| `ACCOUNT_INCLUDE_2` | yes | — | comma-separated accounts for stream 2 |
| `GRPC_X_TOKEN` | no | unset | auth token, if the node requires one |
| `COMMITMENT` | no | `confirmed` | default level for all streams |
| `COMMITMENT_1` | no | `COMMITMENT` | override for stream 1 |
| `COMMITMENT_2` | no | `COMMITMENT` | override for stream 2 |
| `COMMITMENT_BLOCKS_META` | no | `COMMITMENT` | override for blocks_meta |
| `LATENCY_BUFFER_SLOTS` | no | `300` | slots of transactions held awaiting blocks_meta |
| `SLACK_WEBHOOK_URL` | no | unset | if set, disconnects are also posted here |
| `LOG_TRANSACTIONS` | no | `true` | `false` prints no per-transaction line at all |
| `LOG_SIGNATURES` | no | `true` | `false` drops `sig=...` from each latency line |
| `STATS_INTERVAL_SECS` | no | `10` | seconds between p50/p95/p99 reports; `0` disables |
| `RUST_LOG` | no | `info` | `debug` also logs every raw stream update |

Both account lists must be non-empty: an empty `account_include` on a
transactions filter means *every transaction on the chain*, which is never the
intent here, so startup fails instead.

## Behaviour under load

The reader tasks apply backpressure — if the event loop falls behind, the gRPC
streams block rather than dropping transactions. A dropped transaction would
otherwise vanish from the output with no trace.

All three streams reconnect indefinitely, 3s apart, on error or clean close.
Every disconnect is logged locally at `error` (or `warn` for a clean close);
when `SLACK_WEBHOOK_URL` is set, the same event is also posted there. Console
logging always happens regardless of Slack configuration.
