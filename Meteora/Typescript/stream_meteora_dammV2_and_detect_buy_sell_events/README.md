<a id="readme-top"></a>

# Streaming Meteora dammV2 Transactions and Detecting Buy and Sell Events

This project streams and parses Meteora dammV2 transactions on the Solana blockchain using gRPC. By tracking buy and sell events in real-time, it provides valuable insights into on-chain market activity, enabling users to better understand market trends, trading behavior, and liquidity flow within the Meteora ecosystem.
The system decodes and interprets both Meteora dammV2-specific and Solana Token Program instructions, extracting actionable data and formatting it into a clean, serializable structure for seamless downstream processing and analysis. Built using TypeScript, the implementation takes advantage of its flexibility, type safety, and ecosystem support to handle streaming data efficiently and reliably in real time.

![screenshot](assets/usage-screenshot.png?raw=true "Screenshot")

## Getting Started

1. **Clone the repository:**
   ```bash
   git clone https://github.com/Shyft-to/solana-defi.git
   cd Meteora/Typescript/stream_meteora_dammV2_and_detect_buy_sell_events
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