# Solana gRPC transaction streaming and block based latency checker

This project analyzes Solana transaction latency when streaming with gRPC streaming. By monitoring both individual transactions and block processing times, we aim to observe the latency provided by any Yellowstone gRPC on Solana.

To run this project, use the following in your project directory
```
$ cargo run -- --endpoint <endpoint> --x-token <token> --commitment processed --timeout-dur 60
```
   
Parameters
* endpoint: your gRPC url.
* x-token: x-token for gRPC auth.
* commitment: can be processed, confirmed or finalized.
* timeout-dur: time in seconds for which you want to run the code to view latency. 60 here means 60 seconds.

## Notes

gRPC client example in rust: [https://github.com/rpcpool/yellowstone-grpc/tree/master/examples/rust]
Shyft gRPC Docs: [https://docs.shyft.to/solana-grpc-shredstream/grpc-docs]