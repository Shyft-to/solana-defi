# Stream and parse Pump Swap Amm New Pool transactions instructions via Rabbitstream
A Rust service that streams real-time Solana transactions via gRPC and detects new pool creations on the PumpFun AMM by filtering txn using its instruction name `CreatePool`.

## Key Features
- Real-time monitoring of Solana blockchain via Rabbitstream 

- Automated detection of CreatePool instructions from PumpFun AMM

- Comprehensive parsing of transaction data using custom IDL definitions

- Multi-protocol support for both PumpFun AMM and SPL Token programs
```
```
$ cargo run -- --endpoint <endpoint> --x-token <token>
```

![screenshot](assets/usage-screenshot.png?raw=true "Screenshot")

## Related Links

- Rabbitstream vs gRPC comparison: [https://github.com/Shyft-to/yellowstone-grpc-vs-rabbitstream/tree/main/PumpFun/Rust/stream-pump-fun-new-minted-tokens]
- Shyft Rabbitstream Docs: [https://docs.shyft.to/rabbitstream/rabbitstream-overview]
- Shyft Website: [https://shyft.to/]