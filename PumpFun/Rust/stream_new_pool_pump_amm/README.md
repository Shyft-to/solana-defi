# Streaming PumpSwap Amm transaction and Buy/Sell Event Detection on Solana

This project provides real-time streaming and decoding of PumpSwap AMM buy/sell transactions on Solana using gRPC and Rust. It targets the pAMMBay6oceH9fJKBRHGP5D4bD4sWpmSwMn52FMfXEA program to extract key trading data for analytics and monitoring.

## Features
- 🔄 Real-Time gRPC Streaming: Continuously ingests Solana transactions related to the PumpSwap AMM with low-latency performance.
- 🧠 Buy/Sell Instruction Decoding: Parses and structures critical trading instructions, including base/quote pool details.
- 📊 Market Insight Extraction: Delivers structured outputs to analytics platforms, enabling deep visibility into DeFi trading trends.
- ⚙️ High-Performance Rust Backend: Built for speed and reliability, ideal for demanding real-time financial applications.
- 🔧 Easy Integration: Outputs data in a format ready for dashboards, alerts, and market analysis tools.

```
$ cargo run -- --endpoint <endpoint> --x-token <token>
```

![screenshot](assets/pump-amm-event.png?raw=true "Screenshot")

## Notes

gRPC client example in rust: [https://github.com/rpcpool/yellowstone-grpc/tree/master/examples/rust]
