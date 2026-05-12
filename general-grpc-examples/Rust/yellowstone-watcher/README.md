# yellowstone-watcher

Streams Solana transactions from a **Yellowstone gRPC** node and reconciles
every completed block against the **Solana JSON-RPC** to detect any gaps.


## Build & Run

### How to run the project: 
```bash
# Add the details in the .env file and then run

cargo run

```


## Architecture

```
┌─────────────────────────┐          ┌──────────────────────────┐
│  Yellowstone gRPC node  │          │  Solana JSON-RPC node    │
│  (CommitmentLevel:      │          │  (Confirmed commitment)  │
│   CONFIRMED)            │          │                          │
└────────────┬────────────┘          └────────────┬─────────────┘
             │SubscribeUpdate stream              │
             ▼                                    │
  ┌──────────────────────┐                        │
  │   grpc_stream.rs     │  StreamEvent           │
  │  (reconnects on err) │ ──────────────┐        │
  └──────────────────────┘               ▼        │
                                 ┌──────────────────────┐
                                 │     main event loop  │
                                 │  slot_tracker.rs     │◄──── SlotData
                                 └──────────┬───────────┘
                                            │  after lag slots
                                            ▼
                                 ┌──────────────────────┐
                                 │   reconciler.rs      │────► RPC
                                 │  getSignaturesFor    │
                                 │  Address (per acct)  │
                                 └──────────────────────┘
```

### Flow

1. **gRPC stream** subscribes to:
   - `transactions` — non-vote, non-failed, filtered to `ACCOUNT_INCLUDE`
   - `slots` — to detect when a block is confirmed / rooted

2. **SlotTracker** accumulates signatures per slot in a `DashMap`.

3. When a `SlotConfirmed` event arrives and `RECONCILE_LAG_SLOTS` have elapsed,
   the slot is dequeued and handed to the **reconciler**.

4. **Reconciler** runs two complementary RPC strategies:
   - `getBlock` → signatures in the full block
   - `getSignaturesForAddress` for each watched account (filtered to that slot)
   Results are unioned, then diffed against the gRPC set.

5. Any signatures present in RPC but absent from gRPC are logged as **MISSED**.

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
| `RPC_SIGNATURES_LIMIT` | | 1000 | Max sigs per `getSignaturesForAddress` |
| `RUST_LOG` | | info | Tracing filter |

---

## Build & Run

### How to run the project: 
```bash
# Add the details in the .env file and then run

cargo run

```

### Optional
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

## Log Output

```
INFO  yellowstone_watcher > Watching 2 accounts  lag=5 slots
INFO  yellowstone_watcher > Connected. Streaming …
INFO  yellowstone_watcher > TX  slot=285123456  sig=5KtP…
INFO  yellowstone_watcher > SLOT confirmed=285123461
INFO  yellowstone_watcher > ✓ slot=285123456 grpc=3 rpc=3 CLEAN
ERROR yellowstone_watcher > ✗ slot=285123460 MISSED 1 transactions not seen via gRPC:
ERROR yellowstone_watcher >   missed sig=3xQr…
```
