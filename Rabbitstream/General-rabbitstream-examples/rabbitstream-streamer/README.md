# rabbitstream-streamer

Streams Solana transactions via [Yellowstone gRPC](https://github.com/rpcpool/yellowstone-grpc) and prints signatures with throughput stats.

## Setup

**1. Copy the example env file and fill in your values:**

```bash
cp .env.example .env
```

```env
ENDPOINT=https://your-endpoint.solana-mainnet.example.com:10000
X_TOKEN=your_x_token_here
ACCOUNT_INCLUDE=6EF8rrecthR5Dkzon8Nwu78hRvfCKubJ14M5uBEwF6P,pAMMBay6oceH9fJKBRHGP5D4bD4sWpmSwMn52FMfXEA
LOG_SIG=true
```

- `ENDPOINT` — your Yellowstone/Rabbitstream gRPC endpoint (must include port)
- `X_TOKEN` — auth token for the endpoint
- `ACCOUNT_INCLUDE` — comma-separated list of program or account addresses to filter on
- `LOG_SIG` — `true` to print each transaction signature, `false` to suppress (default: `true`)

**2. Run:**

```bash
cargo run
```

You can also pass values directly as flags (these override `.env`):

```bash
cargo run -- --endpoint <URL> --x-token <TOKEN> --account-include <ADDR1>,<ADDR2>
```

## Output

Each received transaction prints its signature:

```
[INFO  rabbitstream_streamer] 2TNkmwSZ...
```

Every 5 seconds a throughput summary is printed:

```
[INFO  rabbitstream_streamer] -----> throughput: 12.4 tx/s | total transactions: 62 <------
```

The streamer automatically reconnects on errors or if no messages are received for 30 seconds.
