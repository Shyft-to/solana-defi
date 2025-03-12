#  Stream Pump.fun Token Transactions & Fetch Bonding Pool Liquidity in Real Time

This code enables real-time streaming of Pump.fun token transactions while simultaneously fetching liquidity details. As a token progresses through its bonding curve, the code retrieves its current liquidity pool details and distinguishes whether each transaction is a buy or a sell. With its unique transaction parsing method, you gain valuable insights into token movements and market activity—all in real time.

1. **Clone the repository:**
   ```bash
   git clone https://github.com/Shyft-to/solana-defi.git
   
   cd PumpFun/Typescript/pumpfun_liquidityDetails
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
Shyft Website: [https://shyft.to/]