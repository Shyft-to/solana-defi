#  Real-Time Streaming of Active Bonding Curve Account Updates on Pump.fun

This code streams all ongoing bonding curve account updates on Pump.fun with defi api created by Shyft for Pumpfun Amm `https://defi.shyft.to/v0/pools/get_by_pair?tokenA=$&tokenB=$`. It provides real-time access to finalized and already trading pumpfun tokens on pumpfun AMM, allowing you to track token progress after migration. Stay updated with the latest bonding curve movements and monitor liquidity flow with precision

## Getting Started

1. **Clone the repository:**
   ```bash
   git clone https://github.com/Shyft-to/solana-defi.git
   
   cd PumpFun/Typescript/stream_completed_bonding_curve_with_defiAPI
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
*Note: On `.env` please input your env details before running the script.*

## Related Links

Shyft gRPC Docs: [https://docs.shyft.to/solana-fast-grpc/grpc-docs]