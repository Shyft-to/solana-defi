use borsh::{BorshDeserialize, BorshSerialize};
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]

#[derive(Clone, Debug, BorshDeserialize, BorshSerialize, PartialEq,Default)]
pub struct InitializeRewardParam {
    /// Reward open time
    pub open_time: u64,
    /// Reward end time
    pub end_time: u64,
    /// Token reward per second are earned per unit of liquidity
    pub emissions_per_second_x64: u128,
}

#[derive(Clone, Debug, BorshDeserialize, BorshSerialize, PartialEq)]
pub struct Observation {
    /// The block timestamp of the observation
    pub block_timestamp: u32,
    /// the cumulative of tick during the duration time
    pub tick_cumulative: i64,
    /// padding for feature update
    pub padding: [u64; 4],
}

#[derive(Clone, Debug, BorshDeserialize, BorshSerialize, PartialEq)]
pub struct PositionRewardInfo {
    pub growth_inside_last_x64: u128,
    pub reward_amount_owed: u64,
}

#[derive(Clone, Debug, BorshDeserialize, BorshSerialize, PartialEq)]
pub struct RewardInfo {
    /// Reward state
    pub reward_state: u8,
    /// Reward open time
    pub open_time: u64,
    /// Reward end time
    pub end_time: u64,
    /// Reward last update time
    pub last_update_time: u64,
    /// Q64.64 number indicates how many tokens per second are earned per unit of liquidity.
    pub emissions_per_second_x64: u128,
    /// The total amount of reward emissioned
    pub reward_total_emissioned: u64,
    /// The total amount of claimed reward
    pub reward_claimed: u64,
    /// Reward token mint.
    pub token_mint: [u8; 32],
    /// Reward vault token account.
    pub token_vault: [u8; 32],
    /// The owner that has permission to set reward param
    pub authority: [u8; 32],
    /// Q64.64 number that tracks the total tokens earned per unit of liquidity since the reward emissions were turned on.
    pub reward_growth_global_x64: u128,
}

#[derive(Clone, Debug, BorshDeserialize, BorshSerialize, PartialEq)]
pub struct TickState {
    pub tick: i32,
    /// Amount of net liquidity added/subtracted
    pub liquidity_net: i128,
    /// Total position liquidity
    pub liquidity_gross: u128,
    /// Fee growth outside 0
    pub fee_growth_outside_0_x64: u128,
    /// Fee growth outside 1
    pub fee_growth_outside_1_x64: u128,
    pub reward_growths_outside_x64: [u128; 3],
    pub padding: [u32; 13],
}

#[derive(Clone, Debug, BorshDeserialize, BorshSerialize, PartialEq)]
pub enum PoolStatusBitIndex {
    OpenPositionOrIncreaseLiquidity,
    DecreaseLiquidity,
    CollectFee,
    CollectReward,
    Swap,
}

#[derive(Clone, Debug, BorshDeserialize, BorshSerialize, PartialEq)]
pub enum PoolStatusBitFlag {
    Enable,
    Disable,
}

#[derive(Clone, Debug, BorshDeserialize, BorshSerialize, PartialEq)]
pub enum RewardState {
    Uninitialized,
    Initialized,
    Opening,
    Ended,
}

pub type TickArryBitmap = [u64; 8];
