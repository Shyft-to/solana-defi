<a id="readme-top"></a>
# Real-Time Raydium Launchpad Transaction Streaming and Buy/Sell Event Detection on Solana

This project provides real-time streaming and advanced analysis of Raydium Launchpad transactions on the Solana blockchain. By continuously monitoring and parsing blockchain data, it instantly identifies and categorizes buy and sell events as they occur.

Designed to deliver actionable insights into market activity, this tool helps users track trading behavior, analyze market trends, and gain a deeper understanding of the Raydium Launchpad ecosystem.

Whether you are a developer, trader, or analyst, this solution empowers you with real-time data to make more informed decisions within the Solana DeFi space. With this tool, you can easily track market trends and better understand user behavior within the Raydium Launchpad ecosystem.

## Key Features:
- Real-Time Streaming of Raydium Launchpad transactions
- Instant Buy and Sell Event Identification
- In-Depth Market Analysis and Behavior Tracking
- Designed for DeFi Developers, Traders, and Analysts
- Unlock Actionable Insights into Raydium Ecosystem Dynamics

```
Raydium Launchpad Program Id: LanMV9sAd7wArD4vJFi2qDdfnVhFxYSUg6eADduJ3uj
```

![screenshot](assets/rl-screenshot.png?raw=true "How to stream data")

## Getting Started

1. **Pre-requisites:**  
    This project works with any Solana gRPC. The `.env` file takes in two values: `GRPC_URL` which is your gRPC connection endpoint and `X_TOKEN` which is your access token. For gRPCs which do not require the access, please keep the latter field blank. 

2. **Clone the repository:**
   ```bash
   git clone https://github.com/Shyft-to/solana-defi.git
   cd Raydium/Typescript/stream_raydium_launchpad_and_detect_buy_sell_events
   ```

3. **Install Dependencies:**

    ```bash
    # For example, if using npm
    npm i
    ```

4. **Run the script:**

    ```bash
    # To run the script
    npm run start
    ```

*Note: Please name a `.env` file and input your env details before running the script.*

<p align="right">(<a href="#readme-top">back to top</a>)</p>

## Related Links

- Shyft gRPC Docs: [https://docs.shyft.to/solana-fast-grpc/grpc-docs]  
- Start Streaming with Shyft: [https://shyft.to/solana-yellowstone-grpc]  
- Shyft Website: [https://shyft.to/]

<p align="right">(<a href="#readme-top">back to top</a>)</p>