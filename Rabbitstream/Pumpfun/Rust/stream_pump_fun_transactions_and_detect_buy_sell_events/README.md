# Stream Pumpfun Transactions and Track Buy/Sell Events in Real-Time with Rabbitstream

This project focuses on streaming Pump.fun transactions using Rabbitstream and parsing buy/sell events in real-time. By decoding transaction data, it identifies key details such as the buyer/seller addresses and the transaction amounts. The implementation leverages Rust’s performance and type safety to efficiently process high-throughput transaction streams from Solana’s ecosystem. Structured for seamless integration, this solution extracts actionable insights from Pump.fun and Token Program instructions, enabling real-time analysis of market activity. Ideal for traders and developers, it provides a robust foundation for tracking and analyzing on-chain trading behavior on Pump.fun.

```
$ cargo run -- --endpoint <endpoint> --x-token <token>
```

![screenshot](assets/usage-screenshot.png?raw=true "Screenshot")

## Related Links
- Rabbitstream vs gRPC comparison: [https://github.com/Shyft-to/yellowstone-grpc-vs-rabbitstream/tree/main/PumpFun/Rust/stream-pump-fun-new-minted-tokens]
- Shyft Rabbitstream Docs: [https://docs.shyft.to/rabbitstream/rabbitstream-overview]    
- Shyft Website: [https://shyft.to/]