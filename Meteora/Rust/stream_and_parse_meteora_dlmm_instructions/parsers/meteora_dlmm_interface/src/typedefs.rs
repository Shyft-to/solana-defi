use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::pubkey::Pubkey;
#[derive(Clone, Debug, BorshDeserialize, BorshSerialize, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InitPresetParametersIx {
    pub bin_step: u16,
    pub base_factor: u16,
    pub filter_period: u16,
    pub decay_period: u16,
    pub reduction_factor: u16,
    pub variable_fee_control: u32,
    pub max_volatility_accumulator: u32,
    pub min_bin_id: i32,
    pub max_bin_id: i32,
    pub protocol_share: u16,
}
#[derive(Clone, Debug, BorshDeserialize, BorshSerialize, PartialEq,Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FeeParameter {
    pub protocol_share: u16,
    pub base_factor: u16,
}
#[derive(Clone, Debug, BorshDeserialize, BorshSerialize, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct LiquidityParameterByStrategyOneSide {
    pub amount: u64,
    pub active_id: i32,
    pub max_active_bin_slippage: i32,
    pub strategy_parameters: StrategyParameters,
}
#[derive(Clone, Debug, BorshDeserialize, BorshSerialize, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct LiquidityParameterByStrategy {
    pub amount_x: u64,
    pub amount_y: u64,
    pub active_id: i32,
    pub max_active_bin_slippage: i32,
    pub strategy_parameters: StrategyParameters,
}
#[derive(Clone, Debug, BorshDeserialize, BorshSerialize, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct StrategyParameters {
    pub min_bin_id: i32,
    pub max_bin_id: i32,
    pub strategy_type: StrategyType,
    pub parameteres: Vec<u8>,
}
#[derive(Clone, Debug, BorshDeserialize, BorshSerialize, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct LiquidityOneSideParameter {
    pub amount: u64,
    pub active_id: i32,
    pub max_active_bin_slippage: i32,
    pub bin_liquidity_dist: Vec<BinLiquidityDistributionByWeight>,
}
#[derive(Clone, Debug, BorshDeserialize, BorshSerialize, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct BinLiquidityDistributionByWeight {
    pub bin_id: i32,
    pub weight: u16,
}
#[derive(Clone, Debug, BorshDeserialize, BorshSerialize, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct LiquidityParameterByWeight {
    pub amount_x: u64,
    pub amount_y: u64,
    pub active_id: i32,
    pub max_active_bin_slippage: i32,
    pub bin_liquidity_dist: Vec<BinLiquidityDistributionByWeight>,
}
#[derive(Clone, Debug, BorshDeserialize, BorshSerialize, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct AddLiquiditySingleSidePreciseParameter {
    pub bins: Vec<CompressedBinDepositAmount>,
    pub decompress_multiplier: u64,
}
#[derive(Clone, Debug, BorshDeserialize, BorshSerialize, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CompressedBinDepositAmount {
    pub bin_id: i32,
    pub amount: u32,
}
#[derive(Clone, Debug, BorshDeserialize, BorshSerialize, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct BinLiquidityDistribution {
    pub bin_id: i32,
    pub distribution_x: u16,
    pub distribution_y: u16,
}
#[derive(Clone, Debug, BorshDeserialize, BorshSerialize, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct LiquidityParameter {
    pub amount_x: u64,
    pub amount_y: u64,
    pub bin_liquidity_dist: Vec<BinLiquidityDistribution>,
}
#[derive(Clone, Debug, BorshDeserialize, BorshSerialize, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CustomizableParams {
    pub active_id: i32,
    pub bin_step: u16,
    pub base_factor: u16,
    pub activation_type: u8,
    pub has_alpha_vault: bool,
    pub activation_point: Option<u64>,
    pub padding: Vec<u8>,
}
#[derive(Clone, Debug, BorshDeserialize, BorshSerialize, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InitPermissionPairIx {
    pub active_id: i32,
    pub bin_step: u16,
    pub base_factor: u16,
    pub min_bin_id: i32,
    pub max_bin_id: i32,
    pub activation_type: u8,
}
#[derive(Clone, Debug, BorshDeserialize, BorshSerialize, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct BinLiquidityReduction {
    pub bin_id: i32,
    pub bps_to_remove: u16,
}
#[derive(Clone, Debug, BorshDeserialize, BorshSerialize, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Bin {
    pub amount_x: u64,
    pub amount_y: u64,
    pub price: u128,
    pub liquidity_supply: u128,
    pub reward_per_token_stored: [u128; 2],
    pub fee_amount_x_per_token_stored: u128,
    pub fee_amount_y_per_token_stored: u128,
    pub amount_x_in: u128,
    pub amount_y_in: u128,
}
#[derive(Clone, Debug, BorshDeserialize, BorshSerialize, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ProtocolFee {
    pub amount_x: u64,
    pub amount_y: u64,
}
#[derive(Clone, Debug, BorshDeserialize, BorshSerialize, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RewardInfo {
    pub mint: Pubkey,
    pub vault: Pubkey,
    pub funder: Pubkey,
    pub reward_duration: u64,
    pub reward_duration_end: u64,
    pub reward_rate: u128,
    pub last_update_time: u64,
    pub cumulative_seconds_with_empty_liquidity_reward: u64,
}
#[derive(Clone, Debug, BorshDeserialize, BorshSerialize, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Observation {
    pub cumulative_active_bin_id: i128,
    pub created_at: i64,
    pub last_updated_at: i64,
}
#[derive(Clone, Debug, BorshDeserialize, BorshSerialize, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct StaticParameters {
    pub base_factor: u16,
    pub filter_period: u16,
    pub decay_period: u16,
    pub reduction_factor: u16,
    pub variable_fee_control: u32,
    pub max_volatility_accumulator: u32,
    pub min_bin_id: i32,
    pub max_bin_id: i32,
    pub protocol_share: u16,
    pub padding: [u8; 6],
}
#[derive(Clone, Debug, BorshDeserialize, BorshSerialize, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct VariableParameters {
    pub volatility_accumulator: u32,
    pub volatility_reference: u32,
    pub index_reference: i32,
    pub padding: [u8; 4],
    pub last_update_timestamp: i64,
    pub padding1: [u8; 8],
}
#[derive(Clone, Debug, BorshDeserialize, BorshSerialize, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FeeInfo {
    pub fee_x_per_token_complete: u128,
    pub fee_y_per_token_complete: u128,
    pub fee_x_pending: u64,
    pub fee_y_pending: u64,
}
#[derive(Clone, Debug, BorshDeserialize, BorshSerialize, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct UserRewardInfo {
    pub reward_per_token_completes: [u128; 2],
    pub reward_pendings: [u64; 2],
}
#[derive(Clone, Debug, BorshDeserialize, BorshSerialize, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum StrategyType {
    #[default]
    SpotOneSide,
    CurveOneSide,
    BidAskOneSide,
    SpotBalanced,
    CurveBalanced,
    BidAskBalanced,
    SpotImBalanced,
    CurveImBalanced,
    BidAskImBalanced,
}
#[derive(Clone, Debug, BorshDeserialize, BorshSerialize, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Rounding {
    Up,
    Down,
}
#[derive(Clone, Debug, BorshDeserialize, BorshSerialize, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum ActivationType {
    Slot,
    Timestamp,
}
#[derive(Clone, Debug, BorshDeserialize, BorshSerialize, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum LayoutVersion {
    V0,
    V1,
}
#[derive(Clone, Debug, BorshDeserialize, BorshSerialize, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum PairType {
    Permissionless,
    Permission,
    CustomizablePermissionless,
}
#[derive(Clone, Debug, BorshDeserialize, BorshSerialize, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum PairStatus {
    Enabled,
    Disabled,
}
