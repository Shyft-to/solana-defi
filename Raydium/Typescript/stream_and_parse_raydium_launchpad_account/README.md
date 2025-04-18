# Real-time Streaming and Parsing of Raydium Launchpad Account Updates via gRPC

This project provides real-time streaming and intelligent parsing of Raydium Launchpad account updates on the Solana blockchain using gRPC services. It listens to on-chain events, decodes key data structures defined by the Marshmallow schema, and delivers structured, actionable data in real time. Designed for DeFi platforms, this system ensures fast, accurate, and reliable access to Raydium pool states and vesting information.

## Features
- 🔄 Real-time Account Streaming: Subscribes to all Raydium Launchpad account changes using Shyft’s gRPC stream API for immediate data updates.

- 📦 Structured Account Parsing: Decodes on-chain data using Marshmallow schema definitions to extract key properties from Raydium-specific accounts like pool states, global config, and vesting records.

- ⚡ Fast and Scalable Data Access: Built on top of Solana’s Shyft gRPC, ensuring low-latency access to blockchain events—ideal for time-sensitive DeFi use cases.

- 🔍 Comprehensive Raydium IDL Integration: Fully supports the Raydium Launchpad Interface Definition Language (IDL) for seamless decoding of smart contract accounts.

- 🔐 Secure & Configurable: Simple .env-based configuration for secure authentication and customizable environment setup.

![screenshot](assets/raydium-launchpad.png?raw=true "Screenshot")

## Getting Started

1. **Clone the repository:**
   ```bash
   git clone https://github.com/Shyft-to/solana-defi.git
   cd Raydium/Typescript/stream_and_parse_raydium_launchpad_account
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

*Note: Please create a `.env` file, input your env details and run the script.*

## Related Links

Shyft gRPC Docs: [https://docs.shyft.to/solana-fast-grpc/grpc-docs]