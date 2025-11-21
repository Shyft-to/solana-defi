# Stream and parse Pump.fun Transactions in Real-time with Rabbitstream

This project streams PumpFun transactions from the Solana blockchain via Rabbitstream and parses them according to the updated program IDL. This real-time streaming provides valuable insights into market activity, enabling users to track various DeFi events such as swaps, buy and sell and understand market trends and user behavior within the PumpFun ecosystem.

```
Pump.fun Program Id: 6EF8rrecthR5Dkzon8Nwu78hRvfCKubJ14M5uBEwF6P
```

![screenshot](assets/usage_screenshot.png?raw=true "How to run project")

## Getting Started

1. **Pre-requisites:**  
    This project works with any Solana Rabbitstream. The `.env` file takes in two values: `RABBITSTREAM_URL` which is your Rabbitstream connection endpoint and `X_TOKEN` which is your access token. For Rabbitstream which do not require the access, please keep the latter field blank.   

2. **Clone the repository:**
   ```bash
   git clone https://github.com/Shyft-to/solana-defi.git
   cd Rabbitstream/PumpFun/Typescript/stream_and_parse_pump_fun_transactions
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
- Rabbitstream vs gRPC comparison: [https://github.com/Shyft-to/yellowstone-grpc-vs-rabbitstream/tree/main/PumpFun/Rust/stream-pump-fun-new-minted-tokens]
- Shyft Rabbitstream Docs: [https://docs.shyft.to/rabbitstream/rabbitstream-overview]    
- Shyft Website: [https://shyft.to/]

<p align="right">(<a href="#readme-top">back to top</a>)</p>