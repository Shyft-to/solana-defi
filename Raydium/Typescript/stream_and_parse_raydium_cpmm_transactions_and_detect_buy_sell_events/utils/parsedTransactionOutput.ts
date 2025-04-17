export function parsedTransactionOutput(parsedTxn, txn) {
  const SOL_MINT = 'So11111111111111111111111111111111111111112';

  const swapInstruction = parsedTxn.innerInstructions?.find((instruction) => instruction.name === 'swapBaseInput');

  if (!swapInstruction) {
    console.error("swapBaseInput instruction not found in innerInstructions");
    return txn; 
  }

  const signerPubkey = swapInstruction.accounts?.find((account) => account.name === 'payer')?.pubkey;

  if (!signerPubkey) {
    console.error("Payer account not found in swapInstruction accounts");
    return txn;
  }

  const swapAmount = swapInstruction.args?.amountIn;
  const minimumAmount = swapInstruction.args?.minimumAmountOut;

  const determineBuySellEvent = () => {
    const inputMintPubkey = swapInstruction.accounts?.find((account) => account.name === 'inputTokenMint')?.pubkey;
    const outputMintPubkey = swapInstruction.accounts?.find((account) => account.name === 'outputTokenMint')?.pubkey;

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
    amount_out: minimumAmount,
  };

  let rpcTxnWithParsed = {};

  if (txn.version === 0) {
    rpcTxnWithParsed = {
      ...txn,
      meta: {
        ...txn.meta,
        innerInstructions: parsedTxn.innerInstructions,
      },
      transaction: {
        ...txn.transaction,
        message: {
          ...txn.transaction.message,
          compiledInstructions: parsedTxn.compiledInstructions,
        },
      }
    }

  }

  return {rpcTxnWithParsed,transactionEvent};
}
