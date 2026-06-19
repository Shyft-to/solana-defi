use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::pubkey::Pubkey;
#[cfg(feature = "bytemuck")]
use bytemuck::{Pod, Zeroable};

// ============================================================
// Type definitions derived from IDL (meteora_damm.json)
// ============================================================

#[derive(Clone, Debug, BorshDeserialize, BorshSerialize, PartialEq, serde::Serialize, serde::Deserialize, Default)]
pub struct AddLiquidityParameters {
    /// delta liquidity
    pub liquidity_delta: u128,
    /// maximum token a amount
    pub token_a_amount_threshold: u64,
    /// maximum token b amount
    pub token_b_amount_threshold: u64,
}

#[derive(Clone, Debug, BorshDeserialize, BorshSerialize, PartialEq, serde::Serialize, serde::Deserialize, Default)]
#[cfg_attr(feature = "bytemuck", derive(Zeroable, Pod))]
#[repr(C)]
pub struct BaseFeeInfo {
    pub data: [u8; 32],
}

#[derive(Clone, Debug, BorshDeserialize, BorshSerialize, PartialEq, serde::Serialize, serde::Deserialize, Default)]
pub struct BaseFeeParameters {
    pub data: [u8; 30],
}

#[derive(Clone, Debug, BorshDeserialize, BorshSerialize, PartialEq, serde::Serialize, serde::Deserialize, Default)]
#[cfg_attr(feature = "bytemuck", derive(Zeroable, Pod))]
#[repr(C)]
pub struct BaseFeeStruct {
    pub base_fee_info: BaseFeeInfo,
    pub padding_1: u64,
}

#[derive(Clone, Debug, BorshDeserialize, BorshSerialize, PartialEq, serde::Serialize, serde::Deserialize, Default)]
pub struct BorshFeeMarketCapScheduler {
    pub cliff_fee_numerator: u64,
    pub number_of_period: u16,
    pub sqrt_price_step_bps: u32,
    pub scheduler_expiration_duration: u32,
    pub reduction_factor: u64,
    pub base_fee_mode: u8,
    pub padding: [u8; 3],
}

#[derive(Clone, Debug, BorshDeserialize, BorshSerialize, PartialEq, serde::Serialize, serde::Deserialize, Default)]
pub struct BorshFeeRateLimiter {
    pub cliff_fee_numerator: u64,
    pub fee_increment_bps: u16,
    pub max_limiter_duration: u32,
    pub max_fee_bps: u32,
    pub reference_amount: u64,
    pub base_fee_mode: u8,
    pub padding: [u8; 3],
}

#[derive(Clone, Debug, BorshDeserialize, BorshSerialize, PartialEq, serde::Serialize, serde::Deserialize, Default)]
pub struct BorshFeeTimeScheduler {
    pub cliff_fee_numerator: u64,
    pub number_of_period: u16,
    pub period_frequency: u64,
    pub reduction_factor: u64,
    pub base_fee_mode: u8,
    pub padding: [u8; 3],
}

#[derive(Clone, Debug, BorshDeserialize, BorshSerialize, PartialEq, serde::Serialize, serde::Deserialize, Default)]
#[cfg_attr(feature = "bytemuck", derive(Zeroable, Pod))]
#[repr(C)]
pub struct Config {
    /// Vault config key
    pub vault_config_key: Pubkey,
    /// Only pool_creator_authority can use the current config to initialize new pool. When it's Pubkey::default, it's a public config.
    pub pool_creator_authority: Pubkey,
    /// Pool fee
    pub pool_fees: PoolFeesConfig,
    /// Activation type
    pub activation_type: u8,
    /// Collect fee mode
    pub collect_fee_mode: u8,
    /// Config type mode, 0 for static, 1 for dynamic
    pub config_type: u8,
    /// padding 0
    pub _padding_0: [u8; 5],
    /// config index
    pub index: u64,
    /// sqrt min price
    pub sqrt_min_price: u128,
    /// sqrt max price
    pub sqrt_max_price: u128,
    /// Fee curve point, Padding for further use
    pub _padding_1: [u64; 10],
}

#[derive(Clone, Debug, BorshDeserialize, BorshSerialize, PartialEq, serde::Serialize, serde::Deserialize, Default)]
pub struct DummyParams {
    pub borsh_fee_time_scheduler_params: BorshFeeTimeScheduler,
    pub borsh_fee_rate_limiter_params: BorshFeeRateLimiter,
    pub borsh_fee_market_cap_scheduler_params: BorshFeeMarketCapScheduler,
}

#[derive(Clone, Debug, BorshDeserialize, BorshSerialize, PartialEq, serde::Serialize, serde::Deserialize, Default)]
pub struct DynamicConfigParameters {
    pub pool_creator_authority: Pubkey,
}

#[derive(Clone, Debug, BorshDeserialize, BorshSerialize, PartialEq, serde::Serialize, serde::Deserialize, Default)]
#[cfg_attr(feature = "bytemuck", derive(Zeroable, Pod))]
#[repr(C)]
pub struct DynamicFeeConfig {
    pub initialized: u8,
    pub padding: [u8; 7],
    pub max_volatility_accumulator: u32,
    pub variable_fee_control: u32,
    pub bin_step: u16,
    pub filter_period: u16,
    pub decay_period: u16,
    pub reduction_factor: u16,
    pub padding_1: [u8; 8],
    pub bin_step_u128: u128,
}

#[derive(Clone, Debug, BorshDeserialize, BorshSerialize, PartialEq, serde::Serialize, serde::Deserialize, Default)]
pub struct DynamicFeeParameters {
    pub bin_step: u16,
    pub bin_step_u128: u128,
    pub filter_period: u16,
    pub decay_period: u16,
    pub reduction_factor: u16,
    pub max_volatility_accumulator: u32,
    pub variable_fee_control: u32,
}

#[derive(Clone, Debug, BorshDeserialize, BorshSerialize, PartialEq, serde::Serialize, serde::Deserialize, Default)]
#[cfg_attr(feature = "bytemuck", derive(Zeroable, Pod))]
#[repr(C)]
pub struct DynamicFeeStruct {
    pub initialized: u8,
    pub padding: [u8; 7],
    pub max_volatility_accumulator: u32,
    pub variable_fee_control: u32,
    pub bin_step: u16,
    pub filter_period: u16,
    pub decay_period: u16,
    pub reduction_factor: u16,
    pub last_update_timestamp: u64,
    pub bin_step_u128: u128,
    pub sqrt_price_reference: u128,
    pub volatility_accumulator: u128,
    pub volatility_reference: u128,
}

#[derive(Clone, Debug, BorshDeserialize, BorshSerialize, PartialEq, serde::Serialize, serde::Deserialize, Default)]
pub struct InitializeCustomizablePoolParameters {
    /// pool fees
    pub pool_fees: PoolFeeParameters,
    /// sqrt min price
    pub sqrt_min_price: u128,
    /// sqrt max price
    pub sqrt_max_price: u128,
    /// has alpha vault
    pub has_alpha_vault: bool,
    /// initialize liquidity
    pub liquidity: u128,
    /// The init price of the pool as a sqrt(token_b/token_a) Q64.64 value. Market cap fee scheduler minimum price will be derived from this value
    pub sqrt_price: u128,
    /// activation type
    pub activation_type: u8,
    /// collect fee mode
    pub collect_fee_mode: u8,
    /// activation point
    pub activation_point: Option<u64>,
}

#[derive(Clone, Debug, BorshDeserialize, BorshSerialize, PartialEq, serde::Serialize, serde::Deserialize, Default)]
pub struct InitializePoolParameters {
    /// initialize liquidity
    pub liquidity: u128,
    /// The init price of the pool as a sqrt(token_b/token_a) Q64.64 value
    pub sqrt_price: u128,
    /// activation point
    pub activation_point: Option<u64>,
}

#[derive(Clone, Debug, BorshDeserialize, BorshSerialize, PartialEq, serde::Serialize, serde::Deserialize, Default)]
#[cfg_attr(feature = "bytemuck", derive(Zeroable, Pod))]
#[repr(C)]
pub struct InnerVesting {
    pub cliff_point: u64,
    pub period_frequency: u64,
    pub cliff_unlock_liquidity: u128,
    pub liquidity_per_period: u128,
    pub total_released_liquidity: u128,
    pub number_of_period: u16,
    pub padding: [u8; 14],
}

#[derive(Clone, Debug, BorshDeserialize, BorshSerialize, PartialEq, serde::Serialize, serde::Deserialize, Default)]
#[cfg_attr(feature = "bytemuck", derive(Zeroable, Pod))]
#[repr(C)]
pub struct Operator {
    pub whitelisted_address: Pubkey,
    pub permission: u128,
    pub padding: [u64; 2],
}

#[derive(Clone, Debug, BorshDeserialize, BorshSerialize, PartialEq, serde::Serialize, serde::Deserialize, Default)]
#[cfg_attr(feature = "bytemuck", derive(Zeroable, Pod))]
#[repr(C)]
pub struct PodAlignedFeeMarketCapScheduler {
    pub cliff_fee_numerator: u64,
    pub base_fee_mode: u8,
    pub padding: [u8; 5],
    pub number_of_period: u16,
    pub sqrt_price_step_bps: u32,
    pub scheduler_expiration_duration: u32,
    pub reduction_factor: u64,
}

#[derive(Clone, Debug, BorshDeserialize, BorshSerialize, PartialEq, serde::Serialize, serde::Deserialize, Default)]
#[cfg_attr(feature = "bytemuck", derive(Zeroable, Pod))]
#[repr(C)]
pub struct PodAlignedFeeRateLimiter {
    pub cliff_fee_numerator: u64,
    pub base_fee_mode: u8,
    pub padding: [u8; 5],
    pub fee_increment_bps: u16,
    pub max_limiter_duration: u32,
    pub max_fee_bps: u32,
    pub reference_amount: u64,
}

#[derive(Clone, Debug, BorshDeserialize, BorshSerialize, PartialEq, serde::Serialize, serde::Deserialize, Default)]
#[cfg_attr(feature = "bytemuck", derive(Zeroable, Pod))]
#[repr(C)]
pub struct PodAlignedFeeTimeScheduler {
    pub cliff_fee_numerator: u64,
    pub base_fee_mode: u8,
    pub padding: [u8; 5],
    pub number_of_period: u16,
    pub period_frequency: u64,
    pub reduction_factor: u64,
}

#[derive(Clone, Debug, BorshDeserialize, BorshSerialize, PartialEq, serde::Serialize, serde::Deserialize, Default)]
#[cfg_attr(feature = "bytemuck", derive(Zeroable, Pod))]
#[repr(C)]
pub struct Pool {
    /// Pool fee
    pub pool_fees: PoolFeesStruct,
    /// token a mint
    pub token_a_mint: Pubkey,
    /// token b mint
    pub token_b_mint: Pubkey,
    /// token a vault
    pub token_a_vault: Pubkey,
    /// token b vault
    pub token_b_vault: Pubkey,
    /// Whitelisted vault to be able to buy pool before activation_point
    pub whitelisted_vault: Pubkey,
    /// partner
    pub partner: Pubkey,
    /// liquidity share
    pub liquidity: u128,
    /// padding, previous reserve amount, be careful to use that field
    pub _padding: u128,
    /// protocol a fee
    pub protocol_a_fee: u64,
    /// protocol b fee
    pub protocol_b_fee: u64,
    /// partner a fee
    pub partner_a_fee: u64,
    /// partner b fee
    pub partner_b_fee: u64,
    /// min price
    pub sqrt_min_price: u128,
    /// max price
    pub sqrt_max_price: u128,
    /// current price
    pub sqrt_price: u128,
    /// Activation point, can be slot or timestamp
    pub activation_point: u64,
    /// Activation type, 0 means by slot, 1 means by timestamp
    pub activation_type: u8,
    /// pool status, 0: enable, 1 disable
    pub pool_status: u8,
    /// token a flag
    pub token_a_flag: u8,
    /// token b flag
    pub token_b_flag: u8,
    /// 0 is collect fee in both token, 1 only collect fee only in token b
    pub collect_fee_mode: u8,
    /// pool type
    pub pool_type: u8,
    /// pool version, 0: max_fee is still capped at 50%, 1: max_fee is capped at 99%
    pub version: u8,
    /// padding
    pub _padding_0: u8,
    /// cumulative
    pub fee_a_per_liquidity: [u8; 32],
    /// cumulative
    pub fee_b_per_liquidity: [u8; 32],
    pub permanent_lock_liquidity: u128,
    /// metrics
    pub metrics: PoolMetrics,
    /// pool creator
    pub creator: Pubkey,
    /// Padding for further use
    pub _padding_1: [u64; 6],
    /// Farming reward information
    pub reward_infos: [RewardInfo; 2],
}

/// Information regarding fee charges
#[derive(Clone, Debug, BorshDeserialize, BorshSerialize, PartialEq, serde::Serialize, serde::Deserialize, Default)]
pub struct PoolFeeParameters {
    /// Base fee
    pub base_fee: BaseFeeParameters,
    /// dynamic fee
    pub dynamic_fee: Option<DynamicFeeParameters>,
}

#[derive(Clone, Debug, BorshDeserialize, BorshSerialize, PartialEq, serde::Serialize, serde::Deserialize, Default)]
#[cfg_attr(feature = "bytemuck", derive(Zeroable, Pod))]
#[repr(C)]
pub struct PoolFeesConfig {
    pub base_fee: BaseFeeInfo,
    pub dynamic_fee: DynamicFeeConfig,
    pub protocol_fee_percent: u8,
    pub partner_fee_percent: u8,
    pub referral_fee_percent: u8,
    pub padding_0: [u8; 5],
    pub padding_1: [u64; 5],
}

/// Information regarding fee charges
/// trading_fee = amount * trade_fee_numerator / denominator
/// protocol_fee = trading_fee * protocol_fee_percentage / 100
/// referral_fee = protocol_fee * referral_percentage / 100
/// partner_fee = (protocol_fee - referral_fee) * partner_fee_percentage / denominator
#[derive(Clone, Debug, BorshDeserialize, BorshSerialize, PartialEq, serde::Serialize, serde::Deserialize, Default)]
#[cfg_attr(feature = "bytemuck", derive(Zeroable, Pod))]
#[repr(C)]
pub struct PoolFeesStruct {
    /// Trade fees are extra token amounts that are held inside the token
    /// accounts during a trade, making the value of liquidity tokens rise.
    /// Trade fee numerator
    pub base_fee: BaseFeeStruct,
    /// Protocol trading fees are extra token amounts that are held inside the token
    /// accounts during a trade, with the equivalent in pool tokens minted to
    /// the protocol of the program.
    /// Protocol trade fee numerator
    pub protocol_fee_percent: u8,
    /// partner fee
    pub partner_fee_percent: u8,
    /// referral fee
    pub referral_fee_percent: u8,
    /// padding
    pub padding_0: [u8; 5],
    /// dynamic fee
    pub dynamic_fee: DynamicFeeStruct,
    pub init_sqrt_price: u128,
}

#[derive(Clone, Debug, BorshDeserialize, BorshSerialize, PartialEq, serde::Serialize, serde::Deserialize, Default)]
#[cfg_attr(feature = "bytemuck", derive(Zeroable, Pod))]
#[repr(C)]
pub struct PoolMetrics {
    pub total_lp_a_fee: u128,
    pub total_lp_b_fee: u128,
    pub total_protocol_a_fee: u64,
    pub total_protocol_b_fee: u64,
    pub total_partner_a_fee: u64,
    pub total_partner_b_fee: u64,
    pub total_position: u64,
    pub padding: u64,
}

#[derive(Clone, Debug, BorshDeserialize, BorshSerialize, PartialEq, serde::Serialize, serde::Deserialize, Default)]
#[cfg_attr(feature = "bytemuck", derive(Zeroable, Pod))]
#[repr(C)]
pub struct Position {
    pub pool: Pubkey,
    /// nft mint
    pub nft_mint: Pubkey,
    /// fee a checkpoint
    pub fee_a_per_token_checkpoint: [u8; 32],
    /// fee b checkpoint
    pub fee_b_per_token_checkpoint: [u8; 32],
    /// fee a pending
    pub fee_a_pending: u64,
    /// fee b pending
    pub fee_b_pending: u64,
    /// unlock liquidity
    pub unlocked_liquidity: u128,
    /// vesting liquidity
    pub vested_liquidity: u128,
    /// permanent locked liquidity
    pub permanent_locked_liquidity: u128,
    /// metrics
    pub metrics: PositionMetrics,
    /// Farming reward information
    pub reward_infos: [UserRewardInfo; 2],
    /// inner vesting info
    pub inner_vesting: InnerVesting,
    /// padding for future usage
    pub padding: u128,
}

#[derive(Clone, Debug, BorshDeserialize, BorshSerialize, PartialEq, serde::Serialize, serde::Deserialize, Default)]
#[cfg_attr(feature = "bytemuck", derive(Zeroable, Pod))]
#[repr(C)]
pub struct PositionMetrics {
    pub total_claimed_a_fee: u64,
    pub total_claimed_b_fee: u64,
}

#[derive(Clone, Debug, BorshDeserialize, BorshSerialize, PartialEq, serde::Serialize, serde::Deserialize, Default)]
pub struct RemoveLiquidityParameters {
    /// delta liquidity
    pub liquidity_delta: u128,
    /// minimum token a amount
    pub token_a_amount_threshold: u64,
    /// minimum token b amount
    pub token_b_amount_threshold: u64,
}

/// Stores the state relevant for tracking liquidity mining rewards
#[derive(Clone, Debug, BorshDeserialize, BorshSerialize, PartialEq, serde::Serialize, serde::Deserialize, Default)]
#[cfg_attr(feature = "bytemuck", derive(Zeroable, Pod))]
#[repr(C)]
pub struct RewardInfo {
    /// Indicates if the reward has been initialized
    pub initialized: u8,
    /// reward token flag
    pub reward_token_flag: u8,
    /// padding
    pub _padding_0: [u8; 6],
    /// Padding to ensure `reward_rate: u128` is 16-byte aligned
    pub _padding_1: [u8; 8],
    /// Reward token mint.
    pub mint: Pubkey,
    /// Reward vault token account.
    pub vault: Pubkey,
    /// Authority account that allows to fund rewards
    pub funder: Pubkey,
    /// reward duration
    pub reward_duration: u64,
    /// reward duration end
    pub reward_duration_end: u64,
    /// reward rate
    pub reward_rate: u128,
    /// Reward per token stored
    pub reward_per_token_stored: [u8; 32],
    /// The last time reward states were updated.
    pub last_update_time: u64,
    /// Accumulated seconds when the farm distributed rewards but the bin was empty.
    /// These rewards will be carried over to the next reward time window.
    pub cumulative_seconds_with_empty_liquidity_reward: u64,
}

#[derive(Clone, Debug, BorshDeserialize, BorshSerialize, PartialEq, serde::Serialize, serde::Deserialize, Default)]
pub struct SplitAmountInfo {
    pub permanent_locked_liquidity: u128,
    pub unlocked_liquidity: u128,
    pub fee_a: u64,
    pub fee_b: u64,
    pub reward_0: u64,
    pub reward_1: u64,
}

#[derive(Clone, Debug, BorshDeserialize, BorshSerialize, PartialEq, serde::Serialize, serde::Deserialize, Default)]
pub struct SplitAmountInfo2 {
    pub permanent_locked_liquidity: u128,
    pub unlocked_liquidity: u128,
    pub vested_liquidity: u128,
    pub fee_a: u64,
    pub fee_b: u64,
    pub reward_0: u64,
    pub reward_1: u64,
}

#[derive(Clone, Debug, BorshDeserialize, BorshSerialize, PartialEq, serde::Serialize, serde::Deserialize, Default)]
pub struct SplitPositionInfo {
    pub liquidity: u128,
    pub fee_a: u64,
    pub fee_b: u64,
    pub reward_0: u64,
    pub reward_1: u64,
}

#[derive(Clone, Debug, BorshDeserialize, BorshSerialize, PartialEq, serde::Serialize, serde::Deserialize, Default)]
pub struct SplitPositionInfo2 {
    pub unlocked_liquidity: u128,
    pub permanent_locked_liquidity: u128,
    pub vested_liquidity: u128,
    pub fee_a: u64,
    pub fee_b: u64,
    pub reward_0: u64,
    pub reward_1: u64,
}

#[derive(Clone, Debug, BorshDeserialize, BorshSerialize, PartialEq, serde::Serialize, serde::Deserialize, Default)]
pub struct SplitPositionParameters {
    /// Percentage of unlocked liquidity to split to the second position
    pub unlocked_liquidity_percentage: u8,
    /// Percentage of permanent locked liquidity to split to the second position
    pub permanent_locked_liquidity_percentage: u8,
    /// Percentage of fee A pending to split to the second position
    pub fee_a_percentage: u8,
    /// Percentage of fee B pending to split to the second position
    pub fee_b_percentage: u8,
    /// Percentage of reward 0 pending to split to the second position
    pub reward_0_percentage: u8,
    /// Percentage of reward 1 pending to split to the second position
    pub reward_1_percentage: u8,
    /// Percentage of inner vesting liquidity
    pub inner_vesting_liquidity_percentage: u8,
    /// padding for future
    pub padding: [u8; 15],
}

#[derive(Clone, Debug, BorshDeserialize, BorshSerialize, PartialEq, serde::Serialize, serde::Deserialize, Default)]
pub struct SplitPositionParameters2 {
    pub unlocked_liquidity_numerator: u32,
    pub permanent_locked_liquidity_numerator: u32,
    pub fee_a_numerator: u32,
    pub fee_b_numerator: u32,
    pub reward_0_numerator: u32,
    pub reward_1_numerator: u32,
}

#[derive(Clone, Debug, BorshDeserialize, BorshSerialize, PartialEq, serde::Serialize, serde::Deserialize, Default)]
pub struct SplitPositionParameters3 {
    pub unlocked_liquidity_numerator: u32,
    pub permanent_locked_liquidity_numerator: u32,
    pub fee_a_numerator: u32,
    pub fee_b_numerator: u32,
    pub reward_0_numerator: u32,
    pub reward_1_numerator: u32,
    pub inner_vesting_liquidity_numerator: u32,
}

#[derive(Clone, Debug, BorshDeserialize, BorshSerialize, PartialEq, serde::Serialize, serde::Deserialize, Default)]
pub struct StaticConfigParameters {
    pub pool_fees: PoolFeeParameters,
    pub sqrt_min_price: u128,
    pub sqrt_max_price: u128,
    pub vault_config_key: Pubkey,
    pub pool_creator_authority: Pubkey,
    pub activation_type: u8,
    pub collect_fee_mode: u8,
}

#[derive(Clone, Debug, BorshDeserialize, BorshSerialize, PartialEq, serde::Serialize, serde::Deserialize, Default)]
pub struct SwapParameters {
    pub amount_in: u64,
    pub minimum_amount_out: u64,
}

#[derive(Clone, Debug, BorshDeserialize, BorshSerialize, PartialEq, serde::Serialize, serde::Deserialize, Default)]
pub struct SwapParameters2 {
    /// When it's exact in, partial fill, this will be amount_in. When it's exact out, this will be amount_out
    pub amount_0: u64,
    /// When it's exact in, partial fill, this will be minimum_amount_out. When it's exact out, this will be maximum_amount_in
    pub amount_1: u64,
    /// Swap mode, refer [SwapMode]
    pub swap_mode: u8,
}

#[derive(Clone, Debug, BorshDeserialize, BorshSerialize, PartialEq, serde::Serialize, serde::Deserialize, Default)]
pub struct SwapResult2 {
    pub included_fee_input_amount: u64,
    pub excluded_fee_input_amount: u64,
    pub amount_left: u64,
    pub output_amount: u64,
    pub next_sqrt_price: u128,
    pub trading_fee: u64,
    pub protocol_fee: u64,
    pub partner_fee: u64,
    pub referral_fee: u64,
}

/// Parameter that set by the protocol
#[derive(Clone, Debug, BorshDeserialize, BorshSerialize, PartialEq, serde::Serialize, serde::Deserialize, Default)]
pub struct TokenBadge {
    /// token mint
    pub token_mint: Pubkey,
    /// Reserve
    pub _padding: Vec<u8>,
}

#[derive(Clone, Debug, BorshDeserialize, BorshSerialize, PartialEq, serde::Serialize, serde::Deserialize, Default)]
pub struct UpdatePoolFeesParameters {
    /// Base fee update mode:
    /// - None: skip base fee update
    /// - Some: update new cliff_fee_numerator if base fee is static
    pub cliff_fee_numerator: Option<u64>,
    /// Dynamic fee update mode:
    /// - None: skip dynamic fee update
    /// - Some(with default value): disable dynamic fee
    /// - Some(with non default value): enable dynamic fee if disabled or update dynamic fee if enabled
    pub dynamic_fee: Option<DynamicFeeParameters>,
}

#[derive(Clone, Debug, BorshDeserialize, BorshSerialize, PartialEq, serde::Serialize, serde::Deserialize, Default)]
pub struct UserRewardInfo {
    /// The latest update reward checkpoint
    pub reward_per_token_checkpoint: [u8; 32],
    /// Current pending rewards
    pub reward_pendings: u64,
    /// Total claimed rewards
    pub total_claimed_rewards: u64,
}

#[derive(Clone, Debug, BorshDeserialize, BorshSerialize, PartialEq, serde::Serialize, serde::Deserialize, Default)]
pub struct Vesting {
    pub position: Pubkey,
    pub inner_vesting: InnerVesting,
    pub padding2: [u128; 4],
}

#[derive(Clone, Debug, BorshDeserialize, BorshSerialize, PartialEq, serde::Serialize, serde::Deserialize, Default)]
pub struct VestingParameters {
    pub cliff_point: Option<u64>,
    pub period_frequency: u64,
    pub cliff_unlock_liquidity: u128,
    pub liquidity_per_period: u128,
    pub number_of_period: u16,
}