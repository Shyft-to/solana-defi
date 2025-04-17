export function parsedTransactionOutput(parsedInstruction, transaction) {
  const SOL_MINT = 'So11111111111111111111111111111111111111112';
  let output = {};

  const swapInstruction = parsedInstruction.instructions.find((instruction) => instruction.name === 'swapV2');
  if (!swapInstruction) {
    console.error("swapV2 instruction not found");
    return transaction;
  }

  const signerPubkey = swapInstruction.accounts.find((account) => account.name === 'payer')?.pubkey;

  const swapAmount = swapInstruction.args?.amount;
  const sqrtPriceLimit = formatSqrtPrice(swapInstruction.args?.sqrtPriceLimitX64);

  const determineBuySellEvent = () => {
    const inputMintPubkey = swapInstruction.accounts.find((account) => account.name === 'inputVaultMint')?.pubkey;
    const outputMintPubkey = swapInstruction.accounts.find((account) => account.name === 'outputVaultMint')?.pubkey;

    if (!inputMintPubkey || !outputMintPubkey) {
      console.error("Input or output mint not found in swap accounts");
      return { type: "Unknown", mint: null };
    }

    const mint = inputMintPubkey === SOL_MINT ? outputMintPubkey : inputMintPubkey;
    const eventType = inputMintPubkey === SOL_MINT ? "Buy" : "Sell";

    return { type: eventType, mint };
  };

  const buySellEvent = determineBuySellEvent();

  const transactionEvent = {
    type: buySellEvent.type,
    user: signerPubkey,
    mint: buySellEvent.mint,
    amount: swapAmount,
    sqrtPrice: sqrtPriceLimit,
  };

  output = {
    ...transaction,
    meta: {
      ...transaction.meta,
      innerInstructions: parsedInstruction.inner_ixs,
    },
    transaction: {
      ...transaction.transaction,
      message: {
        ...transaction.transaction.message,
        compiledInstructions: parsedInstruction.instructions,
      },
    }
  }
  

  return {output, transactionEvent};
}
function formatSqrtPrice(sqrtPrice: number | bigint): string {
  if (typeof sqrtPrice === "bigint") {
    return Number(sqrtPrice).toExponential(2); 
  } else if (typeof sqrtPrice === "number") {
    return sqrtPrice.toExponential(2);
  } else {
    throw new Error("Invalid type for sqrtPrice. Expected number or bigint.");
  }
}
