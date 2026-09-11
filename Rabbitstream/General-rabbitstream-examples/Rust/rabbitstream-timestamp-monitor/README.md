# rabbitstream-streamer

Streams Solana transactions via [Yellowstone gRPC](https://github.com/rpcpool/yellowstone-grpc) filtered to `ACCOUNT_INCLUDE`, prints signatures with throughput stats, and verifies that every received transaction actually references one of those addresses — useful for checking Rabbitstream's server-side filtering.

Verification is done against the transaction's static `message.account_keys`, because Rabbitstream's payloads omit `meta`. That means a transaction can genuinely include a watched address via an Address Lookup Table (ALT) — resolvable only through `meta.loaded_writable_addresses` / `meta.loaded_readonly_addresses`, which aren't available here — without it showing up in `account_keys`. Each transaction is therefore classified into one of three buckets:

- **Matched** — the address appears directly in `message.account_keys`.
- **Indeterminate** — not found in `account_keys`, but the transaction has `address_table_lookups`, so it may be a real match hidden behind an unresolved ALT.
- **Unmatched** — not found in `account_keys`, and no ALT lookups either, so the server-side filter and this check genuinely disagree — worth investigating.

Only the "unmatched" case is logged as a warning; "indeterminate" is logged at debug level since it's an expected consequence of the missing `meta`, not necessarily an anomaly. All three totals are reported in the run summary.

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

A transaction that genuinely doesn't reference any `ACCOUNT_INCLUDE` address (no ALT lookups to blame) is flagged:

```
[WARN  rabbitstream_streamer] [us-east-1] ⚠️ transaction does not include any account_include address (no ALT lookups either): 2TNkmwSZ...
```

(Run with `RUST_LOG=debug` to also see the "indeterminate" cases — matched addresses that couldn't be confirmed because the transaction relies on an unresolved ALT.)

On exit, a summary is printed with the verification breakdown (and throughput stats, once at least two stats windows have elapsed):

```
========== Run Summary ==========
  Endpoint                   : https://your-endpoint.solana-mainnet.example.com:10000
  Address(es) verified       : 6EF8rrecthR5Dkzon8Nwu78hRvfCKubJ14M5uBEwF6P, pAMMBay6oceH9fJKBRHGP5D4bD4sWpmSwMn52FMfXEA
  Transactions received      : 620
  Matched (static keys)      : 540 (87.1%)
  Indeterminate (ALT lookup) : 78 (12.6%)
  Unmatched (no ALT either)  : 2 (0.3%)
  Duration : 3600s
  Avg tx/s : 14.2
  Peak tx/s: 31.0
  Min  tx/s: 4.5
=================================
```

The streamer automatically reconnects on errors or if no messages are received for 30 seconds. Received/matched/indeterminate/unmatched counts and the throughput history persist across reconnects, so the summary reflects the whole run.
