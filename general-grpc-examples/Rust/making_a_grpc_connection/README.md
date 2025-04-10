# Making a gRPC connection on Solana

This project streams Meteora transactions from gRPC, parsing both Meteora and Token Program instructions in real-time. It efficiently decodes transaction data, extracts meaningful insights, and structures the parsed instructions into a serializable format for easy processing and analysis. The implementation ensures smooth integration with Solana’s ecosystem, leveraging Rust’s strong type safety and performance for handling high-throughput transaction streams.

```Rust
async fn connect(&self) -> anyhow::Result<GeyserGrpcClient<impl Interceptor>> {
    GeyserGrpcClient::build_from_shared(self.endpoint.clone())?
        .x_token(Some(self.x_token.clone()))?
        .connect_timeout(Duration::from_secs(10))
        .timeout(Duration::from_secs(10))
        .tls_config(ClientTlsConfig::new().with_native_roots())?
        .max_decoding_message_size(1024 * 1024 * 1024)
        .connect()
        .await
        .map_err(Into::into)
}
```

![screenshot](assets/usage-screenshot.png?raw=true "Screenshot")

## Notes

gRPC client example in rust: [https://github.com/rpcpool/yellowstone-grpc/tree/master/examples/rust]