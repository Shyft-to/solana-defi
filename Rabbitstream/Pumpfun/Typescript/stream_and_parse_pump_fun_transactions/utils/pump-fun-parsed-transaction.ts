import pumpFunAmmIdl from "../idls/pump_0.1.0.json";
import { PublicKey } from "@solana/web3.js";

const BUY_ACCOUNTS =
  pumpFunAmmIdl.instructions.find((ix: any) => ix.name === "buy")?.accounts || [];

const BUY_EXACT_SOL_IN_ACCOUNTS =
  pumpFunAmmIdl.instructions.find((ix: any) => ix.name === "buy_exact_sol_in")?.accounts || [];

const BUY_DISCRIMINATOR = Buffer.from([102, 6, 61, 18, 1, 218, 235, 234]);
const BUY_EXACT_SOL_IN_DISCRIMINATOR = Buffer.from([56, 252, 116, 8, 158, 223, 205, 95]);

export function pumpFunParsedTransaction(parsedInstruction: any, txn: any) {
  const updatedCompiled = parsedInstruction.inner_ixs.map((ix: any) => {
    if (ix.name === "unknown") {
      if (isPumpBuyInstruction(ix, ix.programId)) return decodePumpBuy(ix);
      if (isPumpBuyExactSolInInstruction(ix, ix.programId)) return decodePumpBuyExactSolIn(ix);
    }
    return ix;
  });

  return {
    ...txn,
    meta: { ...txn.meta },
    transaction: {
      ...txn.transaction,
      message: {
        ...txn.transaction.message,
        instructions: parsedInstruction.instructions,
        compiledInstructions: updatedCompiled,
      },
    },
  };
}

export function isPumpBuyInstruction(ix: any, pumpfun_program: PublicKey): boolean {
  if (!ix.programId?.equals?.(pumpfun_program)) return false;
  if (!ix.args?.unknown) return false;

  const data = Buffer.from(ix.args.unknown, "base64");
  const discriminator = data.subarray(0, 8);
  return discriminator.equals(BUY_DISCRIMINATOR);
}
export function decodePumpBuy(ix: any) {
  const data = Buffer.from(ix.args.unknown, "base64");
  const MIN_LENGTH = 8 + 8 + 8;

  if (data.length < MIN_LENGTH) throw new Error(`Invalid BUY instruction length: ${data.length}`);

  const amount = data.readBigUInt64LE(8);
  const maxSolCost = data.readBigUInt64LE(16);

  const namedAccounts = ix.accounts.map((acc: any, index: number) => ({
    name: BUY_ACCOUNTS[index]?.name || `unknown_${index}`,
    isSigner: acc.isSigner,
    isWritable: acc.isWritable,
    pubkey: acc.pubkey,
  }));

  return {
    name: "buy",
    programId: ix.programId.toBase58?.() || ix.programId,
    accounts: namedAccounts,
    args: {
      amount: amount.toString(),
      maxSolCost: maxSolCost.toString(),
    },
  };
}
export function isPumpBuyExactSolInInstruction(ix: any, pumpfun_program: PublicKey): boolean {
  if (!ix.programId?.equals?.(pumpfun_program)) return false;
  if (!ix.args?.unknown) return false;

  const data = Buffer.from(ix.args.unknown, "base64");
  const discriminator = data.subarray(0, 8);
  return discriminator.equals(BUY_EXACT_SOL_IN_DISCRIMINATOR);
}

export function decodePumpBuyExactSolIn(ix: any) {
  const data = Buffer.from(ix.args.unknown, "base64");
  const MIN_LENGTH = 8 + 8 + 8;

  if (data.length < MIN_LENGTH)
    throw new Error(`Invalid BUY EXACT SOL IN instruction length: ${data.length}`);

  const spendableSolIn = data.readBigUInt64LE(8);
  const minTokensOut = data.readBigUInt64LE(16);
  const namedAccounts = ix.accounts.map((acc: any, index: number) => ({
    name: BUY_EXACT_SOL_IN_ACCOUNTS[index]?.name || `unknown_${index}`,
    isSigner: acc.isSigner,
    isWritable: acc.isWritable,
    pubkey: acc.pubkey,
  }));

  return {
    name: "buy_exact_sol_in",
    programId: ix.programId.toBase58?.() || ix.programId,
    accounts: namedAccounts,
    args: {
      spendable_sol_in: spendableSolIn.toString(),
      min_tokens_out: minTokensOut.toString(),
    },
  };
}
