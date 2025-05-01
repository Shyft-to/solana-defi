export function orca_formatter(parsedInstruction, transaction) {
  const SOL_MINT = 'So11111111111111111111111111111111111111112';
  const usdc = 'EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v'
  let output = {};

  const swapInstruction = parsedInstruction.innerInstructions.find((instruction) => instruction.name === 'swapV2');
  if (!swapInstruction) {
    return;
  }
 const amountIn = swapInstruction.args?.amount;

const determineMarket = () => {
      const transferInstruction = parsedInstruction.innerInstructions.find(
         (instruction) =>
         instruction.name === 'transferChecked' &&
         instruction.args?.amount === amountIn
       );
    if (transferInstruction) {
      const destinationAccount = transferInstruction.accounts.find(
        (account) => account.name === 'tokenMint'
      )?.pubkey;
      
      return destinationAccount === SOL_MINT || destinationAccount === usdc;
    }

    return false;
  };

  const signerPubkey = swapInstruction.accounts.find((account) => account.name === 'tokenAuthority')?.pubkey;

  const sqrtPriceLimit = formatSqrtPrice(swapInstruction.args?.sqrtPriceLimit);

  const determineBuySellEvent = () => {
    const determine = determineMarket();
    const inputMintPubkey = swapInstruction.accounts.find((account) => account.name === 'tokenMintA')?.pubkey;
    const outputMintPubkey = swapInstruction.accounts.find((account) => account.name === 'tokenMintB')?.pubkey;

    if (!inputMintPubkey || !outputMintPubkey) {
      console.error("Input or output mint not found in swap accounts");
      return { type: "Unknown", mint: null };
    }

    const mint = inputMintPubkey === SOL_MINT ? outputMintPubkey : inputMintPubkey;
    const eventType = determine ? "Buy" : "Sell";

    return { type: eventType, mint };
  };

  const buySellEvent = determineBuySellEvent();

  const transactionEvent = {
    type: buySellEvent.type,
    user: signerPubkey,
    mint: buySellEvent.mint,
    amount: amountIn,
    sqrtPrice: sqrtPriceLimit,
  };

  output = {
    ...transaction,
    meta: {
      ...transaction.meta,
      innerInstructions: parsedInstruction.innerInstructions,
    },
    transaction: {
      ...transaction.transaction,
      message: {
        ...transaction.transaction.message,
        compiledInstructions: parsedInstruction.compiledInstructions,
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

