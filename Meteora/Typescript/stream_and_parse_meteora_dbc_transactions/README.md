<a id="readme-top"></a>

# Streaming Meteora DBC transaction using gRPC and parsing instructions in Typescript

This project streams Meteora Dynamic Bonding Curve (DBC) transactions via gRPC, 
parsing both Meteora DBC and Token Program instructions in real time.

It powers transactions on Jup Studio Launchpad by decoding 
dynamic bonding curve mechanics—where activity adjusts
dynamically with demand—enabling transparent, efficient execution.

Parsed data is structured for seamless processing and analysis,
unlocking real-time insights into Solana-based transaction flows.

![screenshot](assets/meteora-dbc.png?raw=true "Screenshot")

## Getting Started

1. **Clone the repository:**
   ```bash
   git clone https://github.com/Shyft-to/solana-defi.git
   cd Meteora/Typescript/stream_and_parse_meteora_dbc_transactions
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
