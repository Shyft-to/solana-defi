# Streaming and parsing Meteora Pools accounts using gRPC

This project streams Meteora Pools account updates in real-time using the using gRPC, and then decodes the received account updates using the associated program IDL. By utilizing the IDL for parsing, it enables real-time analysis and insights into DLMM activities on the Solana blockchain.


```
$ cargo run -- --endpoint <endpoint> --x-token <token>
```

![screenshot](assets/usage-screenshot.png?raw=true "Screenshot")

## Docs
Shyft Website: [https://shyft.to/#solana-grpc-streaming-service]  
Shyft gRPC Docs: [https://docs.shyft.to/solana-fast-grpc/grpc-docs]
