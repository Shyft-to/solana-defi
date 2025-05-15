# Real-Time Token Migration Alerts: Pump.fun to Pumpfun Amm

Stay ahead of the curve with this high-performance, real-time tracking system that instantly alerts you when a token migrates from Pump.fun to Pumpfun Amm. As soon as a token completes its bonding phase on Pump.fun, the system scans transaction data for the key "migrate" instruction — a reliable indicator of migration. Once detected, it enriches the data for transparency and immediately triggers an alert.

## How the Code Detects Token Migrations (Step-by-Step):
1. Scans Each Parsed Transaction: The code monitors real-time Solana transactions and inspects the parsed instructions of each transaction.

2. Looks for the "migrate" Instruction: It specifically searches for a transaction instruction named "migrate", which signals that a token is moving from Pump.fun to Pumpfun Amm.

3. Filters Only Migration Events: If no "migrate" instruction is found, the transaction is ignored. Only migrations are processed further.

4. Enriches Transaction Data: For detected migrations, it enhances the transaction object by attaching related metadata and inner instructions for deeper insight.

5. Triggers Real-Time Alerts: Once enriched, this transaction can be used to immediately notify users (via alerting mechanisms you define) of a live token migration.



![screenshot](assets/usage_screenshot.png?raw=true "Screenshot")


## Getting Started

1. **Clone the repository:**
   ```bash
   git clone https://github.com/Shyft-to/solana-defi.git
   
   cd PumpFun/Typescript/stream_pumpfun_to_pumpfunAmm_migration-transactions
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
