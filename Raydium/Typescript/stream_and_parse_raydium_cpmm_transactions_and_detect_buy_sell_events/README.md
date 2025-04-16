<a id="readme-top"></a>
# Real-Time Raydium CPMM Transaction Streaming and Buy/Sell Event Detection on Solana

This project provides real-time streaming and analysis of Raydium CPMM (Constant Product Market Maker) transactions on the Solana blockchain.
By continuously monitoring and parsing blockchain data, it identifies and categorizes buy and sell events as they happen.
Designed to deliver actionable insights into market activity, this tool helps users track trading behavior, analyze trends, 
and gain a deeper understanding of Raydium CPMM ecosystem dynamics. Whether you're a developer, trader, or analyst, 
this project empowers you with real-time data to make informed decisions in the Solana DeFi space. 
This enables users to better understand market trends and user behavior within the Raydium CPMM ecosystem.
```
CPMM Program Id: CPMMoo8L3F4NbTegBCKVNunggL7H1ZpdTHKxQB5qKP1C
```

![screenshot](assets/cpmm-screenshot.png?raw=true "How to stream data")

## Getting Started

1. **Pre-requisites:**  
    This project works with any Solana gRPC. The `.env` file takes in two values: `GRPC_URL` which is your gRPC connection endpoint and `X_TOKEN` which is your access token. For gRPCs which do not require the access, please keep the latter field blank. 

2. **Clone the repository:**
   ```bash
   git clone https://github.com/Shyft-to/solana-defi.git
   cd Raydium/Typescript/stream_and_parse_raydium_cpmm_transactions_and_detect_buy_sell_events
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