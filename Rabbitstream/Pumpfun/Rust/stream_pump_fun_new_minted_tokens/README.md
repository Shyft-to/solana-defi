# Streaming Pumpfun transaction using Rabbitstream and parsing instructions in Rust

This project streams Pump.fun transactions from Rabbitstream, parsing both Pump.fun and Token Program instructions in real-time. It efficiently decodes transaction data, extracts meaningful insights, and structures the parsed instructions into a serializable format for easy processing and analysis. The implementation ensures smooth integration with Solana’s ecosystem, leveraging Rust’s strong type safety and performance for handling high-throughput transaction streams.

```
$ cargo run -- --endpoint <endpoint> --x-token <token>
```

![screenshot](assets/usage-screenshot.png?raw=true "Screenshot")

## Related Links

- Rabbitstream vs gRPC comparison: [https://github.com/Shyft-to/yellowstone-grpc-vs-rabbitstream/tree/main/PumpFun/Rust/stream-pump-fun-new-minted-tokens]
- Shyft Rabbitstream Docs: [https://docs.shyft.to/rabbitstream/rabbitstream-overview]
- Shyft Website: [https://shyft.to/]