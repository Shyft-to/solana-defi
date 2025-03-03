# Stream and parse Raydium Amm account updates in real-time using gRPC services

This project streams Raydium ammInfo account updates on Solana via gRPC, then decode the accounts data based on the associated program IDL. Typically, accounts associated with a program have their owner field set to that program's address. For raydium, this can be used to monitor the constant changes or updates in their AmmInfo Pool account. 

This project uses `BorshAccountCoder` from `@project-serum/anchor` to decode account data.

```
Raydium Amm Program Id: 675kPX9MHTjS2zt1qfr1NYHuzeLXfQM9H24wFSUt1Mp8
```

![screenshot](assets/streaming_raydium_account_updates.jpg?raw=true "How to run project")

## Getting Started

1. **Pre-requisites:**  
    This project works with any Solana gRPC. The `.env` file takes in two values: `GRPC_URL` which is your gRPC connection endpoint and `X_TOKEN` which is your access token. For gRPCs which do not require the access, please keep the latter field blank.   

2. **Clone the repository:**
   ```bash
   git clone https://github.com/Shyft-to/solana-defi.git
   cd Raydium/Typescript/stream_and_parse_all_raydium_accounts
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
This will stream all the account updates from Raydium Amm and decode the updates.

*Note: Please rename the `.env.sample` file to `.env` and input your env details before running the script.*

<p align="right">(<a href="#readme-top">back to top</a>)</p>

## Related Links

- Shyft gRPC Docs: [https://docs.shyft.to/solana-fast-grpc/grpc-docs]  
- Start Streaming with Shyft: [https://shyft.to/solana-yellowstone-grpc]  
- Shyft Website: [https://shyft.to/]

<p align="right">(<a href="#readme-top">back to top</a>)</p>