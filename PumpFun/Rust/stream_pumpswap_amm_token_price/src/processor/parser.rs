use {
    crate::{
        processor::models::mapper::event::DecodedEvent,
        ParsedConfirmedTransactionWithStatusMeta,
        ParsedEventTransaction,
        ParsedTransaction,
        ParsedMessage,
        ParsedTransactionStatusMeta,
        TransactionProcessor,
    },
    solana_sdk::{
        instruction::AccountMeta,
        message::{v0::LoadedAddresses, VersionedMessage},
    },
    std::vec::Vec,
};
use crate::DecodedInstruction;
#[derive(Debug, Clone)]
pub struct PumpAmmSwapOutput {
    pub base_mint: String,
    pub quote_mint: String,
    pub pool_base_token_reserve: String,
    pub pool_quote_token_reserve: String,
    pub price: String
}

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
   
     pub fn parse_swap_transaction_output(
        &self,
        txn: ParsedConfirmedTransactionWithStatusMeta,
        inner_instructions: Vec<DecodedInstruction>,
        compiled_instructions: Vec<DecodedInstruction>,
    ) -> Option<PumpAmmSwapOutput> {
        const SOL_MINT: &str = "So11111111111111111111111111111111111111112";
        let mut price: f64;
        let mut base_reserve_pool : String;
        let mut quote_reserve_pool : String;
        let decimal = txn.meta.pre_token_balances.as_ref()?
            .iter()
            .find(|balance| balance.mint != SOL_MINT)
            .map(|balance| balance.ui_token_amount.decimals)
            .unwrap_or(9); 
        let parsed_instruction = inner_instructions
            .iter()
            .chain(compiled_instructions.iter())
            .find(|ix| ix.name == "Buy" || ix.name == "Sell")?;
        let parsed_event = inner_instructions
            .get(0)
            .and_then(|ix| ix.event.clone())
            .or_else(|| compiled_instructions.get(0).and_then(|ix| ix.event.clone()))?;
        let base_mint = parsed_instruction.accounts.iter()
            .find(|acc| acc.name == "base_mint")
            .map(|acc| acc.pubkey.to_string())?;
        let quote_mint = parsed_instruction.accounts.iter()
            .find(|acc| acc.name == "quote_mint")
            .map(|acc| acc.pubkey.to_string())?;
        let (base_reserve, quote_reserve) = match parsed_event {
            DecodedEvent::BuyEvent(event) => (
                event.pool_base_token_reserves,
                event.pool_quote_token_reserves
            ),
            DecodedEvent::SellEvent(event) => (
                event.pool_base_token_reserves,
                event.pool_quote_token_reserves
            ),
            _ => return None, 
        };
        if base_mint == SOL_MINT {
            base_reserve_pool = (base_reserve as f64 / 1_000_000_000f64).to_string() + " SOL";
            quote_reserve_pool = (quote_reserve as f64 / 10f64.powi(decimal as i32)).to_string();
            price = calculate_pump_amm_price(
                base_reserve,
                quote_reserve,
                decimal
            );
        }else{
            base_reserve_pool = (base_reserve as f64 / 10f64.powi(decimal as i32)).to_string();
            quote_reserve_pool = (quote_reserve as f64 / 1_000_000_000f64).to_string() + " SOL";
            price = calculate_pump_amm_price(
                quote_reserve,
                base_reserve,
                decimal
            );
         }
        Some(PumpAmmSwapOutput {
            base_mint,
            quote_mint,
            pool_base_token_reserve: base_reserve_pool,
            pool_quote_token_reserve: quote_reserve_pool,
            price: price.to_string() + " SOL",
        })
     }
    }
fn calculate_pump_amm_price(
    pool_base_reserve: u64,
    pool_quote_reserve: u64,
    decimal: u8,
    ) -> f64 {
    let base = pool_base_reserve as f64 / 1_000_000_000f64;
    let quote = pool_quote_reserve as f64 / 10f64.powi(decimal as i32);
    base / quote
 } 
