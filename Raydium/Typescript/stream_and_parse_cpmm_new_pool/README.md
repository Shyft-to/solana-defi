<a id="readme-top"></a>
# Streaming Raydium CPMM New Pool using gRPC and parsing transactions

This project enables real-time streaming and parsing of newly created pools on Raydium's CPMM (Constant Product Market Maker).
It efficiently captures, decodes, and structures transaction data from these new pools as they're added.
With real-time processing and optimized performance, it delivers actionable insights while maintaining scalability 
and responsiveness.
```
CPMM Program Id: CPMMoo8L3F4NbTegBCKVNunggL7H1ZpdTHKxQB5qKP1C
```

![screenshot](assets/cpmm-new-pool.png?raw=true "cpmm new pool")

## Getting Started

1. **Pre-requisites:**  
    This project works with any Solana gRPC. The `.env` file takes in two values: `GRPC_URL` which is your gRPC connection endpoint and `X_TOKEN` which is your access token. For gRPCs which do not require the access, please keep the latter field blank. 

2. **Clone the repository:**
   ```bash
   git clone https://github.com/Shyft-to/solana-defi.git
   cd Raydium/Typescript/stream_and_parse_cpmm_new_pool
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

*Note: Please on `.env`  input your env details before running the script.*

<p align="right">(<a href="#readme-top">back to top</a>)</p>

## Related Links

- Shyft gRPC Docs: [https://docs.shyft.to/solana-fast-grpc/grpc-docs]  
- Start Streaming with Shyft: [https://shyft.to/solana-yellowstone-grpc]  
- Shyft Website: [https://shyft.to/]

<p align="right">(<a href="#readme-top">back to top</a>)</p>