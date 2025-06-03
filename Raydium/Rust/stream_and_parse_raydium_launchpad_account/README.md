# Real-time Streaming and Parsing of Raydium Launchpad  Account Updates via gRPC

This project enables real-time streaming and parsing of Raydium Launchpad account updates on the Solana blockchain via gRPC services.
It efficiently monitors on-chain accounts, extracts essential data, and processes it in a structured format.
By integrating seamlessly with the Solana ecosystem, it provides accurate, up-to-date insights into liquidity pool states—ensuring fast, reliable data access for DeFi applications.

```
$ cargo run -- --endpoint <endpoint> --x-token <token>
```

![screenshot](assets/raydium-launchpad.png?raw=true "Screenshot")

## Notes

gRPC client example in rust: [https://github.com/rpcpool/yellowstone-grpc/tree/master/examples/rust]