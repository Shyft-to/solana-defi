#[cfg(feature = "serde")]
use crate::serializer::{
    deserialize_i128_as_string, deserialize_u128_as_string, serialize_i128_as_string,
    serialize_u128_as_string,
};
use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::pubkey::Pubkey;

#[derive(Default, Clone, Debug, BorshDeserialize, BorshSerialize, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct BuyEvent {
    pub timestamp: i64,
    pub base_amount_out: u64,
    pub max_quote_amount_in: u64,
    pub user_base_token_reserves: u64,
    pub user_quote_token_reserves: u64,
    pub pool_base_token_reserves: u64,
    pub pool_quote_token_reserves: u64,
    pub quote_amount_in: u64,
    pub lp_fee_basis_points: u64,
    pub lp_fee: u64,
    pub protocol_fee_basis_points: u64,
    pub protocol_fee: u64,
    pub quote_amount_in_with_lp_fee: u64,
    pub user_quote_amount_in: u64,
    pub pool: Pubkey,
    pub user: Pubkey,
    pub user_base_token_account: Pubkey,
    pub user_quote_token_account: Pubkey,
    pub protocol_fee_recipient: Pubkey,
    pub protocol_fee_recipient_token_account: Pubkey,
}

#[derive(Default, Clone, Debug, BorshDeserialize, BorshSerialize, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CreateConfigEvent {
    pub timestamp: i64,
    pub admin: Pubkey,
    pub lp_fee_basis_points: u64,
    pub protocol_fee_basis_points: u64,
    pub protocol_fee_recipients: [Pubkey; 8],
}

#[derive(Default, Clone, Debug, BorshDeserialize, BorshSerialize, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CreatePoolEvent {
    pub timestamp: i64,
    pub index: u16,
    pub creator: Pubkey,
    pub base_mint: Pubkey,
    pub quote_mint: Pubkey,
    pub base_mint_decimals: u8,
    pub quote_mint_decimals: u8,
    pub base_amount_in: u64,
    pub quote_amount_in: u64,
    pub pool_base_amount: u64,
    pub pool_quote_amount: u64,
    pub minimum_liquidity: u64,
    pub initial_liquidity: u64,
    pub lp_token_amount_out: u64,
    pub pool_bump: u8,
    pub pool: Pubkey,
    pub lp_mint: Pubkey,
    pub user_base_token_account: Pubkey,
    pub user_quote_token_account: Pubkey,
}

#[derive(Default, Clone, Debug, BorshDeserialize, BorshSerialize, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct DepositEvent {
    pub timestamp: i64,
    pub lp_token_amount_out: u64,
    pub max_base_amount_in: u64,
    pub max_quote_amount_in: u64,
    pub user_base_token_reserves: u64,
    pub user_quote_token_reserves: u64,
    pub pool_base_token_reserves: u64,
    pub pool_quote_token_reserves: u64,
    pub base_amount_in: u64,
    pub quote_amount_in: u64,
    pub lp_mint_supply: u64,
    pub pool: Pubkey,
    pub user: Pubkey,
    pub user_base_token_account: Pubkey,
    pub user_quote_token_account: Pubkey,
    pub user_pool_token_account: Pubkey,
}

#[derive(Default, Clone, Debug, BorshDeserialize, BorshSerialize, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct DisableEvent {
    pub timestamp: i64,
    pub admin: Pubkey,
    pub disable_create_pool: bool,
    pub disable_deposit: bool,
    pub disable_withdraw: bool,
    pub disable_buy: bool,
    pub disable_sell: bool,
}

#[derive(Default, Clone, Debug, BorshDeserialize, BorshSerialize, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ExtendAccountEvent {
    pub timestamp: i64,
    pub account: Pubkey,
    pub user: Pubkey,
    pub current_size: u64,
    pub new_size: u64,
}

#[derive(Default, Clone, Debug, BorshDeserialize, BorshSerialize, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct GlobalConfig {
    pub admin: Pubkey,
    pub lp_fee_basis_points: u64,
    pub protocol_fee_basis_points: u64,
    pub disable_flags: u8,
    pub protocol_fee_recipients: [Pubkey; 8],
}

#[derive(Default, Clone, Debug, BorshDeserialize, BorshSerialize, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Pool {
    pub pool_bump: u8,
    pub index: u16,
    pub creator: Pubkey,
    pub base_mint: Pubkey,
    pub quote_mint: Pubkey,
    pub lp_mint: Pubkey,
    pub pool_base_token_account: Pubkey,
    pub pool_quote_token_account: Pubkey,
    pub lp_supply: u64,
}

#[derive(Default, Clone, Debug, BorshDeserialize, BorshSerialize, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SellEvent {
    pub timestamp: i64,
    pub base_amount_in: u64,
    pub min_quote_amount_out: u64,
    pub user_base_token_reserves: u64,
    pub user_quote_token_reserves: u64,
    pub pool_base_token_reserves: u64,
    pub pool_quote_token_reserves: u64,
    pub quote_amount_out: u64,
    pub lp_fee_basis_points: u64,
    pub lp_fee: u64,
    pub protocol_fee_basis_points: u64,
    pub protocol_fee: u64,
    pub quote_amount_out_without_lp_fee: u64,
    pub user_quote_amount_out: u64,
    pub pool: Pubkey,
    pub user: Pubkey,
    pub user_base_token_account: Pubkey,
    pub user_quote_token_account: Pubkey,
    pub protocol_fee_recipient: Pubkey,
    pub protocol_fee_recipient_token_account: Pubkey,
}

#[derive(Default, Clone, Debug, BorshDeserialize, BorshSerialize, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct UpdateAdminEvent {
    pub timestamp: i64,
    pub admin: Pubkey,
    pub new_admin: Pubkey,
}

#[derive(Default, Clone, Debug, BorshDeserialize, BorshSerialize, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct UpdateFeeConfigEvent {
    pub timestamp: i64,
    pub admin: Pubkey,
    pub lp_fee_basis_points: u64,
    pub protocol_fee_basis_points: u64,
    pub protocol_fee_recipients: [Pubkey; 8],
}

#[derive(Default, Clone, Debug, BorshDeserialize, BorshSerialize, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct WithdrawEvent {
    pub timestamp: i64,
    pub lp_token_amount_in: u64,
    pub min_base_amount_out: u64,
    pub min_quote_amount_out: u64,
    pub user_base_token_reserves: u64,
    pub user_quote_token_reserves: u64,
    pub pool_base_token_reserves: u64,
    pub pool_quote_token_reserves: u64,
    pub base_amount_out: u64,
    pub quote_amount_out: u64,
    pub lp_mint_supply: u64,
    pub pool: Pubkey,
    pub user: Pubkey,
    pub user_base_token_account: Pubkey,
    pub user_quote_token_account: Pubkey,
    pub user_pool_token_account: Pubkey,
}
