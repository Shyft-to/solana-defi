# Stream and Parse Pool Account Updates on Pumpfun AMM

This project aims to stream and process account updates for Pumpfun's Automated Market Maker (AMM),
providing real-time data analysis and insights.
By utilizing gRPC, it facilitates the continuous flow of account updates,
enabling effective monitoring of pool activities.
The core objective is to parse and analyze the Pumpfun AMM liquidity pools,
focusing on identifying important account updates and changes, 
thereby offering valuable information about the pool’s status


![screenshot](assets/PumpAmm.png?raw=true "Screenshot")


## Getting Started

1. **Clone the repository:**
   ```bash
   git clone https://github.com/Shyft-to/solana-defi.git
   
   cd PumpFun/Typescript/grpc-stream-and-parse-pump-swap-amm-account
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
*Note: On `.env` please input your env details before running the script.*

## Related Links

Shyft gRPC Docs: [https://docs.shyft.to/solana-fast-grpc/grpc-docs]