# Streaming Meteora DBC transaction using gRPC and parsing instructions in Rust

This project provides real-time gRPC streaming of Meteora DBC transactions on Solana, with a primary focus on the program at dbcij3LWUppWqq96dh6gJWwBifmcGfLSB5D4DuSMaqN. It efficiently decodes and parses instructions from both the Meteora DBC and Solana’s Token Program, extracting key transaction details for in-depth analysis.

Built in Rust, the implementation leverages the language’s strong type safety, concurrency model, and performance guarantees to handle high-throughput transaction streams with reliability. Parsed instructions are organized into a clean, serializable format, enabling seamless integration with downstream systems such as analytics pipelines, monitoring dashboards, or trading agents.

By combining scalable streaming with robust decoding, this architecture delivers a powerful foundation for analyzing Solana-based DeFi activity, making it especially suited for applications that require deep visibility into Meteora DBC transaction flows.

```
$ cargo run -- --endpoint <endpoint> --x-token <token>
```

![screenshot](assets/meteora-dbc.png?raw=true "Screenshot")

## Notes

gRPC client example in rust: [https://github.com/rpcpool/yellowstone-grpc/tree/master/examples/rust]