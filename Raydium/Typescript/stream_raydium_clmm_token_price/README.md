<a id="readme-top"></a>
# Stream Token Price Updates on Raydium CLMM

This project streams real-time token prices from Raydium’s Concentrated Liquidity Market Maker (CLMM) on Solana using gRPC, eliminating the need for external RPC or API calls.

It listens for swap events, extracts the sqrtPrice (stored in Q64.64 fixed-point format), and converts it into the actual token price by adjusting for token decimals.

By tracking these updates live, the project offers insights into market trends, token valuations, and potential arbitrage or trading opportunities

```
Raydium CLMM Program Id: CAMMCzo5YL8w4VFF8KVHrK22GGUsp5VTaW7grrKgrWqK
```

![screenshot](assets/raydium-clmm.png?raw=true "Screenshot")

## Getting Started

1. **Clone the repository:**
   ```bash
   git clone https://github.com/Shyft-to/solana-defi.git
   cd Raydium/Typescript/stream_raydium_clmm_token_price
   ```

2. **Install Dependencies:**

    ```bash
    # For example, if using npm
    npm i
    ```

3. **Run the script:**

    ```bash
    # To run the script
    npm run start
    ```

*Note: Please open `.env` and input your env details before running the script.*

## Related Links

Shyft gRPC Docs: [https://docs.shyft.to/solana-fast-grpc/grpc-docs]
