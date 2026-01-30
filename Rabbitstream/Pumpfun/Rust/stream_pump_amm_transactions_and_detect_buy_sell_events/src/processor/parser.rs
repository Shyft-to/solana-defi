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

    pub fn parsed_pump_amm_txn(
        &self,
        original: ParsedConfirmedTransactionWithStatusMeta,
    ) -> Option<TransactionEvent> {
        let meta = &original.meta;
    let tx = &original.transaction;

    let amm_instruction = tx.message.instructions
    .iter()
    .chain(meta.inner_instructions.iter())
    .find(|instr| instr.name == "Sell" || instr.name == "Buy" || instr.name == "BuyExactQuoteIn")?;

   let amount_in_raw = match amm_instruction.name.as_str() {
     "Buy" => amm_instruction
        .data
        .get("Buy")?
        .get("max_quote_amount_in")?
        .as_u64(),
     "Sell" => amm_instruction
        .data
        .get("Sell")?
        .get("base_amount_in")?
        .as_u64(),
     "BuyExactQuoteIn" => amm_instruction
        .data
        .get("BuyExactQuoteIn")?
        .get("spendable_quote_in")?    
        .as_u64(),
    _ => None,
   };
    let amount_in = amount_in_raw.unwrap_or(0); 
    

    let signer_pubkey = amm_instruction
        .accounts
        .iter()
        .find(|acc| acc.name == "user")
        .map(|acc| acc.pubkey.to_string());

    let base_mint_pubkey = amm_instruction
        .accounts
        .iter()
        .find(|acc| acc.name == "base_mint")
        .map(|acc| acc.pubkey.to_string());
    
    let quote_mint_pubkey = amm_instruction
        .accounts
        .iter()
        .find(|acc| acc.name == "quote_mint") 
        .map(|acc| acc.pubkey.to_string());   
    
    let pool_pubkey = amm_instruction
        .accounts
        .iter()
        .find(|acc| acc.name == "pool")    
        .map(|acc| acc.pubkey.to_string());

    let output_mint = amm_instruction
        .accounts
        .iter()
        .find(|acc| acc.name == "quote_mint")
        .map(|acc| acc.pubkey.to_string());

    let event = TransactionEvent {
        event_type: amm_instruction.name.clone(),
        user: signer_pubkey,
        pool: pool_pubkey,
        base_mint: base_mint_pubkey,
        quote_mint: quote_mint_pubkey,
        amount_in: Some(amount_in),
    };


    Some(event)
 }
}