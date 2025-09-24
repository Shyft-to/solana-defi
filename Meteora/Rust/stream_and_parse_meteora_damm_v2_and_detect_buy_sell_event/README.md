# Streaming Meteora Damm v2 Transactions and Detecting Buy and Sell Events

This project provides real-time monitoring and parsing of Meteora Damm transactions on the Solana blockchain using gRPC. It captures buy and sell events as they occur, offering actionable insights into market trends, trading behaviors, and liquidity flows within the Meteora ecosystem.

The system decodes and interprets instructions from both the Meteora Damm protocol and the Solana Token Program, transforming raw blockchain data into a clean, structured, and serializable format—ready for downstream processing and analytics.

Built with Rust, the implementation takes advantage of the language’s type safety, performance, and ecosystem support to reliably handle high-throughput, real-time data streams with precision and efficiency.
```
$ cargo run -- --endpoint <endpoint> --x-token <token>
```

![screenshot](assets/usage-screenshot.png?raw=true "Screenshot")

## Notes

gRPC client example in rust: [https://github.com/rpcpool/yellowstone-grpc/tree/master/examples/rust]