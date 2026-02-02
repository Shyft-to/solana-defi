use {
    crate::{
        processor::models::mapper::event::DecodedEvent,
        ParsedConfirmedTransactionWithStatusMeta,
        ParsedEventTransaction,
        ParsedTransaction,
        ParsedMessage,
        ParsedTransactionStatusMeta,
        TransactionProcessor,
        DecodedInstruction,
    },
    solana_sdk::{
        instruction::AccountMeta,
        message::{v0::LoadedAddresses, VersionedMessage},
    },
    std::vec::Vec,
};
use crate::processor::types::PumpAmmCreatePoolEvent;


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
    ) -> Option<PumpAmmCreatePoolEvent> {
     let meta = &original.meta;
     let tx = &original.transaction;

     let instr = tx.message.instructions
        .iter()
        .chain(meta.inner_instructions.iter())
        .find(|instr|instr.name == "CreatePool")?;

     let data = instr.data.get("CreatePool")?;

     let quote_amount_in = data.get("quote_amount_in")?.as_u64()?;
     let base_amount_in = data.get("base_amount_in")?.as_u64()?;

     let pool = instr.accounts.iter().find(|a| a.name == "pool")?.pubkey.to_string();
     let creator = instr.accounts.iter().find(|a| a.name == "creator")?.pubkey.to_string();
     let base_mint = instr.accounts.iter().find(|a| a.name == "base_mint")?.pubkey.to_string();
     let quote_mint = instr.accounts.iter().find(|a| a.name == "quote_mint")?.pubkey.to_string();

     Some(PumpAmmCreatePoolEvent {
        pool,
        creator,
        base_mint,
        quote_mint,
        base_amount: base_amount_in,
        quote_amount: quote_amount_in,
     })
    }


}