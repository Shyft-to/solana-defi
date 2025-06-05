# Stream token price update on pumpfun amm

This project streams real-time token prices from the Pump Swap AMM using the formula `pool_sol_reserve / pool_token_reserve`. It enables users to track price movements and market capitalization in real time. By monitoring token prices before migration, the project provides valuable insights into market trends and potential investment opportunities.

![screenshot](assets/amm-usage-screenshot.png?raw=true "Screenshot")

## Getting Started

1. **Clone the repository:**
   ```bash
   git clone https://github.com/Shyft-to/solana-defi.git
   cd PumpFun/Typescript/stream_pump_swap_amm_token_price
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

*Note: Please rename the `.env.sample` file to `.env` and input your env details before running the script.*

## Related Links

Shyft gRPC Docs: [https://docs.shyft.to/solana-fast-grpc/grpc-docs]
