# Stream and parse Pumpswap AMM transaction instructions via Rabbitstream

This project provides a Node.js service to fetch and parse transactions from Pumpswap AMM using Rabbitstream.

## Features
- Uses Solana Rabbitstream API for efficient transaction retrieval.
- Parses transactions using `@shyft-to/solana-transaction-parser`.
- Supports `.env` configuration for easy authentication.

## Installation
```sh
git clone https://github.com/Shyft-to/solana-defi.git
cd Rabbitstream/Pumpfun/Typescript/stream_and_parse_pump_amm_transaction
npm install
```

## Configuration
Create a `.env` file in the root directory based on `.env.example`:
```
RABBITSTREAM_URL=https://grpc.ams.shyft.to
X_TOKEN=YOUR_AUTH_TOKEN
```

## Usage
### Development
```sh
npm run watch
```

### Build
```sh
npm run build
```

### Run
```sh
npm start
```

## Dependencies
- `@solana/web3.js` for interacting with Solana blockchain.
- `@triton-one/yellowstone-grpc` for gRPC communication.
- `@shyft-to/solana-transaction-parser` for transaction parsing.

## License
This project is licensed under the [MIT License](LICENSE).

## Related Links
- Rabbitstream vs gRPC comparison: [https://github.com/Shyft-to/yellowstone-grpc-vs-rabbitstream/tree/main/PumpFun/Rust/stream-pump-fun-new-minted-tokens]
- Shyft Rabbitstream Docs: [https://docs.shyft.to/rabbitstream/rabbitstream-overview]    
- Shyft Website: [https://shyft.to/]