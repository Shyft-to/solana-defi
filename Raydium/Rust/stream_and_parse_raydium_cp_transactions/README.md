# Streaming Raydium CP transaction using gRPC and parsing instructions in Rust

This project enables real-time streaming of Raydium CP (CPMMoo8L3F4NbTegBCKVNunggL7H1ZpdTHKxQB5qKP1C) transactions via gRPC. 
It efficiently parses both Raydium CP and Token Program instructions, extracting key transaction data. 
The implementation focuses on high-performance decoding of transaction details, 
structuring the parsed instructions into a serializable format for seamless processing and analysis.
Built using Rust, the project leverages the language's type safety and 
performance to handle high-throughput transaction streams,
ensuring smooth integration with Solana's ecosystem for optimal scalability and efficiency.

```
$ cargo run -- --endpoint <endpoint> --x-token <token>
```

![screenshot](assets/usage__screenshot.PNG?raw=true "Screenshot")

## Notes

gRPC client example in rust: [https://github.com/rpcpool/yellowstone-grpc/tree/master/examples/rust]