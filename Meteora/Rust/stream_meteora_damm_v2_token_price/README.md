<a id="readme-top"></a>

# Stream token price update on Meteora DAMM v2 Token Price with Rust

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

```rust
fn sqrt_price_x64_to_price(next_sqrt_price_str: &str, decimals_a: u64, decimals_b: u64) -> f64 {
    let sqrt_price_x64 = BigInt::parse_bytes(next_sqrt_price_str.as_bytes(), 10)
        .unwrap_or_else(|| BigInt::zero());

    let two_pow_64 = BigInt::from(1u128 << 64);
    let sqrt_price = sqrt_price_x64.to_f64().unwrap() / two_pow_64.to_f64().unwrap();

    let mut price = sqrt_price * sqrt_price;

    let decimal_adjustment = 10f64.powi((decimals_a as i32) - (decimals_b as i32));
    price *= decimal_adjustment;
    price
}


```
![screenshot](assets/meteora-damm-price.png?raw=true "Screenshot")
```
$ cargo run -- --endpoint <endpoint> --x-token <token>
```

## Related Links

Shyft gRPC Docs: [https://docs.shyft.to/solana-fast-grpc/grpc-docs]
