# Streaming Pumpfun Amm transaction using gRPC and parsing instructions in Rust

This project enables real-time gRPC streaming of Pumpfun AMM transactions on Solana, specifically targeting the program at pAMMBay6oceH9fJKBRHGP5D4bD4sWpmSwMn52FMfXEA.
It efficiently parses instructions from both the Pumpfun AMM and the Token Program, extracting essential transaction details for analysis.

Built with Rust, the implementation emphasizes high-performance decoding, leveraging the language’s strong type safety and concurrency features to handle high-throughput transaction streams.
Parsed instructions are structured into a serializable format to ensure seamless integration with downstream systems.

This architecture ensures scalable and efficient processing, making it well-suited for applications that require deep insight into Solana-based DeFi activity.

```
$ cargo run -- --endpoint <endpoint> --x-token <token>
```

![screenshot](assets/pump-amm.png?raw=true "Screenshot")

## Notes

gRPC client example in rust: [https://github.com/rpcpool/yellowstone-grpc/tree/master/examples/rust]