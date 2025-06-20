# Stream and parse Pump.fun bonding curve in Real-time

This project streams Pump.fun account data on Solana in real-time via gRPC. Specifically, it focuses on parsing and tracking the bonding curve accounts. As updates are received, relevant data such as balance and metadata tied to the bonding curve are extracted and decoded using `BorshAccountCoder` from `@project-serum/anchor`.

By analyzing these updates, the project calculates the bonding curve progress in real-time—providing insights into how far a token has advanced through the Pump.fun lifecycle. This is especially useful for detecting when a token is ready for actions like migration to Raydium or other post-curve events.

```
Pump.fun Program Id: 6EF8rrecthR5Dkzon8Nwu78hRvfCKubJ14M5uBEwF6P
```

![screenshot](assets/streaming_pump_fun_accounts_parsed.jpg?raw=true "How to run project")

## Getting Started

1. **Pre-requisites:**  
    This project works with any Solana gRPC. The `.env` file takes in two values: `GRPC_URL` which is your gRPC connection endpoint and `X_TOKEN` which is your access token. For gRPCs which do not require the access, please keep the latter field blank.   

2. **Clone the repository:**
   ```bash
   git clone https://github.com/Shyft-to/solana-defi.git
   cd PumpFun/Typescript/stream_pump_fun_bonding_curve_progress_accounts
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
This will stream all the account updates from Pump.fun and decode the updates.

*Note: Please rename the `.env.sample` file to `.env` and input your env details before running the script.*

<p align="right">(<a href="#readme-top">back to top</a>)</p>

## Related Links

- Shyft gRPC Docs: [https://docs.shyft.to/solana-fast-grpc/grpc-docs]  
- Start Streaming with Shyft: [https://shyft.to/solana-yellowstone-grpc]  
- Shyft Website: [https://shyft.to/]

<p align="right">(<a href="#readme-top">back to top</a>)</p>