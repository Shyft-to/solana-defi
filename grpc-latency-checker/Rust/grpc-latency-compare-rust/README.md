This tool allows you to compare stream data from different gRPC endpoints, such as Yellowstone and Shred, for performance benchmarking.

---

## 🛠 Environment Setup

Copy the contents of .env.example to a new .env file and update the values accordingly:

```bash
cp .env.example .env
```

## Run
```bash
cargo run --bin compare
```

## Example Configurations

1. Compare multiple gRPC endpoints (Yellowstone)
```
YELLOWSTONE_STREAM_CONFIGS='[{"uri":"yellowstone_grpc_uri_1","x_token":"yellowstone_x_token_1"},{"uri":"yellowstone_grpc_uri_2","x_token":"yellowstone_x_token_2"}]'
# SHRED_STREAM_CONFIGS='[{"uri":"shred_grpc_uri_1","x_token":"shred_x_token_1"},{"uri":"shred_grpc_uri_2","x_token":"shred_x_token_2"}]'
TIMEOUT_DUR=30 # seconds
```

2. Compare Yellowstone gRPC with Shred
```
YELLOWSTONE_STREAM_CONFIGS='[{"uri":"yellowstone_grpc_uri_1","x_token":"yellowstone_x_token_1"}]'
SHRED_STREAM_CONFIGS='[{"uri":"shred_grpc_uri_1","x_token":"shred_x_token_1"}]'
TIMEOUT_DUR=30 # seconds
```

3. Compare Shred with Shred
```
# YELLOWSTONE_STREAM_CONFIGS='[{"uri":"yellowstone_grpc_uri_1","x_token":"yellowstone_x_token_1"}]'
SHRED_STREAM_CONFIGS='[{"uri":"shred_grpc_uri_1","x_token":"shred_x_token_1"},{"uri":"shred_grpc_uri_2","x_token":"shred_x_token_2"}]'
TIMEOUT_DUR=30 # seconds
```

4. Compare multiple Yellowstone and multiple Shred endpoints
```
YELLOWSTONE_STREAM_CONFIGS='[{"uri":"yellowstone_grpc_uri_1","x_token":"yellowstone_x_token_1"},{"uri":"yellowstone_grpc_uri_2","x_token":"yellowstone_x_token_2"}]'
SHRED_STREAM_CONFIGS='[{"uri":"shred_grpc_uri_1","x_token":"shred_x_token_1"},{"uri":"shred_grpc_uri_2","x_token":"shred_x_token_2"}]'
TIMEOUT_DUR=30 # seconds
```

Only set the relevant environment variables based on the type of comparison you want to perform.