<a id="readme-top"></a>
# Real-Time Streaming and Parsing of Meteora DAMM v2 Account Updates on Solana via Shyft gRPC

This project uses Shyft gRPC to stream Meteora DAMM v2 account data in real-time on the Solana blockchain. Leveraging gRPC’s fast and reliable connection, it enables continuous monitoring of account activity related to liquidity pools.

The primary goal is to read and interpret Meteora DAMM v2 account data, with a focus on significant changes and updates in the pools. This provides users with valuable insights into liquidity flow, pool performance, and the overall status of the pools.
## Key highlights:

1. Utilizes gRPC streaming for high-performance, real-time data delivery.

2. Parses on-chain account structures to extract meaningful state changes.

3. Monitors liquidity pool behavior and updates for informed analytics.


![screenshot](assets/usage-screenshot.png?raw=true "Screenshot")

## Getting Started

1. **Clone the repository:**
   ```bash
   git clone https://github.com/Shyft-to/solana-defi.git
   cd Meteora/Typescript/stream_and_parse_meteora_damm_v2_accounts

2. **Install Dependencies:**
    # For example,
    ```bash 
    npm i or npm install

3. **Run the script:**
   
   ```bash
   # To run the script
   npm run start

 *Note: Please in `.env`, input your env details before running the script.*
<p align="right">(<a href="#readme-top">back to top</a>)</p>


## Related Links

_For more examples, please refer to the [Documentation](https://docs.shyft.to/solana-fast-grpc/grpc-docs)_, _or feel free to visit our [Website](https://shyft.to/)_, _to get a free API Key_.
<p align="right">(<a href="#readme-top">back to top</a>)</p>   