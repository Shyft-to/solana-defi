# Stream and parse Raydium Launchpad transaction instructions via gRPC

This project enables real-time streaming and parsing of Raydium Launchpad transactions using gRPC,
ensuring efficient and scalable data extraction. By decoding transaction data on the fly, 
it provides actionable insights through structured serialization, optimized for high-performance processing.
Designed for scalability and low latency, 
this solution is ideal for developers and analysts looking to harness real-time blockchain data from the Raydium ecosystem.

## Features
- Efficient Transaction Retrieval: Utilizes Solana Shyft gRPC for fast and reliable access to blockchain transaction data..
- Accurate Parsing: Leverages @shyft-to/solana-transaction-parser for structured and readable transaction decoding.
- Secure Configuration: Supports .env configuration for simple and secure authentication setup.
- Raydium Launchpad IDL: Easy access to Raydium Launchpad IDL

```
$ cargo run -- --endpoint <endpoint> --x-token <token>
```

![screenshot](assets/usage-screenshot.png?raw=true "Screenshot")

## Notes

gRPC client example in rust: [https://github.com/rpcpool/yellowstone-grpc/tree/master/examples/rust]