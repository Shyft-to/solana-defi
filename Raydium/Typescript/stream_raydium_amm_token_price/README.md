<a id="readme-top"></a>

# Stream and Parse Raydium AMM Price Updates Using gRPC Services

This project demonstrates **real-time transaction streaming** on Solana using **gRPC services**. Transactions from the Raydium AMM program are streamed, parsed, and processed to extract token price information. The advantage of gRPC is **low-latency streaming**, which ensures that updates happen almost instantly.

```
Raydium AMM Program ID: 675kPX9MHTjS2zt1qfr1NYHuzeLXfQM9H24wFSUt1Mp8
```

![screenshot](assets/screenshot-usage.png?raw=true "Streaming parsed Raydium price updates")

---

## Getting Started

### 1. Pre-requisites

This project works with **any Solana gRPC endpoint**. You need a `.env` file with the following values:

* `GRPC_URL` — Your gRPC connection endpoint
* `X_TOKEN` — Your access token (leave blank if not required)

### 2. Clone the Repository

```bash
git clone https://github.com/Shyft-to/solana-defi.git
cd Raydium/Typescript/stream_raydium_amm_token_price
```

### 3. Install Dependencies

```bash
npm install
```

### 4. Run the Script

```bash
npm run start
```

> **Note:** Rename `.env.sample` to `.env` and add your configuration before running.

---

The output shows:

* **Base and quote tokens** involved in the swap
* **Input and output amounts** (both raw and formatted)
* **Pool reserves** and **price per token**
* **Swap type and direction**
* **Computed pool price**

You can use this data for **price monitoring**, **arbitrage detection**, or **DeFi analytics**.

---

## Related Links

* **Shyft gRPC Docs:** [https://docs.shyft.to/solana-fast-grpc/grpc-docs](https://docs.shyft.to/solana-fast-grpc/grpc-docs)
* **Start Streaming with Shyft:** [https://shyft.to/solana-yellowstone-grpc](https://shyft.to/solana-yellowstone-grpc)
* **Shyft Website:** [https://shyft.to/](https://shyft.to/)

<p align="right">(<a href="#readme-top">back to top</a>)</p>

