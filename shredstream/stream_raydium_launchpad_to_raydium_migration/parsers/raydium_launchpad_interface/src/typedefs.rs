use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::pubkey::Pubkey;

#[derive(Clone, Debug, BorshDeserialize, BorshSerialize, PartialEq,serde::Serialize, serde::Deserialize, Default)]
pub struct ClaimVestedEvent {
    pub pool_state: Pubkey,
    pub beneficiary: Pubkey,
    pub claim_amount: u64,
}

#[derive(Clone, Debug, BorshDeserialize, BorshSerialize, PartialEq,serde::Serialize, serde::Deserialize, Default)]
pub struct ConstantCurve {
    pub supply: u64,
    pub total_base_sell: u64,
    pub total_quote_fund_raising: u64,
    pub migrate_type: u8,
}

#[derive(Clone, Debug, BorshDeserialize, BorshSerialize, PartialEq,serde::Serialize, serde::Deserialize, Default)]
pub struct CreateVestingEvent {
    pub pool_state: Pubkey,
    pub beneficiary: Pubkey,
    pub share_amount: u64,
}

#[derive(Clone, Debug, BorshDeserialize, BorshSerialize, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum CurveParams {
    Constant { data: ConstantCurve },
    Fixed { data: FixedCurve },
    Linear { data: LinearCurve },
}

impl Default for CurveParams {
    fn default() -> Self {
        CurveParams::Constant {
            data: ConstantCurve::default(),
        }
    }
}

#[derive(Clone, Debug, BorshDeserialize, BorshSerialize, PartialEq,serde::Serialize, serde::Deserialize, Default)]
pub struct FixedCurve {
    pub supply: u64,
    pub total_quote_fund_raising: u64,
    pub migrate_type: u8,
}

#[derive(Clone, Debug, BorshDeserialize, BorshSerialize, PartialEq,serde::Serialize, serde::Deserialize, Default)]
pub struct GlobalConfig {
    pub epoch: u64,
    pub curve_type: u8,
    pub index: u16,
    pub migrate_fee: u64,
    pub trade_fee_rate: u64,
    pub max_share_fee_rate: u64,
    pub min_base_supply: u64,
    pub max_lock_rate: u64,
    pub min_base_sell_rate: u64,
    pub min_base_migrate_rate: u64,
    pub min_quote_fund_raising: u64,
    pub quote_mint: Pubkey,
    pub protocol_fee_owner: Pubkey,
    pub migrate_fee_owner: Pubkey,
    pub migrate_to_amm_wallet: Pubkey,
    pub migrate_to_cpswap_wallet: Pubkey,
    pub padding: Vec<u8>,
}

#[derive(Clone, Debug, BorshDeserialize, BorshSerialize, PartialEq,serde::Serialize, serde::Deserialize, Default)]
pub struct LinearCurve {
    pub supply: u64,
    pub total_quote_fund_raising: u64,
    pub migrate_type: u8,
}

#[derive(Clone, Debug, BorshDeserialize, BorshSerialize, PartialEq,serde::Serialize, serde::Deserialize, Default)]
pub struct MigrateNftInfo {
    pub platform_scale: u64,
    pub creator_scale: u64,
    pub burn_scale: u64,
}

#[derive(Clone, Debug, BorshDeserialize, BorshSerialize, PartialEq,serde::Serialize, serde::Deserialize, Default)]
pub struct MintParams {
    pub decimals: u8,
    pub name: String,
    pub symbol: String,
    pub uri: String,
}

#[derive(Clone, Debug, BorshDeserialize, BorshSerialize, PartialEq,serde::Serialize, serde::Deserialize, Default)]
pub struct PlatformConfig {
    pub epoch: u64,
    pub platform_fee_wallet: Pubkey,
    pub platform_nft_wallet: Pubkey,
    pub platform_scale: u64,
    pub creator_scale: u64,
    pub burn_scale: u64,
    pub fee_rate: u64,
    pub name: Vec<u8>,
    pub web: Vec<u8>,
    pub img: Vec<u8>,
    pub padding: Vec<u8>,
}

#[derive(Clone, Debug, BorshDeserialize, BorshSerialize, PartialEq,serde::Serialize, serde::Deserialize, Default)]
pub enum PlatformConfigParam {
    FeeWallet(Pubkey),
    NFTWallet(Pubkey),
    MigrateNftInfo(MigrateNftInfo),
    FeeRate(u64),
    Name(String),
    Web(String),
    Img(String),
     #[default]
    None,
}

#[derive(Clone, Debug, BorshDeserialize, BorshSerialize, PartialEq,serde::Serialize, serde::Deserialize, Default)]
pub struct PlatformParams {
    pub migrate_nft_info: MigrateNftInfo,
    pub fee_rate: u64,
    pub name: String,
    pub web: String,
    pub img: String,
}

#[derive(Clone, Debug, BorshDeserialize, BorshSerialize, PartialEq,serde::Serialize, serde::Deserialize, Default)]
pub struct PoolCreateEvent {
    pub pool_state: Pubkey,
    pub creator: Pubkey,
    pub config: Pubkey,
    pub base_mint_param: MintParams,
    pub curve_param: CurveParams,
    pub vesting_param: VestingParams,
}

#[derive(Clone, Debug, BorshDeserialize, BorshSerialize, PartialEq,serde::Serialize, serde::Deserialize, Default)]
pub struct PoolState {
    pub epoch: u64,
    pub auth_bump: u8,
    pub status: u8,
    pub base_decimals: u8,
    pub quote_decimals: u8,
    pub migrate_type: u8,
    pub supply: u64,
    pub total_base_sell: u64,
    pub virtual_base: u64,
    pub virtual_quote: u64,
    pub real_base: u64,
    pub real_quote: u64,
    pub total_quote_fund_raising: u64,
    pub quote_protocol_fee: u64,
    pub platform_fee: u64,
    pub migrate_fee: u64,
    pub vesting_schedule: VestingSchedule,
    pub global_config: Pubkey,
    pub platform_config: Pubkey,
    pub base_mint: Pubkey,
    pub quote_mint: Pubkey,
    pub base_vault: Pubkey,
    pub quote_vault: Pubkey,
    pub creator: Pubkey,
    pub padding: Vec<u8>,
}

#[derive(Clone, Debug, BorshDeserialize, BorshSerialize, PartialEq,serde::Serialize, serde::Deserialize, Default)]
pub enum PoolStatus {
    #[default] 
      Fund,
     Migrate,
     Trade,
}

#[derive(Clone, Debug, BorshDeserialize, BorshSerialize, PartialEq,serde::Serialize, serde::Deserialize, Default)]
pub enum TradeDirection {
    #[default] 
     Buy,
     Sell,
}

#[derive(Clone, Debug, BorshDeserialize, BorshSerialize, PartialEq,serde::Serialize, serde::Deserialize, Default)]
pub struct TradeEvent {
    pub pool_state: Pubkey,
    pub total_base_sell: u64,
    pub virtual_base: u64,
    pub virtual_quote: u64,
    pub real_base_before: u64,
    pub real_quote_before: u64,
    pub real_base_after: u64,
    pub real_quote_after: u64,
    pub amount_in: u64,
    pub amount_out: u64,
    pub protocol_fee: u64,
    pub platform_fee: u64,
    pub share_fee: u64,
    pub trade_direction: TradeDirection,
    pub pool_status: PoolStatus,
}

#[derive(Clone, Debug, BorshDeserialize, BorshSerialize, PartialEq,serde::Serialize, serde::Deserialize, Default)]
pub struct VestingParams {
    pub total_locked_amount: u64,
    pub cliff_period: u64,
    pub unlock_period: u64,
}

#[derive(Clone, Debug, BorshDeserialize, BorshSerialize, PartialEq,serde::Serialize, serde::Deserialize, Default)]
pub struct VestingRecord {
    pub epoch: u64,
    pub pool: Pubkey,
    pub beneficiary: Pubkey,
    pub claimed_amount: u64,
    pub token_share_amount: u64,
    pub padding: Vec<u8>,
}

#[derive(Clone, Debug, BorshDeserialize, BorshSerialize, PartialEq,serde::Serialize, serde::Deserialize, Default)]
pub struct VestingSchedule {
    pub total_locked_amount: u64,
    pub cliff_period: u64,
    pub unlock_period: u64,
    pub start_time: u64,
    pub allocated_share_amount: u64,
}