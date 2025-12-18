<a id="readme-top"></a>

# Streaming Meteora DBC transaction using Rabbitstream and parsing instructions in Typescript

This project streams Meteora Dynamic Bonding Curve (DBC) transactions in real time via Rabbitstream, parsing both Meteora DBC and Solana Token Program instructions as they occur.

It powers transactions on Jup Studio Launchpad by decoding dynamic bonding curve mechanics—where pricing and liquidity adjust in response to demand—enabling transparent, efficient, and reliable execution.

All parsed transaction data is normalized and structured for seamless downstream processing and analysis, unlocking real-time insights into Solana-based transaction flows and on-chain activity.

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
- Rabbitstream vs gRPC comparison: [https://github.com/Shyft-to/yellowstone-grpc-vs-rabbitstream/tree/main/PumpFun/Rust/stream-pump-fun-new-minted-tokens]
- Shyft Rabbitstream Docs: [https://docs.shyft.to/rabbitstream/rabbitstream-overview]    
- Shyft Website: [https://shyft.to/]