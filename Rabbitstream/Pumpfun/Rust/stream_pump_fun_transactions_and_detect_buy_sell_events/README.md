# Stream Pumpfun Transactions and Track Buy/Sell Events in Real-Time with Rabbitstream  
![screenshot](assets/usage-screenshot.png?raw=true "Screenshot")

This project centers on real-time streaming of Pump.fun transactions using Rabbitstream, with a focus on parsing buy and sell events as they occur. By decoding raw transaction data, it extracts essential details such as buyer and seller addresses, token amounts, and trade direction.

Built with Rust for high performance and strong type safety, the implementation is optimized for processing high-throughput transaction streams across the Solana network. Its modular structure allows for seamless integration into larger systems, making it easy to derive actionable insights from Pump.fun and Token Program instructions.

Ideal for developers and traders alike, this solution provides a reliable foundation for monitoring and analyzing on-chain trading behavior within the Pump.fun ecosystem in real time.


```
$ cargo run -- --endpoint <endpoint> --x-token <token>
```

![screenshot](assets/usage-screenshot.png?raw=true "Screenshot")

## Related Links

- Rabbitstream vs gRPC comparison: [https://github.com/Shyft-to/yellowstone-grpc-vs-rabbitstream/tree/main/PumpFun/Rust/stream-pump-fun-new-minted-tokens]
- Shyft Rabbitstream Docs: [https://docs.shyft.to/rabbitstream/rabbitstream-overview]
- Shyft Website: [https://shyft.to/]