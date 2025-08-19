# Stream token price update on pumpfun amm


This project streams real-time token prices from the Pump.fun AMM using the formula `pool_sol_reserve / pool_token_reserve`. It enables users to track price movements and market capitalization in real time. By monitoring token prices after migration, the project provides valuable insights into market trends and potential investment opportunities.

```
$ cargo run -- --endpoint <endpoint> --x-token <token>
```

![screenshot](assets/pump-amm.png?raw=true "Screenshot")

## Notes

gRPC client example in rust: [https://github.com/rpcpool/yellowstone-grpc/tree/master/examples/rust]
