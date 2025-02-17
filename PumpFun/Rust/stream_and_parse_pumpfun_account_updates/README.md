# Streaming and parsing Pumpfun accounts using gRPC

This project leverages Solana's gRPC streaming capabilities to efficiently monitor pump fun account data. By utilizing the IDL for parsing, it enables real-time analysis and insights into pump fun activities on the Solana blockchain.

```
$ cargo run -- --endpoint <endpoint> --x-token <token>
```

![screenshot](assets/usage-screenshot.png?raw=true "Screenshot")

## Notes

gRPC client example in rust: [https://github.com/rpcpool/yellowstone-grpc/tree/master/examples/rust]
Shyft gRPC Docs: [https://docs.shyft.to/solana-fast-grpc/grpc-docs]