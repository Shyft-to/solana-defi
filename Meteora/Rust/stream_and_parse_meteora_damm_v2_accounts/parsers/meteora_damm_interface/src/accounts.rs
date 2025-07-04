use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::pubkey::Pubkey;
use serde::{Serialize, Deserialize};
use crate::{
    RewardInfo,
    UserRewardInfo,
    PoolFeesConfig,
    PoolFeesStruct,
    PoolMetrics,
    PositionMetrics,
};
// For TokenBadge _padding field
//use serde_bytes;

pub const CLAIM_FEE_OPERATOR_ACCOUNT_DISCM: [u8; 8] = [166, 48, 134, 86, 34, 200, 188, 150];

#[derive(Clone, Debug, BorshDeserialize, BorshSerialize, PartialEq, Serialize, Deserialize)]
pub struct ClaimFeeOperator {
    pub operator: Pubkey,
    pub _padding: Vec<u8>,
}
#[derive(Clone, Debug, PartialEq)]
pub struct ClaimFeeOperatorAccount(pub ClaimFeeOperator);
impl ClaimFeeOperatorAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != CLAIM_FEE_OPERATOR_ACCOUNT_DISCM {
            return Err(
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!(
                        "discm does not match. Expected: {:?}. Received: {:?}",
                        CLAIM_FEE_OPERATOR_ACCOUNT_DISCM, maybe_discm
                    ),
                ),
            );
        }
        Ok(Self(<ClaimFeeOperator as BorshDeserialize>::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&CLAIM_FEE_OPERATOR_ACCOUNT_DISCM)?;
        BorshSerialize::serialize(&self.0, &mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}

// Config
pub const CONFIG_ACCOUNT_DISCM: [u8; 8] = [155, 12, 170, 224, 30, 250, 204, 130];
#[derive(Clone, Debug, BorshDeserialize, BorshSerialize, PartialEq, Serialize, Deserialize)]
pub struct Config {
    pub vault_config_key: Pubkey,
    pub pool_creator_authority: Pubkey,
    pub pool_fees: PoolFeesConfig,
    pub activation_type: u8,
    pub collect_fee_mode: u8,
    pub config_type: u8,
    pub _padding_0: [u8; 5],
    pub index: u64,
    pub sqrt_min_price: u128,
    pub sqrt_max_price: u128,
    pub _padding_1: [u64; 10],
}
#[derive(Clone, Debug, PartialEq)]
pub struct ConfigAccount(pub Config);
impl ConfigAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != CONFIG_ACCOUNT_DISCM {
            return Err(
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!(
                        "discm does not match. Expected: {:?}. Received: {:?}",
                        CONFIG_ACCOUNT_DISCM, maybe_discm
                    ),
                ),
            );
        }
        Ok(Self(<Config as BorshDeserialize>::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&CONFIG_ACCOUNT_DISCM)?;
        BorshSerialize::serialize(&self.0, &mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}

// Pool
pub const POOL_ACCOUNT_DISCM: [u8; 8] = [241, 154, 109, 4, 17, 177, 109, 188];
#[derive(Clone, Debug, BorshDeserialize, BorshSerialize, PartialEq, Serialize, Deserialize)]
pub struct Pool {
    pub pool_fees: PoolFeesStruct,
    pub token_a_mint: Pubkey,
    pub token_b_mint: Pubkey,
    pub token_a_vault: Pubkey,
    pub token_b_vault: Pubkey,
    pub whitelisted_vault: Pubkey,
    pub partner: Pubkey,
    pub liquidity: u128,
    pub _padding: u128,
    pub protocol_a_fee: u64,
    pub protocol_b_fee: u64,
    pub partner_a_fee: u64,
    pub partner_b_fee: u64,
    pub sqrt_min_price: u128,
    pub sqrt_max_price: u128,
    pub sqrt_price: u128,
    pub activation_point: u64,
    pub activation_type: u8,
    pub pool_status: u8,
    pub token_a_flag: u8,
    pub token_b_flag: u8,
    pub collect_fee_mode: u8,
    pub pool_type: u8,
    pub _padding_0: [u8; 2],
    pub fee_a_per_liquidity: [u8; 32],
    pub fee_b_per_liquidity: [u8; 32],
    pub permanent_lock_liquidity: u128,
    pub metrics: PoolMetrics,
    pub _padding_1: [u64; 10],
    pub reward_infos: [RewardInfo; 2],
}
#[derive(Clone, Debug, PartialEq)]
pub struct PoolAccount(pub Pool);
impl PoolAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != POOL_ACCOUNT_DISCM {
            return Err(
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!(
                        "discm does not match. Expected: {:?}. Received: {:?}",
                        POOL_ACCOUNT_DISCM, maybe_discm
                    ),
                ),
            );
        }
        Ok(Self(<Pool as BorshDeserialize>::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&POOL_ACCOUNT_DISCM)?;
        BorshSerialize::serialize(&self.0, &mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}

// Position
pub const POSITION_ACCOUNT_DISCM: [u8; 8] = [170, 188, 143, 228, 122, 64, 247, 208];
#[derive(Clone, Debug, BorshDeserialize, BorshSerialize, PartialEq, Serialize, Deserialize)]
pub struct Position {
    pub pool: Pubkey,
    pub nft_mint: Pubkey,
    pub fee_a_per_token_checkpoint: [u8; 32],
    pub fee_b_per_token_checkpoint: [u8; 32],
    pub fee_a_pending: u64,
    pub fee_b_pending: u64,
    pub unlocked_liquidity: u128,
    pub vested_liquidity: u128,
    pub permanent_locked_liquidity: u128,
    pub metrics: PositionMetrics,
    pub reward_infos: [UserRewardInfo; 2],
    pub padding: [u128; 6],
}
#[derive(Clone, Debug, PartialEq)]
pub struct PositionAccount(pub Position);
impl PositionAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != POSITION_ACCOUNT_DISCM {
            return Err(
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!(
                        "discm does not match. Expected: {:?}. Received: {:?}",
                        POSITION_ACCOUNT_DISCM, maybe_discm
                    ),
                ),
            );
        }
        Ok(Self(<Position as BorshDeserialize>::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&POSITION_ACCOUNT_DISCM)?;
        BorshSerialize::serialize(&self.0, &mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}

// TokenBadge
pub const TOKEN_BADGE_ACCOUNT_DISCM: [u8; 8] = [116, 219, 204, 229, 249, 116, 255, 150];
#[derive(Clone, Debug, BorshDeserialize, BorshSerialize, PartialEq, Serialize, Deserialize)]
pub struct TokenBadge {
    pub token_mint: Pubkey,
    pub _padding: Vec<u8>,
}
#[derive(Clone, Debug, PartialEq)]
pub struct TokenBadgeAccount(pub TokenBadge);
impl TokenBadgeAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != TOKEN_BADGE_ACCOUNT_DISCM {
            return Err(
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!(
                        "discm does not match. Expected: {:?}. Received: {:?}",
                        TOKEN_BADGE_ACCOUNT_DISCM, maybe_discm
                    ),
                ),
            );
        }
        Ok(Self(<TokenBadge as BorshDeserialize>::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&TOKEN_BADGE_ACCOUNT_DISCM)?;
        BorshSerialize::serialize(&self.0, &mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}

// Vesting
pub const VESTING_ACCOUNT_DISCM: [u8; 8] = [100, 149, 66, 138, 95, 200, 128, 241];
#[derive(Clone, Debug, BorshDeserialize, BorshSerialize, PartialEq, Serialize, Deserialize)]
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
#[derive(Clone, Debug, PartialEq)]
pub struct VestingAccount(pub Vesting);
impl VestingAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != VESTING_ACCOUNT_DISCM {
            return Err(
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!(
                        "discm does not match. Expected: {:?}. Received: {:?}",
                        VESTING_ACCOUNT_DISCM, maybe_discm
                    ),
                ),
            );
        }
        Ok(Self(<Vesting as BorshDeserialize>::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&VESTING_ACCOUNT_DISCM)?;
        BorshSerialize::serialize(&self.0, &mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}