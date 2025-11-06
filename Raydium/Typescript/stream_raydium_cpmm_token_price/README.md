<a id="readme-top"></a>

# Raydium CP: Real-Time Token Price Streaming from CPMM

This CP proposes a lightweight tool to stream **real-time token prices** from Raydium’s **CPMM (Constant Product Market Maker)** program on Solana using **gRPC**, without relying on RPC or external APIs.

The service listens to **swap events** from:

```
Raydium CPMM Program ID: CPMMoo8L3F4NbTegBCKVNunggL7H1ZpdTHKxQB5qKP1C
```

### 🔢 How Price Is Calculated

Price is derived from token vault balances after each swap:
To get the actual price (adjusted for decimals):

```ts
adjustedPrice =
  Number(outputVault) > 0
    ? (Number(inputVault) / 10 ** inputDecimals) /
      (Number(outputVault) / 10 ** outputDecimals)
    : undefined;
```

This provides accurate, real-time prices useful for analytics, market insights, and trading strategies like arbitrage.

---
![screenshot](assets/raydium-cp.png?raw=true "Screenshot")

## Getting Started

1. **Clone the repository:**
   ```bash
   git clone https://github.com/Shyft-to/solana-defi.git
   cd Raydium/Typescript/stream_raydium_cpmm_token_price
   ```

2. **Install Dependencies:**

    ```bash
    # For example, if using npm
    npm i
    ```

3. **Run the script:**

    ```bash
    # To run the script
    npm run start
    ```

*Note: Please open `.env` and input your env details before running the script.*

## Related Links

Shyft gRPC Docs: [https://docs.shyft.to/solana-fast-grpc/grpc-docs]
