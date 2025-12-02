# Real-Time Token Migration Alerts via RabbitStream: Tracking Migrations from Pump.fun to Pumpfun Amm

Stay on top of every token migration with RabbitStream, a high-performance, real-time streaming solution that instantly alerts you when a token moves from Pump.fun to Pumpfun Amm. Leveraging RabbitStream's capabilities, we ensure that as soon as a token completes its bonding phase on Pump.fun, we track and alert you to the migration in real-time, with enriched transaction data for full transparency.

## How RabbitStream Detects and Alerts Token Migrations (Step-by-Step)
1. Streams Live Solana Transactions: RabbitStream constantly monitors the Solana blockchain for incoming transactions, parsing and inspecting each one in real-time.

2. Detects the migrate Instruction: Using RabbitStream’s robust stream processing, we specifically search for the migrate instruction within the transaction data. This instruction indicates that a token is transitioning from Pump.fun to Pumpfun Amm.

3. Filters Migration Events: Only transactions containing the migrate instruction are processed further. Non-migration transactions are ignored, ensuring you only receive alerts for relevant events.

4. Enriches Transaction Data: Once a migration is detected, RabbitStream enriches the transaction data by adding relevant metadata, such as additional transaction details and inner instructions, to provide a comprehensive view of the migration process.

5. Triggers Real-Time Alerts: As soon as a token migration is detected and enriched, RabbitStream instantly triggers real-time alerts. These alerts can be customized to notify users through various channels (e.g., SMS, email, or webhooks), ensuring you are always in the loop about live migrations.

```
$ cargo run -- --endpoint <endpoint> --x-token <token>
```

![screenshot](assets/usage-screenshot.png?raw=true "Screenshot")

## Related Links
- Rabbitstream vs gRPC comparison: [https://github.com/Shyft-to/yellowstone-grpc-vs-rabbitstream/tree/main/PumpFun/Rust/stream-pump-fun-new-minted-tokens]
- Shyft Rabbitstream Docs: [https://docs.shyft.to/rabbitstream/rabbitstream-overview]
- Shyft Website: [https://shyft.to/]