#[cfg(feature = "serde")]
//use crate::serializer::{deserialize_u128_as_string, serialize_u128_as_string};
use std::fmt;
use crate::*;
use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::AccountInfo, entrypoint::ProgramResult,
    instruction::{AccountMeta, Instruction},
    program::{invoke, invoke_signed},
    pubkey::Pubkey, program_error::ProgramError,
};
use inflector::Inflector;
use std::io::Read;
use strum_macros::{Display, EnumString};


#[derive(Clone, Debug, PartialEq,EnumString,Display)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum MeteoraDlmmProgramIx {
    InitializeLbPair(InitializeLbPairIxArgs),
    InitializePermissionLbPair(InitializePermissionLbPairIxArgs),
    InitializeCustomizablePermissionlessLbPair(
        InitializeCustomizablePermissionlessLbPairIxArgs,
    ),
    InitializeBinArrayBitmapExtension,
    InitializeBinArray(InitializeBinArrayIxArgs),
    AddLiquidity(AddLiquidityIxArgs),
    AddLiquidityByWeight(AddLiquidityByWeightIxArgs),
    AddLiquidityByStrategy(AddLiquidityByStrategyIxArgs),
    AddLiquidityByStrategyOneSide(AddLiquidityByStrategyOneSideIxArgs),
    AddLiquidityOneSide(AddLiquidityOneSideIxArgs),
    RemoveLiquidity(RemoveLiquidityIxArgs),
    InitializePosition(InitializePositionIxArgs),
    InitializePositionPda(InitializePositionPdaIxArgs),
    InitializePositionByOperator(InitializePositionByOperatorIxArgs),
    UpdatePositionOperator(UpdatePositionOperatorIxArgs),
    Swap(SwapIxArgs),
    SwapExactOut(SwapExactOutIxArgs),
    SwapWithPriceImpact(SwapWithPriceImpactIxArgs),
    WithdrawProtocolFee(WithdrawProtocolFeeIxArgs),
    InitializeReward(InitializeRewardIxArgs),
    FundReward(FundRewardIxArgs),
    UpdateRewardFunder(UpdateRewardFunderIxArgs),
    UpdateRewardDuration(UpdateRewardDurationIxArgs),
    ClaimReward(ClaimRewardIxArgs),
    ClaimFee,
    ClosePosition,
    UpdateFeeParameters(UpdateFeeParametersIxArgs),
    IncreaseOracleLength(IncreaseOracleLengthIxArgs),
    InitializePresetParameter(InitializePresetParameterIxArgs),
    ClosePresetParameter,
    RemoveAllLiquidity,
    TogglePairStatus,
    MigratePosition,
    MigrateBinArray,
    UpdateFeesAndRewards,
    WithdrawIneligibleReward(WithdrawIneligibleRewardIxArgs),
    SetActivationPoint(SetActivationPointIxArgs),
    RemoveLiquidityByRange(RemoveLiquidityByRangeIxArgs),
    AddLiquidityOneSidePrecise(AddLiquidityOneSidePreciseIxArgs),
    GoToABin(GoToABinIxArgs),
    SetPreActivationDuration(SetPreActivationDurationIxArgs),
    SetPreActivationSwapAddress(SetPreActivationSwapAddressIxArgs),
}
impl MeteoraDlmmProgramIx {
    pub fn name(&self) -> String {
        // Use the ToString derived method to get the enum variant name
        self.to_string().to_camel_case()
    }
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        match maybe_discm {
            INITIALIZE_LB_PAIR_IX_DISCM => {
                Ok(
                    Self::InitializeLbPair(
                        InitializeLbPairIxArgs::deserialize(&mut reader)?,
                    ),
                )
            }
            INITIALIZE_PERMISSION_LB_PAIR_IX_DISCM => {
                Ok(
                    Self::InitializePermissionLbPair(
                        InitializePermissionLbPairIxArgs::deserialize(&mut reader)?,
                    ),
                )
            }
            INITIALIZE_CUSTOMIZABLE_PERMISSIONLESS_LB_PAIR_IX_DISCM => {
                Ok(
                    Self::InitializeCustomizablePermissionlessLbPair(
                        InitializeCustomizablePermissionlessLbPairIxArgs::deserialize(
                            &mut reader,
                        )?,
                    ),
                )
            }
            INITIALIZE_BIN_ARRAY_BITMAP_EXTENSION_IX_DISCM => {
                Ok(Self::InitializeBinArrayBitmapExtension)
            }
            INITIALIZE_BIN_ARRAY_IX_DISCM => {
                Ok(
                    Self::InitializeBinArray(
                        InitializeBinArrayIxArgs::deserialize(&mut reader)?,
                    ),
                )
            }
            ADD_LIQUIDITY_IX_DISCM => {
                Ok(Self::AddLiquidity(AddLiquidityIxArgs::deserialize(&mut reader)?))
            }
            ADD_LIQUIDITY_BY_WEIGHT_IX_DISCM => {
                Ok(
                    Self::AddLiquidityByWeight(
                        AddLiquidityByWeightIxArgs::deserialize(&mut reader)?,
                    ),
                )
            }
            ADD_LIQUIDITY_BY_STRATEGY_IX_DISCM => {
                Ok(
                    Self::AddLiquidityByStrategy(
                        AddLiquidityByStrategyIxArgs::deserialize(&mut reader)?,
                    ),
                )
            }
            ADD_LIQUIDITY_BY_STRATEGY_ONE_SIDE_IX_DISCM => {
                Ok(
                    Self::AddLiquidityByStrategyOneSide(
                        AddLiquidityByStrategyOneSideIxArgs::deserialize(&mut reader)?,
                    ),
                )
            }
            ADD_LIQUIDITY_ONE_SIDE_IX_DISCM => {
                Ok(
                    Self::AddLiquidityOneSide(
                        AddLiquidityOneSideIxArgs::deserialize(&mut reader)?,
                    ),
                )
            }
            REMOVE_LIQUIDITY_IX_DISCM => {
                Ok(
                    Self::RemoveLiquidity(
                        RemoveLiquidityIxArgs::deserialize(&mut reader)?,
                    ),
                )
            }
            INITIALIZE_POSITION_IX_DISCM => {
                Ok(
                    Self::InitializePosition(
                        InitializePositionIxArgs::deserialize(&mut reader)?,
                    ),
                )
            }
            INITIALIZE_POSITION_PDA_IX_DISCM => {
                Ok(
                    Self::InitializePositionPda(
                        InitializePositionPdaIxArgs::deserialize(&mut reader)?,
                    ),
                )
            }
            INITIALIZE_POSITION_BY_OPERATOR_IX_DISCM => {
                Ok(
                    Self::InitializePositionByOperator(
                        InitializePositionByOperatorIxArgs::deserialize(&mut reader)?,
                    ),
                )
            }
            UPDATE_POSITION_OPERATOR_IX_DISCM => {
                Ok(
                    Self::UpdatePositionOperator(
                        UpdatePositionOperatorIxArgs::deserialize(&mut reader)?,
                    ),
                )
            }
            SWAP_IX_DISCM => Ok(Self::Swap(SwapIxArgs::deserialize(&mut reader)?)),
            SWAP_EXACT_OUT_IX_DISCM => {
                Ok(Self::SwapExactOut(SwapExactOutIxArgs::deserialize(&mut reader)?))
            }
            SWAP_WITH_PRICE_IMPACT_IX_DISCM => {
                Ok(
                    Self::SwapWithPriceImpact(
                        SwapWithPriceImpactIxArgs::deserialize(&mut reader)?,
                    ),
                )
            }
            WITHDRAW_PROTOCOL_FEE_IX_DISCM => {
                Ok(
                    Self::WithdrawProtocolFee(
                        WithdrawProtocolFeeIxArgs::deserialize(&mut reader)?,
                    ),
                )
            }
            INITIALIZE_REWARD_IX_DISCM => {
                Ok(
                    Self::InitializeReward(
                        InitializeRewardIxArgs::deserialize(&mut reader)?,
                    ),
                )
            }
            FUND_REWARD_IX_DISCM => {
                Ok(Self::FundReward(FundRewardIxArgs::deserialize(&mut reader)?))
            }
            UPDATE_REWARD_FUNDER_IX_DISCM => {
                Ok(
                    Self::UpdateRewardFunder(
                        UpdateRewardFunderIxArgs::deserialize(&mut reader)?,
                    ),
                )
            }
            UPDATE_REWARD_DURATION_IX_DISCM => {
                Ok(
                    Self::UpdateRewardDuration(
                        UpdateRewardDurationIxArgs::deserialize(&mut reader)?,
                    ),
                )
            }
            CLAIM_REWARD_IX_DISCM => {
                Ok(Self::ClaimReward(ClaimRewardIxArgs::deserialize(&mut reader)?))
            }
            CLAIM_FEE_IX_DISCM => Ok(Self::ClaimFee),
            CLOSE_POSITION_IX_DISCM => Ok(Self::ClosePosition),
            UPDATE_FEE_PARAMETERS_IX_DISCM => {
                Ok(
                    Self::UpdateFeeParameters(
                        UpdateFeeParametersIxArgs::deserialize(&mut reader)?,
                    ),
                )
            }
            INCREASE_ORACLE_LENGTH_IX_DISCM => {
                Ok(
                    Self::IncreaseOracleLength(
                        IncreaseOracleLengthIxArgs::deserialize(&mut reader)?,
                    ),
                )
            }
            INITIALIZE_PRESET_PARAMETER_IX_DISCM => {
                Ok(
                    Self::InitializePresetParameter(
                        InitializePresetParameterIxArgs::deserialize(&mut reader)?,
                    ),
                )
            }
            CLOSE_PRESET_PARAMETER_IX_DISCM => Ok(Self::ClosePresetParameter),
            REMOVE_ALL_LIQUIDITY_IX_DISCM => Ok(Self::RemoveAllLiquidity),
            TOGGLE_PAIR_STATUS_IX_DISCM => Ok(Self::TogglePairStatus),
            MIGRATE_POSITION_IX_DISCM => Ok(Self::MigratePosition),
            MIGRATE_BIN_ARRAY_IX_DISCM => Ok(Self::MigrateBinArray),
            UPDATE_FEES_AND_REWARDS_IX_DISCM => Ok(Self::UpdateFeesAndRewards),
            WITHDRAW_INELIGIBLE_REWARD_IX_DISCM => {
                Ok(
                    Self::WithdrawIneligibleReward(
                        WithdrawIneligibleRewardIxArgs::deserialize(&mut reader)?,
                    ),
                )
            }
            SET_ACTIVATION_POINT_IX_DISCM => {
                Ok(
                    Self::SetActivationPoint(
                        SetActivationPointIxArgs::deserialize(&mut reader)?,
                    ),
                )
            }
            REMOVE_LIQUIDITY_BY_RANGE_IX_DISCM => {
                Ok(
                    Self::RemoveLiquidityByRange(
                        RemoveLiquidityByRangeIxArgs::deserialize(&mut reader)?,
                    ),
                )
            }
            ADD_LIQUIDITY_ONE_SIDE_PRECISE_IX_DISCM => {
                Ok(
                    Self::AddLiquidityOneSidePrecise(
                        AddLiquidityOneSidePreciseIxArgs::deserialize(&mut reader)?,
                    ),
                )
            }
            GO_TO_A_BIN_IX_DISCM => {
                Ok(Self::GoToABin(GoToABinIxArgs::deserialize(&mut reader)?))
            }
            SET_PRE_ACTIVATION_DURATION_IX_DISCM => {
                Ok(
                    Self::SetPreActivationDuration(
                        SetPreActivationDurationIxArgs::deserialize(&mut reader)?,
                    ),
                )
            }
            SET_PRE_ACTIVATION_SWAP_ADDRESS_IX_DISCM => {
                Ok(
                    Self::SetPreActivationSwapAddress(
                        SetPreActivationSwapAddressIxArgs::deserialize(&mut reader)?,
                    ),
                )
            }
            _ => {
                Err(
                    std::io::Error::new(
                        std::io::ErrorKind::Other,
                        format!("discm {:?} not found", maybe_discm),
                    ),
                )
            }
        }
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        match self {
            Self::InitializeLbPair(args) => {
                writer.write_all(&INITIALIZE_LB_PAIR_IX_DISCM)?;
                args.serialize(&mut writer)
            }
            Self::InitializePermissionLbPair(args) => {
                writer.write_all(&INITIALIZE_PERMISSION_LB_PAIR_IX_DISCM)?;
                args.serialize(&mut writer)
            }
            Self::InitializeCustomizablePermissionlessLbPair(args) => {
                writer
                    .write_all(
                        &INITIALIZE_CUSTOMIZABLE_PERMISSIONLESS_LB_PAIR_IX_DISCM,
                    )?;
                args.serialize(&mut writer)
            }
            Self::InitializeBinArrayBitmapExtension => {
                writer.write_all(&INITIALIZE_BIN_ARRAY_BITMAP_EXTENSION_IX_DISCM)
            }
            Self::InitializeBinArray(args) => {
                writer.write_all(&INITIALIZE_BIN_ARRAY_IX_DISCM)?;
                args.serialize(&mut writer)
            }
            Self::AddLiquidity(args) => {
                writer.write_all(&ADD_LIQUIDITY_IX_DISCM)?;
                args.serialize(&mut writer)
            }
            Self::AddLiquidityByWeight(args) => {
                writer.write_all(&ADD_LIQUIDITY_BY_WEIGHT_IX_DISCM)?;
                args.serialize(&mut writer)
            }
            Self::AddLiquidityByStrategy(args) => {
                writer.write_all(&ADD_LIQUIDITY_BY_STRATEGY_IX_DISCM)?;
                args.serialize(&mut writer)
            }
            Self::AddLiquidityByStrategyOneSide(args) => {
                writer.write_all(&ADD_LIQUIDITY_BY_STRATEGY_ONE_SIDE_IX_DISCM)?;
                args.serialize(&mut writer)
            }
            Self::AddLiquidityOneSide(args) => {
                writer.write_all(&ADD_LIQUIDITY_ONE_SIDE_IX_DISCM)?;
                args.serialize(&mut writer)
            }
            Self::RemoveLiquidity(args) => {
                writer.write_all(&REMOVE_LIQUIDITY_IX_DISCM)?;
                args.serialize(&mut writer)
            }
            Self::InitializePosition(args) => {
                writer.write_all(&INITIALIZE_POSITION_IX_DISCM)?;
                args.serialize(&mut writer)
            }
            Self::InitializePositionPda(args) => {
                writer.write_all(&INITIALIZE_POSITION_PDA_IX_DISCM)?;
                args.serialize(&mut writer)
            }
            Self::InitializePositionByOperator(args) => {
                writer.write_all(&INITIALIZE_POSITION_BY_OPERATOR_IX_DISCM)?;
                args.serialize(&mut writer)
            }
            Self::UpdatePositionOperator(args) => {
                writer.write_all(&UPDATE_POSITION_OPERATOR_IX_DISCM)?;
                args.serialize(&mut writer)
            }
            Self::Swap(args) => {
                writer.write_all(&SWAP_IX_DISCM)?;
                args.serialize(&mut writer)
            }
            Self::SwapExactOut(args) => {
                writer.write_all(&SWAP_EXACT_OUT_IX_DISCM)?;
                args.serialize(&mut writer)
            }
            Self::SwapWithPriceImpact(args) => {
                writer.write_all(&SWAP_WITH_PRICE_IMPACT_IX_DISCM)?;
                args.serialize(&mut writer)
            }
            Self::WithdrawProtocolFee(args) => {
                writer.write_all(&WITHDRAW_PROTOCOL_FEE_IX_DISCM)?;
                args.serialize(&mut writer)
            }
            Self::InitializeReward(args) => {
                writer.write_all(&INITIALIZE_REWARD_IX_DISCM)?;
                args.serialize(&mut writer)
            }
            Self::FundReward(args) => {
                writer.write_all(&FUND_REWARD_IX_DISCM)?;
                args.serialize(&mut writer)
            }
            Self::UpdateRewardFunder(args) => {
                writer.write_all(&UPDATE_REWARD_FUNDER_IX_DISCM)?;
                args.serialize(&mut writer)
            }
            Self::UpdateRewardDuration(args) => {
                writer.write_all(&UPDATE_REWARD_DURATION_IX_DISCM)?;
                args.serialize(&mut writer)
            }
            Self::ClaimReward(args) => {
                writer.write_all(&CLAIM_REWARD_IX_DISCM)?;
                args.serialize(&mut writer)
            }
            Self::ClaimFee => writer.write_all(&CLAIM_FEE_IX_DISCM),
            Self::ClosePosition => writer.write_all(&CLOSE_POSITION_IX_DISCM),
            Self::UpdateFeeParameters(args) => {
                writer.write_all(&UPDATE_FEE_PARAMETERS_IX_DISCM)?;
                args.serialize(&mut writer)
            }
            Self::IncreaseOracleLength(args) => {
                writer.write_all(&INCREASE_ORACLE_LENGTH_IX_DISCM)?;
                args.serialize(&mut writer)
            }
            Self::InitializePresetParameter(args) => {
                writer.write_all(&INITIALIZE_PRESET_PARAMETER_IX_DISCM)?;
                args.serialize(&mut writer)
            }
            Self::ClosePresetParameter => {
                writer.write_all(&CLOSE_PRESET_PARAMETER_IX_DISCM)
            }
            Self::RemoveAllLiquidity => writer.write_all(&REMOVE_ALL_LIQUIDITY_IX_DISCM),
            Self::TogglePairStatus => writer.write_all(&TOGGLE_PAIR_STATUS_IX_DISCM),
            Self::MigratePosition => writer.write_all(&MIGRATE_POSITION_IX_DISCM),
            Self::MigrateBinArray => writer.write_all(&MIGRATE_BIN_ARRAY_IX_DISCM),
            Self::UpdateFeesAndRewards => {
                writer.write_all(&UPDATE_FEES_AND_REWARDS_IX_DISCM)
            }
            Self::WithdrawIneligibleReward(args) => {
                writer.write_all(&WITHDRAW_INELIGIBLE_REWARD_IX_DISCM)?;
                args.serialize(&mut writer)
            }
            Self::SetActivationPoint(args) => {
                writer.write_all(&SET_ACTIVATION_POINT_IX_DISCM)?;
                args.serialize(&mut writer)
            }
            Self::RemoveLiquidityByRange(args) => {
                writer.write_all(&REMOVE_LIQUIDITY_BY_RANGE_IX_DISCM)?;
                args.serialize(&mut writer)
            }
            Self::AddLiquidityOneSidePrecise(args) => {
                writer.write_all(&ADD_LIQUIDITY_ONE_SIDE_PRECISE_IX_DISCM)?;
                args.serialize(&mut writer)
            }
            Self::GoToABin(args) => {
                writer.write_all(&GO_TO_A_BIN_IX_DISCM)?;
                args.serialize(&mut writer)
            }
            Self::SetPreActivationDuration(args) => {
                writer.write_all(&SET_PRE_ACTIVATION_DURATION_IX_DISCM)?;
                args.serialize(&mut writer)
            }
            Self::SetPreActivationSwapAddress(args) => {
                writer.write_all(&SET_PRE_ACTIVATION_SWAP_ADDRESS_IX_DISCM)?;
                args.serialize(&mut writer)
            }
        }
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
fn invoke_instruction<'info, A: Into<[AccountInfo<'info>; N]>, const N: usize>(
    ix: &Instruction,
    accounts: A,
) -> ProgramResult {
    let account_info: [AccountInfo<'info>; N] = accounts.into();
    invoke(ix, &account_info)
}
fn invoke_instruction_signed<'info, A: Into<[AccountInfo<'info>; N]>, const N: usize>(
    ix: &Instruction,
    accounts: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let account_info: [AccountInfo<'info>; N] = accounts.into();
    invoke_signed(ix, &account_info, seeds)
}
pub const INITIALIZE_LB_PAIR_IX_ACCOUNTS_LEN: usize = 14;
#[derive(Copy, Clone, Debug)]
pub struct InitializeLbPairAccounts<'me, 'info> {
    pub lb_pair: &'me AccountInfo<'info>,
    pub bin_array_bitmap_extension: &'me AccountInfo<'info>,
    pub token_mint_x: &'me AccountInfo<'info>,
    pub token_mint_y: &'me AccountInfo<'info>,
    pub reserve_x: &'me AccountInfo<'info>,
    pub reserve_y: &'me AccountInfo<'info>,
    pub oracle: &'me AccountInfo<'info>,
    pub preset_parameter: &'me AccountInfo<'info>,
    pub funder: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub rent: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct InitializeLbPairKeys {
    pub lb_pair: Pubkey,
    pub bin_array_bitmap_extension: Pubkey,
    pub token_mint_x: Pubkey,
    pub token_mint_y: Pubkey,
    pub reserve_x: Pubkey,
    pub reserve_y: Pubkey,
    pub oracle: Pubkey,
    pub preset_parameter: Pubkey,
    pub funder: Pubkey,
    pub token_program: Pubkey,
    pub system_program: Pubkey,
    pub rent: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<InitializeLbPairAccounts<'_, '_>> for InitializeLbPairKeys {
    fn from(accounts: InitializeLbPairAccounts) -> Self {
        Self {
            lb_pair: *accounts.lb_pair.key,
            bin_array_bitmap_extension: *accounts.bin_array_bitmap_extension.key,
            token_mint_x: *accounts.token_mint_x.key,
            token_mint_y: *accounts.token_mint_y.key,
            reserve_x: *accounts.reserve_x.key,
            reserve_y: *accounts.reserve_y.key,
            oracle: *accounts.oracle.key,
            preset_parameter: *accounts.preset_parameter.key,
            funder: *accounts.funder.key,
            token_program: *accounts.token_program.key,
            system_program: *accounts.system_program.key,
            rent: *accounts.rent.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<InitializeLbPairKeys> for [AccountMeta; INITIALIZE_LB_PAIR_IX_ACCOUNTS_LEN] {
    fn from(keys: InitializeLbPairKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.lb_pair,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.bin_array_bitmap_extension,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_mint_x,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_mint_y,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.reserve_x,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.reserve_y,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.oracle,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.preset_parameter,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.funder,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.system_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.rent,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.event_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; INITIALIZE_LB_PAIR_IX_ACCOUNTS_LEN]> for InitializeLbPairKeys {
    fn from(pubkeys: [Pubkey; INITIALIZE_LB_PAIR_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            lb_pair: pubkeys[0],
            bin_array_bitmap_extension: pubkeys[1],
            token_mint_x: pubkeys[2],
            token_mint_y: pubkeys[3],
            reserve_x: pubkeys[4],
            reserve_y: pubkeys[5],
            oracle: pubkeys[6],
            preset_parameter: pubkeys[7],
            funder: pubkeys[8],
            token_program: pubkeys[9],
            system_program: pubkeys[10],
            rent: pubkeys[11],
            event_authority: pubkeys[12],
            program: pubkeys[13],
        }
    }
}
impl<'info> From<InitializeLbPairAccounts<'_, 'info>>
for [AccountInfo<'info>; INITIALIZE_LB_PAIR_IX_ACCOUNTS_LEN] {
    fn from(accounts: InitializeLbPairAccounts<'_, 'info>) -> Self {
        [
            accounts.lb_pair.clone(),
            accounts.bin_array_bitmap_extension.clone(),
            accounts.token_mint_x.clone(),
            accounts.token_mint_y.clone(),
            accounts.reserve_x.clone(),
            accounts.reserve_y.clone(),
            accounts.oracle.clone(),
            accounts.preset_parameter.clone(),
            accounts.funder.clone(),
            accounts.token_program.clone(),
            accounts.system_program.clone(),
            accounts.rent.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; INITIALIZE_LB_PAIR_IX_ACCOUNTS_LEN]>
for InitializeLbPairAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; INITIALIZE_LB_PAIR_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            lb_pair: &arr[0],
            bin_array_bitmap_extension: &arr[1],
            token_mint_x: &arr[2],
            token_mint_y: &arr[3],
            reserve_x: &arr[4],
            reserve_y: &arr[5],
            oracle: &arr[6],
            preset_parameter: &arr[7],
            funder: &arr[8],
            token_program: &arr[9],
            system_program: &arr[10],
            rent: &arr[11],
            event_authority: &arr[12],
            program: &arr[13],
        }
    }
}
pub const INITIALIZE_LB_PAIR_IX_DISCM: [u8; 8] = [45, 154, 237, 210, 221, 15, 166, 92];
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InitializeLbPairIxArgs {
    pub active_id: i32,
    pub bin_step: u16,
}
#[derive(Clone, Debug, PartialEq)]
pub struct InitializeLbPairIxData(pub InitializeLbPairIxArgs);
impl From<InitializeLbPairIxArgs> for InitializeLbPairIxData {
    fn from(args: InitializeLbPairIxArgs) -> Self {
        Self(args)
    }
}
impl InitializeLbPairIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != INITIALIZE_LB_PAIR_IX_DISCM {
            return Err(
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!(
                        "discm does not match. Expected: {:?}. Received: {:?}",
                        INITIALIZE_LB_PAIR_IX_DISCM, maybe_discm
                    ),
                ),
            );
        }
        Ok(Self(InitializeLbPairIxArgs::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&INITIALIZE_LB_PAIR_IX_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn initialize_lb_pair_ix_with_program_id(
    program_id: Pubkey,
    keys: InitializeLbPairKeys,
    args: InitializeLbPairIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; INITIALIZE_LB_PAIR_IX_ACCOUNTS_LEN] = keys.into();
    let data: InitializeLbPairIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn initialize_lb_pair_ix(
    keys: InitializeLbPairKeys,
    args: InitializeLbPairIxArgs,
) -> std::io::Result<Instruction> {
    initialize_lb_pair_ix_with_program_id(crate::ID, keys, args)
}
pub fn initialize_lb_pair_invoke_with_program_id(
    program_id: Pubkey,
    accounts: InitializeLbPairAccounts<'_, '_>,
    args: InitializeLbPairIxArgs,
) -> ProgramResult {
    let keys: InitializeLbPairKeys = accounts.into();
    let ix = initialize_lb_pair_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn initialize_lb_pair_invoke(
    accounts: InitializeLbPairAccounts<'_, '_>,
    args: InitializeLbPairIxArgs,
) -> ProgramResult {
    initialize_lb_pair_invoke_with_program_id(crate::ID, accounts, args)
}
pub fn initialize_lb_pair_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: InitializeLbPairAccounts<'_, '_>,
    args: InitializeLbPairIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: InitializeLbPairKeys = accounts.into();
    let ix = initialize_lb_pair_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn initialize_lb_pair_invoke_signed(
    accounts: InitializeLbPairAccounts<'_, '_>,
    args: InitializeLbPairIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    initialize_lb_pair_invoke_signed_with_program_id(crate::ID, accounts, args, seeds)
}
pub fn initialize_lb_pair_verify_account_keys(
    accounts: InitializeLbPairAccounts<'_, '_>,
    keys: InitializeLbPairKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.lb_pair.key, keys.lb_pair),
        (*accounts.bin_array_bitmap_extension.key, keys.bin_array_bitmap_extension),
        (*accounts.token_mint_x.key, keys.token_mint_x),
        (*accounts.token_mint_y.key, keys.token_mint_y),
        (*accounts.reserve_x.key, keys.reserve_x),
        (*accounts.reserve_y.key, keys.reserve_y),
        (*accounts.oracle.key, keys.oracle),
        (*accounts.preset_parameter.key, keys.preset_parameter),
        (*accounts.funder.key, keys.funder),
        (*accounts.token_program.key, keys.token_program),
        (*accounts.system_program.key, keys.system_program),
        (*accounts.rent.key, keys.rent),
        (*accounts.event_authority.key, keys.event_authority),
        (*accounts.program.key, keys.program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn initialize_lb_pair_verify_writable_privileges<'me, 'info>(
    accounts: InitializeLbPairAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.lb_pair,
        accounts.bin_array_bitmap_extension,
        accounts.reserve_x,
        accounts.reserve_y,
        accounts.oracle,
        accounts.funder,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn initialize_lb_pair_verify_signer_privileges<'me, 'info>(
    accounts: InitializeLbPairAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.funder] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn initialize_lb_pair_verify_account_privileges<'me, 'info>(
    accounts: InitializeLbPairAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    initialize_lb_pair_verify_writable_privileges(accounts)?;
    initialize_lb_pair_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const INITIALIZE_PERMISSION_LB_PAIR_IX_ACCOUNTS_LEN: usize = 14;
#[derive(Copy, Clone, Debug)]
pub struct InitializePermissionLbPairAccounts<'me, 'info> {
    pub base: &'me AccountInfo<'info>,
    pub lb_pair: &'me AccountInfo<'info>,
    pub bin_array_bitmap_extension: &'me AccountInfo<'info>,
    pub token_mint_x: &'me AccountInfo<'info>,
    pub token_mint_y: &'me AccountInfo<'info>,
    pub reserve_x: &'me AccountInfo<'info>,
    pub reserve_y: &'me AccountInfo<'info>,
    pub oracle: &'me AccountInfo<'info>,
    pub admin: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub rent: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct InitializePermissionLbPairKeys {
    pub base: Pubkey,
    pub lb_pair: Pubkey,
    pub bin_array_bitmap_extension: Pubkey,
    pub token_mint_x: Pubkey,
    pub token_mint_y: Pubkey,
    pub reserve_x: Pubkey,
    pub reserve_y: Pubkey,
    pub oracle: Pubkey,
    pub admin: Pubkey,
    pub token_program: Pubkey,
    pub system_program: Pubkey,
    pub rent: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<InitializePermissionLbPairAccounts<'_, '_>>
for InitializePermissionLbPairKeys {
    fn from(accounts: InitializePermissionLbPairAccounts) -> Self {
        Self {
            base: *accounts.base.key,
            lb_pair: *accounts.lb_pair.key,
            bin_array_bitmap_extension: *accounts.bin_array_bitmap_extension.key,
            token_mint_x: *accounts.token_mint_x.key,
            token_mint_y: *accounts.token_mint_y.key,
            reserve_x: *accounts.reserve_x.key,
            reserve_y: *accounts.reserve_y.key,
            oracle: *accounts.oracle.key,
            admin: *accounts.admin.key,
            token_program: *accounts.token_program.key,
            system_program: *accounts.system_program.key,
            rent: *accounts.rent.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<InitializePermissionLbPairKeys>
for [AccountMeta; INITIALIZE_PERMISSION_LB_PAIR_IX_ACCOUNTS_LEN] {
    fn from(keys: InitializePermissionLbPairKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.base,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.lb_pair,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.bin_array_bitmap_extension,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_mint_x,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_mint_y,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.reserve_x,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.reserve_y,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.oracle,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.admin,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.system_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.rent,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.event_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; INITIALIZE_PERMISSION_LB_PAIR_IX_ACCOUNTS_LEN]>
for InitializePermissionLbPairKeys {
    fn from(pubkeys: [Pubkey; INITIALIZE_PERMISSION_LB_PAIR_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            base: pubkeys[0],
            lb_pair: pubkeys[1],
            bin_array_bitmap_extension: pubkeys[2],
            token_mint_x: pubkeys[3],
            token_mint_y: pubkeys[4],
            reserve_x: pubkeys[5],
            reserve_y: pubkeys[6],
            oracle: pubkeys[7],
            admin: pubkeys[8],
            token_program: pubkeys[9],
            system_program: pubkeys[10],
            rent: pubkeys[11],
            event_authority: pubkeys[12],
            program: pubkeys[13],
        }
    }
}
impl<'info> From<InitializePermissionLbPairAccounts<'_, 'info>>
for [AccountInfo<'info>; INITIALIZE_PERMISSION_LB_PAIR_IX_ACCOUNTS_LEN] {
    fn from(accounts: InitializePermissionLbPairAccounts<'_, 'info>) -> Self {
        [
            accounts.base.clone(),
            accounts.lb_pair.clone(),
            accounts.bin_array_bitmap_extension.clone(),
            accounts.token_mint_x.clone(),
            accounts.token_mint_y.clone(),
            accounts.reserve_x.clone(),
            accounts.reserve_y.clone(),
            accounts.oracle.clone(),
            accounts.admin.clone(),
            accounts.token_program.clone(),
            accounts.system_program.clone(),
            accounts.rent.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; INITIALIZE_PERMISSION_LB_PAIR_IX_ACCOUNTS_LEN]>
for InitializePermissionLbPairAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; INITIALIZE_PERMISSION_LB_PAIR_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            base: &arr[0],
            lb_pair: &arr[1],
            bin_array_bitmap_extension: &arr[2],
            token_mint_x: &arr[3],
            token_mint_y: &arr[4],
            reserve_x: &arr[5],
            reserve_y: &arr[6],
            oracle: &arr[7],
            admin: &arr[8],
            token_program: &arr[9],
            system_program: &arr[10],
            rent: &arr[11],
            event_authority: &arr[12],
            program: &arr[13],
        }
    }
}
pub const INITIALIZE_PERMISSION_LB_PAIR_IX_DISCM: [u8; 8] = [
    108,
    102,
    213,
    85,
    251,
    3,
    53,
    21,
];
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InitializePermissionLbPairIxArgs {
    pub ix_data: InitPermissionPairIx,
}
#[derive(Clone, Debug, PartialEq)]
pub struct InitializePermissionLbPairIxData(pub InitializePermissionLbPairIxArgs);
impl From<InitializePermissionLbPairIxArgs> for InitializePermissionLbPairIxData {
    fn from(args: InitializePermissionLbPairIxArgs) -> Self {
        Self(args)
    }
}
impl InitializePermissionLbPairIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != INITIALIZE_PERMISSION_LB_PAIR_IX_DISCM {
            return Err(
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!(
                        "discm does not match. Expected: {:?}. Received: {:?}",
                        INITIALIZE_PERMISSION_LB_PAIR_IX_DISCM, maybe_discm
                    ),
                ),
            );
        }
        Ok(Self(InitializePermissionLbPairIxArgs::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&INITIALIZE_PERMISSION_LB_PAIR_IX_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn initialize_permission_lb_pair_ix_with_program_id(
    program_id: Pubkey,
    keys: InitializePermissionLbPairKeys,
    args: InitializePermissionLbPairIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; INITIALIZE_PERMISSION_LB_PAIR_IX_ACCOUNTS_LEN] = keys
        .into();
    let data: InitializePermissionLbPairIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn initialize_permission_lb_pair_ix(
    keys: InitializePermissionLbPairKeys,
    args: InitializePermissionLbPairIxArgs,
) -> std::io::Result<Instruction> {
    initialize_permission_lb_pair_ix_with_program_id(crate::ID, keys, args)
}
pub fn initialize_permission_lb_pair_invoke_with_program_id(
    program_id: Pubkey,
    accounts: InitializePermissionLbPairAccounts<'_, '_>,
    args: InitializePermissionLbPairIxArgs,
) -> ProgramResult {
    let keys: InitializePermissionLbPairKeys = accounts.into();
    let ix = initialize_permission_lb_pair_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn initialize_permission_lb_pair_invoke(
    accounts: InitializePermissionLbPairAccounts<'_, '_>,
    args: InitializePermissionLbPairIxArgs,
) -> ProgramResult {
    initialize_permission_lb_pair_invoke_with_program_id(crate::ID, accounts, args)
}
pub fn initialize_permission_lb_pair_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: InitializePermissionLbPairAccounts<'_, '_>,
    args: InitializePermissionLbPairIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: InitializePermissionLbPairKeys = accounts.into();
    let ix = initialize_permission_lb_pair_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn initialize_permission_lb_pair_invoke_signed(
    accounts: InitializePermissionLbPairAccounts<'_, '_>,
    args: InitializePermissionLbPairIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    initialize_permission_lb_pair_invoke_signed_with_program_id(
        crate::ID,
        accounts,
        args,
        seeds,
    )
}
pub fn initialize_permission_lb_pair_verify_account_keys(
    accounts: InitializePermissionLbPairAccounts<'_, '_>,
    keys: InitializePermissionLbPairKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.base.key, keys.base),
        (*accounts.lb_pair.key, keys.lb_pair),
        (*accounts.bin_array_bitmap_extension.key, keys.bin_array_bitmap_extension),
        (*accounts.token_mint_x.key, keys.token_mint_x),
        (*accounts.token_mint_y.key, keys.token_mint_y),
        (*accounts.reserve_x.key, keys.reserve_x),
        (*accounts.reserve_y.key, keys.reserve_y),
        (*accounts.oracle.key, keys.oracle),
        (*accounts.admin.key, keys.admin),
        (*accounts.token_program.key, keys.token_program),
        (*accounts.system_program.key, keys.system_program),
        (*accounts.rent.key, keys.rent),
        (*accounts.event_authority.key, keys.event_authority),
        (*accounts.program.key, keys.program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn initialize_permission_lb_pair_verify_writable_privileges<'me, 'info>(
    accounts: InitializePermissionLbPairAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.lb_pair,
        accounts.bin_array_bitmap_extension,
        accounts.reserve_x,
        accounts.reserve_y,
        accounts.oracle,
        accounts.admin,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn initialize_permission_lb_pair_verify_signer_privileges<'me, 'info>(
    accounts: InitializePermissionLbPairAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.base, accounts.admin] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn initialize_permission_lb_pair_verify_account_privileges<'me, 'info>(
    accounts: InitializePermissionLbPairAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    initialize_permission_lb_pair_verify_writable_privileges(accounts)?;
    initialize_permission_lb_pair_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const INITIALIZE_CUSTOMIZABLE_PERMISSIONLESS_LB_PAIR_IX_ACCOUNTS_LEN: usize = 14;
#[derive(Copy, Clone, Debug)]
pub struct InitializeCustomizablePermissionlessLbPairAccounts<'me, 'info> {
    pub lb_pair: &'me AccountInfo<'info>,
    pub bin_array_bitmap_extension: &'me AccountInfo<'info>,
    pub token_mint_x: &'me AccountInfo<'info>,
    pub token_mint_y: &'me AccountInfo<'info>,
    pub reserve_x: &'me AccountInfo<'info>,
    pub reserve_y: &'me AccountInfo<'info>,
    pub oracle: &'me AccountInfo<'info>,
    pub user_token_x: &'me AccountInfo<'info>,
    pub funder: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub rent: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct InitializeCustomizablePermissionlessLbPairKeys {
    pub lb_pair: Pubkey,
    pub bin_array_bitmap_extension: Pubkey,
    pub token_mint_x: Pubkey,
    pub token_mint_y: Pubkey,
    pub reserve_x: Pubkey,
    pub reserve_y: Pubkey,
    pub oracle: Pubkey,
    pub user_token_x: Pubkey,
    pub funder: Pubkey,
    pub token_program: Pubkey,
    pub system_program: Pubkey,
    pub rent: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<InitializeCustomizablePermissionlessLbPairAccounts<'_, '_>>
for InitializeCustomizablePermissionlessLbPairKeys {
    fn from(accounts: InitializeCustomizablePermissionlessLbPairAccounts) -> Self {
        Self {
            lb_pair: *accounts.lb_pair.key,
            bin_array_bitmap_extension: *accounts.bin_array_bitmap_extension.key,
            token_mint_x: *accounts.token_mint_x.key,
            token_mint_y: *accounts.token_mint_y.key,
            reserve_x: *accounts.reserve_x.key,
            reserve_y: *accounts.reserve_y.key,
            oracle: *accounts.oracle.key,
            user_token_x: *accounts.user_token_x.key,
            funder: *accounts.funder.key,
            token_program: *accounts.token_program.key,
            system_program: *accounts.system_program.key,
            rent: *accounts.rent.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<InitializeCustomizablePermissionlessLbPairKeys>
for [AccountMeta; INITIALIZE_CUSTOMIZABLE_PERMISSIONLESS_LB_PAIR_IX_ACCOUNTS_LEN] {
    fn from(keys: InitializeCustomizablePermissionlessLbPairKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.lb_pair,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.bin_array_bitmap_extension,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_mint_x,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_mint_y,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.reserve_x,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.reserve_y,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.oracle,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_token_x,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.funder,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.system_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.rent,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.event_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; INITIALIZE_CUSTOMIZABLE_PERMISSIONLESS_LB_PAIR_IX_ACCOUNTS_LEN]>
for InitializeCustomizablePermissionlessLbPairKeys {
    fn from(
        pubkeys: [Pubkey; INITIALIZE_CUSTOMIZABLE_PERMISSIONLESS_LB_PAIR_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            lb_pair: pubkeys[0],
            bin_array_bitmap_extension: pubkeys[1],
            token_mint_x: pubkeys[2],
            token_mint_y: pubkeys[3],
            reserve_x: pubkeys[4],
            reserve_y: pubkeys[5],
            oracle: pubkeys[6],
            user_token_x: pubkeys[7],
            funder: pubkeys[8],
            token_program: pubkeys[9],
            system_program: pubkeys[10],
            rent: pubkeys[11],
            event_authority: pubkeys[12],
            program: pubkeys[13],
        }
    }
}
impl<'info> From<InitializeCustomizablePermissionlessLbPairAccounts<'_, 'info>>
for [AccountInfo<
    'info,
>; INITIALIZE_CUSTOMIZABLE_PERMISSIONLESS_LB_PAIR_IX_ACCOUNTS_LEN] {
    fn from(
        accounts: InitializeCustomizablePermissionlessLbPairAccounts<'_, 'info>,
    ) -> Self {
        [
            accounts.lb_pair.clone(),
            accounts.bin_array_bitmap_extension.clone(),
            accounts.token_mint_x.clone(),
            accounts.token_mint_y.clone(),
            accounts.reserve_x.clone(),
            accounts.reserve_y.clone(),
            accounts.oracle.clone(),
            accounts.user_token_x.clone(),
            accounts.funder.clone(),
            accounts.token_program.clone(),
            accounts.system_program.clone(),
            accounts.rent.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<
    &'me [AccountInfo<
        'info,
    >; INITIALIZE_CUSTOMIZABLE_PERMISSIONLESS_LB_PAIR_IX_ACCOUNTS_LEN],
> for InitializeCustomizablePermissionlessLbPairAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<
            'info,
        >; INITIALIZE_CUSTOMIZABLE_PERMISSIONLESS_LB_PAIR_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            lb_pair: &arr[0],
            bin_array_bitmap_extension: &arr[1],
            token_mint_x: &arr[2],
            token_mint_y: &arr[3],
            reserve_x: &arr[4],
            reserve_y: &arr[5],
            oracle: &arr[6],
            user_token_x: &arr[7],
            funder: &arr[8],
            token_program: &arr[9],
            system_program: &arr[10],
            rent: &arr[11],
            event_authority: &arr[12],
            program: &arr[13],
        }
    }
}
pub const INITIALIZE_CUSTOMIZABLE_PERMISSIONLESS_LB_PAIR_IX_DISCM: [u8; 8] = [
    46,
    39,
    41,
    135,
    111,
    183,
    200,
    64,
];
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InitializeCustomizablePermissionlessLbPairIxArgs {
    pub params: CustomizableParams,
}
#[derive(Clone, Debug, PartialEq)]
pub struct InitializeCustomizablePermissionlessLbPairIxData(
    pub InitializeCustomizablePermissionlessLbPairIxArgs,
);
impl From<InitializeCustomizablePermissionlessLbPairIxArgs>
for InitializeCustomizablePermissionlessLbPairIxData {
    fn from(args: InitializeCustomizablePermissionlessLbPairIxArgs) -> Self {
        Self(args)
    }
}
impl InitializeCustomizablePermissionlessLbPairIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != INITIALIZE_CUSTOMIZABLE_PERMISSIONLESS_LB_PAIR_IX_DISCM {
            return Err(
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!(
                        "discm does not match. Expected: {:?}. Received: {:?}",
                        INITIALIZE_CUSTOMIZABLE_PERMISSIONLESS_LB_PAIR_IX_DISCM,
                        maybe_discm
                    ),
                ),
            );
        }
        Ok(
            Self(
                InitializeCustomizablePermissionlessLbPairIxArgs::deserialize(
                    &mut reader,
                )?,
            ),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&INITIALIZE_CUSTOMIZABLE_PERMISSIONLESS_LB_PAIR_IX_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn initialize_customizable_permissionless_lb_pair_ix_with_program_id(
    program_id: Pubkey,
    keys: InitializeCustomizablePermissionlessLbPairKeys,
    args: InitializeCustomizablePermissionlessLbPairIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; INITIALIZE_CUSTOMIZABLE_PERMISSIONLESS_LB_PAIR_IX_ACCOUNTS_LEN] = keys
        .into();
    let data: InitializeCustomizablePermissionlessLbPairIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn initialize_customizable_permissionless_lb_pair_ix(
    keys: InitializeCustomizablePermissionlessLbPairKeys,
    args: InitializeCustomizablePermissionlessLbPairIxArgs,
) -> std::io::Result<Instruction> {
    initialize_customizable_permissionless_lb_pair_ix_with_program_id(
        crate::ID,
        keys,
        args,
    )
}
pub fn initialize_customizable_permissionless_lb_pair_invoke_with_program_id(
    program_id: Pubkey,
    accounts: InitializeCustomizablePermissionlessLbPairAccounts<'_, '_>,
    args: InitializeCustomizablePermissionlessLbPairIxArgs,
) -> ProgramResult {
    let keys: InitializeCustomizablePermissionlessLbPairKeys = accounts.into();
    let ix = initialize_customizable_permissionless_lb_pair_ix_with_program_id(
        program_id,
        keys,
        args,
    )?;
    invoke_instruction(&ix, accounts)
}
pub fn initialize_customizable_permissionless_lb_pair_invoke(
    accounts: InitializeCustomizablePermissionlessLbPairAccounts<'_, '_>,
    args: InitializeCustomizablePermissionlessLbPairIxArgs,
) -> ProgramResult {
    initialize_customizable_permissionless_lb_pair_invoke_with_program_id(
        crate::ID,
        accounts,
        args,
    )
}
pub fn initialize_customizable_permissionless_lb_pair_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: InitializeCustomizablePermissionlessLbPairAccounts<'_, '_>,
    args: InitializeCustomizablePermissionlessLbPairIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: InitializeCustomizablePermissionlessLbPairKeys = accounts.into();
    let ix = initialize_customizable_permissionless_lb_pair_ix_with_program_id(
        program_id,
        keys,
        args,
    )?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn initialize_customizable_permissionless_lb_pair_invoke_signed(
    accounts: InitializeCustomizablePermissionlessLbPairAccounts<'_, '_>,
    args: InitializeCustomizablePermissionlessLbPairIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    initialize_customizable_permissionless_lb_pair_invoke_signed_with_program_id(
        crate::ID,
        accounts,
        args,
        seeds,
    )
}
pub fn initialize_customizable_permissionless_lb_pair_verify_account_keys(
    accounts: InitializeCustomizablePermissionlessLbPairAccounts<'_, '_>,
    keys: InitializeCustomizablePermissionlessLbPairKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.lb_pair.key, keys.lb_pair),
        (*accounts.bin_array_bitmap_extension.key, keys.bin_array_bitmap_extension),
        (*accounts.token_mint_x.key, keys.token_mint_x),
        (*accounts.token_mint_y.key, keys.token_mint_y),
        (*accounts.reserve_x.key, keys.reserve_x),
        (*accounts.reserve_y.key, keys.reserve_y),
        (*accounts.oracle.key, keys.oracle),
        (*accounts.user_token_x.key, keys.user_token_x),
        (*accounts.funder.key, keys.funder),
        (*accounts.token_program.key, keys.token_program),
        (*accounts.system_program.key, keys.system_program),
        (*accounts.rent.key, keys.rent),
        (*accounts.event_authority.key, keys.event_authority),
        (*accounts.program.key, keys.program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn initialize_customizable_permissionless_lb_pair_verify_writable_privileges<
    'me,
    'info,
>(
    accounts: InitializeCustomizablePermissionlessLbPairAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.lb_pair,
        accounts.bin_array_bitmap_extension,
        accounts.reserve_x,
        accounts.reserve_y,
        accounts.oracle,
        accounts.funder,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn initialize_customizable_permissionless_lb_pair_verify_signer_privileges<
    'me,
    'info,
>(
    accounts: InitializeCustomizablePermissionlessLbPairAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.funder] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn initialize_customizable_permissionless_lb_pair_verify_account_privileges<
    'me,
    'info,
>(
    accounts: InitializeCustomizablePermissionlessLbPairAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    initialize_customizable_permissionless_lb_pair_verify_writable_privileges(accounts)?;
    initialize_customizable_permissionless_lb_pair_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const INITIALIZE_BIN_ARRAY_BITMAP_EXTENSION_IX_ACCOUNTS_LEN: usize = 5;
#[derive(Copy, Clone, Debug)]
pub struct InitializeBinArrayBitmapExtensionAccounts<'me, 'info> {
    pub lb_pair: &'me AccountInfo<'info>,
    pub bin_array_bitmap_extension: &'me AccountInfo<'info>,
    pub funder: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub rent: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct InitializeBinArrayBitmapExtensionKeys {
    pub lb_pair: Pubkey,
    pub bin_array_bitmap_extension: Pubkey,
    pub funder: Pubkey,
    pub system_program: Pubkey,
    pub rent: Pubkey,
}
impl From<InitializeBinArrayBitmapExtensionAccounts<'_, '_>>
for InitializeBinArrayBitmapExtensionKeys {
    fn from(accounts: InitializeBinArrayBitmapExtensionAccounts) -> Self {
        Self {
            lb_pair: *accounts.lb_pair.key,
            bin_array_bitmap_extension: *accounts.bin_array_bitmap_extension.key,
            funder: *accounts.funder.key,
            system_program: *accounts.system_program.key,
            rent: *accounts.rent.key,
        }
    }
}
impl From<InitializeBinArrayBitmapExtensionKeys>
for [AccountMeta; INITIALIZE_BIN_ARRAY_BITMAP_EXTENSION_IX_ACCOUNTS_LEN] {
    fn from(keys: InitializeBinArrayBitmapExtensionKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.lb_pair,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.bin_array_bitmap_extension,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.funder,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.system_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.rent,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; INITIALIZE_BIN_ARRAY_BITMAP_EXTENSION_IX_ACCOUNTS_LEN]>
for InitializeBinArrayBitmapExtensionKeys {
    fn from(
        pubkeys: [Pubkey; INITIALIZE_BIN_ARRAY_BITMAP_EXTENSION_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            lb_pair: pubkeys[0],
            bin_array_bitmap_extension: pubkeys[1],
            funder: pubkeys[2],
            system_program: pubkeys[3],
            rent: pubkeys[4],
        }
    }
}
impl<'info> From<InitializeBinArrayBitmapExtensionAccounts<'_, 'info>>
for [AccountInfo<'info>; INITIALIZE_BIN_ARRAY_BITMAP_EXTENSION_IX_ACCOUNTS_LEN] {
    fn from(accounts: InitializeBinArrayBitmapExtensionAccounts<'_, 'info>) -> Self {
        [
            accounts.lb_pair.clone(),
            accounts.bin_array_bitmap_extension.clone(),
            accounts.funder.clone(),
            accounts.system_program.clone(),
            accounts.rent.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; INITIALIZE_BIN_ARRAY_BITMAP_EXTENSION_IX_ACCOUNTS_LEN]>
for InitializeBinArrayBitmapExtensionAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<
            'info,
        >; INITIALIZE_BIN_ARRAY_BITMAP_EXTENSION_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            lb_pair: &arr[0],
            bin_array_bitmap_extension: &arr[1],
            funder: &arr[2],
            system_program: &arr[3],
            rent: &arr[4],
        }
    }
}
pub const INITIALIZE_BIN_ARRAY_BITMAP_EXTENSION_IX_DISCM: [u8; 8] = [
    47,
    157,
    226,
    180,
    12,
    240,
    33,
    71,
];
#[derive(Clone, Debug, PartialEq)]
pub struct InitializeBinArrayBitmapExtensionIxData;
impl InitializeBinArrayBitmapExtensionIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != INITIALIZE_BIN_ARRAY_BITMAP_EXTENSION_IX_DISCM {
            return Err(
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!(
                        "discm does not match. Expected: {:?}. Received: {:?}",
                        INITIALIZE_BIN_ARRAY_BITMAP_EXTENSION_IX_DISCM, maybe_discm
                    ),
                ),
            );
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&INITIALIZE_BIN_ARRAY_BITMAP_EXTENSION_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn initialize_bin_array_bitmap_extension_ix_with_program_id(
    program_id: Pubkey,
    keys: InitializeBinArrayBitmapExtensionKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; INITIALIZE_BIN_ARRAY_BITMAP_EXTENSION_IX_ACCOUNTS_LEN] = keys
        .into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: InitializeBinArrayBitmapExtensionIxData.try_to_vec()?,
    })
}
pub fn initialize_bin_array_bitmap_extension_ix(
    keys: InitializeBinArrayBitmapExtensionKeys,
) -> std::io::Result<Instruction> {
    initialize_bin_array_bitmap_extension_ix_with_program_id(crate::ID, keys)
}
pub fn initialize_bin_array_bitmap_extension_invoke_with_program_id(
    program_id: Pubkey,
    accounts: InitializeBinArrayBitmapExtensionAccounts<'_, '_>,
) -> ProgramResult {
    let keys: InitializeBinArrayBitmapExtensionKeys = accounts.into();
    let ix = initialize_bin_array_bitmap_extension_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn initialize_bin_array_bitmap_extension_invoke(
    accounts: InitializeBinArrayBitmapExtensionAccounts<'_, '_>,
) -> ProgramResult {
    initialize_bin_array_bitmap_extension_invoke_with_program_id(crate::ID, accounts)
}
pub fn initialize_bin_array_bitmap_extension_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: InitializeBinArrayBitmapExtensionAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: InitializeBinArrayBitmapExtensionKeys = accounts.into();
    let ix = initialize_bin_array_bitmap_extension_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn initialize_bin_array_bitmap_extension_invoke_signed(
    accounts: InitializeBinArrayBitmapExtensionAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    initialize_bin_array_bitmap_extension_invoke_signed_with_program_id(
        crate::ID,
        accounts,
        seeds,
    )
}
pub fn initialize_bin_array_bitmap_extension_verify_account_keys(
    accounts: InitializeBinArrayBitmapExtensionAccounts<'_, '_>,
    keys: InitializeBinArrayBitmapExtensionKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.lb_pair.key, keys.lb_pair),
        (*accounts.bin_array_bitmap_extension.key, keys.bin_array_bitmap_extension),
        (*accounts.funder.key, keys.funder),
        (*accounts.system_program.key, keys.system_program),
        (*accounts.rent.key, keys.rent),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn initialize_bin_array_bitmap_extension_verify_writable_privileges<'me, 'info>(
    accounts: InitializeBinArrayBitmapExtensionAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.bin_array_bitmap_extension, accounts.funder] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn initialize_bin_array_bitmap_extension_verify_signer_privileges<'me, 'info>(
    accounts: InitializeBinArrayBitmapExtensionAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.funder] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn initialize_bin_array_bitmap_extension_verify_account_privileges<'me, 'info>(
    accounts: InitializeBinArrayBitmapExtensionAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    initialize_bin_array_bitmap_extension_verify_writable_privileges(accounts)?;
    initialize_bin_array_bitmap_extension_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const INITIALIZE_BIN_ARRAY_IX_ACCOUNTS_LEN: usize = 4;
#[derive(Copy, Clone, Debug)]
pub struct InitializeBinArrayAccounts<'me, 'info> {
    pub lb_pair: &'me AccountInfo<'info>,
    pub bin_array: &'me AccountInfo<'info>,
    pub funder: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct InitializeBinArrayKeys {
    pub lb_pair: Pubkey,
    pub bin_array: Pubkey,
    pub funder: Pubkey,
    pub system_program: Pubkey,
}
impl From<InitializeBinArrayAccounts<'_, '_>> for InitializeBinArrayKeys {
    fn from(accounts: InitializeBinArrayAccounts) -> Self {
        Self {
            lb_pair: *accounts.lb_pair.key,
            bin_array: *accounts.bin_array.key,
            funder: *accounts.funder.key,
            system_program: *accounts.system_program.key,
        }
    }
}
impl From<InitializeBinArrayKeys>
for [AccountMeta; INITIALIZE_BIN_ARRAY_IX_ACCOUNTS_LEN] {
    fn from(keys: InitializeBinArrayKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.lb_pair,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.bin_array,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.funder,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.system_program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; INITIALIZE_BIN_ARRAY_IX_ACCOUNTS_LEN]> for InitializeBinArrayKeys {
    fn from(pubkeys: [Pubkey; INITIALIZE_BIN_ARRAY_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            lb_pair: pubkeys[0],
            bin_array: pubkeys[1],
            funder: pubkeys[2],
            system_program: pubkeys[3],
        }
    }
}
impl<'info> From<InitializeBinArrayAccounts<'_, 'info>>
for [AccountInfo<'info>; INITIALIZE_BIN_ARRAY_IX_ACCOUNTS_LEN] {
    fn from(accounts: InitializeBinArrayAccounts<'_, 'info>) -> Self {
        [
            accounts.lb_pair.clone(),
            accounts.bin_array.clone(),
            accounts.funder.clone(),
            accounts.system_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; INITIALIZE_BIN_ARRAY_IX_ACCOUNTS_LEN]>
for InitializeBinArrayAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; INITIALIZE_BIN_ARRAY_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            lb_pair: &arr[0],
            bin_array: &arr[1],
            funder: &arr[2],
            system_program: &arr[3],
        }
    }
}
pub const INITIALIZE_BIN_ARRAY_IX_DISCM: [u8; 8] = [35, 86, 19, 185, 78, 212, 75, 211];
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InitializeBinArrayIxArgs {
    pub index: i64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct InitializeBinArrayIxData(pub InitializeBinArrayIxArgs);
impl From<InitializeBinArrayIxArgs> for InitializeBinArrayIxData {
    fn from(args: InitializeBinArrayIxArgs) -> Self {
        Self(args)
    }
}
impl InitializeBinArrayIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != INITIALIZE_BIN_ARRAY_IX_DISCM {
            return Err(
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!(
                        "discm does not match. Expected: {:?}. Received: {:?}",
                        INITIALIZE_BIN_ARRAY_IX_DISCM, maybe_discm
                    ),
                ),
            );
        }
        Ok(Self(InitializeBinArrayIxArgs::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&INITIALIZE_BIN_ARRAY_IX_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn initialize_bin_array_ix_with_program_id(
    program_id: Pubkey,
    keys: InitializeBinArrayKeys,
    args: InitializeBinArrayIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; INITIALIZE_BIN_ARRAY_IX_ACCOUNTS_LEN] = keys.into();
    let data: InitializeBinArrayIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn initialize_bin_array_ix(
    keys: InitializeBinArrayKeys,
    args: InitializeBinArrayIxArgs,
) -> std::io::Result<Instruction> {
    initialize_bin_array_ix_with_program_id(crate::ID, keys, args)
}
pub fn initialize_bin_array_invoke_with_program_id(
    program_id: Pubkey,
    accounts: InitializeBinArrayAccounts<'_, '_>,
    args: InitializeBinArrayIxArgs,
) -> ProgramResult {
    let keys: InitializeBinArrayKeys = accounts.into();
    let ix = initialize_bin_array_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn initialize_bin_array_invoke(
    accounts: InitializeBinArrayAccounts<'_, '_>,
    args: InitializeBinArrayIxArgs,
) -> ProgramResult {
    initialize_bin_array_invoke_with_program_id(crate::ID, accounts, args)
}
pub fn initialize_bin_array_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: InitializeBinArrayAccounts<'_, '_>,
    args: InitializeBinArrayIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: InitializeBinArrayKeys = accounts.into();
    let ix = initialize_bin_array_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn initialize_bin_array_invoke_signed(
    accounts: InitializeBinArrayAccounts<'_, '_>,
    args: InitializeBinArrayIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    initialize_bin_array_invoke_signed_with_program_id(crate::ID, accounts, args, seeds)
}
pub fn initialize_bin_array_verify_account_keys(
    accounts: InitializeBinArrayAccounts<'_, '_>,
    keys: InitializeBinArrayKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.lb_pair.key, keys.lb_pair),
        (*accounts.bin_array.key, keys.bin_array),
        (*accounts.funder.key, keys.funder),
        (*accounts.system_program.key, keys.system_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn initialize_bin_array_verify_writable_privileges<'me, 'info>(
    accounts: InitializeBinArrayAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.bin_array, accounts.funder] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn initialize_bin_array_verify_signer_privileges<'me, 'info>(
    accounts: InitializeBinArrayAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.funder] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn initialize_bin_array_verify_account_privileges<'me, 'info>(
    accounts: InitializeBinArrayAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    initialize_bin_array_verify_writable_privileges(accounts)?;
    initialize_bin_array_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const ADD_LIQUIDITY_IX_ACCOUNTS_LEN: usize = 16;
#[derive(Copy, Clone, Debug)]
pub struct AddLiquidityAccounts<'me, 'info> {
    pub position: &'me AccountInfo<'info>,
    pub lb_pair: &'me AccountInfo<'info>,
    pub bin_array_bitmap_extension: &'me AccountInfo<'info>,
    pub user_token_x: &'me AccountInfo<'info>,
    pub user_token_y: &'me AccountInfo<'info>,
    pub reserve_x: &'me AccountInfo<'info>,
    pub reserve_y: &'me AccountInfo<'info>,
    pub token_x_mint: &'me AccountInfo<'info>,
    pub token_y_mint: &'me AccountInfo<'info>,
    pub bin_array_lower: &'me AccountInfo<'info>,
    pub bin_array_upper: &'me AccountInfo<'info>,
    pub sender: &'me AccountInfo<'info>,
    pub token_x_program: &'me AccountInfo<'info>,
    pub token_y_program: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct AddLiquidityKeys {
    pub position: Pubkey,
    pub lb_pair: Pubkey,
    pub bin_array_bitmap_extension: Pubkey,
    pub user_token_x: Pubkey,
    pub user_token_y: Pubkey,
    pub reserve_x: Pubkey,
    pub reserve_y: Pubkey,
    pub token_x_mint: Pubkey,
    pub token_y_mint: Pubkey,
    pub bin_array_lower: Pubkey,
    pub bin_array_upper: Pubkey,
    pub sender: Pubkey,
    pub token_x_program: Pubkey,
    pub token_y_program: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<AddLiquidityAccounts<'_, '_>> for AddLiquidityKeys {
    fn from(accounts: AddLiquidityAccounts) -> Self {
        Self {
            position: *accounts.position.key,
            lb_pair: *accounts.lb_pair.key,
            bin_array_bitmap_extension: *accounts.bin_array_bitmap_extension.key,
            user_token_x: *accounts.user_token_x.key,
            user_token_y: *accounts.user_token_y.key,
            reserve_x: *accounts.reserve_x.key,
            reserve_y: *accounts.reserve_y.key,
            token_x_mint: *accounts.token_x_mint.key,
            token_y_mint: *accounts.token_y_mint.key,
            bin_array_lower: *accounts.bin_array_lower.key,
            bin_array_upper: *accounts.bin_array_upper.key,
            sender: *accounts.sender.key,
            token_x_program: *accounts.token_x_program.key,
            token_y_program: *accounts.token_y_program.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<AddLiquidityKeys> for [AccountMeta; ADD_LIQUIDITY_IX_ACCOUNTS_LEN] {
    fn from(keys: AddLiquidityKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.position,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.lb_pair,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.bin_array_bitmap_extension,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_token_x,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_token_y,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.reserve_x,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.reserve_y,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_x_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_y_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.bin_array_lower,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.bin_array_upper,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.sender,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_x_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_y_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.event_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; ADD_LIQUIDITY_IX_ACCOUNTS_LEN]> for AddLiquidityKeys {
    fn from(pubkeys: [Pubkey; ADD_LIQUIDITY_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            position: pubkeys[0],
            lb_pair: pubkeys[1],
            bin_array_bitmap_extension: pubkeys[2],
            user_token_x: pubkeys[3],
            user_token_y: pubkeys[4],
            reserve_x: pubkeys[5],
            reserve_y: pubkeys[6],
            token_x_mint: pubkeys[7],
            token_y_mint: pubkeys[8],
            bin_array_lower: pubkeys[9],
            bin_array_upper: pubkeys[10],
            sender: pubkeys[11],
            token_x_program: pubkeys[12],
            token_y_program: pubkeys[13],
            event_authority: pubkeys[14],
            program: pubkeys[15],
        }
    }
}
impl<'info> From<AddLiquidityAccounts<'_, 'info>>
for [AccountInfo<'info>; ADD_LIQUIDITY_IX_ACCOUNTS_LEN] {
    fn from(accounts: AddLiquidityAccounts<'_, 'info>) -> Self {
        [
            accounts.position.clone(),
            accounts.lb_pair.clone(),
            accounts.bin_array_bitmap_extension.clone(),
            accounts.user_token_x.clone(),
            accounts.user_token_y.clone(),
            accounts.reserve_x.clone(),
            accounts.reserve_y.clone(),
            accounts.token_x_mint.clone(),
            accounts.token_y_mint.clone(),
            accounts.bin_array_lower.clone(),
            accounts.bin_array_upper.clone(),
            accounts.sender.clone(),
            accounts.token_x_program.clone(),
            accounts.token_y_program.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; ADD_LIQUIDITY_IX_ACCOUNTS_LEN]>
for AddLiquidityAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; ADD_LIQUIDITY_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            position: &arr[0],
            lb_pair: &arr[1],
            bin_array_bitmap_extension: &arr[2],
            user_token_x: &arr[3],
            user_token_y: &arr[4],
            reserve_x: &arr[5],
            reserve_y: &arr[6],
            token_x_mint: &arr[7],
            token_y_mint: &arr[8],
            bin_array_lower: &arr[9],
            bin_array_upper: &arr[10],
            sender: &arr[11],
            token_x_program: &arr[12],
            token_y_program: &arr[13],
            event_authority: &arr[14],
            program: &arr[15],
        }
    }
}
pub const ADD_LIQUIDITY_IX_DISCM: [u8; 8] = [181, 157, 89, 67, 143, 182, 52, 72];
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct AddLiquidityIxArgs {
    pub liquidity_parameter: LiquidityParameter,
}
#[derive(Clone, Debug, PartialEq)]
pub struct AddLiquidityIxData(pub AddLiquidityIxArgs);
impl From<AddLiquidityIxArgs> for AddLiquidityIxData {
    fn from(args: AddLiquidityIxArgs) -> Self {
        Self(args)
    }
}
impl AddLiquidityIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != ADD_LIQUIDITY_IX_DISCM {
            return Err(
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!(
                        "discm does not match. Expected: {:?}. Received: {:?}",
                        ADD_LIQUIDITY_IX_DISCM, maybe_discm
                    ),
                ),
            );
        }
        Ok(Self(AddLiquidityIxArgs::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&ADD_LIQUIDITY_IX_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn add_liquidity_ix_with_program_id(
    program_id: Pubkey,
    keys: AddLiquidityKeys,
    args: AddLiquidityIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; ADD_LIQUIDITY_IX_ACCOUNTS_LEN] = keys.into();
    let data: AddLiquidityIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn add_liquidity_ix(
    keys: AddLiquidityKeys,
    args: AddLiquidityIxArgs,
) -> std::io::Result<Instruction> {
    add_liquidity_ix_with_program_id(crate::ID, keys, args)
}
pub fn add_liquidity_invoke_with_program_id(
    program_id: Pubkey,
    accounts: AddLiquidityAccounts<'_, '_>,
    args: AddLiquidityIxArgs,
) -> ProgramResult {
    let keys: AddLiquidityKeys = accounts.into();
    let ix = add_liquidity_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn add_liquidity_invoke(
    accounts: AddLiquidityAccounts<'_, '_>,
    args: AddLiquidityIxArgs,
) -> ProgramResult {
    add_liquidity_invoke_with_program_id(crate::ID, accounts, args)
}
pub fn add_liquidity_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: AddLiquidityAccounts<'_, '_>,
    args: AddLiquidityIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: AddLiquidityKeys = accounts.into();
    let ix = add_liquidity_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn add_liquidity_invoke_signed(
    accounts: AddLiquidityAccounts<'_, '_>,
    args: AddLiquidityIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    add_liquidity_invoke_signed_with_program_id(crate::ID, accounts, args, seeds)
}
pub fn add_liquidity_verify_account_keys(
    accounts: AddLiquidityAccounts<'_, '_>,
    keys: AddLiquidityKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.position.key, keys.position),
        (*accounts.lb_pair.key, keys.lb_pair),
        (*accounts.bin_array_bitmap_extension.key, keys.bin_array_bitmap_extension),
        (*accounts.user_token_x.key, keys.user_token_x),
        (*accounts.user_token_y.key, keys.user_token_y),
        (*accounts.reserve_x.key, keys.reserve_x),
        (*accounts.reserve_y.key, keys.reserve_y),
        (*accounts.token_x_mint.key, keys.token_x_mint),
        (*accounts.token_y_mint.key, keys.token_y_mint),
        (*accounts.bin_array_lower.key, keys.bin_array_lower),
        (*accounts.bin_array_upper.key, keys.bin_array_upper),
        (*accounts.sender.key, keys.sender),
        (*accounts.token_x_program.key, keys.token_x_program),
        (*accounts.token_y_program.key, keys.token_y_program),
        (*accounts.event_authority.key, keys.event_authority),
        (*accounts.program.key, keys.program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn add_liquidity_verify_writable_privileges<'me, 'info>(
    accounts: AddLiquidityAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.position,
        accounts.lb_pair,
        accounts.bin_array_bitmap_extension,
        accounts.user_token_x,
        accounts.user_token_y,
        accounts.reserve_x,
        accounts.reserve_y,
        accounts.bin_array_lower,
        accounts.bin_array_upper,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn add_liquidity_verify_signer_privileges<'me, 'info>(
    accounts: AddLiquidityAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.sender] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn add_liquidity_verify_account_privileges<'me, 'info>(
    accounts: AddLiquidityAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    add_liquidity_verify_writable_privileges(accounts)?;
    add_liquidity_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const ADD_LIQUIDITY_BY_WEIGHT_IX_ACCOUNTS_LEN: usize = 16;
#[derive(Copy, Clone, Debug)]
pub struct AddLiquidityByWeightAccounts<'me, 'info> {
    pub position: &'me AccountInfo<'info>,
    pub lb_pair: &'me AccountInfo<'info>,
    pub bin_array_bitmap_extension: &'me AccountInfo<'info>,
    pub user_token_x: &'me AccountInfo<'info>,
    pub user_token_y: &'me AccountInfo<'info>,
    pub reserve_x: &'me AccountInfo<'info>,
    pub reserve_y: &'me AccountInfo<'info>,
    pub token_x_mint: &'me AccountInfo<'info>,
    pub token_y_mint: &'me AccountInfo<'info>,
    pub bin_array_lower: &'me AccountInfo<'info>,
    pub bin_array_upper: &'me AccountInfo<'info>,
    pub sender: &'me AccountInfo<'info>,
    pub token_x_program: &'me AccountInfo<'info>,
    pub token_y_program: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct AddLiquidityByWeightKeys {
    pub position: Pubkey,
    pub lb_pair: Pubkey,
    pub bin_array_bitmap_extension: Pubkey,
    pub user_token_x: Pubkey,
    pub user_token_y: Pubkey,
    pub reserve_x: Pubkey,
    pub reserve_y: Pubkey,
    pub token_x_mint: Pubkey,
    pub token_y_mint: Pubkey,
    pub bin_array_lower: Pubkey,
    pub bin_array_upper: Pubkey,
    pub sender: Pubkey,
    pub token_x_program: Pubkey,
    pub token_y_program: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<AddLiquidityByWeightAccounts<'_, '_>> for AddLiquidityByWeightKeys {
    fn from(accounts: AddLiquidityByWeightAccounts) -> Self {
        Self {
            position: *accounts.position.key,
            lb_pair: *accounts.lb_pair.key,
            bin_array_bitmap_extension: *accounts.bin_array_bitmap_extension.key,
            user_token_x: *accounts.user_token_x.key,
            user_token_y: *accounts.user_token_y.key,
            reserve_x: *accounts.reserve_x.key,
            reserve_y: *accounts.reserve_y.key,
            token_x_mint: *accounts.token_x_mint.key,
            token_y_mint: *accounts.token_y_mint.key,
            bin_array_lower: *accounts.bin_array_lower.key,
            bin_array_upper: *accounts.bin_array_upper.key,
            sender: *accounts.sender.key,
            token_x_program: *accounts.token_x_program.key,
            token_y_program: *accounts.token_y_program.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<AddLiquidityByWeightKeys>
for [AccountMeta; ADD_LIQUIDITY_BY_WEIGHT_IX_ACCOUNTS_LEN] {
    fn from(keys: AddLiquidityByWeightKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.position,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.lb_pair,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.bin_array_bitmap_extension,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_token_x,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_token_y,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.reserve_x,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.reserve_y,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_x_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_y_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.bin_array_lower,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.bin_array_upper,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.sender,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_x_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_y_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.event_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; ADD_LIQUIDITY_BY_WEIGHT_IX_ACCOUNTS_LEN]>
for AddLiquidityByWeightKeys {
    fn from(pubkeys: [Pubkey; ADD_LIQUIDITY_BY_WEIGHT_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            position: pubkeys[0],
            lb_pair: pubkeys[1],
            bin_array_bitmap_extension: pubkeys[2],
            user_token_x: pubkeys[3],
            user_token_y: pubkeys[4],
            reserve_x: pubkeys[5],
            reserve_y: pubkeys[6],
            token_x_mint: pubkeys[7],
            token_y_mint: pubkeys[8],
            bin_array_lower: pubkeys[9],
            bin_array_upper: pubkeys[10],
            sender: pubkeys[11],
            token_x_program: pubkeys[12],
            token_y_program: pubkeys[13],
            event_authority: pubkeys[14],
            program: pubkeys[15],
        }
    }
}
impl<'info> From<AddLiquidityByWeightAccounts<'_, 'info>>
for [AccountInfo<'info>; ADD_LIQUIDITY_BY_WEIGHT_IX_ACCOUNTS_LEN] {
    fn from(accounts: AddLiquidityByWeightAccounts<'_, 'info>) -> Self {
        [
            accounts.position.clone(),
            accounts.lb_pair.clone(),
            accounts.bin_array_bitmap_extension.clone(),
            accounts.user_token_x.clone(),
            accounts.user_token_y.clone(),
            accounts.reserve_x.clone(),
            accounts.reserve_y.clone(),
            accounts.token_x_mint.clone(),
            accounts.token_y_mint.clone(),
            accounts.bin_array_lower.clone(),
            accounts.bin_array_upper.clone(),
            accounts.sender.clone(),
            accounts.token_x_program.clone(),
            accounts.token_y_program.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; ADD_LIQUIDITY_BY_WEIGHT_IX_ACCOUNTS_LEN]>
for AddLiquidityByWeightAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; ADD_LIQUIDITY_BY_WEIGHT_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            position: &arr[0],
            lb_pair: &arr[1],
            bin_array_bitmap_extension: &arr[2],
            user_token_x: &arr[3],
            user_token_y: &arr[4],
            reserve_x: &arr[5],
            reserve_y: &arr[6],
            token_x_mint: &arr[7],
            token_y_mint: &arr[8],
            bin_array_lower: &arr[9],
            bin_array_upper: &arr[10],
            sender: &arr[11],
            token_x_program: &arr[12],
            token_y_program: &arr[13],
            event_authority: &arr[14],
            program: &arr[15],
        }
    }
}
pub const ADD_LIQUIDITY_BY_WEIGHT_IX_DISCM: [u8; 8] = [
    28,
    140,
    238,
    99,
    231,
    162,
    21,
    149,
];
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct AddLiquidityByWeightIxArgs {
    pub liquidity_parameter: LiquidityParameterByWeight,
}
#[derive(Clone, Debug, PartialEq)]
pub struct AddLiquidityByWeightIxData(pub AddLiquidityByWeightIxArgs);
impl From<AddLiquidityByWeightIxArgs> for AddLiquidityByWeightIxData {
    fn from(args: AddLiquidityByWeightIxArgs) -> Self {
        Self(args)
    }
}
impl AddLiquidityByWeightIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != ADD_LIQUIDITY_BY_WEIGHT_IX_DISCM {
            return Err(
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!(
                        "discm does not match. Expected: {:?}. Received: {:?}",
                        ADD_LIQUIDITY_BY_WEIGHT_IX_DISCM, maybe_discm
                    ),
                ),
            );
        }
        Ok(Self(AddLiquidityByWeightIxArgs::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&ADD_LIQUIDITY_BY_WEIGHT_IX_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn add_liquidity_by_weight_ix_with_program_id(
    program_id: Pubkey,
    keys: AddLiquidityByWeightKeys,
    args: AddLiquidityByWeightIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; ADD_LIQUIDITY_BY_WEIGHT_IX_ACCOUNTS_LEN] = keys.into();
    let data: AddLiquidityByWeightIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn add_liquidity_by_weight_ix(
    keys: AddLiquidityByWeightKeys,
    args: AddLiquidityByWeightIxArgs,
) -> std::io::Result<Instruction> {
    add_liquidity_by_weight_ix_with_program_id(crate::ID, keys, args)
}
pub fn add_liquidity_by_weight_invoke_with_program_id(
    program_id: Pubkey,
    accounts: AddLiquidityByWeightAccounts<'_, '_>,
    args: AddLiquidityByWeightIxArgs,
) -> ProgramResult {
    let keys: AddLiquidityByWeightKeys = accounts.into();
    let ix = add_liquidity_by_weight_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn add_liquidity_by_weight_invoke(
    accounts: AddLiquidityByWeightAccounts<'_, '_>,
    args: AddLiquidityByWeightIxArgs,
) -> ProgramResult {
    add_liquidity_by_weight_invoke_with_program_id(crate::ID, accounts, args)
}
pub fn add_liquidity_by_weight_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: AddLiquidityByWeightAccounts<'_, '_>,
    args: AddLiquidityByWeightIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: AddLiquidityByWeightKeys = accounts.into();
    let ix = add_liquidity_by_weight_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn add_liquidity_by_weight_invoke_signed(
    accounts: AddLiquidityByWeightAccounts<'_, '_>,
    args: AddLiquidityByWeightIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    add_liquidity_by_weight_invoke_signed_with_program_id(
        crate::ID,
        accounts,
        args,
        seeds,
    )
}
pub fn add_liquidity_by_weight_verify_account_keys(
    accounts: AddLiquidityByWeightAccounts<'_, '_>,
    keys: AddLiquidityByWeightKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.position.key, keys.position),
        (*accounts.lb_pair.key, keys.lb_pair),
        (*accounts.bin_array_bitmap_extension.key, keys.bin_array_bitmap_extension),
        (*accounts.user_token_x.key, keys.user_token_x),
        (*accounts.user_token_y.key, keys.user_token_y),
        (*accounts.reserve_x.key, keys.reserve_x),
        (*accounts.reserve_y.key, keys.reserve_y),
        (*accounts.token_x_mint.key, keys.token_x_mint),
        (*accounts.token_y_mint.key, keys.token_y_mint),
        (*accounts.bin_array_lower.key, keys.bin_array_lower),
        (*accounts.bin_array_upper.key, keys.bin_array_upper),
        (*accounts.sender.key, keys.sender),
        (*accounts.token_x_program.key, keys.token_x_program),
        (*accounts.token_y_program.key, keys.token_y_program),
        (*accounts.event_authority.key, keys.event_authority),
        (*accounts.program.key, keys.program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn add_liquidity_by_weight_verify_writable_privileges<'me, 'info>(
    accounts: AddLiquidityByWeightAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.position,
        accounts.lb_pair,
        accounts.bin_array_bitmap_extension,
        accounts.user_token_x,
        accounts.user_token_y,
        accounts.reserve_x,
        accounts.reserve_y,
        accounts.bin_array_lower,
        accounts.bin_array_upper,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn add_liquidity_by_weight_verify_signer_privileges<'me, 'info>(
    accounts: AddLiquidityByWeightAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.sender] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn add_liquidity_by_weight_verify_account_privileges<'me, 'info>(
    accounts: AddLiquidityByWeightAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    add_liquidity_by_weight_verify_writable_privileges(accounts)?;
    add_liquidity_by_weight_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const ADD_LIQUIDITY_BY_STRATEGY_IX_ACCOUNTS_LEN: usize = 16;
#[derive(Copy, Clone, Debug)]
pub struct AddLiquidityByStrategyAccounts<'me, 'info> {
    pub position: &'me AccountInfo<'info>,
    pub lb_pair: &'me AccountInfo<'info>,
    pub bin_array_bitmap_extension: &'me AccountInfo<'info>,
    pub user_token_x: &'me AccountInfo<'info>,
    pub user_token_y: &'me AccountInfo<'info>,
    pub reserve_x: &'me AccountInfo<'info>,
    pub reserve_y: &'me AccountInfo<'info>,
    pub token_x_mint: &'me AccountInfo<'info>,
    pub token_y_mint: &'me AccountInfo<'info>,
    pub bin_array_lower: &'me AccountInfo<'info>,
    pub bin_array_upper: &'me AccountInfo<'info>,
    pub sender: &'me AccountInfo<'info>,
    pub token_x_program: &'me AccountInfo<'info>,
    pub token_y_program: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct AddLiquidityByStrategyKeys {
    pub position: Pubkey,
    pub lb_pair: Pubkey,
    pub bin_array_bitmap_extension: Pubkey,
    pub user_token_x: Pubkey,
    pub user_token_y: Pubkey,
    pub reserve_x: Pubkey,
    pub reserve_y: Pubkey,
    pub token_x_mint: Pubkey,
    pub token_y_mint: Pubkey,
    pub bin_array_lower: Pubkey,
    pub bin_array_upper: Pubkey,
    pub sender: Pubkey,
    pub token_x_program: Pubkey,
    pub token_y_program: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<AddLiquidityByStrategyAccounts<'_, '_>> for AddLiquidityByStrategyKeys {
    fn from(accounts: AddLiquidityByStrategyAccounts) -> Self {
        Self {
            position: *accounts.position.key,
            lb_pair: *accounts.lb_pair.key,
            bin_array_bitmap_extension: *accounts.bin_array_bitmap_extension.key,
            user_token_x: *accounts.user_token_x.key,
            user_token_y: *accounts.user_token_y.key,
            reserve_x: *accounts.reserve_x.key,
            reserve_y: *accounts.reserve_y.key,
            token_x_mint: *accounts.token_x_mint.key,
            token_y_mint: *accounts.token_y_mint.key,
            bin_array_lower: *accounts.bin_array_lower.key,
            bin_array_upper: *accounts.bin_array_upper.key,
            sender: *accounts.sender.key,
            token_x_program: *accounts.token_x_program.key,
            token_y_program: *accounts.token_y_program.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<AddLiquidityByStrategyKeys>
for [AccountMeta; ADD_LIQUIDITY_BY_STRATEGY_IX_ACCOUNTS_LEN] {
    fn from(keys: AddLiquidityByStrategyKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.position,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.lb_pair,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.bin_array_bitmap_extension,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_token_x,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_token_y,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.reserve_x,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.reserve_y,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_x_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_y_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.bin_array_lower,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.bin_array_upper,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.sender,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_x_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_y_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.event_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; ADD_LIQUIDITY_BY_STRATEGY_IX_ACCOUNTS_LEN]>
for AddLiquidityByStrategyKeys {
    fn from(pubkeys: [Pubkey; ADD_LIQUIDITY_BY_STRATEGY_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            position: pubkeys[0],
            lb_pair: pubkeys[1],
            bin_array_bitmap_extension: pubkeys[2],
            user_token_x: pubkeys[3],
            user_token_y: pubkeys[4],
            reserve_x: pubkeys[5],
            reserve_y: pubkeys[6],
            token_x_mint: pubkeys[7],
            token_y_mint: pubkeys[8],
            bin_array_lower: pubkeys[9],
            bin_array_upper: pubkeys[10],
            sender: pubkeys[11],
            token_x_program: pubkeys[12],
            token_y_program: pubkeys[13],
            event_authority: pubkeys[14],
            program: pubkeys[15],
        }
    }
}
impl<'info> From<AddLiquidityByStrategyAccounts<'_, 'info>>
for [AccountInfo<'info>; ADD_LIQUIDITY_BY_STRATEGY_IX_ACCOUNTS_LEN] {
    fn from(accounts: AddLiquidityByStrategyAccounts<'_, 'info>) -> Self {
        [
            accounts.position.clone(),
            accounts.lb_pair.clone(),
            accounts.bin_array_bitmap_extension.clone(),
            accounts.user_token_x.clone(),
            accounts.user_token_y.clone(),
            accounts.reserve_x.clone(),
            accounts.reserve_y.clone(),
            accounts.token_x_mint.clone(),
            accounts.token_y_mint.clone(),
            accounts.bin_array_lower.clone(),
            accounts.bin_array_upper.clone(),
            accounts.sender.clone(),
            accounts.token_x_program.clone(),
            accounts.token_y_program.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; ADD_LIQUIDITY_BY_STRATEGY_IX_ACCOUNTS_LEN]>
for AddLiquidityByStrategyAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; ADD_LIQUIDITY_BY_STRATEGY_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            position: &arr[0],
            lb_pair: &arr[1],
            bin_array_bitmap_extension: &arr[2],
            user_token_x: &arr[3],
            user_token_y: &arr[4],
            reserve_x: &arr[5],
            reserve_y: &arr[6],
            token_x_mint: &arr[7],
            token_y_mint: &arr[8],
            bin_array_lower: &arr[9],
            bin_array_upper: &arr[10],
            sender: &arr[11],
            token_x_program: &arr[12],
            token_y_program: &arr[13],
            event_authority: &arr[14],
            program: &arr[15],
        }
    }
}
pub const ADD_LIQUIDITY_BY_STRATEGY_IX_DISCM: [u8; 8] = [
    7,
    3,
    150,
    127,
    148,
    40,
    61,
    200,
];
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct AddLiquidityByStrategyIxArgs {
    pub liquidity_parameter: LiquidityParameterByStrategy,
}
#[derive(Clone, Debug, PartialEq)]
pub struct AddLiquidityByStrategyIxData(pub AddLiquidityByStrategyIxArgs);
impl From<AddLiquidityByStrategyIxArgs> for AddLiquidityByStrategyIxData {
    fn from(args: AddLiquidityByStrategyIxArgs) -> Self {
        Self(args)
    }
}
impl AddLiquidityByStrategyIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != ADD_LIQUIDITY_BY_STRATEGY_IX_DISCM {
            return Err(
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!(
                        "discm does not match. Expected: {:?}. Received: {:?}",
                        ADD_LIQUIDITY_BY_STRATEGY_IX_DISCM, maybe_discm
                    ),
                ),
            );
        }
        Ok(Self(AddLiquidityByStrategyIxArgs::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&ADD_LIQUIDITY_BY_STRATEGY_IX_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn add_liquidity_by_strategy_ix_with_program_id(
    program_id: Pubkey,
    keys: AddLiquidityByStrategyKeys,
    args: AddLiquidityByStrategyIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; ADD_LIQUIDITY_BY_STRATEGY_IX_ACCOUNTS_LEN] = keys.into();
    let data: AddLiquidityByStrategyIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn add_liquidity_by_strategy_ix(
    keys: AddLiquidityByStrategyKeys,
    args: AddLiquidityByStrategyIxArgs,
) -> std::io::Result<Instruction> {
    add_liquidity_by_strategy_ix_with_program_id(crate::ID, keys, args)
}
pub fn add_liquidity_by_strategy_invoke_with_program_id(
    program_id: Pubkey,
    accounts: AddLiquidityByStrategyAccounts<'_, '_>,
    args: AddLiquidityByStrategyIxArgs,
) -> ProgramResult {
    let keys: AddLiquidityByStrategyKeys = accounts.into();
    let ix = add_liquidity_by_strategy_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn add_liquidity_by_strategy_invoke(
    accounts: AddLiquidityByStrategyAccounts<'_, '_>,
    args: AddLiquidityByStrategyIxArgs,
) -> ProgramResult {
    add_liquidity_by_strategy_invoke_with_program_id(crate::ID, accounts, args)
}
pub fn add_liquidity_by_strategy_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: AddLiquidityByStrategyAccounts<'_, '_>,
    args: AddLiquidityByStrategyIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: AddLiquidityByStrategyKeys = accounts.into();
    let ix = add_liquidity_by_strategy_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn add_liquidity_by_strategy_invoke_signed(
    accounts: AddLiquidityByStrategyAccounts<'_, '_>,
    args: AddLiquidityByStrategyIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    add_liquidity_by_strategy_invoke_signed_with_program_id(
        crate::ID,
        accounts,
        args,
        seeds,
    )
}
pub fn add_liquidity_by_strategy_verify_account_keys(
    accounts: AddLiquidityByStrategyAccounts<'_, '_>,
    keys: AddLiquidityByStrategyKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.position.key, keys.position),
        (*accounts.lb_pair.key, keys.lb_pair),
        (*accounts.bin_array_bitmap_extension.key, keys.bin_array_bitmap_extension),
        (*accounts.user_token_x.key, keys.user_token_x),
        (*accounts.user_token_y.key, keys.user_token_y),
        (*accounts.reserve_x.key, keys.reserve_x),
        (*accounts.reserve_y.key, keys.reserve_y),
        (*accounts.token_x_mint.key, keys.token_x_mint),
        (*accounts.token_y_mint.key, keys.token_y_mint),
        (*accounts.bin_array_lower.key, keys.bin_array_lower),
        (*accounts.bin_array_upper.key, keys.bin_array_upper),
        (*accounts.sender.key, keys.sender),
        (*accounts.token_x_program.key, keys.token_x_program),
        (*accounts.token_y_program.key, keys.token_y_program),
        (*accounts.event_authority.key, keys.event_authority),
        (*accounts.program.key, keys.program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn add_liquidity_by_strategy_verify_writable_privileges<'me, 'info>(
    accounts: AddLiquidityByStrategyAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.position,
        accounts.lb_pair,
        accounts.bin_array_bitmap_extension,
        accounts.user_token_x,
        accounts.user_token_y,
        accounts.reserve_x,
        accounts.reserve_y,
        accounts.bin_array_lower,
        accounts.bin_array_upper,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn add_liquidity_by_strategy_verify_signer_privileges<'me, 'info>(
    accounts: AddLiquidityByStrategyAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.sender] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn add_liquidity_by_strategy_verify_account_privileges<'me, 'info>(
    accounts: AddLiquidityByStrategyAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    add_liquidity_by_strategy_verify_writable_privileges(accounts)?;
    add_liquidity_by_strategy_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const ADD_LIQUIDITY_BY_STRATEGY_ONE_SIDE_IX_ACCOUNTS_LEN: usize = 12;
#[derive(Copy, Clone, Debug)]
pub struct AddLiquidityByStrategyOneSideAccounts<'me, 'info> {
    pub position: &'me AccountInfo<'info>,
    pub lb_pair: &'me AccountInfo<'info>,
    pub bin_array_bitmap_extension: &'me AccountInfo<'info>,
    pub user_token: &'me AccountInfo<'info>,
    pub reserve: &'me AccountInfo<'info>,
    pub token_mint: &'me AccountInfo<'info>,
    pub bin_array_lower: &'me AccountInfo<'info>,
    pub bin_array_upper: &'me AccountInfo<'info>,
    pub sender: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct AddLiquidityByStrategyOneSideKeys {
    pub position: Pubkey,
    pub lb_pair: Pubkey,
    pub bin_array_bitmap_extension: Pubkey,
    pub user_token: Pubkey,
    pub reserve: Pubkey,
    pub token_mint: Pubkey,
    pub bin_array_lower: Pubkey,
    pub bin_array_upper: Pubkey,
    pub sender: Pubkey,
    pub token_program: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<AddLiquidityByStrategyOneSideAccounts<'_, '_>>
for AddLiquidityByStrategyOneSideKeys {
    fn from(accounts: AddLiquidityByStrategyOneSideAccounts) -> Self {
        Self {
            position: *accounts.position.key,
            lb_pair: *accounts.lb_pair.key,
            bin_array_bitmap_extension: *accounts.bin_array_bitmap_extension.key,
            user_token: *accounts.user_token.key,
            reserve: *accounts.reserve.key,
            token_mint: *accounts.token_mint.key,
            bin_array_lower: *accounts.bin_array_lower.key,
            bin_array_upper: *accounts.bin_array_upper.key,
            sender: *accounts.sender.key,
            token_program: *accounts.token_program.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<AddLiquidityByStrategyOneSideKeys>
for [AccountMeta; ADD_LIQUIDITY_BY_STRATEGY_ONE_SIDE_IX_ACCOUNTS_LEN] {
    fn from(keys: AddLiquidityByStrategyOneSideKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.position,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.lb_pair,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.bin_array_bitmap_extension,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_token,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.reserve,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.bin_array_lower,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.bin_array_upper,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.sender,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.event_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; ADD_LIQUIDITY_BY_STRATEGY_ONE_SIDE_IX_ACCOUNTS_LEN]>
for AddLiquidityByStrategyOneSideKeys {
    fn from(
        pubkeys: [Pubkey; ADD_LIQUIDITY_BY_STRATEGY_ONE_SIDE_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            position: pubkeys[0],
            lb_pair: pubkeys[1],
            bin_array_bitmap_extension: pubkeys[2],
            user_token: pubkeys[3],
            reserve: pubkeys[4],
            token_mint: pubkeys[5],
            bin_array_lower: pubkeys[6],
            bin_array_upper: pubkeys[7],
            sender: pubkeys[8],
            token_program: pubkeys[9],
            event_authority: pubkeys[10],
            program: pubkeys[11],
        }
    }
}
impl<'info> From<AddLiquidityByStrategyOneSideAccounts<'_, 'info>>
for [AccountInfo<'info>; ADD_LIQUIDITY_BY_STRATEGY_ONE_SIDE_IX_ACCOUNTS_LEN] {
    fn from(accounts: AddLiquidityByStrategyOneSideAccounts<'_, 'info>) -> Self {
        [
            accounts.position.clone(),
            accounts.lb_pair.clone(),
            accounts.bin_array_bitmap_extension.clone(),
            accounts.user_token.clone(),
            accounts.reserve.clone(),
            accounts.token_mint.clone(),
            accounts.bin_array_lower.clone(),
            accounts.bin_array_upper.clone(),
            accounts.sender.clone(),
            accounts.token_program.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; ADD_LIQUIDITY_BY_STRATEGY_ONE_SIDE_IX_ACCOUNTS_LEN]>
for AddLiquidityByStrategyOneSideAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<
            'info,
        >; ADD_LIQUIDITY_BY_STRATEGY_ONE_SIDE_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            position: &arr[0],
            lb_pair: &arr[1],
            bin_array_bitmap_extension: &arr[2],
            user_token: &arr[3],
            reserve: &arr[4],
            token_mint: &arr[5],
            bin_array_lower: &arr[6],
            bin_array_upper: &arr[7],
            sender: &arr[8],
            token_program: &arr[9],
            event_authority: &arr[10],
            program: &arr[11],
        }
    }
}
pub const ADD_LIQUIDITY_BY_STRATEGY_ONE_SIDE_IX_DISCM: [u8; 8] = [
    41,
    5,
    238,
    175,
    100,
    225,
    6,
    205,
];
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct AddLiquidityByStrategyOneSideIxArgs {
    pub liquidity_parameter: LiquidityParameterByStrategyOneSide,
}
#[derive(Clone, Debug, PartialEq)]
pub struct AddLiquidityByStrategyOneSideIxData(pub AddLiquidityByStrategyOneSideIxArgs);
impl From<AddLiquidityByStrategyOneSideIxArgs> for AddLiquidityByStrategyOneSideIxData {
    fn from(args: AddLiquidityByStrategyOneSideIxArgs) -> Self {
        Self(args)
    }
}
impl AddLiquidityByStrategyOneSideIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != ADD_LIQUIDITY_BY_STRATEGY_ONE_SIDE_IX_DISCM {
            return Err(
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!(
                        "discm does not match. Expected: {:?}. Received: {:?}",
                        ADD_LIQUIDITY_BY_STRATEGY_ONE_SIDE_IX_DISCM, maybe_discm
                    ),
                ),
            );
        }
        Ok(Self(AddLiquidityByStrategyOneSideIxArgs::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&ADD_LIQUIDITY_BY_STRATEGY_ONE_SIDE_IX_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn add_liquidity_by_strategy_one_side_ix_with_program_id(
    program_id: Pubkey,
    keys: AddLiquidityByStrategyOneSideKeys,
    args: AddLiquidityByStrategyOneSideIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; ADD_LIQUIDITY_BY_STRATEGY_ONE_SIDE_IX_ACCOUNTS_LEN] = keys
        .into();
    let data: AddLiquidityByStrategyOneSideIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn add_liquidity_by_strategy_one_side_ix(
    keys: AddLiquidityByStrategyOneSideKeys,
    args: AddLiquidityByStrategyOneSideIxArgs,
) -> std::io::Result<Instruction> {
    add_liquidity_by_strategy_one_side_ix_with_program_id(crate::ID, keys, args)
}
pub fn add_liquidity_by_strategy_one_side_invoke_with_program_id(
    program_id: Pubkey,
    accounts: AddLiquidityByStrategyOneSideAccounts<'_, '_>,
    args: AddLiquidityByStrategyOneSideIxArgs,
) -> ProgramResult {
    let keys: AddLiquidityByStrategyOneSideKeys = accounts.into();
    let ix = add_liquidity_by_strategy_one_side_ix_with_program_id(
        program_id,
        keys,
        args,
    )?;
    invoke_instruction(&ix, accounts)
}
pub fn add_liquidity_by_strategy_one_side_invoke(
    accounts: AddLiquidityByStrategyOneSideAccounts<'_, '_>,
    args: AddLiquidityByStrategyOneSideIxArgs,
) -> ProgramResult {
    add_liquidity_by_strategy_one_side_invoke_with_program_id(crate::ID, accounts, args)
}
pub fn add_liquidity_by_strategy_one_side_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: AddLiquidityByStrategyOneSideAccounts<'_, '_>,
    args: AddLiquidityByStrategyOneSideIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: AddLiquidityByStrategyOneSideKeys = accounts.into();
    let ix = add_liquidity_by_strategy_one_side_ix_with_program_id(
        program_id,
        keys,
        args,
    )?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn add_liquidity_by_strategy_one_side_invoke_signed(
    accounts: AddLiquidityByStrategyOneSideAccounts<'_, '_>,
    args: AddLiquidityByStrategyOneSideIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    add_liquidity_by_strategy_one_side_invoke_signed_with_program_id(
        crate::ID,
        accounts,
        args,
        seeds,
    )
}
pub fn add_liquidity_by_strategy_one_side_verify_account_keys(
    accounts: AddLiquidityByStrategyOneSideAccounts<'_, '_>,
    keys: AddLiquidityByStrategyOneSideKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.position.key, keys.position),
        (*accounts.lb_pair.key, keys.lb_pair),
        (*accounts.bin_array_bitmap_extension.key, keys.bin_array_bitmap_extension),
        (*accounts.user_token.key, keys.user_token),
        (*accounts.reserve.key, keys.reserve),
        (*accounts.token_mint.key, keys.token_mint),
        (*accounts.bin_array_lower.key, keys.bin_array_lower),
        (*accounts.bin_array_upper.key, keys.bin_array_upper),
        (*accounts.sender.key, keys.sender),
        (*accounts.token_program.key, keys.token_program),
        (*accounts.event_authority.key, keys.event_authority),
        (*accounts.program.key, keys.program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn add_liquidity_by_strategy_one_side_verify_writable_privileges<'me, 'info>(
    accounts: AddLiquidityByStrategyOneSideAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.position,
        accounts.lb_pair,
        accounts.bin_array_bitmap_extension,
        accounts.user_token,
        accounts.reserve,
        accounts.bin_array_lower,
        accounts.bin_array_upper,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn add_liquidity_by_strategy_one_side_verify_signer_privileges<'me, 'info>(
    accounts: AddLiquidityByStrategyOneSideAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.sender] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn add_liquidity_by_strategy_one_side_verify_account_privileges<'me, 'info>(
    accounts: AddLiquidityByStrategyOneSideAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    add_liquidity_by_strategy_one_side_verify_writable_privileges(accounts)?;
    add_liquidity_by_strategy_one_side_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const ADD_LIQUIDITY_ONE_SIDE_IX_ACCOUNTS_LEN: usize = 12;
#[derive(Copy, Clone, Debug)]
pub struct AddLiquidityOneSideAccounts<'me, 'info> {
    pub position: &'me AccountInfo<'info>,
    pub lb_pair: &'me AccountInfo<'info>,
    pub bin_array_bitmap_extension: &'me AccountInfo<'info>,
    pub user_token: &'me AccountInfo<'info>,
    pub reserve: &'me AccountInfo<'info>,
    pub token_mint: &'me AccountInfo<'info>,
    pub bin_array_lower: &'me AccountInfo<'info>,
    pub bin_array_upper: &'me AccountInfo<'info>,
    pub sender: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct AddLiquidityOneSideKeys {
    pub position: Pubkey,
    pub lb_pair: Pubkey,
    pub bin_array_bitmap_extension: Pubkey,
    pub user_token: Pubkey,
    pub reserve: Pubkey,
    pub token_mint: Pubkey,
    pub bin_array_lower: Pubkey,
    pub bin_array_upper: Pubkey,
    pub sender: Pubkey,
    pub token_program: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<AddLiquidityOneSideAccounts<'_, '_>> for AddLiquidityOneSideKeys {
    fn from(accounts: AddLiquidityOneSideAccounts) -> Self {
        Self {
            position: *accounts.position.key,
            lb_pair: *accounts.lb_pair.key,
            bin_array_bitmap_extension: *accounts.bin_array_bitmap_extension.key,
            user_token: *accounts.user_token.key,
            reserve: *accounts.reserve.key,
            token_mint: *accounts.token_mint.key,
            bin_array_lower: *accounts.bin_array_lower.key,
            bin_array_upper: *accounts.bin_array_upper.key,
            sender: *accounts.sender.key,
            token_program: *accounts.token_program.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<AddLiquidityOneSideKeys>
for [AccountMeta; ADD_LIQUIDITY_ONE_SIDE_IX_ACCOUNTS_LEN] {
    fn from(keys: AddLiquidityOneSideKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.position,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.lb_pair,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.bin_array_bitmap_extension,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_token,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.reserve,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.bin_array_lower,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.bin_array_upper,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.sender,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.event_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; ADD_LIQUIDITY_ONE_SIDE_IX_ACCOUNTS_LEN]> for AddLiquidityOneSideKeys {
    fn from(pubkeys: [Pubkey; ADD_LIQUIDITY_ONE_SIDE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            position: pubkeys[0],
            lb_pair: pubkeys[1],
            bin_array_bitmap_extension: pubkeys[2],
            user_token: pubkeys[3],
            reserve: pubkeys[4],
            token_mint: pubkeys[5],
            bin_array_lower: pubkeys[6],
            bin_array_upper: pubkeys[7],
            sender: pubkeys[8],
            token_program: pubkeys[9],
            event_authority: pubkeys[10],
            program: pubkeys[11],
        }
    }
}
impl<'info> From<AddLiquidityOneSideAccounts<'_, 'info>>
for [AccountInfo<'info>; ADD_LIQUIDITY_ONE_SIDE_IX_ACCOUNTS_LEN] {
    fn from(accounts: AddLiquidityOneSideAccounts<'_, 'info>) -> Self {
        [
            accounts.position.clone(),
            accounts.lb_pair.clone(),
            accounts.bin_array_bitmap_extension.clone(),
            accounts.user_token.clone(),
            accounts.reserve.clone(),
            accounts.token_mint.clone(),
            accounts.bin_array_lower.clone(),
            accounts.bin_array_upper.clone(),
            accounts.sender.clone(),
            accounts.token_program.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; ADD_LIQUIDITY_ONE_SIDE_IX_ACCOUNTS_LEN]>
for AddLiquidityOneSideAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; ADD_LIQUIDITY_ONE_SIDE_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            position: &arr[0],
            lb_pair: &arr[1],
            bin_array_bitmap_extension: &arr[2],
            user_token: &arr[3],
            reserve: &arr[4],
            token_mint: &arr[5],
            bin_array_lower: &arr[6],
            bin_array_upper: &arr[7],
            sender: &arr[8],
            token_program: &arr[9],
            event_authority: &arr[10],
            program: &arr[11],
        }
    }
}
pub const ADD_LIQUIDITY_ONE_SIDE_IX_DISCM: [u8; 8] = [
    94,
    155,
    103,
    151,
    70,
    95,
    220,
    165,
];
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct AddLiquidityOneSideIxArgs {
    pub liquidity_parameter: LiquidityOneSideParameter,
}
#[derive(Clone, Debug, PartialEq)]
pub struct AddLiquidityOneSideIxData(pub AddLiquidityOneSideIxArgs);
impl From<AddLiquidityOneSideIxArgs> for AddLiquidityOneSideIxData {
    fn from(args: AddLiquidityOneSideIxArgs) -> Self {
        Self(args)
    }
}
impl AddLiquidityOneSideIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != ADD_LIQUIDITY_ONE_SIDE_IX_DISCM {
            return Err(
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!(
                        "discm does not match. Expected: {:?}. Received: {:?}",
                        ADD_LIQUIDITY_ONE_SIDE_IX_DISCM, maybe_discm
                    ),
                ),
            );
        }
        Ok(Self(AddLiquidityOneSideIxArgs::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&ADD_LIQUIDITY_ONE_SIDE_IX_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn add_liquidity_one_side_ix_with_program_id(
    program_id: Pubkey,
    keys: AddLiquidityOneSideKeys,
    args: AddLiquidityOneSideIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; ADD_LIQUIDITY_ONE_SIDE_IX_ACCOUNTS_LEN] = keys.into();
    let data: AddLiquidityOneSideIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn add_liquidity_one_side_ix(
    keys: AddLiquidityOneSideKeys,
    args: AddLiquidityOneSideIxArgs,
) -> std::io::Result<Instruction> {
    add_liquidity_one_side_ix_with_program_id(crate::ID, keys, args)
}
pub fn add_liquidity_one_side_invoke_with_program_id(
    program_id: Pubkey,
    accounts: AddLiquidityOneSideAccounts<'_, '_>,
    args: AddLiquidityOneSideIxArgs,
) -> ProgramResult {
    let keys: AddLiquidityOneSideKeys = accounts.into();
    let ix = add_liquidity_one_side_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn add_liquidity_one_side_invoke(
    accounts: AddLiquidityOneSideAccounts<'_, '_>,
    args: AddLiquidityOneSideIxArgs,
) -> ProgramResult {
    add_liquidity_one_side_invoke_with_program_id(crate::ID, accounts, args)
}
pub fn add_liquidity_one_side_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: AddLiquidityOneSideAccounts<'_, '_>,
    args: AddLiquidityOneSideIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: AddLiquidityOneSideKeys = accounts.into();
    let ix = add_liquidity_one_side_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn add_liquidity_one_side_invoke_signed(
    accounts: AddLiquidityOneSideAccounts<'_, '_>,
    args: AddLiquidityOneSideIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    add_liquidity_one_side_invoke_signed_with_program_id(
        crate::ID,
        accounts,
        args,
        seeds,
    )
}
pub fn add_liquidity_one_side_verify_account_keys(
    accounts: AddLiquidityOneSideAccounts<'_, '_>,
    keys: AddLiquidityOneSideKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.position.key, keys.position),
        (*accounts.lb_pair.key, keys.lb_pair),
        (*accounts.bin_array_bitmap_extension.key, keys.bin_array_bitmap_extension),
        (*accounts.user_token.key, keys.user_token),
        (*accounts.reserve.key, keys.reserve),
        (*accounts.token_mint.key, keys.token_mint),
        (*accounts.bin_array_lower.key, keys.bin_array_lower),
        (*accounts.bin_array_upper.key, keys.bin_array_upper),
        (*accounts.sender.key, keys.sender),
        (*accounts.token_program.key, keys.token_program),
        (*accounts.event_authority.key, keys.event_authority),
        (*accounts.program.key, keys.program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn add_liquidity_one_side_verify_writable_privileges<'me, 'info>(
    accounts: AddLiquidityOneSideAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.position,
        accounts.lb_pair,
        accounts.bin_array_bitmap_extension,
        accounts.user_token,
        accounts.reserve,
        accounts.bin_array_lower,
        accounts.bin_array_upper,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn add_liquidity_one_side_verify_signer_privileges<'me, 'info>(
    accounts: AddLiquidityOneSideAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.sender] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn add_liquidity_one_side_verify_account_privileges<'me, 'info>(
    accounts: AddLiquidityOneSideAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    add_liquidity_one_side_verify_writable_privileges(accounts)?;
    add_liquidity_one_side_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const REMOVE_LIQUIDITY_IX_ACCOUNTS_LEN: usize = 16;
#[derive(Copy, Clone, Debug)]
pub struct RemoveLiquidityAccounts<'me, 'info> {
    pub position: &'me AccountInfo<'info>,
    pub lb_pair: &'me AccountInfo<'info>,
    pub bin_array_bitmap_extension: &'me AccountInfo<'info>,
    pub user_token_x: &'me AccountInfo<'info>,
    pub user_token_y: &'me AccountInfo<'info>,
    pub reserve_x: &'me AccountInfo<'info>,
    pub reserve_y: &'me AccountInfo<'info>,
    pub token_x_mint: &'me AccountInfo<'info>,
    pub token_y_mint: &'me AccountInfo<'info>,
    pub bin_array_lower: &'me AccountInfo<'info>,
    pub bin_array_upper: &'me AccountInfo<'info>,
    pub sender: &'me AccountInfo<'info>,
    pub token_x_program: &'me AccountInfo<'info>,
    pub token_y_program: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct RemoveLiquidityKeys {
    pub position: Pubkey,
    pub lb_pair: Pubkey,
    pub bin_array_bitmap_extension: Pubkey,
    pub user_token_x: Pubkey,
    pub user_token_y: Pubkey,
    pub reserve_x: Pubkey,
    pub reserve_y: Pubkey,
    pub token_x_mint: Pubkey,
    pub token_y_mint: Pubkey,
    pub bin_array_lower: Pubkey,
    pub bin_array_upper: Pubkey,
    pub sender: Pubkey,
    pub token_x_program: Pubkey,
    pub token_y_program: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<RemoveLiquidityAccounts<'_, '_>> for RemoveLiquidityKeys {
    fn from(accounts: RemoveLiquidityAccounts) -> Self {
        Self {
            position: *accounts.position.key,
            lb_pair: *accounts.lb_pair.key,
            bin_array_bitmap_extension: *accounts.bin_array_bitmap_extension.key,
            user_token_x: *accounts.user_token_x.key,
            user_token_y: *accounts.user_token_y.key,
            reserve_x: *accounts.reserve_x.key,
            reserve_y: *accounts.reserve_y.key,
            token_x_mint: *accounts.token_x_mint.key,
            token_y_mint: *accounts.token_y_mint.key,
            bin_array_lower: *accounts.bin_array_lower.key,
            bin_array_upper: *accounts.bin_array_upper.key,
            sender: *accounts.sender.key,
            token_x_program: *accounts.token_x_program.key,
            token_y_program: *accounts.token_y_program.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<RemoveLiquidityKeys> for [AccountMeta; REMOVE_LIQUIDITY_IX_ACCOUNTS_LEN] {
    fn from(keys: RemoveLiquidityKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.position,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.lb_pair,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.bin_array_bitmap_extension,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_token_x,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_token_y,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.reserve_x,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.reserve_y,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_x_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_y_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.bin_array_lower,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.bin_array_upper,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.sender,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_x_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_y_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.event_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; REMOVE_LIQUIDITY_IX_ACCOUNTS_LEN]> for RemoveLiquidityKeys {
    fn from(pubkeys: [Pubkey; REMOVE_LIQUIDITY_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            position: pubkeys[0],
            lb_pair: pubkeys[1],
            bin_array_bitmap_extension: pubkeys[2],
            user_token_x: pubkeys[3],
            user_token_y: pubkeys[4],
            reserve_x: pubkeys[5],
            reserve_y: pubkeys[6],
            token_x_mint: pubkeys[7],
            token_y_mint: pubkeys[8],
            bin_array_lower: pubkeys[9],
            bin_array_upper: pubkeys[10],
            sender: pubkeys[11],
            token_x_program: pubkeys[12],
            token_y_program: pubkeys[13],
            event_authority: pubkeys[14],
            program: pubkeys[15],
        }
    }
}
impl<'info> From<RemoveLiquidityAccounts<'_, 'info>>
for [AccountInfo<'info>; REMOVE_LIQUIDITY_IX_ACCOUNTS_LEN] {
    fn from(accounts: RemoveLiquidityAccounts<'_, 'info>) -> Self {
        [
            accounts.position.clone(),
            accounts.lb_pair.clone(),
            accounts.bin_array_bitmap_extension.clone(),
            accounts.user_token_x.clone(),
            accounts.user_token_y.clone(),
            accounts.reserve_x.clone(),
            accounts.reserve_y.clone(),
            accounts.token_x_mint.clone(),
            accounts.token_y_mint.clone(),
            accounts.bin_array_lower.clone(),
            accounts.bin_array_upper.clone(),
            accounts.sender.clone(),
            accounts.token_x_program.clone(),
            accounts.token_y_program.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; REMOVE_LIQUIDITY_IX_ACCOUNTS_LEN]>
for RemoveLiquidityAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; REMOVE_LIQUIDITY_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            position: &arr[0],
            lb_pair: &arr[1],
            bin_array_bitmap_extension: &arr[2],
            user_token_x: &arr[3],
            user_token_y: &arr[4],
            reserve_x: &arr[5],
            reserve_y: &arr[6],
            token_x_mint: &arr[7],
            token_y_mint: &arr[8],
            bin_array_lower: &arr[9],
            bin_array_upper: &arr[10],
            sender: &arr[11],
            token_x_program: &arr[12],
            token_y_program: &arr[13],
            event_authority: &arr[14],
            program: &arr[15],
        }
    }
}
pub const REMOVE_LIQUIDITY_IX_DISCM: [u8; 8] = [80, 85, 209, 72, 24, 206, 177, 108];
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RemoveLiquidityIxArgs {
    pub bin_liquidity_removal: Vec<BinLiquidityReduction>,
}
#[derive(Clone, Debug, PartialEq)]
pub struct RemoveLiquidityIxData(pub RemoveLiquidityIxArgs);
impl From<RemoveLiquidityIxArgs> for RemoveLiquidityIxData {
    fn from(args: RemoveLiquidityIxArgs) -> Self {
        Self(args)
    }
}
impl RemoveLiquidityIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != REMOVE_LIQUIDITY_IX_DISCM {
            return Err(
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!(
                        "discm does not match. Expected: {:?}. Received: {:?}",
                        REMOVE_LIQUIDITY_IX_DISCM, maybe_discm
                    ),
                ),
            );
        }
        Ok(Self(RemoveLiquidityIxArgs::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&REMOVE_LIQUIDITY_IX_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn remove_liquidity_ix_with_program_id(
    program_id: Pubkey,
    keys: RemoveLiquidityKeys,
    args: RemoveLiquidityIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; REMOVE_LIQUIDITY_IX_ACCOUNTS_LEN] = keys.into();
    let data: RemoveLiquidityIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn remove_liquidity_ix(
    keys: RemoveLiquidityKeys,
    args: RemoveLiquidityIxArgs,
) -> std::io::Result<Instruction> {
    remove_liquidity_ix_with_program_id(crate::ID, keys, args)
}
pub fn remove_liquidity_invoke_with_program_id(
    program_id: Pubkey,
    accounts: RemoveLiquidityAccounts<'_, '_>,
    args: RemoveLiquidityIxArgs,
) -> ProgramResult {
    let keys: RemoveLiquidityKeys = accounts.into();
    let ix = remove_liquidity_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn remove_liquidity_invoke(
    accounts: RemoveLiquidityAccounts<'_, '_>,
    args: RemoveLiquidityIxArgs,
) -> ProgramResult {
    remove_liquidity_invoke_with_program_id(crate::ID, accounts, args)
}
pub fn remove_liquidity_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: RemoveLiquidityAccounts<'_, '_>,
    args: RemoveLiquidityIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: RemoveLiquidityKeys = accounts.into();
    let ix = remove_liquidity_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn remove_liquidity_invoke_signed(
    accounts: RemoveLiquidityAccounts<'_, '_>,
    args: RemoveLiquidityIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    remove_liquidity_invoke_signed_with_program_id(crate::ID, accounts, args, seeds)
}
pub fn remove_liquidity_verify_account_keys(
    accounts: RemoveLiquidityAccounts<'_, '_>,
    keys: RemoveLiquidityKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.position.key, keys.position),
        (*accounts.lb_pair.key, keys.lb_pair),
        (*accounts.bin_array_bitmap_extension.key, keys.bin_array_bitmap_extension),
        (*accounts.user_token_x.key, keys.user_token_x),
        (*accounts.user_token_y.key, keys.user_token_y),
        (*accounts.reserve_x.key, keys.reserve_x),
        (*accounts.reserve_y.key, keys.reserve_y),
        (*accounts.token_x_mint.key, keys.token_x_mint),
        (*accounts.token_y_mint.key, keys.token_y_mint),
        (*accounts.bin_array_lower.key, keys.bin_array_lower),
        (*accounts.bin_array_upper.key, keys.bin_array_upper),
        (*accounts.sender.key, keys.sender),
        (*accounts.token_x_program.key, keys.token_x_program),
        (*accounts.token_y_program.key, keys.token_y_program),
        (*accounts.event_authority.key, keys.event_authority),
        (*accounts.program.key, keys.program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn remove_liquidity_verify_writable_privileges<'me, 'info>(
    accounts: RemoveLiquidityAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.position,
        accounts.lb_pair,
        accounts.bin_array_bitmap_extension,
        accounts.user_token_x,
        accounts.user_token_y,
        accounts.reserve_x,
        accounts.reserve_y,
        accounts.bin_array_lower,
        accounts.bin_array_upper,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn remove_liquidity_verify_signer_privileges<'me, 'info>(
    accounts: RemoveLiquidityAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.sender] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn remove_liquidity_verify_account_privileges<'me, 'info>(
    accounts: RemoveLiquidityAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    remove_liquidity_verify_writable_privileges(accounts)?;
    remove_liquidity_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const INITIALIZE_POSITION_IX_ACCOUNTS_LEN: usize = 8;
#[derive(Copy, Clone, Debug)]
pub struct InitializePositionAccounts<'me, 'info> {
    pub payer: &'me AccountInfo<'info>,
    pub position: &'me AccountInfo<'info>,
    pub lb_pair: &'me AccountInfo<'info>,
    pub owner: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub rent: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct InitializePositionKeys {
    pub payer: Pubkey,
    pub position: Pubkey,
    pub lb_pair: Pubkey,
    pub owner: Pubkey,
    pub system_program: Pubkey,
    pub rent: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<InitializePositionAccounts<'_, '_>> for InitializePositionKeys {
    fn from(accounts: InitializePositionAccounts) -> Self {
        Self {
            payer: *accounts.payer.key,
            position: *accounts.position.key,
            lb_pair: *accounts.lb_pair.key,
            owner: *accounts.owner.key,
            system_program: *accounts.system_program.key,
            rent: *accounts.rent.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<InitializePositionKeys>
for [AccountMeta; INITIALIZE_POSITION_IX_ACCOUNTS_LEN] {
    fn from(keys: InitializePositionKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.payer,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.position,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.lb_pair,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.owner,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.system_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.rent,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.event_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; INITIALIZE_POSITION_IX_ACCOUNTS_LEN]> for InitializePositionKeys {
    fn from(pubkeys: [Pubkey; INITIALIZE_POSITION_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            payer: pubkeys[0],
            position: pubkeys[1],
            lb_pair: pubkeys[2],
            owner: pubkeys[3],
            system_program: pubkeys[4],
            rent: pubkeys[5],
            event_authority: pubkeys[6],
            program: pubkeys[7],
        }
    }
}
impl<'info> From<InitializePositionAccounts<'_, 'info>>
for [AccountInfo<'info>; INITIALIZE_POSITION_IX_ACCOUNTS_LEN] {
    fn from(accounts: InitializePositionAccounts<'_, 'info>) -> Self {
        [
            accounts.payer.clone(),
            accounts.position.clone(),
            accounts.lb_pair.clone(),
            accounts.owner.clone(),
            accounts.system_program.clone(),
            accounts.rent.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; INITIALIZE_POSITION_IX_ACCOUNTS_LEN]>
for InitializePositionAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; INITIALIZE_POSITION_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            payer: &arr[0],
            position: &arr[1],
            lb_pair: &arr[2],
            owner: &arr[3],
            system_program: &arr[4],
            rent: &arr[5],
            event_authority: &arr[6],
            program: &arr[7],
        }
    }
}
pub const INITIALIZE_POSITION_IX_DISCM: [u8; 8] = [219, 192, 234, 71, 190, 191, 102, 80];
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InitializePositionIxArgs {
    pub lower_bin_id: i32,
    pub width: i32,
}
#[derive(Clone, Debug, PartialEq)]
pub struct InitializePositionIxData(pub InitializePositionIxArgs);
impl From<InitializePositionIxArgs> for InitializePositionIxData {
    fn from(args: InitializePositionIxArgs) -> Self {
        Self(args)
    }
}
impl InitializePositionIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != INITIALIZE_POSITION_IX_DISCM {
            return Err(
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!(
                        "discm does not match. Expected: {:?}. Received: {:?}",
                        INITIALIZE_POSITION_IX_DISCM, maybe_discm
                    ),
                ),
            );
        }
        Ok(Self(InitializePositionIxArgs::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&INITIALIZE_POSITION_IX_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn initialize_position_ix_with_program_id(
    program_id: Pubkey,
    keys: InitializePositionKeys,
    args: InitializePositionIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; INITIALIZE_POSITION_IX_ACCOUNTS_LEN] = keys.into();
    let data: InitializePositionIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn initialize_position_ix(
    keys: InitializePositionKeys,
    args: InitializePositionIxArgs,
) -> std::io::Result<Instruction> {
    initialize_position_ix_with_program_id(crate::ID, keys, args)
}
pub fn initialize_position_invoke_with_program_id(
    program_id: Pubkey,
    accounts: InitializePositionAccounts<'_, '_>,
    args: InitializePositionIxArgs,
) -> ProgramResult {
    let keys: InitializePositionKeys = accounts.into();
    let ix = initialize_position_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn initialize_position_invoke(
    accounts: InitializePositionAccounts<'_, '_>,
    args: InitializePositionIxArgs,
) -> ProgramResult {
    initialize_position_invoke_with_program_id(crate::ID, accounts, args)
}
pub fn initialize_position_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: InitializePositionAccounts<'_, '_>,
    args: InitializePositionIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: InitializePositionKeys = accounts.into();
    let ix = initialize_position_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn initialize_position_invoke_signed(
    accounts: InitializePositionAccounts<'_, '_>,
    args: InitializePositionIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    initialize_position_invoke_signed_with_program_id(crate::ID, accounts, args, seeds)
}
pub fn initialize_position_verify_account_keys(
    accounts: InitializePositionAccounts<'_, '_>,
    keys: InitializePositionKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.payer.key, keys.payer),
        (*accounts.position.key, keys.position),
        (*accounts.lb_pair.key, keys.lb_pair),
        (*accounts.owner.key, keys.owner),
        (*accounts.system_program.key, keys.system_program),
        (*accounts.rent.key, keys.rent),
        (*accounts.event_authority.key, keys.event_authority),
        (*accounts.program.key, keys.program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn initialize_position_verify_writable_privileges<'me, 'info>(
    accounts: InitializePositionAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.payer, accounts.position] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn initialize_position_verify_signer_privileges<'me, 'info>(
    accounts: InitializePositionAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.payer, accounts.position, accounts.owner] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn initialize_position_verify_account_privileges<'me, 'info>(
    accounts: InitializePositionAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    initialize_position_verify_writable_privileges(accounts)?;
    initialize_position_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const INITIALIZE_POSITION_PDA_IX_ACCOUNTS_LEN: usize = 9;
#[derive(Copy, Clone, Debug)]
pub struct InitializePositionPdaAccounts<'me, 'info> {
    pub payer: &'me AccountInfo<'info>,
    pub base: &'me AccountInfo<'info>,
    pub position: &'me AccountInfo<'info>,
    pub lb_pair: &'me AccountInfo<'info>,
    pub owner: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub rent: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct InitializePositionPdaKeys {
    pub payer: Pubkey,
    pub base: Pubkey,
    pub position: Pubkey,
    pub lb_pair: Pubkey,
    pub owner: Pubkey,
    pub system_program: Pubkey,
    pub rent: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<InitializePositionPdaAccounts<'_, '_>> for InitializePositionPdaKeys {
    fn from(accounts: InitializePositionPdaAccounts) -> Self {
        Self {
            payer: *accounts.payer.key,
            base: *accounts.base.key,
            position: *accounts.position.key,
            lb_pair: *accounts.lb_pair.key,
            owner: *accounts.owner.key,
            system_program: *accounts.system_program.key,
            rent: *accounts.rent.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<InitializePositionPdaKeys>
for [AccountMeta; INITIALIZE_POSITION_PDA_IX_ACCOUNTS_LEN] {
    fn from(keys: InitializePositionPdaKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.payer,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.base,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.position,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.lb_pair,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.owner,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.system_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.rent,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.event_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; INITIALIZE_POSITION_PDA_IX_ACCOUNTS_LEN]>
for InitializePositionPdaKeys {
    fn from(pubkeys: [Pubkey; INITIALIZE_POSITION_PDA_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            payer: pubkeys[0],
            base: pubkeys[1],
            position: pubkeys[2],
            lb_pair: pubkeys[3],
            owner: pubkeys[4],
            system_program: pubkeys[5],
            rent: pubkeys[6],
            event_authority: pubkeys[7],
            program: pubkeys[8],
        }
    }
}
impl<'info> From<InitializePositionPdaAccounts<'_, 'info>>
for [AccountInfo<'info>; INITIALIZE_POSITION_PDA_IX_ACCOUNTS_LEN] {
    fn from(accounts: InitializePositionPdaAccounts<'_, 'info>) -> Self {
        [
            accounts.payer.clone(),
            accounts.base.clone(),
            accounts.position.clone(),
            accounts.lb_pair.clone(),
            accounts.owner.clone(),
            accounts.system_program.clone(),
            accounts.rent.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; INITIALIZE_POSITION_PDA_IX_ACCOUNTS_LEN]>
for InitializePositionPdaAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; INITIALIZE_POSITION_PDA_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            payer: &arr[0],
            base: &arr[1],
            position: &arr[2],
            lb_pair: &arr[3],
            owner: &arr[4],
            system_program: &arr[5],
            rent: &arr[6],
            event_authority: &arr[7],
            program: &arr[8],
        }
    }
}
pub const INITIALIZE_POSITION_PDA_IX_DISCM: [u8; 8] = [
    46,
    82,
    125,
    146,
    85,
    141,
    228,
    153,
];
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InitializePositionPdaIxArgs {
    pub lower_bin_id: i32,
    pub width: i32,
}
#[derive(Clone, Debug, PartialEq)]
pub struct InitializePositionPdaIxData(pub InitializePositionPdaIxArgs);
impl From<InitializePositionPdaIxArgs> for InitializePositionPdaIxData {
    fn from(args: InitializePositionPdaIxArgs) -> Self {
        Self(args)
    }
}
impl InitializePositionPdaIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != INITIALIZE_POSITION_PDA_IX_DISCM {
            return Err(
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!(
                        "discm does not match. Expected: {:?}. Received: {:?}",
                        INITIALIZE_POSITION_PDA_IX_DISCM, maybe_discm
                    ),
                ),
            );
        }
        Ok(Self(InitializePositionPdaIxArgs::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&INITIALIZE_POSITION_PDA_IX_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn initialize_position_pda_ix_with_program_id(
    program_id: Pubkey,
    keys: InitializePositionPdaKeys,
    args: InitializePositionPdaIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; INITIALIZE_POSITION_PDA_IX_ACCOUNTS_LEN] = keys.into();
    let data: InitializePositionPdaIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn initialize_position_pda_ix(
    keys: InitializePositionPdaKeys,
    args: InitializePositionPdaIxArgs,
) -> std::io::Result<Instruction> {
    initialize_position_pda_ix_with_program_id(crate::ID, keys, args)
}
pub fn initialize_position_pda_invoke_with_program_id(
    program_id: Pubkey,
    accounts: InitializePositionPdaAccounts<'_, '_>,
    args: InitializePositionPdaIxArgs,
) -> ProgramResult {
    let keys: InitializePositionPdaKeys = accounts.into();
    let ix = initialize_position_pda_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn initialize_position_pda_invoke(
    accounts: InitializePositionPdaAccounts<'_, '_>,
    args: InitializePositionPdaIxArgs,
) -> ProgramResult {
    initialize_position_pda_invoke_with_program_id(crate::ID, accounts, args)
}
pub fn initialize_position_pda_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: InitializePositionPdaAccounts<'_, '_>,
    args: InitializePositionPdaIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: InitializePositionPdaKeys = accounts.into();
    let ix = initialize_position_pda_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn initialize_position_pda_invoke_signed(
    accounts: InitializePositionPdaAccounts<'_, '_>,
    args: InitializePositionPdaIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    initialize_position_pda_invoke_signed_with_program_id(
        crate::ID,
        accounts,
        args,
        seeds,
    )
}
pub fn initialize_position_pda_verify_account_keys(
    accounts: InitializePositionPdaAccounts<'_, '_>,
    keys: InitializePositionPdaKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.payer.key, keys.payer),
        (*accounts.base.key, keys.base),
        (*accounts.position.key, keys.position),
        (*accounts.lb_pair.key, keys.lb_pair),
        (*accounts.owner.key, keys.owner),
        (*accounts.system_program.key, keys.system_program),
        (*accounts.rent.key, keys.rent),
        (*accounts.event_authority.key, keys.event_authority),
        (*accounts.program.key, keys.program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn initialize_position_pda_verify_writable_privileges<'me, 'info>(
    accounts: InitializePositionPdaAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.payer, accounts.position] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn initialize_position_pda_verify_signer_privileges<'me, 'info>(
    accounts: InitializePositionPdaAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.payer, accounts.base, accounts.owner] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn initialize_position_pda_verify_account_privileges<'me, 'info>(
    accounts: InitializePositionPdaAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    initialize_position_pda_verify_writable_privileges(accounts)?;
    initialize_position_pda_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const INITIALIZE_POSITION_BY_OPERATOR_IX_ACCOUNTS_LEN: usize = 11;
#[derive(Copy, Clone, Debug)]
pub struct InitializePositionByOperatorAccounts<'me, 'info> {
    pub payer: &'me AccountInfo<'info>,
    pub base: &'me AccountInfo<'info>,
    pub position: &'me AccountInfo<'info>,
    pub lb_pair: &'me AccountInfo<'info>,
    pub owner: &'me AccountInfo<'info>,
    pub operator: &'me AccountInfo<'info>,
    pub operator_token_x: &'me AccountInfo<'info>,
    pub owner_token_x: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct InitializePositionByOperatorKeys {
    pub payer: Pubkey,
    pub base: Pubkey,
    pub position: Pubkey,
    pub lb_pair: Pubkey,
    pub owner: Pubkey,
    pub operator: Pubkey,
    pub operator_token_x: Pubkey,
    pub owner_token_x: Pubkey,
    pub system_program: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<InitializePositionByOperatorAccounts<'_, '_>>
for InitializePositionByOperatorKeys {
    fn from(accounts: InitializePositionByOperatorAccounts) -> Self {
        Self {
            payer: *accounts.payer.key,
            base: *accounts.base.key,
            position: *accounts.position.key,
            lb_pair: *accounts.lb_pair.key,
            owner: *accounts.owner.key,
            operator: *accounts.operator.key,
            operator_token_x: *accounts.operator_token_x.key,
            owner_token_x: *accounts.owner_token_x.key,
            system_program: *accounts.system_program.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<InitializePositionByOperatorKeys>
for [AccountMeta; INITIALIZE_POSITION_BY_OPERATOR_IX_ACCOUNTS_LEN] {
    fn from(keys: InitializePositionByOperatorKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.payer,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.base,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.position,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.lb_pair,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.owner,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.operator,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.operator_token_x,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.owner_token_x,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.system_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.event_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; INITIALIZE_POSITION_BY_OPERATOR_IX_ACCOUNTS_LEN]>
for InitializePositionByOperatorKeys {
    fn from(pubkeys: [Pubkey; INITIALIZE_POSITION_BY_OPERATOR_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            payer: pubkeys[0],
            base: pubkeys[1],
            position: pubkeys[2],
            lb_pair: pubkeys[3],
            owner: pubkeys[4],
            operator: pubkeys[5],
            operator_token_x: pubkeys[6],
            owner_token_x: pubkeys[7],
            system_program: pubkeys[8],
            event_authority: pubkeys[9],
            program: pubkeys[10],
        }
    }
}
impl<'info> From<InitializePositionByOperatorAccounts<'_, 'info>>
for [AccountInfo<'info>; INITIALIZE_POSITION_BY_OPERATOR_IX_ACCOUNTS_LEN] {
    fn from(accounts: InitializePositionByOperatorAccounts<'_, 'info>) -> Self {
        [
            accounts.payer.clone(),
            accounts.base.clone(),
            accounts.position.clone(),
            accounts.lb_pair.clone(),
            accounts.owner.clone(),
            accounts.operator.clone(),
            accounts.operator_token_x.clone(),
            accounts.owner_token_x.clone(),
            accounts.system_program.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; INITIALIZE_POSITION_BY_OPERATOR_IX_ACCOUNTS_LEN]>
for InitializePositionByOperatorAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; INITIALIZE_POSITION_BY_OPERATOR_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            payer: &arr[0],
            base: &arr[1],
            position: &arr[2],
            lb_pair: &arr[3],
            owner: &arr[4],
            operator: &arr[5],
            operator_token_x: &arr[6],
            owner_token_x: &arr[7],
            system_program: &arr[8],
            event_authority: &arr[9],
            program: &arr[10],
        }
    }
}
pub const INITIALIZE_POSITION_BY_OPERATOR_IX_DISCM: [u8; 8] = [
    251,
    189,
    190,
    244,
    117,
    254,
    35,
    148,
];
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InitializePositionByOperatorIxArgs {
    pub lower_bin_id: i32,
    pub width: i32,
    pub fee_owner: Pubkey,
    pub lock_release_point: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct InitializePositionByOperatorIxData(pub InitializePositionByOperatorIxArgs);
impl From<InitializePositionByOperatorIxArgs> for InitializePositionByOperatorIxData {
    fn from(args: InitializePositionByOperatorIxArgs) -> Self {
        Self(args)
    }
}
impl InitializePositionByOperatorIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != INITIALIZE_POSITION_BY_OPERATOR_IX_DISCM {
            return Err(
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!(
                        "discm does not match. Expected: {:?}. Received: {:?}",
                        INITIALIZE_POSITION_BY_OPERATOR_IX_DISCM, maybe_discm
                    ),
                ),
            );
        }
        Ok(Self(InitializePositionByOperatorIxArgs::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&INITIALIZE_POSITION_BY_OPERATOR_IX_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn initialize_position_by_operator_ix_with_program_id(
    program_id: Pubkey,
    keys: InitializePositionByOperatorKeys,
    args: InitializePositionByOperatorIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; INITIALIZE_POSITION_BY_OPERATOR_IX_ACCOUNTS_LEN] = keys
        .into();
    let data: InitializePositionByOperatorIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn initialize_position_by_operator_ix(
    keys: InitializePositionByOperatorKeys,
    args: InitializePositionByOperatorIxArgs,
) -> std::io::Result<Instruction> {
    initialize_position_by_operator_ix_with_program_id(crate::ID, keys, args)
}
pub fn initialize_position_by_operator_invoke_with_program_id(
    program_id: Pubkey,
    accounts: InitializePositionByOperatorAccounts<'_, '_>,
    args: InitializePositionByOperatorIxArgs,
) -> ProgramResult {
    let keys: InitializePositionByOperatorKeys = accounts.into();
    let ix = initialize_position_by_operator_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn initialize_position_by_operator_invoke(
    accounts: InitializePositionByOperatorAccounts<'_, '_>,
    args: InitializePositionByOperatorIxArgs,
) -> ProgramResult {
    initialize_position_by_operator_invoke_with_program_id(crate::ID, accounts, args)
}
pub fn initialize_position_by_operator_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: InitializePositionByOperatorAccounts<'_, '_>,
    args: InitializePositionByOperatorIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: InitializePositionByOperatorKeys = accounts.into();
    let ix = initialize_position_by_operator_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn initialize_position_by_operator_invoke_signed(
    accounts: InitializePositionByOperatorAccounts<'_, '_>,
    args: InitializePositionByOperatorIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    initialize_position_by_operator_invoke_signed_with_program_id(
        crate::ID,
        accounts,
        args,
        seeds,
    )
}
pub fn initialize_position_by_operator_verify_account_keys(
    accounts: InitializePositionByOperatorAccounts<'_, '_>,
    keys: InitializePositionByOperatorKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.payer.key, keys.payer),
        (*accounts.base.key, keys.base),
        (*accounts.position.key, keys.position),
        (*accounts.lb_pair.key, keys.lb_pair),
        (*accounts.owner.key, keys.owner),
        (*accounts.operator.key, keys.operator),
        (*accounts.operator_token_x.key, keys.operator_token_x),
        (*accounts.owner_token_x.key, keys.owner_token_x),
        (*accounts.system_program.key, keys.system_program),
        (*accounts.event_authority.key, keys.event_authority),
        (*accounts.program.key, keys.program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn initialize_position_by_operator_verify_writable_privileges<'me, 'info>(
    accounts: InitializePositionByOperatorAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.payer, accounts.position] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn initialize_position_by_operator_verify_signer_privileges<'me, 'info>(
    accounts: InitializePositionByOperatorAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.payer, accounts.base, accounts.operator] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn initialize_position_by_operator_verify_account_privileges<'me, 'info>(
    accounts: InitializePositionByOperatorAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    initialize_position_by_operator_verify_writable_privileges(accounts)?;
    initialize_position_by_operator_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const UPDATE_POSITION_OPERATOR_IX_ACCOUNTS_LEN: usize = 4;
#[derive(Copy, Clone, Debug)]
pub struct UpdatePositionOperatorAccounts<'me, 'info> {
    pub position: &'me AccountInfo<'info>,
    pub owner: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct UpdatePositionOperatorKeys {
    pub position: Pubkey,
    pub owner: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<UpdatePositionOperatorAccounts<'_, '_>> for UpdatePositionOperatorKeys {
    fn from(accounts: UpdatePositionOperatorAccounts) -> Self {
        Self {
            position: *accounts.position.key,
            owner: *accounts.owner.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<UpdatePositionOperatorKeys>
for [AccountMeta; UPDATE_POSITION_OPERATOR_IX_ACCOUNTS_LEN] {
    fn from(keys: UpdatePositionOperatorKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.position,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.owner,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.event_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; UPDATE_POSITION_OPERATOR_IX_ACCOUNTS_LEN]>
for UpdatePositionOperatorKeys {
    fn from(pubkeys: [Pubkey; UPDATE_POSITION_OPERATOR_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            position: pubkeys[0],
            owner: pubkeys[1],
            event_authority: pubkeys[2],
            program: pubkeys[3],
        }
    }
}
impl<'info> From<UpdatePositionOperatorAccounts<'_, 'info>>
for [AccountInfo<'info>; UPDATE_POSITION_OPERATOR_IX_ACCOUNTS_LEN] {
    fn from(accounts: UpdatePositionOperatorAccounts<'_, 'info>) -> Self {
        [
            accounts.position.clone(),
            accounts.owner.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; UPDATE_POSITION_OPERATOR_IX_ACCOUNTS_LEN]>
for UpdatePositionOperatorAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; UPDATE_POSITION_OPERATOR_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            position: &arr[0],
            owner: &arr[1],
            event_authority: &arr[2],
            program: &arr[3],
        }
    }
}
pub const UPDATE_POSITION_OPERATOR_IX_DISCM: [u8; 8] = [
    202,
    184,
    103,
    143,
    180,
    191,
    116,
    217,
];
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct UpdatePositionOperatorIxArgs {
    pub operator: Pubkey,
}
#[derive(Clone, Debug, PartialEq)]
pub struct UpdatePositionOperatorIxData(pub UpdatePositionOperatorIxArgs);
impl From<UpdatePositionOperatorIxArgs> for UpdatePositionOperatorIxData {
    fn from(args: UpdatePositionOperatorIxArgs) -> Self {
        Self(args)
    }
}
impl UpdatePositionOperatorIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != UPDATE_POSITION_OPERATOR_IX_DISCM {
            return Err(
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!(
                        "discm does not match. Expected: {:?}. Received: {:?}",
                        UPDATE_POSITION_OPERATOR_IX_DISCM, maybe_discm
                    ),
                ),
            );
        }
        Ok(Self(UpdatePositionOperatorIxArgs::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&UPDATE_POSITION_OPERATOR_IX_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn update_position_operator_ix_with_program_id(
    program_id: Pubkey,
    keys: UpdatePositionOperatorKeys,
    args: UpdatePositionOperatorIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; UPDATE_POSITION_OPERATOR_IX_ACCOUNTS_LEN] = keys.into();
    let data: UpdatePositionOperatorIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn update_position_operator_ix(
    keys: UpdatePositionOperatorKeys,
    args: UpdatePositionOperatorIxArgs,
) -> std::io::Result<Instruction> {
    update_position_operator_ix_with_program_id(crate::ID, keys, args)
}
pub fn update_position_operator_invoke_with_program_id(
    program_id: Pubkey,
    accounts: UpdatePositionOperatorAccounts<'_, '_>,
    args: UpdatePositionOperatorIxArgs,
) -> ProgramResult {
    let keys: UpdatePositionOperatorKeys = accounts.into();
    let ix = update_position_operator_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn update_position_operator_invoke(
    accounts: UpdatePositionOperatorAccounts<'_, '_>,
    args: UpdatePositionOperatorIxArgs,
) -> ProgramResult {
    update_position_operator_invoke_with_program_id(crate::ID, accounts, args)
}
pub fn update_position_operator_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: UpdatePositionOperatorAccounts<'_, '_>,
    args: UpdatePositionOperatorIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: UpdatePositionOperatorKeys = accounts.into();
    let ix = update_position_operator_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn update_position_operator_invoke_signed(
    accounts: UpdatePositionOperatorAccounts<'_, '_>,
    args: UpdatePositionOperatorIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    update_position_operator_invoke_signed_with_program_id(
        crate::ID,
        accounts,
        args,
        seeds,
    )
}
pub fn update_position_operator_verify_account_keys(
    accounts: UpdatePositionOperatorAccounts<'_, '_>,
    keys: UpdatePositionOperatorKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.position.key, keys.position),
        (*accounts.owner.key, keys.owner),
        (*accounts.event_authority.key, keys.event_authority),
        (*accounts.program.key, keys.program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn update_position_operator_verify_writable_privileges<'me, 'info>(
    accounts: UpdatePositionOperatorAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.position] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn update_position_operator_verify_signer_privileges<'me, 'info>(
    accounts: UpdatePositionOperatorAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.owner] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn update_position_operator_verify_account_privileges<'me, 'info>(
    accounts: UpdatePositionOperatorAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    update_position_operator_verify_writable_privileges(accounts)?;
    update_position_operator_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const SWAP_IX_ACCOUNTS_LEN: usize = 15;
#[derive(Copy, Clone, Debug)]
pub struct SwapAccounts<'me, 'info> {
    pub lb_pair: &'me AccountInfo<'info>,
    pub bin_array_bitmap_extension: &'me AccountInfo<'info>,
    pub reserve_x: &'me AccountInfo<'info>,
    pub reserve_y: &'me AccountInfo<'info>,
    pub user_token_in: &'me AccountInfo<'info>,
    pub user_token_out: &'me AccountInfo<'info>,
    pub token_x_mint: &'me AccountInfo<'info>,
    pub token_y_mint: &'me AccountInfo<'info>,
    pub oracle: &'me AccountInfo<'info>,
    pub host_fee_in: &'me AccountInfo<'info>,
    pub user: &'me AccountInfo<'info>,
    pub token_x_program: &'me AccountInfo<'info>,
    pub token_y_program: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SwapKeys {
    pub lb_pair: Pubkey,
    pub bin_array_bitmap_extension: Pubkey,
    pub reserve_x: Pubkey,
    pub reserve_y: Pubkey,
    pub user_token_in: Pubkey,
    pub user_token_out: Pubkey,
    pub token_x_mint: Pubkey,
    pub token_y_mint: Pubkey,
    pub oracle: Pubkey,
    pub host_fee_in: Pubkey,
    pub user: Pubkey,
    pub token_x_program: Pubkey,
    pub token_y_program: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<SwapAccounts<'_, '_>> for SwapKeys {
    fn from(accounts: SwapAccounts) -> Self {
        Self {
            lb_pair: *accounts.lb_pair.key,
            bin_array_bitmap_extension: *accounts.bin_array_bitmap_extension.key,
            reserve_x: *accounts.reserve_x.key,
            reserve_y: *accounts.reserve_y.key,
            user_token_in: *accounts.user_token_in.key,
            user_token_out: *accounts.user_token_out.key,
            token_x_mint: *accounts.token_x_mint.key,
            token_y_mint: *accounts.token_y_mint.key,
            oracle: *accounts.oracle.key,
            host_fee_in: *accounts.host_fee_in.key,
            user: *accounts.user.key,
            token_x_program: *accounts.token_x_program.key,
            token_y_program: *accounts.token_y_program.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<SwapKeys> for [AccountMeta; SWAP_IX_ACCOUNTS_LEN] {
    fn from(keys: SwapKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.lb_pair,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.bin_array_bitmap_extension,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.reserve_x,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.reserve_y,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_token_in,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_token_out,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_x_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_y_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.oracle,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.host_fee_in,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_x_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_y_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.event_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; SWAP_IX_ACCOUNTS_LEN]> for SwapKeys {
    fn from(pubkeys: [Pubkey; SWAP_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            lb_pair: pubkeys[0],
            bin_array_bitmap_extension: pubkeys[1],
            reserve_x: pubkeys[2],
            reserve_y: pubkeys[3],
            user_token_in: pubkeys[4],
            user_token_out: pubkeys[5],
            token_x_mint: pubkeys[6],
            token_y_mint: pubkeys[7],
            oracle: pubkeys[8],
            host_fee_in: pubkeys[9],
            user: pubkeys[10],
            token_x_program: pubkeys[11],
            token_y_program: pubkeys[12],
            event_authority: pubkeys[13],
            program: pubkeys[14],
        }
    }
}
impl<'info> From<SwapAccounts<'_, 'info>>
for [AccountInfo<'info>; SWAP_IX_ACCOUNTS_LEN] {
    fn from(accounts: SwapAccounts<'_, 'info>) -> Self {
        [
            accounts.lb_pair.clone(),
            accounts.bin_array_bitmap_extension.clone(),
            accounts.reserve_x.clone(),
            accounts.reserve_y.clone(),
            accounts.user_token_in.clone(),
            accounts.user_token_out.clone(),
            accounts.token_x_mint.clone(),
            accounts.token_y_mint.clone(),
            accounts.oracle.clone(),
            accounts.host_fee_in.clone(),
            accounts.user.clone(),
            accounts.token_x_program.clone(),
            accounts.token_y_program.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; SWAP_IX_ACCOUNTS_LEN]>
for SwapAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; SWAP_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            lb_pair: &arr[0],
            bin_array_bitmap_extension: &arr[1],
            reserve_x: &arr[2],
            reserve_y: &arr[3],
            user_token_in: &arr[4],
            user_token_out: &arr[5],
            token_x_mint: &arr[6],
            token_y_mint: &arr[7],
            oracle: &arr[8],
            host_fee_in: &arr[9],
            user: &arr[10],
            token_x_program: &arr[11],
            token_y_program: &arr[12],
            event_authority: &arr[13],
            program: &arr[14],
        }
    }
}
pub const SWAP_IX_DISCM: [u8; 8] = [248, 198, 158, 145, 225, 117, 135, 200];
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SwapIxArgs {
    pub amount_in: u64,
    pub min_amount_out: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct SwapIxData(pub SwapIxArgs);
impl From<SwapIxArgs> for SwapIxData {
    fn from(args: SwapIxArgs) -> Self {
        Self(args)
    }
}
impl SwapIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != SWAP_IX_DISCM {
            return Err(
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!(
                        "discm does not match. Expected: {:?}. Received: {:?}",
                        SWAP_IX_DISCM, maybe_discm
                    ),
                ),
            );
        }
        Ok(Self(SwapIxArgs::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&SWAP_IX_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn swap_ix_with_program_id(
    program_id: Pubkey,
    keys: SwapKeys,
    args: SwapIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; SWAP_IX_ACCOUNTS_LEN] = keys.into();
    let data: SwapIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn swap_ix(keys: SwapKeys, args: SwapIxArgs) -> std::io::Result<Instruction> {
    swap_ix_with_program_id(crate::ID, keys, args)
}
pub fn swap_invoke_with_program_id(
    program_id: Pubkey,
    accounts: SwapAccounts<'_, '_>,
    args: SwapIxArgs,
) -> ProgramResult {
    let keys: SwapKeys = accounts.into();
    let ix = swap_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn swap_invoke(accounts: SwapAccounts<'_, '_>, args: SwapIxArgs) -> ProgramResult {
    swap_invoke_with_program_id(crate::ID, accounts, args)
}
pub fn swap_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: SwapAccounts<'_, '_>,
    args: SwapIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: SwapKeys = accounts.into();
    let ix = swap_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn swap_invoke_signed(
    accounts: SwapAccounts<'_, '_>,
    args: SwapIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    swap_invoke_signed_with_program_id(crate::ID, accounts, args, seeds)
}
pub fn swap_verify_account_keys(
    accounts: SwapAccounts<'_, '_>,
    keys: SwapKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.lb_pair.key, keys.lb_pair),
        (*accounts.bin_array_bitmap_extension.key, keys.bin_array_bitmap_extension),
        (*accounts.reserve_x.key, keys.reserve_x),
        (*accounts.reserve_y.key, keys.reserve_y),
        (*accounts.user_token_in.key, keys.user_token_in),
        (*accounts.user_token_out.key, keys.user_token_out),
        (*accounts.token_x_mint.key, keys.token_x_mint),
        (*accounts.token_y_mint.key, keys.token_y_mint),
        (*accounts.oracle.key, keys.oracle),
        (*accounts.host_fee_in.key, keys.host_fee_in),
        (*accounts.user.key, keys.user),
        (*accounts.token_x_program.key, keys.token_x_program),
        (*accounts.token_y_program.key, keys.token_y_program),
        (*accounts.event_authority.key, keys.event_authority),
        (*accounts.program.key, keys.program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn swap_verify_writable_privileges<'me, 'info>(
    accounts: SwapAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.lb_pair,
        accounts.reserve_x,
        accounts.reserve_y,
        accounts.user_token_in,
        accounts.user_token_out,
        accounts.oracle,
        accounts.host_fee_in,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn swap_verify_signer_privileges<'me, 'info>(
    accounts: SwapAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.user] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn swap_verify_account_privileges<'me, 'info>(
    accounts: SwapAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    swap_verify_writable_privileges(accounts)?;
    swap_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const SWAP_EXACT_OUT_IX_ACCOUNTS_LEN: usize = 15;
#[derive(Copy, Clone, Debug)]
pub struct SwapExactOutAccounts<'me, 'info> {
    pub lb_pair: &'me AccountInfo<'info>,
    pub bin_array_bitmap_extension: &'me AccountInfo<'info>,
    pub reserve_x: &'me AccountInfo<'info>,
    pub reserve_y: &'me AccountInfo<'info>,
    pub user_token_in: &'me AccountInfo<'info>,
    pub user_token_out: &'me AccountInfo<'info>,
    pub token_x_mint: &'me AccountInfo<'info>,
    pub token_y_mint: &'me AccountInfo<'info>,
    pub oracle: &'me AccountInfo<'info>,
    pub host_fee_in: &'me AccountInfo<'info>,
    pub user: &'me AccountInfo<'info>,
    pub token_x_program: &'me AccountInfo<'info>,
    pub token_y_program: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SwapExactOutKeys {
    pub lb_pair: Pubkey,
    pub bin_array_bitmap_extension: Pubkey,
    pub reserve_x: Pubkey,
    pub reserve_y: Pubkey,
    pub user_token_in: Pubkey,
    pub user_token_out: Pubkey,
    pub token_x_mint: Pubkey,
    pub token_y_mint: Pubkey,
    pub oracle: Pubkey,
    pub host_fee_in: Pubkey,
    pub user: Pubkey,
    pub token_x_program: Pubkey,
    pub token_y_program: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<SwapExactOutAccounts<'_, '_>> for SwapExactOutKeys {
    fn from(accounts: SwapExactOutAccounts) -> Self {
        Self {
            lb_pair: *accounts.lb_pair.key,
            bin_array_bitmap_extension: *accounts.bin_array_bitmap_extension.key,
            reserve_x: *accounts.reserve_x.key,
            reserve_y: *accounts.reserve_y.key,
            user_token_in: *accounts.user_token_in.key,
            user_token_out: *accounts.user_token_out.key,
            token_x_mint: *accounts.token_x_mint.key,
            token_y_mint: *accounts.token_y_mint.key,
            oracle: *accounts.oracle.key,
            host_fee_in: *accounts.host_fee_in.key,
            user: *accounts.user.key,
            token_x_program: *accounts.token_x_program.key,
            token_y_program: *accounts.token_y_program.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<SwapExactOutKeys> for [AccountMeta; SWAP_EXACT_OUT_IX_ACCOUNTS_LEN] {
    fn from(keys: SwapExactOutKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.lb_pair,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.bin_array_bitmap_extension,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.reserve_x,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.reserve_y,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_token_in,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_token_out,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_x_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_y_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.oracle,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.host_fee_in,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_x_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_y_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.event_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; SWAP_EXACT_OUT_IX_ACCOUNTS_LEN]> for SwapExactOutKeys {
    fn from(pubkeys: [Pubkey; SWAP_EXACT_OUT_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            lb_pair: pubkeys[0],
            bin_array_bitmap_extension: pubkeys[1],
            reserve_x: pubkeys[2],
            reserve_y: pubkeys[3],
            user_token_in: pubkeys[4],
            user_token_out: pubkeys[5],
            token_x_mint: pubkeys[6],
            token_y_mint: pubkeys[7],
            oracle: pubkeys[8],
            host_fee_in: pubkeys[9],
            user: pubkeys[10],
            token_x_program: pubkeys[11],
            token_y_program: pubkeys[12],
            event_authority: pubkeys[13],
            program: pubkeys[14],
        }
    }
}
impl<'info> From<SwapExactOutAccounts<'_, 'info>>
for [AccountInfo<'info>; SWAP_EXACT_OUT_IX_ACCOUNTS_LEN] {
    fn from(accounts: SwapExactOutAccounts<'_, 'info>) -> Self {
        [
            accounts.lb_pair.clone(),
            accounts.bin_array_bitmap_extension.clone(),
            accounts.reserve_x.clone(),
            accounts.reserve_y.clone(),
            accounts.user_token_in.clone(),
            accounts.user_token_out.clone(),
            accounts.token_x_mint.clone(),
            accounts.token_y_mint.clone(),
            accounts.oracle.clone(),
            accounts.host_fee_in.clone(),
            accounts.user.clone(),
            accounts.token_x_program.clone(),
            accounts.token_y_program.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; SWAP_EXACT_OUT_IX_ACCOUNTS_LEN]>
for SwapExactOutAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; SWAP_EXACT_OUT_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            lb_pair: &arr[0],
            bin_array_bitmap_extension: &arr[1],
            reserve_x: &arr[2],
            reserve_y: &arr[3],
            user_token_in: &arr[4],
            user_token_out: &arr[5],
            token_x_mint: &arr[6],
            token_y_mint: &arr[7],
            oracle: &arr[8],
            host_fee_in: &arr[9],
            user: &arr[10],
            token_x_program: &arr[11],
            token_y_program: &arr[12],
            event_authority: &arr[13],
            program: &arr[14],
        }
    }
}
pub const SWAP_EXACT_OUT_IX_DISCM: [u8; 8] = [250, 73, 101, 33, 38, 207, 75, 184];
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SwapExactOutIxArgs {
    pub max_in_amount: u64,
    pub out_amount: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct SwapExactOutIxData(pub SwapExactOutIxArgs);
impl From<SwapExactOutIxArgs> for SwapExactOutIxData {
    fn from(args: SwapExactOutIxArgs) -> Self {
        Self(args)
    }
}
impl SwapExactOutIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != SWAP_EXACT_OUT_IX_DISCM {
            return Err(
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!(
                        "discm does not match. Expected: {:?}. Received: {:?}",
                        SWAP_EXACT_OUT_IX_DISCM, maybe_discm
                    ),
                ),
            );
        }
        Ok(Self(SwapExactOutIxArgs::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&SWAP_EXACT_OUT_IX_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn swap_exact_out_ix_with_program_id(
    program_id: Pubkey,
    keys: SwapExactOutKeys,
    args: SwapExactOutIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; SWAP_EXACT_OUT_IX_ACCOUNTS_LEN] = keys.into();
    let data: SwapExactOutIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn swap_exact_out_ix(
    keys: SwapExactOutKeys,
    args: SwapExactOutIxArgs,
) -> std::io::Result<Instruction> {
    swap_exact_out_ix_with_program_id(crate::ID, keys, args)
}
pub fn swap_exact_out_invoke_with_program_id(
    program_id: Pubkey,
    accounts: SwapExactOutAccounts<'_, '_>,
    args: SwapExactOutIxArgs,
) -> ProgramResult {
    let keys: SwapExactOutKeys = accounts.into();
    let ix = swap_exact_out_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn swap_exact_out_invoke(
    accounts: SwapExactOutAccounts<'_, '_>,
    args: SwapExactOutIxArgs,
) -> ProgramResult {
    swap_exact_out_invoke_with_program_id(crate::ID, accounts, args)
}
pub fn swap_exact_out_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: SwapExactOutAccounts<'_, '_>,
    args: SwapExactOutIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: SwapExactOutKeys = accounts.into();
    let ix = swap_exact_out_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn swap_exact_out_invoke_signed(
    accounts: SwapExactOutAccounts<'_, '_>,
    args: SwapExactOutIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    swap_exact_out_invoke_signed_with_program_id(crate::ID, accounts, args, seeds)
}
pub fn swap_exact_out_verify_account_keys(
    accounts: SwapExactOutAccounts<'_, '_>,
    keys: SwapExactOutKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.lb_pair.key, keys.lb_pair),
        (*accounts.bin_array_bitmap_extension.key, keys.bin_array_bitmap_extension),
        (*accounts.reserve_x.key, keys.reserve_x),
        (*accounts.reserve_y.key, keys.reserve_y),
        (*accounts.user_token_in.key, keys.user_token_in),
        (*accounts.user_token_out.key, keys.user_token_out),
        (*accounts.token_x_mint.key, keys.token_x_mint),
        (*accounts.token_y_mint.key, keys.token_y_mint),
        (*accounts.oracle.key, keys.oracle),
        (*accounts.host_fee_in.key, keys.host_fee_in),
        (*accounts.user.key, keys.user),
        (*accounts.token_x_program.key, keys.token_x_program),
        (*accounts.token_y_program.key, keys.token_y_program),
        (*accounts.event_authority.key, keys.event_authority),
        (*accounts.program.key, keys.program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn swap_exact_out_verify_writable_privileges<'me, 'info>(
    accounts: SwapExactOutAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.lb_pair,
        accounts.reserve_x,
        accounts.reserve_y,
        accounts.user_token_in,
        accounts.user_token_out,
        accounts.oracle,
        accounts.host_fee_in,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn swap_exact_out_verify_signer_privileges<'me, 'info>(
    accounts: SwapExactOutAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.user] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn swap_exact_out_verify_account_privileges<'me, 'info>(
    accounts: SwapExactOutAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    swap_exact_out_verify_writable_privileges(accounts)?;
    swap_exact_out_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const SWAP_WITH_PRICE_IMPACT_IX_ACCOUNTS_LEN: usize = 15;
#[derive(Copy, Clone, Debug)]
pub struct SwapWithPriceImpactAccounts<'me, 'info> {
    pub lb_pair: &'me AccountInfo<'info>,
    pub bin_array_bitmap_extension: &'me AccountInfo<'info>,
    pub reserve_x: &'me AccountInfo<'info>,
    pub reserve_y: &'me AccountInfo<'info>,
    pub user_token_in: &'me AccountInfo<'info>,
    pub user_token_out: &'me AccountInfo<'info>,
    pub token_x_mint: &'me AccountInfo<'info>,
    pub token_y_mint: &'me AccountInfo<'info>,
    pub oracle: &'me AccountInfo<'info>,
    pub host_fee_in: &'me AccountInfo<'info>,
    pub user: &'me AccountInfo<'info>,
    pub token_x_program: &'me AccountInfo<'info>,
    pub token_y_program: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SwapWithPriceImpactKeys {
    pub lb_pair: Pubkey,
    pub bin_array_bitmap_extension: Pubkey,
    pub reserve_x: Pubkey,
    pub reserve_y: Pubkey,
    pub user_token_in: Pubkey,
    pub user_token_out: Pubkey,
    pub token_x_mint: Pubkey,
    pub token_y_mint: Pubkey,
    pub oracle: Pubkey,
    pub host_fee_in: Pubkey,
    pub user: Pubkey,
    pub token_x_program: Pubkey,
    pub token_y_program: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<SwapWithPriceImpactAccounts<'_, '_>> for SwapWithPriceImpactKeys {
    fn from(accounts: SwapWithPriceImpactAccounts) -> Self {
        Self {
            lb_pair: *accounts.lb_pair.key,
            bin_array_bitmap_extension: *accounts.bin_array_bitmap_extension.key,
            reserve_x: *accounts.reserve_x.key,
            reserve_y: *accounts.reserve_y.key,
            user_token_in: *accounts.user_token_in.key,
            user_token_out: *accounts.user_token_out.key,
            token_x_mint: *accounts.token_x_mint.key,
            token_y_mint: *accounts.token_y_mint.key,
            oracle: *accounts.oracle.key,
            host_fee_in: *accounts.host_fee_in.key,
            user: *accounts.user.key,
            token_x_program: *accounts.token_x_program.key,
            token_y_program: *accounts.token_y_program.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<SwapWithPriceImpactKeys>
for [AccountMeta; SWAP_WITH_PRICE_IMPACT_IX_ACCOUNTS_LEN] {
    fn from(keys: SwapWithPriceImpactKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.lb_pair,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.bin_array_bitmap_extension,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.reserve_x,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.reserve_y,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_token_in,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_token_out,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_x_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_y_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.oracle,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.host_fee_in,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_x_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_y_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.event_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; SWAP_WITH_PRICE_IMPACT_IX_ACCOUNTS_LEN]> for SwapWithPriceImpactKeys {
    fn from(pubkeys: [Pubkey; SWAP_WITH_PRICE_IMPACT_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            lb_pair: pubkeys[0],
            bin_array_bitmap_extension: pubkeys[1],
            reserve_x: pubkeys[2],
            reserve_y: pubkeys[3],
            user_token_in: pubkeys[4],
            user_token_out: pubkeys[5],
            token_x_mint: pubkeys[6],
            token_y_mint: pubkeys[7],
            oracle: pubkeys[8],
            host_fee_in: pubkeys[9],
            user: pubkeys[10],
            token_x_program: pubkeys[11],
            token_y_program: pubkeys[12],
            event_authority: pubkeys[13],
            program: pubkeys[14],
        }
    }
}
impl<'info> From<SwapWithPriceImpactAccounts<'_, 'info>>
for [AccountInfo<'info>; SWAP_WITH_PRICE_IMPACT_IX_ACCOUNTS_LEN] {
    fn from(accounts: SwapWithPriceImpactAccounts<'_, 'info>) -> Self {
        [
            accounts.lb_pair.clone(),
            accounts.bin_array_bitmap_extension.clone(),
            accounts.reserve_x.clone(),
            accounts.reserve_y.clone(),
            accounts.user_token_in.clone(),
            accounts.user_token_out.clone(),
            accounts.token_x_mint.clone(),
            accounts.token_y_mint.clone(),
            accounts.oracle.clone(),
            accounts.host_fee_in.clone(),
            accounts.user.clone(),
            accounts.token_x_program.clone(),
            accounts.token_y_program.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; SWAP_WITH_PRICE_IMPACT_IX_ACCOUNTS_LEN]>
for SwapWithPriceImpactAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; SWAP_WITH_PRICE_IMPACT_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            lb_pair: &arr[0],
            bin_array_bitmap_extension: &arr[1],
            reserve_x: &arr[2],
            reserve_y: &arr[3],
            user_token_in: &arr[4],
            user_token_out: &arr[5],
            token_x_mint: &arr[6],
            token_y_mint: &arr[7],
            oracle: &arr[8],
            host_fee_in: &arr[9],
            user: &arr[10],
            token_x_program: &arr[11],
            token_y_program: &arr[12],
            event_authority: &arr[13],
            program: &arr[14],
        }
    }
}
pub const SWAP_WITH_PRICE_IMPACT_IX_DISCM: [u8; 8] = [
    56,
    173,
    230,
    208,
    173,
    228,
    156,
    205,
];
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SwapWithPriceImpactIxArgs {
    pub amount_in: u64,
    pub active_id: Option<i32>,
    pub max_price_impact_bps: u16,
}
#[derive(Clone, Debug, PartialEq)]
pub struct SwapWithPriceImpactIxData(pub SwapWithPriceImpactIxArgs);
impl From<SwapWithPriceImpactIxArgs> for SwapWithPriceImpactIxData {
    fn from(args: SwapWithPriceImpactIxArgs) -> Self {
        Self(args)
    }
}
impl SwapWithPriceImpactIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != SWAP_WITH_PRICE_IMPACT_IX_DISCM {
            return Err(
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!(
                        "discm does not match. Expected: {:?}. Received: {:?}",
                        SWAP_WITH_PRICE_IMPACT_IX_DISCM, maybe_discm
                    ),
                ),
            );
        }
        Ok(Self(SwapWithPriceImpactIxArgs::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&SWAP_WITH_PRICE_IMPACT_IX_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn swap_with_price_impact_ix_with_program_id(
    program_id: Pubkey,
    keys: SwapWithPriceImpactKeys,
    args: SwapWithPriceImpactIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; SWAP_WITH_PRICE_IMPACT_IX_ACCOUNTS_LEN] = keys.into();
    let data: SwapWithPriceImpactIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn swap_with_price_impact_ix(
    keys: SwapWithPriceImpactKeys,
    args: SwapWithPriceImpactIxArgs,
) -> std::io::Result<Instruction> {
    swap_with_price_impact_ix_with_program_id(crate::ID, keys, args)
}
pub fn swap_with_price_impact_invoke_with_program_id(
    program_id: Pubkey,
    accounts: SwapWithPriceImpactAccounts<'_, '_>,
    args: SwapWithPriceImpactIxArgs,
) -> ProgramResult {
    let keys: SwapWithPriceImpactKeys = accounts.into();
    let ix = swap_with_price_impact_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn swap_with_price_impact_invoke(
    accounts: SwapWithPriceImpactAccounts<'_, '_>,
    args: SwapWithPriceImpactIxArgs,
) -> ProgramResult {
    swap_with_price_impact_invoke_with_program_id(crate::ID, accounts, args)
}
pub fn swap_with_price_impact_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: SwapWithPriceImpactAccounts<'_, '_>,
    args: SwapWithPriceImpactIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: SwapWithPriceImpactKeys = accounts.into();
    let ix = swap_with_price_impact_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn swap_with_price_impact_invoke_signed(
    accounts: SwapWithPriceImpactAccounts<'_, '_>,
    args: SwapWithPriceImpactIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    swap_with_price_impact_invoke_signed_with_program_id(
        crate::ID,
        accounts,
        args,
        seeds,
    )
}
pub fn swap_with_price_impact_verify_account_keys(
    accounts: SwapWithPriceImpactAccounts<'_, '_>,
    keys: SwapWithPriceImpactKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.lb_pair.key, keys.lb_pair),
        (*accounts.bin_array_bitmap_extension.key, keys.bin_array_bitmap_extension),
        (*accounts.reserve_x.key, keys.reserve_x),
        (*accounts.reserve_y.key, keys.reserve_y),
        (*accounts.user_token_in.key, keys.user_token_in),
        (*accounts.user_token_out.key, keys.user_token_out),
        (*accounts.token_x_mint.key, keys.token_x_mint),
        (*accounts.token_y_mint.key, keys.token_y_mint),
        (*accounts.oracle.key, keys.oracle),
        (*accounts.host_fee_in.key, keys.host_fee_in),
        (*accounts.user.key, keys.user),
        (*accounts.token_x_program.key, keys.token_x_program),
        (*accounts.token_y_program.key, keys.token_y_program),
        (*accounts.event_authority.key, keys.event_authority),
        (*accounts.program.key, keys.program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn swap_with_price_impact_verify_writable_privileges<'me, 'info>(
    accounts: SwapWithPriceImpactAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.lb_pair,
        accounts.reserve_x,
        accounts.reserve_y,
        accounts.user_token_in,
        accounts.user_token_out,
        accounts.oracle,
        accounts.host_fee_in,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn swap_with_price_impact_verify_signer_privileges<'me, 'info>(
    accounts: SwapWithPriceImpactAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.user] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn swap_with_price_impact_verify_account_privileges<'me, 'info>(
    accounts: SwapWithPriceImpactAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    swap_with_price_impact_verify_writable_privileges(accounts)?;
    swap_with_price_impact_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const WITHDRAW_PROTOCOL_FEE_IX_ACCOUNTS_LEN: usize = 9;
#[derive(Copy, Clone, Debug)]
pub struct WithdrawProtocolFeeAccounts<'me, 'info> {
    pub lb_pair: &'me AccountInfo<'info>,
    pub reserve_x: &'me AccountInfo<'info>,
    pub reserve_y: &'me AccountInfo<'info>,
    pub token_x_mint: &'me AccountInfo<'info>,
    pub token_y_mint: &'me AccountInfo<'info>,
    pub receiver_token_x: &'me AccountInfo<'info>,
    pub receiver_token_y: &'me AccountInfo<'info>,
    pub token_x_program: &'me AccountInfo<'info>,
    pub token_y_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct WithdrawProtocolFeeKeys {
    pub lb_pair: Pubkey,
    pub reserve_x: Pubkey,
    pub reserve_y: Pubkey,
    pub token_x_mint: Pubkey,
    pub token_y_mint: Pubkey,
    pub receiver_token_x: Pubkey,
    pub receiver_token_y: Pubkey,
    pub token_x_program: Pubkey,
    pub token_y_program: Pubkey,
}
impl From<WithdrawProtocolFeeAccounts<'_, '_>> for WithdrawProtocolFeeKeys {
    fn from(accounts: WithdrawProtocolFeeAccounts) -> Self {
        Self {
            lb_pair: *accounts.lb_pair.key,
            reserve_x: *accounts.reserve_x.key,
            reserve_y: *accounts.reserve_y.key,
            token_x_mint: *accounts.token_x_mint.key,
            token_y_mint: *accounts.token_y_mint.key,
            receiver_token_x: *accounts.receiver_token_x.key,
            receiver_token_y: *accounts.receiver_token_y.key,
            token_x_program: *accounts.token_x_program.key,
            token_y_program: *accounts.token_y_program.key,
        }
    }
}
impl From<WithdrawProtocolFeeKeys>
for [AccountMeta; WITHDRAW_PROTOCOL_FEE_IX_ACCOUNTS_LEN] {
    fn from(keys: WithdrawProtocolFeeKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.lb_pair,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.reserve_x,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.reserve_y,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_x_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_y_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.receiver_token_x,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.receiver_token_y,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_x_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_y_program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; WITHDRAW_PROTOCOL_FEE_IX_ACCOUNTS_LEN]> for WithdrawProtocolFeeKeys {
    fn from(pubkeys: [Pubkey; WITHDRAW_PROTOCOL_FEE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            lb_pair: pubkeys[0],
            reserve_x: pubkeys[1],
            reserve_y: pubkeys[2],
            token_x_mint: pubkeys[3],
            token_y_mint: pubkeys[4],
            receiver_token_x: pubkeys[5],
            receiver_token_y: pubkeys[6],
            token_x_program: pubkeys[7],
            token_y_program: pubkeys[8],
        }
    }
}
impl<'info> From<WithdrawProtocolFeeAccounts<'_, 'info>>
for [AccountInfo<'info>; WITHDRAW_PROTOCOL_FEE_IX_ACCOUNTS_LEN] {
    fn from(accounts: WithdrawProtocolFeeAccounts<'_, 'info>) -> Self {
        [
            accounts.lb_pair.clone(),
            accounts.reserve_x.clone(),
            accounts.reserve_y.clone(),
            accounts.token_x_mint.clone(),
            accounts.token_y_mint.clone(),
            accounts.receiver_token_x.clone(),
            accounts.receiver_token_y.clone(),
            accounts.token_x_program.clone(),
            accounts.token_y_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; WITHDRAW_PROTOCOL_FEE_IX_ACCOUNTS_LEN]>
for WithdrawProtocolFeeAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; WITHDRAW_PROTOCOL_FEE_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            lb_pair: &arr[0],
            reserve_x: &arr[1],
            reserve_y: &arr[2],
            token_x_mint: &arr[3],
            token_y_mint: &arr[4],
            receiver_token_x: &arr[5],
            receiver_token_y: &arr[6],
            token_x_program: &arr[7],
            token_y_program: &arr[8],
        }
    }
}
pub const WITHDRAW_PROTOCOL_FEE_IX_DISCM: [u8; 8] = [
    158,
    201,
    158,
    189,
    33,
    93,
    162,
    103,
];
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct WithdrawProtocolFeeIxArgs {
    pub amount_x: u64,
    pub amount_y: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct WithdrawProtocolFeeIxData(pub WithdrawProtocolFeeIxArgs);
impl From<WithdrawProtocolFeeIxArgs> for WithdrawProtocolFeeIxData {
    fn from(args: WithdrawProtocolFeeIxArgs) -> Self {
        Self(args)
    }
}
impl WithdrawProtocolFeeIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != WITHDRAW_PROTOCOL_FEE_IX_DISCM {
            return Err(
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!(
                        "discm does not match. Expected: {:?}. Received: {:?}",
                        WITHDRAW_PROTOCOL_FEE_IX_DISCM, maybe_discm
                    ),
                ),
            );
        }
        Ok(Self(WithdrawProtocolFeeIxArgs::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&WITHDRAW_PROTOCOL_FEE_IX_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn withdraw_protocol_fee_ix_with_program_id(
    program_id: Pubkey,
    keys: WithdrawProtocolFeeKeys,
    args: WithdrawProtocolFeeIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; WITHDRAW_PROTOCOL_FEE_IX_ACCOUNTS_LEN] = keys.into();
    let data: WithdrawProtocolFeeIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn withdraw_protocol_fee_ix(
    keys: WithdrawProtocolFeeKeys,
    args: WithdrawProtocolFeeIxArgs,
) -> std::io::Result<Instruction> {
    withdraw_protocol_fee_ix_with_program_id(crate::ID, keys, args)
}
pub fn withdraw_protocol_fee_invoke_with_program_id(
    program_id: Pubkey,
    accounts: WithdrawProtocolFeeAccounts<'_, '_>,
    args: WithdrawProtocolFeeIxArgs,
) -> ProgramResult {
    let keys: WithdrawProtocolFeeKeys = accounts.into();
    let ix = withdraw_protocol_fee_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn withdraw_protocol_fee_invoke(
    accounts: WithdrawProtocolFeeAccounts<'_, '_>,
    args: WithdrawProtocolFeeIxArgs,
) -> ProgramResult {
    withdraw_protocol_fee_invoke_with_program_id(crate::ID, accounts, args)
}
pub fn withdraw_protocol_fee_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: WithdrawProtocolFeeAccounts<'_, '_>,
    args: WithdrawProtocolFeeIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: WithdrawProtocolFeeKeys = accounts.into();
    let ix = withdraw_protocol_fee_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn withdraw_protocol_fee_invoke_signed(
    accounts: WithdrawProtocolFeeAccounts<'_, '_>,
    args: WithdrawProtocolFeeIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    withdraw_protocol_fee_invoke_signed_with_program_id(crate::ID, accounts, args, seeds)
}
pub fn withdraw_protocol_fee_verify_account_keys(
    accounts: WithdrawProtocolFeeAccounts<'_, '_>,
    keys: WithdrawProtocolFeeKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.lb_pair.key, keys.lb_pair),
        (*accounts.reserve_x.key, keys.reserve_x),
        (*accounts.reserve_y.key, keys.reserve_y),
        (*accounts.token_x_mint.key, keys.token_x_mint),
        (*accounts.token_y_mint.key, keys.token_y_mint),
        (*accounts.receiver_token_x.key, keys.receiver_token_x),
        (*accounts.receiver_token_y.key, keys.receiver_token_y),
        (*accounts.token_x_program.key, keys.token_x_program),
        (*accounts.token_y_program.key, keys.token_y_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn withdraw_protocol_fee_verify_writable_privileges<'me, 'info>(
    accounts: WithdrawProtocolFeeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.lb_pair,
        accounts.reserve_x,
        accounts.reserve_y,
        accounts.receiver_token_x,
        accounts.receiver_token_y,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn withdraw_protocol_fee_verify_account_privileges<'me, 'info>(
    accounts: WithdrawProtocolFeeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    withdraw_protocol_fee_verify_writable_privileges(accounts)?;
    Ok(())
}
pub const INITIALIZE_REWARD_IX_ACCOUNTS_LEN: usize = 9;
#[derive(Copy, Clone, Debug)]
pub struct InitializeRewardAccounts<'me, 'info> {
    pub lb_pair: &'me AccountInfo<'info>,
    pub reward_vault: &'me AccountInfo<'info>,
    pub reward_mint: &'me AccountInfo<'info>,
    pub admin: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub rent: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct InitializeRewardKeys {
    pub lb_pair: Pubkey,
    pub reward_vault: Pubkey,
    pub reward_mint: Pubkey,
    pub admin: Pubkey,
    pub token_program: Pubkey,
    pub system_program: Pubkey,
    pub rent: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<InitializeRewardAccounts<'_, '_>> for InitializeRewardKeys {
    fn from(accounts: InitializeRewardAccounts) -> Self {
        Self {
            lb_pair: *accounts.lb_pair.key,
            reward_vault: *accounts.reward_vault.key,
            reward_mint: *accounts.reward_mint.key,
            admin: *accounts.admin.key,
            token_program: *accounts.token_program.key,
            system_program: *accounts.system_program.key,
            rent: *accounts.rent.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<InitializeRewardKeys> for [AccountMeta; INITIALIZE_REWARD_IX_ACCOUNTS_LEN] {
    fn from(keys: InitializeRewardKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.lb_pair,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.reward_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.reward_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.admin,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.system_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.rent,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.event_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; INITIALIZE_REWARD_IX_ACCOUNTS_LEN]> for InitializeRewardKeys {
    fn from(pubkeys: [Pubkey; INITIALIZE_REWARD_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            lb_pair: pubkeys[0],
            reward_vault: pubkeys[1],
            reward_mint: pubkeys[2],
            admin: pubkeys[3],
            token_program: pubkeys[4],
            system_program: pubkeys[5],
            rent: pubkeys[6],
            event_authority: pubkeys[7],
            program: pubkeys[8],
        }
    }
}
impl<'info> From<InitializeRewardAccounts<'_, 'info>>
for [AccountInfo<'info>; INITIALIZE_REWARD_IX_ACCOUNTS_LEN] {
    fn from(accounts: InitializeRewardAccounts<'_, 'info>) -> Self {
        [
            accounts.lb_pair.clone(),
            accounts.reward_vault.clone(),
            accounts.reward_mint.clone(),
            accounts.admin.clone(),
            accounts.token_program.clone(),
            accounts.system_program.clone(),
            accounts.rent.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; INITIALIZE_REWARD_IX_ACCOUNTS_LEN]>
for InitializeRewardAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; INITIALIZE_REWARD_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            lb_pair: &arr[0],
            reward_vault: &arr[1],
            reward_mint: &arr[2],
            admin: &arr[3],
            token_program: &arr[4],
            system_program: &arr[5],
            rent: &arr[6],
            event_authority: &arr[7],
            program: &arr[8],
        }
    }
}
pub const INITIALIZE_REWARD_IX_DISCM: [u8; 8] = [95, 135, 192, 196, 242, 129, 230, 68];
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InitializeRewardIxArgs {
    pub reward_index: u64,
    pub reward_duration: u64,
    pub funder: Pubkey,
}
#[derive(Clone, Debug, PartialEq)]
pub struct InitializeRewardIxData(pub InitializeRewardIxArgs);
impl From<InitializeRewardIxArgs> for InitializeRewardIxData {
    fn from(args: InitializeRewardIxArgs) -> Self {
        Self(args)
    }
}
impl InitializeRewardIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != INITIALIZE_REWARD_IX_DISCM {
            return Err(
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!(
                        "discm does not match. Expected: {:?}. Received: {:?}",
                        INITIALIZE_REWARD_IX_DISCM, maybe_discm
                    ),
                ),
            );
        }
        Ok(Self(InitializeRewardIxArgs::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&INITIALIZE_REWARD_IX_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn initialize_reward_ix_with_program_id(
    program_id: Pubkey,
    keys: InitializeRewardKeys,
    args: InitializeRewardIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; INITIALIZE_REWARD_IX_ACCOUNTS_LEN] = keys.into();
    let data: InitializeRewardIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn initialize_reward_ix(
    keys: InitializeRewardKeys,
    args: InitializeRewardIxArgs,
) -> std::io::Result<Instruction> {
    initialize_reward_ix_with_program_id(crate::ID, keys, args)
}
pub fn initialize_reward_invoke_with_program_id(
    program_id: Pubkey,
    accounts: InitializeRewardAccounts<'_, '_>,
    args: InitializeRewardIxArgs,
) -> ProgramResult {
    let keys: InitializeRewardKeys = accounts.into();
    let ix = initialize_reward_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn initialize_reward_invoke(
    accounts: InitializeRewardAccounts<'_, '_>,
    args: InitializeRewardIxArgs,
) -> ProgramResult {
    initialize_reward_invoke_with_program_id(crate::ID, accounts, args)
}
pub fn initialize_reward_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: InitializeRewardAccounts<'_, '_>,
    args: InitializeRewardIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: InitializeRewardKeys = accounts.into();
    let ix = initialize_reward_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn initialize_reward_invoke_signed(
    accounts: InitializeRewardAccounts<'_, '_>,
    args: InitializeRewardIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    initialize_reward_invoke_signed_with_program_id(crate::ID, accounts, args, seeds)
}
pub fn initialize_reward_verify_account_keys(
    accounts: InitializeRewardAccounts<'_, '_>,
    keys: InitializeRewardKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.lb_pair.key, keys.lb_pair),
        (*accounts.reward_vault.key, keys.reward_vault),
        (*accounts.reward_mint.key, keys.reward_mint),
        (*accounts.admin.key, keys.admin),
        (*accounts.token_program.key, keys.token_program),
        (*accounts.system_program.key, keys.system_program),
        (*accounts.rent.key, keys.rent),
        (*accounts.event_authority.key, keys.event_authority),
        (*accounts.program.key, keys.program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn initialize_reward_verify_writable_privileges<'me, 'info>(
    accounts: InitializeRewardAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.lb_pair, accounts.reward_vault, accounts.admin] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn initialize_reward_verify_signer_privileges<'me, 'info>(
    accounts: InitializeRewardAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.admin] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn initialize_reward_verify_account_privileges<'me, 'info>(
    accounts: InitializeRewardAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    initialize_reward_verify_writable_privileges(accounts)?;
    initialize_reward_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const FUND_REWARD_IX_ACCOUNTS_LEN: usize = 9;
#[derive(Copy, Clone, Debug)]
pub struct FundRewardAccounts<'me, 'info> {
    pub lb_pair: &'me AccountInfo<'info>,
    pub reward_vault: &'me AccountInfo<'info>,
    pub reward_mint: &'me AccountInfo<'info>,
    pub funder_token_account: &'me AccountInfo<'info>,
    pub funder: &'me AccountInfo<'info>,
    pub bin_array: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct FundRewardKeys {
    pub lb_pair: Pubkey,
    pub reward_vault: Pubkey,
    pub reward_mint: Pubkey,
    pub funder_token_account: Pubkey,
    pub funder: Pubkey,
    pub bin_array: Pubkey,
    pub token_program: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<FundRewardAccounts<'_, '_>> for FundRewardKeys {
    fn from(accounts: FundRewardAccounts) -> Self {
        Self {
            lb_pair: *accounts.lb_pair.key,
            reward_vault: *accounts.reward_vault.key,
            reward_mint: *accounts.reward_mint.key,
            funder_token_account: *accounts.funder_token_account.key,
            funder: *accounts.funder.key,
            bin_array: *accounts.bin_array.key,
            token_program: *accounts.token_program.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<FundRewardKeys> for [AccountMeta; FUND_REWARD_IX_ACCOUNTS_LEN] {
    fn from(keys: FundRewardKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.lb_pair,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.reward_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.reward_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.funder_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.funder,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.bin_array,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.event_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; FUND_REWARD_IX_ACCOUNTS_LEN]> for FundRewardKeys {
    fn from(pubkeys: [Pubkey; FUND_REWARD_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            lb_pair: pubkeys[0],
            reward_vault: pubkeys[1],
            reward_mint: pubkeys[2],
            funder_token_account: pubkeys[3],
            funder: pubkeys[4],
            bin_array: pubkeys[5],
            token_program: pubkeys[6],
            event_authority: pubkeys[7],
            program: pubkeys[8],
        }
    }
}
impl<'info> From<FundRewardAccounts<'_, 'info>>
for [AccountInfo<'info>; FUND_REWARD_IX_ACCOUNTS_LEN] {
    fn from(accounts: FundRewardAccounts<'_, 'info>) -> Self {
        [
            accounts.lb_pair.clone(),
            accounts.reward_vault.clone(),
            accounts.reward_mint.clone(),
            accounts.funder_token_account.clone(),
            accounts.funder.clone(),
            accounts.bin_array.clone(),
            accounts.token_program.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; FUND_REWARD_IX_ACCOUNTS_LEN]>
for FundRewardAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; FUND_REWARD_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            lb_pair: &arr[0],
            reward_vault: &arr[1],
            reward_mint: &arr[2],
            funder_token_account: &arr[3],
            funder: &arr[4],
            bin_array: &arr[5],
            token_program: &arr[6],
            event_authority: &arr[7],
            program: &arr[8],
        }
    }
}
pub const FUND_REWARD_IX_DISCM: [u8; 8] = [188, 50, 249, 165, 93, 151, 38, 63];
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FundRewardIxArgs {
    pub reward_index: u64,
    pub amount: u64,
    pub carry_forward: bool,
}
#[derive(Clone, Debug, PartialEq)]
pub struct FundRewardIxData(pub FundRewardIxArgs);
impl From<FundRewardIxArgs> for FundRewardIxData {
    fn from(args: FundRewardIxArgs) -> Self {
        Self(args)
    }
}
impl FundRewardIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != FUND_REWARD_IX_DISCM {
            return Err(
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!(
                        "discm does not match. Expected: {:?}. Received: {:?}",
                        FUND_REWARD_IX_DISCM, maybe_discm
                    ),
                ),
            );
        }
        Ok(Self(FundRewardIxArgs::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&FUND_REWARD_IX_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn fund_reward_ix_with_program_id(
    program_id: Pubkey,
    keys: FundRewardKeys,
    args: FundRewardIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; FUND_REWARD_IX_ACCOUNTS_LEN] = keys.into();
    let data: FundRewardIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn fund_reward_ix(
    keys: FundRewardKeys,
    args: FundRewardIxArgs,
) -> std::io::Result<Instruction> {
    fund_reward_ix_with_program_id(crate::ID, keys, args)
}
pub fn fund_reward_invoke_with_program_id(
    program_id: Pubkey,
    accounts: FundRewardAccounts<'_, '_>,
    args: FundRewardIxArgs,
) -> ProgramResult {
    let keys: FundRewardKeys = accounts.into();
    let ix = fund_reward_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn fund_reward_invoke(
    accounts: FundRewardAccounts<'_, '_>,
    args: FundRewardIxArgs,
) -> ProgramResult {
    fund_reward_invoke_with_program_id(crate::ID, accounts, args)
}
pub fn fund_reward_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: FundRewardAccounts<'_, '_>,
    args: FundRewardIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: FundRewardKeys = accounts.into();
    let ix = fund_reward_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn fund_reward_invoke_signed(
    accounts: FundRewardAccounts<'_, '_>,
    args: FundRewardIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    fund_reward_invoke_signed_with_program_id(crate::ID, accounts, args, seeds)
}
pub fn fund_reward_verify_account_keys(
    accounts: FundRewardAccounts<'_, '_>,
    keys: FundRewardKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.lb_pair.key, keys.lb_pair),
        (*accounts.reward_vault.key, keys.reward_vault),
        (*accounts.reward_mint.key, keys.reward_mint),
        (*accounts.funder_token_account.key, keys.funder_token_account),
        (*accounts.funder.key, keys.funder),
        (*accounts.bin_array.key, keys.bin_array),
        (*accounts.token_program.key, keys.token_program),
        (*accounts.event_authority.key, keys.event_authority),
        (*accounts.program.key, keys.program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn fund_reward_verify_writable_privileges<'me, 'info>(
    accounts: FundRewardAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.lb_pair,
        accounts.reward_vault,
        accounts.funder_token_account,
        accounts.bin_array,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn fund_reward_verify_signer_privileges<'me, 'info>(
    accounts: FundRewardAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.funder] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn fund_reward_verify_account_privileges<'me, 'info>(
    accounts: FundRewardAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    fund_reward_verify_writable_privileges(accounts)?;
    fund_reward_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const UPDATE_REWARD_FUNDER_IX_ACCOUNTS_LEN: usize = 4;
#[derive(Copy, Clone, Debug)]
pub struct UpdateRewardFunderAccounts<'me, 'info> {
    pub lb_pair: &'me AccountInfo<'info>,
    pub admin: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct UpdateRewardFunderKeys {
    pub lb_pair: Pubkey,
    pub admin: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<UpdateRewardFunderAccounts<'_, '_>> for UpdateRewardFunderKeys {
    fn from(accounts: UpdateRewardFunderAccounts) -> Self {
        Self {
            lb_pair: *accounts.lb_pair.key,
            admin: *accounts.admin.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<UpdateRewardFunderKeys>
for [AccountMeta; UPDATE_REWARD_FUNDER_IX_ACCOUNTS_LEN] {
    fn from(keys: UpdateRewardFunderKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.lb_pair,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.admin,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.event_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; UPDATE_REWARD_FUNDER_IX_ACCOUNTS_LEN]> for UpdateRewardFunderKeys {
    fn from(pubkeys: [Pubkey; UPDATE_REWARD_FUNDER_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            lb_pair: pubkeys[0],
            admin: pubkeys[1],
            event_authority: pubkeys[2],
            program: pubkeys[3],
        }
    }
}
impl<'info> From<UpdateRewardFunderAccounts<'_, 'info>>
for [AccountInfo<'info>; UPDATE_REWARD_FUNDER_IX_ACCOUNTS_LEN] {
    fn from(accounts: UpdateRewardFunderAccounts<'_, 'info>) -> Self {
        [
            accounts.lb_pair.clone(),
            accounts.admin.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; UPDATE_REWARD_FUNDER_IX_ACCOUNTS_LEN]>
for UpdateRewardFunderAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; UPDATE_REWARD_FUNDER_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            lb_pair: &arr[0],
            admin: &arr[1],
            event_authority: &arr[2],
            program: &arr[3],
        }
    }
}
pub const UPDATE_REWARD_FUNDER_IX_DISCM: [u8; 8] = [211, 28, 48, 32, 215, 160, 35, 23];
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct UpdateRewardFunderIxArgs {
    pub reward_index: u64,
    pub new_funder: Pubkey,
}
#[derive(Clone, Debug, PartialEq)]
pub struct UpdateRewardFunderIxData(pub UpdateRewardFunderIxArgs);
impl From<UpdateRewardFunderIxArgs> for UpdateRewardFunderIxData {
    fn from(args: UpdateRewardFunderIxArgs) -> Self {
        Self(args)
    }
}
impl UpdateRewardFunderIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != UPDATE_REWARD_FUNDER_IX_DISCM {
            return Err(
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!(
                        "discm does not match. Expected: {:?}. Received: {:?}",
                        UPDATE_REWARD_FUNDER_IX_DISCM, maybe_discm
                    ),
                ),
            );
        }
        Ok(Self(UpdateRewardFunderIxArgs::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&UPDATE_REWARD_FUNDER_IX_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn update_reward_funder_ix_with_program_id(
    program_id: Pubkey,
    keys: UpdateRewardFunderKeys,
    args: UpdateRewardFunderIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; UPDATE_REWARD_FUNDER_IX_ACCOUNTS_LEN] = keys.into();
    let data: UpdateRewardFunderIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn update_reward_funder_ix(
    keys: UpdateRewardFunderKeys,
    args: UpdateRewardFunderIxArgs,
) -> std::io::Result<Instruction> {
    update_reward_funder_ix_with_program_id(crate::ID, keys, args)
}
pub fn update_reward_funder_invoke_with_program_id(
    program_id: Pubkey,
    accounts: UpdateRewardFunderAccounts<'_, '_>,
    args: UpdateRewardFunderIxArgs,
) -> ProgramResult {
    let keys: UpdateRewardFunderKeys = accounts.into();
    let ix = update_reward_funder_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn update_reward_funder_invoke(
    accounts: UpdateRewardFunderAccounts<'_, '_>,
    args: UpdateRewardFunderIxArgs,
) -> ProgramResult {
    update_reward_funder_invoke_with_program_id(crate::ID, accounts, args)
}
pub fn update_reward_funder_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: UpdateRewardFunderAccounts<'_, '_>,
    args: UpdateRewardFunderIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: UpdateRewardFunderKeys = accounts.into();
    let ix = update_reward_funder_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn update_reward_funder_invoke_signed(
    accounts: UpdateRewardFunderAccounts<'_, '_>,
    args: UpdateRewardFunderIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    update_reward_funder_invoke_signed_with_program_id(crate::ID, accounts, args, seeds)
}
pub fn update_reward_funder_verify_account_keys(
    accounts: UpdateRewardFunderAccounts<'_, '_>,
    keys: UpdateRewardFunderKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.lb_pair.key, keys.lb_pair),
        (*accounts.admin.key, keys.admin),
        (*accounts.event_authority.key, keys.event_authority),
        (*accounts.program.key, keys.program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn update_reward_funder_verify_writable_privileges<'me, 'info>(
    accounts: UpdateRewardFunderAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.lb_pair] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn update_reward_funder_verify_signer_privileges<'me, 'info>(
    accounts: UpdateRewardFunderAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.admin] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn update_reward_funder_verify_account_privileges<'me, 'info>(
    accounts: UpdateRewardFunderAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    update_reward_funder_verify_writable_privileges(accounts)?;
    update_reward_funder_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const UPDATE_REWARD_DURATION_IX_ACCOUNTS_LEN: usize = 5;
#[derive(Copy, Clone, Debug)]
pub struct UpdateRewardDurationAccounts<'me, 'info> {
    pub lb_pair: &'me AccountInfo<'info>,
    pub admin: &'me AccountInfo<'info>,
    pub bin_array: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct UpdateRewardDurationKeys {
    pub lb_pair: Pubkey,
    pub admin: Pubkey,
    pub bin_array: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<UpdateRewardDurationAccounts<'_, '_>> for UpdateRewardDurationKeys {
    fn from(accounts: UpdateRewardDurationAccounts) -> Self {
        Self {
            lb_pair: *accounts.lb_pair.key,
            admin: *accounts.admin.key,
            bin_array: *accounts.bin_array.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<UpdateRewardDurationKeys>
for [AccountMeta; UPDATE_REWARD_DURATION_IX_ACCOUNTS_LEN] {
    fn from(keys: UpdateRewardDurationKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.lb_pair,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.admin,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.bin_array,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.event_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; UPDATE_REWARD_DURATION_IX_ACCOUNTS_LEN]>
for UpdateRewardDurationKeys {
    fn from(pubkeys: [Pubkey; UPDATE_REWARD_DURATION_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            lb_pair: pubkeys[0],
            admin: pubkeys[1],
            bin_array: pubkeys[2],
            event_authority: pubkeys[3],
            program: pubkeys[4],
        }
    }
}
impl<'info> From<UpdateRewardDurationAccounts<'_, 'info>>
for [AccountInfo<'info>; UPDATE_REWARD_DURATION_IX_ACCOUNTS_LEN] {
    fn from(accounts: UpdateRewardDurationAccounts<'_, 'info>) -> Self {
        [
            accounts.lb_pair.clone(),
            accounts.admin.clone(),
            accounts.bin_array.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; UPDATE_REWARD_DURATION_IX_ACCOUNTS_LEN]>
for UpdateRewardDurationAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; UPDATE_REWARD_DURATION_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            lb_pair: &arr[0],
            admin: &arr[1],
            bin_array: &arr[2],
            event_authority: &arr[3],
            program: &arr[4],
        }
    }
}
pub const UPDATE_REWARD_DURATION_IX_DISCM: [u8; 8] = [
    138,
    174,
    196,
    169,
    213,
    235,
    254,
    107,
];
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct UpdateRewardDurationIxArgs {
    pub reward_index: u64,
    pub new_duration: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct UpdateRewardDurationIxData(pub UpdateRewardDurationIxArgs);
impl From<UpdateRewardDurationIxArgs> for UpdateRewardDurationIxData {
    fn from(args: UpdateRewardDurationIxArgs) -> Self {
        Self(args)
    }
}
impl UpdateRewardDurationIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != UPDATE_REWARD_DURATION_IX_DISCM {
            return Err(
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!(
                        "discm does not match. Expected: {:?}. Received: {:?}",
                        UPDATE_REWARD_DURATION_IX_DISCM, maybe_discm
                    ),
                ),
            );
        }
        Ok(Self(UpdateRewardDurationIxArgs::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&UPDATE_REWARD_DURATION_IX_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn update_reward_duration_ix_with_program_id(
    program_id: Pubkey,
    keys: UpdateRewardDurationKeys,
    args: UpdateRewardDurationIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; UPDATE_REWARD_DURATION_IX_ACCOUNTS_LEN] = keys.into();
    let data: UpdateRewardDurationIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn update_reward_duration_ix(
    keys: UpdateRewardDurationKeys,
    args: UpdateRewardDurationIxArgs,
) -> std::io::Result<Instruction> {
    update_reward_duration_ix_with_program_id(crate::ID, keys, args)
}
pub fn update_reward_duration_invoke_with_program_id(
    program_id: Pubkey,
    accounts: UpdateRewardDurationAccounts<'_, '_>,
    args: UpdateRewardDurationIxArgs,
) -> ProgramResult {
    let keys: UpdateRewardDurationKeys = accounts.into();
    let ix = update_reward_duration_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn update_reward_duration_invoke(
    accounts: UpdateRewardDurationAccounts<'_, '_>,
    args: UpdateRewardDurationIxArgs,
) -> ProgramResult {
    update_reward_duration_invoke_with_program_id(crate::ID, accounts, args)
}
pub fn update_reward_duration_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: UpdateRewardDurationAccounts<'_, '_>,
    args: UpdateRewardDurationIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: UpdateRewardDurationKeys = accounts.into();
    let ix = update_reward_duration_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn update_reward_duration_invoke_signed(
    accounts: UpdateRewardDurationAccounts<'_, '_>,
    args: UpdateRewardDurationIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    update_reward_duration_invoke_signed_with_program_id(
        crate::ID,
        accounts,
        args,
        seeds,
    )
}
pub fn update_reward_duration_verify_account_keys(
    accounts: UpdateRewardDurationAccounts<'_, '_>,
    keys: UpdateRewardDurationKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.lb_pair.key, keys.lb_pair),
        (*accounts.admin.key, keys.admin),
        (*accounts.bin_array.key, keys.bin_array),
        (*accounts.event_authority.key, keys.event_authority),
        (*accounts.program.key, keys.program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn update_reward_duration_verify_writable_privileges<'me, 'info>(
    accounts: UpdateRewardDurationAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.lb_pair, accounts.bin_array] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn update_reward_duration_verify_signer_privileges<'me, 'info>(
    accounts: UpdateRewardDurationAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.admin] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn update_reward_duration_verify_account_privileges<'me, 'info>(
    accounts: UpdateRewardDurationAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    update_reward_duration_verify_writable_privileges(accounts)?;
    update_reward_duration_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const CLAIM_REWARD_IX_ACCOUNTS_LEN: usize = 11;
#[derive(Copy, Clone, Debug)]
pub struct ClaimRewardAccounts<'me, 'info> {
    pub lb_pair: &'me AccountInfo<'info>,
    pub position: &'me AccountInfo<'info>,
    pub bin_array_lower: &'me AccountInfo<'info>,
    pub bin_array_upper: &'me AccountInfo<'info>,
    pub sender: &'me AccountInfo<'info>,
    pub reward_vault: &'me AccountInfo<'info>,
    pub reward_mint: &'me AccountInfo<'info>,
    pub user_token_account: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct ClaimRewardKeys {
    pub lb_pair: Pubkey,
    pub position: Pubkey,
    pub bin_array_lower: Pubkey,
    pub bin_array_upper: Pubkey,
    pub sender: Pubkey,
    pub reward_vault: Pubkey,
    pub reward_mint: Pubkey,
    pub user_token_account: Pubkey,
    pub token_program: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<ClaimRewardAccounts<'_, '_>> for ClaimRewardKeys {
    fn from(accounts: ClaimRewardAccounts) -> Self {
        Self {
            lb_pair: *accounts.lb_pair.key,
            position: *accounts.position.key,
            bin_array_lower: *accounts.bin_array_lower.key,
            bin_array_upper: *accounts.bin_array_upper.key,
            sender: *accounts.sender.key,
            reward_vault: *accounts.reward_vault.key,
            reward_mint: *accounts.reward_mint.key,
            user_token_account: *accounts.user_token_account.key,
            token_program: *accounts.token_program.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<ClaimRewardKeys> for [AccountMeta; CLAIM_REWARD_IX_ACCOUNTS_LEN] {
    fn from(keys: ClaimRewardKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.lb_pair,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.position,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.bin_array_lower,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.bin_array_upper,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.sender,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.reward_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.reward_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.user_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.event_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; CLAIM_REWARD_IX_ACCOUNTS_LEN]> for ClaimRewardKeys {
    fn from(pubkeys: [Pubkey; CLAIM_REWARD_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            lb_pair: pubkeys[0],
            position: pubkeys[1],
            bin_array_lower: pubkeys[2],
            bin_array_upper: pubkeys[3],
            sender: pubkeys[4],
            reward_vault: pubkeys[5],
            reward_mint: pubkeys[6],
            user_token_account: pubkeys[7],
            token_program: pubkeys[8],
            event_authority: pubkeys[9],
            program: pubkeys[10],
        }
    }
}
impl<'info> From<ClaimRewardAccounts<'_, 'info>>
for [AccountInfo<'info>; CLAIM_REWARD_IX_ACCOUNTS_LEN] {
    fn from(accounts: ClaimRewardAccounts<'_, 'info>) -> Self {
        [
            accounts.lb_pair.clone(),
            accounts.position.clone(),
            accounts.bin_array_lower.clone(),
            accounts.bin_array_upper.clone(),
            accounts.sender.clone(),
            accounts.reward_vault.clone(),
            accounts.reward_mint.clone(),
            accounts.user_token_account.clone(),
            accounts.token_program.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; CLAIM_REWARD_IX_ACCOUNTS_LEN]>
for ClaimRewardAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; CLAIM_REWARD_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            lb_pair: &arr[0],
            position: &arr[1],
            bin_array_lower: &arr[2],
            bin_array_upper: &arr[3],
            sender: &arr[4],
            reward_vault: &arr[5],
            reward_mint: &arr[6],
            user_token_account: &arr[7],
            token_program: &arr[8],
            event_authority: &arr[9],
            program: &arr[10],
        }
    }
}
pub const CLAIM_REWARD_IX_DISCM: [u8; 8] = [149, 95, 181, 242, 94, 90, 158, 162];
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ClaimRewardIxArgs {
    pub reward_index: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct ClaimRewardIxData(pub ClaimRewardIxArgs);
impl From<ClaimRewardIxArgs> for ClaimRewardIxData {
    fn from(args: ClaimRewardIxArgs) -> Self {
        Self(args)
    }
}
impl ClaimRewardIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != CLAIM_REWARD_IX_DISCM {
            return Err(
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!(
                        "discm does not match. Expected: {:?}. Received: {:?}",
                        CLAIM_REWARD_IX_DISCM, maybe_discm
                    ),
                ),
            );
        }
        Ok(Self(ClaimRewardIxArgs::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&CLAIM_REWARD_IX_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn claim_reward_ix_with_program_id(
    program_id: Pubkey,
    keys: ClaimRewardKeys,
    args: ClaimRewardIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; CLAIM_REWARD_IX_ACCOUNTS_LEN] = keys.into();
    let data: ClaimRewardIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn claim_reward_ix(
    keys: ClaimRewardKeys,
    args: ClaimRewardIxArgs,
) -> std::io::Result<Instruction> {
    claim_reward_ix_with_program_id(crate::ID, keys, args)
}
pub fn claim_reward_invoke_with_program_id(
    program_id: Pubkey,
    accounts: ClaimRewardAccounts<'_, '_>,
    args: ClaimRewardIxArgs,
) -> ProgramResult {
    let keys: ClaimRewardKeys = accounts.into();
    let ix = claim_reward_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn claim_reward_invoke(
    accounts: ClaimRewardAccounts<'_, '_>,
    args: ClaimRewardIxArgs,
) -> ProgramResult {
    claim_reward_invoke_with_program_id(crate::ID, accounts, args)
}
pub fn claim_reward_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: ClaimRewardAccounts<'_, '_>,
    args: ClaimRewardIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: ClaimRewardKeys = accounts.into();
    let ix = claim_reward_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn claim_reward_invoke_signed(
    accounts: ClaimRewardAccounts<'_, '_>,
    args: ClaimRewardIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    claim_reward_invoke_signed_with_program_id(crate::ID, accounts, args, seeds)
}
pub fn claim_reward_verify_account_keys(
    accounts: ClaimRewardAccounts<'_, '_>,
    keys: ClaimRewardKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.lb_pair.key, keys.lb_pair),
        (*accounts.position.key, keys.position),
        (*accounts.bin_array_lower.key, keys.bin_array_lower),
        (*accounts.bin_array_upper.key, keys.bin_array_upper),
        (*accounts.sender.key, keys.sender),
        (*accounts.reward_vault.key, keys.reward_vault),
        (*accounts.reward_mint.key, keys.reward_mint),
        (*accounts.user_token_account.key, keys.user_token_account),
        (*accounts.token_program.key, keys.token_program),
        (*accounts.event_authority.key, keys.event_authority),
        (*accounts.program.key, keys.program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn claim_reward_verify_writable_privileges<'me, 'info>(
    accounts: ClaimRewardAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.lb_pair,
        accounts.position,
        accounts.bin_array_lower,
        accounts.bin_array_upper,
        accounts.reward_vault,
        accounts.user_token_account,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn claim_reward_verify_signer_privileges<'me, 'info>(
    accounts: ClaimRewardAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.sender] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn claim_reward_verify_account_privileges<'me, 'info>(
    accounts: ClaimRewardAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    claim_reward_verify_writable_privileges(accounts)?;
    claim_reward_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const CLAIM_FEE_IX_ACCOUNTS_LEN: usize = 14;
#[derive(Copy, Clone, Debug)]
pub struct ClaimFeeAccounts<'me, 'info> {
    pub lb_pair: &'me AccountInfo<'info>,
    pub position: &'me AccountInfo<'info>,
    pub bin_array_lower: &'me AccountInfo<'info>,
    pub bin_array_upper: &'me AccountInfo<'info>,
    pub sender: &'me AccountInfo<'info>,
    pub reserve_x: &'me AccountInfo<'info>,
    pub reserve_y: &'me AccountInfo<'info>,
    pub user_token_x: &'me AccountInfo<'info>,
    pub user_token_y: &'me AccountInfo<'info>,
    pub token_x_mint: &'me AccountInfo<'info>,
    pub token_y_mint: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct ClaimFeeKeys {
    pub lb_pair: Pubkey,
    pub position: Pubkey,
    pub bin_array_lower: Pubkey,
    pub bin_array_upper: Pubkey,
    pub sender: Pubkey,
    pub reserve_x: Pubkey,
    pub reserve_y: Pubkey,
    pub user_token_x: Pubkey,
    pub user_token_y: Pubkey,
    pub token_x_mint: Pubkey,
    pub token_y_mint: Pubkey,
    pub token_program: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<ClaimFeeAccounts<'_, '_>> for ClaimFeeKeys {
    fn from(accounts: ClaimFeeAccounts) -> Self {
        Self {
            lb_pair: *accounts.lb_pair.key,
            position: *accounts.position.key,
            bin_array_lower: *accounts.bin_array_lower.key,
            bin_array_upper: *accounts.bin_array_upper.key,
            sender: *accounts.sender.key,
            reserve_x: *accounts.reserve_x.key,
            reserve_y: *accounts.reserve_y.key,
            user_token_x: *accounts.user_token_x.key,
            user_token_y: *accounts.user_token_y.key,
            token_x_mint: *accounts.token_x_mint.key,
            token_y_mint: *accounts.token_y_mint.key,
            token_program: *accounts.token_program.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<ClaimFeeKeys> for [AccountMeta; CLAIM_FEE_IX_ACCOUNTS_LEN] {
    fn from(keys: ClaimFeeKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.lb_pair,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.position,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.bin_array_lower,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.bin_array_upper,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.sender,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.reserve_x,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.reserve_y,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_token_x,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_token_y,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_x_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_y_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.event_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; CLAIM_FEE_IX_ACCOUNTS_LEN]> for ClaimFeeKeys {
    fn from(pubkeys: [Pubkey; CLAIM_FEE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            lb_pair: pubkeys[0],
            position: pubkeys[1],
            bin_array_lower: pubkeys[2],
            bin_array_upper: pubkeys[3],
            sender: pubkeys[4],
            reserve_x: pubkeys[5],
            reserve_y: pubkeys[6],
            user_token_x: pubkeys[7],
            user_token_y: pubkeys[8],
            token_x_mint: pubkeys[9],
            token_y_mint: pubkeys[10],
            token_program: pubkeys[11],
            event_authority: pubkeys[12],
            program: pubkeys[13],
        }
    }
}
impl<'info> From<ClaimFeeAccounts<'_, 'info>>
for [AccountInfo<'info>; CLAIM_FEE_IX_ACCOUNTS_LEN] {
    fn from(accounts: ClaimFeeAccounts<'_, 'info>) -> Self {
        [
            accounts.lb_pair.clone(),
            accounts.position.clone(),
            accounts.bin_array_lower.clone(),
            accounts.bin_array_upper.clone(),
            accounts.sender.clone(),
            accounts.reserve_x.clone(),
            accounts.reserve_y.clone(),
            accounts.user_token_x.clone(),
            accounts.user_token_y.clone(),
            accounts.token_x_mint.clone(),
            accounts.token_y_mint.clone(),
            accounts.token_program.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; CLAIM_FEE_IX_ACCOUNTS_LEN]>
for ClaimFeeAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; CLAIM_FEE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            lb_pair: &arr[0],
            position: &arr[1],
            bin_array_lower: &arr[2],
            bin_array_upper: &arr[3],
            sender: &arr[4],
            reserve_x: &arr[5],
            reserve_y: &arr[6],
            user_token_x: &arr[7],
            user_token_y: &arr[8],
            token_x_mint: &arr[9],
            token_y_mint: &arr[10],
            token_program: &arr[11],
            event_authority: &arr[12],
            program: &arr[13],
        }
    }
}
pub const CLAIM_FEE_IX_DISCM: [u8; 8] = [169, 32, 79, 137, 136, 232, 70, 137];
#[derive(Clone, Debug, PartialEq)]
pub struct ClaimFeeIxData;
impl ClaimFeeIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != CLAIM_FEE_IX_DISCM {
            return Err(
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!(
                        "discm does not match. Expected: {:?}. Received: {:?}",
                        CLAIM_FEE_IX_DISCM, maybe_discm
                    ),
                ),
            );
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&CLAIM_FEE_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn claim_fee_ix_with_program_id(
    program_id: Pubkey,
    keys: ClaimFeeKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; CLAIM_FEE_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: ClaimFeeIxData.try_to_vec()?,
    })
}
pub fn claim_fee_ix(keys: ClaimFeeKeys) -> std::io::Result<Instruction> {
    claim_fee_ix_with_program_id(crate::ID, keys)
}
pub fn claim_fee_invoke_with_program_id(
    program_id: Pubkey,
    accounts: ClaimFeeAccounts<'_, '_>,
) -> ProgramResult {
    let keys: ClaimFeeKeys = accounts.into();
    let ix = claim_fee_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn claim_fee_invoke(accounts: ClaimFeeAccounts<'_, '_>) -> ProgramResult {
    claim_fee_invoke_with_program_id(crate::ID, accounts)
}
pub fn claim_fee_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: ClaimFeeAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: ClaimFeeKeys = accounts.into();
    let ix = claim_fee_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn claim_fee_invoke_signed(
    accounts: ClaimFeeAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    claim_fee_invoke_signed_with_program_id(crate::ID, accounts, seeds)
}
pub fn claim_fee_verify_account_keys(
    accounts: ClaimFeeAccounts<'_, '_>,
    keys: ClaimFeeKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.lb_pair.key, keys.lb_pair),
        (*accounts.position.key, keys.position),
        (*accounts.bin_array_lower.key, keys.bin_array_lower),
        (*accounts.bin_array_upper.key, keys.bin_array_upper),
        (*accounts.sender.key, keys.sender),
        (*accounts.reserve_x.key, keys.reserve_x),
        (*accounts.reserve_y.key, keys.reserve_y),
        (*accounts.user_token_x.key, keys.user_token_x),
        (*accounts.user_token_y.key, keys.user_token_y),
        (*accounts.token_x_mint.key, keys.token_x_mint),
        (*accounts.token_y_mint.key, keys.token_y_mint),
        (*accounts.token_program.key, keys.token_program),
        (*accounts.event_authority.key, keys.event_authority),
        (*accounts.program.key, keys.program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn claim_fee_verify_writable_privileges<'me, 'info>(
    accounts: ClaimFeeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.lb_pair,
        accounts.position,
        accounts.bin_array_lower,
        accounts.bin_array_upper,
        accounts.reserve_x,
        accounts.reserve_y,
        accounts.user_token_x,
        accounts.user_token_y,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn claim_fee_verify_signer_privileges<'me, 'info>(
    accounts: ClaimFeeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.sender] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn claim_fee_verify_account_privileges<'me, 'info>(
    accounts: ClaimFeeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    claim_fee_verify_writable_privileges(accounts)?;
    claim_fee_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const CLOSE_POSITION_IX_ACCOUNTS_LEN: usize = 8;
#[derive(Copy, Clone, Debug)]
pub struct ClosePositionAccounts<'me, 'info> {
    pub position: &'me AccountInfo<'info>,
    pub lb_pair: &'me AccountInfo<'info>,
    pub bin_array_lower: &'me AccountInfo<'info>,
    pub bin_array_upper: &'me AccountInfo<'info>,
    pub sender: &'me AccountInfo<'info>,
    pub rent_receiver: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct ClosePositionKeys {
    pub position: Pubkey,
    pub lb_pair: Pubkey,
    pub bin_array_lower: Pubkey,
    pub bin_array_upper: Pubkey,
    pub sender: Pubkey,
    pub rent_receiver: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<ClosePositionAccounts<'_, '_>> for ClosePositionKeys {
    fn from(accounts: ClosePositionAccounts) -> Self {
        Self {
            position: *accounts.position.key,
            lb_pair: *accounts.lb_pair.key,
            bin_array_lower: *accounts.bin_array_lower.key,
            bin_array_upper: *accounts.bin_array_upper.key,
            sender: *accounts.sender.key,
            rent_receiver: *accounts.rent_receiver.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<ClosePositionKeys> for [AccountMeta; CLOSE_POSITION_IX_ACCOUNTS_LEN] {
    fn from(keys: ClosePositionKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.position,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.lb_pair,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.bin_array_lower,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.bin_array_upper,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.sender,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.rent_receiver,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.event_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; CLOSE_POSITION_IX_ACCOUNTS_LEN]> for ClosePositionKeys {
    fn from(pubkeys: [Pubkey; CLOSE_POSITION_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            position: pubkeys[0],
            lb_pair: pubkeys[1],
            bin_array_lower: pubkeys[2],
            bin_array_upper: pubkeys[3],
            sender: pubkeys[4],
            rent_receiver: pubkeys[5],
            event_authority: pubkeys[6],
            program: pubkeys[7],
        }
    }
}
impl<'info> From<ClosePositionAccounts<'_, 'info>>
for [AccountInfo<'info>; CLOSE_POSITION_IX_ACCOUNTS_LEN] {
    fn from(accounts: ClosePositionAccounts<'_, 'info>) -> Self {
        [
            accounts.position.clone(),
            accounts.lb_pair.clone(),
            accounts.bin_array_lower.clone(),
            accounts.bin_array_upper.clone(),
            accounts.sender.clone(),
            accounts.rent_receiver.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; CLOSE_POSITION_IX_ACCOUNTS_LEN]>
for ClosePositionAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; CLOSE_POSITION_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            position: &arr[0],
            lb_pair: &arr[1],
            bin_array_lower: &arr[2],
            bin_array_upper: &arr[3],
            sender: &arr[4],
            rent_receiver: &arr[5],
            event_authority: &arr[6],
            program: &arr[7],
        }
    }
}
pub const CLOSE_POSITION_IX_DISCM: [u8; 8] = [123, 134, 81, 0, 49, 68, 98, 98];
#[derive(Clone, Debug, PartialEq)]
pub struct ClosePositionIxData;
impl ClosePositionIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != CLOSE_POSITION_IX_DISCM {
            return Err(
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!(
                        "discm does not match. Expected: {:?}. Received: {:?}",
                        CLOSE_POSITION_IX_DISCM, maybe_discm
                    ),
                ),
            );
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&CLOSE_POSITION_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn close_position_ix_with_program_id(
    program_id: Pubkey,
    keys: ClosePositionKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; CLOSE_POSITION_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: ClosePositionIxData.try_to_vec()?,
    })
}
pub fn close_position_ix(keys: ClosePositionKeys) -> std::io::Result<Instruction> {
    close_position_ix_with_program_id(crate::ID, keys)
}
pub fn close_position_invoke_with_program_id(
    program_id: Pubkey,
    accounts: ClosePositionAccounts<'_, '_>,
) -> ProgramResult {
    let keys: ClosePositionKeys = accounts.into();
    let ix = close_position_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn close_position_invoke(accounts: ClosePositionAccounts<'_, '_>) -> ProgramResult {
    close_position_invoke_with_program_id(crate::ID, accounts)
}
pub fn close_position_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: ClosePositionAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: ClosePositionKeys = accounts.into();
    let ix = close_position_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn close_position_invoke_signed(
    accounts: ClosePositionAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    close_position_invoke_signed_with_program_id(crate::ID, accounts, seeds)
}
pub fn close_position_verify_account_keys(
    accounts: ClosePositionAccounts<'_, '_>,
    keys: ClosePositionKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.position.key, keys.position),
        (*accounts.lb_pair.key, keys.lb_pair),
        (*accounts.bin_array_lower.key, keys.bin_array_lower),
        (*accounts.bin_array_upper.key, keys.bin_array_upper),
        (*accounts.sender.key, keys.sender),
        (*accounts.rent_receiver.key, keys.rent_receiver),
        (*accounts.event_authority.key, keys.event_authority),
        (*accounts.program.key, keys.program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn close_position_verify_writable_privileges<'me, 'info>(
    accounts: ClosePositionAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.position,
        accounts.lb_pair,
        accounts.bin_array_lower,
        accounts.bin_array_upper,
        accounts.rent_receiver,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn close_position_verify_signer_privileges<'me, 'info>(
    accounts: ClosePositionAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.sender] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn close_position_verify_account_privileges<'me, 'info>(
    accounts: ClosePositionAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    close_position_verify_writable_privileges(accounts)?;
    close_position_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const UPDATE_FEE_PARAMETERS_IX_ACCOUNTS_LEN: usize = 4;
#[derive(Copy, Clone, Debug)]
pub struct UpdateFeeParametersAccounts<'me, 'info> {
    pub lb_pair: &'me AccountInfo<'info>,
    pub admin: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct UpdateFeeParametersKeys {
    pub lb_pair: Pubkey,
    pub admin: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<UpdateFeeParametersAccounts<'_, '_>> for UpdateFeeParametersKeys {
    fn from(accounts: UpdateFeeParametersAccounts) -> Self {
        Self {
            lb_pair: *accounts.lb_pair.key,
            admin: *accounts.admin.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<UpdateFeeParametersKeys>
for [AccountMeta; UPDATE_FEE_PARAMETERS_IX_ACCOUNTS_LEN] {
    fn from(keys: UpdateFeeParametersKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.lb_pair,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.admin,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.event_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; UPDATE_FEE_PARAMETERS_IX_ACCOUNTS_LEN]> for UpdateFeeParametersKeys {
    fn from(pubkeys: [Pubkey; UPDATE_FEE_PARAMETERS_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            lb_pair: pubkeys[0],
            admin: pubkeys[1],
            event_authority: pubkeys[2],
            program: pubkeys[3],
        }
    }
}
impl<'info> From<UpdateFeeParametersAccounts<'_, 'info>>
for [AccountInfo<'info>; UPDATE_FEE_PARAMETERS_IX_ACCOUNTS_LEN] {
    fn from(accounts: UpdateFeeParametersAccounts<'_, 'info>) -> Self {
        [
            accounts.lb_pair.clone(),
            accounts.admin.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; UPDATE_FEE_PARAMETERS_IX_ACCOUNTS_LEN]>
for UpdateFeeParametersAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; UPDATE_FEE_PARAMETERS_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            lb_pair: &arr[0],
            admin: &arr[1],
            event_authority: &arr[2],
            program: &arr[3],
        }
    }
}
pub const UPDATE_FEE_PARAMETERS_IX_DISCM: [u8; 8] = [
    128,
    128,
    208,
    91,
    246,
    53,
    31,
    176,
];
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct UpdateFeeParametersIxArgs {
    pub fee_parameter: FeeParameter,
}
#[derive(Clone, Debug, PartialEq)]
pub struct UpdateFeeParametersIxData(pub UpdateFeeParametersIxArgs);
impl From<UpdateFeeParametersIxArgs> for UpdateFeeParametersIxData {
    fn from(args: UpdateFeeParametersIxArgs) -> Self {
        Self(args)
    }
}
impl UpdateFeeParametersIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != UPDATE_FEE_PARAMETERS_IX_DISCM {
            return Err(
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!(
                        "discm does not match. Expected: {:?}. Received: {:?}",
                        UPDATE_FEE_PARAMETERS_IX_DISCM, maybe_discm
                    ),
                ),
            );
        }
        Ok(Self(UpdateFeeParametersIxArgs::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&UPDATE_FEE_PARAMETERS_IX_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn update_fee_parameters_ix_with_program_id(
    program_id: Pubkey,
    keys: UpdateFeeParametersKeys,
    args: UpdateFeeParametersIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; UPDATE_FEE_PARAMETERS_IX_ACCOUNTS_LEN] = keys.into();
    let data: UpdateFeeParametersIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn update_fee_parameters_ix(
    keys: UpdateFeeParametersKeys,
    args: UpdateFeeParametersIxArgs,
) -> std::io::Result<Instruction> {
    update_fee_parameters_ix_with_program_id(crate::ID, keys, args)
}
pub fn update_fee_parameters_invoke_with_program_id(
    program_id: Pubkey,
    accounts: UpdateFeeParametersAccounts<'_, '_>,
    args: UpdateFeeParametersIxArgs,
) -> ProgramResult {
    let keys: UpdateFeeParametersKeys = accounts.into();
    let ix = update_fee_parameters_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn update_fee_parameters_invoke(
    accounts: UpdateFeeParametersAccounts<'_, '_>,
    args: UpdateFeeParametersIxArgs,
) -> ProgramResult {
    update_fee_parameters_invoke_with_program_id(crate::ID, accounts, args)
}
pub fn update_fee_parameters_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: UpdateFeeParametersAccounts<'_, '_>,
    args: UpdateFeeParametersIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: UpdateFeeParametersKeys = accounts.into();
    let ix = update_fee_parameters_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn update_fee_parameters_invoke_signed(
    accounts: UpdateFeeParametersAccounts<'_, '_>,
    args: UpdateFeeParametersIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    update_fee_parameters_invoke_signed_with_program_id(crate::ID, accounts, args, seeds)
}
pub fn update_fee_parameters_verify_account_keys(
    accounts: UpdateFeeParametersAccounts<'_, '_>,
    keys: UpdateFeeParametersKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.lb_pair.key, keys.lb_pair),
        (*accounts.admin.key, keys.admin),
        (*accounts.event_authority.key, keys.event_authority),
        (*accounts.program.key, keys.program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn update_fee_parameters_verify_writable_privileges<'me, 'info>(
    accounts: UpdateFeeParametersAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.lb_pair] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn update_fee_parameters_verify_signer_privileges<'me, 'info>(
    accounts: UpdateFeeParametersAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.admin] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn update_fee_parameters_verify_account_privileges<'me, 'info>(
    accounts: UpdateFeeParametersAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    update_fee_parameters_verify_writable_privileges(accounts)?;
    update_fee_parameters_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const INCREASE_ORACLE_LENGTH_IX_ACCOUNTS_LEN: usize = 5;
#[derive(Copy, Clone, Debug)]
pub struct IncreaseOracleLengthAccounts<'me, 'info> {
    pub oracle: &'me AccountInfo<'info>,
    pub funder: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct IncreaseOracleLengthKeys {
    pub oracle: Pubkey,
    pub funder: Pubkey,
    pub system_program: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<IncreaseOracleLengthAccounts<'_, '_>> for IncreaseOracleLengthKeys {
    fn from(accounts: IncreaseOracleLengthAccounts) -> Self {
        Self {
            oracle: *accounts.oracle.key,
            funder: *accounts.funder.key,
            system_program: *accounts.system_program.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<IncreaseOracleLengthKeys>
for [AccountMeta; INCREASE_ORACLE_LENGTH_IX_ACCOUNTS_LEN] {
    fn from(keys: IncreaseOracleLengthKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.oracle,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.funder,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.system_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.event_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; INCREASE_ORACLE_LENGTH_IX_ACCOUNTS_LEN]>
for IncreaseOracleLengthKeys {
    fn from(pubkeys: [Pubkey; INCREASE_ORACLE_LENGTH_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            oracle: pubkeys[0],
            funder: pubkeys[1],
            system_program: pubkeys[2],
            event_authority: pubkeys[3],
            program: pubkeys[4],
        }
    }
}
impl<'info> From<IncreaseOracleLengthAccounts<'_, 'info>>
for [AccountInfo<'info>; INCREASE_ORACLE_LENGTH_IX_ACCOUNTS_LEN] {
    fn from(accounts: IncreaseOracleLengthAccounts<'_, 'info>) -> Self {
        [
            accounts.oracle.clone(),
            accounts.funder.clone(),
            accounts.system_program.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; INCREASE_ORACLE_LENGTH_IX_ACCOUNTS_LEN]>
for IncreaseOracleLengthAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; INCREASE_ORACLE_LENGTH_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            oracle: &arr[0],
            funder: &arr[1],
            system_program: &arr[2],
            event_authority: &arr[3],
            program: &arr[4],
        }
    }
}
pub const INCREASE_ORACLE_LENGTH_IX_DISCM: [u8; 8] = [
    190,
    61,
    125,
    87,
    103,
    79,
    158,
    173,
];
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct IncreaseOracleLengthIxArgs {
    pub length_to_add: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct IncreaseOracleLengthIxData(pub IncreaseOracleLengthIxArgs);
impl From<IncreaseOracleLengthIxArgs> for IncreaseOracleLengthIxData {
    fn from(args: IncreaseOracleLengthIxArgs) -> Self {
        Self(args)
    }
}
impl IncreaseOracleLengthIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != INCREASE_ORACLE_LENGTH_IX_DISCM {
            return Err(
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!(
                        "discm does not match. Expected: {:?}. Received: {:?}",
                        INCREASE_ORACLE_LENGTH_IX_DISCM, maybe_discm
                    ),
                ),
            );
        }
        Ok(Self(IncreaseOracleLengthIxArgs::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&INCREASE_ORACLE_LENGTH_IX_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn increase_oracle_length_ix_with_program_id(
    program_id: Pubkey,
    keys: IncreaseOracleLengthKeys,
    args: IncreaseOracleLengthIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; INCREASE_ORACLE_LENGTH_IX_ACCOUNTS_LEN] = keys.into();
    let data: IncreaseOracleLengthIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn increase_oracle_length_ix(
    keys: IncreaseOracleLengthKeys,
    args: IncreaseOracleLengthIxArgs,
) -> std::io::Result<Instruction> {
    increase_oracle_length_ix_with_program_id(crate::ID, keys, args)
}
pub fn increase_oracle_length_invoke_with_program_id(
    program_id: Pubkey,
    accounts: IncreaseOracleLengthAccounts<'_, '_>,
    args: IncreaseOracleLengthIxArgs,
) -> ProgramResult {
    let keys: IncreaseOracleLengthKeys = accounts.into();
    let ix = increase_oracle_length_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn increase_oracle_length_invoke(
    accounts: IncreaseOracleLengthAccounts<'_, '_>,
    args: IncreaseOracleLengthIxArgs,
) -> ProgramResult {
    increase_oracle_length_invoke_with_program_id(crate::ID, accounts, args)
}
pub fn increase_oracle_length_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: IncreaseOracleLengthAccounts<'_, '_>,
    args: IncreaseOracleLengthIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: IncreaseOracleLengthKeys = accounts.into();
    let ix = increase_oracle_length_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn increase_oracle_length_invoke_signed(
    accounts: IncreaseOracleLengthAccounts<'_, '_>,
    args: IncreaseOracleLengthIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    increase_oracle_length_invoke_signed_with_program_id(
        crate::ID,
        accounts,
        args,
        seeds,
    )
}
pub fn increase_oracle_length_verify_account_keys(
    accounts: IncreaseOracleLengthAccounts<'_, '_>,
    keys: IncreaseOracleLengthKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.oracle.key, keys.oracle),
        (*accounts.funder.key, keys.funder),
        (*accounts.system_program.key, keys.system_program),
        (*accounts.event_authority.key, keys.event_authority),
        (*accounts.program.key, keys.program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn increase_oracle_length_verify_writable_privileges<'me, 'info>(
    accounts: IncreaseOracleLengthAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.oracle, accounts.funder] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn increase_oracle_length_verify_signer_privileges<'me, 'info>(
    accounts: IncreaseOracleLengthAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.funder] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn increase_oracle_length_verify_account_privileges<'me, 'info>(
    accounts: IncreaseOracleLengthAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    increase_oracle_length_verify_writable_privileges(accounts)?;
    increase_oracle_length_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const INITIALIZE_PRESET_PARAMETER_IX_ACCOUNTS_LEN: usize = 4;
#[derive(Copy, Clone, Debug)]
pub struct InitializePresetParameterAccounts<'me, 'info> {
    pub preset_parameter: &'me AccountInfo<'info>,
    pub admin: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub rent: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct InitializePresetParameterKeys {
    pub preset_parameter: Pubkey,
    pub admin: Pubkey,
    pub system_program: Pubkey,
    pub rent: Pubkey,
}
impl From<InitializePresetParameterAccounts<'_, '_>> for InitializePresetParameterKeys {
    fn from(accounts: InitializePresetParameterAccounts) -> Self {
        Self {
            preset_parameter: *accounts.preset_parameter.key,
            admin: *accounts.admin.key,
            system_program: *accounts.system_program.key,
            rent: *accounts.rent.key,
        }
    }
}
impl From<InitializePresetParameterKeys>
for [AccountMeta; INITIALIZE_PRESET_PARAMETER_IX_ACCOUNTS_LEN] {
    fn from(keys: InitializePresetParameterKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.preset_parameter,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.admin,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.system_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.rent,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; INITIALIZE_PRESET_PARAMETER_IX_ACCOUNTS_LEN]>
for InitializePresetParameterKeys {
    fn from(pubkeys: [Pubkey; INITIALIZE_PRESET_PARAMETER_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            preset_parameter: pubkeys[0],
            admin: pubkeys[1],
            system_program: pubkeys[2],
            rent: pubkeys[3],
        }
    }
}
impl<'info> From<InitializePresetParameterAccounts<'_, 'info>>
for [AccountInfo<'info>; INITIALIZE_PRESET_PARAMETER_IX_ACCOUNTS_LEN] {
    fn from(accounts: InitializePresetParameterAccounts<'_, 'info>) -> Self {
        [
            accounts.preset_parameter.clone(),
            accounts.admin.clone(),
            accounts.system_program.clone(),
            accounts.rent.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; INITIALIZE_PRESET_PARAMETER_IX_ACCOUNTS_LEN]>
for InitializePresetParameterAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; INITIALIZE_PRESET_PARAMETER_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            preset_parameter: &arr[0],
            admin: &arr[1],
            system_program: &arr[2],
            rent: &arr[3],
        }
    }
}
pub const INITIALIZE_PRESET_PARAMETER_IX_DISCM: [u8; 8] = [
    66,
    188,
    71,
    211,
    98,
    109,
    14,
    186,
];
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InitializePresetParameterIxArgs {
    pub ix: InitPresetParametersIx,
}
#[derive(Clone, Debug, PartialEq)]
pub struct InitializePresetParameterIxData(pub InitializePresetParameterIxArgs);
impl From<InitializePresetParameterIxArgs> for InitializePresetParameterIxData {
    fn from(args: InitializePresetParameterIxArgs) -> Self {
        Self(args)
    }
}
impl InitializePresetParameterIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != INITIALIZE_PRESET_PARAMETER_IX_DISCM {
            return Err(
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!(
                        "discm does not match. Expected: {:?}. Received: {:?}",
                        INITIALIZE_PRESET_PARAMETER_IX_DISCM, maybe_discm
                    ),
                ),
            );
        }
        Ok(Self(InitializePresetParameterIxArgs::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&INITIALIZE_PRESET_PARAMETER_IX_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn initialize_preset_parameter_ix_with_program_id(
    program_id: Pubkey,
    keys: InitializePresetParameterKeys,
    args: InitializePresetParameterIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; INITIALIZE_PRESET_PARAMETER_IX_ACCOUNTS_LEN] = keys.into();
    let data: InitializePresetParameterIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn initialize_preset_parameter_ix(
    keys: InitializePresetParameterKeys,
    args: InitializePresetParameterIxArgs,
) -> std::io::Result<Instruction> {
    initialize_preset_parameter_ix_with_program_id(crate::ID, keys, args)
}
pub fn initialize_preset_parameter_invoke_with_program_id(
    program_id: Pubkey,
    accounts: InitializePresetParameterAccounts<'_, '_>,
    args: InitializePresetParameterIxArgs,
) -> ProgramResult {
    let keys: InitializePresetParameterKeys = accounts.into();
    let ix = initialize_preset_parameter_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn initialize_preset_parameter_invoke(
    accounts: InitializePresetParameterAccounts<'_, '_>,
    args: InitializePresetParameterIxArgs,
) -> ProgramResult {
    initialize_preset_parameter_invoke_with_program_id(crate::ID, accounts, args)
}
pub fn initialize_preset_parameter_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: InitializePresetParameterAccounts<'_, '_>,
    args: InitializePresetParameterIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: InitializePresetParameterKeys = accounts.into();
    let ix = initialize_preset_parameter_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn initialize_preset_parameter_invoke_signed(
    accounts: InitializePresetParameterAccounts<'_, '_>,
    args: InitializePresetParameterIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    initialize_preset_parameter_invoke_signed_with_program_id(
        crate::ID,
        accounts,
        args,
        seeds,
    )
}
pub fn initialize_preset_parameter_verify_account_keys(
    accounts: InitializePresetParameterAccounts<'_, '_>,
    keys: InitializePresetParameterKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.preset_parameter.key, keys.preset_parameter),
        (*accounts.admin.key, keys.admin),
        (*accounts.system_program.key, keys.system_program),
        (*accounts.rent.key, keys.rent),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn initialize_preset_parameter_verify_writable_privileges<'me, 'info>(
    accounts: InitializePresetParameterAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.preset_parameter, accounts.admin] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn initialize_preset_parameter_verify_signer_privileges<'me, 'info>(
    accounts: InitializePresetParameterAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.admin] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn initialize_preset_parameter_verify_account_privileges<'me, 'info>(
    accounts: InitializePresetParameterAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    initialize_preset_parameter_verify_writable_privileges(accounts)?;
    initialize_preset_parameter_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const CLOSE_PRESET_PARAMETER_IX_ACCOUNTS_LEN: usize = 3;
#[derive(Copy, Clone, Debug)]
pub struct ClosePresetParameterAccounts<'me, 'info> {
    pub preset_parameter: &'me AccountInfo<'info>,
    pub admin: &'me AccountInfo<'info>,
    pub rent_receiver: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct ClosePresetParameterKeys {
    pub preset_parameter: Pubkey,
    pub admin: Pubkey,
    pub rent_receiver: Pubkey,
}
impl From<ClosePresetParameterAccounts<'_, '_>> for ClosePresetParameterKeys {
    fn from(accounts: ClosePresetParameterAccounts) -> Self {
        Self {
            preset_parameter: *accounts.preset_parameter.key,
            admin: *accounts.admin.key,
            rent_receiver: *accounts.rent_receiver.key,
        }
    }
}
impl From<ClosePresetParameterKeys>
for [AccountMeta; CLOSE_PRESET_PARAMETER_IX_ACCOUNTS_LEN] {
    fn from(keys: ClosePresetParameterKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.preset_parameter,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.admin,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.rent_receiver,
                is_signer: false,
                is_writable: true,
            },
        ]
    }
}
impl From<[Pubkey; CLOSE_PRESET_PARAMETER_IX_ACCOUNTS_LEN]>
for ClosePresetParameterKeys {
    fn from(pubkeys: [Pubkey; CLOSE_PRESET_PARAMETER_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            preset_parameter: pubkeys[0],
            admin: pubkeys[1],
            rent_receiver: pubkeys[2],
        }
    }
}
impl<'info> From<ClosePresetParameterAccounts<'_, 'info>>
for [AccountInfo<'info>; CLOSE_PRESET_PARAMETER_IX_ACCOUNTS_LEN] {
    fn from(accounts: ClosePresetParameterAccounts<'_, 'info>) -> Self {
        [
            accounts.preset_parameter.clone(),
            accounts.admin.clone(),
            accounts.rent_receiver.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; CLOSE_PRESET_PARAMETER_IX_ACCOUNTS_LEN]>
for ClosePresetParameterAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; CLOSE_PRESET_PARAMETER_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            preset_parameter: &arr[0],
            admin: &arr[1],
            rent_receiver: &arr[2],
        }
    }
}
pub const CLOSE_PRESET_PARAMETER_IX_DISCM: [u8; 8] = [
    4,
    148,
    145,
    100,
    134,
    26,
    181,
    61,
];
#[derive(Clone, Debug, PartialEq)]
pub struct ClosePresetParameterIxData;
impl ClosePresetParameterIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != CLOSE_PRESET_PARAMETER_IX_DISCM {
            return Err(
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!(
                        "discm does not match. Expected: {:?}. Received: {:?}",
                        CLOSE_PRESET_PARAMETER_IX_DISCM, maybe_discm
                    ),
                ),
            );
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&CLOSE_PRESET_PARAMETER_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn close_preset_parameter_ix_with_program_id(
    program_id: Pubkey,
    keys: ClosePresetParameterKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; CLOSE_PRESET_PARAMETER_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: ClosePresetParameterIxData.try_to_vec()?,
    })
}
pub fn close_preset_parameter_ix(
    keys: ClosePresetParameterKeys,
) -> std::io::Result<Instruction> {
    close_preset_parameter_ix_with_program_id(crate::ID, keys)
}
pub fn close_preset_parameter_invoke_with_program_id(
    program_id: Pubkey,
    accounts: ClosePresetParameterAccounts<'_, '_>,
) -> ProgramResult {
    let keys: ClosePresetParameterKeys = accounts.into();
    let ix = close_preset_parameter_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn close_preset_parameter_invoke(
    accounts: ClosePresetParameterAccounts<'_, '_>,
) -> ProgramResult {
    close_preset_parameter_invoke_with_program_id(crate::ID, accounts)
}
pub fn close_preset_parameter_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: ClosePresetParameterAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: ClosePresetParameterKeys = accounts.into();
    let ix = close_preset_parameter_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn close_preset_parameter_invoke_signed(
    accounts: ClosePresetParameterAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    close_preset_parameter_invoke_signed_with_program_id(crate::ID, accounts, seeds)
}
pub fn close_preset_parameter_verify_account_keys(
    accounts: ClosePresetParameterAccounts<'_, '_>,
    keys: ClosePresetParameterKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.preset_parameter.key, keys.preset_parameter),
        (*accounts.admin.key, keys.admin),
        (*accounts.rent_receiver.key, keys.rent_receiver),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn close_preset_parameter_verify_writable_privileges<'me, 'info>(
    accounts: ClosePresetParameterAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.preset_parameter,
        accounts.admin,
        accounts.rent_receiver,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn close_preset_parameter_verify_signer_privileges<'me, 'info>(
    accounts: ClosePresetParameterAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.admin] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn close_preset_parameter_verify_account_privileges<'me, 'info>(
    accounts: ClosePresetParameterAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    close_preset_parameter_verify_writable_privileges(accounts)?;
    close_preset_parameter_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const REMOVE_ALL_LIQUIDITY_IX_ACCOUNTS_LEN: usize = 16;
#[derive(Copy, Clone, Debug)]
pub struct RemoveAllLiquidityAccounts<'me, 'info> {
    pub position: &'me AccountInfo<'info>,
    pub lb_pair: &'me AccountInfo<'info>,
    pub bin_array_bitmap_extension: &'me AccountInfo<'info>,
    pub user_token_x: &'me AccountInfo<'info>,
    pub user_token_y: &'me AccountInfo<'info>,
    pub reserve_x: &'me AccountInfo<'info>,
    pub reserve_y: &'me AccountInfo<'info>,
    pub token_x_mint: &'me AccountInfo<'info>,
    pub token_y_mint: &'me AccountInfo<'info>,
    pub bin_array_lower: &'me AccountInfo<'info>,
    pub bin_array_upper: &'me AccountInfo<'info>,
    pub sender: &'me AccountInfo<'info>,
    pub token_x_program: &'me AccountInfo<'info>,
    pub token_y_program: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct RemoveAllLiquidityKeys {
    pub position: Pubkey,
    pub lb_pair: Pubkey,
    pub bin_array_bitmap_extension: Pubkey,
    pub user_token_x: Pubkey,
    pub user_token_y: Pubkey,
    pub reserve_x: Pubkey,
    pub reserve_y: Pubkey,
    pub token_x_mint: Pubkey,
    pub token_y_mint: Pubkey,
    pub bin_array_lower: Pubkey,
    pub bin_array_upper: Pubkey,
    pub sender: Pubkey,
    pub token_x_program: Pubkey,
    pub token_y_program: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<RemoveAllLiquidityAccounts<'_, '_>> for RemoveAllLiquidityKeys {
    fn from(accounts: RemoveAllLiquidityAccounts) -> Self {
        Self {
            position: *accounts.position.key,
            lb_pair: *accounts.lb_pair.key,
            bin_array_bitmap_extension: *accounts.bin_array_bitmap_extension.key,
            user_token_x: *accounts.user_token_x.key,
            user_token_y: *accounts.user_token_y.key,
            reserve_x: *accounts.reserve_x.key,
            reserve_y: *accounts.reserve_y.key,
            token_x_mint: *accounts.token_x_mint.key,
            token_y_mint: *accounts.token_y_mint.key,
            bin_array_lower: *accounts.bin_array_lower.key,
            bin_array_upper: *accounts.bin_array_upper.key,
            sender: *accounts.sender.key,
            token_x_program: *accounts.token_x_program.key,
            token_y_program: *accounts.token_y_program.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<RemoveAllLiquidityKeys>
for [AccountMeta; REMOVE_ALL_LIQUIDITY_IX_ACCOUNTS_LEN] {
    fn from(keys: RemoveAllLiquidityKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.position,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.lb_pair,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.bin_array_bitmap_extension,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_token_x,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_token_y,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.reserve_x,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.reserve_y,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_x_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_y_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.bin_array_lower,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.bin_array_upper,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.sender,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_x_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_y_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.event_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; REMOVE_ALL_LIQUIDITY_IX_ACCOUNTS_LEN]> for RemoveAllLiquidityKeys {
    fn from(pubkeys: [Pubkey; REMOVE_ALL_LIQUIDITY_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            position: pubkeys[0],
            lb_pair: pubkeys[1],
            bin_array_bitmap_extension: pubkeys[2],
            user_token_x: pubkeys[3],
            user_token_y: pubkeys[4],
            reserve_x: pubkeys[5],
            reserve_y: pubkeys[6],
            token_x_mint: pubkeys[7],
            token_y_mint: pubkeys[8],
            bin_array_lower: pubkeys[9],
            bin_array_upper: pubkeys[10],
            sender: pubkeys[11],
            token_x_program: pubkeys[12],
            token_y_program: pubkeys[13],
            event_authority: pubkeys[14],
            program: pubkeys[15],
        }
    }
}
impl<'info> From<RemoveAllLiquidityAccounts<'_, 'info>>
for [AccountInfo<'info>; REMOVE_ALL_LIQUIDITY_IX_ACCOUNTS_LEN] {
    fn from(accounts: RemoveAllLiquidityAccounts<'_, 'info>) -> Self {
        [
            accounts.position.clone(),
            accounts.lb_pair.clone(),
            accounts.bin_array_bitmap_extension.clone(),
            accounts.user_token_x.clone(),
            accounts.user_token_y.clone(),
            accounts.reserve_x.clone(),
            accounts.reserve_y.clone(),
            accounts.token_x_mint.clone(),
            accounts.token_y_mint.clone(),
            accounts.bin_array_lower.clone(),
            accounts.bin_array_upper.clone(),
            accounts.sender.clone(),
            accounts.token_x_program.clone(),
            accounts.token_y_program.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; REMOVE_ALL_LIQUIDITY_IX_ACCOUNTS_LEN]>
for RemoveAllLiquidityAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; REMOVE_ALL_LIQUIDITY_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            position: &arr[0],
            lb_pair: &arr[1],
            bin_array_bitmap_extension: &arr[2],
            user_token_x: &arr[3],
            user_token_y: &arr[4],
            reserve_x: &arr[5],
            reserve_y: &arr[6],
            token_x_mint: &arr[7],
            token_y_mint: &arr[8],
            bin_array_lower: &arr[9],
            bin_array_upper: &arr[10],
            sender: &arr[11],
            token_x_program: &arr[12],
            token_y_program: &arr[13],
            event_authority: &arr[14],
            program: &arr[15],
        }
    }
}
pub const REMOVE_ALL_LIQUIDITY_IX_DISCM: [u8; 8] = [10, 51, 61, 35, 112, 105, 24, 85];
#[derive(Clone, Debug, PartialEq)]
pub struct RemoveAllLiquidityIxData;
impl RemoveAllLiquidityIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != REMOVE_ALL_LIQUIDITY_IX_DISCM {
            return Err(
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!(
                        "discm does not match. Expected: {:?}. Received: {:?}",
                        REMOVE_ALL_LIQUIDITY_IX_DISCM, maybe_discm
                    ),
                ),
            );
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&REMOVE_ALL_LIQUIDITY_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn remove_all_liquidity_ix_with_program_id(
    program_id: Pubkey,
    keys: RemoveAllLiquidityKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; REMOVE_ALL_LIQUIDITY_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: RemoveAllLiquidityIxData.try_to_vec()?,
    })
}
pub fn remove_all_liquidity_ix(
    keys: RemoveAllLiquidityKeys,
) -> std::io::Result<Instruction> {
    remove_all_liquidity_ix_with_program_id(crate::ID, keys)
}
pub fn remove_all_liquidity_invoke_with_program_id(
    program_id: Pubkey,
    accounts: RemoveAllLiquidityAccounts<'_, '_>,
) -> ProgramResult {
    let keys: RemoveAllLiquidityKeys = accounts.into();
    let ix = remove_all_liquidity_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn remove_all_liquidity_invoke(
    accounts: RemoveAllLiquidityAccounts<'_, '_>,
) -> ProgramResult {
    remove_all_liquidity_invoke_with_program_id(crate::ID, accounts)
}
pub fn remove_all_liquidity_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: RemoveAllLiquidityAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: RemoveAllLiquidityKeys = accounts.into();
    let ix = remove_all_liquidity_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn remove_all_liquidity_invoke_signed(
    accounts: RemoveAllLiquidityAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    remove_all_liquidity_invoke_signed_with_program_id(crate::ID, accounts, seeds)
}
pub fn remove_all_liquidity_verify_account_keys(
    accounts: RemoveAllLiquidityAccounts<'_, '_>,
    keys: RemoveAllLiquidityKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.position.key, keys.position),
        (*accounts.lb_pair.key, keys.lb_pair),
        (*accounts.bin_array_bitmap_extension.key, keys.bin_array_bitmap_extension),
        (*accounts.user_token_x.key, keys.user_token_x),
        (*accounts.user_token_y.key, keys.user_token_y),
        (*accounts.reserve_x.key, keys.reserve_x),
        (*accounts.reserve_y.key, keys.reserve_y),
        (*accounts.token_x_mint.key, keys.token_x_mint),
        (*accounts.token_y_mint.key, keys.token_y_mint),
        (*accounts.bin_array_lower.key, keys.bin_array_lower),
        (*accounts.bin_array_upper.key, keys.bin_array_upper),
        (*accounts.sender.key, keys.sender),
        (*accounts.token_x_program.key, keys.token_x_program),
        (*accounts.token_y_program.key, keys.token_y_program),
        (*accounts.event_authority.key, keys.event_authority),
        (*accounts.program.key, keys.program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn remove_all_liquidity_verify_writable_privileges<'me, 'info>(
    accounts: RemoveAllLiquidityAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.position,
        accounts.lb_pair,
        accounts.bin_array_bitmap_extension,
        accounts.user_token_x,
        accounts.user_token_y,
        accounts.reserve_x,
        accounts.reserve_y,
        accounts.bin_array_lower,
        accounts.bin_array_upper,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn remove_all_liquidity_verify_signer_privileges<'me, 'info>(
    accounts: RemoveAllLiquidityAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.sender] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn remove_all_liquidity_verify_account_privileges<'me, 'info>(
    accounts: RemoveAllLiquidityAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    remove_all_liquidity_verify_writable_privileges(accounts)?;
    remove_all_liquidity_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const TOGGLE_PAIR_STATUS_IX_ACCOUNTS_LEN: usize = 2;
#[derive(Copy, Clone, Debug)]
pub struct TogglePairStatusAccounts<'me, 'info> {
    pub lb_pair: &'me AccountInfo<'info>,
    pub admin: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct TogglePairStatusKeys {
    pub lb_pair: Pubkey,
    pub admin: Pubkey,
}
impl From<TogglePairStatusAccounts<'_, '_>> for TogglePairStatusKeys {
    fn from(accounts: TogglePairStatusAccounts) -> Self {
        Self {
            lb_pair: *accounts.lb_pair.key,
            admin: *accounts.admin.key,
        }
    }
}
impl From<TogglePairStatusKeys> for [AccountMeta; TOGGLE_PAIR_STATUS_IX_ACCOUNTS_LEN] {
    fn from(keys: TogglePairStatusKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.lb_pair,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.admin,
                is_signer: true,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; TOGGLE_PAIR_STATUS_IX_ACCOUNTS_LEN]> for TogglePairStatusKeys {
    fn from(pubkeys: [Pubkey; TOGGLE_PAIR_STATUS_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            lb_pair: pubkeys[0],
            admin: pubkeys[1],
        }
    }
}
impl<'info> From<TogglePairStatusAccounts<'_, 'info>>
for [AccountInfo<'info>; TOGGLE_PAIR_STATUS_IX_ACCOUNTS_LEN] {
    fn from(accounts: TogglePairStatusAccounts<'_, 'info>) -> Self {
        [accounts.lb_pair.clone(), accounts.admin.clone()]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; TOGGLE_PAIR_STATUS_IX_ACCOUNTS_LEN]>
for TogglePairStatusAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; TOGGLE_PAIR_STATUS_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            lb_pair: &arr[0],
            admin: &arr[1],
        }
    }
}
pub const TOGGLE_PAIR_STATUS_IX_DISCM: [u8; 8] = [61, 115, 52, 23, 46, 13, 31, 144];
#[derive(Clone, Debug, PartialEq)]
pub struct TogglePairStatusIxData;
impl TogglePairStatusIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != TOGGLE_PAIR_STATUS_IX_DISCM {
            return Err(
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!(
                        "discm does not match. Expected: {:?}. Received: {:?}",
                        TOGGLE_PAIR_STATUS_IX_DISCM, maybe_discm
                    ),
                ),
            );
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&TOGGLE_PAIR_STATUS_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn toggle_pair_status_ix_with_program_id(
    program_id: Pubkey,
    keys: TogglePairStatusKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; TOGGLE_PAIR_STATUS_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: TogglePairStatusIxData.try_to_vec()?,
    })
}
pub fn toggle_pair_status_ix(
    keys: TogglePairStatusKeys,
) -> std::io::Result<Instruction> {
    toggle_pair_status_ix_with_program_id(crate::ID, keys)
}
pub fn toggle_pair_status_invoke_with_program_id(
    program_id: Pubkey,
    accounts: TogglePairStatusAccounts<'_, '_>,
) -> ProgramResult {
    let keys: TogglePairStatusKeys = accounts.into();
    let ix = toggle_pair_status_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn toggle_pair_status_invoke(
    accounts: TogglePairStatusAccounts<'_, '_>,
) -> ProgramResult {
    toggle_pair_status_invoke_with_program_id(crate::ID, accounts)
}
pub fn toggle_pair_status_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: TogglePairStatusAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: TogglePairStatusKeys = accounts.into();
    let ix = toggle_pair_status_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn toggle_pair_status_invoke_signed(
    accounts: TogglePairStatusAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    toggle_pair_status_invoke_signed_with_program_id(crate::ID, accounts, seeds)
}
pub fn toggle_pair_status_verify_account_keys(
    accounts: TogglePairStatusAccounts<'_, '_>,
    keys: TogglePairStatusKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.lb_pair.key, keys.lb_pair),
        (*accounts.admin.key, keys.admin),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn toggle_pair_status_verify_writable_privileges<'me, 'info>(
    accounts: TogglePairStatusAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.lb_pair] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn toggle_pair_status_verify_signer_privileges<'me, 'info>(
    accounts: TogglePairStatusAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.admin] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn toggle_pair_status_verify_account_privileges<'me, 'info>(
    accounts: TogglePairStatusAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    toggle_pair_status_verify_writable_privileges(accounts)?;
    toggle_pair_status_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const MIGRATE_POSITION_IX_ACCOUNTS_LEN: usize = 10;
#[derive(Copy, Clone, Debug)]
pub struct MigratePositionAccounts<'me, 'info> {
    pub position_v2: &'me AccountInfo<'info>,
    pub position_v1: &'me AccountInfo<'info>,
    pub lb_pair: &'me AccountInfo<'info>,
    pub bin_array_lower: &'me AccountInfo<'info>,
    pub bin_array_upper: &'me AccountInfo<'info>,
    pub owner: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub rent_receiver: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MigratePositionKeys {
    pub position_v2: Pubkey,
    pub position_v1: Pubkey,
    pub lb_pair: Pubkey,
    pub bin_array_lower: Pubkey,
    pub bin_array_upper: Pubkey,
    pub owner: Pubkey,
    pub system_program: Pubkey,
    pub rent_receiver: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<MigratePositionAccounts<'_, '_>> for MigratePositionKeys {
    fn from(accounts: MigratePositionAccounts) -> Self {
        Self {
            position_v2: *accounts.position_v2.key,
            position_v1: *accounts.position_v1.key,
            lb_pair: *accounts.lb_pair.key,
            bin_array_lower: *accounts.bin_array_lower.key,
            bin_array_upper: *accounts.bin_array_upper.key,
            owner: *accounts.owner.key,
            system_program: *accounts.system_program.key,
            rent_receiver: *accounts.rent_receiver.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<MigratePositionKeys> for [AccountMeta; MIGRATE_POSITION_IX_ACCOUNTS_LEN] {
    fn from(keys: MigratePositionKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.position_v2,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.position_v1,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.lb_pair,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.bin_array_lower,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.bin_array_upper,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.owner,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.system_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.rent_receiver,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.event_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; MIGRATE_POSITION_IX_ACCOUNTS_LEN]> for MigratePositionKeys {
    fn from(pubkeys: [Pubkey; MIGRATE_POSITION_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            position_v2: pubkeys[0],
            position_v1: pubkeys[1],
            lb_pair: pubkeys[2],
            bin_array_lower: pubkeys[3],
            bin_array_upper: pubkeys[4],
            owner: pubkeys[5],
            system_program: pubkeys[6],
            rent_receiver: pubkeys[7],
            event_authority: pubkeys[8],
            program: pubkeys[9],
        }
    }
}
impl<'info> From<MigratePositionAccounts<'_, 'info>>
for [AccountInfo<'info>; MIGRATE_POSITION_IX_ACCOUNTS_LEN] {
    fn from(accounts: MigratePositionAccounts<'_, 'info>) -> Self {
        [
            accounts.position_v2.clone(),
            accounts.position_v1.clone(),
            accounts.lb_pair.clone(),
            accounts.bin_array_lower.clone(),
            accounts.bin_array_upper.clone(),
            accounts.owner.clone(),
            accounts.system_program.clone(),
            accounts.rent_receiver.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; MIGRATE_POSITION_IX_ACCOUNTS_LEN]>
for MigratePositionAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; MIGRATE_POSITION_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            position_v2: &arr[0],
            position_v1: &arr[1],
            lb_pair: &arr[2],
            bin_array_lower: &arr[3],
            bin_array_upper: &arr[4],
            owner: &arr[5],
            system_program: &arr[6],
            rent_receiver: &arr[7],
            event_authority: &arr[8],
            program: &arr[9],
        }
    }
}
pub const MIGRATE_POSITION_IX_DISCM: [u8; 8] = [15, 132, 59, 50, 199, 6, 251, 46];
#[derive(Clone, Debug, PartialEq)]
pub struct MigratePositionIxData;
impl MigratePositionIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != MIGRATE_POSITION_IX_DISCM {
            return Err(
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!(
                        "discm does not match. Expected: {:?}. Received: {:?}",
                        MIGRATE_POSITION_IX_DISCM, maybe_discm
                    ),
                ),
            );
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&MIGRATE_POSITION_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn migrate_position_ix_with_program_id(
    program_id: Pubkey,
    keys: MigratePositionKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; MIGRATE_POSITION_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: MigratePositionIxData.try_to_vec()?,
    })
}
pub fn migrate_position_ix(keys: MigratePositionKeys) -> std::io::Result<Instruction> {
    migrate_position_ix_with_program_id(crate::ID, keys)
}
pub fn migrate_position_invoke_with_program_id(
    program_id: Pubkey,
    accounts: MigratePositionAccounts<'_, '_>,
) -> ProgramResult {
    let keys: MigratePositionKeys = accounts.into();
    let ix = migrate_position_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn migrate_position_invoke(
    accounts: MigratePositionAccounts<'_, '_>,
) -> ProgramResult {
    migrate_position_invoke_with_program_id(crate::ID, accounts)
}
pub fn migrate_position_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: MigratePositionAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: MigratePositionKeys = accounts.into();
    let ix = migrate_position_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn migrate_position_invoke_signed(
    accounts: MigratePositionAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    migrate_position_invoke_signed_with_program_id(crate::ID, accounts, seeds)
}
pub fn migrate_position_verify_account_keys(
    accounts: MigratePositionAccounts<'_, '_>,
    keys: MigratePositionKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.position_v2.key, keys.position_v2),
        (*accounts.position_v1.key, keys.position_v1),
        (*accounts.lb_pair.key, keys.lb_pair),
        (*accounts.bin_array_lower.key, keys.bin_array_lower),
        (*accounts.bin_array_upper.key, keys.bin_array_upper),
        (*accounts.owner.key, keys.owner),
        (*accounts.system_program.key, keys.system_program),
        (*accounts.rent_receiver.key, keys.rent_receiver),
        (*accounts.event_authority.key, keys.event_authority),
        (*accounts.program.key, keys.program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn migrate_position_verify_writable_privileges<'me, 'info>(
    accounts: MigratePositionAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.position_v2,
        accounts.position_v1,
        accounts.bin_array_lower,
        accounts.bin_array_upper,
        accounts.owner,
        accounts.rent_receiver,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn migrate_position_verify_signer_privileges<'me, 'info>(
    accounts: MigratePositionAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.position_v2, accounts.owner] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn migrate_position_verify_account_privileges<'me, 'info>(
    accounts: MigratePositionAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    migrate_position_verify_writable_privileges(accounts)?;
    migrate_position_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const MIGRATE_BIN_ARRAY_IX_ACCOUNTS_LEN: usize = 1;
#[derive(Copy, Clone, Debug)]
pub struct MigrateBinArrayAccounts<'me, 'info> {
    pub lb_pair: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MigrateBinArrayKeys {
    pub lb_pair: Pubkey,
}
impl From<MigrateBinArrayAccounts<'_, '_>> for MigrateBinArrayKeys {
    fn from(accounts: MigrateBinArrayAccounts) -> Self {
        Self {
            lb_pair: *accounts.lb_pair.key,
        }
    }
}
impl From<MigrateBinArrayKeys> for [AccountMeta; MIGRATE_BIN_ARRAY_IX_ACCOUNTS_LEN] {
    fn from(keys: MigrateBinArrayKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.lb_pair,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; MIGRATE_BIN_ARRAY_IX_ACCOUNTS_LEN]> for MigrateBinArrayKeys {
    fn from(pubkeys: [Pubkey; MIGRATE_BIN_ARRAY_IX_ACCOUNTS_LEN]) -> Self {
        Self { lb_pair: pubkeys[0] }
    }
}
impl<'info> From<MigrateBinArrayAccounts<'_, 'info>>
for [AccountInfo<'info>; MIGRATE_BIN_ARRAY_IX_ACCOUNTS_LEN] {
    fn from(accounts: MigrateBinArrayAccounts<'_, 'info>) -> Self {
        [accounts.lb_pair.clone()]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; MIGRATE_BIN_ARRAY_IX_ACCOUNTS_LEN]>
for MigrateBinArrayAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; MIGRATE_BIN_ARRAY_IX_ACCOUNTS_LEN]) -> Self {
        Self { lb_pair: &arr[0] }
    }
}
pub const MIGRATE_BIN_ARRAY_IX_DISCM: [u8; 8] = [17, 23, 159, 211, 101, 184, 41, 241];
#[derive(Clone, Debug, PartialEq)]
pub struct MigrateBinArrayIxData;
impl MigrateBinArrayIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != MIGRATE_BIN_ARRAY_IX_DISCM {
            return Err(
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!(
                        "discm does not match. Expected: {:?}. Received: {:?}",
                        MIGRATE_BIN_ARRAY_IX_DISCM, maybe_discm
                    ),
                ),
            );
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&MIGRATE_BIN_ARRAY_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn migrate_bin_array_ix_with_program_id(
    program_id: Pubkey,
    keys: MigrateBinArrayKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; MIGRATE_BIN_ARRAY_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: MigrateBinArrayIxData.try_to_vec()?,
    })
}
pub fn migrate_bin_array_ix(keys: MigrateBinArrayKeys) -> std::io::Result<Instruction> {
    migrate_bin_array_ix_with_program_id(crate::ID, keys)
}
pub fn migrate_bin_array_invoke_with_program_id(
    program_id: Pubkey,
    accounts: MigrateBinArrayAccounts<'_, '_>,
) -> ProgramResult {
    let keys: MigrateBinArrayKeys = accounts.into();
    let ix = migrate_bin_array_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn migrate_bin_array_invoke(
    accounts: MigrateBinArrayAccounts<'_, '_>,
) -> ProgramResult {
    migrate_bin_array_invoke_with_program_id(crate::ID, accounts)
}
pub fn migrate_bin_array_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: MigrateBinArrayAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: MigrateBinArrayKeys = accounts.into();
    let ix = migrate_bin_array_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn migrate_bin_array_invoke_signed(
    accounts: MigrateBinArrayAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    migrate_bin_array_invoke_signed_with_program_id(crate::ID, accounts, seeds)
}
pub fn migrate_bin_array_verify_account_keys(
    accounts: MigrateBinArrayAccounts<'_, '_>,
    keys: MigrateBinArrayKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [(*accounts.lb_pair.key, keys.lb_pair)] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub const UPDATE_FEES_AND_REWARDS_IX_ACCOUNTS_LEN: usize = 5;
#[derive(Copy, Clone, Debug)]
pub struct UpdateFeesAndRewardsAccounts<'me, 'info> {
    pub position: &'me AccountInfo<'info>,
    pub lb_pair: &'me AccountInfo<'info>,
    pub bin_array_lower: &'me AccountInfo<'info>,
    pub bin_array_upper: &'me AccountInfo<'info>,
    pub owner: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct UpdateFeesAndRewardsKeys {
    pub position: Pubkey,
    pub lb_pair: Pubkey,
    pub bin_array_lower: Pubkey,
    pub bin_array_upper: Pubkey,
    pub owner: Pubkey,
}
impl From<UpdateFeesAndRewardsAccounts<'_, '_>> for UpdateFeesAndRewardsKeys {
    fn from(accounts: UpdateFeesAndRewardsAccounts) -> Self {
        Self {
            position: *accounts.position.key,
            lb_pair: *accounts.lb_pair.key,
            bin_array_lower: *accounts.bin_array_lower.key,
            bin_array_upper: *accounts.bin_array_upper.key,
            owner: *accounts.owner.key,
        }
    }
}
impl From<UpdateFeesAndRewardsKeys>
for [AccountMeta; UPDATE_FEES_AND_REWARDS_IX_ACCOUNTS_LEN] {
    fn from(keys: UpdateFeesAndRewardsKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.position,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.lb_pair,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.bin_array_lower,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.bin_array_upper,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.owner,
                is_signer: true,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; UPDATE_FEES_AND_REWARDS_IX_ACCOUNTS_LEN]>
for UpdateFeesAndRewardsKeys {
    fn from(pubkeys: [Pubkey; UPDATE_FEES_AND_REWARDS_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            position: pubkeys[0],
            lb_pair: pubkeys[1],
            bin_array_lower: pubkeys[2],
            bin_array_upper: pubkeys[3],
            owner: pubkeys[4],
        }
    }
}
impl<'info> From<UpdateFeesAndRewardsAccounts<'_, 'info>>
for [AccountInfo<'info>; UPDATE_FEES_AND_REWARDS_IX_ACCOUNTS_LEN] {
    fn from(accounts: UpdateFeesAndRewardsAccounts<'_, 'info>) -> Self {
        [
            accounts.position.clone(),
            accounts.lb_pair.clone(),
            accounts.bin_array_lower.clone(),
            accounts.bin_array_upper.clone(),
            accounts.owner.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; UPDATE_FEES_AND_REWARDS_IX_ACCOUNTS_LEN]>
for UpdateFeesAndRewardsAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; UPDATE_FEES_AND_REWARDS_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            position: &arr[0],
            lb_pair: &arr[1],
            bin_array_lower: &arr[2],
            bin_array_upper: &arr[3],
            owner: &arr[4],
        }
    }
}
pub const UPDATE_FEES_AND_REWARDS_IX_DISCM: [u8; 8] = [
    154,
    230,
    250,
    13,
    236,
    209,
    75,
    223,
];
#[derive(Clone, Debug, PartialEq)]
pub struct UpdateFeesAndRewardsIxData;
impl UpdateFeesAndRewardsIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != UPDATE_FEES_AND_REWARDS_IX_DISCM {
            return Err(
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!(
                        "discm does not match. Expected: {:?}. Received: {:?}",
                        UPDATE_FEES_AND_REWARDS_IX_DISCM, maybe_discm
                    ),
                ),
            );
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&UPDATE_FEES_AND_REWARDS_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn update_fees_and_rewards_ix_with_program_id(
    program_id: Pubkey,
    keys: UpdateFeesAndRewardsKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; UPDATE_FEES_AND_REWARDS_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: UpdateFeesAndRewardsIxData.try_to_vec()?,
    })
}
pub fn update_fees_and_rewards_ix(
    keys: UpdateFeesAndRewardsKeys,
) -> std::io::Result<Instruction> {
    update_fees_and_rewards_ix_with_program_id(crate::ID, keys)
}
pub fn update_fees_and_rewards_invoke_with_program_id(
    program_id: Pubkey,
    accounts: UpdateFeesAndRewardsAccounts<'_, '_>,
) -> ProgramResult {
    let keys: UpdateFeesAndRewardsKeys = accounts.into();
    let ix = update_fees_and_rewards_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn update_fees_and_rewards_invoke(
    accounts: UpdateFeesAndRewardsAccounts<'_, '_>,
) -> ProgramResult {
    update_fees_and_rewards_invoke_with_program_id(crate::ID, accounts)
}
pub fn update_fees_and_rewards_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: UpdateFeesAndRewardsAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: UpdateFeesAndRewardsKeys = accounts.into();
    let ix = update_fees_and_rewards_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn update_fees_and_rewards_invoke_signed(
    accounts: UpdateFeesAndRewardsAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    update_fees_and_rewards_invoke_signed_with_program_id(crate::ID, accounts, seeds)
}
pub fn update_fees_and_rewards_verify_account_keys(
    accounts: UpdateFeesAndRewardsAccounts<'_, '_>,
    keys: UpdateFeesAndRewardsKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.position.key, keys.position),
        (*accounts.lb_pair.key, keys.lb_pair),
        (*accounts.bin_array_lower.key, keys.bin_array_lower),
        (*accounts.bin_array_upper.key, keys.bin_array_upper),
        (*accounts.owner.key, keys.owner),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn update_fees_and_rewards_verify_writable_privileges<'me, 'info>(
    accounts: UpdateFeesAndRewardsAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.position,
        accounts.lb_pair,
        accounts.bin_array_lower,
        accounts.bin_array_upper,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn update_fees_and_rewards_verify_signer_privileges<'me, 'info>(
    accounts: UpdateFeesAndRewardsAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.owner] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn update_fees_and_rewards_verify_account_privileges<'me, 'info>(
    accounts: UpdateFeesAndRewardsAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    update_fees_and_rewards_verify_writable_privileges(accounts)?;
    update_fees_and_rewards_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const WITHDRAW_INELIGIBLE_REWARD_IX_ACCOUNTS_LEN: usize = 9;
#[derive(Copy, Clone, Debug)]
pub struct WithdrawIneligibleRewardAccounts<'me, 'info> {
    pub lb_pair: &'me AccountInfo<'info>,
    pub reward_vault: &'me AccountInfo<'info>,
    pub reward_mint: &'me AccountInfo<'info>,
    pub funder_token_account: &'me AccountInfo<'info>,
    pub funder: &'me AccountInfo<'info>,
    pub bin_array: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct WithdrawIneligibleRewardKeys {
    pub lb_pair: Pubkey,
    pub reward_vault: Pubkey,
    pub reward_mint: Pubkey,
    pub funder_token_account: Pubkey,
    pub funder: Pubkey,
    pub bin_array: Pubkey,
    pub token_program: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<WithdrawIneligibleRewardAccounts<'_, '_>> for WithdrawIneligibleRewardKeys {
    fn from(accounts: WithdrawIneligibleRewardAccounts) -> Self {
        Self {
            lb_pair: *accounts.lb_pair.key,
            reward_vault: *accounts.reward_vault.key,
            reward_mint: *accounts.reward_mint.key,
            funder_token_account: *accounts.funder_token_account.key,
            funder: *accounts.funder.key,
            bin_array: *accounts.bin_array.key,
            token_program: *accounts.token_program.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<WithdrawIneligibleRewardKeys>
for [AccountMeta; WITHDRAW_INELIGIBLE_REWARD_IX_ACCOUNTS_LEN] {
    fn from(keys: WithdrawIneligibleRewardKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.lb_pair,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.reward_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.reward_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.funder_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.funder,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.bin_array,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.event_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; WITHDRAW_INELIGIBLE_REWARD_IX_ACCOUNTS_LEN]>
for WithdrawIneligibleRewardKeys {
    fn from(pubkeys: [Pubkey; WITHDRAW_INELIGIBLE_REWARD_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            lb_pair: pubkeys[0],
            reward_vault: pubkeys[1],
            reward_mint: pubkeys[2],
            funder_token_account: pubkeys[3],
            funder: pubkeys[4],
            bin_array: pubkeys[5],
            token_program: pubkeys[6],
            event_authority: pubkeys[7],
            program: pubkeys[8],
        }
    }
}
impl<'info> From<WithdrawIneligibleRewardAccounts<'_, 'info>>
for [AccountInfo<'info>; WITHDRAW_INELIGIBLE_REWARD_IX_ACCOUNTS_LEN] {
    fn from(accounts: WithdrawIneligibleRewardAccounts<'_, 'info>) -> Self {
        [
            accounts.lb_pair.clone(),
            accounts.reward_vault.clone(),
            accounts.reward_mint.clone(),
            accounts.funder_token_account.clone(),
            accounts.funder.clone(),
            accounts.bin_array.clone(),
            accounts.token_program.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; WITHDRAW_INELIGIBLE_REWARD_IX_ACCOUNTS_LEN]>
for WithdrawIneligibleRewardAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; WITHDRAW_INELIGIBLE_REWARD_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            lb_pair: &arr[0],
            reward_vault: &arr[1],
            reward_mint: &arr[2],
            funder_token_account: &arr[3],
            funder: &arr[4],
            bin_array: &arr[5],
            token_program: &arr[6],
            event_authority: &arr[7],
            program: &arr[8],
        }
    }
}
pub const WITHDRAW_INELIGIBLE_REWARD_IX_DISCM: [u8; 8] = [
    148,
    206,
    42,
    195,
    247,
    49,
    103,
    8,
];
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct WithdrawIneligibleRewardIxArgs {
    pub reward_index: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct WithdrawIneligibleRewardIxData(pub WithdrawIneligibleRewardIxArgs);
impl From<WithdrawIneligibleRewardIxArgs> for WithdrawIneligibleRewardIxData {
    fn from(args: WithdrawIneligibleRewardIxArgs) -> Self {
        Self(args)
    }
}
impl WithdrawIneligibleRewardIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != WITHDRAW_INELIGIBLE_REWARD_IX_DISCM {
            return Err(
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!(
                        "discm does not match. Expected: {:?}. Received: {:?}",
                        WITHDRAW_INELIGIBLE_REWARD_IX_DISCM, maybe_discm
                    ),
                ),
            );
        }
        Ok(Self(WithdrawIneligibleRewardIxArgs::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&WITHDRAW_INELIGIBLE_REWARD_IX_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn withdraw_ineligible_reward_ix_with_program_id(
    program_id: Pubkey,
    keys: WithdrawIneligibleRewardKeys,
    args: WithdrawIneligibleRewardIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; WITHDRAW_INELIGIBLE_REWARD_IX_ACCOUNTS_LEN] = keys.into();
    let data: WithdrawIneligibleRewardIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn withdraw_ineligible_reward_ix(
    keys: WithdrawIneligibleRewardKeys,
    args: WithdrawIneligibleRewardIxArgs,
) -> std::io::Result<Instruction> {
    withdraw_ineligible_reward_ix_with_program_id(crate::ID, keys, args)
}
pub fn withdraw_ineligible_reward_invoke_with_program_id(
    program_id: Pubkey,
    accounts: WithdrawIneligibleRewardAccounts<'_, '_>,
    args: WithdrawIneligibleRewardIxArgs,
) -> ProgramResult {
    let keys: WithdrawIneligibleRewardKeys = accounts.into();
    let ix = withdraw_ineligible_reward_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn withdraw_ineligible_reward_invoke(
    accounts: WithdrawIneligibleRewardAccounts<'_, '_>,
    args: WithdrawIneligibleRewardIxArgs,
) -> ProgramResult {
    withdraw_ineligible_reward_invoke_with_program_id(crate::ID, accounts, args)
}
pub fn withdraw_ineligible_reward_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: WithdrawIneligibleRewardAccounts<'_, '_>,
    args: WithdrawIneligibleRewardIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: WithdrawIneligibleRewardKeys = accounts.into();
    let ix = withdraw_ineligible_reward_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn withdraw_ineligible_reward_invoke_signed(
    accounts: WithdrawIneligibleRewardAccounts<'_, '_>,
    args: WithdrawIneligibleRewardIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    withdraw_ineligible_reward_invoke_signed_with_program_id(
        crate::ID,
        accounts,
        args,
        seeds,
    )
}
pub fn withdraw_ineligible_reward_verify_account_keys(
    accounts: WithdrawIneligibleRewardAccounts<'_, '_>,
    keys: WithdrawIneligibleRewardKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.lb_pair.key, keys.lb_pair),
        (*accounts.reward_vault.key, keys.reward_vault),
        (*accounts.reward_mint.key, keys.reward_mint),
        (*accounts.funder_token_account.key, keys.funder_token_account),
        (*accounts.funder.key, keys.funder),
        (*accounts.bin_array.key, keys.bin_array),
        (*accounts.token_program.key, keys.token_program),
        (*accounts.event_authority.key, keys.event_authority),
        (*accounts.program.key, keys.program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn withdraw_ineligible_reward_verify_writable_privileges<'me, 'info>(
    accounts: WithdrawIneligibleRewardAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.lb_pair,
        accounts.reward_vault,
        accounts.funder_token_account,
        accounts.bin_array,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn withdraw_ineligible_reward_verify_signer_privileges<'me, 'info>(
    accounts: WithdrawIneligibleRewardAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.funder] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn withdraw_ineligible_reward_verify_account_privileges<'me, 'info>(
    accounts: WithdrawIneligibleRewardAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    withdraw_ineligible_reward_verify_writable_privileges(accounts)?;
    withdraw_ineligible_reward_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const SET_ACTIVATION_POINT_IX_ACCOUNTS_LEN: usize = 2;
#[derive(Copy, Clone, Debug)]
pub struct SetActivationPointAccounts<'me, 'info> {
    pub lb_pair: &'me AccountInfo<'info>,
    pub admin: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SetActivationPointKeys {
    pub lb_pair: Pubkey,
    pub admin: Pubkey,
}
impl From<SetActivationPointAccounts<'_, '_>> for SetActivationPointKeys {
    fn from(accounts: SetActivationPointAccounts) -> Self {
        Self {
            lb_pair: *accounts.lb_pair.key,
            admin: *accounts.admin.key,
        }
    }
}
impl From<SetActivationPointKeys>
for [AccountMeta; SET_ACTIVATION_POINT_IX_ACCOUNTS_LEN] {
    fn from(keys: SetActivationPointKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.lb_pair,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.admin,
                is_signer: true,
                is_writable: true,
            },
        ]
    }
}
impl From<[Pubkey; SET_ACTIVATION_POINT_IX_ACCOUNTS_LEN]> for SetActivationPointKeys {
    fn from(pubkeys: [Pubkey; SET_ACTIVATION_POINT_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            lb_pair: pubkeys[0],
            admin: pubkeys[1],
        }
    }
}
impl<'info> From<SetActivationPointAccounts<'_, 'info>>
for [AccountInfo<'info>; SET_ACTIVATION_POINT_IX_ACCOUNTS_LEN] {
    fn from(accounts: SetActivationPointAccounts<'_, 'info>) -> Self {
        [accounts.lb_pair.clone(), accounts.admin.clone()]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; SET_ACTIVATION_POINT_IX_ACCOUNTS_LEN]>
for SetActivationPointAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; SET_ACTIVATION_POINT_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            lb_pair: &arr[0],
            admin: &arr[1],
        }
    }
}
pub const SET_ACTIVATION_POINT_IX_DISCM: [u8; 8] = [91, 249, 15, 165, 26, 129, 254, 125];
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SetActivationPointIxArgs {
    pub activation_point: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct SetActivationPointIxData(pub SetActivationPointIxArgs);
impl From<SetActivationPointIxArgs> for SetActivationPointIxData {
    fn from(args: SetActivationPointIxArgs) -> Self {
        Self(args)
    }
}
impl SetActivationPointIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != SET_ACTIVATION_POINT_IX_DISCM {
            return Err(
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!(
                        "discm does not match. Expected: {:?}. Received: {:?}",
                        SET_ACTIVATION_POINT_IX_DISCM, maybe_discm
                    ),
                ),
            );
        }
        Ok(Self(SetActivationPointIxArgs::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&SET_ACTIVATION_POINT_IX_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn set_activation_point_ix_with_program_id(
    program_id: Pubkey,
    keys: SetActivationPointKeys,
    args: SetActivationPointIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; SET_ACTIVATION_POINT_IX_ACCOUNTS_LEN] = keys.into();
    let data: SetActivationPointIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn set_activation_point_ix(
    keys: SetActivationPointKeys,
    args: SetActivationPointIxArgs,
) -> std::io::Result<Instruction> {
    set_activation_point_ix_with_program_id(crate::ID, keys, args)
}
pub fn set_activation_point_invoke_with_program_id(
    program_id: Pubkey,
    accounts: SetActivationPointAccounts<'_, '_>,
    args: SetActivationPointIxArgs,
) -> ProgramResult {
    let keys: SetActivationPointKeys = accounts.into();
    let ix = set_activation_point_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn set_activation_point_invoke(
    accounts: SetActivationPointAccounts<'_, '_>,
    args: SetActivationPointIxArgs,
) -> ProgramResult {
    set_activation_point_invoke_with_program_id(crate::ID, accounts, args)
}
pub fn set_activation_point_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: SetActivationPointAccounts<'_, '_>,
    args: SetActivationPointIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: SetActivationPointKeys = accounts.into();
    let ix = set_activation_point_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn set_activation_point_invoke_signed(
    accounts: SetActivationPointAccounts<'_, '_>,
    args: SetActivationPointIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    set_activation_point_invoke_signed_with_program_id(crate::ID, accounts, args, seeds)
}
pub fn set_activation_point_verify_account_keys(
    accounts: SetActivationPointAccounts<'_, '_>,
    keys: SetActivationPointKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.lb_pair.key, keys.lb_pair),
        (*accounts.admin.key, keys.admin),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn set_activation_point_verify_writable_privileges<'me, 'info>(
    accounts: SetActivationPointAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.lb_pair, accounts.admin] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn set_activation_point_verify_signer_privileges<'me, 'info>(
    accounts: SetActivationPointAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.admin] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn set_activation_point_verify_account_privileges<'me, 'info>(
    accounts: SetActivationPointAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    set_activation_point_verify_writable_privileges(accounts)?;
    set_activation_point_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const REMOVE_LIQUIDITY_BY_RANGE_IX_ACCOUNTS_LEN: usize = 16;
#[derive(Copy, Clone, Debug)]
pub struct RemoveLiquidityByRangeAccounts<'me, 'info> {
    pub position: &'me AccountInfo<'info>,
    pub lb_pair: &'me AccountInfo<'info>,
    pub bin_array_bitmap_extension: &'me AccountInfo<'info>,
    pub user_token_x: &'me AccountInfo<'info>,
    pub user_token_y: &'me AccountInfo<'info>,
    pub reserve_x: &'me AccountInfo<'info>,
    pub reserve_y: &'me AccountInfo<'info>,
    pub token_x_mint: &'me AccountInfo<'info>,
    pub token_y_mint: &'me AccountInfo<'info>,
    pub bin_array_lower: &'me AccountInfo<'info>,
    pub bin_array_upper: &'me AccountInfo<'info>,
    pub sender: &'me AccountInfo<'info>,
    pub token_x_program: &'me AccountInfo<'info>,
    pub token_y_program: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct RemoveLiquidityByRangeKeys {
    pub position: Pubkey,
    pub lb_pair: Pubkey,
    pub bin_array_bitmap_extension: Pubkey,
    pub user_token_x: Pubkey,
    pub user_token_y: Pubkey,
    pub reserve_x: Pubkey,
    pub reserve_y: Pubkey,
    pub token_x_mint: Pubkey,
    pub token_y_mint: Pubkey,
    pub bin_array_lower: Pubkey,
    pub bin_array_upper: Pubkey,
    pub sender: Pubkey,
    pub token_x_program: Pubkey,
    pub token_y_program: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<RemoveLiquidityByRangeAccounts<'_, '_>> for RemoveLiquidityByRangeKeys {
    fn from(accounts: RemoveLiquidityByRangeAccounts) -> Self {
        Self {
            position: *accounts.position.key,
            lb_pair: *accounts.lb_pair.key,
            bin_array_bitmap_extension: *accounts.bin_array_bitmap_extension.key,
            user_token_x: *accounts.user_token_x.key,
            user_token_y: *accounts.user_token_y.key,
            reserve_x: *accounts.reserve_x.key,
            reserve_y: *accounts.reserve_y.key,
            token_x_mint: *accounts.token_x_mint.key,
            token_y_mint: *accounts.token_y_mint.key,
            bin_array_lower: *accounts.bin_array_lower.key,
            bin_array_upper: *accounts.bin_array_upper.key,
            sender: *accounts.sender.key,
            token_x_program: *accounts.token_x_program.key,
            token_y_program: *accounts.token_y_program.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<RemoveLiquidityByRangeKeys>
for [AccountMeta; REMOVE_LIQUIDITY_BY_RANGE_IX_ACCOUNTS_LEN] {
    fn from(keys: RemoveLiquidityByRangeKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.position,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.lb_pair,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.bin_array_bitmap_extension,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_token_x,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_token_y,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.reserve_x,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.reserve_y,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_x_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_y_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.bin_array_lower,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.bin_array_upper,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.sender,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_x_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_y_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.event_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; REMOVE_LIQUIDITY_BY_RANGE_IX_ACCOUNTS_LEN]>
for RemoveLiquidityByRangeKeys {
    fn from(pubkeys: [Pubkey; REMOVE_LIQUIDITY_BY_RANGE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            position: pubkeys[0],
            lb_pair: pubkeys[1],
            bin_array_bitmap_extension: pubkeys[2],
            user_token_x: pubkeys[3],
            user_token_y: pubkeys[4],
            reserve_x: pubkeys[5],
            reserve_y: pubkeys[6],
            token_x_mint: pubkeys[7],
            token_y_mint: pubkeys[8],
            bin_array_lower: pubkeys[9],
            bin_array_upper: pubkeys[10],
            sender: pubkeys[11],
            token_x_program: pubkeys[12],
            token_y_program: pubkeys[13],
            event_authority: pubkeys[14],
            program: pubkeys[15],
        }
    }
}
impl<'info> From<RemoveLiquidityByRangeAccounts<'_, 'info>>
for [AccountInfo<'info>; REMOVE_LIQUIDITY_BY_RANGE_IX_ACCOUNTS_LEN] {
    fn from(accounts: RemoveLiquidityByRangeAccounts<'_, 'info>) -> Self {
        [
            accounts.position.clone(),
            accounts.lb_pair.clone(),
            accounts.bin_array_bitmap_extension.clone(),
            accounts.user_token_x.clone(),
            accounts.user_token_y.clone(),
            accounts.reserve_x.clone(),
            accounts.reserve_y.clone(),
            accounts.token_x_mint.clone(),
            accounts.token_y_mint.clone(),
            accounts.bin_array_lower.clone(),
            accounts.bin_array_upper.clone(),
            accounts.sender.clone(),
            accounts.token_x_program.clone(),
            accounts.token_y_program.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; REMOVE_LIQUIDITY_BY_RANGE_IX_ACCOUNTS_LEN]>
for RemoveLiquidityByRangeAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; REMOVE_LIQUIDITY_BY_RANGE_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            position: &arr[0],
            lb_pair: &arr[1],
            bin_array_bitmap_extension: &arr[2],
            user_token_x: &arr[3],
            user_token_y: &arr[4],
            reserve_x: &arr[5],
            reserve_y: &arr[6],
            token_x_mint: &arr[7],
            token_y_mint: &arr[8],
            bin_array_lower: &arr[9],
            bin_array_upper: &arr[10],
            sender: &arr[11],
            token_x_program: &arr[12],
            token_y_program: &arr[13],
            event_authority: &arr[14],
            program: &arr[15],
        }
    }
}
pub const REMOVE_LIQUIDITY_BY_RANGE_IX_DISCM: [u8; 8] = [
    26,
    82,
    102,
    152,
    240,
    74,
    105,
    26,
];
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RemoveLiquidityByRangeIxArgs {
    pub from_bin_id: i32,
    pub to_bin_id: i32,
    pub bps_to_remove: u16,
}
#[derive(Clone, Debug, PartialEq)]
pub struct RemoveLiquidityByRangeIxData(pub RemoveLiquidityByRangeIxArgs);
impl From<RemoveLiquidityByRangeIxArgs> for RemoveLiquidityByRangeIxData {
    fn from(args: RemoveLiquidityByRangeIxArgs) -> Self {
        Self(args)
    }
}
impl RemoveLiquidityByRangeIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != REMOVE_LIQUIDITY_BY_RANGE_IX_DISCM {
            return Err(
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!(
                        "discm does not match. Expected: {:?}. Received: {:?}",
                        REMOVE_LIQUIDITY_BY_RANGE_IX_DISCM, maybe_discm
                    ),
                ),
            );
        }
        Ok(Self(RemoveLiquidityByRangeIxArgs::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&REMOVE_LIQUIDITY_BY_RANGE_IX_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn remove_liquidity_by_range_ix_with_program_id(
    program_id: Pubkey,
    keys: RemoveLiquidityByRangeKeys,
    args: RemoveLiquidityByRangeIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; REMOVE_LIQUIDITY_BY_RANGE_IX_ACCOUNTS_LEN] = keys.into();
    let data: RemoveLiquidityByRangeIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn remove_liquidity_by_range_ix(
    keys: RemoveLiquidityByRangeKeys,
    args: RemoveLiquidityByRangeIxArgs,
) -> std::io::Result<Instruction> {
    remove_liquidity_by_range_ix_with_program_id(crate::ID, keys, args)
}
pub fn remove_liquidity_by_range_invoke_with_program_id(
    program_id: Pubkey,
    accounts: RemoveLiquidityByRangeAccounts<'_, '_>,
    args: RemoveLiquidityByRangeIxArgs,
) -> ProgramResult {
    let keys: RemoveLiquidityByRangeKeys = accounts.into();
    let ix = remove_liquidity_by_range_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn remove_liquidity_by_range_invoke(
    accounts: RemoveLiquidityByRangeAccounts<'_, '_>,
    args: RemoveLiquidityByRangeIxArgs,
) -> ProgramResult {
    remove_liquidity_by_range_invoke_with_program_id(crate::ID, accounts, args)
}
pub fn remove_liquidity_by_range_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: RemoveLiquidityByRangeAccounts<'_, '_>,
    args: RemoveLiquidityByRangeIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: RemoveLiquidityByRangeKeys = accounts.into();
    let ix = remove_liquidity_by_range_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn remove_liquidity_by_range_invoke_signed(
    accounts: RemoveLiquidityByRangeAccounts<'_, '_>,
    args: RemoveLiquidityByRangeIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    remove_liquidity_by_range_invoke_signed_with_program_id(
        crate::ID,
        accounts,
        args,
        seeds,
    )
}
pub fn remove_liquidity_by_range_verify_account_keys(
    accounts: RemoveLiquidityByRangeAccounts<'_, '_>,
    keys: RemoveLiquidityByRangeKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.position.key, keys.position),
        (*accounts.lb_pair.key, keys.lb_pair),
        (*accounts.bin_array_bitmap_extension.key, keys.bin_array_bitmap_extension),
        (*accounts.user_token_x.key, keys.user_token_x),
        (*accounts.user_token_y.key, keys.user_token_y),
        (*accounts.reserve_x.key, keys.reserve_x),
        (*accounts.reserve_y.key, keys.reserve_y),
        (*accounts.token_x_mint.key, keys.token_x_mint),
        (*accounts.token_y_mint.key, keys.token_y_mint),
        (*accounts.bin_array_lower.key, keys.bin_array_lower),
        (*accounts.bin_array_upper.key, keys.bin_array_upper),
        (*accounts.sender.key, keys.sender),
        (*accounts.token_x_program.key, keys.token_x_program),
        (*accounts.token_y_program.key, keys.token_y_program),
        (*accounts.event_authority.key, keys.event_authority),
        (*accounts.program.key, keys.program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn remove_liquidity_by_range_verify_writable_privileges<'me, 'info>(
    accounts: RemoveLiquidityByRangeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.position,
        accounts.lb_pair,
        accounts.bin_array_bitmap_extension,
        accounts.user_token_x,
        accounts.user_token_y,
        accounts.reserve_x,
        accounts.reserve_y,
        accounts.bin_array_lower,
        accounts.bin_array_upper,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn remove_liquidity_by_range_verify_signer_privileges<'me, 'info>(
    accounts: RemoveLiquidityByRangeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.sender] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn remove_liquidity_by_range_verify_account_privileges<'me, 'info>(
    accounts: RemoveLiquidityByRangeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    remove_liquidity_by_range_verify_writable_privileges(accounts)?;
    remove_liquidity_by_range_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const ADD_LIQUIDITY_ONE_SIDE_PRECISE_IX_ACCOUNTS_LEN: usize = 12;
#[derive(Copy, Clone, Debug)]
pub struct AddLiquidityOneSidePreciseAccounts<'me, 'info> {
    pub position: &'me AccountInfo<'info>,
    pub lb_pair: &'me AccountInfo<'info>,
    pub bin_array_bitmap_extension: &'me AccountInfo<'info>,
    pub user_token: &'me AccountInfo<'info>,
    pub reserve: &'me AccountInfo<'info>,
    pub token_mint: &'me AccountInfo<'info>,
    pub bin_array_lower: &'me AccountInfo<'info>,
    pub bin_array_upper: &'me AccountInfo<'info>,
    pub sender: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct AddLiquidityOneSidePreciseKeys {
    pub position: Pubkey,
    pub lb_pair: Pubkey,
    pub bin_array_bitmap_extension: Pubkey,
    pub user_token: Pubkey,
    pub reserve: Pubkey,
    pub token_mint: Pubkey,
    pub bin_array_lower: Pubkey,
    pub bin_array_upper: Pubkey,
    pub sender: Pubkey,
    pub token_program: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<AddLiquidityOneSidePreciseAccounts<'_, '_>>
for AddLiquidityOneSidePreciseKeys {
    fn from(accounts: AddLiquidityOneSidePreciseAccounts) -> Self {
        Self {
            position: *accounts.position.key,
            lb_pair: *accounts.lb_pair.key,
            bin_array_bitmap_extension: *accounts.bin_array_bitmap_extension.key,
            user_token: *accounts.user_token.key,
            reserve: *accounts.reserve.key,
            token_mint: *accounts.token_mint.key,
            bin_array_lower: *accounts.bin_array_lower.key,
            bin_array_upper: *accounts.bin_array_upper.key,
            sender: *accounts.sender.key,
            token_program: *accounts.token_program.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<AddLiquidityOneSidePreciseKeys>
for [AccountMeta; ADD_LIQUIDITY_ONE_SIDE_PRECISE_IX_ACCOUNTS_LEN] {
    fn from(keys: AddLiquidityOneSidePreciseKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.position,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.lb_pair,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.bin_array_bitmap_extension,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_token,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.reserve,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.bin_array_lower,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.bin_array_upper,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.sender,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.event_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; ADD_LIQUIDITY_ONE_SIDE_PRECISE_IX_ACCOUNTS_LEN]>
for AddLiquidityOneSidePreciseKeys {
    fn from(pubkeys: [Pubkey; ADD_LIQUIDITY_ONE_SIDE_PRECISE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            position: pubkeys[0],
            lb_pair: pubkeys[1],
            bin_array_bitmap_extension: pubkeys[2],
            user_token: pubkeys[3],
            reserve: pubkeys[4],
            token_mint: pubkeys[5],
            bin_array_lower: pubkeys[6],
            bin_array_upper: pubkeys[7],
            sender: pubkeys[8],
            token_program: pubkeys[9],
            event_authority: pubkeys[10],
            program: pubkeys[11],
        }
    }
}
impl<'info> From<AddLiquidityOneSidePreciseAccounts<'_, 'info>>
for [AccountInfo<'info>; ADD_LIQUIDITY_ONE_SIDE_PRECISE_IX_ACCOUNTS_LEN] {
    fn from(accounts: AddLiquidityOneSidePreciseAccounts<'_, 'info>) -> Self {
        [
            accounts.position.clone(),
            accounts.lb_pair.clone(),
            accounts.bin_array_bitmap_extension.clone(),
            accounts.user_token.clone(),
            accounts.reserve.clone(),
            accounts.token_mint.clone(),
            accounts.bin_array_lower.clone(),
            accounts.bin_array_upper.clone(),
            accounts.sender.clone(),
            accounts.token_program.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; ADD_LIQUIDITY_ONE_SIDE_PRECISE_IX_ACCOUNTS_LEN]>
for AddLiquidityOneSidePreciseAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; ADD_LIQUIDITY_ONE_SIDE_PRECISE_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            position: &arr[0],
            lb_pair: &arr[1],
            bin_array_bitmap_extension: &arr[2],
            user_token: &arr[3],
            reserve: &arr[4],
            token_mint: &arr[5],
            bin_array_lower: &arr[6],
            bin_array_upper: &arr[7],
            sender: &arr[8],
            token_program: &arr[9],
            event_authority: &arr[10],
            program: &arr[11],
        }
    }
}
pub const ADD_LIQUIDITY_ONE_SIDE_PRECISE_IX_DISCM: [u8; 8] = [
    161,
    194,
    103,
    84,
    171,
    71,
    250,
    154,
];
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct AddLiquidityOneSidePreciseIxArgs {
    pub parameter: AddLiquiditySingleSidePreciseParameter,
}
#[derive(Clone, Debug, PartialEq)]
pub struct AddLiquidityOneSidePreciseIxData(pub AddLiquidityOneSidePreciseIxArgs);
impl From<AddLiquidityOneSidePreciseIxArgs> for AddLiquidityOneSidePreciseIxData {
    fn from(args: AddLiquidityOneSidePreciseIxArgs) -> Self {
        Self(args)
    }
}
impl AddLiquidityOneSidePreciseIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != ADD_LIQUIDITY_ONE_SIDE_PRECISE_IX_DISCM {
            return Err(
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!(
                        "discm does not match. Expected: {:?}. Received: {:?}",
                        ADD_LIQUIDITY_ONE_SIDE_PRECISE_IX_DISCM, maybe_discm
                    ),
                ),
            );
        }
        Ok(Self(AddLiquidityOneSidePreciseIxArgs::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&ADD_LIQUIDITY_ONE_SIDE_PRECISE_IX_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn add_liquidity_one_side_precise_ix_with_program_id(
    program_id: Pubkey,
    keys: AddLiquidityOneSidePreciseKeys,
    args: AddLiquidityOneSidePreciseIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; ADD_LIQUIDITY_ONE_SIDE_PRECISE_IX_ACCOUNTS_LEN] = keys
        .into();
    let data: AddLiquidityOneSidePreciseIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn add_liquidity_one_side_precise_ix(
    keys: AddLiquidityOneSidePreciseKeys,
    args: AddLiquidityOneSidePreciseIxArgs,
) -> std::io::Result<Instruction> {
    add_liquidity_one_side_precise_ix_with_program_id(crate::ID, keys, args)
}
pub fn add_liquidity_one_side_precise_invoke_with_program_id(
    program_id: Pubkey,
    accounts: AddLiquidityOneSidePreciseAccounts<'_, '_>,
    args: AddLiquidityOneSidePreciseIxArgs,
) -> ProgramResult {
    let keys: AddLiquidityOneSidePreciseKeys = accounts.into();
    let ix = add_liquidity_one_side_precise_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn add_liquidity_one_side_precise_invoke(
    accounts: AddLiquidityOneSidePreciseAccounts<'_, '_>,
    args: AddLiquidityOneSidePreciseIxArgs,
) -> ProgramResult {
    add_liquidity_one_side_precise_invoke_with_program_id(crate::ID, accounts, args)
}
pub fn add_liquidity_one_side_precise_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: AddLiquidityOneSidePreciseAccounts<'_, '_>,
    args: AddLiquidityOneSidePreciseIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: AddLiquidityOneSidePreciseKeys = accounts.into();
    let ix = add_liquidity_one_side_precise_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn add_liquidity_one_side_precise_invoke_signed(
    accounts: AddLiquidityOneSidePreciseAccounts<'_, '_>,
    args: AddLiquidityOneSidePreciseIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    add_liquidity_one_side_precise_invoke_signed_with_program_id(
        crate::ID,
        accounts,
        args,
        seeds,
    )
}
pub fn add_liquidity_one_side_precise_verify_account_keys(
    accounts: AddLiquidityOneSidePreciseAccounts<'_, '_>,
    keys: AddLiquidityOneSidePreciseKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.position.key, keys.position),
        (*accounts.lb_pair.key, keys.lb_pair),
        (*accounts.bin_array_bitmap_extension.key, keys.bin_array_bitmap_extension),
        (*accounts.user_token.key, keys.user_token),
        (*accounts.reserve.key, keys.reserve),
        (*accounts.token_mint.key, keys.token_mint),
        (*accounts.bin_array_lower.key, keys.bin_array_lower),
        (*accounts.bin_array_upper.key, keys.bin_array_upper),
        (*accounts.sender.key, keys.sender),
        (*accounts.token_program.key, keys.token_program),
        (*accounts.event_authority.key, keys.event_authority),
        (*accounts.program.key, keys.program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn add_liquidity_one_side_precise_verify_writable_privileges<'me, 'info>(
    accounts: AddLiquidityOneSidePreciseAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.position,
        accounts.lb_pair,
        accounts.bin_array_bitmap_extension,
        accounts.user_token,
        accounts.reserve,
        accounts.bin_array_lower,
        accounts.bin_array_upper,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn add_liquidity_one_side_precise_verify_signer_privileges<'me, 'info>(
    accounts: AddLiquidityOneSidePreciseAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.sender] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn add_liquidity_one_side_precise_verify_account_privileges<'me, 'info>(
    accounts: AddLiquidityOneSidePreciseAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    add_liquidity_one_side_precise_verify_writable_privileges(accounts)?;
    add_liquidity_one_side_precise_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const GO_TO_A_BIN_IX_ACCOUNTS_LEN: usize = 6;
#[derive(Copy, Clone, Debug)]
pub struct GoToABinAccounts<'me, 'info> {
    pub lb_pair: &'me AccountInfo<'info>,
    pub bin_array_bitmap_extension: &'me AccountInfo<'info>,
    pub from_bin_array: &'me AccountInfo<'info>,
    pub to_bin_array: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoToABinKeys {
    pub lb_pair: Pubkey,
    pub bin_array_bitmap_extension: Pubkey,
    pub from_bin_array: Pubkey,
    pub to_bin_array: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<GoToABinAccounts<'_, '_>> for GoToABinKeys {
    fn from(accounts: GoToABinAccounts) -> Self {
        Self {
            lb_pair: *accounts.lb_pair.key,
            bin_array_bitmap_extension: *accounts.bin_array_bitmap_extension.key,
            from_bin_array: *accounts.from_bin_array.key,
            to_bin_array: *accounts.to_bin_array.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<GoToABinKeys> for [AccountMeta; GO_TO_A_BIN_IX_ACCOUNTS_LEN] {
    fn from(keys: GoToABinKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.lb_pair,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.bin_array_bitmap_extension,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.from_bin_array,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.to_bin_array,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.event_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; GO_TO_A_BIN_IX_ACCOUNTS_LEN]> for GoToABinKeys {
    fn from(pubkeys: [Pubkey; GO_TO_A_BIN_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            lb_pair: pubkeys[0],
            bin_array_bitmap_extension: pubkeys[1],
            from_bin_array: pubkeys[2],
            to_bin_array: pubkeys[3],
            event_authority: pubkeys[4],
            program: pubkeys[5],
        }
    }
}
impl<'info> From<GoToABinAccounts<'_, 'info>>
for [AccountInfo<'info>; GO_TO_A_BIN_IX_ACCOUNTS_LEN] {
    fn from(accounts: GoToABinAccounts<'_, 'info>) -> Self {
        [
            accounts.lb_pair.clone(),
            accounts.bin_array_bitmap_extension.clone(),
            accounts.from_bin_array.clone(),
            accounts.to_bin_array.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; GO_TO_A_BIN_IX_ACCOUNTS_LEN]>
for GoToABinAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; GO_TO_A_BIN_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            lb_pair: &arr[0],
            bin_array_bitmap_extension: &arr[1],
            from_bin_array: &arr[2],
            to_bin_array: &arr[3],
            event_authority: &arr[4],
            program: &arr[5],
        }
    }
}
pub const GO_TO_A_BIN_IX_DISCM: [u8; 8] = [146, 72, 174, 224, 40, 253, 84, 174];
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct GoToABinIxArgs {
    pub bin_id: i32,
}
#[derive(Clone, Debug, PartialEq)]
pub struct GoToABinIxData(pub GoToABinIxArgs);
impl From<GoToABinIxArgs> for GoToABinIxData {
    fn from(args: GoToABinIxArgs) -> Self {
        Self(args)
    }
}
impl GoToABinIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != GO_TO_A_BIN_IX_DISCM {
            return Err(
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!(
                        "discm does not match. Expected: {:?}. Received: {:?}",
                        GO_TO_A_BIN_IX_DISCM, maybe_discm
                    ),
                ),
            );
        }
        Ok(Self(GoToABinIxArgs::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&GO_TO_A_BIN_IX_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn go_to_a_bin_ix_with_program_id(
    program_id: Pubkey,
    keys: GoToABinKeys,
    args: GoToABinIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; GO_TO_A_BIN_IX_ACCOUNTS_LEN] = keys.into();
    let data: GoToABinIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn go_to_a_bin_ix(
    keys: GoToABinKeys,
    args: GoToABinIxArgs,
) -> std::io::Result<Instruction> {
    go_to_a_bin_ix_with_program_id(crate::ID, keys, args)
}
pub fn go_to_a_bin_invoke_with_program_id(
    program_id: Pubkey,
    accounts: GoToABinAccounts<'_, '_>,
    args: GoToABinIxArgs,
) -> ProgramResult {
    let keys: GoToABinKeys = accounts.into();
    let ix = go_to_a_bin_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn go_to_a_bin_invoke(
    accounts: GoToABinAccounts<'_, '_>,
    args: GoToABinIxArgs,
) -> ProgramResult {
    go_to_a_bin_invoke_with_program_id(crate::ID, accounts, args)
}
pub fn go_to_a_bin_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: GoToABinAccounts<'_, '_>,
    args: GoToABinIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: GoToABinKeys = accounts.into();
    let ix = go_to_a_bin_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn go_to_a_bin_invoke_signed(
    accounts: GoToABinAccounts<'_, '_>,
    args: GoToABinIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    go_to_a_bin_invoke_signed_with_program_id(crate::ID, accounts, args, seeds)
}
pub fn go_to_a_bin_verify_account_keys(
    accounts: GoToABinAccounts<'_, '_>,
    keys: GoToABinKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.lb_pair.key, keys.lb_pair),
        (*accounts.bin_array_bitmap_extension.key, keys.bin_array_bitmap_extension),
        (*accounts.from_bin_array.key, keys.from_bin_array),
        (*accounts.to_bin_array.key, keys.to_bin_array),
        (*accounts.event_authority.key, keys.event_authority),
        (*accounts.program.key, keys.program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn go_to_a_bin_verify_writable_privileges<'me, 'info>(
    accounts: GoToABinAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.lb_pair] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn go_to_a_bin_verify_account_privileges<'me, 'info>(
    accounts: GoToABinAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    go_to_a_bin_verify_writable_privileges(accounts)?;
    Ok(())
}
pub const SET_PRE_ACTIVATION_DURATION_IX_ACCOUNTS_LEN: usize = 2;
#[derive(Copy, Clone, Debug)]
pub struct SetPreActivationDurationAccounts<'me, 'info> {
    pub lb_pair: &'me AccountInfo<'info>,
    pub creator: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SetPreActivationDurationKeys {
    pub lb_pair: Pubkey,
    pub creator: Pubkey,
}
impl From<SetPreActivationDurationAccounts<'_, '_>> for SetPreActivationDurationKeys {
    fn from(accounts: SetPreActivationDurationAccounts) -> Self {
        Self {
            lb_pair: *accounts.lb_pair.key,
            creator: *accounts.creator.key,
        }
    }
}
impl From<SetPreActivationDurationKeys>
for [AccountMeta; SET_PRE_ACTIVATION_DURATION_IX_ACCOUNTS_LEN] {
    fn from(keys: SetPreActivationDurationKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.lb_pair,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.creator,
                is_signer: true,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; SET_PRE_ACTIVATION_DURATION_IX_ACCOUNTS_LEN]>
for SetPreActivationDurationKeys {
    fn from(pubkeys: [Pubkey; SET_PRE_ACTIVATION_DURATION_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            lb_pair: pubkeys[0],
            creator: pubkeys[1],
        }
    }
}
impl<'info> From<SetPreActivationDurationAccounts<'_, 'info>>
for [AccountInfo<'info>; SET_PRE_ACTIVATION_DURATION_IX_ACCOUNTS_LEN] {
    fn from(accounts: SetPreActivationDurationAccounts<'_, 'info>) -> Self {
        [accounts.lb_pair.clone(), accounts.creator.clone()]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; SET_PRE_ACTIVATION_DURATION_IX_ACCOUNTS_LEN]>
for SetPreActivationDurationAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; SET_PRE_ACTIVATION_DURATION_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            lb_pair: &arr[0],
            creator: &arr[1],
        }
    }
}
pub const SET_PRE_ACTIVATION_DURATION_IX_DISCM: [u8; 8] = [
    165,
    61,
    201,
    244,
    130,
    159,
    22,
    100,
];
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SetPreActivationDurationIxArgs {
    pub pre_activation_duration: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct SetPreActivationDurationIxData(pub SetPreActivationDurationIxArgs);
impl From<SetPreActivationDurationIxArgs> for SetPreActivationDurationIxData {
    fn from(args: SetPreActivationDurationIxArgs) -> Self {
        Self(args)
    }
}
impl SetPreActivationDurationIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != SET_PRE_ACTIVATION_DURATION_IX_DISCM {
            return Err(
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!(
                        "discm does not match. Expected: {:?}. Received: {:?}",
                        SET_PRE_ACTIVATION_DURATION_IX_DISCM, maybe_discm
                    ),
                ),
            );
        }
        Ok(Self(SetPreActivationDurationIxArgs::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&SET_PRE_ACTIVATION_DURATION_IX_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn set_pre_activation_duration_ix_with_program_id(
    program_id: Pubkey,
    keys: SetPreActivationDurationKeys,
    args: SetPreActivationDurationIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; SET_PRE_ACTIVATION_DURATION_IX_ACCOUNTS_LEN] = keys.into();
    let data: SetPreActivationDurationIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn set_pre_activation_duration_ix(
    keys: SetPreActivationDurationKeys,
    args: SetPreActivationDurationIxArgs,
) -> std::io::Result<Instruction> {
    set_pre_activation_duration_ix_with_program_id(crate::ID, keys, args)
}
pub fn set_pre_activation_duration_invoke_with_program_id(
    program_id: Pubkey,
    accounts: SetPreActivationDurationAccounts<'_, '_>,
    args: SetPreActivationDurationIxArgs,
) -> ProgramResult {
    let keys: SetPreActivationDurationKeys = accounts.into();
    let ix = set_pre_activation_duration_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn set_pre_activation_duration_invoke(
    accounts: SetPreActivationDurationAccounts<'_, '_>,
    args: SetPreActivationDurationIxArgs,
) -> ProgramResult {
    set_pre_activation_duration_invoke_with_program_id(crate::ID, accounts, args)
}
pub fn set_pre_activation_duration_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: SetPreActivationDurationAccounts<'_, '_>,
    args: SetPreActivationDurationIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: SetPreActivationDurationKeys = accounts.into();
    let ix = set_pre_activation_duration_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn set_pre_activation_duration_invoke_signed(
    accounts: SetPreActivationDurationAccounts<'_, '_>,
    args: SetPreActivationDurationIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    set_pre_activation_duration_invoke_signed_with_program_id(
        crate::ID,
        accounts,
        args,
        seeds,
    )
}
pub fn set_pre_activation_duration_verify_account_keys(
    accounts: SetPreActivationDurationAccounts<'_, '_>,
    keys: SetPreActivationDurationKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.lb_pair.key, keys.lb_pair),
        (*accounts.creator.key, keys.creator),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn set_pre_activation_duration_verify_writable_privileges<'me, 'info>(
    accounts: SetPreActivationDurationAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.lb_pair] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn set_pre_activation_duration_verify_signer_privileges<'me, 'info>(
    accounts: SetPreActivationDurationAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.creator] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn set_pre_activation_duration_verify_account_privileges<'me, 'info>(
    accounts: SetPreActivationDurationAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    set_pre_activation_duration_verify_writable_privileges(accounts)?;
    set_pre_activation_duration_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const SET_PRE_ACTIVATION_SWAP_ADDRESS_IX_ACCOUNTS_LEN: usize = 2;
#[derive(Copy, Clone, Debug)]
pub struct SetPreActivationSwapAddressAccounts<'me, 'info> {
    pub lb_pair: &'me AccountInfo<'info>,
    pub creator: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SetPreActivationSwapAddressKeys {
    pub lb_pair: Pubkey,
    pub creator: Pubkey,
}
impl From<SetPreActivationSwapAddressAccounts<'_, '_>>
for SetPreActivationSwapAddressKeys {
    fn from(accounts: SetPreActivationSwapAddressAccounts) -> Self {
        Self {
            lb_pair: *accounts.lb_pair.key,
            creator: *accounts.creator.key,
        }
    }
}
impl From<SetPreActivationSwapAddressKeys>
for [AccountMeta; SET_PRE_ACTIVATION_SWAP_ADDRESS_IX_ACCOUNTS_LEN] {
    fn from(keys: SetPreActivationSwapAddressKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.lb_pair,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.creator,
                is_signer: true,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; SET_PRE_ACTIVATION_SWAP_ADDRESS_IX_ACCOUNTS_LEN]>
for SetPreActivationSwapAddressKeys {
    fn from(pubkeys: [Pubkey; SET_PRE_ACTIVATION_SWAP_ADDRESS_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            lb_pair: pubkeys[0],
            creator: pubkeys[1],
        }
    }
}
impl<'info> From<SetPreActivationSwapAddressAccounts<'_, 'info>>
for [AccountInfo<'info>; SET_PRE_ACTIVATION_SWAP_ADDRESS_IX_ACCOUNTS_LEN] {
    fn from(accounts: SetPreActivationSwapAddressAccounts<'_, 'info>) -> Self {
        [accounts.lb_pair.clone(), accounts.creator.clone()]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; SET_PRE_ACTIVATION_SWAP_ADDRESS_IX_ACCOUNTS_LEN]>
for SetPreActivationSwapAddressAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; SET_PRE_ACTIVATION_SWAP_ADDRESS_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            lb_pair: &arr[0],
            creator: &arr[1],
        }
    }
}
pub const SET_PRE_ACTIVATION_SWAP_ADDRESS_IX_DISCM: [u8; 8] = [
    57,
    139,
    47,
    123,
    216,
    80,
    223,
    10,
];
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SetPreActivationSwapAddressIxArgs {
    pub pre_activation_swap_address: Pubkey,
}
#[derive(Clone, Debug, PartialEq)]
pub struct SetPreActivationSwapAddressIxData(pub SetPreActivationSwapAddressIxArgs);
impl From<SetPreActivationSwapAddressIxArgs> for SetPreActivationSwapAddressIxData {
    fn from(args: SetPreActivationSwapAddressIxArgs) -> Self {
        Self(args)
    }
}
impl SetPreActivationSwapAddressIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != SET_PRE_ACTIVATION_SWAP_ADDRESS_IX_DISCM {
            return Err(
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!(
                        "discm does not match. Expected: {:?}. Received: {:?}",
                        SET_PRE_ACTIVATION_SWAP_ADDRESS_IX_DISCM, maybe_discm
                    ),
                ),
            );
        }
        Ok(Self(SetPreActivationSwapAddressIxArgs::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&SET_PRE_ACTIVATION_SWAP_ADDRESS_IX_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn set_pre_activation_swap_address_ix_with_program_id(
    program_id: Pubkey,
    keys: SetPreActivationSwapAddressKeys,
    args: SetPreActivationSwapAddressIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; SET_PRE_ACTIVATION_SWAP_ADDRESS_IX_ACCOUNTS_LEN] = keys
        .into();
    let data: SetPreActivationSwapAddressIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn set_pre_activation_swap_address_ix(
    keys: SetPreActivationSwapAddressKeys,
    args: SetPreActivationSwapAddressIxArgs,
) -> std::io::Result<Instruction> {
    set_pre_activation_swap_address_ix_with_program_id(crate::ID, keys, args)
}
pub fn set_pre_activation_swap_address_invoke_with_program_id(
    program_id: Pubkey,
    accounts: SetPreActivationSwapAddressAccounts<'_, '_>,
    args: SetPreActivationSwapAddressIxArgs,
) -> ProgramResult {
    let keys: SetPreActivationSwapAddressKeys = accounts.into();
    let ix = set_pre_activation_swap_address_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn set_pre_activation_swap_address_invoke(
    accounts: SetPreActivationSwapAddressAccounts<'_, '_>,
    args: SetPreActivationSwapAddressIxArgs,
) -> ProgramResult {
    set_pre_activation_swap_address_invoke_with_program_id(crate::ID, accounts, args)
}
pub fn set_pre_activation_swap_address_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: SetPreActivationSwapAddressAccounts<'_, '_>,
    args: SetPreActivationSwapAddressIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: SetPreActivationSwapAddressKeys = accounts.into();
    let ix = set_pre_activation_swap_address_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn set_pre_activation_swap_address_invoke_signed(
    accounts: SetPreActivationSwapAddressAccounts<'_, '_>,
    args: SetPreActivationSwapAddressIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    set_pre_activation_swap_address_invoke_signed_with_program_id(
        crate::ID,
        accounts,
        args,
        seeds,
    )
}
pub fn set_pre_activation_swap_address_verify_account_keys(
    accounts: SetPreActivationSwapAddressAccounts<'_, '_>,
    keys: SetPreActivationSwapAddressKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.lb_pair.key, keys.lb_pair),
        (*accounts.creator.key, keys.creator),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn set_pre_activation_swap_address_verify_writable_privileges<'me, 'info>(
    accounts: SetPreActivationSwapAddressAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.lb_pair] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn set_pre_activation_swap_address_verify_signer_privileges<'me, 'info>(
    accounts: SetPreActivationSwapAddressAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.creator] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn set_pre_activation_swap_address_verify_account_privileges<'me, 'info>(
    accounts: SetPreActivationSwapAddressAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    set_pre_activation_swap_address_verify_writable_privileges(accounts)?;
    set_pre_activation_swap_address_verify_signer_privileges(accounts)?;
    Ok(())
}
