use {
    crate::{
         TransactionProcessor,
         ParsedConfirmedTransactionWithStatusMeta,
         processor::types::PriceData
    },
    solana_sdk::{
        instruction::AccountMeta,
        message::{v0::LoadedAddresses, VersionedMessage},
    },
    std::vec::Vec,
};
use num_bigint::BigInt;
use num_traits::cast::ToPrimitive;
use crate::processor::models::mapper::event::DecodedEvent;
use num_traits::identities::Zero;

#[derive(Debug, Clone)]
pub struct TransferInfo {
    pub mint: Option<String>,
    pub source: Option<String>,
    pub destination: Option<String>,
    pub decimal: u64,
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


   pub fn meteoradbc_transaction_output(&self, parsed_instruction: ParsedConfirmedTransactionWithStatusMeta) -> Option<PriceData> {
     let meta = &parsed_instruction.meta;
     let tx = &parsed_instruction.transaction;

      let inner_ixs: Vec<_>= tx.message.instructions
        .iter()
        .chain(meta.inner_instructions.iter())
        .collect();
    //println!("Inners {:#?}", inner_ixs);
       let swap_txn = inner_ixs
        .iter()
        .find(|ix| ix.name == "swap")?;
       let swap_event = swap_txn
             .event
             .clone();

         let transfer_check: Vec<_> = inner_ixs
          .iter()
          .filter(|ix| ix.name == "transferChecked")
          .collect();
         let transfers : Vec<_>  = transfer_check
              .into_iter()
              .map(|ix| {
                  let mint = ix.accounts.iter().find(|a| a.name == "mint").map(|a| a.pubkey.to_string().clone());
                  let source = ix.accounts.iter().find(|a| a.name == "source").map(|a| a.pubkey.to_string().clone());
                  let destination = ix.accounts.iter().find(|a| a.name == "destination").map(|a| a.pubkey.to_string().clone());
                let decimal = ix.data.
                get("TransferChecked")
                .and_then(|v| v.get("decimals"))
                .and_then(|v| v.as_u64())  
                .unwrap_or(0);

              TransferInfo {
                  mint,
                  source,
                  destination,
                  decimal,
              }
          })
          .collect();

      let base_mint = swap_txn.accounts.iter().find(|a| a.name == "base_mint")?.pubkey.to_string().clone();
      let quote_mint = swap_txn.accounts.iter().find(|a| a.name == "quote_mint")?.pubkey.to_string().clone();

      let base_decimal = transfers.iter().find(|t| t.mint.as_ref() == Some(&base_mint)).map(|t| t.decimal)?;
      let quote_decimal = transfers.iter().find(|t| t.mint.as_ref() == Some(&quote_mint)).map(|t| t.decimal)?;

       let sqrt_price_str = match &swap_event {
            Some(DecodedEvent::Swap(event)) => event.swap_result.next_sqrt_price.to_string(),
            _ => return None,
         };
      let calculated_price = sqrt_price_x64_to_price(&sqrt_price_str, base_decimal, quote_decimal);
      let formatted_price = format!("{:.13} SOL", calculated_price) ;

    Some(PriceData {
        token_a: base_mint,
        token_b: quote_mint,
        price: formatted_price, 
        })
    }
}
fn sqrt_price_x64_to_price(next_sqrt_price_str: &str, decimals_a: u64, decimals_b: u64) -> f64 {
    let sqrt_price_x64 = BigInt::parse_bytes(next_sqrt_price_str.as_bytes(), 10)
        .unwrap_or_else(|| BigInt::zero());

    let two_pow_64 = BigInt::from(1u128 << 64);
    let sqrt_price = sqrt_price_x64.to_f64().unwrap() / two_pow_64.to_f64().unwrap();

    let mut price = sqrt_price * sqrt_price;

    let decimal_adjustment = 10f64.powi((decimals_a as i32) - (decimals_b as i32));
    price *= decimal_adjustment;
    price
}