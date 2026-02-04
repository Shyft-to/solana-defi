# Wallet Trade Behavior on Pump.fun via Real-Time Buy and Sell Detection

This project provides a **production-style reference implementation** for tracking and interpreting **wallet trade behavior on Pump.fun** in real time using **Shyft’s Solana gRPC streaming API**.

Instead of merely detecting swaps, the focus is on understanding **what a wallet is doing in the market**. The system classifies buy and sell actions and enriches each trade with execution price, pool liquidity state, fees, and precise timestamps — all derived directly from streamed on-chain transactions, with **no additional RPC calls**.

This makes the project well suited for developers building **wallet monitoring tools, trading analytics, alerting systems, or on-chain intelligence** around the Pump.fun ecosystem.

![screenshot](assets/usage-screenshot.png?raw=true "Example Output")

---

## Overview

**Key characteristics**

- Real-time streaming of Pump.fun AMM transactions via **Shyft gRPC**
- Implemented entirely in **Rust**
- Wallet-centric trade detection (buy / sell)
- Fully enriched trade context extracted from on-chain data only

---

## What the Example Does

- Subscribes to Pump.fun program activity using Shyft gRPC
- Detects wallet-level **buy** and **sell** instructions
- Extracts and enriches each trade with:
  - Wallet address (trader)
  - Token mint and bonding curve
  - Buy or sell direction
  - Input and output amounts
  - Execution price (SOL per token)
  - Pool liquidity state at execution time  
    (virtual and real reserves)
  - Protocol and creator fees
  - Unix timestamp and human-readable timestamp
- Emits a clean, structured trade object suitable for downstream use

---

## Why This Matters

Most on-chain examples stop at *“a swap happened.”*  
This project answers deeper questions:

- **Who** executed the trade?
- **Was it a buy or a sell?**
- **At what price was it executed?**
- **What was the pool liquidity state at that moment?**
- **How much was paid in protocol and creator fees?**
- **Exactly when did the trade occur?**

By focusing on **wallet behavior**, this example goes beyond transaction parsing and enables meaningful market and trader analysis.

> **Note**  
> The wallet address  
> `Gygj9QQby4j2jryqyqBHvLP7ctv2SaANgh4sCb69BUpA`  
> is used as a **dummy wallet** for demonstration purposes.

---

## Running the Example

```bash
cargo run -- --endpoint <endpoint> --x-token <token>
````

Once running, the application continuously streams Pump.fun transactions and logs **enriched wallet buy/sell events** as they occur.

---

## Output

Each detected trade includes:

* Wallet that executed the trade
* Token mint and bonding curve
* Buy or sell classification
* Execution price
* Pool liquidity snapshot at trade time
* Protocol and creator fees
* Unix and human-readable timestamps

The output is designed to be easily consumed by:

* Analytics pipelines
* Alerting systems
* Trading bots
* Dashboards and research tools

---

## Use Cases

* Wallet trade behavior monitoring
* Pump.fun trader analytics
* Real-time alerts (large buys, early entries, exits)
* On-chain market intelligence
* Data ingestion for dashboards, bots, and research

---

## Related Resources

* **Shyft Solana gRPC Documentation**
  [https://docs.shyft.to/solana-fast-grpc/grpc-docs](https://docs.shyft.to/solana-fast-grpc/grpc-docs)
* **Start Streaming with Shyft**
  [https://shyft.to/solana-yellowstone-grpc](https://shyft.to/solana-yellowstone-grpc)
* **Shyft Website**
  [https://shyft.to/](https://shyft.to/)


