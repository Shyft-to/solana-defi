# 🚀 Create and Deploy Token on Pump.fun

A Rust code example for creating and deploying tokens on [Pump.fun](https://pump.fun).

## Features
- ✅ Create tokens (`create` and `create_v2`)
- ✅ Live pool verification after deployment
- ✅ Devnet & Mainnet support

## Environment Setup
Create a `.env` file in your project root:

```dotenv
# Your wallet private key (base58 encoded) 
PRIVATE_KEY=YOUR_BASE58_PRIVATE_KEY_HERE

# RPC endpoint — get a free key at shyft.to or helius.dev
RPC_URL=https://devnet-rpc.shyft.to?api_key=YOUR_API_KEY

# Pump.fun program ID 
PUMP_FUN_PROGRAM=6EF8rrecthR5Dkzon8Nwu78hRvfCKubJ14M5uBEwF6P

# Optional: Jito MEV (mainnet only)
JITO_UUID=
JITO_BLOCK_ENGINE_URL=https://mainnet.block-engine.jito.wtf
```

## Usage

Run the interactive CLI:
```bash
cargo run
```
Or run the example directly:
```bash
cargo run --example create_token
```

Both commands walk you through the same prompts. Here is exactly what to expect:

---

### Step 1 — Network selection

```
📝 Creator Wallet: YOUR_WALLET

Select network:
1. Devnet (testing)
2. Mainnet (real)
Choice:
```
Type `1` for Devnet or `2` for Mainnet and press Enter.

---

### Step 2 — Create method

```
Select create method:
1. Create_v2 
2. Create 
Choice:
```
- **`1` — create_v2**
- **`2` — create** 

---

### Step 3 — Token details

```
Token name: My Token
Token symbol: MYTKN
Token URI (metadata): https://your-metadata-uri.com/token.json
```
Enter your token name , symbol, and a publicly accessible metadata URI (click enter at interval). The URI should point to a JSON file with at least `name`, `symbol`, `description`, and `image` fields.

---

### Step 4 — Mayhem & cashback (`create_v2` only)

If you selected `create_v2` you will also be asked (only when you run `cargo run`.. `cargo run --exampe create_token` already have redefined mayhem and cashback):

```
Mayhem mode? (y/n, default: n):
Cashback enabled? (y/n, default: n):
```
- Type `y` or `n` for each and press Enter.
- For standard token creation leave both as `n`.
- Mayhem mode enables the Pump.fun mayhem program integration.
- Cashback routes a portion of fees back to the creator.

> If you selected **Create** these two prompts are skipped entirely.

---

### Step 5 — Result

On success you will see:
```
✅ TOKEN CREATED SUCCESSFULLY!
   Mint: 9X3yZ5gfr4oKBApyEEMWVjWdwmzNrk66N6TtrTMgmj9R
   Signature: 5Zy...
   Explorer: https://explorer.solana.com/tx/...?cluster=devnet

📝 Save these values:
   MINT=9X3yZ5gfr4oKBApyEEMWVjWdwmzNrk66N6TtrTMgmj9R
   BONDING_CURVE=GzLP7XnEduyKmSWFXb968bYfnd8fzHFKSfzu3iyfTWhF
```
Save the mint address — you will need it for any further interactions with the token.

> 💡 Test on **Devnet** first. You need at least **0.1 SOL** to cover rent and fees. Get free Devnet SOL at [faucet.solana.com](https://faucet.solana.com).

---

## Switching to Mainnet

Two things to update:

1. **`RPC_URL`** in your `.env` — swap to a mainnet endpoint e.g. `https://rpc.shyft.to?api_key=YOUR_API_KEY`
2. **Priority fee** in `client.rs` — increase from `5_000` to at least `50_000` microlamports, mainnet is competitive and low fees will get your transaction dropped

The program ID is the same on both networks. No other changes needed.

ps: the project is set to dev net. recommended you run first on devnet before mainnet
## Related Links

- [Shyft gRPC Docs](https://docs.shyft.to/solana-fast-grpc/grpc-docs)
- [Start Streaming with Shyft](https://shyft.to/solana-yellowstone-grpc)
- [Shyft Website](https://shyft.to/)
