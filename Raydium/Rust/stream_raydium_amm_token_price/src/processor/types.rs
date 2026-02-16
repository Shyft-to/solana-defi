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
#[derive(Clone, Serialize)]
//#[derive(Debug, Clone)]
pub struct RaydiumSwapParsed {
    pub base_mint: String,
    pub quote_mint: String,
    pub base_decimals: i32,
    pub quote_decimals: i32,
    pub amount_in: u64,
    pub amount_out: u64,
    pub amount_in_formatted: String,
    pub amount_out_formatted: String,
    pub pool_base_reserves: u64,
    pub pool_quote_reserves: u64,
    pub pool_base_reserves_decimal: f64,
    pub pool_quote_reserves_decimal: f64,
    pub base_token_price: String,      // SOL per token (formatted, no scientific notation)
    pub quote_token_price: String,     // tokens per SOL (formatted, no scientific notation)
    pub pool_base_token_price: String, // SOL per token from pool (formatted, no scientific notation)
    pub pool_quote_token_price: String, // tokens per SOL from pool (formatted, no scientific notation)
    pub swap_type: String,
    pub direction: u64,
    pub pool_price: String,
    pub price_impact: String,
}