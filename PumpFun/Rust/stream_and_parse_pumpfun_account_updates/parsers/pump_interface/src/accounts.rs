use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::pubkey::Pubkey;

// Account discriminators
pub const GLOBAL_ACCOUNT_DISCM: [u8; 8] = [167, 232, 232, 177, 200, 108, 114, 127];
pub const BONDING_CURVE_ACCOUNT_DISCM: [u8; 8] = [23, 183, 248, 55, 96, 216, 172, 96];
pub const FEE_CONFIG_ACCOUNT_DISCM: [u8; 8] = [143, 52, 146, 187, 219, 123, 76, 155];
pub const GLOBAL_VOLUME_ACCUMULATOR_ACCOUNT_DISCM: [u8; 8] = [202, 42, 246, 43, 142, 190, 30, 255];
pub const SHARING_CONFIG_ACCOUNT_DISCM: [u8; 8] = [216, 74, 9, 0, 56, 140, 93, 75];
pub const USER_VOLUME_ACCUMULATOR_ACCOUNT_DISCM: [u8; 8] = [86, 255, 112, 14, 102, 53, 154, 250];

// Types
#[derive(Clone, Debug, BorshDeserialize, BorshSerialize, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum ConfigStatus {
    Paused,
    Active,
}

#[derive(Clone, Debug, BorshDeserialize, BorshSerialize, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Fees {
    pub lp_fee_bps: u64,
    pub protocol_fee_bps: u64,
    pub creator_fee_bps: u64,
}

#[derive(Clone, Debug, BorshDeserialize, BorshSerialize, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FeeTier {
    pub market_cap_lamports_threshold: u128,
    pub fees: Fees,
}

#[derive(Clone, Debug, BorshDeserialize, BorshSerialize, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Shareholder {
    pub address: Pubkey,
    pub share_bps: u16,
}

// Global Account
#[derive(Clone, Debug, BorshDeserialize, BorshSerialize, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Global {
    pub initialized: bool,
    pub authority: Pubkey,
    pub fee_recipient: Pubkey,
    pub initial_virtual_token_reserves: u64,
    pub initial_virtual_sol_reserves: u64,
    pub initial_real_token_reserves: u64,
    pub token_total_supply: u64,
    pub fee_basis_points: u64,
    pub withdraw_authority: Pubkey,
    pub enable_migrate: bool,
    pub pool_migration_fee: u64,
    pub creator_fee_basis_points: u64,
    pub fee_recipients: [Pubkey; 7],
    pub set_creator_authority: Pubkey,
    pub admin_set_creator_authority: Pubkey,
    pub create_v2_enabled: bool,
    pub whitelist_pda: Pubkey,
    pub reserved_fee_recipient: Pubkey,
    pub mayhem_mode_enabled: bool,
    pub reserved_fee_recipients: [Pubkey; 7],
}

#[derive(Clone, Debug, PartialEq)]
pub struct GlobalAccount(pub Global);

impl GlobalAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != GLOBAL_ACCOUNT_DISCM {
            return Err(
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!(
                        "discm does not match. Expected: {:?}. Received: {:?}",
                        GLOBAL_ACCOUNT_DISCM, maybe_discm
                    ),
                ),
            );
        }
        Ok(Self(Global::deserialize(&mut reader)?))
    }

    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&GLOBAL_ACCOUNT_DISCM)?;
        self.0.serialize(&mut writer)
    }

    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}

// BondingCurve Account
#[derive(Clone, Debug, BorshDeserialize, BorshSerialize, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct BondingCurve {
    pub virtual_token_reserves: u64,
    pub virtual_sol_reserves: u64,
    pub real_token_reserves: u64,
    pub real_sol_reserves: u64,
    pub token_total_supply: u64,
    pub complete: bool,
    pub creator: Pubkey,
    pub is_mayhem_mode: bool,
}

#[derive(Clone, Debug, PartialEq)]
pub struct BondingCurveAccount(pub BondingCurve);

impl BondingCurveAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != BONDING_CURVE_ACCOUNT_DISCM {
            return Err(
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!(
                        "discm does not match. Expected: {:?}. Received: {:?}",
                        BONDING_CURVE_ACCOUNT_DISCM, maybe_discm
                    ),
                ),
            );
        }
        Ok(Self(BondingCurve::deserialize(&mut reader)?))
    }

    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&BONDING_CURVE_ACCOUNT_DISCM)?;
        self.0.serialize(&mut writer)
    }

    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}

// FeeConfig Account
#[derive(Clone, Debug, BorshDeserialize, BorshSerialize, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FeeConfig {
    pub bump: u8,
    pub admin: Pubkey,
    pub flat_fees: Fees,
    pub fee_tiers: Vec<FeeTier>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct FeeConfigAccount(pub FeeConfig);

impl FeeConfigAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != FEE_CONFIG_ACCOUNT_DISCM {
            return Err(
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!(
                        "discm does not match. Expected: {:?}. Received: {:?}",
                        FEE_CONFIG_ACCOUNT_DISCM, maybe_discm
                    ),
                ),
            );
        }
        Ok(Self(FeeConfig::deserialize(&mut reader)?))
    }

    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&FEE_CONFIG_ACCOUNT_DISCM)?;
        self.0.serialize(&mut writer)
    }

    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}

// GlobalVolumeAccumulator Account
#[derive(Clone, Debug, BorshDeserialize, BorshSerialize, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct GlobalVolumeAccumulator {
    pub start_time: i64,
    pub end_time: i64,
    pub seconds_in_a_day: i64,
    pub mint: Pubkey,
    pub total_token_supply: [u64; 30],
    pub sol_volumes: [u64; 30],
}

#[derive(Clone, Debug, PartialEq)]
pub struct GlobalVolumeAccumulatorAccount(pub GlobalVolumeAccumulator);

impl GlobalVolumeAccumulatorAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != GLOBAL_VOLUME_ACCUMULATOR_ACCOUNT_DISCM {
            return Err(
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!(
                        "discm does not match. Expected: {:?}. Received: {:?}",
                        GLOBAL_VOLUME_ACCUMULATOR_ACCOUNT_DISCM, maybe_discm
                    ),
                ),
            );
        }
        Ok(Self(GlobalVolumeAccumulator::deserialize(&mut reader)?))
    }

    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&GLOBAL_VOLUME_ACCUMULATOR_ACCOUNT_DISCM)?;
        self.0.serialize(&mut writer)
    }

    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}

// SharingConfig Account
#[derive(Clone, Debug, BorshDeserialize, BorshSerialize, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SharingConfig {
    pub bump: u8,
    pub version: u8,
    pub status: ConfigStatus,
    pub mint: Pubkey,
    pub admin: Pubkey,
    pub admin_revoked: bool,
    pub shareholders: Vec<Shareholder>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct SharingConfigAccount(pub SharingConfig);

impl SharingConfigAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != SHARING_CONFIG_ACCOUNT_DISCM {
            return Err(
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!(
                        "discm does not match. Expected: {:?}. Received: {:?}",
                        SHARING_CONFIG_ACCOUNT_DISCM, maybe_discm
                    ),
                ),
            );
        }
        Ok(Self(SharingConfig::deserialize(&mut reader)?))
    }

    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&SHARING_CONFIG_ACCOUNT_DISCM)?;
        self.0.serialize(&mut writer)
    }

    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}

// UserVolumeAccumulator Account
#[derive(Clone, Debug, BorshDeserialize, BorshSerialize, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct UserVolumeAccumulator {
    pub user: Pubkey,
    pub needs_claim: bool,
    pub total_unclaimed_tokens: u64,
    pub total_claimed_tokens: u64,
    pub current_sol_volume: u64,
    pub last_update_timestamp: i64,
    pub has_total_claimed_tokens: bool,
}

#[derive(Clone, Debug, PartialEq)]
pub struct UserVolumeAccumulatorAccount(pub UserVolumeAccumulator);

impl UserVolumeAccumulatorAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != USER_VOLUME_ACCUMULATOR_ACCOUNT_DISCM {
            return Err(
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!(
                        "discm does not match. Expected: {:?}. Received: {:?}",
                        USER_VOLUME_ACCUMULATOR_ACCOUNT_DISCM, maybe_discm
                    ),
                ),
            );
        }
        Ok(Self(UserVolumeAccumulator::deserialize(&mut reader)?))
    }

    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&USER_VOLUME_ACCUMULATOR_ACCOUNT_DISCM)?;
        self.0.serialize(&mut writer)
    }

    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
