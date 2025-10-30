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

    pub fn parsed_pump_txn(
        &self,
        original: ParsedConfirmedTransactionWithStatusMeta,
    ) -> Option<TransactionEvent> {
        let meta = &original.meta;
        let tx = &original.transaction;

        let amm_instruction = tx
            .message
            .instructions
            .iter()
            .chain(meta.inner_instructions.iter())
            .find(|instr| {
                instr.name.to_lowercase() == "sell" || instr.name.to_lowercase() == "buy"
            })?;

        let in_amount = match amm_instruction.name.as_str() {
            "buy" => amm_instruction.data.get("Buy")?.get("amount")?.as_u64(),
            "sell" => amm_instruction.data.get("Sell")?.get("amount")?.as_u64(),
            _ => Some(0),
        }?;

        let sol_amount = meta
            .inner_instructions
            .iter()
            .chain(tx.message.instructions.iter())
            .filter_map(|instr| {
                match &instr.event {
                    Some(DecodedEvent::TradeEvent(event)) => Some(event.sol_amount),
                    _ => None,
                }
            })
            .next()
            .unwrap_or(0);

        let amount_in = match amm_instruction.name.as_str() {
            "buy" => sol_amount,
            "sell" => in_amount,
            _ => 0,
        };

        let amount_out = match amm_instruction.name.as_str() {
            "buy" => in_amount,
            "sell" => sol_amount,
            _ => 0,
        };

        let signer_pubkey = amm_instruction
            .accounts
            .iter()
            .find(|acc| acc.name == "user")
            .map(|acc| acc.pubkey.to_string());

        let input_mint = amm_instruction
            .accounts
            .iter()
            .find(|acc| acc.name == "mint")
            .map(|acc| acc.pubkey.to_string())?;

        let bonding_curve = amm_instruction
            .accounts
            .iter()
            .find(|acc| acc.name == "bonding_curve")
            .map(|acc| acc.pubkey.to_string())?;

        let output =  TransactionEvent {
                event_type: Some(amm_instruction.name.to_string()),
                user: signer_pubkey,
                mint: Some(input_mint),
                bonding_curve: Some(bonding_curve),
                amount_in: Some(amount_in),
                amount_out: Some(amount_out),
        };

        Some(output)
    }
}