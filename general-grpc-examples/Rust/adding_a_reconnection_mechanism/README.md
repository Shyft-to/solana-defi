# Adding a reconnection mechanism to your gRPC connection

This illustrates adding an automatic reconnection mechanism to your gRPC connection on Rust, to make the streaming process more robust and production ready. This is crucial for maintaining a continuous data flow and preventing application downtime during temporary network issues. The reconnection logic ensures your application can gracefully handle interruptions and automatically re-establish the gRPC connection, improving overall resilience.

```
$ cargo run -- --endpoint <endpoint> --x-token <token>
```

![screenshot](assets/usage-screenshot.png?raw=true "Screenshot")

## Notes

gRPC client example in rust: [https://github.com/rpcpool/yellowstone-grpc/tree/master/examples/rust]
