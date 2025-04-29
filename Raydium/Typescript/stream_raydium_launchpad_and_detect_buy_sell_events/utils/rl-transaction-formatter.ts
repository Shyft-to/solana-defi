export function parsedTransactionOutput(parsedInstruction, transaction) {
  const SOL_MINT = 'So11111111111111111111111111111111111111112';
  let output = {};

  const supportedInstructions = ['buy_exact_in', 'buy_exact_out', 'sell_exact_in', 'sell_exact_out'];
  const swapInstruction = parsedInstruction.instructions.find((instruction) =>
    supportedInstructions.includes(instruction.name)
  );

  if (!swapInstruction) {
    console.error("Supported instruction not found");
    return transaction;
  }

  const signerPubkey = swapInstruction.accounts.find((account) => account.name === 'payer')?.pubkey;

  const swapAmount = swapInstruction.args?.amount_in || swapInstruction.args?.amount_out;
  const slippageLimit = swapInstruction.args?.minimum_amount_out || swapInstruction.args?.maximum_amount_in;

  const determineBuySellEvent = () => {
    const baseMintPubkey = swapInstruction.accounts.find((account) => account.name === 'base_token_mint')?.pubkey;
    const quoteMintPubkey = swapInstruction.accounts.find((account) => account.name === 'quote_token_mint')?.pubkey;

    if (!baseMintPubkey || !quoteMintPubkey) {
      console.error("Base or quote mint not found in swap accounts");
      return { type: "Unknown", mint: null };
    }

    let eventType = "Unknown";
    let mint = null;

    if (swapInstruction.name.startsWith('buy')) {
      eventType = "Buy";
      mint = baseMintPubkey;
    } else if (swapInstruction.name.startsWith('sell')) {
      eventType = "Sell";
      mint = quoteMintPubkey;
    }

    return { type: eventType, mint };
  };

  const buySellEvent = determineBuySellEvent();

  const transactionEvent = {
    type: buySellEvent.type,
    user: signerPubkey,
    mint: buySellEvent.mint,
    amount: swapAmount,
    minimumAmount : slippageLimit
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
  };

  return { output, transactionEvent };
}


