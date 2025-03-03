# Stream and parse all Whirlpool account updates in real-time using gRPC

This project streams Orca Whirlpool account updates on Solana using gRPC services in real-time. Once the updates are received, these updates are parsed using their corresponding IDL. For Whirlpool, this can be used in monitoring various activities such as swaps, reading pool states, and other DeFi activities.  

This project uses `BorshAccountCoder` from `@project-serum/anchor` to decode account data. Typically, accounts associated with a program have their owner field set to that program's address.

```
Whirlpool Program Id: whirLbMiicVdio4qvUfM5KAg6Ct8VwpYzGff3uctyCc
```

![screenshot](assets/stream_whirlpool_accounts.jpg?raw=true "How to run project")

## Getting Started

1. **Pre-requisites:**  
    This project works with any Solana gRPC. The `.env` file takes in two values: `GRPC_URL` which is your gRPC connection endpoint and `X_TOKEN` which is your access token. For gRPCs which do not require the access, please keep the latter field blank.   

2. **Clone the repository:**
   ```bash
   git clone https://github.com/Shyft-to/solana-defi.git
   cd Orca/Typescript/stream_and_parse_all_whirlpool_accounts
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