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

   
pub fn pump_amm_formatter(
    &self,
    original: ParsedConfirmedTransactionWithStatusMeta,
) -> Option<ParsedConfirmedTransactionWithStatusMeta> {
    let meta = &original.meta;
    let tx = &original.transaction;

    tx.message.instructions
        .iter()
        .chain(meta.inner_instructions.iter())
        .find(|instr| instr.name == "migrate")?; 

    Some(ParsedConfirmedTransactionWithStatusMeta {
        slot: original.slot,
        transaction: ParsedTransaction {
            signatures: tx.signatures.clone(),
            message: ParsedMessage {
                header: tx.message.header.clone(),
                account_keys: tx.message.account_keys.clone(),
                recent_blockhash: tx.message.recent_blockhash.clone(),
                instructions: tx.message.instructions.clone(),
                address_table_lookups: tx.message.address_table_lookups.clone(),
            },
        },
        meta: ParsedTransactionStatusMeta {
            status: meta.status.clone(),
            fee: meta.fee,
            pre_balances: meta.pre_balances.clone(),
            post_balances: meta.post_balances.clone(),
            inner_instructions: meta.inner_instructions.clone(),
            log_messages: meta.log_messages.clone(),
            pre_token_balances: meta.pre_token_balances.clone(),
            post_token_balances: meta.post_token_balances.clone(),
            rewards: meta.rewards.clone(),
            loaded_addresses: meta.loaded_addresses.clone(),
            return_data: meta.return_data.clone(),
            compute_units_consumed: meta.compute_units_consumed,
        },
        block_time: original.block_time,
    })
}
}
