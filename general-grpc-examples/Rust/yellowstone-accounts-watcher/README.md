# Yellowstone gRPC Account Verifier

A Rust tool that verifies Yellowstone gRPC account update delivery against on-chain block data. For every finalized slot it checks that every transaction which wrote to a watched account also produced a gRPC update — and when it finds a gap, it compares account data snapshots to tell you whether the account actually changed.

---

## How It Works

### Plain English

1. **Subscribe** to Yellowstone gRPC for two things at once: account updates for your watched pubkeys, and slot finalization events.
2. **Every account update** that arrives is recorded in memory: `(slot, pubkey) → [transaction signatures]`.
3. **Every time a slot finalizes**, two things happen in parallel:
   - The **Account Fetcher** immediately calls `getMultipleAccounts` for all watched pubkeys and saves a snapshot of their raw data at that slot.
   - The **Verifier** waits 2 seconds (for the RPC to catch up), then calls `getBlock` to get every transaction in that slot.
4. **The Verifier** scans the block for transactions that listed a watched pubkey as writable (skipping failed txns and vote txns) and compares that count against how many gRPC updates arrived.
   - Counts match → `OK`.
   - gRPC sent fewer updates → `NO_GRPC_UPDATE`, and we dig deeper.
5. **Dig deeper** — compare the account's raw data between slot N-1 and slot N:
   - **Data unchanged** → the transaction listed the account as writable but didn't actually modify it. Yellowstone correctly sent no update. Not a bug.
   - **Data changed** → the account was genuinely modified but no gRPC update arrived. Real delivery gap.
   - If the snapshot isn't ready yet → pushed to a **retry queue** that keeps checking until data arrives (up to 30 s).

---

### Technical Detail

#### Stream 1 — Yellowstone gRPC

A single gRPC subscription covers two filter types:

- **Account updates** for the target pubkey(s) at `Confirmed` commitment. Each `AccountUpdate` carries the slot, pubkey, and transaction signature.
- **Slot status updates** — the tool listens for `SlotFinalized` events. Every `SlotFinalized` is broadcast to both the Account Fetcher and the Verifier via separate channels.

#### Stream 2 — Account Fetcher (`fetcher.rs`)

On every `SlotFinalized` event, a dedicated task calls `getMultipleAccounts` for all watched pubkeys and stores `(slot, pubkey) → Option<Vec<u8>>` in a shared map. Snapshots for the last 50 slots are kept; older ones are evicted automatically.

#### Stream 3 — Slot Verifier (`verifier.rs`)

On every `SlotFinalized` event (after a 2 s delay), calls `getBlock(N)` with full transaction details. For each transaction it:

1. **Skips failed transactions** — `meta.err` is set.
2. **Skips vote transactions** — detected by the Vote program ID in the static account keys.
3. **Derives the writable account set** using the Solana message header formula:
   - Writable signers: `account_keys[0 .. num_required_signatures - num_readonly_signed_accounts]`
   - Writable non-signers: `account_keys[num_required_signatures .. total - num_readonly_unsigned_accounts]`
   - ALT-resolved writable: `meta.loadedAddresses.writable`
4. If a watched pubkey is in the writable set, that transaction counts as one expected update.

#### Stream 4 — Comparison Worker (`verifier.rs`)

When a `NO_GRPC_UPDATE` gap is found and the account-data snapshots are not yet in the map (fetcher RPC still in flight), the `(slot, pubkey)` pair is pushed to a pending queue. A dedicated worker retries the comparison every 500 ms for up to 30 s, then gives up with a warning.

---

## Output

**All gRPC updates accounted for:**
```
SLOT 428979620 | pubkey 6EF8rr...ewF6P | gRPC: 3 | rpc_writable: 3 | OK
```

**Gap found — account data did not change (writable-but-no-modify, expected):**
```
SLOT 428979621 | pubkey 6EF8rr...ewF6P | gRPC: 1 | rpc_writable: 2 | NO_GRPC_UPDATE (delta: 1)
  pubkey 6EF8rr...ewF6P | TXN 5Kx9abc... — in writable set, account may not have been modified. No update from gRPC.
  INFO  pubkey 6EF8rr...ewF6P | account data UNCHANGED in slot 428979621 — writable-but-no-modify, no real gap
```

**Gap found — account data changed (real delivery gap):**
```
SLOT 428979622 | pubkey 6EF8rr...ewF6P | gRPC: 0 | rpc_writable: 1 | NO_GRPC_UPDATE (delta: 1)
  pubkey 6EF8rr...ewF6P | TXN 3Abc... — in writable set, account may not have been modified. No update from gRPC.
  ERROR pubkey 6EF8rr...ewF6P | account data CHANGED between slot 428979621 and 428979622 — likely a real gRPC delivery gap
```

### Fields

| Field | Meaning |
|---|---|
| `gRPC` | Account update events delivered by Yellowstone for this pubkey in this slot |
| `rpc_writable` | Successful, non-vote transactions in the block that listed this pubkey as writable |
| `delta` | `rpc_writable - gRPC` — how many transactions are unaccounted for |

---

## Architecture

```
┌─────────────────────────────────────────────┐
│               Yellowstone gRPC              │
│   (account updates + slot finalization)     │
└────────────────────┬────────────────────────┘
                     │
              ┌──────▼──────┐
              │  stream.rs  │  auto-reconnect, 15 s keepalive pings
              └──────┬──────┘
                     │ three mpsc channels
       ┌─────────────┼──────────────┐
       │             │              │
  AccountUpdate  SlotFinalized  SlotFinalized
       │             │              │
  ┌────▼────┐   ┌────▼────┐   ┌────▼──────┐
  │ main.rs │   │fetcher  │   │ verifier  │
  │ DashMap │   │per-slot │   │ per-slot  │
  │(slot,pk)│   │getMulti │   │ getBlock  │
  │→ [sigs] │   │Accounts │   │ +compare  │
  └─────────┘   └────┬────┘   └─────┬─────┘
                     │              │ NO_GRPC_UPDATE
                     │  AccountState│ → pending queue
                     │     Map      │
                     └──────────────┤
                                    ▼
                           ┌────────────────┐
                           │comparison worker│
                           │ retries up to  │
                           │     30 s       │
                           └────────────────┘
```

---

## Prerequisites

- **Rust** 1.75 or later — install via [rustup.rs](https://rustup.rs)
- A **Yellowstone gRPC endpoint** (e.g. from [Triton One](https://triton.one) or a self-hosted Geyser node)
- A **Solana JSON-RPC endpoint** that supports `getBlock` with `max_supported_transaction_version: 0`

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

| Variable | Required | Description |
|---|---|---|
| `GRPC_ENDPOINT` | Yes | Yellowstone gRPC URL, e.g. `https://grpc.ny.shyft.to` |
| `RPC_ENDPOINT` | Yes | Solana JSON-RPC URL, e.g. `https://api.mainnet-beta.solana.com` |
| `TARGET_PUBKEYS` | Yes | Comma-separated list of account pubkeys to watch |
| `GRPC_X_TOKEN` | No | Auth token for the Yellowstone node (omit if not required) |
| `RUST_LOG` | No | Log verbosity — `info` (default), `debug` for verbose output |

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

```bash
cargo run --release
```

The tool runs indefinitely and reconnects to the gRPC endpoint automatically if the stream drops.
