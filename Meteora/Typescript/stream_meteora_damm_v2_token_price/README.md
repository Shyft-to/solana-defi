<a id="readme-top"></a>

# Stream token price update on Meteora DAMM v2 Token Price

This project streams real-time token prices directly from Meteora DAMM v2 on Solana using gRPC — with no reliance on external RPC or API calls.
It listens to on-chain swap events, extracts the `nextSqrtPrice` field, and converts it from `Q64.64` fixed-point format into a human-readable token price.

By By continuously monitoring these updates, the project offers early insights into **market trends, token valuations, and potential investment opportunities ahead of liquidity migrations**.

---

## 🔎 How Price Conversion Works

Meteora DAMM v2 operates on a **concentrated liquidity AMM model**, where the pool price is represented as a square root value in `Q64.64` fixed-point format.

1. **Extract `nextSqrtPrice`** from the `swapResult` event.
2. **Convert Q64.64 sqrtPrice → Price**:

   $$
   P = \left(\frac{\text{nextSqrtPrice}}{2^{64}}\right)^2
   $$
3. **Adjust for token decimals** to get the ratio between tokenA and tokenB.

---

## 📈 Why This Matters

* **Real-Time Tracking** → Live token price updates as swaps on meteora damm v2.
* **Market Cap & Trend Analysis** → Combine prices with supply data to estimate market capitalization and detect trend shifts.
* **Pre-Migration Insights** → Price signals before liquidity migration can highlight profitable opportunities.

---

## 🚀 Use Cases

* Build Automated **trading bots** reacting to price changes.
* **Portfolio trackers** showing up-to-the-second valuations.
* **Market analysis tools** monitoring liquidity shifts in Meteora pools. etc.

## 🛠️ Core Function

```ts
function sqrtPriceX64ToPrice(nextSqrtPriceStr: string, decimalsA: number, decimalsB: number) {
  const sqrtPriceX64 = BigInt(nextSqrtPriceStr);
  const sqrtPrice = Number(sqrtPriceX64) / 2 ** 64;
  let price = sqrtPrice * sqrtPrice;
  const decimalAdjustment = 10 ** (decimalsA - decimalsB);
  price = price * decimalAdjustment;
  return price;
}

```

![screenshot](assets/meteora-damm.png?raw=true "Screenshot")

## Getting Started

1. **Clone the repository:**
   ```bash
   git clone https://github.com/Shyft-to/solana-defi.git
   cd Meteora/Typescript/stream_meteora_damm_v2_token_price
   ```

2. **Install Dependencies:**

    ```bash
    # For example, if using npm
    npm i
    ```

3. **Run the script:**

    ```bash
    # To run the script
    npm run start
    ```

---
*Note: Please rename the `.env.sample` file to `.env` and input your env details before running the script.*

## Related Links

Shyft gRPC Docs: [https://docs.shyft.to/solana-fast-grpc/grpc-docs]
