<a id="readme-top"></a>

# Streaming Meteora DBC Buy Sell transaction with Rabbitstream.

This project delivers real-time streaming and decoding of Meteora Dynamic Bonding Curve (DBC) transactions on the Solana blockchain using Rabbitstream. It continuously tracks live buy and sell events from both the Meteora DBC and the Solana Token Program to provide detailed visibility into liquidity movements and market activity.

By interpreting dynamic bonding curve mechanics—where prices adjust in response to demand and trade volume—the system enables transparent analysis of pricing behavior and trade execution efficiency.

All parsed transaction data is cleanly structured for seamless downstream processing and analytics. This makes the project well-suited for developers, traders, and analysts seeking real-time insights into DeFi activity on Solana, including user behavior, liquidity flows, and trading dynamics across the Meteora ecosystem.

![screenshot](assets/meteora-dbc.png?raw=true "Screenshot")

## Getting Started

1. **Clone the repository:**
   ```bash
   git clone https://github.com/Shyft-to/solana-defi.git
   cd Rabbitstream/Meteora/Typescript/stream_meteora_dbc_and_detect_buy_sell_events
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