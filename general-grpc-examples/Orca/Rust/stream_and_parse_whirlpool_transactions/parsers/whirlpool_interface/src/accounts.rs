#[cfg(feature = "serde")]
use crate::serializer::{deserialize_u128_as_string, serialize_u128_as_string};
use crate::*;
use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::pubkey::Pubkey;
pub const WHIRLPOOLS_CONFIG_EXTENSION_ACCOUNT_DISCM: [u8; 8] = [2, 99, 215, 163, 240, 26, 153, 58];
#[derive(Clone, Debug, BorshDeserialize, BorshSerialize, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct WhirlpoolsConfigExtension {
    pub whirlpools_config: Pubkey,
    pub config_extension_authority: Pubkey,
    pub token_badge_authority: Pubkey,
}
#[derive(Clone, Debug, PartialEq)]
pub struct WhirlpoolsConfigExtensionAccount(pub WhirlpoolsConfigExtension);
impl WhirlpoolsConfigExtensionAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != WHIRLPOOLS_CONFIG_EXTENSION_ACCOUNT_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    WHIRLPOOLS_CONFIG_EXTENSION_ACCOUNT_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(WhirlpoolsConfigExtension::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&WHIRLPOOLS_CONFIG_EXTENSION_ACCOUNT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const WHIRLPOOLS_CONFIG_ACCOUNT_DISCM: [u8; 8] = [157, 20, 49, 224, 217, 87, 193, 254];
#[derive(Clone, Debug, BorshDeserialize, BorshSerialize, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct WhirlpoolsConfig {
    pub fee_authority: Pubkey,
    pub collect_protocol_fees_authority: Pubkey,
    pub reward_emissions_super_authority: Pubkey,
    pub default_protocol_fee_rate: u16,
}
#[derive(Clone, Debug, PartialEq)]
pub struct WhirlpoolsConfigAccount(pub WhirlpoolsConfig);
impl WhirlpoolsConfigAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != WHIRLPOOLS_CONFIG_ACCOUNT_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    WHIRLPOOLS_CONFIG_ACCOUNT_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(WhirlpoolsConfig::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&WHIRLPOOLS_CONFIG_ACCOUNT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const FEE_TIER_ACCOUNT_DISCM: [u8; 8] = [56, 75, 159, 76, 142, 68, 190, 105];
#[derive(Clone, Debug, BorshDeserialize, BorshSerialize, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FeeTier {
    pub whirlpools_config: Pubkey,
    pub tick_spacing: u16,
    pub default_fee_rate: u16,
}
#[derive(Clone, Debug, PartialEq)]
pub struct FeeTierAccount(pub FeeTier);
impl FeeTierAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != FEE_TIER_ACCOUNT_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    FEE_TIER_ACCOUNT_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(FeeTier::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&FEE_TIER_ACCOUNT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const POSITION_BUNDLE_ACCOUNT_DISCM: [u8; 8] = [129, 169, 175, 65, 185, 95, 32, 100];
#[derive(Clone, Debug, BorshDeserialize, BorshSerialize, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct PositionBundle {
    pub position_bundle_mint: Pubkey,
    pub position_bitmap: [u8; 32],
}
#[derive(Clone, Debug, PartialEq)]
pub struct PositionBundleAccount(pub PositionBundle);
impl PositionBundleAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != POSITION_BUNDLE_ACCOUNT_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    POSITION_BUNDLE_ACCOUNT_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(PositionBundle::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&POSITION_BUNDLE_ACCOUNT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const POSITION_ACCOUNT_DISCM: [u8; 8] = [170, 188, 143, 228, 122, 64, 247, 208];
#[derive(Clone, Debug, BorshDeserialize, BorshSerialize, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Position {
    pub whirlpool: Pubkey,
    pub position_mint: Pubkey,
    #[cfg_attr(
        feature = "serde",
        serde(
            serialize_with = "serialize_u128_as_string",
            deserialize_with = "deserialize_u128_as_string"
        )
    )]
    pub liquidity: u128,
    pub tick_lower_index: i32,
    pub tick_upper_index: i32,
    #[cfg_attr(
        feature = "serde",
        serde(
            serialize_with = "serialize_u128_as_string",
            deserialize_with = "deserialize_u128_as_string"
        )
    )]
    pub fee_growth_checkpoint_a: u128,
    pub fee_owed_a: u64,
    #[cfg_attr(
        feature = "serde",
        serde(
            serialize_with = "serialize_u128_as_string",
            deserialize_with = "deserialize_u128_as_string"
        )
    )]
    pub fee_growth_checkpoint_b: u128,
    pub fee_owed_b: u64,
    pub reward_infos: [PositionRewardInfo; 3],
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
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    POSITION_ACCOUNT_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(Position::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&POSITION_ACCOUNT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const TICK_ARRAY_ACCOUNT_DISCM: [u8; 8] = [69, 97, 189, 190, 110, 7, 66, 187];
#[derive(Clone, Debug, BorshDeserialize, BorshSerialize, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct TickArray {
    pub start_tick_index: i32,
    pub ticks: Vec<Tick>, // Changed from [Tick; 88] to Vec<Tick>
    pub whirlpool: Pubkey,
}
#[derive(Clone, Debug, PartialEq)]
pub struct TickArrayAccount(pub TickArray);
impl TickArrayAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != TICK_ARRAY_ACCOUNT_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    TICK_ARRAY_ACCOUNT_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(TickArray::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&TICK_ARRAY_ACCOUNT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const TOKEN_BADGE_ACCOUNT_DISCM: [u8; 8] = [116, 219, 204, 229, 249, 116, 255, 150];
#[derive(Clone, Debug, BorshDeserialize, BorshSerialize, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct TokenBadge {
    pub whirlpools_config: Pubkey,
    pub token_mint: Pubkey,
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
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    TOKEN_BADGE_ACCOUNT_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(TokenBadge::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&TOKEN_BADGE_ACCOUNT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const WHIRLPOOL_ACCOUNT_DISCM: [u8; 8] = [63, 149, 209, 12, 225, 128, 99, 9];
#[derive(Clone, Debug, BorshDeserialize, BorshSerialize, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Whirlpool {
    pub whirlpools_config: Pubkey,
    pub whirlpool_bump: [u8; 1],
    pub tick_spacing: u16,
    pub tick_spacing_seed: [u8; 2],
    pub fee_rate: u16,
    pub protocol_fee_rate: u16,
    #[cfg_attr(
        feature = "serde",
        serde(
            serialize_with = "serialize_u128_as_string",
            deserialize_with = "deserialize_u128_as_string"
        )
    )]
    pub liquidity: u128,
    #[cfg_attr(
        feature = "serde",
        serde(
            serialize_with = "serialize_u128_as_string",
            deserialize_with = "deserialize_u128_as_string"
        )
    )]
    pub sqrt_price: u128,
    pub tick_current_index: i32,
    pub protocol_fee_owed_a: u64,
    pub protocol_fee_owed_b: u64,
    pub token_mint_a: Pubkey,
    pub token_vault_a: Pubkey,
    #[cfg_attr(
        feature = "serde",
        serde(
            serialize_with = "serialize_u128_as_string",
            deserialize_with = "deserialize_u128_as_string"
        )
    )]
    pub fee_growth_global_a: u128,
    pub token_mint_b: Pubkey,
    pub token_vault_b: Pubkey,
    #[cfg_attr(
        feature = "serde",
        serde(
            serialize_with = "serialize_u128_as_string",
            deserialize_with = "deserialize_u128_as_string"
        )
    )]
    pub fee_growth_global_b: u128,
    pub reward_last_updated_timestamp: u64,
    pub reward_infos: [WhirlpoolRewardInfo; 3],
}
#[derive(Clone, Debug, PartialEq)]
pub struct WhirlpoolAccount(pub Whirlpool);
impl WhirlpoolAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != WHIRLPOOL_ACCOUNT_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    WHIRLPOOL_ACCOUNT_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(Whirlpool::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&WHIRLPOOL_ACCOUNT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
