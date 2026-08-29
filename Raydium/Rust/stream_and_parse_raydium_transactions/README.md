# Streaming Raydium transaction using gRPC and parsing instructions in Rust

This project streams Raydium Amm (675kPX9MHTjS2zt1qfr1NYHuzeLXfQM9H24wFSUt1Mp8) transactions from gRPC, parsing both Raydium and Token Program instructions in real-time. It efficiently decodes transaction data, extracts meaningful insights, and structures the parsed instructions into a serializable format for easy processing and analysis. The implementation ensures smooth integration with Solana’s ecosystem, leveraging Rust’s strong type safety and performance for handling high-throughput transaction streams.

```
$ cargo run -- --endpoint <endpoint> --x-token <token>
```

![screenshot](assets/usage-screenshot.png?raw=true "Screenshot")

## Notes

gRPC client example in rust: [https://github.com/rpcpool/yellowstone-grpc/tree/master/examples/rust]