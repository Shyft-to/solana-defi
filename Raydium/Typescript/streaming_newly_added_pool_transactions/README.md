#  Real-Time Streaming of Newly Added Pool Transactions on Raydium

This code streams transactions for newly added pools on Raydium by detecting `raydiumInitialize` and `raydiumInitialize2` events. By filtering only fresh pool creations, it provides real-time insights into new liquidity pools as they go live. Stay ahead of market movements with instant updates on new pool transactions.

1. **Clone the repository:**
   ```bash
   git clone https://github.com/Shyft-to/solana-defi.git
   
   cd -LiteralPath "Raydium/Typescript/[GRPC]RaydiumOldPool"
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