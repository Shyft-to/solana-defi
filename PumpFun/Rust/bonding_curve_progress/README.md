# Monitoring Bonding Curve Progress

This project is designed to stream and monitor the progress of bonding curves on Pumpfun—a platform that enables the tracking and analysis of token dynamics. By leveraging the power of gRPC, this tool allows users to get real-time updates on the bonding curve progress of tokens listed on Pumpfun, providing insights into token behavior, pricing mechanisms, and market liquidity.
```
$ cargo run -- --endpoint <endpoint> --x-token <token>
```

![screenshot](assets/usage-screenshot.png?raw=true "Screenshot")

## Notes
Please input your API_KEY at get_balance.rs

gRPC client example in rust: [https://github.com/rpcpool/yellowstone-grpc/tree/master/examples/rust]