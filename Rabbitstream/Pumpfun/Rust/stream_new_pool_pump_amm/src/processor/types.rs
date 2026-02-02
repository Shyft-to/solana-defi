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
#[derive(Debug, Clone)]
pub struct PumpAmmCreatePoolEvent {
    pub pool: String,
    pub creator: String,
    pub base_mint: String,
    pub quote_mint: String,
    pub base_amount: u64,
    pub quote_amount: u64,
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
