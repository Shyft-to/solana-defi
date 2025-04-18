# Stream and parse Raydium Launchpad transaction instructions via gRPC

This project enables real-time streaming and parsing of Raydium Launchpad transactions using gRPC,
ensuring efficient and scalable data extraction. By decoding transaction data on the fly, 
it provides actionable insights through structured serialization, optimized for high-performance processing.
Designed for scalability and low latency, 
this solution is ideal for developers and analysts looking to harness real-time blockchain data from the Raydium ecosystem.

## Features
- Efficient Transaction Retrieval: Utilizes Solana Shyft gRPC for fast and reliable access to blockchain transaction data..
- Accurate Parsing: Leverages @shyft-to/solana-transaction-parser for structured and readable transaction decoding.
- Secure Configuration: Supports .env configuration for simple and secure authentication setup.
- Raydium Launchpad IDL: Easy access to Raydium Launchpad IDL

- ![screenshot](assets/raydium-launchpad.png?raw=true "Screenshot")


## Installation
```sh
git clone https://github.com/Shyft-to/solana-defi.git
cd stream_and_parse_raydium_launchpad_transactions
npm install
```

## Configuration
Create a `.env` file in the root directory based on `.env.example`:
```

GRPC_URL=https://grpc.ams.shyft.to
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

