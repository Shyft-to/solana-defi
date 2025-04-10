# Real-time Streaming and Parsing of Raydium CLMM  Account Updates via gRPC

This project enables real-time streaming and efficient parsing of Raydium CLMM (Constant Product Market Maker) account updates on the Solana blockchain using gRPC services.
By continuously monitoring on-chain events, the platform extracts essential liquidity pool data and processes it with high efficiency.
With seamless integration into the Solana ecosystem, it provides up-to-date and accurate liquidity pool state information, ensuring fast, reliable access to critical data for DeFi applications.
This robust solution supports decentralized finance (DeFi) projects by delivering real-time, actionable data for optimal decision-making.

```
$ cargo run -- --endpoint <endpoint> --x-token <token>
```

![screenshot](assets/clmm-account.PNG?raw=true "Screenshot")

## Notes

gRPC client example in rust: [https://github.com/rpcpool/yellowstone-grpc/tree/master/examples/rust]