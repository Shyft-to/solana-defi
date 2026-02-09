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
pub struct TransactionEvent {
    pub event_type: Option<String>,
    pub user: Option<String>,
    pub mint: Option<String>,
    pub bonding_curve: Option<String>,

    pub amount_in: Option<u64>,
    pub amount_out: Option<u64>,

    pub timestamp_in_blockchain: Option<i64>,
    pub readable_time_of_trade: Option<String>,

    pub price: Option<PriceInfo>,
    pub pool_state: Option<PoolState>,
    pub fees: Option<FeeInfo>,
    pub behavior: Option<TradeBehavior>,
}

#[derive(Debug, Clone)]
pub struct PriceInfo {
    pub sol_per_token: Option<f64>,
    pub token_per_sol: Option<f64>,
    pub market_price: Option<String>,
}

#[derive(Debug, Clone)]
pub struct PoolState {
    pub virtual_sol: u64,
    pub virtual_token: u64,
    pub real_sol: u64,
    pub real_token: u64,
}

#[derive(Debug, Clone)]
pub struct FeeInfo {
    pub protocol: FeePart,
    pub creator: FeePart,
}

#[derive(Debug, Clone)]
pub struct FeePart {
    pub amount: u64,
    pub bps: u64,
}

#[derive(Debug, Clone)]
pub struct TradeBehavior {
    pub is_buy: bool,
    pub is_sell: bool,
    pub large_trade: bool,
    pub early_pool: bool,
}
