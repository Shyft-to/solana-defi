# Streaming and Parsing Meteora DAMM v2 New Pools using gRPC

This project enables real-time streaming and parsing of newly created pools on Meteora’s DAMM (Dynamic Automated Market Maker) by detecting the `initialize_pool` instruction.
It efficiently captures, decodes, and structures transaction data related to Meteora DAMM and Solana Token Program instructions as they occur on-chain.

Optimized for high-throughput environments on the Solana blockchain, this solution delivers low-latency, structured insights ideal for DeFi analytics, protocol monitoring, and automated workflows.

Built in Rust for safety and performance, and using gRPC streaming, the implementation is highly scalable and reliable for processing Solana transaction data in real time.


```
$ cargo run -- --endpoint <endpoint> --x-token <token>
```

![screenshot](assets/usage-screenshot.png?raw=true "Screenshot")

## Notes

gRPC client example in rust: [https://github.com/rpcpool/yellowstone-grpc/tree/master/examples/rust]