# Stream token price update on pumpfun

This project streams real-time token prices from the Pump.fun using the formula `virtualSolReserves / virtualTokenReserves`. It allows users to monitor live price movements and market capitalization. By tracking token prices prior to migration, the project delivers valuable insights into market trends and emerging investment opportunities.



screenshot

![screenshot](assets/usage-screenshot-pumpfun.png?raw=true "Screenshot")

## Getting Started

1. **Clone the repository:**
   ```bash
   git clone https://github.com/Shyft-to/solana-defi.git
   cd PumpFun/Typescript/stream_pumpfun_token_price
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