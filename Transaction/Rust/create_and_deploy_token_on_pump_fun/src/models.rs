use solana_sdk::pubkey::Pubkey;
use serde::{Deserialize, Serialize};
use std::str::FromStr;  // This import is critical

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PumpPoolInfo {
    pub mint: Pubkey,
    pub bonding_curve: Pubkey,
    pub associated_bonding_curve: Pubkey,
    pub creator: Pubkey,  // Add this field
    pub virtual_token_reserves: u64,
    pub virtual_sol_reserves: u64,
    pub token_total_supply: u64,
    pub complete: bool,
    pub created_at: Option<i64>,
}

#[derive(Debug, Clone)]
pub struct BuyQuote {
    pub input_amount: u64,
    pub expected_output: u64,
    pub min_output: u64,
    pub fee_amount: u64,
    pub price: f64,
    pub price_impact: f64,
}

#[derive(Debug, Clone)]
pub struct SellQuote {
    pub input_amount: u64,
    pub expected_output: u64,
    pub max_output: u64,
    pub fee_amount: u64,
    pub price: f64,
    pub price_impact: f64,
}

#[derive(Debug, Clone)]
pub struct SwapConfig {
    pub slippage_bps: u16,
    pub priority_fee_microlamports: Option<u64>,
    pub compute_unit_limit: Option<u32>,
    pub use_jito: bool,
    pub network: Network,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Network {
    Devnet,
    Mainnet,
    Localnet,
}

impl Default for SwapConfig {
    fn default() -> Self {
        Self {
            slippage_bps: 100, // 1%
            priority_fee_microlamports: Some(5000),
            compute_unit_limit: Some(200_000),
            use_jito: false,
            network: Network::Mainnet,
        }
    }
}

impl SwapConfig {
    pub fn devnet() -> Self {
        Self {
            network: Network::Devnet,
            ..Default::default()
        }
    }
    
    pub fn mainnet() -> Self {
        Self {
            network: Network::Mainnet,
            ..Default::default()
        }
    }
    
    pub fn get_program_id(&self) -> Pubkey {
        match self.network {
            Network::Devnet => Pubkey::from_str(crate::constants::PUMP_FUN_PROGRAM).unwrap(),
            Network::Mainnet => Pubkey::from_str(crate::constants::PUMP_FUN_PROGRAM).unwrap(),
            Network::Localnet => Pubkey::from_str(crate::constants::PUMP_FUN_PROGRAM).unwrap(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct TokenInfo {
    pub mint: Pubkey,
    pub name: String,
    pub symbol: String,
    pub uri: String,
    pub decimals: u8,
    pub supply: u64,
}

#[derive(Debug, Clone)]
pub struct TransactionResult {
    pub signature: String,
    pub explorer_url: String,
    pub slot: u64,
    pub block_time: Option<i64>,
    pub fee: u64,
}