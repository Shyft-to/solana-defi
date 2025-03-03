# Streaming Pumpfun migrations using gRPC based on account updates

This project leverages Shyft gRPC to stream pump fun accounts on Solana. It focuses on identifying Raydium migrations by analyzing accounts with "bondingCurveComplete: true" and extracting the transferred token address from associated transactions.

```
$ cargo run -- --endpoint <endpoint> --x-token <token> --rpc-url <your-rpc-url>
```

![screenshot](assets/usage-screenshot.png?raw=true "Screenshot")

## Useful Links

gRPC client example in rust: [https://github.com/rpcpool/yellowstone-grpc/tree/master/examples/rust]
Shyft gRPC Docs: [https://docs.shyft.to/solana-fast-grpc/grpc-docs]

### Notes
In certain cases, we have observed that gRPC account updates are in random order and not always the latest one. In such cases, we get the last successful transaction in the account, and check for its blocktime against current time.