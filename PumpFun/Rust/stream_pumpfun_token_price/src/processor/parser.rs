use {
    crate::{
        processor::models::mapper::event::DecodedEvent,
        ParsedConfirmedTransactionWithStatusMeta,
        ParsedEventTransaction,
        ParsedTransaction,
        ParsedMessage,
        ParsedTransactionStatusMeta,
        TransactionEvent,
        TransactionProcessor,
    },
    solana_sdk::{
        instruction::AccountMeta,
        message::{v0::LoadedAddresses, VersionedMessage},
    },
    std::vec::Vec,
};
use crate::processor::core::PumpAmmSwapOutput;

impl TransactionProcessor {
    pub fn parse_transaction_accounts(
        &self,
        message: &VersionedMessage,
        loaded_addresses: LoadedAddresses,
    ) -> Vec<AccountMeta> {
        let accounts = message.static_account_keys();
        let readonly_signed_accounts_count = message.header().num_readonly_signed_accounts as usize;
        let readonly_unsigned_accounts_count = message.header().num_readonly_unsigned_accounts as usize;
        let required_signatures_accounts_count = message.header().num_required_signatures as usize;
        let total_accounts = accounts.len();

        let mut parsed_accounts: Vec<AccountMeta> = accounts
            .iter()
            .enumerate()
            .map(|(index, pubkey)| {
                let is_writable = index
                    < required_signatures_accounts_count - readonly_signed_accounts_count
                    || (index >= required_signatures_accounts_count
                        && index < total_accounts - readonly_unsigned_accounts_count);

                AccountMeta {
                    pubkey: *pubkey,
                    is_signer: index < required_signatures_accounts_count,
                    is_writable,
                }
            })
            .collect();

        parsed_accounts.extend(loaded_addresses.writable.into_iter().map(|pubkey| AccountMeta {
            pubkey,
            is_signer: false,
            is_writable: true,
        }));

        parsed_accounts.extend(loaded_addresses.readonly.into_iter().map(|pubkey| AccountMeta {
            pubkey,
            is_signer: false,
            is_writable: false,
        }));

        parsed_accounts
    }

   
pub fn parsed_pump_txn(
    &self,
    original: ParsedConfirmedTransactionWithStatusMeta,
) -> Option<PumpAmmSwapOutput> {
    const SOL_MINT: &str = "So11111111111111111111111111111111111111112";
    let meta = &original.meta;
    let tx = &original.transaction;
    let decimal = meta.pre_token_balances.as_ref()?
        .iter()
        .find(|balance| balance.mint != SOL_MINT)
        .map(|balance| balance.ui_token_amount.decimals)
        .unwrap_or(6); 
    let amm_instruction = tx
        .message
        .instructions
        .iter()
        .chain(meta.inner_instructions.iter())
        .find(|instr| {
            instr.name.to_lowercase() == "sell" || instr.name.to_lowercase() == "buy"
        })?;

    let (virtual_sol_reserve, virtual_token_reserve, real_sol_reserve, real_token_reserve, creator) = meta
            .inner_instructions
            .iter()
            .chain(tx.message.instructions.iter())
            .filter_map(|instr| {
                match &instr.event {
                    Some(DecodedEvent::TradeEvent(event)) => Some((
                    event.virtual_sol_reserves,
                    event.virtual_token_reserves,
                    event.real_sol_reserves,
                    event.real_token_reserves,
                    event.creator.to_string(),
                )),
                _ => None,
            }
        })
        .next()
        .unwrap_or((0, 0, 0, 0, String::default()));

    // Extract signer
    let mint = amm_instruction
        .accounts
        .iter()
        .find(|acc| acc.name == "mint")
        .map(|acc| acc.pubkey.to_string())?;

    let bonding_curve = amm_instruction
        .accounts
        .iter()
        .find(|acc| acc.name == "bonding_curve")
        .map(|acc| acc.pubkey.to_string())?;

     let price = calculate_pump_price(
            virtual_sol_reserve,
            virtual_token_reserve,
            decimal
        ).to_string() + " SOL";
    let output = PumpAmmSwapOutput  {
                     bonding_curve: bonding_curve,
                     virtual_sol_reserves: virtual_sol_reserve,
                     virtual_token_reserves: virtual_token_reserve,
                     real_sol_reserves : real_sol_reserve,
                     real_token_reserves : real_token_reserve,
                     mint : mint,
                     creator : creator,
                     price : price 
                  };
    Some(output)
}
}
fn calculate_pump_price(
    virtual_sol_reserves: u64,
    virtual_token_reserve: u64,
    decimal: u8,
) -> f64 {
    let base = virtual_sol_reserves as f64 / 1_000_000_000f64;
    let quote = virtual_token_reserve as f64 / 10f64.powi(decimal as i32);
    base / quote
}