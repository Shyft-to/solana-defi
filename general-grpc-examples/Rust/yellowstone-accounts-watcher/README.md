# Yellowstone gRPC Account Verifier

A Rust tool that cross-checks Yellowstone gRPC account update delivery against on-chain block data. For every finalized slot, it fetches the block via JSON-RPC and verifies that every transaction which declared a watched account as **writable** also produced a gRPC account update.

---

## How It Works

The tool runs two concurrent data streams and compares what each one sees for every finalized slot.

### Stream 1 — Yellowstone gRPC (account + slot updates)

A single gRPC subscription is opened to the Yellowstone node. It subscribes to two things at once:

- **Account updates** for the target pubkey(s) at `Confirmed` commitment. Every time a transaction touches a watched account, Yellowstone fires an `AccountUpdate` containing the slot number, the account pubkey, and the transaction signature.
- **Slot status updates** (all commitment levels). The tool listens for `SlotFinalized` events to know when a slot is permanently settled on-chain.

Each account update is recorded in an in-memory map keyed by `(slot, pubkey)`.

### Stream 2 — Solana JSON-RPC (`getBlock`)

When a `SlotFinalized` event arrives for slot N, the tool waits 2 seconds (to let the RPC node catch up), then calls `getBlock(N)` with full transaction details.

For each transaction in the block the tool:

1. **Skips failed transactions** — `meta.err` is set, so the account was never actually modified.
2. **Skips vote transactions** — detected by the presence of the Vote program (`Vote111...`) in the static account keys.
3. **Derives the writable account set** for each remaining transaction using the Solana message header formula:
   - Writable signers: `account_keys[0 .. num_required_signatures - num_readonly_signed_accounts]`
   - Writable non-signers: `account_keys[num_required_signatures .. total - num_readonly_unsigned_accounts]`
   - ALT-resolved writable: `meta.loadedAddresses.writable` (for versioned transactions using Address Lookup Tables)
4. **Checks if a watched pubkey appears in this writable set.** If it does, that transaction is counted as a candidate — one for which Yellowstone should have fired an account update.

### Comparison and Output

For each `(slot, pubkey)` pair:

```
grpc_count    = number of account updates received from gRPC for this pubkey in this slot
rpc_writable  = number of transactions in the block that had this pubkey in the writable set
                (excluding failed and vote transactions)
```

> **Important:** `rpc_writable` is not the same as "account was modified". A transaction can list an account as writable without actually changing its state. The tool flags transactions where the pubkey was in the writable set but no gRPC update arrived — the account may or may not have been modified. This is intentional: Yellowstone only fires updates when the account state actually changes, so a zero-delta writable is expected to produce no update.

| Condition | Output |
|---|---|
| `grpc_count == rpc_writable` | `OK` |
| `grpc_count < rpc_writable` | `NO_GRPC_UPDATE` — lists the specific transaction signatures |
| Slot has no activity for this pubkey | Silent (no output) |

---

## Prerequisites

- **Rust** 1.75 or later — install via [rustup.rs](https://rustup.rs)
- A **Yellowstone gRPC endpoint** (e.g. from [Triton One](https://triton.one) or a self-hosted Geyser node)
- A **Solana JSON-RPC endpoint** that supports `getBlock` with `max_supported_transaction_version: 0` (public mainnet endpoint works, a private one is recommended for production)

---

## Setup

**1. Clone the repository**

```bash
git clone https://github.com/Shyft-to/solana-defi.git
cd solana-defi/general-grpc-examples/Rust/yellowstone-accounts-watcher
```

**2. Copy the example env file**

```bash
cp .env.example .env
```

**3. Fill in `.env`**

Open `.env` and set the following values:

| Variable | Required | Description |
|---|---|---|
| `GRPC_ENDPOINT` | Yes | Yellowstone gRPC URL, e.g. `https://grpc.ny.shyft.to` |
| `RPC_ENDPOINT` | Yes | Solana JSON-RPC URL, e.g. `https://api.mainnet-beta.solana.com` |
| `TARGET_PUBKEYS` | Yes | Comma-separated list of account pubkeys to watch |
| `GRPC_X_TOKEN` | No | Auth token for the Yellowstone node (omit if not required) |
| `RUST_LOG` | No | Log verbosity — `info` (default), `debug` for ping/pong details |

Example `.env`:

```env
GRPC_ENDPOINT=https://grpc.ny.shyft.to
RPC_ENDPOINT=https://api.mainnet-beta.solana.com
TARGET_PUBKEYS=6EF8rrecthR5Dkzon8Nwu78hRvfCKubJ14M5uBEwF6P,DNfuF1L62WWyW3pNakVkyGGFzVVhj4Yr52jSmdTyeBHm
GRPC_X_TOKEN=your_token_here
RUST_LOG=info
```

---

## Running

**Build and run in one step:**

```bash
cargo run --release
```

**Or build first, then run:**

```bash
cargo build --release
./target/release/yellowstone-grpc-verifier
```

The tool runs indefinitely. It will automatically reconnect to the gRPC endpoint if the stream drops.

---

## Reading the Output

**Normal activity — all gRPC updates accounted for:**
```
SLOT 428979620 | pubkey 6EF8rr...ewF6P | gRPC: 3 | rpc_writable: 3 | OK
```

**Gap detected — one or more transactions had the pubkey in the writable set but no gRPC update arrived:**
```
SLOT 428979621 | pubkey 6EF8rr...ewF6P | gRPC: 1 | rpc_writable: 2 | NO_GRPC_UPDATE (delta: 1)
  pubkey 6EF8rr...ewF6P | TXN 5Kx9abc... — in writable set, no gRPC update received (account may not have been modified)
```

**Warning — getBlock failed for a slot where gRPC had updates (rare):**
```
WARN SLOT 428979622 | getBlock failed (gRPC had 1 updates): ...
```

### Fields explained

| Field | Meaning |
|---|---|
| `gRPC` | Number of account update events delivered by Yellowstone for this pubkey in this slot |
| `rpc_writable` | Number of successful, non-vote transactions in the block that listed this pubkey as writable |
| `delta` | `rpc_writable - gRPC` — how many transactions are unaccounted for |

### Why `NO_GRPC_UPDATE` is not necessarily a bug

Yellowstone fires account updates only when the account state **actually changes** (lamports, data, owner, etc.). A transaction can declare an account writable without modifying it (the runtime allows this). So a `NO_GRPC_UPDATE` result means:

- Either the account was listed as writable but not actually written — Yellowstone correctly sent no update.
- Or the account was written but the gRPC update was dropped — a real delivery gap.

To distinguish the two cases, compare the account's pre- and post-state for that transaction using the block data from `getBlock`.

---

## Architecture

```
┌─────────────────────────────────────────────┐
│               Yellowstone gRPC              │
│   (account updates + slot finalization)     │
└────────────────────┬────────────────────────┘
                     │
              ┌──────▼──────┐
              │  stream.rs  │  decodes pubkey + sig from each update
              └──────┬──────┘
                     │ mpsc channels
          ┌──────────┴──────────┐
          │                     │
   AccountUpdate             SlotFinalized
          │                     │
   ┌──────▼──────┐       ┌──────▼──────┐
   │  main.rs    │       │ verifier.rs │
   │  DashMap    │◄──────│  verify_slot│
   │(slot,pubkey)│       │  (per slot) │
   │  → [sigs]   │       └──────┬──────┘
   └─────────────┘              │ getBlock (once per slot)
                                │
                     ┌──────────▼──────────┐
                     │   Solana JSON-RPC   │
                     └─────────────────────┘
```
