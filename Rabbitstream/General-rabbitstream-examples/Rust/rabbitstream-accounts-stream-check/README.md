# rabbitstream-accounts-stream-check

Runs two independent Yellowstone gRPC subscriptions concurrently against the same watched address(es):

- a **transaction stream** (`ENDPOINT`, e.g. Rabbitstream — server-side filtered on `ACCOUNT_INCLUDE`), and
- an **account-update stream** (`ACCOUNTS_ENDPOINT`, since Rabbitstream itself typically only streams transactions), filtered directly on the same address(es).

Every account update carries the `txn_signature` of the transaction that produced it, so each signature can be correlated across both streams by arrival time. This answers two questions:

1. **Filter correctness** — does every transaction Rabbitstream sends actually reference a watched address? (see [Filter verification](#filter-verification) below)
2. **Arrival race** — for a given transaction, which stream reports it first, the transaction stream or the account-update stream — and by how much?

## Arrival race

For every signature seen on **both** streams, the arrival timestamps are compared and classified as `tx-first`, `account-first`, or (rarely) a tie. A signature seen on only one stream for longer than `MATCH_TIMEOUT_SECS` (default 10s) is counted as that stream's exclusive miss — it may still show up later, but the race for that signature is called at the timeout mark. Any signature still pending when the run ends is finalized the same way.

The two streams connect independently and neither backfills — whichever one finishes subscribing first can briefly see chain activity the other, not yet listening, will never receive. A one-sided signature first seen within `WARMUP_SECS` (default 5s) of startup is treated as this connection-startup skew rather than a genuine miss: it's excluded from `tx_only`/`account_only` and counted separately as `warmup_skipped` instead. A genuine match that happens to land inside the window still counts normally — only the miss classification is suppressed. Set `WARMUP_SECS=0` to disable.

The run summary's race table reports, per stream: how many signatures it won, what fraction of matched signatures that is, and — restricted to just the races it won — the p50/p95/p99 lead time (ms) over the other stream. See [Output](#output) for the full report layout.

## Filter verification

Verification is done against the transaction's static `message.account_keys`, because Rabbitstream's payloads omit `meta`. That means a transaction can genuinely include a watched address via an Address Lookup Table (ALT) — resolvable only through `meta.loaded_writable_addresses` / `meta.loaded_readonly_addresses`, which aren't available here — without it showing up in `account_keys`. Each transaction is therefore classified into one of three buckets:

- **Matched** — the address appears directly in `message.account_keys`.
- **Indeterminate** — not found in `account_keys`, but the transaction has `address_table_lookups`, so it may be a real match hidden behind an unresolved ALT.
- **Unmatched** — not found in `account_keys`, and no ALT lookups either, so the server-side filter and this check genuinely disagree — worth investigating.

Only the "unmatched" case is logged as a warning; "indeterminate" is logged at debug level since it's an expected consequence of the missing `meta`, not necessarily an anomaly. Both are live per-transaction log lines only — the totals aren't broken out in the final report.

## Setup

**1. Copy the example env file and fill in your values:**

```bash
cp .env.example .env
```

```env
ENDPOINT=https://your-endpoint.solana-mainnet.example.com:10000
X_TOKEN=your_x_token_here
ENDPOINT_NAME=rabbitstream
ACCOUNTS_ENDPOINT=https://your-other-endpoint.solana-mainnet.example.com:10000
ACCOUNTS_X_TOKEN=your_accounts_x_token_here
ACCOUNTS_ENDPOINT_NAME=yellowstone
ACCOUNT_INCLUDE=6EF8rrecthR5Dkzon8Nwu78hRvfCKubJ14M5uBEwF6P,pAMMBay6oceH9fJKBRHGP5D4bD4sWpmSwMn52FMfXEA
LOG_SIG=true
STATS_INTERVAL_SECS=5
MATCH_TIMEOUT_SECS=10
WARMUP_SECS=5
PRINT_MISSED_SIGS=true
RUN_DURATION_MINS=60
```

| Variable | Description | Default |
|---|---|---|
| `ENDPOINT` | gRPC endpoint for the transaction stream (must include port) | required |
| `X_TOKEN` | Auth token for `ENDPOINT` | required |
| `ENDPOINT_NAME` | Optional display name for `ENDPOINT` in logs and the summary | endpoint URL |
| `ACCOUNTS_ENDPOINT` | gRPC endpoint for the account-update stream (must include port) | required |
| `ACCOUNTS_X_TOKEN` | Auth token for `ACCOUNTS_ENDPOINT` | required |
| `ACCOUNTS_ENDPOINT_NAME` | Optional display name for `ACCOUNTS_ENDPOINT` in logs and the summary | endpoint URL |
| `ACCOUNT_INCLUDE` | Comma-separated program/account addresses watched on both streams | required |
| `LOG_SIG` | `true` to print each transaction signature, `false` to suppress | `true` |
| `STATS_INTERVAL_SECS` | How often to print throughput stats for both streams (seconds) | `5` |
| `MATCH_TIMEOUT_SECS` | Seconds to wait for a signature on the other stream before counting it stream-exclusive | `10` |
| `WARMUP_SECS` | Seconds after startup during which a one-sided signature is treated as connection skew, not a miss | `5` |
| `PRINT_MISSED_SIGS` | `true` to print every account-only-miss signature (with timestamp) at the end, `false` to keep just the count | `true` |
| `RUN_DURATION_MINS` | Auto-stop after this many minutes, omit to run forever | — |

**2. Run:**

```bash
cargo run
```

You can also pass values directly as flags (these override `.env`):

```bash
cargo run -- \
  --endpoint <TX_URL> \
  --x-token <TX_TOKEN> \
  --endpoint-name rabbitstream \
  --accounts-endpoint <ACCOUNTS_URL> \
  --accounts-x-token <ACCOUNTS_TOKEN> \
  --accounts-endpoint-name yellowstone \
  --account-include <ADDR1>,<ADDR2> \
  --log-sig true \
  --stats-interval-secs 1 \
  --match-timeout-secs 10 \
  --warmup-secs 5 \
  --print-missed-sigs true \
  --run-duration-mins 60
```

## Output

Each received transaction prints its signature (when `LOG_SIG=true`):

```
[INFO  rabbitstream_accounts_stream_check] 2TNkmwSZ...
```

Every `STATS_INTERVAL_SECS` seconds a throughput line is printed for each stream:

```
[INFO  rabbitstream_accounts_stream_check] -----> throughput: 12.4 tx/s | total transactions: 62 <------
[INFO  rabbitstream_accounts_stream_check] -----> account throughput: 11.9 upd/s | total account updates: 59 <------
```

A transaction that genuinely doesn't reference any `ACCOUNT_INCLUDE` address (no ALT lookups to blame) is flagged:

```
[WARN  rabbitstream_accounts_stream_check] [us-east-1] ⚠️ transaction does not include any account_include address (no ALT lookups either): 2TNkmwSZ...
```

(Run with `RUST_LOG=debug` to also see the "indeterminate" cases — matched addresses that couldn't be confirmed because the transaction relies on an unresolved ALT.)

On exit, a single combined report is printed: throughput stats for both streams (once at least two stats windows have elapsed each) and the account-vs-transaction arrival race table:

```
================================ Run Summary ================================
  Address(es) watched        : 6EF8rrecthR5Dkzon8Nwu78hRvfCKubJ14M5uBEwF6P, pAMMBay6oceH9fJKBRHGP5D4bD4sWpmSwMn52FMfXEA
  Duration                   : 3600s

  Transactions received      : 620
  Tx avg/peak/min tx/s       : 14.2 / 31.0 / 4.5

  Account updates received   : 611
  Account avg/peak/min upd/s : 13.5 / 29.4 / 4.1

  Matched signatures (seen on both streams) : 604
  Tx-stream only (no account match)         : 12
  Account-stream only (no tx match)         : 4
  Skipped as startup warm-up skew           : 3

  Arrival race — lead time when each stream reported the signature first:
  Stream    Name                                   Total txns  Ahead %   p99 (ms)   p95 (ms)   p50 (ms)
  Tx        rabbitstream                                  401    66.4%      210.7       96.2       18.4
  Account   yellowstone                                   203    33.6%      205.3       91.0       17.9
================================================================================

Signatures seen on the account stream but never on the tx stream (4):
2026-09-03T08:11:42.104Z  5h3v9K...
2026-09-03T08:11:42.611Z  7bWn2T...
2026-09-03T08:11:43.002Z  2TNkmwSZ...
2026-09-03T08:11:44.887Z  9xQpLm...
```

After the report, if `Account-stream only (no tx match)` is non-zero and `PRINT_MISSED_SIGS=true` (the default), every one of those signatures is printed with the wall-clock time (RFC3339, millisecond precision — matching the log's own timestamp format) it was first seen on the account stream, so you can jump straight to that moment in your logs. One per line, easy to copy or pipe into another tool for investigation. Set `PRINT_MISSED_SIGS=false` to keep just the count in the summary and skip the dump.

`ENDPOINT_NAME` / `ACCOUNTS_ENDPOINT_NAME` are optional — leave either unset and that row falls back to showing the endpoint URL (scheme stripped) instead of a name, everywhere identity is displayed: the startup log lines, the summary header, and the race table.

Both streams reconnect automatically and independently on errors or if no messages are received for 30 seconds; a disconnect on one stream doesn't interrupt the other. All counters, both throughput histories, and the race tracker persist across reconnects, so the report reflects the whole run.
