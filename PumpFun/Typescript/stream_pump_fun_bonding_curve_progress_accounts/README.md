# Stream and parse Pump.fun account updates in Real-time

This project streams PumpFun account updates on Solana in real-time. Once the updates are received, these updates are parsed using their corresponding IDL. Typically, accounts associated with a program have their owner field set to that program's address. For pump.fun, this can be used in monitoring various activities such as Token migration to Raydium, once the bonding curve is complete.  

This project uses `BorshAccountCoder` from `@project-serum/anchor` to decode account data.

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
   cd PumpFun/Typescript/stream_and_parse_all_pump_fun_accounts
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