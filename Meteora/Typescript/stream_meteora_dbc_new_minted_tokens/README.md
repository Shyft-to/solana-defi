<a id="readme-top"></a>

# Streaming New Minted tokens on Meteora DBC transaction using gRPC and parsing instructions in Typescript

This project streams Newly Minted Token on Meteora Dynamic Bonding Curve (DBC), 
decoding and parsing on-chain instructions in real time with TypeScript.
It extracts and interprets both Meteora DBC and Token Program instructions,
surfacing newly minted token events and dynamic curve adjustments as they happen.
Can also work for Jup Studio Launchpad, 
this system decodes DBC logic—where transactions shifts algorithmically 
with user activity—enabling transparent and automated execution.
All parsed data is normalized into structured outputs for downstream analysis, 
supporting real-time dashboards, on-chain metrics, 
and liquidity insights across the Solana ecosystem.

![screenshot](assets/meteora-dbc.png?raw=true "Screenshot")

## Getting Started

1. **Clone the repository:**
   ```bash
   git clone https://github.com/Shyft-to/solana-defi.git
   cd Meteora/Typescript/stream_meteora_dbc_new_minted_tokens
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