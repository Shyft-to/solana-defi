use {
    crate::{
         TransactionProcessor,
    },
    solana_sdk::{
        instruction::AccountMeta,
        message::{v0::LoadedAddresses, VersionedMessage},
    },
    std::vec::Vec,
};
use crate::ParsedConfirmedTransactionWithStatusMeta;
use crate::processor::models::mapper::event::DecodedEvent;
use crate::processor::types::TransactionEvent;
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
  pub fn parsed_meteora_damm_txn(
        &self,
        original: ParsedConfirmedTransactionWithStatusMeta,
    ) -> Option<TransactionEvent> {
     let meta = &original.meta;
     let tx = &original.transaction;

     let launchpad_instruction = tx.message.instructions
     .iter()
     .chain(meta.inner_instructions.iter())
     .find(|instr| instr.name == "swap")?;
     
   
     let base_mint = launchpad_instruction.accounts.iter().find(|a| a.name == "token_a_mint")?.pubkey.to_string().clone();
     let quote_mint = launchpad_instruction.accounts.iter().find(|a| a.name == "token_b_mint")?.pubkey.to_string().clone();
     let signer_pubkey = launchpad_instruction
                  .accounts
                  .iter()
                  .find(|acc| acc.name == "payer")
                  .map(|acc| acc.pubkey.to_string());
                    
     let traded_event = launchpad_instruction.event.as_ref()?;
     
     let (amount_in,amount_out, trade_direction) = match traded_event {
            DecodedEvent::Swap(event) => (
             event.params.amount_in,
             event.swap_result.output_amount,
             event.trade_direction.clone(),
            ),
            _ => return None,
     };
    
     let buy_sell_determiner = if trade_direction == 1 {
        "Buy"
     }else {
        "Sell"
     };

    let output = TransactionEvent {
        event_type: buy_sell_determiner.to_string(),
        user: signer_pubkey,
        mint_a: Some(base_mint),
        mint_b: Some(quote_mint),
        amount_in: Some(amount_in),
        amount_out: Some(amount_out),
    };
    Some(output)
  }

}
