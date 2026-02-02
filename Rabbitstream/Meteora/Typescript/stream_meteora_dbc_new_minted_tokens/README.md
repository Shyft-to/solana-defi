<a id="readme-top"></a>

# Streaming New Minted tokens on Meteora DBC transaction using Rabbitstream and parsing instructions in Typescript

This project provides real-time streaming and decoding of newly minted tokens on the Meteora Dynamic Bonding Curve (DBC) with Rabbitstream. It continuously listens to on-chain transactions, decoding and parsing newly minted tokens on Meteora DBC as they occur.

The system identifies newly minted token events and tracks dynamic bonding curve adjustments in real time. It is also compatible with Jupiter Studio Launchpad, where it decodes DBC-based logic that algorithmically shifts transaction behavior in response to user activity, enabling transparent and automated execution.

All parsed on-chain data is normalized into structured outputs, making it suitable for downstream processing such as real-time dashboards, on-chain analytics, liquidity monitoring, and market insight generation across the Solana ecosystem.

![screenshot](assets/meteora-dbc.png?raw=true "Screenshot")

## Getting Started

1. **Clone the repository:**
   ```bash
   git clone https://github.com/Shyft-to/solana-defi.git
   cd Rabbitstream/Meteora/Typescript/stream_meteora_dbc_new_minted_tokens
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