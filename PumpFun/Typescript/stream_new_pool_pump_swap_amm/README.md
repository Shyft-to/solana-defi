# Stream and parse Pump Swap Amm New Pool transactions instructions via gRPC

This project provides a Node.js service to fetch and parse new pool transactions from Pump Swap AMM using gRPC.

## Features
- Uses Solana gRPC API to retrieve new pool transaction.
- Parses transactions using `@shyft-to/solana-transaction-parser`.
- Supports `.env` configuration for easy authentication.

- ![screenshot](assets/new-pool.png?raw=true "Screenshot")


## Installation
```sh
git clone https://github.com/Shyft-to/solana-defi.git
cd stream_new_pool_pumpfun_swap_amm
npm install
```

## Configuration
Create a `.env` file in the root directory based on `.env.example`:
```

GRPC_URL= GRPC_URL
X_TOKEN= YOUR_AUTH_TOKEN
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
