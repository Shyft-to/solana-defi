<a id="readme-top"></a>

# Streaming and Parsing Meteora DAMM v2 New Pools using Rabbitstream 

This project provides real-time detection, streaming, and parsing of newly created liquidity pools on Meteora’s DAMM (Dynamic Automated Market Maker) by monitoring the `initialize_pool` instruction on-chain.

It captures, decodes, and structures transaction data related to both Meteora DAMM and Solana Token Program instructions as transactions are finalized, enabling immediate and reliable insight into pool creation events.

Designed for high-throughput environments on the Solana blockchain, the system delivers low-latency, structured data suitable for DeFi analytics, protocol monitoring, and automated trading or alerting workflows.

Powered by Rabbitstream for real-time streaming, the solution is highly scalable and resilient, making it well-suited for continuous, production-grade on-chain data processing.


![screenshot](assets/usage-screenshot.png?raw=true "Screenshot")

## Getting Started

1. **Clone the repository:**
   ```bash
   git clone https://github.com/Shyft-to/solana-defi.git
   cd Rabbitstream/Meteora/Typescript/stream_and_parse_meteora_damm_v2_new_pools
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
- Rabbitstream vs gRPC comparison: [https://github.com/Shyft-to/yellowstone-grpc-vs-rabbitstream/tree/main/PumpFun/Rust/stream-pump-fun-new-minted-tokens]
- Shyft Rabbitstream Docs: [https://docs.shyft.to/rabbitstream/rabbitstream-overview]    
- Shyft Website: [https://shyft.to/]