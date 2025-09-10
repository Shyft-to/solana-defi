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
#[derive(Debug)]
pub struct PriceData {
    pub token_a: String,
    pub token_b: String,
    pub price: String,
}
