# Real-time Streaming and Parsing of Raydium CP PoolInfoLayout Account Updates via gRPC

This project enables real-time streaming and parsing of Raydium CP PoolInfoLayout account updates on the Solana blockchain using gRPC services. It continuously monitors on-chain events, extracts key properties defined in the **Marshmallow schema**, and processes structured data efficiently. By seamlessly integrating with Solana’s blockchain, it provides accurate, up-to-date liquidity pool state information, ensuring reliable and fast data access for DeFi applications.

![screenshot](assets/raydium_cpAccount.png?raw=true "Screenshot")

## Getting Started

1. **Clone the repository:**
   ```bash
   git clone https://github.com/Shyft-to/solana-defi.git
   cd Raydium/Typescript/stream_and_parse_cp_accounts
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