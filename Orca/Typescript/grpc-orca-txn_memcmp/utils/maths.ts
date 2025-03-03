import { BN } from "@project-serum/anchor";
import Decimal from "decimal.js";

export class MathUtil {
    public static toX64_BN(num: BN): BN {
        return num.mul(new BN(2).pow(new BN(64)));
      }
    
      public static toX64_Decimal(num: Decimal): Decimal {
        return num.mul(Decimal.pow(2, 64));
      }
    
      public static toX64(num: Decimal): BN {
        return new BN(num.mul(Decimal.pow(2, 64)).floor().toFixed());
      }
    
      public static fromX64(num: BN): Decimal {
        return new Decimal(num.toString()).mul(Decimal.pow(2, -64));
      }
    
      public static fromX64_Decimal(num: Decimal): Decimal {
        return num.mul(Decimal.pow(2, -64));
      }
    
      public static fromX64_BN(num: BN): BN {
        return num.div(new BN(2).pow(new BN(64)));
      }
    
}