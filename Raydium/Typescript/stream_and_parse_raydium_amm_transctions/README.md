<a id="readme-top"></a>
# Stream and parse Raydium AMM transactions using gRPC Services

This project illustrates transaction streaming on Solana using gRPC services. Raydium transactions are streamed and then parsed based on the program IDL. These parsed transactions can then be used for analysis and processing purposes. The main advantage of gRPC is its extremely fast transaction updates.


```
Raydium Amm Program Id: 675kPX9MHTjS2zt1qfr1NYHuzeLXfQM9H24wFSUt1Mp8
```

![screenshot](assets/stream_parsed_raydium_txns.jpg?raw=true "How to stream data")

## Getting Started

1. **Pre-requisites:**  
    This project works with any Solana gRPC. The `.env` file takes in two values: `GRPC_URL` which is your gRPC connection endpoint and `X_TOKEN` which is your access token. For gRPCs which do not require the access, please keep the latter field blank. 

2. **Clone the repository:**
   ```bash
   git clone https://github.com/Shyft-to/solana-defi.git
   cd Raydium/Typescript/[GRPC]Streaming_all_Transaction
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

*Note: Please rename the `.env.sample` file to `.env` and input your env details before running the script.*

<p align="right">(<a href="#readme-top">back to top</a>)</p>

## Related Links

- Shyft gRPC Docs: [https://docs.shyft.to/solana-fast-grpc/grpc-docs]  
- Start Streaming with Shyft: [https://shyft.to/solana-yellowstone-grpc]  
- Shyft Website: [https://shyft.to/]

<p align="right">(<a href="#readme-top">back to top</a>)</p>