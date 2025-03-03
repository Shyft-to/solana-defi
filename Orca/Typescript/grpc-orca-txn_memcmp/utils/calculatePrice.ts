import { BN } from "@project-serum/anchor";
import { MathUtil } from "./maths";
import Decimal from "decimal.js";

export function sqrtPriceX64ToPrice(
    sqrtPriceX64: BN,
    decimalsA: number,
    decimalsB: number,
  ) {
    return MathUtil.fromX64(sqrtPriceX64)
      .pow(2)
      .mul(Decimal.pow(10, decimalsA - decimalsB));
  }
