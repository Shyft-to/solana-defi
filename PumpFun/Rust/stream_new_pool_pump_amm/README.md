# Stream and parse Pump Swap Amm New Pool transactions instructions via gRPC
A Rust service that streams real-time Solana transactions via gRPC and detects new pool creations on the PumpFun AMM by filtering txn using its instruction name `CreatePool`.

## Key Features
- Real-time monitoring of Solana blockchain via gRPC WebSocket

- Automated detection of CreatePool instructions from PumpFun AMM

- Comprehensive parsing of transaction data using custom IDL definitions

- Multi-protocol support for both PumpFun AMM and SPL Token programs
```
$ cargo run -- --endpoint <endpoint> --x-token <token>
```

![screenshot](assets/pump-amm-event.png?raw=true "Screenshot")

## Notes

gRPC client example in rust: [https://github.com/rpcpool/yellowstone-grpc/tree/master/examples/rust]
