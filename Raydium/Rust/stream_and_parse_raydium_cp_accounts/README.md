# Real-time Streaming and Parsing of Raydium CP  Account Updates via gRPC

This project facilitates real-time streaming and parsing of Raydium CP account updates on the Solana blockchain through gRPC services.
It continuously monitors on-chain events, extracting key properties and processing structured data with high efficiency. 
By seamlessly integrating with Solana’s ecosystem, it delivers accurate and up-to-date liquidity pool state information, 
ensuring reliable and fast data access for DeFi applications.

```
$ cargo run -- --endpoint <endpoint> --x-token <token>
```

![screenshot](assets/cp-account.PNG?raw=true "Screenshot")

## Notes

gRPC client example in rust: [https://github.com/rpcpool/yellowstone-grpc/tree/master/examples/rust]