export function parseSwapTransactionOutput(parsedInstruction) {
    const parsedEvent = parsedInstruction?.events?.find(e => e.name === 'TradeEvent').data;
    const supply = 1_000_000_000_000_000;

    let swapInstruction = 
        parsedInstruction?.instructions?.pumpAmmIxs?.find(
            instruction => instruction.name === 'buy' || instruction.name === 'sell'
        ) ||
        parsedInstruction?.inner_ixs?.find(
            instruction => instruction.name === 'buy' || instruction.name === 'sell'
        ) ||
        parsedInstruction?.inner_ixs?.pump_amm_inner_ixs?.find(
            instruction => instruction.name === 'buy' || instruction.name === 'sell'
        );

    if (!swapInstruction) return;

    const bonding_curve = swapInstruction.accounts.find(
        (account) => account.name === 'bonding_curve'
    )?.pubkey;

    const virtual_sol_reserves = parsedEvent?.virtual_sol_reserves;
    const virtual_token_reserves = parsedEvent?.virtual_token_reserves;
    const real_sol_reserves = parsedEvent?.real_sol_reserves;
    const real_token_reserves = parsedEvent?.real_token_reserves;
    const mint = parsedEvent?.mint;
    const creator = parsedEvent?.creator;

    const price = calculatePumpFunPrice(
        virtual_sol_reserves,
        virtual_token_reserves
    ); 
    const formattedPrice = price.toFixed(20).replace(/0+$/, '');

    return {
        bonding_curve,
        virtual_sol_reserves,
        virtual_token_reserves,
        real_sol_reserves,
        real_token_reserves,
        mint,
        creator,
        formattedPrice,
        supply,
    };
}

function calculatePumpFunPrice(
    virtualSolReserves: number,
    virtualTokenReserves: number
): number {
    const sol = virtualSolReserves / 1_000_000_000; 
    const tokens = virtualTokenReserves / 1_000_000; 
    return sol / tokens;
}
