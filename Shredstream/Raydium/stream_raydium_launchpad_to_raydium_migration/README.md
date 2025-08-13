# Real-Time Token Migration Alerts: Raydium Launchpad to Raydium Amm & Raydium CP

Stay informed with this real-time, low-latency system designed to stream and decode **Raydium Launchpad** transaction instructions using [Shredstream](https://github.com/Shyft-to/shredstream-decode-examples). This tool enables advanced monitoring of compiled Solana instructions, detecting and decoding critical calls like `migrate_to_amm` or `migrate_to_cpswap` — which indicate migration of tokens or liquidity from launchpad to Raydium AMM or CPSwap pools.


## How the Decoder Works (Step-by-Step):

1. **Streams Compiled Instructions**
   The system uses Shredstream to continuously receive and parse compiled Solana transactions in real-time.

2. **Filters for Raydium Migration Instructions**
   It scans incoming instructions for specific Raydium Launchpad program calls — primarily `migrate_to_amm` and `migrate_to_cpswap`.

3. **Decodes Using IDL**
   Upon detecting relevant instructions, it decodes them using the Raydium IDL (Interface Definition Language) to produce a readable and structured format.

4. **Enriches Transaction Data**
   Adds context such as source addresses, token info, and parsed instruction arguments for full transparency.

5. **Triggers Real-Time Alerts**
   Decoded and enriched data can be sent to dashboards, bots, or other alerting tools you define for instant awareness of migration events.


Replace values with your actual Shredstream endpoint and token:

   ```bash
   cargo run -- --shredstream-uri <uri> --x-token <token> --pubkey LanMV9sAd7wArD4vJFi2qDdfnVhFxYSUg6eADduJ3uj
   ```

   * `--shredstream-uri`: The Shredstream endpoint.
   * `--x-token`: Your API token from Shyft.
   * `--pubkey`: The Raydium Launchpad program ID to monitor.



### NOTES:
* See Shredstream Rust Client Examples: [https://github.com/Shyft-to/shredstream-decode-examples](https://github.com/Shyft-to/shredstream-decode-examples)
* Get the Jito-Protos files from :[https://github.com/jito-labs/mev-protos/tree/d297adc6e97e79df167a7ef1c2b1378973aa4b8c](https://github.com/jito-labs/mev-protos/tree/d297adc6e97e79df167a7ef1c2b1378973aa4b8c)
* Compatible with both Raydium Launchpad and extended use-cases like CPSwap.


