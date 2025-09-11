<a id="readme-top"></a>

# Streaming Meteora DBC Buy Sell transaction with gRPC.

This project provides real-time streaming and decoding of Meteora Dynamic Bonding Curve (DBC) transactions on the Solana blockchain via gRPC. By tracking live buy and sell events from both the Meteora DBC and Token Program, it offers deep insights into liquidity movements and market behavior.

 The system interprets dynamic bonding curve mechanics—where pricing adjusts based on demand and volume—enabling transparent, efficient trade execution.

Parsed data is structured for seamless downstream processing and analysis, making it ideal for developers, traders, and analysts looking to understand real-time DeFi activity on Solana. Unlock actionable insights into user behavior, liquidity flow, and trading dynamics across the Meteora ecosystem.


![screenshot](assets/meteora-dbc.png?raw=true "Screenshot")

## Getting Started

1. **Clone the repository:**
   ```bash
   git clone https://github.com/Shyft-to/solana-defi.git
   cd Meteora/Typescript/stream_meteora_dbc_and_detect_buy_sell_events
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
