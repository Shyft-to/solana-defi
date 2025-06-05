<a id="readme-top"></a>

# Streaming and Parsing Meteora DAMM New Pools using gRPC

This project enables real-time streaming and parsing of newly created pools on Meteora’s DAMM (Dynamic Automated Market Maker) by detecting the `initialize_pool` instruction.
It efficiently captures, decodes, and structures transaction data related to Meteora DAMM and Solana Token Program instructions as they occur on-chain.

Optimized for high-throughput environments on the Solana blockchain, this solution delivers low-latency, structured insights ideal for DeFi analytics, protocol monitoring, and automated workflows.

Built in Rust for safety and performance, and using gRPC streaming, the implementation is highly scalable and reliable for processing Solana transaction data in real time.


![screenshot](assets/usage-screenshot.png?raw=true "Screenshot")

## Getting Started

1. **Clone the repository:**
   ```bash
   git clone https://github.com/Shyft-to/solana-defi.git
   cd Meteora/Typescript/stream_and_parse_meteora_damm_new_pools
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