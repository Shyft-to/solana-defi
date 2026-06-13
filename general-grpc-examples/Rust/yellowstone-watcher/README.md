# yellowstone-watcher

Streams Solana transactions from a **Yellowstone gRPC** node and reconciles
every completed block against the **Solana JSON-RPC** to detect any gaps.


## Build & Run

```bash
# Add the details in the .env file and then run
# or rename the .env.example file to .env and add your details, then run using the command below
cargo run
```

### Release build (optional)
```bash
# 1. Install the Solana BPF toolchain (needed for solana-* crate linking)
sh -c "$(curl -sSfL https://release.solana.com/stable/install)"

# 2. Build
cargo build --release

# 3. Run
cp .env.example .env        # edit as needed
export $(grep -v '^#' .env | xargs)
./target/release/yellowstone-watcher
```

---

## Architecture

```
┌─────────────────────────┐          ┌──────────────────────────┐
│  Yellowstone gRPC node  │          │  Solana JSON-RPC node    │
│  (CommitmentLevel:      │          │  (Confirmed commitment)  │
│   CONFIRMED)            │          │                          │
└────────────┬────────────┘          └────────────┬─────────────┘
             │SubscribeUpdate stream              │
             ▼                                   │
  ┌──────────────────────┐                       │
  │   grpc_stream.rs     │  StreamEvent          │
  │  (reconnects on err) │ ──────────────┐       │
  └──────────────────────┘               ▼       │
                                 ┌──────────────────────┐
                                 │     main event loop  │
                                 │  slot_tracker.rs     │◄──── SlotData
                                 └──────────┬───────────┘
                                            │  after lag slots
                                            ▼
                                 ┌──────────────────────┐
                                 │   reconciler.rs      │────► RPC
                                 │  getBlock            │
                                 │    OR                │
                                 │  getSignaturesFor    │
                                 │  Address (per acct)  │
                                 └──────────────────────┘
```

### Flow

1. **gRPC stream** subscribes to `transactions` — non-vote, non-failed, filtered
   to `ACCOUNT_INCLUDE` at `Confirmed` commitment. Reconnects automatically on error.

2. **SlotTracker** accumulates signatures per slot in a `DashMap`.

3. When a transaction arrives in a slot that is at least `RECONCILE_LAG_SLOTS`
   behind the current chain tip, that slot is dequeued and handed to the **reconciler**.

4. **Reconciler** fetches the RPC view of that slot using one of two strategies
   (controlled by `USE_GET_BLOCK`):
   - `getSignaturesForAddress` (default) — one paginated call per watched account,
     filtered to the target slot.
   - `getBlock` — fetches the entire block, then filters locally to transactions
     touching a watched account (excluding failed and vote transactions).

5. Any signatures present in the RPC result but absent from gRPC are logged as **MISSED**.
   Signatures seen by gRPC but not confirmed by RPC are logged as **warnings**.

---

## Configuration

Copy `.env.example` → `.env` and fill in your values, or export the env vars
directly.

| Variable | Required | Default | Description |
|---|---|---|---|
| `GRPC_ENDPOINT` | ✓ | — | Yellowstone gRPC URL |
| `GRPC_X_TOKEN` | | — | Auth token for the gRPC node |
| `ACCOUNT_INCLUDE` | ✓ | — | Comma-separated pubkeys to watch |
| `RPC_ENDPOINT` | | mainnet-beta | Solana JSON-RPC URL |
| `RECONCILE_LAG_SLOTS` | | 5 | Slots to wait before reconciling |
| `RPC_DELAY_SECS` | | 5 | Seconds to wait before querying RPC after a slot is ready |
| `USE_GET_BLOCK` | | false | Use `getBlock` instead of `getSignaturesForAddress` |
| `RPC_SIGNATURES_LIMIT` | | 1000 | Max sigs per `getSignaturesForAddress` page (ignored when `USE_GET_BLOCK=true`) |
| `RUST_LOG` | | info | Tracing filter |

---

## Log Output

```
INFO  yellowstone_watcher > Watching 2 accounts  lag=5 slots
INFO  yellowstone_watcher > Connecting to Yellowstone gRPC at https://…
INFO  yellowstone_watcher > Subscribed. Streaming …
INFO  yellowstone_watcher > Chain advanced to slot 285123456 — currently buffering 3 slots waiting for verification
INFO  yellowstone_watcher > Slot 285123451 verified cleanly — 3 transactions matched — latency min=45ms avg=62ms max=81ms
INFO  yellowstone_watcher >   5KtP…  latency=45ms
ERROR yellowstone_watcher > Slot 285123450 is missing 1 transaction(s) that the RPC confirmed but the gRPC stream never delivered — possible data loss:
ERROR yellowstone_watcher >   3xQr…
WARN  yellowstone_watcher > Slot 285123452 has 1 transaction(s) seen via gRPC that the RPC does not recognise — may be a timing issue:
WARN  yellowstone_watcher >   7mNz…
```
