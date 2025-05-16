# Streaming Raydium CLMM transaction using gRPC and parsing instructions in Rust

This Rust-based project enables high-performance, real-time streaming of Raydium CLMM (CAMMCzo5YL8w4VFF8KVHrK22GGUsp5VTaW7grrKgrWqK) transactions on the Solana blockchain using gRPC. Designed for scalability and efficiency, it decodes and parses both Raydium Concentrated Liquidity Market Maker (CLMM) and Token Program instructions, extracting essential transaction data for downstream processing.

Leveraging Rust’s memory safety, speed, and concurrency features, the implementation ensures robust handling of high-throughput transaction streams. Parsed instructions are structured into a clean, serializable format, enabling seamless integration with data analytics platforms, trading bots, or blockchain monitoring tools.

This solution is ideal for developers and teams looking to build scalable DeFi tools, real-time dashboards, or automated systems in the Solana DeFi ecosystem with optimal performance and reliability.

```
$ cargo run -- --endpoint <endpoint> --x-token <token>
```

![screenshot](assets/usage__screenshot.png?raw=true "Screenshot")

## Notes

gRPC client example in rust: [https://github.com/rpcpool/yellowstone-grpc/tree/master/examples/rust]