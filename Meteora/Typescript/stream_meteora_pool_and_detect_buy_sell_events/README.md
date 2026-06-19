<a id="readme-top"></a>

# Stream Meteora Pool Transactions and Detect Buy/Sell Events using gRPC

This project enables real-time streaming and parsing of Meteora pool swap transactions using gRPC. It decodes transaction data on-the-fly, identifies whether each swap is a **Buy** or **Sell** by comparing pre and post token balances, and extracts key trade details such as `tokenIn`, `tokenOut`, `amountIn`, and `amountOut` from the on-chain `TradeEvent`. Built for Solana's DeFi ecosystem, this is useful for real-time trade monitoring, analytics dashboards, and bot integrations.

![screenshot](assets/meteora_pool_screenshot.png?raw=true "Screenshot")

## Getting Started

1. **Clone the repository:**
```bash
   git clone https://github.com/Shyft-to/solana-defi.git
   cd Meteora/Typescript/stream_meteora_pool_and_detect_buy_sell_events
```

2. **Install Dependencies:**
```bash
   npm i
```

3. **Configure environment variables:**

   Create a `.env` file in the root and fill in your details:
```env
   GRPC_URL=your_grpc_url
   X_TOKEN=your_api_key
```

4. **Run the script:**
```bash
   npm run start
```

## Related Links

- Shyft gRPC Docs: [https://docs.shyft.to/solana-fast-grpc/grpc-docs](https://docs.shyft.to/solana-fast-grpc/grpc-docs)
- Meteora: [https://meteora.ag](https://meteora.ag)
- Shyft: [https://shyft.to](https://shyft.to)

<p align="right">(<a href="#readme-top">back to top</a>)</p>