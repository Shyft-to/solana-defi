# Streaming Meteora DBC Transactions and Detecting Buy and Sell Events

This project monitors and parses Meteora DBC transactions on the Solana blockchain in real time using gRPC. By tracking buy and sell events as they happen, it delivers valuable insights into on-chain market activity, helping users understand market trends, trading behavior, and liquidity flows within the Meteora ecosystem.

The system decodes and interprets both Meteora DBC-specific instructions and those from the Solana Token Program, extracting actionable data and transforming it into a clean, serializable format for seamless downstream processing and analysis.

Built with Rust, the implementation leverages its type safety, flexibility, and rich ecosystem support to efficiently handle real-time streaming data with reliability and precision.


```
$ cargo run -- --endpoint <endpoint> --x-token <token>
```

![screenshot](assets/meteora-dbc.png?raw=true "Screenshot")

## Notes

gRPC client example in rust: [https://github.com/rpcpool/yellowstone-grpc/tree/master/examples/rust]