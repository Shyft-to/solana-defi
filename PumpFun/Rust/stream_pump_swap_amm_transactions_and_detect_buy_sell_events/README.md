# Streaming Pumpfun Amm transaction using gRPC and parsing instructions in Rust

This Rust-powered service provides real-time gRPC streaming and parsing of buy and sell transactions from the PumpFun AMM on Solana. By targeting the pAMMBay6oceH9fJKBRHGP5D4bD4sWpmSwMn52FMfXEA program, it efficiently decodes critical buy/sell instructions, including token amounts and price data, helping traders and analysts track market trends. The system extracts detailed transaction insights such as base and quote amounts, ensuring deep visibility into the dynamics of decentralized finance (DeFi) on Solana. Built for high performance, it seamlessly integrates parsed data into analytics platforms, making it an essential tool for understanding and reacting to market shifts in real-time.

```
$ cargo run -- --endpoint <endpoint> --x-token <token>
```

![screenshot](assets/pump-amm-event.png?raw=true "Screenshot")

## Notes

gRPC client example in rust: [https://github.com/rpcpool/yellowstone-grpc/tree/master/examples/rust]