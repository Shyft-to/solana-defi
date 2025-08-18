<a id="readme-top"></a>

# Streaming Meteora DAMM v2 transaction using gRPC and parsing instructions in Typescript

This project streams Meteora Damm v2 transactions via gRPC, parsing both Meteora Damm and Token Program instructions in real time. It efficiently decodes transaction data, extracts valuable insights, and formats parsed instructions into a serializable structure for seamless processing and analysis. Built for Solana’s ecosystem, the implementation leverages Rust’s strong type safety and performance to handle high-throughput transaction streams effectively.

![screenshot](assets/meteora_damm_screenshot.png?raw=true "Screenshot")

## Getting Started

1. **Clone the repository:**
   ```bash
   git clone https://github.com/Shyft-to/solana-defi.git
   cd PumpFun/Typescript/stream_and_parse_meteora_damm_v2_transactions
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