# Wallet Trade Behavior on Pump.fun via Real-Time Buy and Sell Detection

This project demonstrates how to **track and interpret wallet trade behavior on Pump.fun** by streaming on-chain transactions in real time using **Shyft gRPC**.

Rather than simply detecting swaps, this example focuses on **what a wallet is doing in the market** — identifying buy and sell actions and enriching them with execution price, pool liquidity state, fees, and human-readable timestamps. All data is derived directly from streamed transactions, with no additional RPC calls.

It serves as a **practical, production-style reference** for developers building wallet monitoring tools, trading analytics, alerting systems, or on-chain intelligence around the Pump.fun ecosystem.

![screenshot](assets/run-project.png?raw=true "Screenshot")

---

## What This Example Does

- Streams Pump.fun AMM transactions in real time via **Shyft gRPC**
- Filters and detects **wallet-level buy and sell events**
- Interprets wallet trade behavior by extracting:
  - Wallet address and traded mint
  - Buy / sell direction
  - Input and output amounts
  - Execution price (SOL per token)
  - Pool liquidity snapshot at execution time (virtual & real reserves)
  - Protocol and creator fees
  - Unix timestamp and human-readable time
- Emits a clean, enriched trade object suitable for downstream consumption

---

## Why This Matters

Most examples stop at “a swap happened.”

This example answers:
- *Who traded?*
- *Was it a buy or a sell?*
- *At what price?*
- *Under what pool liquidity conditions?*
- *How much was paid in fees?*
- *When exactly did it happen?*

This makes it ideal for **wallet behavior analysis**, not just transaction parsing.
ps: `J5a5zpboEobw73uDVnWGBVxcezooC4agcrHKSZbpSorS` we use this address as a dummy wallet.
---

## Getting Started

### 1. Clone the repository

```bash
git clone https://github.com/Shyft-to/solana-defi.git
cd Wallet-based-examples/PumpFun/Typescript/stream_wallet_trade_behaviour_on_pump_fun

````
### 2. Install dependencies

```bash
npm install
```

### 3. Configure environment variables

Rename the sample environment file and add your credentials:

```bash
cp .env.sample .env
```

### 4. Run the script

```bash
npm run start
```

Once running, the script will continuously stream Pump.fun transactions and log **enriched wallet buy/sell events** as they occur.

---

## Output Overview

Each detected wallet trade includes contextual information such as:

* Wallet that executed the trade
* Token mint and bonding curve
* Buy or sell classification
* Execution price
* Pool liquidity state at trade time
* Fees paid
* Human-readable timestamp

This makes it easy to understand **what a wallet did, when it did it, and under what market conditions**.

---

## Use Cases

* Wallet trade behavior monitoring
* Pump.fun trader analytics
* Real-time alerts (large buys, early entries, exits)
* On-chain market intelligence
* Data pipelines for dashboards, bots, or research

---

## Related Links

* **Shyft Solana gRPC Documentation**
  [https://docs.shyft.to/solana-fast-grpc/grpc-docs](https://docs.shyft.to/solana-fast-grpc/grpc-docs)
* **Start Streaming with Shyft** [https://shyft.to/solana-yellowstone-grpc]
* **Shyft Website** [https://shyft.to/]
