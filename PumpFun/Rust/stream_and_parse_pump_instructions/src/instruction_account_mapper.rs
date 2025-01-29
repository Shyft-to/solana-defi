use crate::serialization::serialize_pubkey;
use serde::{Deserialize, Serialize};
use solana_program::{program_error::ProgramError, pubkey::Pubkey};
use solana_sdk::instruction::AccountMeta;

#[derive(Deserialize)]
struct IdlInstruction {
    name: String,
    accounts: Vec<IdlAccount>,
}

#[derive(Deserialize)]
struct IdlAccount {
    name: String,
    #[serde(rename = "isMut")]
    is_mut: bool,
    #[serde(rename = "isSigner")]
    is_signer: bool,
}

#[derive(Deserialize)]
pub struct Idl {
    instructions: Vec<IdlInstruction>,
}

#[derive(Debug, Serialize)]
pub struct AccountMetadata {
    #[serde(serialize_with = "serialize_pubkey")]
    pub pubkey: Pubkey,
    pub is_writable: bool,
    pub is_signer: bool,
    pub name: String,
}

pub trait InstructionAccountMapper<'info> {
    fn map_accounts<'me>(
        &self,
        accounts: &[AccountMeta],
        instruction_name: &str,
    ) -> Result<Vec<AccountMetadata>, ProgramError>;
}

impl<'info> InstructionAccountMapper<'info> for Idl {
    fn map_accounts<'me>(
        &self,
        accounts: &[AccountMeta],
        instruction_name: &str,
    ) -> Result<Vec<AccountMetadata>, ProgramError> {
        let instruction = self
            .instructions
            .iter()
            .find(|ix| ix.name == instruction_name)
            .ok_or(ProgramError::InvalidArgument)?;

        let mut account_metadata: Vec<AccountMetadata> = accounts
            .iter()
            .take(instruction.accounts.len())
            .enumerate()
            .map(|(i, account)| {
                let account_info = &instruction.accounts[i];
                AccountMetadata {
                    pubkey: account.pubkey,
                    is_writable: account_info.is_mut,
                    is_signer: account_info.is_signer,
                    name: account_info.name.clone(),
                }
            })
            .collect();

        for (i, account) in accounts.iter().enumerate().skip(instruction.accounts.len()) {
            account_metadata.push(AccountMetadata {
                pubkey: account.pubkey,
                is_writable: account.is_writable,
                is_signer: account.is_signer,
                name: format!("Remaining accounts {}", i - instruction.accounts.len() + 1),
            });
        }

        Ok(account_metadata)
    }
}
