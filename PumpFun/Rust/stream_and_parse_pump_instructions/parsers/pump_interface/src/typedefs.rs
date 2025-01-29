#[cfg(feature = "serde")]
use crate::serializer::{
    deserialize_i128_as_string, deserialize_u128_as_string, serialize_i128_as_string,
    serialize_u128_as_string,
};
use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::pubkey::Pubkey;
#[derive(Default, Clone, Debug, BorshDeserialize, BorshSerialize, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct OpenPositionBumps {
    pub position_bump: u8,
}
#[derive(Default, Clone, Debug, BorshDeserialize, BorshSerialize, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct OpenPositionWithMetadataBumps {
    pub position_bump: u8,
    pub metadata_bump: u8,
}
#[derive(Clone, Debug, BorshDeserialize, BorshSerialize, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct PositionRewardInfo {
    #[cfg_attr(
        feature = "serde",
        serde(
            serialize_with = "serialize_u128_as_string",
            deserialize_with = "deserialize_u128_as_string"
        )
    )]
    pub growth_inside_checkpoint: u128,
    pub amount_owed: u64,
}
#[derive(Clone, Debug, BorshDeserialize, BorshSerialize, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Tick {
    pub initialized: bool,
    #[cfg_attr(
        feature = "serde",
        serde(
            serialize_with = "serialize_i128_as_string",
            deserialize_with = "deserialize_i128_as_string"
        )
    )]
    pub liquidity_net: i128,
    #[cfg_attr(
        feature = "serde",
        serde(
            serialize_with = "serialize_u128_as_string",
            deserialize_with = "deserialize_u128_as_string"
        )
    )]
    pub liquidity_gross: u128,
    #[cfg_attr(
        feature = "serde",
        serde(
            serialize_with = "serialize_u128_as_string",
            deserialize_with = "deserialize_u128_as_string"
        )
    )]
    pub fee_growth_outside_a: u128,
    #[cfg_attr(
        feature = "serde",
        serde(
            serialize_with = "serialize_u128_as_string",
            deserialize_with = "deserialize_u128_as_string"
        )
    )]
    pub fee_growth_outside_b: u128,
    pub reward_growths_outside: [u128; 3],
}
#[derive(Clone, Debug, BorshDeserialize, BorshSerialize, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct WhirlpoolRewardInfo {
    pub mint: Pubkey,
    pub vault: Pubkey,
    pub authority: Pubkey,
    #[cfg_attr(
        feature = "serde",
        serde(
            serialize_with = "serialize_u128_as_string",
            deserialize_with = "deserialize_u128_as_string"
        )
    )]
    pub emissions_per_second_x64: u128,
    #[cfg_attr(
        feature = "serde",
        serde(
            serialize_with = "serialize_u128_as_string",
            deserialize_with = "deserialize_u128_as_string"
        )
    )]
    pub growth_global_x64: u128,
}
#[derive(Default, Clone, Debug, BorshDeserialize, BorshSerialize, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct WhirlpoolBumps {
    pub whirlpool_bump: u8,
}
#[derive(Clone, Debug, BorshDeserialize, BorshSerialize, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RemainingAccountsSlice {
    pub accounts_type: AccountsType,
    pub length: u8,
}
#[derive(Clone, Debug, BorshDeserialize, BorshSerialize, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RemainingAccountsInfo {
    pub slices: Vec<RemainingAccountsSlice>,
}
#[derive(Clone, Debug, BorshDeserialize, BorshSerialize, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum CurrIndex {
    Below,
    Inside,
    Above,
}
#[derive(Clone, Debug, BorshDeserialize, BorshSerialize, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum TickLabel {
    Upper,
    Lower,
}
#[derive(Clone, Debug, BorshDeserialize, BorshSerialize, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Direction {
    Left,
    Right,
}
#[derive(Clone, Debug, BorshDeserialize, BorshSerialize, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum AccountsType {
    TransferHookA,
    TransferHookB,
    TransferHookReward,
    TransferHookInput,
    TransferHookIntermediate,
    TransferHookOutput,
    SupplementalTickArrays,
    SupplementalTickArraysOne,
    SupplementalTickArraysTwo,
}