use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::pubkey::Pubkey;
use serde_with::serde_as;
use serde_with::DisplayFromStr;
use crate::typedefs::{TickState,Observation,RewardInfo};
pub const AMM_CONFIG_ACCOUNT_DISCM: [u8; 8] = [218, 244, 33, 104, 203, 203, 43, 111];
#[derive(Clone, Debug, BorshDeserialize, BorshSerialize, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct AmmConfig {
   pub bump: u8, 
    pub index: u16,
    pub owner: Pubkey, 
    pub protocol_fee_rate: u32, 
    pub trade_fee_rate: u32, 
    pub tick_spacing: u16, 
    pub fund_fee_rate: u32,
    pub padding_u32: u32,
    pub fund_owner: Pubkey,
    pub padding: [u64; 3],
}
#[derive(Clone, Debug, PartialEq)]
pub struct AmmConfigAccount(pub AmmConfig);
impl AmmConfigAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != AMM_CONFIG_ACCOUNT_DISCM {
            return Err(
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!(
                        "discm does not match. Expected: {:?}. Received: {:?}",
                        AMM_CONFIG_ACCOUNT_DISCM, maybe_discm
                    ),
                ),
            );
        }
        Ok(Self(AmmConfig::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&AMM_CONFIG_ACCOUNT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const OBSERVATION_STATE_ACCOUNT_DISCM: [u8; 8] = [
    122,
    174,
    197,
    53,
    129,
    9,
    165,
    132,
];
#[derive(Clone, Debug, BorshDeserialize, BorshSerialize, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ObservationState {
    pub initialized: bool,
    pub observation_index: u16,
    pub pool_id: Pubkey,
    pub observations: Vec<Observation>,
    pub padding: [u64; 4],
}
#[derive(Clone, Debug, PartialEq)]
pub struct ObservationStateAccount(pub ObservationState);
impl ObservationStateAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != OBSERVATION_STATE_ACCOUNT_DISCM {
            return Err(
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!(
                        "discm does not match. Expected: {:?}. Received: {:?}",
                        OBSERVATION_STATE_ACCOUNT_DISCM, maybe_discm
                    ),
                ),
            );
        }
        Ok(Self(ObservationState::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&OBSERVATION_STATE_ACCOUNT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const POOL_STATE_ACCOUNT_DISCM: [u8; 8] = [247, 237, 227, 245, 215, 195, 222, 70];
#[serde_as]
#[derive(Clone, Debug, BorshDeserialize, BorshSerialize, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
  pub struct PoolState {
    pub bump: [u8; 1],
    pub amm_config: Pubkey,
    pub owner: Pubkey,
    pub token_mint_0: Pubkey,
    pub token_mint_1: Pubkey,
    pub token_vault_0: Pubkey,
    pub token_vault_1: Pubkey,
    pub observation_key: Pubkey,
    pub mint_decimals_0: u8,
    pub mint_decimals_1: u8,
    pub tick_spacing: u16,
    #[serde_as(as = "DisplayFromStr")]
    pub liquidity: u128,
    #[serde_as(as = "DisplayFromStr")]
    pub sqrt_price_x64: u128,
    pub tick_current: i32,
    pub padding3: u16,
    pub padding4: u16,
    #[serde_as(as = "DisplayFromStr")]
    pub fee_growth_global_0_x64: u128,
    #[serde_as(as = "DisplayFromStr")]
    pub fee_growth_global_1_x64: u128,
    pub protocol_fees_token_0: u64,
    pub protocol_fees_token_1: u64,
    #[serde_as(as = "DisplayFromStr")]
    pub swap_in_amount_token_0: u128,
    #[serde_as(as = "DisplayFromStr")]
    pub swap_out_amount_token_1: u128,
    #[serde_as(as = "DisplayFromStr")]
    pub swap_in_amount_token_1: u128,
    #[serde_as(as = "DisplayFromStr")]
    pub swap_out_amount_token_0: u128,
    pub status: u8,
    pub padding: Vec<u8>,
    pub reward_infos: [RewardInfo; 3],
    pub tick_array_bitmap: [u64; 16],
    pub total_fees_token_0: u64,
    pub total_fees_claimed_token_0: u64,
    pub total_fees_token_1: u64,
    pub total_fees_claimed_token_1: u64,
    pub fund_fees_token_0: u64,
    pub fund_fees_token_1: u64,
    pub open_time: u64,
    pub recent_epoch: u64,
    pub padding1: Vec<u8>,
    pub padding2: Vec<u8>,
}
#[derive(Clone, Debug, PartialEq)]
pub struct PoolStateAccount(pub PoolState);
impl PoolStateAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != POOL_STATE_ACCOUNT_DISCM {
            return Err(
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!(
                        "discm does not match. Expected: {:?}. Received: {:?}",
                        POOL_STATE_ACCOUNT_DISCM, maybe_discm
                    ),
                ),
            );
        }
        Ok(Self(PoolState::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&POOL_STATE_ACCOUNT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}

pub const TICK_ARRAY_STATE_DISCM: [u8; 8] = [192, 155, 85, 205, 49, 249, 129, 42];
#[serde_as]
#[derive(Clone, Debug, BorshDeserialize, BorshSerialize, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct TickArrayState {
    pub tick: i32,
    #[serde_as(as = "DisplayFromStr")]
    pub liquidity_net: i128,
    #[serde_as(as = "DisplayFromStr")]
    pub liquidity_gross: u128,
    #[serde_as(as = "DisplayFromStr")]
    pub fee_growth_outside_0_x64: u128,
    #[serde_as(as = "DisplayFromStr")]
    pub fee_growth_outside_1_x64: u128,
    #[serde_as(as = "Vec<DisplayFromStr>")]
    pub reward_growths_outside_x64: Vec<u128>,
    pub padding: Vec<u32>,
}
#[derive(Clone, Debug, PartialEq)]
pub struct TickArrayStateAccount(pub TickArrayState);
impl TickArrayStateAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != TICK_ARRAY_STATE_DISCM {
            return Err(
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!(
                        "discm does not match. Expected: {:?}. Received: {:?}",
                        TICK_ARRAY_STATE_DISCM, maybe_discm
                    ),
                ),
            );
        }
        Ok(Self(TickArrayState::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&TICK_ARRAY_STATE_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
