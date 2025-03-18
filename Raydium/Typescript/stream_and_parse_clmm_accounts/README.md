# Real-time Streaming and Parsing of Raydium CLMM Poolstate Account Updates via gRPC

This project enables real-time streaming and parsing of Raydium CLMM (Concentrated Liquidity Market Maker) pool state account updates on the Solana blockchain using gRPC services. It continuously listens for updates, extracts relevant properties as defined in the Marshmallow schema, and processes the structured data efficiently. The implementation ensures seamless integration with Solana's on-chain events, providing accurate and up-to-date liquidity pool state information.

![screenshot](assets/raydiumClmm_screenshotA.png?raw=true "Screenshot")

## Getting Started

1. **Clone the repository:**
   ```bash
   git clone https://github.com/Shyft-to/solana-defi.git
   cd Raydium/Typescript/stream_and_parse_clmm_accounts
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