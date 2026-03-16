<a id="readme-top"></a>
# Stream and parse Raydium AMM transactions using Rabbitstream Services

This project showcases real-time transaction streaming on Solana through Rabbitstream. It captures transactions from the Raydium AMM program and parses them using the program’s IDL. The parsed transaction data can then be used for analytics, monitoring, or other processing workflows. Rabbitstream’s primary advantage is its ability to provide highly responsive transaction updates.

```
Raydium Amm Program Id: 675kPX9MHTjS2zt1qfr1NYHuzeLXfQM9H24wFSUt1Mp8
```

![screenshot](assets/stream_parsed_raydium_txns.jpg?raw=true "How to stream data")

## Getting Started

1. **Pre-requisites:**  
    This project works with any Solana Rabbitstream. The `.env` file takes in two values: `RABBITSTREAM_URL` which is your Rabbitstream connection endpoint and `X_TOKEN` which is your access token. For Rabbitstream which do not require the access, please keep the latter field blank. 

2. **Clone the repository:**
   ```bash
   git clone https://github.com/Shyft-to/solana-defi.git
   cd Rabbitstream/Raydium/Typescript/stream_and_parse_raydium_amm_transactions
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
- Rabbitstream vs gRPC comparison: [https://github.com/Shyft-to/yellowstone-grpc-vs-rabbitstream/tree/main/PumpFun/Rust/stream-pump-fun-new-minted-tokens]
- Shyft Rabbitstream Docs: [https://docs.shyft.to/rabbitstream/rabbitstream-overview]    
- Shyft Website: [https://shyft.to/]

<p align="right">(<a href="#readme-top">back to top</a>)</p>