# Streaming Meteora DAMM transaction using gRPC and parsing instructions in Rust

This project streams Meteora Damm transactions via gRPC, parsing both Meteora Damm and Token Program instructions in real time. It efficiently decodes transaction data, extracts valuable insights, and formats parsed instructions into a serializable structure for seamless processing and analysis. Built for Solana’s ecosystem, the implementation leverages Rust’s strong type safety and performance to handle high-throughput transaction streams effectively.
```
$ cargo run -- --endpoint <endpoint> --x-token <token>
```

![screenshot](assets/usage-screenshot.png?raw=true "Screenshot")

## Notes

gRPC client example in rust: [https://github.com/rpcpool/yellowstone-grpc/tree/master/examples/rust]