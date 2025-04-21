# Adding a reconnect mechanism to your gRPC connection

Any gRPC connections stronger by adding a way for them to automatically reconnect if they get disconnected. This is important because it stops your app from breaking if the internet has a small problem, and it keeps the data flowing without you doing anything. This automatic reconnecting makes your app more reliable.

This is another building block which illustrates adding a reconnection mechanism to your gRPC connection for making it more robust.

```
Program Id which we are streaming: 6EF8rrecthR5Dkzon8Nwu78hRvfCKubJ14M5uBEwF6P
```

![screenshot](assets/stream_parsed_raydium_txns.jpg?raw=true "How to run project")

## Getting Started

1. **Pre-requisites:**  
    This project works with any Solana gRPC. The `.env` file takes in two values: `GRPC_URL` which is your gRPC connection endpoint and `X_TOKEN` which is your access token. For gRPCs which do not require the access, please keep the latter field blank.   

2. **Clone the repository:**
   ```bash
   git clone https://github.com/Shyft-to/solana-defi.git
   cd add_a_reconnect_mechanism/Typescript/add_a_reconnect_mechanism
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