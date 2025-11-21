# Stream and parse Pump.fun Transactions in Real-time

This project streams PumpFun transactions from the Solana blockchain via gRPC and parses them according to the updated program IDL. This real-time streaming provides valuable insights into market activity, enabling users to track various DeFi events such as swaps, buy and sell and understand market trends and user behavior within the PumpFun ecosystem.

```
Pump.fun Program Id: 6EF8rrecthR5Dkzon8Nwu78hRvfCKubJ14M5uBEwF6P
```

![screenshot](assets/usage_screenshot.png?raw=true "How to run project")

## Getting Started

1. **Pre-requisites:**  
    This project works with any Solana gRPC. The `.env` file takes in two values: `GRPC_URL` which is your gRPC connection endpoint and `X_TOKEN` which is your access token. For gRPCs which do not require the access, please keep the latter field blank.   

2. **Clone the repository:**
   ```bash
   git clone https://github.com/Shyft-to/solana-defi.git
   cd PumpFun/Typescript/stream_and_parse_pump_fun_transactions
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

    # or you can also use
    npx ts-node index.ts
    ```
This will stream all the transactions from Pump.fun and parse the Pump.fun and Token Instructions.

*Note: Please rename the `.env.sample` file to `.env` and input your env details before running the script.*

<p align="right">(<a href="#readme-top">back to top</a>)</p>

## Related Links

- Shyft gRPC Docs: [https://docs.shyft.to/solana-fast-grpc/grpc-docs]  
- Start Streaming with Shyft: [https://shyft.to/solana-yellowstone-grpc]  
- Shyft Website: [https://shyft.to/]

<p align="right">(<a href="#readme-top">back to top</a>)</p>