use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::pubkey::Pubkey;
#[cfg(feature = "bytemuck")]
use bytemuck::{Pod, Zeroable};

#[derive(Clone, Debug, BorshDeserialize, BorshSerialize, PartialEq,serde::Serialize, serde::Deserialize, Default)]
pub struct AddLiquidityParameters {
    /// delta liquidity
    pub liquidity_delta: u128,
    /// maximum token a amount
    pub token_a_amount_threshold: u64,
    /// maximum token b amount
    pub token_b_amount_threshold: u64,
}

#[derive(Clone, Debug, BorshDeserialize, BorshSerialize, PartialEq,serde::Serialize, serde::Deserialize, Default)]
pub struct BaseFeeConfig {
    pub cliff_fee_numerator: u64,
    pub fee_scheduler_mode: u8,
    pub padding: [u8; 5],
    pub number_of_period: u16,
    pub period_frequency: u64,
    pub reduction_factor: u64,
}

#[derive(Clone, Debug, BorshDeserialize, BorshSerialize, PartialEq,serde::Serialize, serde::Deserialize, Default)]
pub struct BaseFeeParameters {
    pub cliff_fee_numerator: u64,
    pub number_of_period: u16,
    pub period_frequency: u64,
    pub reduction_factor: u64,
    pub fee_scheduler_mode: u8,
}

#[derive(Clone, Debug, BorshDeserialize, BorshSerialize, PartialEq,serde::Serialize, serde::Deserialize, Default)]
pub struct BaseFeeStruct {
    pub cliff_fee_numerator: u64,
    pub fee_scheduler_mode: u8,
    pub padding_0: [u8; 5],
    pub number_of_period: u16,
    pub period_frequency: u64,
    pub reduction_factor: u64,
    pub padding_1: u64,
}

#[derive(Clone, Debug, BorshDeserialize, BorshSerialize, PartialEq)]
pub struct ClaimFeeOperator {
    /// operator
    pub operator: Pubkey,
    /// Reserve
    pub _padding: [u8; 128],
}

#[derive(Clone, Debug, BorshDeserialize, BorshSerialize, PartialEq,Default)]
pub struct Config {
    /// Vault config key
    pub vault_config_key: Pubkey,
    /// Only pool_creator_authority can use the current config to initialize new pool. When it's Pubkey::default, it's a public config.
    pub pool_creator_authority: Pubkey,    /// Pool fee
    pub pool_fees: PoolFeesConfig,
    /// Activation type
    pub activation_type: u8,
    /// Collect fee mode
    pub collect_fee_mode: u8,
    /// Config type mode, 0 for static, 1 for dynamic
    pub config_type: Pubkey,
    /// config index
    pub index: u8,
    /// sqrt min price
    pub sqrt_min_price: u128,
    /// sqrt max price
    pub sqrt_max_price: u128,
    /// Fee curve point, padding for further use
    pub _padding_1: [u64; 10],
}

#[derive(Clone, Debug, BorshDeserialize, BorshSerialize, PartialEq,serde::Serialize, serde::Deserialize, Default)]
pub struct DynamicConfigParameters {
    pool_creator: Pubkey
}
#[derive(Clone, Debug, BorshDeserialize, BorshSerialize, PartialEq,serde::Serialize, serde::Deserialize, Default)]
pub struct DynamicFeeConfig {
    pub initialized: u8,
    pub padding: [u8; 7],
    pub max_volatility: u32,
    pub variable_fee_control: u32,
    pub bin_step: u16,
    pub filter_period: u16,
    pub decay_period: u16,
    pub reduction_factor: u16,
    pub _padding_1: [u64; 8],
    pub bin_step_u128: u128,
}

#[derive(Clone, Debug, BorshDeserialize, BorshSerialize, PartialEq,serde::Serialize, serde::Deserialize, Default)]
pub struct DynamicFeeParameters {
    pub bin_step: u16,
    pub bin_step_u128: u128,
    pub filter_period: u16,
    pub decay_period: u16,
    pub reduction_factor: u16,
    pub max_volatility: u32,
    pub variable_fee_control: u32,
}

#[derive(Clone, Debug, BorshDeserialize, BorshSerialize, PartialEq,serde::Serialize, serde::Deserialize, Default)]
pub struct DynamicFeeStruct {
    pub initialized: u8,
    pub padding: [u8; 7],
    pub max_volatility_accumulator: u32,
    pub variable_fee: u32,
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

#[derive(Clone, Debug, BorshDeserialize, BorshSerialize, PartialEq,serde::Serialize, serde::Deserialize, Default)]
pub struct EvtAddLiquidity {
    pub pool: Pubkey,
    pub position: Pubkey,
    pub owner: Pubkey,
    pub params: AddLiquidityParameters,
    pub token_a_amount: u64,
    pub token_b_amount: u64,
    pub total_amount_a: u64,
    pub total_amount_b: u64,
}

#[derive(Clone, Debug, BorshDeserialize, BorshSerialize, PartialEq,serde::Serialize, serde::Deserialize, Default)]
pub struct EvtClaimPartnerFee {
    pub pool: Pubkey,
    pub token_a_amount: u64,
    pub token_b_amount: u64,
}

#[derive(Clone, Debug, BorshDeserialize, BorshSerialize, PartialEq,serde::Serialize, serde::Deserialize, Default)]
pub struct EvtClaimPositionFee {
    pub pool: u64,
    pub position: u64,
    pub owner: u64,
    pub fee_a_claimed: u64,
    pub fee_b_claimed: u64,
}

#[derive(Clone, Debug, BorshDeserialize, BorshSerialize, PartialEq,serde::Serialize, serde::Deserialize, Default)]
pub struct EvtClaimProtocolFee {
    pub pool: Pubkey,
    pub token_a_amount: u64,
    pub token_b_amount: u64,
}

#[derive(Clone, Debug, BorshDeserialize, BorshSerialize, PartialEq,serde::Serialize, serde::Deserialize, Default)]
pub struct EvtClaimReward {
    pub pool: Pubkey,
    pub position: Pubkey,
    pub owner: Pubkey,
    pub mint_reward: Pubkey,
    pub reward_index: u8,
    pub total_reward: u64,
}

#[derive(Clone, Debug, BorshDeserialize, BorshSerialize, PartialEq,serde::Serialize, serde::Deserialize, Default)]
pub struct EvtCloseClaimFeeOperator {
    /// Close claim fee operator
    pub claim_fee_operator: Pubkey,
    pub operator: Pubkey,
}

#[derive(Clone, Debug, BorshDeserialize, BorshSerialize, PartialEq,serde::Serialize, serde::Deserialize, Default)]
pub struct EvtCloseConfig {
    /// Config pubkey
    pub config: Pubkey,
    /// admin pk
    pub admin: Pubkey,
}

#[derive(Clone, Debug, BorshDeserialize, BorshSerialize, PartialEq,serde::Serialize, serde::Deserialize, Default)]
pub struct EvtClosePosition {
    pub pool: Pubkey,
    pub owner: Pubkey,
    pub position: Pubkey,
    pub position_nft_mint: Pubkey,
}

#[derive(Clone, Debug, BorshDeserialize, BorshSerialize, PartialEq,serde::Serialize, serde::Deserialize, Default)]
pub struct EvtCreateClaimFeeOperator {
    pub operator: Pubkey,
}

#[derive(Clone, Debug, BorshDeserialize, BorshSerialize, PartialEq,serde::Serialize, serde::Deserialize, Default)]
pub struct EvtCreateConfig {
    pub pool_fees: PoolFeeParameters,
    pub vault_config_key: Pubkey,
    pub pool_creator_authority: Pubkey,
    pub activation_type: u8,
    pub sqrt_min_price: u128,
    pub sqrt_max_price: u128,
    pub collect_fee_mode: u8,
    pub index: u64,
    pub config: Pubkey,
}

#[derive(Clone, Debug, BorshDeserialize, BorshSerialize, PartialEq,serde::Serialize, serde::Deserialize, Default)]
pub struct EvtCreateDynamicConfig {
    pub config: Pubkey,
    pub pool_creator_authority: Pubkey,
    pub index: u64,
}

#[derive(Clone, Debug, BorshDeserialize, BorshSerialize, PartialEq,serde::Serialize, serde::Deserialize, Default)]
pub struct EvtCreatePosition {
    pub pool: Pubkey,
    pub owner: Pubkey,
    pub position: Pubkey,
    pub position_nft_mint: Pubkey,
}

#[derive(Clone, Debug, BorshDeserialize, BorshSerialize, PartialEq,serde::Serialize, serde::Deserialize, Default)]
pub struct EvtCreateTokenBadge {
    pub token_mint: Pubkey,
}

#[derive(Clone, Debug, BorshDeserialize, BorshSerialize, PartialEq,serde::Serialize, serde::Deserialize, Default)]
pub struct EvtFundReward {
    pub pool: Pubkey,
    pub funder: Pubkey,
    pub mint_reward: Pubkey,
    pub reward_index: u8,
    pub amount: u64,
    pub transfer_fee_excluded_amount_in: u64,
}

#[derive(Clone, Debug, BorshDeserialize, BorshSerialize, PartialEq,serde::Serialize, serde::Deserialize, Default)]
pub struct EvtInitializePool {
    pub pool: Pubkey,
    pub token_a_mint: Pubkey,
    pub token_b_mint: Pubkey,
    pub creator: Pubkey,
    pub payer: Pubkey,
    pub alpha_vault: Pubkey,
    pub pool_fees: PoolFeeParameters,
    pub sqrt_min_price: u128,
    pub sqrt_max_price: u128,
    pub activation_type: u8,
    pub collect_fee_mode: u8,
    pub liquidity: u128,
    pub sqrt_price: u128,
    pub activation_point: u64,
    pub token_a_flag: u8,
    pub token_b_flag: u8,
    pub token_a_amount: u64,
    pub token_b_amount: u64,
    pub total_amount_a: u64,
    pub total_amount_b: u64,
    pub pool_type: u8,
}

#[derive(Clone, Debug, BorshDeserialize, BorshSerialize, PartialEq,serde::Serialize, serde::Deserialize, Default)]
pub struct EvtInitializeReward {
    pub pool: Pubkey,
    pub reward_mint: Pubkey,
    pub funder: Pubkey,
    pub reward_index: u8,
    pub reward_duration: u64,
}

#[derive(Clone, Debug, BorshDeserialize, BorshSerialize, PartialEq,serde::Serialize, serde::Deserialize, Default)]
pub struct EvtLockPosition {
    pub pool: Pubkey,
    pub position: Pubkey,
    pub owner: Pubkey,
    pub vesting: Pubkey,
    pub cliff_point: u64,
    pub period_frequency: u64,
    pub cliff_unlock_liquidity: u128,
    pub liquidity_per_period: u128,
    pub number_of_period: u16,
}

#[derive(Clone, Debug, BorshDeserialize, BorshSerialize, PartialEq,serde::Serialize, serde::Deserialize, Default)]
pub struct EvtPermanentLockPosition {
    pub pool: Pubkey,
    pub position: Pubkey,
    pub lock_liquidity_amount: u128,
    pub total_permanent_locked_liquidity: u128,
}

#[derive(Clone, Debug, BorshDeserialize, BorshSerialize, PartialEq,serde::Serialize, serde::Deserialize, Default)]
pub struct EvtRemoveLiquidity {
    pub pool: Pubkey,
    pub position: Pubkey,
    pub owner: Pubkey,
    pub params: RemoveLiquidityParameters,
    pub token_a_amount: u64,
    pub token_b_amount: u64,
}

#[derive(Clone, Debug, BorshDeserialize, BorshSerialize, PartialEq,serde::Serialize, serde::Deserialize, Default)]
pub struct EvtSetPoolStatus {
    pub pool: Pubkey,
    pub status: u8,
}

#[derive(Clone, Debug, BorshDeserialize, BorshSerialize, PartialEq,serde::Serialize, serde::Deserialize, Default)]
pub struct EvtSwap {
    pub pool: Pubkey,
    pub trade_direction: u8,
    pub has_referral: bool,
    pub params: SwapParameters,
    pub swap_result: SwapResult,
    pub actual_amount_in: u64,
    pub current_timestamp: u64,
}

#[derive(Clone, Debug, BorshDeserialize, BorshSerialize, PartialEq,serde::Serialize, serde::Deserialize, Default)]
pub struct EvtUpdateRewardDuration {
    pub pool: Pubkey,
    pub reward_index: u8,
    pub old_reward_duration: u64,
    pub new_reward_duration: u64,
}

#[derive(Clone, Debug, BorshDeserialize, BorshSerialize, PartialEq,serde::Serialize, serde::Deserialize, Default)]
pub struct EvtUpdateRewardFunder {
    pub pool: Pubkey,
    pub reward_index: u8,
    pub old_funder: Pubkey,
    pub new_funder: Pubkey,
}

#[derive(Clone, Debug, BorshDeserialize, BorshSerialize, PartialEq,serde::Serialize, serde::Deserialize, Default)]
pub struct EvtWithdrawIneligibleReward {
    pub pool: Pubkey,
    pub reward_mint: Pubkey,
    pub amount: u64,
}

#[derive(Clone, Debug, BorshDeserialize, BorshSerialize, PartialEq,serde::Serialize, serde::Deserialize, Default)]
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
    /// The init price of the pool as a sqrt(token_b/token_a) Q64.64 value
    pub sqrt_price: u128,
    /// activation type
    pub activation_type: u8,
    /// collect fee mode
    pub collect_fee_mode: u8,
    /// activation point
    pub activation_point: Option<u64>,
}

#[derive(Clone, Debug, BorshDeserialize, BorshSerialize, PartialEq,serde::Serialize, serde::Deserialize, Default)]
pub struct InitializePoolParameters {
    /// initialize liquidity
    pub liquidity: u128,
    /// The init price of the pool as a sqrt(token_b/token_a) Q64.64 value
    pub sqrt_price: u128,
    /// activation point
    pub activation_point: Option<u64>,
}

#[derive(Clone, Debug, BorshDeserialize, BorshSerialize, PartialEq,serde::Serialize, serde::Deserialize, Default)]
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
    /// 0 is collect fee in both token, 1 only collect fee in token a, 2 only collect fee in token b
    pub collect_fee_mode: u8,
    /// pool type
    pub pool_type: u8,
    /// padding
    pub _padding_0: [u8; 2],
    /// cumulative
    pub fee_a_per_liquidity: [u8; 32],
    /// cumulative
    pub fee_b_per_liquidity: [u8; 32],
    pub permanent_lock_liquidity: u128,
    /// metrics
    pub metrics: PoolMetrics,
    /// Padding for further use
    pub _padding_1: [u64; 10],
    /// Farming reward information
    pub reward_infos: [RewardInfo; 2],
}

#[derive(Clone, Debug, BorshDeserialize, BorshSerialize, PartialEq,serde::Serialize, serde::Deserialize, Default)]
pub struct PoolFeeParameters {
    /// Base fee
    pub base_fee: BaseFeeParameters,
    /// Protocol trade fee percent
    pub protocol_fee_percent: u8,
    /// partner fee percent
    pub partner_fee_percent: u8,
    /// referral fee percent
    pub referral_fee_percent: u8,
    /// dynamic fee
    pub dynamic_fee: Option<DynamicFeeParameters>,
}

#[derive(Clone, Debug, BorshDeserialize, BorshSerialize, PartialEq,serde::Serialize, serde::Deserialize, Default)]
pub struct PoolFeesConfig {
    pub base_fee: BaseFeeConfig,
    pub dynamic_fee: DynamicFeeConfig,
    pub protocol_fee_percent: u8,
    pub partner_fee_percent: u8,
    pub referral_fee_percent: u8,
    pub padding_0: [u8; 5],
    pub padding_1: [u64; 5],
}
#[derive(Clone, Debug, BorshDeserialize, BorshSerialize, PartialEq,serde::Serialize, serde::Deserialize, Default)]
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
    /// padding
    pub padding_1: [u64; 2],
}

#[derive(Clone, Debug, BorshDeserialize, BorshSerialize, PartialEq,serde::Serialize, serde::Deserialize, Default)]
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
#[derive(Clone, Debug, BorshDeserialize, BorshSerialize, PartialEq,serde::Serialize, serde::Deserialize, Default)]
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
    /// padding for future usage
    pub padding: [u128; 6],
}

#[derive(Clone, Debug, BorshDeserialize, BorshSerialize, PartialEq,serde::Serialize, serde::Deserialize, Default)]
pub struct PositionMetrics {
    pub total_claimed_a_fee: u64,
    pub total_claimed_b_fee: u64,
}

#[derive(Clone, Debug, BorshDeserialize, BorshSerialize, PartialEq,serde::Serialize, serde::Deserialize, Default)]
pub struct RemoveLiquidityParameters {
    /// delta liquidity
    pub liquidity_delta: u128,
    /// minimum token a amount
    pub token_a_amount_threshold: u64,
    /// minimum token b amount
    pub token_b_amount_threshold: u64,
}

#[derive(Clone, Debug, BorshDeserialize, BorshSerialize, PartialEq,serde::Serialize, serde::Deserialize, Default)]
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

#[derive(Clone, Debug, BorshDeserialize, BorshSerialize, PartialEq,serde::Serialize, serde::Deserialize, Default)]
pub struct StaticConfigParameters {
    pub pool_fees: PoolFeeParameters,
    pub sqrt_min_price: u128,
    pub sqrt_max_price: u128,
    pub vault_config_key: Pubkey,
    pub pool_creator_authority: Pubkey,
    pub activation_type: u8,
    pub collect_fee_mode: u8,
}

#[derive(Clone, Debug, BorshDeserialize, BorshSerialize, PartialEq,serde::Serialize, serde::Deserialize, Default)]
pub struct SwapParameters {
    pub amount_in: u64,
    pub minimum_amount_out: u64,
}

#[derive(Clone, Debug, BorshDeserialize, BorshSerialize, PartialEq,serde::Serialize, serde::Deserialize, Default)]
pub struct SwapResult {
    pub output_amount: u64,
    pub next_sqrt_price: u128,
    pub lp_fee: u64,
    pub protocol_fee: u64,
    pub partner_fee: u64,
    pub referral_fee: u64,
}

#[derive(Clone, Debug, BorshDeserialize, BorshSerialize, PartialEq)]
pub struct TokenBadge {
    /// token mint
    pub token_mint: Pubkey,
    /// Reserve
    pub _padding: [u8; 128],
}

#[derive(Clone, Debug, BorshDeserialize, BorshSerialize, PartialEq,serde::Serialize, serde::Deserialize, Default)]
pub struct UserRewardInfo {
    /// The latest update reward checkpoint
    pub reward_per_token_checkpoint: [u8; 32],
    /// Current pending rewards
    pub reward_pendings: u64,
    /// Total claimed rewards
    pub total_claimed_rewards: u64,
}

#[derive(Clone, Debug, BorshDeserialize, BorshSerialize, PartialEq,serde::Serialize, serde::Deserialize, Default)]
pub struct Vesting {
    pub position: Pubkey,
    pub cliff_point: u64,
    pub period_frequency: u64,
    pub cliff_unlock_liquidity: u128,
    pub liquidity_per_period: u128,
    pub total_released_liquidity: u128,
    pub number_of_period: u16,
    pub padding: [u8; 14],
    pub padding2: [u128; 4],
}

#[derive(Clone, Debug, BorshDeserialize, BorshSerialize, PartialEq,serde::Serialize, serde::Deserialize, Default)]
pub struct VestingParameters {
    pub cliff_point: Option<u64>,
    pub period_frequency: u64,
    pub cliff_unlock_liquidity: u128,
    pub liquidity_per_period: u128,
    pub number_of_period: u16,
}