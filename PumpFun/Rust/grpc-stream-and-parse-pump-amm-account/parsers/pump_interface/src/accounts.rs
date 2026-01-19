use borsh::{BorshDeserialize, BorshSerialize};
use crate::{Fees,FeeTier};
use solana_program::pubkey::Pubkey;

pub const GLOBAL_CONFIG_ACCOUNT_DISCM: [u8; 8] = [149, 8, 156, 202, 160, 252, 176, 217];
pub const BONDING_CURVE_ACCOUNT_DISCM: [u8; 8] = [23,183,248,55,96,216,172,96];
pub const POOL_ACCOUNT_DISCM: [u8; 8] = [241, 154, 109, 4, 17, 177, 109, 188];
pub const FEE_CONFIG_ACCOUNT_DISCM: [u8; 8] = [143, 52, 146, 187, 219, 123, 76, 155];
pub const GLOBAL_VOLUME_ACCUMULATOR_ACCOUNT_DISCM: [u8; 8] = [202, 42, 246, 43, 142, 190, 30, 255];
pub const USER_VOLUME_ACCUMULATOR_ACCOUNT_DISCM: [u8; 8] = [86, 255, 112, 14, 102, 53, 154, 250];

#[derive(Clone, Debug, BorshDeserialize, BorshSerialize, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct BondingCurve {
    pub virtual_token_reserves: u64,
    pub virtual_sol_reserves: u64,
    pub real_token_reserves: u64,
    pub real_sol_reserves: u64,
    pub token_total_supply: u64,
    pub complete : bool,
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

#[derive(Clone, Debug, BorshDeserialize, BorshSerialize, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct GlobalConfig {
    pub admin: Pubkey,

    pub lp_fee_basis_points: u64,

    pub protocol_fee_basis_points: u64,
    pub disable_flags: u8,
    pub protocol_fee_recipients: [Pubkey; 8],
    
    pub coin_creator_fee_basis_points: u64,
    pub admin_set_coin_creator_authority: Pubkey,

    pub whitelist_pda: Pubkey,
    pub reserved_fee_recipient: Pubkey,

    pub mayhem_mode_enabled: bool,

    pub reserved_fee_recipients: [Pubkey; 7],
}

#[derive(Clone, Debug, PartialEq)]
pub struct GlobalConfigAccount(pub GlobalConfig);

impl GlobalConfigAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != GLOBAL_CONFIG_ACCOUNT_DISCM {
            return Err(
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!(
                        "discm does not match. Expected: {:?}. Received: {:?}",
                        GLOBAL_CONFIG_ACCOUNT_DISCM, maybe_discm
                    ),
                ),
            );
        }
        Ok(Self(GlobalConfig::deserialize(&mut reader)?))
    }

    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&GLOBAL_CONFIG_ACCOUNT_DISCM)?;
        self.0.serialize(&mut writer)
    }

    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}

#[derive(Clone, Debug, BorshDeserialize, BorshSerialize, PartialEq)]
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
    pub coin_creator: Pubkey,
    pub is_mayhem_mode: bool,

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
        Ok(Self(Pool::deserialize(&mut reader)?))
    }

    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&POOL_ACCOUNT_DISCM)?;
        self.0.serialize(&mut writer)
    }

    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}


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