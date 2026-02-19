<a id="readme-top"></a>

# Stream and Parse Raydium AMM Token Prices Using gRPC Services in Rust

This project demonstrates **real-time token price streaming** from Raydium AMM on Solana using **gRPC services** in Rust. Transactions from the Raydium AMM program are streamed, parsed, and processed to extract **live token price updates**. The advantage of gRPC is **low-latency streaming**, which ensures that price updates happen almost instantly.

```
Raydium AMM Program ID: 675kPX9MHTjS2zt1qfr1NYHuzeLXfQM9H24wFSUt1Mp8
```

![screenshot](assets/usage-screenshot.png?raw=true "Streaming parsed Raydium token prices in Rust")

## Features

- **Real-time token price streaming** using Yellowstone gRPC
- **Extracts live swap prices** from Raydium AMM transactions
- **Calculates pool prices** from reserve ratios
- **Tracks price impact** for each swap
- **Monitors multiple token pairs** automatically
- **Formats price output** for easy readability
- **Built with Rust** for high-performance, low-latency price feeds

---

## Getting Started

### 1. Prerequisites

- **Rust** (install via [rustup](https://rustup.rs/))
- A **Solana gRPC endpoint** with Yellowstone support (e.g., from [Shyft](https://shyft.to/))

### 2. Clone the Repository

```bash
git clone https://github.com/Shyft-to/solana-defi.git
cd Raydium/Rust/stream_raydium_amm_token_price
```


### 3. Run the Price Stream

```bash
cargo run --endpoint <YOUR_GRPC_ENDPOINT> --x-token <YOUR_X_TOKEN>
```

## How Price Extraction Works

1. **gRPC Connection**: Establishes a persistent connection to a Solana gRPC endpoint
2. **Transaction Streaming**: Subscribes to real-time transactions from the Raydium AMM program
3. **Swap Detection**: Identifies swap instructions (SwapBaseIn, SwapBaseOut, SwapBaseIn2, SwapBaseOut2)
4. **Event Parsing**: Extracts swap events containing pool state and amounts
5. **Balance Analysis**: Examines pre and post token balances to identify token types
6. **Price Calculation**:
   - **Swap Price** = amount_out / amount_in (or inverse based on direction)
   - **Pool Price** = pool_quote_reserves / pool_base_reserves
7. **Price Formatting**: Converts scientific notation to readable decimal format
8. **Real-time Output**: Displays price updates as they happen

---

## Use Cases

- **Live Price Feeds**: Build your own real-time price oracle for DeFi applications
- **Arbitrage Monitoring**: Detect price discrepancies across different DEXes instantly
- **Trading Bots**: Create automated trading strategies based on real-time price movements
- **Market Analysis**: Track price trends and liquidity changes in real-time
- **Research**: Study price impact and market depth for various token pairs
- **Portfolio Tracking**: Monitor the value of your holdings in real-time

---

## Why Rust for Price Streaming?

- **Performance**: Rust's zero-cost abstractions ensure minimal latency
- **Memory Safety**: No garbage collection pauses means consistent performance
- **Type Safety**: Catch errors at compile time, not in production
- **Concurrency**: Efficiently handle multiple price streams simultaneously
- **Low Resource Usage**: Run price streams on small instances cost-effectively

---

## Related Links

- **Shyft gRPC Docs:** [https://docs.shyft.to/solana-fast-grpc/grpc-docs](https://docs.shyft.to/solana-fast-grpc/grpc-docs)
- **Get a gRPC Endpoint:** [https://shyft.to/solana-yellowstone-grpc](https://shyft.to/solana-yellowstone-grpc)
- **Raydium Protocol Docs:** [https://docs.raydium.io/](https://docs.raydium.io/)
- **Yellowstone gRPC Examples:** [https://github.com/rpcpool/yellowstone-grpc/tree/master/examples/rust](https://github.com/rpcpool/yellowstone-grpc/tree/master/examples/rust)
- **Solana Price Feeds:** [https://shyft.to/](https://shyft.to/)


<p align="right">(<a href="#readme-top">back to top</a>)</p>