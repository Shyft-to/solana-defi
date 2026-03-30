# Stream Real-Time Token Prices from Meteora Pool via gRPC

A Typescript code that streams and parses Meteora Pool swap transactions in real-time using Solana gRPC, computing live token prices from on-chain trade events.

## Features

- Streams Meteora Pool swap transactions in real-time via Solana gRPC.
- Parses swap instructions using `@shyft-to/solana-transaction-parser`.
- Computes token price per swap (denominated in SOL, USDC, USDT, or any quote token).
- Identifies trade side — **buy** or **sell** — relative to token X.
- Extracts fee info, bin IDs, raw and human-readable amounts.
- Supports `.env` configuration for easy authentication.

## How It Works

Every time a swap occurs on a Meteora Pool, the service:
1. Receives the transaction over gRPC.
2. Locates the `TradeEvent` emitted by the DLMM program.
3. Resolves token decimals from pre/post balances (with fallbacks for SOL, USDC, USDT).
4. Computes the price of token X in terms of token Y (e.g. `0.0426 SOL`).
5. Outputs a structured price object per swap.

## Sample Output

```json
 {
  "tokenIn": {
    "mint": "So11111111111111111111111111111111111111112",
    "decimals": 9,
    "amount": "499999",
    "uiAmount": 0.000499999
  },
  "tokenOut": {
    "mint": "2e72ABjRFGBEuTSaPhYoGx67gaYv53n2qoYSfrxm3sUV",
    "decimals": 6,
    "amount": "278657770057217",
    "uiAmount": 278657770.057217
  },
  "priceOfTokenIn": "557316654747.7435302734375",
  "priceOfTokenOut": "0.00000000000179431207"
}

--------------------------------------------------------------------------------------------------



```

## Installation

```sh
git clone https://github.com/Shyft-to/solana-defi.git
cd Meteora/Typescript/stream_meteora_pool_token_price
npm install
```

## Configuration

Create a `.env` file in the root directory based on `.env.example`:

```env
GRPC_URL=https://grpc.ams.shyft.to
X_TOKEN=YOUR_AUTH_TOKEN
```

Get your token at [shyft.to](https://shyft.to).

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
npm run start
```

## Dependencies

- `@solana/web3.js` — Solana blockchain interaction.
- `@triton-one/yellowstone-grpc` — gRPC streaming.
- `@shyft-to/solana-transaction-parser` — Transaction parsing.
- `@coral-xyz/anchor` — BN arithmetic for raw token amounts.

## Related Links

* Shyft gRPC Docs: [https://docs.shyft.to/solana-fast-grpc/grpc-docs](https://docs.shyft.to/solana-fast-grpc/grpc-docs)
* Start Streaming with Shyft: [https://shyft.to/solana-yellowstone-grpc](https://shyft.to/solana-yellowstone-grpc)
* Shyft Website: [https://shyft.to/](https://shyft.to/)