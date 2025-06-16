# Real-Time Streaming and Parsing of Meteora DAMM v2 Account Updates on Solana via Shyft gRPC

This project uses Shyft gRPC to stream Meteora DAMM v2 account data in real-time on the Solana blockchain. Leveraging gRPC’s fast and reliable connection, it enables continuous monitoring of account activity related to liquidity pools.

The primary goal is to read and interpret Meteora DAMM v2 account data, with a focus on significant changes and updates in the pools. This provides users with valuable insights into liquidity flow, pool performance, and the overall status of the pools.
## Key highlights:

1. Utilizes gRPC streaming for high-performance, real-time data delivery.

2. Parses on-chain account structures to extract meaningful state changes.

3. Monitors liquidity pool behavior and updates for informed analytics.


```
$ cargo run -- --endpoint <endpoint> --x-token <token>
```

![screenshot](assets/usage-screenshot.png?raw=true "Screenshot")

## Notes

gRPC client example in rust: [https://github.com/rpcpool/yellowstone-grpc/tree/master/examples/rust]