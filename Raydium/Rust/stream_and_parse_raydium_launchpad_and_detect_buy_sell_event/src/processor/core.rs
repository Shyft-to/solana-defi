use { 
    anyhow::Context,
    super::types::*,
    yellowstone_grpc_proto::prelude::{
        SubscribeUpdateTransaction, 
    },
    crate::processor::models::mapper::event::{DecodedEvent, decode_event_data},
    solana_account_decoder_client_types::token::UiTokenAmount, 
    solana_sdk::{instruction::{CompiledInstruction, AccountMeta, Instruction}, signature::Signature},
    std::{
        fs, str::FromStr, time::{SystemTime, UNIX_EPOCH}
    },
    solana_transaction_status::{
        ConfirmedTransactionWithStatusMeta, TransactionTokenBalance, TransactionWithStatusMeta, VersionedTransactionWithStatusMeta
    },
    solana_sdk::{pubkey::Pubkey, hash::Hash},
};
use crate::processor::models::mapper::instruction::Idl;
use crate::RAYDIUM_LAUNCHPAD_PROGRAM_ID;
use crate::ParsedEventTransaction;
use crate::TOKEN_PROGRAM_ID;
use spl_token::instruction::TokenInstruction;
use crate::processor::models::mapper::event;
use crate::ParsedConfirmedTransactionWithStatusMeta;
pub struct TransactionProcessor {
    pub raydium_launchpad_idl: Idl,
    pub token_idl: Idl,
    pub raydium_launchpad_program_id: Pubkey,
    pub token_program_id: Pubkey,
}

impl TransactionProcessor {
    pub fn new() -> anyhow::Result<Self> {
        let token_idl_json = fs::read_to_string("idls/token_program_idl.json")
            .context("Unable to read Token IDL JSON file")?;
        let raydium_launchpad_idl = fs::read_to_string("idls/raydium_launchpad.json")
            .context("Unable to read Raydium IDL JSON file")?;

        Ok(Self {
            raydium_launchpad_idl: serde_json::from_str(&raydium_launchpad_idl)?,
            token_idl: serde_json::from_str(&token_idl_json)?,
            raydium_launchpad_program_id: Pubkey::from_str(RAYDIUM_LAUNCHPAD_PROGRAM_ID)?,
            token_program_id: Pubkey::from_str(TOKEN_PROGRAM_ID)?,
        })
    }

    pub fn process_transaction_update(
    &self,
    update: SubscribeUpdateTransaction,
    ) -> anyhow::Result<Option<ParsedEventTransaction>> {
      let slot = update.slot;
      let block_time = SystemTime::now()
        .duration_since(UNIX_EPOCH)?
        .as_secs() as i64;

      let txn_info: Option<yellowstone_grpc_proto::prelude::SubscribeUpdateTransactionInfo> = update.transaction;

        if let Some(txn_info) = txn_info {
            let signature = Self::parse_signature(&txn_info.signature)
            .context("invalid signature format")?;

            let transaction = txn_info.transaction.context("transaction data empty")?;
            let raw_message = transaction.message.context("message empty")?;
            let meta = txn_info.meta.context("Meta empty")?;
            let recent_blockhash = Self::parse_blockhash(&raw_message.recent_blockhash)?;

            let confirmed_txn = Self::build_confirmed_transaction(
                slot,
                signature,
                raw_message,
                meta,
                block_time,
            )?;
            let decoded_event = self.extract_decoded_event(&confirmed_txn);
            let instructions = self.extract_all_instructions(&confirmed_txn)?;

            let (decoded_compiled, decoded_inner) = self.decode_instructions(
                &instructions.compiled,
                &instructions.inner,
                &decoded_event,
             )?;           

            let parsed_txn = Self::build_parsed_transaction(
                slot,
                &confirmed_txn,
                decoded_compiled,
                decoded_inner,
                block_time,
            )?;

            Ok(self.parsed_raydium_launchpad_txn(parsed_txn))
        } else {
            Ok(None)
     }
}


    pub fn parse_signature(signature: &[u8]) -> anyhow::Result<Signature> {
        if signature.len() != 64 {
            anyhow::bail!("Signature must be exactly 64 bytes");
        }
        let raw_signature_array: [u8; 64] = signature.try_into()?;
        Ok(Signature::from(raw_signature_array))
    }

    pub fn parse_blockhash(blockhash: &[u8]) -> anyhow::Result<Hash> {
        Ok(Hash::new_from_array(
            blockhash.try_into().context("Failed to convert blockhash to [u8; 32]")?,
        ))
    }

    pub fn extract_decoded_event(
        &self,
        confirmed_txn: &ConfirmedTransactionWithStatusMeta,
    ) -> Option<DecodedEvent> {
        if let TransactionWithStatusMeta::Complete(versioned_meta) = &confirmed_txn.tx_with_meta {
            if let Some(inner_instructions) = &versioned_meta.meta.inner_instructions {
                if let data_msg = event::extract_inner_data(inner_instructions) {
                  for data in data_msg.iter() {
                    match event::decode_event_data(data) {
                          Ok(decoded_event) => {return Some(decoded_event)},
                         Err(e) => { },
                        }
                    }
                 }
                }
            }
        
        None
    }

    pub fn extract_all_instructions(
        &self,
        confirmed_txn: &ConfirmedTransactionWithStatusMeta,
    ) -> anyhow::Result<ExtractedInstructions> {
        match &confirmed_txn.tx_with_meta {
            TransactionWithStatusMeta::Complete(versioned_tx_with_meta) => {
                Ok(ExtractedInstructions {
                    compiled: self.flatten_compiled_instructions(versioned_tx_with_meta),
                    inner: self.flatten_inner_instructions(versioned_tx_with_meta),
                })
            }
            TransactionWithStatusMeta::MissingMetadata(_) => Ok(ExtractedInstructions {
                compiled: vec![],
                inner: vec![],
            }),
        }
    }
    
    pub fn get_instruction_name_with_typename(&self,instruction: &TokenInstruction) -> String {
    let debug_string = format!("{:?}", instruction);
    if let Some(first_brace) = debug_string.find(" {") {
        let name = &debug_string[..first_brace]; // Extract name before `{`
        self.to_camel_case(name)
    } else {
        self.to_camel_case(&debug_string) // Directly convert unit variant names
    }
    }

    pub fn flatten_compiled_instructions(
        &self,
        transaction_with_meta: &VersionedTransactionWithStatusMeta,
    ) -> Vec<TransactionInstructionWithParent> {
        let mut compiled_result = Vec::new();
        let transaction = &transaction_with_meta.transaction;
        let ci_ixs = transaction.message.instructions();
        let parsed_accounts = self.parse_transaction_accounts(
            &transaction.message,
            transaction_with_meta.meta.loaded_addresses.clone(),
        );

        for ci_ix in ci_ixs {
            compiled_result.push(TransactionInstructionWithParent {
                instruction: self.compiled_instruction_to_instruction(&ci_ix, parsed_accounts.clone()),
                parent_program_id: None,
            });
        }

        compiled_result
    }

    pub fn flatten_inner_instructions(
        &self,
        transaction_with_meta: &VersionedTransactionWithStatusMeta,
    ) -> Vec<TransactionInstructionWithParent> {
        let mut inner_result = Vec::new();
        let transaction = &transaction_with_meta.transaction;
        let ci_ixs = transaction.message.instructions();
        let parsed_accounts = self.parse_transaction_accounts(
            &transaction.message,
            transaction_with_meta.meta.loaded_addresses.clone(),
        );

        if let Some(inner_ixs) = &transaction_with_meta.meta.inner_instructions {
            let mut ordered_cii = inner_ixs.clone();
            ordered_cii.sort_by(|a, b| a.index.cmp(&b.index));

            for cii in ordered_cii {
                let parent_program_id =
                    parsed_accounts[ci_ixs[cii.index as usize].program_id_index as usize].pubkey;

                for cii_entry in cii.instructions {
                    let ix = CompiledInstruction {
                        program_id_index: cii_entry.instruction.program_id_index,
                        accounts: cii_entry.instruction.accounts.clone(),
                        data: cii_entry.instruction.data.clone(),
                    };
                    inner_result.push(TransactionInstructionWithParent {
                        instruction: self.compiled_instruction_to_instruction(&ix, parsed_accounts.clone()),
                        parent_program_id: Some(parent_program_id),
                    });
                }
            }
        }

        inner_result
    }

    fn compiled_instruction_to_instruction(
        &self,
        ci: &CompiledInstruction,
        parsed_accounts: Vec<AccountMeta>,
    ) -> Instruction {
        let program_id = parsed_accounts[ci.program_id_index as usize].pubkey;
        let accounts: Vec<AccountMeta> = ci.accounts.iter().map(|&index| {
            if index as usize >= parsed_accounts.len() {
                panic!(
                    "Trying to resolve account at index {} while parsedAccounts is only {}. \
                    Looks like you're trying to parse versioned transaction, make sure that LoadedAddresses are passed to the \
                    parseTransactionAccounts function",
                    index, parsed_accounts.len()
                );
            }
            parsed_accounts[index as usize].clone()
        }).collect();

        Instruction {
            program_id,
            accounts,
            data: ci.data.clone(),
        }
    }

    pub fn convert_token_balance(
        tb: TransactionTokenBalance,
    ) -> TransactionTokenBalance {
        let ui_token_amount = tb.ui_token_amount.clone();
        TransactionTokenBalance {
            account_index: tb.account_index as u8,
            mint: tb.mint.clone(),
            ui_token_amount: UiTokenAmount {
                ui_amount: if ui_token_amount.ui_amount == Some(0.0) {
                    None
                } else {
                    ui_token_amount.ui_amount
                },
                decimals: ui_token_amount.decimals as u8,
                amount: ui_token_amount.amount,
                ui_amount_string: ui_token_amount.ui_amount_string,
            },
            owner: tb.owner.clone(),
            program_id: tb.program_id.clone(),
        }
    }

    fn to_camel_case(&self, name: &str) -> String {
        let mut chars = name.chars();
        match chars.next() {
        Some(first_char) => first_char.to_lowercase().collect::<String>() + chars.as_str(),
        None => String::new(),
    }
}
}