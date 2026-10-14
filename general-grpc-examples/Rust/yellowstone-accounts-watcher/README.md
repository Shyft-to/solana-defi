# Yellowstone gRPC Account Verifier

A Rust tool that verifies Yellowstone gRPC account update delivery in real time. It streams account updates for a watched pubkey, then on every finalized slot calls `getAccountInfo` and compares the on-chain state against what gRPC delivered - telling you immediately whether a delivery gap occurred.

---

## The Problem It Solves

When you stream account updates from a Yellowstone gRPC node, you need confidence that every update was actually delivered - that nothing was silently dropped. This tool continuously verifies that in real time.

---

## How It Verifies

1. **Two streams open simultaneously** from the same gRPC connection:
   - A stream of account updates for your pubkey (fires whenever the account is written to on-chain)
   - A stream of slot finalization events (Solana finalizes a slot roughly every 400 ms)

2. **Every account update that arrives from gRPC is saved to memory**, keyed by the slot it came from. We keep the last 300 slots worth of data (configurable). This is our record of *what gRPC told us the account looked like, and when.*

3. **Every time a slot finalizes, we ask the RPC node directly:** *"What does this account look like right now?"* - this is a standard `getAccountInfo` call. The RPC responds with two things:
   - The account data bytes
   - A `context.slot` number - the exact finalized slot whose state this data reflects

4. **We look at our gRPC record** and find the most recent update we received for that pubkey, at or before `context.slot`.

5. **We compare the two:**
   - **Bytes match** → gRPC is in sync with the chain. No update was missed.
   - **Bytes don't match** → the account changed on-chain but gRPC never delivered the updated state. Confirmed delivery gap.

---

## The Three Outcomes

| Output | Meaning |
|---|---|
| `gRPC update present for this slot, data matches RPC \| OK` | gRPC delivered an update for this exact slot and it matches on-chain state. Perfect. |
| `data matches RPC \| last update received at slot X \| OK` | gRPC's last delivery was from an earlier slot, but the account hasn't changed since. gRPC correctly sent nothing. Still fine. |
| `MISS — gRPC data (from slot X) does not match RPC` | The on-chain state is ahead of what gRPC last delivered. A real update was dropped. |

---

## Why gRPC is `confirmed` but RPC is `finalized`

gRPC streams at `confirmed` commitment — updates arrive roughly 13 seconds before a slot is finalized. By the time the verifier checks a slot, the gRPC data is already sitting in memory — no race condition. If both were set to `finalized`, they would arrive at the same moment and the check would be unreliable.

---

## Prerequisites

- **Rust** 1.75 or later — install via [rustup.rs](https://rustup.rs)
- A **Yellowstone gRPC endpoint** (e.g. from [Shyft](https://shyft.to) or a self-hosted Geyser node)
- A **Solana JSON-RPC endpoint** that supports `getAccountInfo`

---

## Setup and Running

**1. Clone the repository**

```bash
git clone https://github.com/Shyft-to/solana-defi.git
cd solana-defi/general-grpc-examples/Rust/yellowstone-accounts-watcher
```

**2. Copy the example env file and fill it in**

```bash
cp .env.example .env
```

Open `.env` and set at minimum the three required variables:

```env
GRPC_ENDPOINT=https://grpc.ny.shyft.to
RPC_ENDPOINT=https://api.mainnet-beta.solana.com
TARGET_PUBKEY=6EF8rrecthR5Dkzon8Nwu78hRvfCKubJ14M5uBEwF6P
```

If your Yellowstone node requires an auth token, also set:

```env
GRPC_X_TOKEN=your_token_here
```

**3. Build and run**

```bash
cargo run --release
```

The tool starts immediately and runs indefinitely. It reconnects to the gRPC endpoint automatically if the stream drops.

---

## Configuration Reference

| Variable | Required | Default | Description |
|---|---|---|---|
| `GRPC_ENDPOINT` | Yes | — | Yellowstone gRPC URL |
| `RPC_ENDPOINT` | Yes | — | Solana JSON-RPC URL |
| `TARGET_PUBKEY` | Yes | — | Account pubkey to watch |
| `GRPC_X_TOKEN` | No | — | Auth token for the Yellowstone node |
| `GRPC_COMMITMENT` | No | `confirmed` | Commitment level for the gRPC account subscription |
| `RPC_COMMITMENT` | No | `finalized` | Commitment level for `getAccountInfo` calls |
| `GRPC_DATA_RETAIN_SLOTS` | No | `300` | Slots of gRPC data to keep in memory (~2 min at default). Increase if you see `no gRPC data in map` warnings |
| `RUST_LOG` | No | `info` | Log verbosity: `error` / `warn` / `info` / `debug` / `trace` |

---

## Sample Output

```
INFO  verifier started — target=6EF8rr... rpc=https://... grpc_commitment=confirmed rpc_commitment=finalized retain_slots=300

INFO  SLOT FINALIZED | slot=347291840
ACCOUNT UPDATE | slot=347291842 | sig=3xK9mNabc... | data=0102030405…abcdef (1544 bytes)
INFO  SLOT FINALIZED | slot=347291841
INFO  SLOT FINALIZED | slot=347291842
SLOT 347291842 | context_slot=347291842 | gRPC update present for this slot, data matches RPC | OK

INFO  SLOT FINALIZED | slot=347291870
SLOT 347291870 | context_slot=347291865 | data matches RPC | last update received at slot 347291842 | OK

INFO  SLOT FINALIZED | slot=347291900
SLOT 347291900 | context_slot=347291898 | MISS — gRPC data (from slot 347291842) does not match RPC
  gRPC (slot 347291842): 0102030405…abcd (1544 bytes)
  RPC  (slot 347291898): 0102030405…ff11 (1544 bytes)
```


## Architecture

```
┌─────────────────────────────────────────┐
│            Yellowstone gRPC             │
│  account updates (confirmed) + slots    │
└───────────────────┬─────────────────────┘
                    │
             ┌──────▼──────┐
             │  stream.rs  │  auto-reconnect, 15 s keepalive pings
             └──────┬──────┘
                    │ two mpsc channels
          ┌─────────┴──────────┐
          │                    │
     AccountUpdate        SlotFinalized
          │                    │
   ┌──────▼──────┐      ┌──────▼──────┐
   │  Task 1     │      │  Task 2     │
   │  recorder   │      │  verifier   │
   │             │      │             │
   │ grpc_data   │      │ getAccount  │
   │ (slot,pk)   │◄─────│ Info →      │
   │ → bytes     │      │ compare     │
   └─────────────┘      └─────────────┘
```

---

## Limitation

This tool verifies **end-of-slot state sync**, not per-transaction update delivery.

If multiple transactions write to the account within a single slot, the tool only checks whether the final account state at `context.slot` matches what gRPC last delivered — it does not verify that each individual transaction produced a corresponding gRPC update. If gRPC delivered some but not all updates within a slot, yet the final state still happens to match, this tool will report `OK`.

Use it to confirm that gRPC stays in sync with the chain over time. For transaction-level delivery auditing, a different approach (e.g. matching against `getBlock`) would be needed.

> **Note:** There is a subtle edge case where the result may appear out of sync without a real miss — if gRPC delivers a new update for the account at the exact moment `getAccountInfo` is executing, the two reads may reflect different slots. This would resolve itself on the next slot check.

---