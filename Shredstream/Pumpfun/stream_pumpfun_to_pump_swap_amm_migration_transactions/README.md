# Real-Time Token Migration Alerts: Pumpfun  to Pumpswap Amm 

Stay informed with this real-time, low-latency system designed to stream and decode Pumpfun Launchpad transaction instructions using Shredstream. This tool enables advanced monitoring of compiled Solana instructions, detecting and decoding critical calls like `migrate` — which indicate migration of tokens or liquidity from Pumpfun Launchpad to Pumpswap AMM pools.

## How the Decoder Works (Step-by-Step):

1. **Streams Compiled Instructions**
    The system uses Shredstream to continuously receive and parse compiled Solana transactions in real-time
2. **Filters for Pumpswap AMM Migration Instructions**
   It scans incoming instructions for specific Pumpfun  program calls — primarily `migrate`.

3. **Decodes Using IDL**
   Upon detecting relevant instructions, it decodes them using the Pumpfun IDL (Interface Definition Language) to produce a readable and structured format.

4. **Enriches Transaction Data**
   Adds context such as source addresses, token info, and parsed instruction arguments for full transparency.

5. **Triggers Real-Time Alerts**
   Decoded and enriched data can be sent to dashboards, bots, or other alerting tools you define for instant awareness of migration events.


Replace values with your actual Shredstream endpoint and token:

   ```bash
   cargo run -- --shredstream-uri <uri> --x-token <token> --pubkey 6EF8rrecthR5Dkzon8Nwu78hRvfCKubJ14M5uBEwF6P
   ```

   * `--shredstream-uri`: The Shredstream endpoint.
   * `--x-token`: Your API token from Shyft.
   * `--pubkey`: The Pumpfun program ID to monitor.



### NOTES:
* See Shredstream Rust Client Examples: [https://github.com/Shyft-to/shredstream-decode-examples](https://github.com/Shyft-to/shredstream-decode-examples)
* Get the Jito-Protos files from :[https://github.com/jito-labs/mev-protos/tree/d297adc6e97e79df167a7ef1c2b1378973aa4b8c](https://github.com/jito-labs/mev-protos/tree/d297adc6e97e79df167a7ef1c2b1378973aa4b8c)
* Compatible with Pumpfun Launchpad and extended use-cases like Pumpswap AMM.

