<a id="readme-top"></a>

# Raydium Launchpad: Real-Time Token Price Streaming from Launchpad

This Raydium Launchpad tool provides a lightweight and efficient way to stream real-time token prices directly from the Raydium Launchpad Program on Solana using gRPC, without relying on RPC nodes or third-party APIs.

The service listens to swap and calculates token price emitted by:
Raydium Launchpad Program ID: CPMMoo8L3F4NbTegBCKVNunggL7H1ZpdTHKxQB5qKP1C

## 🔢 How Price Is Calculated

Each token price is derived from vault reserves updated after every swap event.
The adjusted price is calculated as follows:

```ts
function calculateRaydiumLaunchPadPrice(
  virtualBaseReserves,
  virtualQuoteReserve,
  quoteDecimals,
  baseDecimals
) {
  const quoteAmount = virtualQuoteReserve / 10 ** quoteDecimals;
  const baseAmount = virtualBaseReserves / 10 ** baseDecimals;
  if (baseAmount === 0 || quoteAmount === 0) return NaN; 
  return quoteAmount / baseAmount;
}
```
This provides accurate real-time prices directly from on-chain liquidity data, enabling market analytics, price tracking, and arbitrage detection.

```
Raydium Launchpad Program Id: LanMV9sAd7wArD4vJFi2qDdfnVhFxYSUg6eADduJ3uj
```

![screenshot](assets/rl-screenshot.png?raw=true "How to stream data")

## Getting Started

1. **Pre-requisites:**  
    This project works with any Solana gRPC. The `.env` file takes in two values: `GRPC_URL` which is your gRPC connection endpoint and `X_TOKEN` which is your access token. For gRPCs which do not require the access, please keep the latter field blank. 

2. **Clone the repository:**
   ```bash
   git clone https://github.com/Shyft-to/solana-defi.git
   cd Raydium/Typescript/stream_raydium_launchpad_and_detect_buy_sell_events
   ```

3. **Install Dependencies:**

    ```bash
    # For example, if using npm
    npm i
    ```

4. **Run the script:**

    ```bash
    # To run the script
    npm run start
    ```

*Note: Please name a `.env` file and input your env details before running the script.*

<p align="right">(<a href="#readme-top">back to top</a>)</p>

## Related Links

- Shyft gRPC Docs: [https://docs.shyft.to/solana-fast-grpc/grpc-docs]  
- Start Streaming with Shyft: [https://shyft.to/solana-yellowstone-grpc]  
- Shyft Website: [https://shyft.to/]

<p align="right">(<a href="#readme-top">back to top</a>)</p>