use solana_sdk::{instruction::Instruction, pubkey::Pubkey};
use serde::Serialize;

use crate::processor::models::mapper::event::DecodedEvent;
use crate::processor::models::mapper::instruction::AccountMetadata;
use crate::processor::models::serialize::serialization::{
    serialize_option_pubkey,
    serialize_pubkey,
};

#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct DecodedInstruction {
    pub name: String,
    pub accounts: Vec<AccountMetadata>,
    pub data: serde_json::Value,
    pub event: Option<DecodedEvent>,

    #[serde(serialize_with = "serialize_pubkey")]
    pub program_id: Pubkey,

    #[serde(serialize_with = "serialize_option_pubkey")]
    pub parent_program_id: Option<Pubkey>,
}

#[derive(Debug)]
pub struct TransactionInstructionWithParent {
    pub instruction: Instruction,
    pub parent_program_id: Option<Pubkey>,
}

pub struct ExtractedInstructions {
    pub compiled: Vec<TransactionInstructionWithParent>,
    pub inner: Vec<TransactionInstructionWithParent>,
}
#[derive(Debug, Clone)]
pub struct PumpFunParsedResult {
    pub name: String,
    pub symbol: String,
    pub uri: String,
    pub mint: Option<String>,
    pub bonding_curve: Option<String>,
    pub creator: String,
    pub virtual_token_reserves: u64,
    pub virtual_sol_reserves: u64,
    pub real_token_reserves: u64,
    pub token_total_supply: u64,
}