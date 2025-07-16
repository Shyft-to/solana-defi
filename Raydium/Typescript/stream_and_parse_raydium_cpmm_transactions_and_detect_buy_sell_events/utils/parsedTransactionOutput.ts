export function parsedTransactionOutput(parsedTxn, txn) {
  const SOL_MINT = 'So11111111111111111111111111111111111111112';

  const swapInstruction = parsedTxn.innerInstructions?.find(
    (instr) => instr.name === 'swapBaseInput' || instr.name === 'swapBaseOutput'
  );

  if (!swapInstruction) return;

  const getAccountPubkey = (name) =>
    swapInstruction.accounts?.find((acc) => acc.name === name)?.pubkey;

  const signerPubkey = getAccountPubkey('payer');
  if (!signerPubkey) {
    console.error("Payer account not found in swapInstruction accounts");
    return txn;
  }

  const expectedAmount = swapInstruction.args.amountIn ?? swapInstruction.args.amountOut;

  const transferInstruction = parsedTxn.innerInstructions.find(
    (x) => x.name === 'transferChecked' && x.args.amount !== expectedAmount
  );

  const amountOut = transferInstruction?.args?.amount ?? 0;

  const isSwapInput = swapInstruction.name === 'swapBaseInput';

  const swapAmount = isSwapInput
    ? swapInstruction.args?.amountIn
    : swapInstruction.args?.maxAmountIn;

  const inputMint = getAccountPubkey('inputTokenMint');
  const outputMint = getAccountPubkey('outputTokenMint');

  let type = 'Unknown';
  let mint = null;

  if (inputMint && outputMint) {
    type = inputMint === SOL_MINT ? 'Buy' : 'Sell';
    mint = inputMint === SOL_MINT ? outputMint : inputMint;
  } else {
    console.error("Input or output token mint not found.");
  }
  const amount_in = type === 'Buy' ? swapAmount : swapAmount;
  const amount_out = type === 'Buy' ? amountOut : amountOut;

  const transactionEvent = {
    name: swapInstruction.name,
    type,
    user: signerPubkey,
    mint,
    amount: amount_in,
    amount_out,
  };

  const rpcTxnWithParsed = txn.version === 0
    ? {
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
        },
      }
    : txn;

  return { rpcTxnWithParsed, transactionEvent };
}
