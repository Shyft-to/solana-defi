use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::pubkey::Pubkey;
use serde::{Serialize, Deserialize};
use crate::{
    PoolFees,PoolFeesConfig,LockedVestingConfig,
    LiquidityDistributionConfig,PoolMetrics,
    VolatilityTracker
};

pub const CLAIM_FEE_OPERATOR_ACCOUNT_DISCM: [u8; 8] = [166, 48, 134, 86, 34, 200, 188, 150];
#[derive(Clone, Debug, BorshDeserialize, BorshSerialize, PartialEq, Serialize, Deserialize)]
pub struct ClaimFeeOperator {
    pub operator: Pubkey,
    pub _padding: [u8;12],
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

pub const CONFIG_ACCOUNT_DISCM: [u8; 8] = [155, 12, 170, 224, 30, 250, 204, 130];
#[derive(Clone, Debug, BorshDeserialize, BorshSerialize, PartialEq, Serialize, Deserialize)]
pub struct Config {
    pub pool_fees: PoolFees,
    pub activation_duration: u64,
    pub vault_config_key: Pubkey,
    pub pool_creator_authority: Pubkey,
    pub activation_type: u8,
    pub partner_fee_numerator: u64,
    pub padding: Vec<u8>,
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

pub const LOCK_ESCROW_ACCOUNT_DISCM: [u8; 8] = [190, 106, 121, 6, 200, 182, 21, 75];
#[derive(Clone, Debug, BorshDeserialize, BorshSerialize, PartialEq, Serialize, Deserialize)]
pub struct LockEscrow {
    pub pool: Pubkey,
    pub owner: Pubkey,
    pub escrow_vault: Pubkey,
    pub bump: u8,
    pub total_locked_amount: u64,
    pub lp_per_token: u128,
    pub unclaimed_fee_pending: u64,
    pub a_fee: u64,
    pub b_fee: u64,
}

#[derive(Clone, Debug, PartialEq)]
pub struct LockEscrowAccount(pub LockEscrow);

impl LockEscrowAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != LOCK_ESCROW_ACCOUNT_DISCM {
            return Err(
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!(
                        "discm does not match. Expected: {:?}. Received: {:?}",
                        LOCK_ESCROW_ACCOUNT_DISCM, maybe_discm
                    ),
                ),
            );
        }
        Ok(Self(<LockEscrow as BorshDeserialize>::deserialize(&mut reader)?))
    }

    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&LOCK_ESCROW_ACCOUNT_DISCM)?;
        BorshSerialize::serialize(&self.0, &mut writer)
    }

    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}

pub const METEORA_DAMM_MIGRATION_METADATA_ACCOUNT_DISCM: [u8; 8] = [17, 155, 141, 215, 207, 4, 133, 156];
#[derive(Clone, Debug, BorshDeserialize, BorshSerialize, PartialEq, Serialize, Deserialize)]
pub struct MeteoraDammMigrationMetadata {
    pub virtual_pool: Pubkey,
    pub padding_0: [u8; 32],
    pub partner: Pubkey,
    pub lp_mint: Pubkey,
    pub partner_locked_lp: u64,
    pub partner_lp: u64,
    pub creator_locked_lp: u64,
    pub creator_lp: u64,
    pub _padding_0: u8,
    pub creator_locked_status: u8,
    pub partner_locked_status: u8,
    pub creator_claim_status: u8,
    pub partner_claim_status: u8,
    pub _padding: Vec<u8>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct MeteoraDammMigrationMetadataAccount(pub MeteoraDammMigrationMetadata);

impl MeteoraDammMigrationMetadataAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != METEORA_DAMM_MIGRATION_METADATA_ACCOUNT_DISCM {
            return Err(
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!(
                        "discm does not match. Expected: {:?}. Received: {:?}",
                        METEORA_DAMM_MIGRATION_METADATA_ACCOUNT_DISCM, maybe_discm
                    ),
                ),
            );
        }
        Ok(Self(<MeteoraDammMigrationMetadata as BorshDeserialize>::deserialize(&mut reader)?))
    }

    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&METEORA_DAMM_MIGRATION_METADATA_ACCOUNT_DISCM)?;
        BorshSerialize::serialize(&self.0, &mut writer)
    }

    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}

pub const METEORA_DAMM_V2_METADATA_ACCOUNT_DISCM: [u8; 8] = [104, 221, 219, 203, 10, 142, 250, 163];
#[derive(Clone, Debug, BorshDeserialize, BorshSerialize, PartialEq, Serialize, Deserialize)]
pub struct MeteoraDammV2Metadata {
    pub virtual_pool: Pubkey,
    pub padding_0: [u8; 32],
    pub partner: Pubkey,
    pub _padding: Vec<u8>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct MeteoraDammV2MetadataAccount(pub MeteoraDammV2Metadata);

impl MeteoraDammV2MetadataAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != METEORA_DAMM_V2_METADATA_ACCOUNT_DISCM {
            return Err(
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!(
                        "discm does not match. Expected: {:?}. Received: {:?}",
                        METEORA_DAMM_V2_METADATA_ACCOUNT_DISCM, maybe_discm
                    ),
                ),
            );
        }       
        Ok(Self(<MeteoraDammV2Metadata as BorshDeserialize>::deserialize(&mut reader)?))
    }

    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&METEORA_DAMM_V2_METADATA_ACCOUNT_DISCM)?;
        BorshSerialize::serialize(&self.0, &mut writer)
    }

    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}

pub const PARTNER_METADATA_ACCOUNT_DISCM: [u8; 8] = [68, 68, 130, 19, 16, 209, 98, 156];
#[derive(Clone, Debug, BorshDeserialize, BorshSerialize, PartialEq, Serialize, Deserialize)]
pub struct PartnerMetadata {
    pub fee_claimer: Pubkey,
    pub padding: Vec<u128>,
    pub name: String,
    pub website: String,
    pub logo: String,
}

#[derive(Clone, Debug, PartialEq)]
pub struct PartnerMetadataAccount(pub PartnerMetadata);

impl PartnerMetadataAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != PARTNER_METADATA_ACCOUNT_DISCM {
            return Err(
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!(
                        "discm does not match. Expected: {:?}. Received: {:?}",
                        PARTNER_METADATA_ACCOUNT_DISCM, maybe_discm
                    ),
                ),
            );
        }
        Ok(Self(<PartnerMetadata as BorshDeserialize>::deserialize(&mut reader)?))
    }

    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&PARTNER_METADATA_ACCOUNT_DISCM)?;
        BorshSerialize::serialize(&self.0, &mut writer)
    }

    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}

pub const POOL_CONFIG_ACCOUNT_DISCM: [u8; 8] = [26, 108, 14, 123, 116, 230, 129, 43];
#[derive(Clone, Debug, BorshDeserialize, BorshSerialize, PartialEq, Serialize, Deserialize)]
pub struct PoolConfig {
    pub quote_mint: Pubkey,
    pub fee_claimer: Pubkey,
    pub leftover_receiver: Pubkey,
    pub pool_fees: PoolFeesConfig,
    pub collect_fee_mode: u8,
    pub migration_option: u8,
    pub activation_type: u8,
    pub token_decimal: u8,
    pub version: u8,
    pub token_type: u8,
    pub quote_token_flag: u8,
    pub partner_locked_lp_percentage: u8,
    pub partner_lp_percentage: u8,
    pub creator_locked_lp_percentage: u8,
    pub creator_lp_percentage: u8,
    pub migration_fee_option: u8,
    pub fixed_token_supply_flag: u8,
    pub creator_trading_fee_percentage: u8,
    pub token_update_authority: u8,
    pub migration_fee_percentage: u8,
    pub creator_migration_fee_percentage: u8,
    pub _padding_0: Vec<u8>,
    pub swap_base_amount: u64,
    pub migration_quote_threshold: u64,
    pub migration_base_threshold: u64,
    pub migration_sqrt_price: u128,
    pub locked_vesting_config: LockedVestingConfig,
    pub pre_migration_token_supply: u64,
    pub post_migration_token_supply: u64,
    pub migrated_collect_fee_mode: u8,
    pub migrated_dynamic_fee: u8,
    pub migrated_pool_fee_bps: u16,
    pub _padding_1: Vec<u8>,
    pub _padding_2: u128,
    pub sqrt_start_price: u128,
    pub curve: Vec<LiquidityDistributionConfig>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct PoolConfigAccount(pub PoolConfig);

impl PoolConfigAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != POOL_CONFIG_ACCOUNT_DISCM {
            return Err(
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!(
                        "discm does not match. Expected: {:?}. Received: {:?}",
                        POOL_CONFIG_ACCOUNT_DISCM, maybe_discm
                    ),
                ),
            );
        }
        Ok(Self(<PoolConfig as BorshDeserialize>::deserialize(&mut reader)?))
    }

    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&POOL_CONFIG_ACCOUNT_DISCM)?;
        BorshSerialize::serialize(&self.0, &mut writer)
    }

    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}

pub const VIRTUAL_POOL_ACCOUNT_DISCM: [u8; 8] = [213, 224, 5, 209, 98, 69, 119, 92];
#[derive(Clone, Debug, BorshDeserialize, BorshSerialize, PartialEq)]
#[repr(C)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct VirtualPool {
    pub volatility_tracker: VolatilityTracker,
    pub config: Pubkey,
    pub creator: Pubkey,
    pub base_mint: Pubkey,
    pub base_vault: Pubkey,
    pub quote_vault: Pubkey,
    pub base_reserve: u64,
    pub quote_reserve: u64,
    pub protocol_base_fee: u64,
    pub protocol_quote_fee: u64,
    pub partner_base_fee: u64,
    pub partner_quote_fee: u64,
    pub sqrt_price: u128,
    pub activation_point: u64,
    pub pool_type: u8,
    pub is_migrated: u8,
    pub is_partner_withdraw_surplus: u8,
    pub is_protocol_withdraw_surplus: u8,
    pub migration_progress: u8,
    pub is_withdraw_leftover: u8,
    pub is_creator_withdraw_surplus: u8,
    pub migration_fee_withdraw_status: u8,
    pub metrics: PoolMetrics,
    pub finish_curve_timestamp: u64,
    pub creator_base_fee: u64,
    pub creator_quote_fee: u64,
    pub _padding_1: [u64;7],
}

#[derive(Clone, Debug, PartialEq)]
pub struct VirtualPoolAccount(pub VirtualPool);

impl VirtualPoolAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != VIRTUAL_POOL_ACCOUNT_DISCM {
            return Err(
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!(
                        "discm does not match. Expected: {:?}. Received: {:?}",
                        VIRTUAL_POOL_ACCOUNT_DISCM, maybe_discm
                    ),
                ),
            );
        } 

        Ok(Self(<VirtualPool as BorshDeserialize>::deserialize(&mut reader)?))
    }

    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&VIRTUAL_POOL_ACCOUNT_DISCM)?;
        BorshSerialize::serialize(&self.0, &mut writer)
    }

    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}


pub const VIRTUAL_POOL_METADATA_ACCOUNT_DISCM: [u8; 8] = [217, 37, 82, 250, 43, 47, 228, 254];
#[derive(Clone, Debug, BorshDeserialize, BorshSerialize, PartialEq, Serialize, Deserialize)]
pub struct VirtualPoolMetadata {
    pub virtual_pool: Pubkey,
    pub padding: Vec<u128>,
    pub name: String,
    pub website: String,
    pub logo: String,
}

#[derive(Clone, Debug, PartialEq)]
pub struct VirtualPoolMetadataAccount(pub VirtualPoolMetadata);

impl VirtualPoolMetadataAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != VIRTUAL_POOL_METADATA_ACCOUNT_DISCM {
            return Err(
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!(
                        "discm does not match. Expected: {:?}. Received: {:?}",
                        VIRTUAL_POOL_METADATA_ACCOUNT_DISCM, maybe_discm
                    ),
                ),
            );
        }

        Ok(Self(<VirtualPoolMetadata as BorshDeserialize>::deserialize(&mut reader)?))
    }

    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&VIRTUAL_POOL_METADATA_ACCOUNT_DISCM)?;
        BorshSerialize::serialize(&self.0, &mut writer)
    }

    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
