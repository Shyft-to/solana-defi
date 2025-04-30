# Real-Time Raydium Launchpad Pool Tracker with gRPC

This project enables real-time tracking and parsing of new Raydium Launchpad pool transactions using gRPC for fast, low-latency data streaming. It decodes transactions on the fly to deliver structured, actionable insights—perfect for developers and analysts building on the Raydium ecosystem.

## Features
- Efficient Transaction Retrieval: Utilizes Solana Shyft gRPC for fast and reliable access to blockchain transaction data.

- Accurate Parsing: Leverages @shyft-to/solana-transaction-parser for structured and readable transaction decoding.

- Secure Configuration: Supports .env configuration for simple and secure authentication setup.

- Raydium Launchpad IDL Access: Provides easy access to the Raydium Launchpad Interface Definition Language for seamless integration.

 ![screenshot](assets/raydium-new-pool.png?raw=true "Screenshot")


## Installation
```sh
git clone https://github.com/Shyft-to/solana-defi.git
cd stream_new_pool_raydium_launchpad
npm install
```

## Configuration
Create a `.env` file in the root directory based on `.env.example`:
```

GRPC_URL= YOUR_GRPC_LOCATION_URL
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

