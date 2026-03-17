# Stream Real-Time Token Prices from Meteora DLMM via gRPC

A Typescript code that streams and parses Meteora DLMM swap transactions in real-time using Solana gRPC, computing live token prices from on-chain trade events.

## Features

- Streams Meteora DLMM swap transactions in real-time via Solana gRPC.
- Parses swap instructions using `@shyft-to/solana-transaction-parser`.
- Computes token price per swap (denominated in SOL, USDC, USDT, or any quote token).
- Identifies trade side — **buy** or **sell** — relative to token X.
- Extracts fee info, bin IDs, raw and human-readable amounts.
- Supports `.env` configuration for easy authentication.

## How It Works

Every time a swap occurs on a Meteora DLMM pool, the service:
1. Receives the transaction over gRPC.
2. Locates the `TradeEvent` emitted by the DLMM program.
3. Resolves token decimals from pre/post balances (with fallbacks for SOL, USDC, USDT).
4. Computes the price of token X in terms of token Y (e.g. `0.0426 SOL`).
5. Outputs a structured price object per swap.

## Sample Output

```json
{
  "lbPair": "G7ixPyiyNeggVf1VanSetFMNbVuVCPtimJmd9axfQqng",
  "from": "NNNmq2kyMRToN1vXWLU6wnXXWqkoQxV3PKi9DWXFyha",
  "side": "Sell",
  "tokenInMint": "27G8MtK7VtTcCHkpASjSDdkWWYfoqT6ggEuKidVJidD4",
  "tokenOutMint": "So11111111111111111111111111111111111111112",
  "amountIn": 7.007167,
  "amountOut": 0.298197451,
  "price": 0.042556,
  "priceOfXInY": "0.0426 SOL",
  "binId": { "start": 37512, "end": 37511 },
  "fees": {
    "fee": 757,
    "protocolFee": 37,
    "feeBps": 109102,
    "feePercent": 0.109102
  }
}
```

## Installation

```sh
git clone https://github.com/Shyft-to/solana-defi.git
cd Meteora/Typescript/stream-meteora-dlmm-token-price
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
