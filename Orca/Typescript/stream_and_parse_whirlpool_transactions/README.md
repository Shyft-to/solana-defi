<a id="readme-top"></a>
# Stream Orca Whirlpool transactions and parse using program IDL

Using gRPC, this project streams Orca whirlpool transactions on Solana. These transactions are then parsed based on the program IDL, and this decoded information can be used in various DeFi applications such as Trading Bots, Dex Aggregators etc.

```
Whirlpool Program Id: whirLbMiicVdio4qvUfM5KAg6Ct8VwpYzGff3uctyCc
```

![screenshot](assets/stream_parsed_raydium_txns.jpg?raw=true "How to stream data")

## Getting Started

1. **Pre-requisites:**  
    This project works with any Solana gRPC. The `.env` file takes in two values: `GRPC_URL` which is your gRPC connection endpoint and `X_TOKEN` which is your access token. For gRPCs which do not require the access, please keep the latter field blank. 

2. **Clone the repository:**
   ```bash
   git clone https://github.com/Shyft-to/solana-defi.git
   cd Orca/Typescript/stream_and_parse_whirlpool_transactions
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

*Note: Please rename the `.env.sample` file to `.env` and input your env details before running the script.*

<p align="right">(<a href="#readme-top">back to top</a>)</p>

## Related Links

- Shyft gRPC Docs: [https://docs.shyft.to/solana-fast-grpc/grpc-docs]  
- Start Streaming with Shyft: [https://shyft.to/solana-yellowstone-grpc]  
- Shyft Website: [https://shyft.to/]

<p align="right">(<a href="#readme-top">back to top</a>)</p>