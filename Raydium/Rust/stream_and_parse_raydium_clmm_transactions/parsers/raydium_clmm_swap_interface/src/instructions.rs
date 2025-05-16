#[cfg(feature = "serde")]
use std::fmt;
use crate::*;
use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::AccountInfo, entrypoint::ProgramResult,
    instruction::{AccountMeta, Instruction},
    program::{invoke, invoke_signed},
    pubkey::Pubkey, program_error::ProgramError,
};
use typedefs::{
    InitializeRewardParam
};
use inflector::Inflector;
use std::io::{Read,Cursor};
use strum_macros::{Display, EnumString};

#[derive(Clone, Debug, PartialEq, EnumString, Display)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum RaydiumClmmSwapProgramIx {
    CreateAmmConfig(CreateAmmConfigIxArgs),
    UpdateAmmConfig(UpdateAmmConfigIxArgs),
    CreatePool(CreatePoolIxArgs),
    UpdatePoolStatus(UpdatePoolStatusIxArgs),
    CreateOperationAccount,
    UpdateOperationAccount(UpdateOperationAccountIxArgs),
    TransferRewardOwner(TransferRewardOwnerIxArgs),
    InitializeReward(InitializeRewardIxArgs),
    CollectRemainingRewards(CollectRemainingRewardsIxArgs),
    UpdateRewardInfos,
    SetRewardParams(SetRewardParamsIxArgs),
    CollectProtocolFee(CollectProtocolFeeIxArgs),
    CollectFundFee(CollectFundFeeIxArgs),
    OpenPosition(OpenPositionIxArgs),
    OpenPositionV2(OpenPositionV2IxArgs),
    OpenPositionWithToken22Nft(OpenPositionWithToken22NftIxArgs),
    ClosePosition,
    IncreaseLiquidity(IncreaseLiquidityIxArgs),
    IncreaseLiquidityV2(IncreaseLiquidityV2IxArgs),
    DecreaseLiquidity(DecreaseLiquidityIxArgs),
    DecreaseLiquidityV2(DecreaseLiquidityV2IxArgs),
    Swap(SwapIxArgs),
    SwapV2(SwapV2IxArgs),
    SwapRouterBaseIn(SwapRouterBaseInIxArgs),
}

impl RaydiumClmmSwapProgramIx {
    pub fn name(&self) -> String {
        self.to_string().to_camel_case()
    }
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm_buf = [0u8; 1];
        reader.read_exact(&mut maybe_discm_buf)?;
        let maybe_discm = maybe_discm_buf[0];
        match maybe_discm {
            CREATE_AMM_CONFIG_IX_DISCM => Ok(Self::CreateAmmConfig(CreateAmmConfigIxArgs::deserialize(&mut reader)?)),
            UPDATE_AMM_CONFIG_IX_DISCM => Ok(Self::UpdateAmmConfig(UpdateAmmConfigIxArgs::deserialize(&mut reader)?)),
            CREATE_POOL_IX_DISCM => Ok(Self::CreatePool(CreatePoolIxArgs::deserialize(&mut reader)?)),
            UPDATE_POOL_STATUS_IX_DISCM => Ok(Self::UpdatePoolStatus(UpdatePoolStatusIxArgs::deserialize(&mut reader)?)),
            CREATE_OPERATION_ACCOUNT_IX_DISCM => Ok(Self::CreateOperationAccount),
            UPDATE_OPERATION_ACCOUNT_IX_DISCM => Ok(Self::UpdateOperationAccount(UpdateOperationAccountIxArgs::deserialize(&mut reader)?)),
            TRANSFER_REWARD_OWNER_IX_DISCM => Ok(Self::TransferRewardOwner(TransferRewardOwnerIxArgs::deserialize(&mut reader)?)),
            INITIALIZE_REWARD_IX_DISCM => Ok(Self::InitializeReward(InitializeRewardIxArgs::deserialize(&mut reader)?)),
            COLLECT_REMAINING_REWARDS_IX_DISCM => Ok(Self::CollectRemainingRewards(CollectRemainingRewardsIxArgs::deserialize(&mut reader)?)),
            UPDATE_REWARD_INFOS_IX_DISCM => Ok(Self::UpdateRewardInfos),
            SET_REWARD_PARAMS_IX_DISCM => Ok(Self::SetRewardParams(SetRewardParamsIxArgs::deserialize(&mut reader)?)),
            CREATE_OPERATION_ACCOUNT_IX_DISCM => Ok(Self::CollectProtocolFee(CollectProtocolFeeIxArgs::deserialize(&mut reader)?)),
            COLLECT_FUND_FEE_IX_DISCM => Ok(Self::CollectFundFee(CollectFundFeeIxArgs::deserialize(&mut reader)?)),
            OPEN_POSITION_IX_DISCM => Ok(Self::OpenPosition(OpenPositionIxArgs::deserialize(&mut reader)?)),
            OPEN_POSITION_V2_IX_DISCM => Ok(Self::OpenPositionV2(OpenPositionV2IxArgs::deserialize(&mut reader)?)),
            OPEN_POSITION_WITH_TOKEN22_NFT_IX_DISCM => Ok(Self::OpenPositionWithToken22Nft(OpenPositionWithToken22NftIxArgs::deserialize(&mut reader)?)),
            CLOSE_POSITION_IX_DISCM => Ok(Self::ClosePosition),
            INCREASE_LIQUIDITY_IX_DISCM => Ok(Self::IncreaseLiquidity(IncreaseLiquidityIxArgs::deserialize(&mut reader)?)),
            INCREASE_LIQUIDITY_V2_IX_DISCM => Ok(Self::IncreaseLiquidityV2(IncreaseLiquidityV2IxArgs::deserialize(&mut reader)?)),
            DECREASE_LIQUIDITY_IX_DISCM => Ok(Self::DecreaseLiquidity(DecreaseLiquidityIxArgs::deserialize(&mut reader)?)),
            DECREASE_LIQUIDITY_V2_IX_DISCM => Ok(Self::DecreaseLiquidityV2(DecreaseLiquidityV2IxArgs::deserialize(&mut reader)?)),
            SWAP_IX_DISCM => Ok(Self::Swap(SwapIxArgs::deserialize(&mut reader)?)),
            SWAP_V2_IX_DISCM => Ok(Self::SwapV2(SwapV2IxArgs::deserialize(&mut reader)?)),
            SWAP_ROUTER_BASE_IN_IX_DISCM => Ok(Self::SwapRouterBaseIn(SwapRouterBaseInIxArgs::deserialize(&mut reader)?)),
            _ => Err(std::io::Error::new(std::io::ErrorKind::Other, format!("discm {:?} not found,", maybe_discm))),
        }
    }

    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        match self {
            Self::CreateAmmConfig(args) => {
                writer.write_all(&[CREATE_AMM_CONFIG_IX_DISCM])?;
                args.serialize(&mut writer)
            }
            Self::UpdateAmmConfig(args) => {
                writer.write_all(&[UPDATE_AMM_CONFIG_IX_DISCM])?;
                args.serialize(&mut writer)
            }
            Self::CreatePool(args) => {
                writer.write_all(&[CREATE_POOL_IX_DISCM])?;
                args.serialize(&mut writer)
            }
            Self::UpdatePoolStatus(args) => {
                writer.write_all(&[UPDATE_POOL_STATUS_IX_DISCM])?;
                args.serialize(&mut writer)
            }
            Self::CreateOperationAccount => {
                writer.write_all(&[CREATE_OPERATION_ACCOUNT_IX_DISCM])?;
                Ok(())
            }
            Self::UpdateOperationAccount(args) => {
                writer.write_all(&[UPDATE_OPERATION_ACCOUNT_IX_DISCM])?;
                args.serialize(&mut writer)
            }
            Self::TransferRewardOwner(args) => {
                writer.write_all(&[TRANSFER_REWARD_OWNER_IX_DISCM])?;
                args.serialize(&mut writer)
            }
            Self::InitializeReward(args) => {
                writer.write_all(&[INITIALIZE_REWARD_IX_DISCM])?;
                args.serialize(&mut writer)
            }
            Self::CollectRemainingRewards(args) => {
                writer.write_all(&[COLLECT_REMAINING_REWARDS_IX_DISCM])?;
                args.serialize(&mut writer)
            }
            Self::UpdateRewardInfos => {
                writer.write_all(&[UPDATE_REWARD_INFOS_IX_DISCM])?;
                Ok(())
            }
            Self::SetRewardParams(args) => {
                writer.write_all(&[SET_REWARD_PARAMS_IX_DISCM])?;
                args.serialize(&mut writer)
            }
            Self::CollectProtocolFee(args) => {
                writer.write_all(&[CREATE_OPERATION_ACCOUNT_IX_DISCM])?;
                args.serialize(&mut writer)
            }
            Self::CollectFundFee(args) => {
                writer.write_all(&[COLLECT_FUND_FEE_IX_DISCM])?;
                args.serialize(&mut writer)
            }
            Self::OpenPosition(args) => {
                writer.write_all(&[OPEN_POSITION_IX_DISCM])?;
                args.serialize(&mut writer)
            }
            Self::OpenPositionV2(args) => {
                writer.write_all(&[OPEN_POSITION_V2_IX_DISCM])?;
                args.serialize(&mut writer)
            }
            Self::OpenPositionWithToken22Nft(args) => {
                writer.write_all(&[OPEN_POSITION_WITH_TOKEN22_NFT_IX_DISCM])?;
                args.serialize(&mut writer)
            }
            Self::ClosePosition => {
                writer.write_all(&[CLOSE_POSITION_IX_DISCM])?;
                Ok(())
            }
            Self::IncreaseLiquidity(args) => {
                writer.write_all(&[INCREASE_LIQUIDITY_IX_DISCM])?;
                args.serialize(&mut writer)
            }
            Self::IncreaseLiquidityV2(args) => {
                writer.write_all(&[INCREASE_LIQUIDITY_V2_IX_DISCM])?;
                args.serialize(&mut writer)
            }
            Self::DecreaseLiquidity(args) => {
                writer.write_all(&[DECREASE_LIQUIDITY_IX_DISCM])?;
                args.serialize(&mut writer)
            }
            Self::DecreaseLiquidityV2(args) => {
                writer.write_all(&[DECREASE_LIQUIDITY_V2_IX_DISCM])?;
                args.serialize(&mut writer)
            }
            Self::Swap(args) => {
                writer.write_all(&[SWAP_IX_DISCM])?;
                args.serialize(&mut writer)
            }
            Self::SwapV2(args) => {
                writer.write_all(&[SWAP_V2_IX_DISCM])?;
                args.serialize(&mut writer)
            }
            Self::SwapRouterBaseIn(args) => {
                writer.write_all(&[SWAP_ROUTER_BASE_IN_IX_DISCM])?;
                args.serialize(&mut writer)
            }
        }
    } 

    pub fn try_to_vec(&self) ->  std::io::Result<Vec<u8>>  {
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
pub const CREATE_AMM_CONFIG_IX_ACCOUNTS_LEN: usize = 3;
#[derive(Copy, Clone, Debug)]
pub struct CreateAmmConfigAccounts<'me, 'info> {
    pub owner: &'me AccountInfo<'info>,
    pub amm_config: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CreateAmmConfigKeys {
    pub owner: Pubkey,
    pub amm_config: Pubkey,
    pub system_program: Pubkey,
}
impl From<CreateAmmConfigAccounts<'_, '_>> for CreateAmmConfigKeys {
    fn from(accounts: CreateAmmConfigAccounts) -> Self {
        Self {
            owner: *accounts.owner.key,
            amm_config: *accounts.amm_config.key,
            system_program: *accounts.system_program.key,
        }
    }
}
impl From<CreateAmmConfigKeys> for [AccountMeta; CREATE_AMM_CONFIG_IX_ACCOUNTS_LEN] {
    fn from(keys: CreateAmmConfigKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.owner,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.amm_config,
                is_signer: false,
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
impl From<[Pubkey; CREATE_AMM_CONFIG_IX_ACCOUNTS_LEN]> for CreateAmmConfigKeys {
    fn from(pubkeys: [Pubkey; CREATE_AMM_CONFIG_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            owner: pubkeys[0],
            amm_config: pubkeys[1],
            system_program: pubkeys[2],
        }
    }
}
impl<'info> From<CreateAmmConfigAccounts<'_, 'info>>
for [AccountInfo<'info>; CREATE_AMM_CONFIG_IX_ACCOUNTS_LEN] {
    fn from(accounts: CreateAmmConfigAccounts<'_, 'info>) -> Self {
        [
            accounts.owner.clone(),
            accounts.amm_config.clone(),
            accounts.system_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; CREATE_AMM_CONFIG_IX_ACCOUNTS_LEN]>
for CreateAmmConfigAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; CREATE_AMM_CONFIG_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            owner: &arr[0],
            amm_config: &arr[1],
            system_program: &arr[2],
        }
    }
}
pub const CREATE_AMM_CONFIG_IX_DISCM:u8 = 1u8;
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CreateAmmConfigIxArgs {
    pub index: u16,
    pub tick_spacing: u16,
    pub trade_fee_rate: u32,
    pub protocol_fee_rate: u32,
    pub fund_fee_rate: u32
}
#[derive(Clone, Debug, PartialEq)]
pub struct CreateAmmConfigIxData(pub CreateAmmConfigIxArgs);
impl From<CreateAmmConfigIxArgs> for CreateAmmConfigIxData {
    fn from(args: CreateAmmConfigIxArgs) -> Self {
        Self(args)
    }
}
impl CreateAmmConfigIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
          let mut reader = buf;
        let mut maybe_discm_buf = [0u8; 1];
        reader.read_exact(&mut maybe_discm_buf)?;
        let maybe_discm = maybe_discm_buf[0];
        if maybe_discm != CREATE_AMM_CONFIG_IX_DISCM {
            return Err(
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!(
                        "discm does not match. Expected: {:?}. Received: {:?}",
                        CREATE_AMM_CONFIG_IX_DISCM, maybe_discm
                    ),
                ),
            );
        }
        Ok(Self(CreateAmmConfigIxArgs::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&[CREATE_AMM_CONFIG_IX_DISCM])?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>>  {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn create_amm_config_ix_with_program_id(
    program_id: Pubkey,
    keys: CreateAmmConfigKeys,
    args: CreateAmmConfigIxArgs,
) ->  std::io::Result<Instruction> {
    let metas: [AccountMeta; CREATE_AMM_CONFIG_IX_ACCOUNTS_LEN] = keys.into();
    let data: CreateAmmConfigIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn create_amm_config_ix(
    keys: CreateAmmConfigKeys,
    args: CreateAmmConfigIxArgs,
) ->  std::io::Result<Instruction> {
    create_amm_config_ix_with_program_id(crate::ID, keys, args)
}
pub fn create_amm_config_invoke_with_program_id(
    program_id: Pubkey,
    accounts: CreateAmmConfigAccounts<'_, '_>,
    args: CreateAmmConfigIxArgs,
) -> ProgramResult {
    let keys: CreateAmmConfigKeys = accounts.into();
    let ix = create_amm_config_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn create_amm_config_invoke(
    accounts: CreateAmmConfigAccounts<'_, '_>,
    args: CreateAmmConfigIxArgs,
) -> ProgramResult {
    create_amm_config_invoke_with_program_id(crate::ID, accounts, args)
}
pub fn create_amm_config_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: CreateAmmConfigAccounts<'_, '_>,
    args: CreateAmmConfigIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: CreateAmmConfigKeys = accounts.into();
    let ix = create_amm_config_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn create_amm_config_invoke_signed(
    accounts: CreateAmmConfigAccounts<'_, '_>,
    args: CreateAmmConfigIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    create_amm_config_invoke_signed_with_program_id(crate::ID, accounts, args, seeds)
}
pub fn create_amm_config_verify_account_keys(
    accounts: CreateAmmConfigAccounts<'_, '_>,
    keys: CreateAmmConfigKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.owner.key, keys.owner),
        (*accounts.amm_config.key, keys.amm_config),
        (*accounts.system_program.key, keys.system_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn create_amm_config_verify_writable_privileges<'me, 'info>(
    accounts: CreateAmmConfigAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.owner, accounts.amm_config] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn create_amm_config_verify_signer_privileges<'me, 'info>(
    accounts: CreateAmmConfigAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.owner] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn create_amm_config_verify_account_privileges<'me, 'info>(
    accounts: CreateAmmConfigAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    create_amm_config_verify_writable_privileges(accounts)?;
    create_amm_config_verify_signer_privileges(accounts)?;
    Ok(())
}

pub const UPDATE_AMM_CONFIG_IX_ACCOUNTS_LEN: usize = 2;
#[derive(Copy, Clone, Debug)]
pub struct UpdateAmmConfigAccounts<'me, 'info> {
    pub owner: &'me AccountInfo<'info>,
    pub amm_config: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct UpdateAmmConfigKeys {
    pub owner: Pubkey,
    pub amm_config: Pubkey,
}
impl From<UpdateAmmConfigAccounts<'_, '_>> for UpdateAmmConfigKeys {
    fn from(accounts: UpdateAmmConfigAccounts) -> Self {
        Self {
            owner: *accounts.owner.key,
            amm_config: *accounts.amm_config.key,
        }
    }
}
impl From<UpdateAmmConfigKeys> for [AccountMeta; UPDATE_AMM_CONFIG_IX_ACCOUNTS_LEN] {
    fn from(keys: UpdateAmmConfigKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.owner,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.amm_config,
                is_signer: false,
                is_writable: true,
            },
        ]
    }
}
impl From<[Pubkey; UPDATE_AMM_CONFIG_IX_ACCOUNTS_LEN]> for UpdateAmmConfigKeys {
    fn from(pubkeys: [Pubkey; UPDATE_AMM_CONFIG_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            owner: pubkeys[0],
            amm_config: pubkeys[1],
        }
    }
}
impl<'info> From<UpdateAmmConfigAccounts<'_, 'info>>
for [AccountInfo<'info>; UPDATE_AMM_CONFIG_IX_ACCOUNTS_LEN] {
    fn from(accounts: UpdateAmmConfigAccounts<'_, 'info>) -> Self {
        [accounts.owner.clone(), accounts.amm_config.clone()]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; UPDATE_AMM_CONFIG_IX_ACCOUNTS_LEN]>
for UpdateAmmConfigAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; UPDATE_AMM_CONFIG_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            owner: &arr[0],
            amm_config: &arr[1],
        }
    }
}
pub const UPDATE_AMM_CONFIG_IX_DISCM : u8 = 2u8;
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct UpdateAmmConfigIxArgs {
    pub param: u8,
    pub value: u32,
}
#[derive(Clone, Debug, PartialEq)]
pub struct UpdateAmmConfigIxData(pub UpdateAmmConfigIxArgs);
impl From<UpdateAmmConfigIxArgs> for UpdateAmmConfigIxData {
    fn from(args: UpdateAmmConfigIxArgs) -> Self {
        Self(args)
    }
}
impl UpdateAmmConfigIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
          let mut reader = buf;
        let mut maybe_discm_buf = [0u8; 1];
        reader.read_exact(&mut maybe_discm_buf)?;
        let maybe_discm = maybe_discm_buf[0];
        if maybe_discm != UPDATE_AMM_CONFIG_IX_DISCM {
            return Err(
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!(
                        "discm does not match. Expected: {:?}. Received: {:?}",
                        UPDATE_AMM_CONFIG_IX_DISCM, maybe_discm
                    ),
                ),
            );
        }
        Ok(Self(UpdateAmmConfigIxArgs::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&[UPDATE_AMM_CONFIG_IX_DISCM])?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>>  {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn update_amm_config_ix_with_program_id(
    program_id: Pubkey,
    keys: UpdateAmmConfigKeys,
    args: UpdateAmmConfigIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; UPDATE_AMM_CONFIG_IX_ACCOUNTS_LEN] = keys.into();
    let data: UpdateAmmConfigIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn update_amm_config_ix(
    keys: UpdateAmmConfigKeys,
    args: UpdateAmmConfigIxArgs,
) -> std::io::Result<Instruction> {
    update_amm_config_ix_with_program_id(crate::ID, keys, args)
}
pub fn update_amm_config_invoke_with_program_id(
    program_id: Pubkey,
    accounts: UpdateAmmConfigAccounts<'_, '_>,
    args: UpdateAmmConfigIxArgs,
) -> ProgramResult {
    let keys: UpdateAmmConfigKeys = accounts.into();
    let ix = update_amm_config_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn update_amm_config_invoke(
    accounts: UpdateAmmConfigAccounts<'_, '_>,
    args: UpdateAmmConfigIxArgs,
) -> ProgramResult {
    update_amm_config_invoke_with_program_id(crate::ID, accounts, args)
}
pub fn update_amm_config_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: UpdateAmmConfigAccounts<'_, '_>,
    args: UpdateAmmConfigIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: UpdateAmmConfigKeys = accounts.into();
    let ix = update_amm_config_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn update_amm_config_invoke_signed(
    accounts: UpdateAmmConfigAccounts<'_, '_>,
    args: UpdateAmmConfigIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    update_amm_config_invoke_signed_with_program_id(crate::ID, accounts, args, seeds)
}
pub fn update_amm_config_verify_account_keys(
    accounts: UpdateAmmConfigAccounts<'_, '_>,
    keys: UpdateAmmConfigKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.owner.key, keys.owner),
        (*accounts.amm_config.key, keys.amm_config),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn update_amm_config_verify_writable_privileges<'me, 'info>(
    accounts: UpdateAmmConfigAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.amm_config] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn update_amm_config_verify_signer_privileges<'me, 'info>(
    accounts: UpdateAmmConfigAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.owner] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn update_amm_config_verify_account_privileges<'me, 'info>(
    accounts: UpdateAmmConfigAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    update_amm_config_verify_writable_privileges(accounts)?;
    update_amm_config_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const CREATE_POOL_IX_ACCOUNTS_LEN: usize = 13;

#[derive(Copy, Clone, Debug)]
pub struct CreatePoolAccounts<'me, 'info> {
    pub pool_creator: &'me AccountInfo<'info>,
    pub amm_config: &'me AccountInfo<'info>,
    pub pool_state: &'me AccountInfo<'info>,
    pub token_mint_0: &'me AccountInfo<'info>,
    pub token_mint_1: &'me AccountInfo<'info>,
    pub token_vault_0: &'me AccountInfo<'info>,
    pub token_vault_1: &'me AccountInfo<'info>,
    pub observation_state: &'me AccountInfo<'info>,
    pub tick_array_bitmap: &'me AccountInfo<'info>,
    pub token_program_0: &'me AccountInfo<'info>,
    pub token_program_1: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub rent: &'me AccountInfo<'info>,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CreatePoolKeys {
    pub pool_creator: Pubkey,
    pub amm_config: Pubkey,
    pub pool_state: Pubkey,
    pub token_mint_0: Pubkey,
    pub token_mint_1: Pubkey,
    pub token_vault_0: Pubkey,
    pub token_vault_1: Pubkey,
    pub observation_state: Pubkey,
    pub tick_array_bitmap: Pubkey,
    pub token_program_0: Pubkey,
    pub token_program_1: Pubkey,
    pub system_program: Pubkey,
    pub rent: Pubkey,
}

impl From<CreatePoolAccounts<'_, '_>> for CreatePoolKeys {
    fn from(accounts: CreatePoolAccounts) -> Self {
        Self {
            pool_creator: *accounts.pool_creator.key,
            amm_config: *accounts.amm_config.key,
            pool_state: *accounts.pool_state.key,
            token_mint_0: *accounts.token_mint_0.key,
            token_mint_1: *accounts.token_mint_1.key,
            token_vault_0: *accounts.token_vault_0.key,
            token_vault_1: *accounts.token_vault_1.key,
            observation_state: *accounts.observation_state.key,
            tick_array_bitmap: *accounts.tick_array_bitmap.key,
            token_program_0: *accounts.token_program_0.key,
            token_program_1: *accounts.token_program_1.key,
            system_program: *accounts.system_program.key,
            rent: *accounts.rent.key,
        }
    }
}

impl From<CreatePoolKeys> for [AccountMeta; CREATE_POOL_IX_ACCOUNTS_LEN] {
    fn from(keys: CreatePoolKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.pool_creator,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.amm_config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pool_state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_mint_0,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_mint_1,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_vault_0,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_vault_1,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.observation_state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.tick_array_bitmap,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_program_0,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_program_1,
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
        ]
    }
}

impl From<[Pubkey; CREATE_POOL_IX_ACCOUNTS_LEN]> for CreatePoolKeys {
    fn from(pubkeys: [Pubkey; CREATE_POOL_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            pool_creator: pubkeys[0],
            amm_config: pubkeys[1],
            pool_state: pubkeys[2],
            token_mint_0: pubkeys[3],
            token_mint_1: pubkeys[4],
            token_vault_0: pubkeys[5],
            token_vault_1: pubkeys[6],
            observation_state: pubkeys[7],
            tick_array_bitmap: pubkeys[8],
            token_program_0: pubkeys[9],
            token_program_1: pubkeys[10],
            system_program: pubkeys[11],
            rent: pubkeys[12],
        }
    }
}

impl<'info> From<CreatePoolAccounts<'_, 'info>> for [AccountInfo<'info>; CREATE_POOL_IX_ACCOUNTS_LEN] {
    fn from(accounts: CreatePoolAccounts<'_, 'info>) -> Self {
        [
            accounts.pool_creator.clone(),
            accounts.amm_config.clone(),
            accounts.pool_state.clone(),
            accounts.token_mint_0.clone(),
            accounts.token_mint_1.clone(),
            accounts.token_vault_0.clone(),
            accounts.token_vault_1.clone(),
            accounts.observation_state.clone(),
            accounts.tick_array_bitmap.clone(),
            accounts.token_program_0.clone(),
            accounts.token_program_1.clone(),
            accounts.system_program.clone(),
            accounts.rent.clone(),
        ]
    }
}

impl<'me, 'info> From<&'me [AccountInfo<'info>; CREATE_POOL_IX_ACCOUNTS_LEN]> for CreatePoolAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; CREATE_POOL_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            pool_creator: &arr[0],
            amm_config: &arr[1],
            pool_state: &arr[2],
            token_mint_0: &arr[3],
            token_mint_1: &arr[4],
            token_vault_0: &arr[5],
            token_vault_1: &arr[6],
            observation_state: &arr[7],
            tick_array_bitmap: &arr[8],
            token_program_0: &arr[9],
            token_program_1: &arr[10],
            system_program: &arr[11],
            rent: &arr[12],
        }
    }
}
pub const CREATE_POOL_IX_DISCM: u8 = 233u8;
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CreatePoolIxArgs {
    pub sqrt_price_x64: u128,
    pub open_time: u64,
}

// Instruction data struct
#[derive(Clone, Debug, PartialEq)]
pub struct CreatePoolIxData(pub CreatePoolIxArgs);

impl From<CreatePoolIxArgs> for CreatePoolIxData {
    fn from(args: CreatePoolIxArgs) -> Self {
        Self(args)
    }
}

impl CreatePoolIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm_buf = [0u8; 1];
        reader.read_exact(&mut maybe_discm_buf)?;
        let maybe_discm = maybe_discm_buf[0];
        if maybe_discm != CREATE_POOL_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    CREATE_POOL_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(CreatePoolIxArgs::deserialize(&mut reader)?))
    }

    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&[CREATE_POOL_IX_DISCM])?;
        self.0.serialize(&mut writer)
    }

    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>>  {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}

// Helper functions
pub fn create_pool_ix_with_program_id(
    program_id: Pubkey,
    keys: CreatePoolKeys,
    args: CreatePoolIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; CREATE_POOL_IX_ACCOUNTS_LEN] = keys.into();
    let data: CreatePoolIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}

pub fn create_pool_ix(keys: CreatePoolKeys, args: CreatePoolIxArgs) -> std::io::Result<Instruction> {
    create_pool_ix_with_program_id(crate::ID, keys, args)
}

pub fn create_pool_invoke_with_program_id(
    program_id: Pubkey,
    accounts: CreatePoolAccounts<'_, '_>,
    args: CreatePoolIxArgs,
) -> ProgramResult {
    let keys: CreatePoolKeys = accounts.into();
    let ix = create_pool_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}

pub fn create_pool_invoke(accounts: CreatePoolAccounts<'_, '_>, args: CreatePoolIxArgs) -> ProgramResult {
    create_pool_invoke_with_program_id(crate::ID, accounts, args)
}

pub fn create_pool_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: CreatePoolAccounts<'_, '_>,
    args: CreatePoolIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: CreatePoolKeys = accounts.into();
    let ix = create_pool_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}

pub fn create_pool_invoke_signed(
    accounts: CreatePoolAccounts<'_, '_>,
    args: CreatePoolIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    create_pool_invoke_signed_with_program_id(crate::ID, accounts, args, seeds)
}

pub fn create_pool_verify_account_keys(
    accounts: CreatePoolAccounts<'_, '_>,
    keys: CreatePoolKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.pool_creator.key, keys.pool_creator),
        (*accounts.amm_config.key, keys.amm_config),
        (*accounts.pool_state.key, keys.pool_state),
        (*accounts.token_mint_0.key, keys.token_mint_0),
        (*accounts.token_mint_1.key, keys.token_mint_1),
        (*accounts.token_vault_0.key, keys.token_vault_0),
        (*accounts.token_vault_1.key, keys.token_vault_1),
        (*accounts.observation_state.key, keys.observation_state),
        (*accounts.tick_array_bitmap.key, keys.tick_array_bitmap),
        (*accounts.token_program_0.key, keys.token_program_0),
        (*accounts.token_program_1.key, keys.token_program_1),
        (*accounts.system_program.key, keys.system_program),
        (*accounts.rent.key, keys.rent),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}

pub fn create_pool_verify_writable_privileges<'me, 'info>(
    accounts: CreatePoolAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.pool_creator,
        accounts.pool_state,
        accounts.token_vault_0,
        accounts.token_vault_1,
        accounts.observation_state,
        accounts.tick_array_bitmap,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}

pub fn create_pool_verify_signer_privileges<'me, 'info>(
    accounts: CreatePoolAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.pool_creator] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}

pub fn create_pool_verify_account_privileges<'me, 'info>(
    accounts: CreatePoolAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    create_pool_verify_writable_privileges(accounts)?;
    create_pool_verify_signer_privileges(accounts)?;
    Ok(())
}

pub const UPDATE_POOL_STATUS_IX_ACCOUNTS_LEN: usize = 2;
#[derive(Copy, Clone, Debug)]
pub struct UpdatePoolStatusAccounts<'me, 'info> {
    pub authority: &'me AccountInfo<'info>,
    pub pool_state: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct UpdatePoolStatusKeys {
    pub authority: Pubkey,
    pub pool_state: Pubkey,
}
impl From<UpdatePoolStatusAccounts<'_, '_>> for UpdatePoolStatusKeys {
    fn from(accounts: UpdatePoolStatusAccounts) -> Self {
        Self {
            authority: *accounts.authority.key,
            pool_state: *accounts.pool_state.key,
        }
    }
}
impl From<UpdatePoolStatusKeys> for [AccountMeta; UPDATE_POOL_STATUS_IX_ACCOUNTS_LEN] {
    fn from(keys: UpdatePoolStatusKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.authority,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pool_state,
                is_signer: false,
                is_writable: true,
            },
        ]
    }
}
impl From<[Pubkey; UPDATE_POOL_STATUS_IX_ACCOUNTS_LEN]> for UpdatePoolStatusKeys {
    fn from(pubkeys: [Pubkey; UPDATE_POOL_STATUS_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            authority: pubkeys[0],
            pool_state: pubkeys[1],
        }
    }
}
impl<'info> From<UpdatePoolStatusAccounts<'_, 'info>>
for [AccountInfo<'info>; UPDATE_POOL_STATUS_IX_ACCOUNTS_LEN] {
    fn from(accounts: UpdatePoolStatusAccounts<'_, 'info>) -> Self {
        [accounts.authority.clone(), accounts.pool_state.clone()]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; UPDATE_POOL_STATUS_IX_ACCOUNTS_LEN]>
for UpdatePoolStatusAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; UPDATE_POOL_STATUS_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            authority: &arr[0],
            pool_state: &arr[1],
        }
    }
}

pub const UPDATE_POOL_STATUS_IX_DISCM: u8 = 4u8;
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct UpdatePoolStatusIxArgs {
    pub status: u8,
}
#[derive(Clone, Debug, PartialEq)]
pub struct UpdatePoolStatusIxData(pub UpdatePoolStatusIxArgs);
impl From<UpdatePoolStatusIxArgs> for UpdatePoolStatusIxData {
    fn from(args: UpdatePoolStatusIxArgs) -> Self {
        Self(args)
    }
}
impl UpdatePoolStatusIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
          let mut reader = buf;
        let mut maybe_discm_buf = [0u8; 1];
        reader.read_exact(&mut maybe_discm_buf)?;
        let maybe_discm = maybe_discm_buf[0];
        if maybe_discm != UPDATE_POOL_STATUS_IX_DISCM {
            return Err(
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!(
                        "discm does not match. Expected: {:?}. Received: {:?}",
                        UPDATE_POOL_STATUS_IX_DISCM, maybe_discm
                    ),
                ),
            );
        }
        Ok(Self(UpdatePoolStatusIxArgs::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&[UPDATE_POOL_STATUS_IX_DISCM])?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>>  {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn update_pool_status_ix_with_program_id(
    program_id: Pubkey,
    keys: UpdatePoolStatusKeys,
    args: UpdatePoolStatusIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; UPDATE_POOL_STATUS_IX_ACCOUNTS_LEN] = keys.into();
    let data: UpdatePoolStatusIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn update_pool_status_ix(
    keys: UpdatePoolStatusKeys,
    args: UpdatePoolStatusIxArgs,
) -> std::io::Result<Instruction> {
    update_pool_status_ix_with_program_id(crate::ID, keys, args)
}
pub fn update_pool_status_invoke_with_program_id(
    program_id: Pubkey,
    accounts: UpdatePoolStatusAccounts<'_, '_>,
    args: UpdatePoolStatusIxArgs,
) -> ProgramResult {
    let keys: UpdatePoolStatusKeys = accounts.into();
    let ix = update_pool_status_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn update_pool_status_invoke(
    accounts: UpdatePoolStatusAccounts<'_, '_>,
    args: UpdatePoolStatusIxArgs,
) -> ProgramResult {
    update_pool_status_invoke_with_program_id(crate::ID, accounts, args)
}
pub fn update_pool_status_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: UpdatePoolStatusAccounts<'_, '_>,
    args: UpdatePoolStatusIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: UpdatePoolStatusKeys = accounts.into();
    let ix = update_pool_status_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn update_pool_status_invoke_signed(
    accounts: UpdatePoolStatusAccounts<'_, '_>,
    args: UpdatePoolStatusIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    update_pool_status_invoke_signed_with_program_id(crate::ID, accounts, args, seeds)
}
pub fn update_pool_status_verify_account_keys(
    accounts: UpdatePoolStatusAccounts<'_, '_>,
    keys: UpdatePoolStatusKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.authority.key, keys.authority),
        (*accounts.pool_state.key, keys.pool_state),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn update_pool_status_verify_writable_privileges<'me, 'info>(
    accounts: UpdatePoolStatusAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.pool_state] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn update_pool_status_verify_signer_privileges<'me, 'info>(
    accounts: UpdatePoolStatusAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn update_pool_status_verify_account_privileges<'me, 'info>(
    accounts: UpdatePoolStatusAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    update_pool_status_verify_writable_privileges(accounts)?;
    update_pool_status_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const CREATE_OPERATION_ACCOUNT_IX_ACCOUNTS_LEN: usize = 3;
#[derive(Copy, Clone, Debug)]
pub struct CreateOperationAccountAccounts<'me, 'info> {
    pub owner: &'me AccountInfo<'info>,
    pub operation_state: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CreateOperationAccountKeys {
    pub owner: Pubkey,
    pub operation_state: Pubkey,
    pub system_program: Pubkey
}
impl From<CreateOperationAccountAccounts<'_, '_>> for CreateOperationAccountKeys {
    fn from(accounts: CreateOperationAccountAccounts) -> Self {
        Self {
            owner: *accounts.owner.key,
            operation_state: *accounts.operation_state.key,
            system_program: *accounts.system_program.key
        }
    }
}
impl From<CreateOperationAccountKeys>
for [AccountMeta; CREATE_OPERATION_ACCOUNT_IX_ACCOUNTS_LEN] {
    fn from(keys: CreateOperationAccountKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.owner,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.operation_state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.system_program,
                is_signer: false,
                is_writable: false,
            }
        ]
    }
}
impl From<[Pubkey; CREATE_OPERATION_ACCOUNT_IX_ACCOUNTS_LEN]> for CreateOperationAccountKeys {
    fn from(pubkeys: [Pubkey; CREATE_OPERATION_ACCOUNT_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            owner: pubkeys[0],
            operation_state: pubkeys[1],
            system_program: pubkeys[2]
        }
    }
}
impl<'info> From<CreateOperationAccountAccounts<'_, 'info>>
for [AccountInfo<'info>; CREATE_OPERATION_ACCOUNT_IX_ACCOUNTS_LEN] {
    fn from(accounts: CreateOperationAccountAccounts<'_, 'info>) -> Self {
        [
            accounts.owner.clone(),
            accounts.operation_state.clone(),
            accounts.system_program.clone()
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; CREATE_OPERATION_ACCOUNT_IX_ACCOUNTS_LEN]>
for CreateOperationAccountAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; CREATE_OPERATION_ACCOUNT_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            owner: &arr[0],
            operation_state: &arr[1],
            system_program: &arr[2]
        }
    }
}
pub const CREATE_OPERATION_ACCOUNT_IX_DISCM : u8 = 5u8;
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CreateOperationAccountIxArgs;
#[derive(Clone, Debug, PartialEq)]
pub struct CreateOperationAccountIxData(pub CreateOperationAccountIxArgs);
impl From<CreateOperationAccountIxArgs> for CreateOperationAccountIxData {
    fn from(args: CreateOperationAccountIxArgs) -> Self {
        Self(args)
    }
}
impl CreateOperationAccountIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm_buf = [0u8; 1];
        reader.read_exact(&mut maybe_discm_buf)?;
        let maybe_discm = maybe_discm_buf[0];
        if maybe_discm != CREATE_OPERATION_ACCOUNT_IX_DISCM {
            return Err(
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!(
                        "discm does not match. Expected: {:?}. Received: {:?}",
                        CREATE_OPERATION_ACCOUNT_IX_DISCM, maybe_discm
                    ),
                ),
            );
        }
        Ok(Self(CreateOperationAccountIxArgs::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&[CREATE_OPERATION_ACCOUNT_IX_DISCM])?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>>  {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn create_operation_account_ix_with_program_id(
    program_id: Pubkey,
    keys: CreateOperationAccountKeys,
    args: CreateOperationAccountIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; CREATE_OPERATION_ACCOUNT_IX_ACCOUNTS_LEN] = keys.into();
    let data: CreateOperationAccountIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn create_operation_account_ix(
    keys: CreateOperationAccountKeys,
    args: CreateOperationAccountIxArgs,
) -> std::io::Result<Instruction> {
    create_operation_account_ix_with_program_id(crate::ID, keys, args)
}
pub fn create_operation_account_invoke_with_program_id(
    program_id: Pubkey,
    accounts: CreateOperationAccountAccounts<'_, '_>,
    args: CreateOperationAccountIxArgs,
) -> ProgramResult {
    let keys: CreateOperationAccountKeys = accounts.into();
    let ix = create_operation_account_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn create_operation_account_invoke(
    accounts: CreateOperationAccountAccounts<'_, '_>,
    args: CreateOperationAccountIxArgs,
) -> ProgramResult {
    create_operation_account_invoke_with_program_id(crate::ID, accounts, args)
}
pub fn create_operation_account_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: CreateOperationAccountAccounts<'_, '_>,
    args: CreateOperationAccountIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: CreateOperationAccountKeys = accounts.into();
    let ix = create_operation_account_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn create_operation_account_invoke_signed(
    accounts: CreateOperationAccountAccounts<'_, '_>,
    args: CreateOperationAccountIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    create_operation_account_invoke_signed_with_program_id(crate::ID, accounts, args, seeds)
}
pub fn create_operation_account_verify_account_keys(
    accounts: CreateOperationAccountAccounts<'_, '_>,
    keys: CreateOperationAccountKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.owner.key, keys.owner),
        (*accounts.operation_state.key, keys.operation_state),
        (*accounts.system_program.key, keys.system_program)
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn create_operation_account_verify_writable_privileges<'me, 'info>(
    accounts: CreateOperationAccountAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.owner,
        accounts.operation_state
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn create_operation_account_verify_signer_privileges<'me, 'info>(
    accounts: CreateOperationAccountAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.owner] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn create_operation_account_verify_account_privileges<'me, 'info>(
    accounts: CreateOperationAccountAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    create_operation_account_verify_writable_privileges(accounts)?;
    create_operation_account_verify_signer_privileges(accounts)?;
    Ok(())
}

pub const UPDATE_OPERATION_ACCOUNT_IX_ACCOUNTS_LEN: usize = 3;
#[derive(Copy, Clone, Debug)]
pub struct UpdateOperationAccountAccounts<'me, 'info> {
    pub owner: &'me AccountInfo<'info>,
    pub operation_state: &'me AccountInfo<'info>,
    pub system_program : &'me AccountInfo<'info>
}
#[derive(Copy, Clone, Debug, PartialEq)]   
pub struct UpdateOperationAccountKeys {
    pub owner: Pubkey,
    pub operation_state: Pubkey,
    pub system_program: Pubkey
}
impl From<UpdateOperationAccountAccounts<'_, '_>> for UpdateOperationAccountKeys {
    fn from(accounts: UpdateOperationAccountAccounts) -> Self {
        Self {
            owner: *accounts.owner.key,
            operation_state: *accounts.operation_state.key,
            system_program: *accounts.system_program.key
        }
    }
}
impl From<UpdateOperationAccountKeys> for [AccountMeta; UPDATE_OPERATION_ACCOUNT_IX_ACCOUNTS_LEN] {
    fn from(keys: UpdateOperationAccountKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.owner,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.operation_state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.system_program,
                is_signer: false,
                is_writable: false,
            }
        ]
    }
}
impl From<[Pubkey; UPDATE_OPERATION_ACCOUNT_IX_ACCOUNTS_LEN]> for UpdateOperationAccountKeys {
    fn from(pubkeys: [Pubkey; UPDATE_OPERATION_ACCOUNT_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            owner: pubkeys[0],
            operation_state: pubkeys[1],
            system_program: pubkeys[2]
        }
    }
}
impl<'info> From<UpdateOperationAccountAccounts<'_, 'info>> 
for [AccountInfo<'info>; UPDATE_OPERATION_ACCOUNT_IX_ACCOUNTS_LEN] {
    fn from(accounts: UpdateOperationAccountAccounts<'_, 'info>) -> Self {
        [
            accounts.owner.clone(),
            accounts.operation_state.clone(),
            accounts.system_program.clone()
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; UPDATE_OPERATION_ACCOUNT_IX_ACCOUNTS_LEN]>
for UpdateOperationAccountAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; UPDATE_OPERATION_ACCOUNT_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            owner: &arr[0],
            operation_state: &arr[1],
            system_program: &arr[2]
        }
    }
}
pub const UPDATE_OPERATION_ACCOUNT_IX_DISCM: u8 = 6u8;
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct UpdateOperationAccountIxArgs {
    pub param: u64,
    pub keys: Vec<Pubkey>
}
#[derive(Clone, Debug, PartialEq)]
pub struct UpdateOperationAccountIxData(pub UpdateOperationAccountIxArgs);
impl From<UpdateOperationAccountIxArgs> for UpdateOperationAccountIxData {
    fn from(args: UpdateOperationAccountIxArgs) -> Self {
        Self(args)
    }
}
impl UpdateOperationAccountIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
          let mut reader = buf;
        let mut maybe_discm_buf = [0u8; 1];
        reader.read_exact(&mut maybe_discm_buf)?;
        let maybe_discm = maybe_discm_buf[0];
        if maybe_discm != UPDATE_OPERATION_ACCOUNT_IX_DISCM {
            return Err(
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!(
                        "discm does not match. Expected: {:?}. Received: {:?}",
                        UPDATE_OPERATION_ACCOUNT_IX_DISCM, maybe_discm
                    ),
                ),
            );
        }
        Ok(Self(UpdateOperationAccountIxArgs::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&[UPDATE_OPERATION_ACCOUNT_IX_DISCM])?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>>  {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn update_operation_account_ix_with_program_id(
    program_id: Pubkey,
    keys: UpdateOperationAccountKeys,
    args: UpdateOperationAccountIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; UPDATE_OPERATION_ACCOUNT_IX_ACCOUNTS_LEN] = keys.into();
    let data: UpdateOperationAccountIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn update_operation_account_ix(
    keys: UpdateOperationAccountKeys,
    args: UpdateOperationAccountIxArgs,
) -> std::io::Result<Instruction> {
    update_operation_account_ix_with_program_id(crate::ID, keys, args)
}

pub fn update_operation_account_invoke_with_program_id(
    program_id: Pubkey,
    accounts: UpdateOperationAccountAccounts<'_, '_>,
    args: UpdateOperationAccountIxArgs,
) -> ProgramResult {
    let keys: UpdateOperationAccountKeys = accounts.into();
    let ix = update_operation_account_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn update_operation_account_invoke(
    accounts: UpdateOperationAccountAccounts<'_, '_>,
    args: UpdateOperationAccountIxArgs,
) -> ProgramResult {
    update_operation_account_invoke_with_program_id(crate::ID, accounts, args)
}

pub fn update_operation_account_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: UpdateOperationAccountAccounts<'_, '_>,
    args: UpdateOperationAccountIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: UpdateOperationAccountKeys = accounts.into();
    let ix = update_operation_account_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn update_operation_account_invoke_signed(
    accounts: UpdateOperationAccountAccounts<'_, '_>,
    args: UpdateOperationAccountIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    update_operation_account_invoke_signed_with_program_id(crate::ID, accounts, args, seeds)
}
pub fn update_operation_account_verify_account_keys(
    accounts: UpdateOperationAccountAccounts<'_, '_>,
    keys: UpdateOperationAccountKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.owner.key, keys.owner),
        (*accounts.operation_state.key, keys.operation_state),
        (*accounts.system_program.key, keys.system_program)
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn update_operation_account_verify_writable_privileges<'me, 'info>(
    accounts: UpdateOperationAccountAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.owner,
        accounts.operation_state
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn update_operation_account_verify_signer_privileges<'me, 'info>(
    accounts: UpdateOperationAccountAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.owner] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn update_operation_account_verify_account_privileges<'me, 'info>(
    accounts: UpdateOperationAccountAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    update_operation_account_verify_writable_privileges(accounts)?;
    update_operation_account_verify_signer_privileges(accounts)?;
    Ok(())
}

pub const TRANSFER_REWARD_OWNER_IX_ACCOUNTS_LEN: usize = 2;
#[derive(Copy, Clone, Debug)]
pub struct TransferRewardOwnerAccounts<'me, 'info> {
    pub authority: &'me AccountInfo<'info>,
    pub pool_state: &'me AccountInfo<'info>
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct TransferRewardOwnerKeys {
    pub authority: Pubkey,
    pub pool_state: Pubkey
}
impl From<TransferRewardOwnerAccounts<'_, '_>> for TransferRewardOwnerKeys {
    fn from(accounts: TransferRewardOwnerAccounts) -> Self {
        Self {
            authority: *accounts.authority.key,
            pool_state: *accounts.pool_state.key
        }
    }
}
impl From<TransferRewardOwnerKeys> for [AccountMeta; TRANSFER_REWARD_OWNER_IX_ACCOUNTS_LEN] {
    fn from(keys: TransferRewardOwnerKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.authority,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pool_state,
                is_signer: false,
                is_writable: true,
            }
        ]
    }
}
impl From<[Pubkey; TRANSFER_REWARD_OWNER_IX_ACCOUNTS_LEN]> for TransferRewardOwnerKeys {
    fn from(pubkeys: [Pubkey; TRANSFER_REWARD_OWNER_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            authority: pubkeys[0],
            pool_state: pubkeys[1]
        }
    }
}
impl<'info> From<TransferRewardOwnerAccounts<'_, 'info>>
for [AccountInfo<'info>; TRANSFER_REWARD_OWNER_IX_ACCOUNTS_LEN] {
    fn from(accounts: TransferRewardOwnerAccounts<'_, 'info>) -> Self {
        [
            accounts.authority.clone(),
            accounts.pool_state.clone()
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; TRANSFER_REWARD_OWNER_IX_ACCOUNTS_LEN]>	
for TransferRewardOwnerAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; TRANSFER_REWARD_OWNER_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            authority: &arr[0],
            pool_state: &arr[1]
        }  
    }
}
pub const TRANSFER_REWARD_OWNER_IX_DISCM: u8 = 7u8;
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct TransferRewardOwnerIxArgs {
    pub new_owner: Pubkey
}
#[derive(Clone, Debug, PartialEq)]
pub struct TransferRewardOwnerIxData(pub TransferRewardOwnerIxArgs);
impl From<TransferRewardOwnerIxArgs> for TransferRewardOwnerIxData {
    fn from(args: TransferRewardOwnerIxArgs) -> Self {
        Self(args)
    }
}
impl TransferRewardOwnerIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
          let mut reader = buf;
        let mut maybe_discm_buf = [0u8; 1];
        reader.read_exact(&mut maybe_discm_buf)?;
        let maybe_discm = maybe_discm_buf[0];
        if maybe_discm != TRANSFER_REWARD_OWNER_IX_DISCM {
            return Err(
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!(
                        "discm does not match. Expected: {:?}. Received: {:?}",
                        TRANSFER_REWARD_OWNER_IX_DISCM, maybe_discm
                    ),
                ),
            );
        }
        Ok(Self(TransferRewardOwnerIxArgs::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&[TRANSFER_REWARD_OWNER_IX_DISCM])?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>>  {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn transfer_reward_owner_ix_with_program_id(
    program_id: Pubkey,
    keys: TransferRewardOwnerKeys,
    args: TransferRewardOwnerIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; TRANSFER_REWARD_OWNER_IX_ACCOUNTS_LEN] = keys.into();
    let data: TransferRewardOwnerIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn transfer_reward_owner_ix(
    keys: TransferRewardOwnerKeys,
    args: TransferRewardOwnerIxArgs,
) -> std::io::Result<Instruction> {
    transfer_reward_owner_ix_with_program_id(crate::ID, keys, args)
}
pub fn transfer_reward_owner_invoke_with_program_id(
    program_id: Pubkey,
    accounts: TransferRewardOwnerAccounts<'_, '_>,
    args: TransferRewardOwnerIxArgs,
) -> ProgramResult {
    let keys: TransferRewardOwnerKeys = accounts.into();
    let ix = transfer_reward_owner_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn transfer_reward_owner_invoke(
    accounts: TransferRewardOwnerAccounts<'_, '_>,
    args: TransferRewardOwnerIxArgs,
) -> ProgramResult {
    transfer_reward_owner_invoke_with_program_id(crate::ID, accounts, args)
}
pub fn transfer_reward_owner_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: TransferRewardOwnerAccounts<'_, '_>,
    args: TransferRewardOwnerIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: TransferRewardOwnerKeys = accounts.into();
    let ix = transfer_reward_owner_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn transfer_reward_owner_invoke_signed(
    accounts: TransferRewardOwnerAccounts<'_, '_>,
    args: TransferRewardOwnerIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    transfer_reward_owner_invoke_signed_with_program_id(crate::ID, accounts, args, seeds)
}
pub fn transfer_reward_owner_verify_account_keys(
    accounts: TransferRewardOwnerAccounts<'_, '_>,
    keys: TransferRewardOwnerKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.authority.key, keys.authority),
        (*accounts.pool_state.key, keys.pool_state)
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn transfer_reward_owner_verify_writable_privileges<'me, 'info>(
    accounts: TransferRewardOwnerAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.pool_state] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn transfer_reward_owner_verify_signer_privileges<'me, 'info>(
    accounts: TransferRewardOwnerAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn transfer_reward_owner_verify_account_privileges<'me, 'info>(
    accounts: TransferRewardOwnerAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    transfer_reward_owner_verify_writable_privileges(accounts)?;
    transfer_reward_owner_verify_signer_privileges(accounts)?;
    Ok(())
}

pub const INITIALIZE_REWARD_IX_ACCOUNTS_LEN: usize = 10;
#[derive(Copy, Clone, Debug)]
pub struct InitializeRewardAccounts<'me, 'info> {
    pub reward_funder: &'me AccountInfo<'info>,
    pub funder_token_account :  &'me AccountInfo<'info>,
    pub amm_config :  &'me AccountInfo<'info>,
    pub pool_state :  &'me AccountInfo<'info>,
    pub operation_state :  &'me AccountInfo<'info>,
    pub reward_token_mint :  &'me AccountInfo<'info>,
    pub reward_token_vault :  &'me AccountInfo<'info>,
    pub reward_token_program :  &'me AccountInfo<'info>,
    pub system_program :  &'me AccountInfo<'info>,
    pub rent :  &'me AccountInfo<'info>
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct InitializeRewardKeys {
    pub reward_funder: Pubkey,
    pub funder_token_account : Pubkey,
    pub amm_config : Pubkey,
    pub pool_state : Pubkey,
    pub operation_state : Pubkey,
    pub reward_token_mint : Pubkey,
    pub reward_token_vault : Pubkey,
    pub reward_token_program : Pubkey,
    pub system_program : Pubkey,
    pub rent : Pubkey
}
impl From<InitializeRewardAccounts<'_, '_>> for InitializeRewardKeys {
    fn from(accounts: InitializeRewardAccounts) -> Self {
        Self {
            reward_funder: *accounts.reward_funder.key,
            funder_token_account : *accounts.funder_token_account.key,
            amm_config : *accounts.amm_config.key,
            pool_state : *accounts.pool_state.key,
            operation_state : *accounts.operation_state.key,
            reward_token_mint : *accounts.reward_token_mint.key,
            reward_token_vault : *accounts.reward_token_vault.key,
            reward_token_program : *accounts.reward_token_program.key,
            system_program : *accounts.system_program.key,
            rent : *accounts.rent.key
        }
    }
}
impl From<InitializeRewardKeys> for [AccountMeta; INITIALIZE_REWARD_IX_ACCOUNTS_LEN] {
    fn from(keys: InitializeRewardKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.reward_funder,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.funder_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.amm_config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pool_state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.operation_state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.reward_token_mint,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.reward_token_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.reward_token_program,
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
            }
        ]
    }
}
impl From<[Pubkey; INITIALIZE_REWARD_IX_ACCOUNTS_LEN]> for InitializeRewardKeys {
    fn from(pubkeys: [Pubkey; INITIALIZE_REWARD_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            reward_funder: pubkeys[0],
            funder_token_account : pubkeys[1],
            amm_config : pubkeys[2],
            pool_state : pubkeys[3],
            operation_state : pubkeys[4],
            reward_token_mint : pubkeys[5],
            reward_token_vault : pubkeys[6],
            reward_token_program : pubkeys[7],
            system_program : pubkeys[8],
            rent : pubkeys[9]
        }
    }
}
impl<'info> From<InitializeRewardAccounts<'_, 'info>>
for [AccountInfo<'info>; INITIALIZE_REWARD_IX_ACCOUNTS_LEN] {
    fn from(accounts: InitializeRewardAccounts<'_, 'info>) -> Self {
        [
            accounts.reward_funder.clone(),
            accounts.funder_token_account.clone(),
            accounts.amm_config.clone(),
            accounts.pool_state.clone(),
            accounts.operation_state.clone(),
            accounts.reward_token_mint.clone(),
            accounts.reward_token_vault.clone(),
            accounts.reward_token_program.clone(),
            accounts.system_program.clone(),
            accounts.rent.clone()
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; INITIALIZE_REWARD_IX_ACCOUNTS_LEN]>
for InitializeRewardAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; INITIALIZE_REWARD_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            reward_funder: &arr[0],
            funder_token_account : &arr[1],
            amm_config : &arr[2],
            pool_state : &arr[3],
            operation_state : &arr[4],
            reward_token_mint : &arr[5],
            reward_token_vault : &arr[6],
            reward_token_program : &arr[7],
            system_program : &arr[8],
            rent : &arr[9]
        }
    }
}
pub const INITIALIZE_REWARD_IX_DISCM:u8 = 95u8;
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InitializeRewardIxArgs {
    pub param : InitializeRewardParam
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
        let mut maybe_discm_buf = [0u8; 1];
        reader.read_exact(&mut maybe_discm_buf)?;
        let maybe_discm = maybe_discm_buf[0];
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
        writer.write_all(&[INITIALIZE_REWARD_IX_DISCM])?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>>  {
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
        (*accounts.reward_funder.key, keys.reward_funder),
        (*accounts.funder_token_account.key, keys.funder_token_account),
        (*accounts.amm_config.key, keys.amm_config),
        (*accounts.pool_state.key, keys.pool_state),
        (*accounts.operation_state.key, keys.operation_state),
        (*accounts.reward_token_mint.key, keys.reward_token_mint),
        (*accounts.reward_token_vault.key, keys.reward_token_vault),
        (*accounts.reward_token_program.key, keys.reward_token_program),
        (*accounts.system_program.key, keys.system_program),
        (*accounts.rent.key, keys.rent)
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
    for should_be_writable in [
        accounts.reward_funder,
        accounts.funder_token_account,
        accounts.pool_state,
        accounts.reward_token_vault
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn initialize_reward_verify_signer_privileges<'me, 'info>(
    accounts: InitializeRewardAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.reward_funder] {
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

pub const COLLECT_REMAINING_REWARDS_IX_ACCOUNTS_LEN: usize = 8;
#[derive(Copy, Clone, Debug)]
pub struct CollectRemainingRewardsAccounts<'me, 'info> {
    pub reward_funder: &'me AccountInfo<'info>,
    pub funder_token_account: &'me AccountInfo<'info>,
    pub pool_state: &'me AccountInfo<'info>,
    pub reward_token_vault: &'me AccountInfo<'info>,
    pub reward_vault_mint: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub token_program2022: &'me AccountInfo<'info>,
    pub memo_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CollectRemainingRewardsKeys {
    pub reward_funder: Pubkey,
    pub funder_token_account: Pubkey,
    pub pool_state: Pubkey,
    pub reward_token_vault: Pubkey,
    pub reward_vault_mint: Pubkey,
    pub token_program: Pubkey,
    pub token_program2022: Pubkey,
    pub memo_program: Pubkey,
}
impl From<CollectRemainingRewardsAccounts<'_, '_>> for CollectRemainingRewardsKeys {
    fn from(accounts: CollectRemainingRewardsAccounts) -> Self {
        Self {
            reward_funder: *accounts.reward_funder.key,
            funder_token_account: *accounts.funder_token_account.key,
            pool_state: *accounts.pool_state.key,
            reward_token_vault: *accounts.reward_token_vault.key,
            reward_vault_mint: *accounts.reward_vault_mint.key,
            token_program: *accounts.token_program.key,
            token_program2022: *accounts.token_program2022.key,
            memo_program: *accounts.memo_program.key,
        }
    }
}
impl From<CollectRemainingRewardsKeys> for [AccountMeta; COLLECT_REMAINING_REWARDS_IX_ACCOUNTS_LEN] {
    fn from(keys: CollectRemainingRewardsKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.reward_funder,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.funder_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.pool_state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.reward_token_vault,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.reward_vault_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_program2022,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.memo_program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}

impl From<[Pubkey; COLLECT_REMAINING_REWARDS_IX_ACCOUNTS_LEN]> for CollectRemainingRewardsKeys {
    fn from(pubkeys: [Pubkey; COLLECT_REMAINING_REWARDS_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            reward_funder: pubkeys[0],
            funder_token_account: pubkeys[1],
            pool_state: pubkeys[2],
            reward_token_vault: pubkeys[3],
            reward_vault_mint: pubkeys[4],
            token_program: pubkeys[5],
            token_program2022: pubkeys[6],
            memo_program: pubkeys[7],
        }
    }
}
impl<'info> From<CollectRemainingRewardsAccounts<'_, 'info>>
for [AccountInfo<'info>; COLLECT_REMAINING_REWARDS_IX_ACCOUNTS_LEN] {
    fn from(accounts: CollectRemainingRewardsAccounts<'_, 'info>) -> Self {
        [
            accounts.reward_funder.clone(),
            accounts.funder_token_account.clone(),
            accounts.pool_state.clone(),
            accounts.reward_token_vault.clone(),
            accounts.reward_vault_mint.clone(),
            accounts.token_program.clone(),
            accounts.token_program2022.clone(),
            accounts.memo_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; COLLECT_REMAINING_REWARDS_IX_ACCOUNTS_LEN]>
for CollectRemainingRewardsAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; COLLECT_REMAINING_REWARDS_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            reward_funder: &arr[0],
            funder_token_account: &arr[1],
            pool_state: &arr[2],
            reward_token_vault: &arr[3],
            reward_vault_mint: &arr[4],
            token_program: &arr[5],
            token_program2022: &arr[6],
            memo_program: &arr[7],
        }
    }
}
pub const COLLECT_REMAINING_REWARDS_IX_DISCM: u8 = 9u8;
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CollectRemainingRewardsIxArgs {
    pub reward_index: u8,
}
#[derive(Clone, Debug, PartialEq)]
pub struct CollectRemainingRewardsIxData(pub CollectRemainingRewardsIxArgs);
impl From<CollectRemainingRewardsIxArgs> for CollectRemainingRewardsIxData {
    fn from(args: CollectRemainingRewardsIxArgs) -> Self {
        Self(args)
    }
}
impl CollectRemainingRewardsIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
          let mut reader = buf;
        let mut maybe_discm_buf = [0u8; 1];
        reader.read_exact(&mut maybe_discm_buf)?;
        let maybe_discm = maybe_discm_buf[0];
        if maybe_discm != COLLECT_REMAINING_REWARDS_IX_DISCM {
            return Err(
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!(
                        "discm does not match. Expected: {:?}. Received: {:?}",
                        COLLECT_REMAINING_REWARDS_IX_DISCM, maybe_discm
                    ),
                ),
            );
        }
        Ok(Self(CollectRemainingRewardsIxArgs::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&[COLLECT_REMAINING_REWARDS_IX_DISCM])?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>>  {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn collect_remaining_rewards_ix_with_program_id(
    program_id: Pubkey,
    keys: CollectRemainingRewardsKeys,
    args: CollectRemainingRewardsIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; COLLECT_REMAINING_REWARDS_IX_ACCOUNTS_LEN] = keys.into();
    let data: CollectRemainingRewardsIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn collect_remaining_rewards_ix(
    keys: CollectRemainingRewardsKeys,
    args: CollectRemainingRewardsIxArgs,
) -> std::io::Result<Instruction> {
    collect_remaining_rewards_ix_with_program_id(crate::ID, keys, args)
}
pub fn collect_remaining_rewards_invoke_with_program_id(
    program_id: Pubkey,
    accounts: CollectRemainingRewardsAccounts<'_, '_>,
    args: CollectRemainingRewardsIxArgs,
) -> ProgramResult {
    let keys: CollectRemainingRewardsKeys = accounts.into();
    let ix = collect_remaining_rewards_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn collect_remaining_rewards_invoke(
    accounts: CollectRemainingRewardsAccounts<'_, '_>,
    args: CollectRemainingRewardsIxArgs,
) -> ProgramResult {
    collect_remaining_rewards_invoke_with_program_id(crate::ID, accounts, args)
}
pub fn collect_remaining_rewards_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: CollectRemainingRewardsAccounts<'_, '_>,
    args: CollectRemainingRewardsIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: CollectRemainingRewardsKeys = accounts.into();
    let ix = collect_remaining_rewards_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn collect_remaining_rewards_invoke_signed(
    accounts: CollectRemainingRewardsAccounts<'_, '_>,
    args: CollectRemainingRewardsIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    collect_remaining_rewards_invoke_signed_with_program_id(crate::ID, accounts, args, seeds)
}
pub fn collect_remaining_rewards_verify_account_keys(
    accounts: CollectRemainingRewardsAccounts<'_, '_>,
    keys: CollectRemainingRewardsKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.reward_funder.key, keys.reward_funder),
        (*accounts.funder_token_account.key, keys.funder_token_account),
        (*accounts.pool_state.key, keys.pool_state),
        (*accounts.reward_token_vault.key, keys.reward_token_vault),
        (*accounts.reward_vault_mint.key, keys.reward_vault_mint),
        (*accounts.token_program.key, keys.token_program),
        (*accounts.token_program2022.key, keys.token_program2022),
        (*accounts.memo_program.key, keys.memo_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn collect_remaining_rewards_verify_writable_privileges<'me, 'info>(
    accounts: CollectRemainingRewardsAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.funder_token_account,
        accounts.pool_state,
        ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn collect_remaining_rewards_verify_signer_privileges<'me, 'info>(
    accounts: CollectRemainingRewardsAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.reward_funder] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn collect_remaining_rewards_verify_account_privileges<'me, 'info>(
    accounts: CollectRemainingRewardsAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    collect_remaining_rewards_verify_writable_privileges(accounts)?;
    collect_remaining_rewards_verify_signer_privileges(accounts)?;
    Ok(())
}

pub const UPDATE_REWARD_INFOS_IX_ACCOUNTS_LEN: usize = 1;
#[derive(Copy, Clone, Debug)]
pub struct UpdateRewardInfosAccounts<'me, 'info> {
    pub pool_state: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct UpdateRewardInfosKeys {
    pub pool_state: Pubkey,
}
impl From<UpdateRewardInfosAccounts<'_, '_>> for UpdateRewardInfosKeys {
    fn from(accounts: UpdateRewardInfosAccounts) -> Self {
        Self {
            pool_state: *accounts.pool_state.key,
        }
    }
}
impl From<UpdateRewardInfosKeys> for [AccountMeta; UPDATE_REWARD_INFOS_IX_ACCOUNTS_LEN] {
    fn from(keys: UpdateRewardInfosKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.pool_state,
                is_signer: false,
                is_writable: true,
            },
        ]
    }
}
impl From<[Pubkey; UPDATE_REWARD_INFOS_IX_ACCOUNTS_LEN]> for UpdateRewardInfosKeys {
    fn from(pubkeys: [Pubkey; UPDATE_REWARD_INFOS_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            pool_state: pubkeys[0],
        }
    }
}
impl<'info> From<UpdateRewardInfosAccounts<'_, 'info>>
for [AccountInfo<'info>; UPDATE_REWARD_INFOS_IX_ACCOUNTS_LEN] {
    fn from(accounts: UpdateRewardInfosAccounts<'_, 'info>) -> Self {
        [
            accounts.pool_state.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; UPDATE_REWARD_INFOS_IX_ACCOUNTS_LEN]>
for UpdateRewardInfosAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; UPDATE_REWARD_INFOS_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            pool_state: &arr[0],
        }
    }
}
pub const UPDATE_REWARD_INFOS_IX_DISCM: u8 = 10u8;
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq, Default)]   
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct UpdateRewardInfosIxArgs;
#[derive(Clone, Debug, PartialEq)]
pub struct UpdateRewardInfosIxData(pub UpdateRewardInfosIxArgs);
impl From<UpdateRewardInfosIxArgs> for UpdateRewardInfosIxData {
    fn from(args: UpdateRewardInfosIxArgs) -> Self {
        Self(args)
    }
}
impl UpdateRewardInfosIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
          let mut reader = buf;
        let mut maybe_discm_buf = [0u8; 1];
        reader.read_exact(&mut maybe_discm_buf)?;
        let maybe_discm = maybe_discm_buf[0];
        if maybe_discm != UPDATE_REWARD_INFOS_IX_DISCM {
            return Err(
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!(
                        "discm does not match. Expected: {:?}. Received: {:?}",
                        UPDATE_REWARD_INFOS_IX_DISCM, maybe_discm
                    ),
                ),
            );
        }
        Ok(Self(UpdateRewardInfosIxArgs::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&[UPDATE_REWARD_INFOS_IX_DISCM])?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>>  {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn update_reward_infos_ix_with_program_id(
    program_id: Pubkey,
    keys: UpdateRewardInfosKeys,
    args: UpdateRewardInfosIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; UPDATE_REWARD_INFOS_IX_ACCOUNTS_LEN] = keys.into();
    let data: UpdateRewardInfosIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn update_reward_infos_ix(
    keys: UpdateRewardInfosKeys,
    args: UpdateRewardInfosIxArgs,
) ->  std::io::Result<Instruction> {
    update_reward_infos_ix_with_program_id(crate::ID, keys, args)
}
pub fn update_reward_infos_invoke_with_program_id(
    program_id: Pubkey,
    accounts: UpdateRewardInfosAccounts<'_, '_>,
    args: UpdateRewardInfosIxArgs,
) -> ProgramResult {
    let keys: UpdateRewardInfosKeys = accounts.into();
    let ix = update_reward_infos_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn update_reward_infos_invoke(
    accounts: UpdateRewardInfosAccounts<'_, '_>,
    args: UpdateRewardInfosIxArgs,
) -> ProgramResult {
    update_reward_infos_invoke_with_program_id(crate::ID, accounts, args)
}
pub fn update_reward_infos_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: UpdateRewardInfosAccounts<'_, '_>,
    args: UpdateRewardInfosIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: UpdateRewardInfosKeys = accounts.into();
    let ix = update_reward_infos_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn update_reward_infos_invoke_signed(
    accounts: UpdateRewardInfosAccounts<'_, '_>,
    args: UpdateRewardInfosIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    update_reward_infos_invoke_signed_with_program_id(crate::ID, accounts, args, seeds)
}
pub fn update_reward_infos_verify_account_keys(
    accounts: UpdateRewardInfosAccounts<'_, '_>,
    keys: UpdateRewardInfosKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.pool_state.key, keys.pool_state),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn update_reward_infos_verify_writable_privileges<'me, 'info>(
    accounts: UpdateRewardInfosAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.pool_state] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn update_reward_infos_verify_signer_privileges<'me, 'info>(
    accounts: UpdateRewardInfosAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    Ok(())
}
pub fn update_reward_infos_verify_account_privileges<'me, 'info>(
    accounts: UpdateRewardInfosAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    update_reward_infos_verify_writable_privileges(accounts)?;
    update_reward_infos_verify_signer_privileges(accounts)?;
    Ok(())
}



pub const SET_REWARD_PARAMS_IX_ACCOUNTS_LEN: usize = 6;

#[derive(Copy, Clone, Debug)]
pub struct SetRewardParamsAccounts<'me, 'info> {
    pub authority: &'me AccountInfo<'info>,
    pub amm_config: &'me AccountInfo<'info>,
    pub pool_state: &'me AccountInfo<'info>,
    pub operation_state: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub token_program2022: &'me AccountInfo<'info>,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SetRewardParamsKeys {
    pub authority: Pubkey,
    pub amm_config: Pubkey,
    pub pool_state: Pubkey,
    pub operation_state: Pubkey,
    pub token_program: Pubkey,
    pub token_program2022: Pubkey,
}

impl From<SetRewardParamsAccounts<'_, '_>> for SetRewardParamsKeys {
    fn from(accounts: SetRewardParamsAccounts) -> Self {
        Self {
            authority: *accounts.authority.key,
            amm_config: *accounts.amm_config.key,
            pool_state: *accounts.pool_state.key,
            operation_state: *accounts.operation_state.key,
            token_program: *accounts.token_program.key,
            token_program2022: *accounts.token_program2022.key,
        }
    }
}

impl From<SetRewardParamsKeys> for [AccountMeta; SET_REWARD_PARAMS_IX_ACCOUNTS_LEN] {
    fn from(keys: SetRewardParamsKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.authority,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.amm_config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pool_state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.operation_state,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_program2022,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}

impl From<[Pubkey; SET_REWARD_PARAMS_IX_ACCOUNTS_LEN]> for SetRewardParamsKeys {
    fn from(pubkeys: [Pubkey; SET_REWARD_PARAMS_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            authority: pubkeys[0],
            amm_config: pubkeys[1],
            pool_state: pubkeys[2],
            operation_state: pubkeys[3],
            token_program: pubkeys[4],
            token_program2022: pubkeys[5],
        }
    }
}

impl<'info> From<SetRewardParamsAccounts<'_, 'info>> for [AccountInfo<'info>; SET_REWARD_PARAMS_IX_ACCOUNTS_LEN] {
    fn from(accounts: SetRewardParamsAccounts<'_, 'info>) -> Self {
        [
            accounts.authority.clone(),
            accounts.amm_config.clone(),
            accounts.pool_state.clone(),
            accounts.operation_state.clone(),
            accounts.token_program.clone(),
            accounts.token_program2022.clone(),
        ]
    }
}

impl<'me, 'info> From<&'me [AccountInfo<'info>; SET_REWARD_PARAMS_IX_ACCOUNTS_LEN]> for SetRewardParamsAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; SET_REWARD_PARAMS_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            authority: &arr[0],
            amm_config: &arr[1],
            pool_state: &arr[2],
            operation_state: &arr[3],
            token_program: &arr[4],
            token_program2022: &arr[5],
        }
    }
}

pub const SET_REWARD_PARAMS_IX_DISCM:u8 = 11u8;

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SetRewardParamsIxArgs {
    pub reward_index: u8,
    pub emissions_per_second_x64: u128,
    pub open_time: u64,
    pub end_time: u64,
}

#[derive(Clone, Debug, PartialEq)]
pub struct SetRewardParamsIxData(pub SetRewardParamsIxArgs);

impl From<SetRewardParamsIxArgs> for SetRewardParamsIxData {
    fn from(args: SetRewardParamsIxArgs) -> Self {
        Self(args)
    }
}

impl SetRewardParamsIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
          let mut reader = buf;
        let mut maybe_discm_buf = [0u8; 1];
        reader.read_exact(&mut maybe_discm_buf)?;
        let maybe_discm = maybe_discm_buf[0];
        if maybe_discm != SET_REWARD_PARAMS_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    SET_REWARD_PARAMS_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(SetRewardParamsIxArgs::deserialize(&mut reader)?))
    }

    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&[SET_REWARD_PARAMS_IX_DISCM])?;
        self.0.serialize(&mut writer)
    }

    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>>  {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}

pub fn set_reward_params_ix_with_program_id(
    program_id: Pubkey,
    keys: SetRewardParamsKeys,
    args: SetRewardParamsIxArgs,
) ->  std::io::Result<Instruction> {
    let metas: [AccountMeta; SET_REWARD_PARAMS_IX_ACCOUNTS_LEN] = keys.into();
    let data: SetRewardParamsIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}

pub fn set_reward_params_ix(
    keys: SetRewardParamsKeys,
    args: SetRewardParamsIxArgs,
) ->  std::io::Result<Instruction> {
    set_reward_params_ix_with_program_id(crate::ID, keys, args)
}

pub fn set_reward_params_invoke_with_program_id(
    program_id: Pubkey,
    accounts: SetRewardParamsAccounts<'_, '_>,
    args: SetRewardParamsIxArgs,
) -> ProgramResult {
    let keys: SetRewardParamsKeys = accounts.into();
    let ix = set_reward_params_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}

pub fn set_reward_params_invoke(
    accounts: SetRewardParamsAccounts<'_, '_>,
    args: SetRewardParamsIxArgs,
) -> ProgramResult {
    set_reward_params_invoke_with_program_id(crate::ID, accounts, args)
}

pub fn set_reward_params_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: SetRewardParamsAccounts<'_, '_>,
    args: SetRewardParamsIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: SetRewardParamsKeys = accounts.into();
    let ix = set_reward_params_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}

pub fn set_reward_params_invoke_signed(
    accounts: SetRewardParamsAccounts<'_, '_>,
    args: SetRewardParamsIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    set_reward_params_invoke_signed_with_program_id(crate::ID, accounts, args, seeds)
}

pub fn set_reward_params_verify_account_keys(
    accounts: SetRewardParamsAccounts<'_, '_>,
    keys: SetRewardParamsKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.authority.key, keys.authority),
        (*accounts.amm_config.key, keys.amm_config),
        (*accounts.pool_state.key, keys.pool_state),
        (*accounts.operation_state.key, keys.operation_state),
        (*accounts.token_program.key, keys.token_program),
        (*accounts.token_program2022.key, keys.token_program2022),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}

pub fn set_reward_params_verify_writable_privileges<'me, 'info>(
    accounts: SetRewardParamsAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    if !accounts.pool_state.is_writable {
        return Err((accounts.pool_state, ProgramError::InvalidAccountData));
    }
    Ok(())
}

pub fn set_reward_params_verify_signer_privileges<'me, 'info>(
    accounts: SetRewardParamsAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    if !accounts.authority.is_signer {
        return Err((accounts.authority, ProgramError::MissingRequiredSignature));
    }
    Ok(())
}

pub fn set_reward_params_verify_account_privileges<'me, 'info>(
    accounts: SetRewardParamsAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    set_reward_params_verify_writable_privileges(accounts)?;
    set_reward_params_verify_signer_privileges(accounts)?;
    Ok(())
}


pub const COLLECT_PROTOCOL_FEE_IX_ACCOUNTS_LEN: usize = 11;

#[derive(Copy, Clone, Debug)]
pub struct CollectProtocolFeeAccounts<'me, 'info> {
    pub owner: &'me AccountInfo<'info>,
    pub pool_state: &'me AccountInfo<'info>,
    pub amm_config: &'me AccountInfo<'info>,
    pub token_vault0: &'me AccountInfo<'info>,
    pub token_vault1: &'me AccountInfo<'info>,
    pub vault0_mint: &'me AccountInfo<'info>,
    pub vault1_mint: &'me AccountInfo<'info>,
    pub recipient_token_account0: &'me AccountInfo<'info>,
    pub recipient_token_account1: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub token_program2022: &'me AccountInfo<'info>,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CollectProtocolFeeKeys {
    pub owner: Pubkey,
    pub pool_state: Pubkey,
    pub amm_config: Pubkey,
    pub token_vault0: Pubkey,
    pub token_vault1: Pubkey,
    pub vault0_mint: Pubkey,
    pub vault1_mint: Pubkey,
    pub recipient_token_account0: Pubkey,
    pub recipient_token_account1: Pubkey,
    pub token_program: Pubkey,
    pub token_program2022: Pubkey,
}

impl From<CollectProtocolFeeAccounts<'_, '_>> for CollectProtocolFeeKeys {
    fn from(accounts: CollectProtocolFeeAccounts) -> Self {
        Self {
            owner: *accounts.owner.key,
            pool_state: *accounts.pool_state.key,
            amm_config: *accounts.amm_config.key,
            token_vault0: *accounts.token_vault0.key,
            token_vault1: *accounts.token_vault1.key,
            vault0_mint: *accounts.vault0_mint.key,
            vault1_mint: *accounts.vault1_mint.key,
            recipient_token_account0: *accounts.recipient_token_account0.key,
            recipient_token_account1: *accounts.recipient_token_account1.key,
            token_program: *accounts.token_program.key,
            token_program2022: *accounts.token_program2022.key,
        }
    }
}

impl From<CollectProtocolFeeKeys> for [AccountMeta; COLLECT_PROTOCOL_FEE_IX_ACCOUNTS_LEN] {
    fn from(keys: CollectProtocolFeeKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.owner,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pool_state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.amm_config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_vault0,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_vault1,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.vault0_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.vault1_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.recipient_token_account0,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.recipient_token_account1,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_program2022,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}

impl From<[Pubkey; COLLECT_PROTOCOL_FEE_IX_ACCOUNTS_LEN]> for CollectProtocolFeeKeys {
    fn from(pubkeys: [Pubkey; COLLECT_PROTOCOL_FEE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            owner: pubkeys[0],
            pool_state: pubkeys[1],
            amm_config: pubkeys[2],
            token_vault0: pubkeys[3],
            token_vault1: pubkeys[4],
            vault0_mint: pubkeys[5],
            vault1_mint: pubkeys[6],
            recipient_token_account0: pubkeys[7],
            recipient_token_account1: pubkeys[8],
            token_program: pubkeys[9],
            token_program2022: pubkeys[10],
        }
    }
}

impl<'info> From<CollectProtocolFeeAccounts<'_, 'info>> for [AccountInfo<'info>; COLLECT_PROTOCOL_FEE_IX_ACCOUNTS_LEN] {
    fn from(accounts: CollectProtocolFeeAccounts<'_, 'info>) -> Self {
        [
            accounts.owner.clone(),
            accounts.pool_state.clone(),
            accounts.amm_config.clone(),
            accounts.token_vault0.clone(),
            accounts.token_vault1.clone(),
            accounts.vault0_mint.clone(),
            accounts.vault1_mint.clone(),
            accounts.recipient_token_account0.clone(),
            accounts.recipient_token_account1.clone(),
            accounts.token_program.clone(),
            accounts.token_program2022.clone(),
        ]
    }
}

impl<'me, 'info> From<&'me [AccountInfo<'info>; COLLECT_PROTOCOL_FEE_IX_ACCOUNTS_LEN]> for CollectProtocolFeeAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; COLLECT_PROTOCOL_FEE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            owner: &arr[0],
            pool_state: &arr[1],
            amm_config: &arr[2],
            token_vault0: &arr[3],
            token_vault1: &arr[4],
            vault0_mint: &arr[5],
            vault1_mint: &arr[6],
            recipient_token_account0: &arr[7],
            recipient_token_account1: &arr[8],
            token_program: &arr[9],
            token_program2022: &arr[10],
        }
    }
}

pub const COLLECT_PROTOCOL_FEE_IX_DISCM:u8 = 12u8;

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CollectProtocolFeeIxArgs {
    pub amount0_requested: u64,
    pub amount1_requested: u64,
}

#[derive(Clone, Debug, PartialEq)]
pub struct CollectProtocolFeeIxData(pub CollectProtocolFeeIxArgs);

impl From<CollectProtocolFeeIxArgs> for CollectProtocolFeeIxData {
    fn from(args: CollectProtocolFeeIxArgs) -> Self {
        Self(args)
    }
}

impl CollectProtocolFeeIxData {
    pub fn deserialize(buf: &[u8]) ->std::io::Result<Self> {
          let mut reader = buf;
        let mut maybe_discm_buf = [0u8; 1];
        reader.read_exact(&mut maybe_discm_buf)?;
        let maybe_discm = maybe_discm_buf[0];
        if maybe_discm != COLLECT_PROTOCOL_FEE_IX_DISCM {
            return Err(std::io::Error::new(
               std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    COLLECT_PROTOCOL_FEE_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(CollectProtocolFeeIxArgs::deserialize(&mut reader)?))
    }

    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&[COLLECT_PROTOCOL_FEE_IX_DISCM])?;
        self.0.serialize(&mut writer)
    }

    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>>  {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}

pub fn collect_protocol_fee_ix_with_program_id(
    program_id: Pubkey,
    keys: CollectProtocolFeeKeys,
    args: CollectProtocolFeeIxArgs,
) ->  std::io::Result<Instruction> {
    let metas: [AccountMeta; COLLECT_PROTOCOL_FEE_IX_ACCOUNTS_LEN] = keys.into();
    let data: CollectProtocolFeeIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}

pub fn collect_protocol_fee_ix(
    keys: CollectProtocolFeeKeys,
    args: CollectProtocolFeeIxArgs,
) ->  std::io::Result<Instruction> {
    collect_protocol_fee_ix_with_program_id(crate::ID, keys, args)
}

pub fn collect_protocol_fee_invoke_with_program_id(
    program_id: Pubkey,
    accounts: CollectProtocolFeeAccounts<'_, '_>,
    args: CollectProtocolFeeIxArgs,
) -> ProgramResult {
    let keys: CollectProtocolFeeKeys = accounts.into();
    let ix = collect_protocol_fee_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}

pub fn collect_protocol_fee_invoke(
    accounts: CollectProtocolFeeAccounts<'_, '_>,
    args: CollectProtocolFeeIxArgs,
) -> ProgramResult {
    collect_protocol_fee_invoke_with_program_id(crate::ID, accounts, args)
}

pub fn collect_protocol_fee_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: CollectProtocolFeeAccounts<'_, '_>,
    args: CollectProtocolFeeIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: CollectProtocolFeeKeys = accounts.into();
    let ix = collect_protocol_fee_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}

pub fn collect_protocol_fee_invoke_signed(
    accounts: CollectProtocolFeeAccounts<'_, '_>,
    args: CollectProtocolFeeIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    collect_protocol_fee_invoke_signed_with_program_id(crate::ID, accounts, args, seeds)
}

pub fn collect_protocol_fee_verify_account_keys(
    accounts: CollectProtocolFeeAccounts<'_, '_>,
    keys: CollectProtocolFeeKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.owner.key, keys.owner),
        (*accounts.pool_state.key, keys.pool_state),
        (*accounts.amm_config.key, keys.amm_config),
        (*accounts.token_vault0.key, keys.token_vault0),
        (*accounts.token_vault1.key, keys.token_vault1),
        (*accounts.vault0_mint.key, keys.vault0_mint),
        (*accounts.vault1_mint.key, keys.vault1_mint),
        (*accounts.recipient_token_account0.key, keys.recipient_token_account0),
        (*accounts.recipient_token_account1.key, keys.recipient_token_account1),
        (*accounts.token_program.key, keys.token_program),
        (*accounts.token_program2022.key, keys.token_program2022),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}

pub fn collect_protocol_fee_verify_writable_privileges<'me, 'info>(
    accounts: CollectProtocolFeeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.pool_state,
        accounts.token_vault0,
        accounts.token_vault1,
        accounts.recipient_token_account0,
        accounts.recipient_token_account1,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}

pub fn collect_protocol_fee_verify_signer_privileges<'me, 'info>(
    accounts: CollectProtocolFeeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    if !accounts.owner.is_signer {
        return Err((accounts.owner, ProgramError::MissingRequiredSignature));
    }
    Ok(())
}

pub fn collect_protocol_fee_verify_account_privileges<'me, 'info>(
    accounts: CollectProtocolFeeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    collect_protocol_fee_verify_writable_privileges(accounts)?;
    collect_protocol_fee_verify_signer_privileges(accounts)?;
    Ok(())
}


pub const COLLECT_FUND_FEE_IX_ACCOUNTS_LEN: usize = 11;

#[derive(Copy, Clone, Debug)]
pub struct CollectFundFeeAccounts<'me, 'info> {
    pub owner: &'me AccountInfo<'info>,
    pub pool_state: &'me AccountInfo<'info>,
    pub amm_config: &'me AccountInfo<'info>,
    pub token_vault0: &'me AccountInfo<'info>,
    pub token_vault1: &'me AccountInfo<'info>,
    pub vault0_mint: &'me AccountInfo<'info>,
    pub vault1_mint: &'me AccountInfo<'info>,
    pub recipient_token_account0: &'me AccountInfo<'info>,
    pub recipient_token_account1: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub token_program2022: &'me AccountInfo<'info>,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CollectFundFeeKeys {
    pub owner: Pubkey,
    pub pool_state: Pubkey,
    pub amm_config: Pubkey,
    pub token_vault0: Pubkey,
    pub token_vault1: Pubkey,
    pub vault0_mint: Pubkey,
    pub vault1_mint: Pubkey,
    pub recipient_token_account0: Pubkey,
    pub recipient_token_account1: Pubkey,
    pub token_program: Pubkey,
    pub token_program2022: Pubkey,
}

impl From<CollectFundFeeAccounts<'_, '_>> for CollectFundFeeKeys {
    fn from(accounts: CollectFundFeeAccounts) -> Self {
        Self {
            owner: *accounts.owner.key,
            pool_state: *accounts.pool_state.key,
            amm_config: *accounts.amm_config.key,
            token_vault0: *accounts.token_vault0.key,
            token_vault1: *accounts.token_vault1.key,
            vault0_mint: *accounts.vault0_mint.key,
            vault1_mint: *accounts.vault1_mint.key,
            recipient_token_account0: *accounts.recipient_token_account0.key,
            recipient_token_account1: *accounts.recipient_token_account1.key,
            token_program: *accounts.token_program.key,
            token_program2022: *accounts.token_program2022.key,
        }
    }
}

impl From<CollectFundFeeKeys> for [AccountMeta; COLLECT_FUND_FEE_IX_ACCOUNTS_LEN] {
    fn from(keys: CollectFundFeeKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.owner,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pool_state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.amm_config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_vault0,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_vault1,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.vault0_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.vault1_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.recipient_token_account0,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.recipient_token_account1,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_program2022,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}

impl From<[Pubkey; COLLECT_FUND_FEE_IX_ACCOUNTS_LEN]> for CollectFundFeeKeys {
    fn from(pubkeys: [Pubkey; COLLECT_FUND_FEE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            owner: pubkeys[0],
            pool_state: pubkeys[1],
            amm_config: pubkeys[2],
            token_vault0: pubkeys[3],
            token_vault1: pubkeys[4],
            vault0_mint: pubkeys[5],
            vault1_mint: pubkeys[6],
            recipient_token_account0: pubkeys[7],
            recipient_token_account1: pubkeys[8],
            token_program: pubkeys[9],
            token_program2022: pubkeys[10],
        }
    }
}

impl<'info> From<CollectFundFeeAccounts<'_, 'info>> for [AccountInfo<'info>; COLLECT_FUND_FEE_IX_ACCOUNTS_LEN] {
    fn from(accounts: CollectFundFeeAccounts<'_, 'info>) -> Self {
        [
            accounts.owner.clone(),
            accounts.pool_state.clone(),
            accounts.amm_config.clone(),
            accounts.token_vault0.clone(),
            accounts.token_vault1.clone(),
            accounts.vault0_mint.clone(),
            accounts.vault1_mint.clone(),
            accounts.recipient_token_account0.clone(),
            accounts.recipient_token_account1.clone(),
            accounts.token_program.clone(),
            accounts.token_program2022.clone(),
        ]
    }
}

impl<'me, 'info> From<&'me [AccountInfo<'info>; COLLECT_FUND_FEE_IX_ACCOUNTS_LEN]> for CollectFundFeeAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; COLLECT_FUND_FEE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            owner: &arr[0],
            pool_state: &arr[1],
            amm_config: &arr[2],
            token_vault0: &arr[3],
            token_vault1: &arr[4],
            vault0_mint: &arr[5],
            vault1_mint: &arr[6],
            recipient_token_account0: &arr[7],
            recipient_token_account1: &arr[8],
            token_program: &arr[9],
            token_program2022: &arr[10],
        }
    }
}

pub const COLLECT_FUND_FEE_IX_DISCM: u8 = 13u8;

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CollectFundFeeIxArgs {
    pub amount0_requested: u64,
    pub amount1_requested: u64,
}

#[derive(Clone, Debug, PartialEq)]
pub struct CollectFundFeeIxData(pub CollectFundFeeIxArgs);

impl From<CollectFundFeeIxArgs> for CollectFundFeeIxData {
    fn from(args: CollectFundFeeIxArgs) -> Self {
        Self(args)
    }
}

impl CollectFundFeeIxData {
    pub fn deserialize(buf: &[u8]) ->std::io::Result<Self> {
          let mut reader = buf;
        let mut maybe_discm_buf = [0u8; 1];
        reader.read_exact(&mut maybe_discm_buf)?;
        let maybe_discm = maybe_discm_buf[0];
        if maybe_discm != COLLECT_FUND_FEE_IX_DISCM {
            return Err(std::io::Error::new(
               std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    COLLECT_FUND_FEE_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(CollectFundFeeIxArgs::deserialize(&mut reader)?))
    }

    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&[COLLECT_FUND_FEE_IX_DISCM])?;
        self.0.serialize(&mut writer)
    }

    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>>  {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}

pub fn collect_fund_fee_ix_with_program_id(
    program_id: Pubkey,
    keys: CollectFundFeeKeys,
    args: CollectFundFeeIxArgs,
) ->  std::io::Result<Instruction> {
    let metas: [AccountMeta; COLLECT_FUND_FEE_IX_ACCOUNTS_LEN] = keys.into();
    let data: CollectFundFeeIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}

pub fn collect_fund_fee_ix(
    keys: CollectFundFeeKeys,
    args: CollectFundFeeIxArgs,
) ->  std::io::Result<Instruction> {
    collect_fund_fee_ix_with_program_id(crate::ID, keys, args)
}

pub fn collect_fund_fee_invoke_with_program_id(
    program_id: Pubkey,
    accounts: CollectFundFeeAccounts<'_, '_>,
    args: CollectFundFeeIxArgs,
) -> ProgramResult {
    let keys: CollectFundFeeKeys = accounts.into();
    let ix = collect_fund_fee_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}

pub fn collect_fund_fee_invoke(
    accounts: CollectFundFeeAccounts<'_, '_>,
    args: CollectFundFeeIxArgs,
) -> ProgramResult {
    collect_fund_fee_invoke_with_program_id(crate::ID, accounts, args)
}

pub fn collect_fund_fee_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: CollectFundFeeAccounts<'_, '_>,
    args: CollectFundFeeIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: CollectFundFeeKeys = accounts.into();
    let ix = collect_fund_fee_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}

pub fn collect_fund_fee_invoke_signed(
    accounts: CollectFundFeeAccounts<'_, '_>,
    args: CollectFundFeeIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    collect_fund_fee_invoke_signed_with_program_id(crate::ID, accounts, args, seeds)
}

pub fn collect_fund_fee_verify_account_keys(
    accounts: CollectFundFeeAccounts<'_, '_>,
    keys: CollectFundFeeKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.owner.key, keys.owner),
        (*accounts.pool_state.key, keys.pool_state),
        (*accounts.amm_config.key, keys.amm_config),
        (*accounts.token_vault0.key, keys.token_vault0),
        (*accounts.token_vault1.key, keys.token_vault1),
        (*accounts.vault0_mint.key, keys.vault0_mint),
        (*accounts.vault1_mint.key, keys.vault1_mint),
        (*accounts.recipient_token_account0.key, keys.recipient_token_account0),
        (*accounts.recipient_token_account1.key, keys.recipient_token_account1),
        (*accounts.token_program.key, keys.token_program),
        (*accounts.token_program2022.key, keys.token_program2022),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}

pub fn collect_fund_fee_verify_writable_privileges<'me, 'info>(
    accounts: CollectFundFeeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.pool_state,
        accounts.token_vault0,
        accounts.token_vault1,
        accounts.recipient_token_account0,
        accounts.recipient_token_account1,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}

pub fn collect_fund_fee_verify_signer_privileges<'me, 'info>(
    accounts: CollectFundFeeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    if !accounts.owner.is_signer {
        return Err((accounts.owner, ProgramError::MissingRequiredSignature));
    }
    Ok(())
}

pub fn collect_fund_fee_verify_account_privileges<'me, 'info>(
    accounts: CollectFundFeeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    collect_fund_fee_verify_writable_privileges(accounts)?;
    collect_fund_fee_verify_signer_privileges(accounts)?;
    Ok(())
}

// --- openPosition ---

pub const OPEN_POSITION_IX_ACCOUNTS_LEN: usize = 19;

#[derive(Copy, Clone, Debug)]
pub struct OpenPositionAccounts<'me, 'info> {
    pub payer: &'me AccountInfo<'info>,
    pub position_nft_owner: &'me AccountInfo<'info>,
    pub position_nft_mint: &'me AccountInfo<'info>,
    pub position_nft_account: &'me AccountInfo<'info>,
    pub metadata_account: &'me AccountInfo<'info>,
    pub pool_state: &'me AccountInfo<'info>,
    pub protocol_position: &'me AccountInfo<'info>,
    pub tick_array_lower: &'me AccountInfo<'info>,
    pub tick_array_upper: &'me AccountInfo<'info>,
    pub personal_position: &'me AccountInfo<'info>,
    pub token_account0: &'me AccountInfo<'info>,
    pub token_account1: &'me AccountInfo<'info>,
    pub token_vault0: &'me AccountInfo<'info>,
    pub token_vault1: &'me AccountInfo<'info>,
    pub rent: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub associated_token_program: &'me AccountInfo<'info>,
    pub metadata_program: &'me AccountInfo<'info>,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct OpenPositionKeys {
    pub payer: Pubkey,
    pub position_nft_owner: Pubkey,
    pub position_nft_mint: Pubkey,
    pub position_nft_account: Pubkey,
    pub metadata_account: Pubkey,
    pub pool_state: Pubkey,
    pub protocol_position: Pubkey,
    pub tick_array_lower: Pubkey,
    pub tick_array_upper: Pubkey,
    pub personal_position: Pubkey,
    pub token_account0: Pubkey,
    pub token_account1: Pubkey,
    pub token_vault0: Pubkey,
    pub token_vault1: Pubkey,
    pub rent: Pubkey,
    pub system_program: Pubkey,
    pub token_program: Pubkey,
    pub associated_token_program: Pubkey,
    pub metadata_program: Pubkey,
}

impl From<OpenPositionAccounts<'_, '_>> for OpenPositionKeys {
    fn from(accounts: OpenPositionAccounts) -> Self {
        Self {
            payer: *accounts.payer.key,
            position_nft_owner: *accounts.position_nft_owner.key,
            position_nft_mint: *accounts.position_nft_mint.key,
            position_nft_account: *accounts.position_nft_account.key,
            metadata_account: *accounts.metadata_account.key,
            pool_state: *accounts.pool_state.key,
            protocol_position: *accounts.protocol_position.key,
            tick_array_lower: *accounts.tick_array_lower.key,
            tick_array_upper: *accounts.tick_array_upper.key,
            personal_position: *accounts.personal_position.key,
            token_account0: *accounts.token_account0.key,
            token_account1: *accounts.token_account1.key,
            token_vault0: *accounts.token_vault0.key,
            token_vault1: *accounts.token_vault1.key,
            rent: *accounts.rent.key,
            system_program: *accounts.system_program.key,
            token_program: *accounts.token_program.key,
            associated_token_program: *accounts.associated_token_program.key,
            metadata_program: *accounts.metadata_program.key,
        }
    }
}

impl From<OpenPositionKeys> for [AccountMeta; OPEN_POSITION_IX_ACCOUNTS_LEN] {
    fn from(keys: OpenPositionKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.payer,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.position_nft_owner,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.position_nft_mint,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.position_nft_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.metadata_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.pool_state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.protocol_position,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.tick_array_lower,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.tick_array_upper,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.personal_position,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_account0,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_account1,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_vault0,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_vault1,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.rent,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.system_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.associated_token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.metadata_program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}

impl From<[Pubkey; OPEN_POSITION_IX_ACCOUNTS_LEN]> for OpenPositionKeys {
    fn from(pubkeys: [Pubkey; OPEN_POSITION_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            payer: pubkeys[0],
            position_nft_owner: pubkeys[1],
            position_nft_mint: pubkeys[2],
            position_nft_account: pubkeys[3],
            metadata_account: pubkeys[4],
            pool_state: pubkeys[5],
            protocol_position: pubkeys[6],
            tick_array_lower: pubkeys[7],
            tick_array_upper: pubkeys[8],
            personal_position: pubkeys[9],
            token_account0: pubkeys[10],
            token_account1: pubkeys[11],
            token_vault0: pubkeys[12],
            token_vault1: pubkeys[13],
            rent: pubkeys[14],
            system_program: pubkeys[15],
            token_program: pubkeys[16],
            associated_token_program: pubkeys[17],
            metadata_program: pubkeys[18],
        }
    }
}

impl<'info> From<OpenPositionAccounts<'_, 'info>> for [AccountInfo<'info>; OPEN_POSITION_IX_ACCOUNTS_LEN] {
    fn from(accounts: OpenPositionAccounts<'_, 'info>) -> Self {
        [
            accounts.payer.clone(),
            accounts.position_nft_owner.clone(),
            accounts.position_nft_mint.clone(),
            accounts.position_nft_account.clone(),
            accounts.metadata_account.clone(),
            accounts.pool_state.clone(),
            accounts.protocol_position.clone(),
            accounts.tick_array_lower.clone(),
            accounts.tick_array_upper.clone(),
            accounts.personal_position.clone(),
            accounts.token_account0.clone(),
            accounts.token_account1.clone(),
            accounts.token_vault0.clone(),
            accounts.token_vault1.clone(),
            accounts.rent.clone(),
            accounts.system_program.clone(),
            accounts.token_program.clone(),
            accounts.associated_token_program.clone(),
            accounts.metadata_program.clone(),
        ]
    }
}

impl<'me, 'info> From<&'me [AccountInfo<'info>; OPEN_POSITION_IX_ACCOUNTS_LEN]> for OpenPositionAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; OPEN_POSITION_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            payer: &arr[0],
            position_nft_owner: &arr[1],
            position_nft_mint: &arr[2],
            position_nft_account: &arr[3],
            metadata_account: &arr[4],
            pool_state: &arr[5],
            protocol_position: &arr[6],
            tick_array_lower: &arr[7],
            tick_array_upper: &arr[8],
            personal_position: &arr[9],
            token_account0: &arr[10],
            token_account1: &arr[11],
            token_vault0: &arr[12],
            token_vault1: &arr[13],
            rent: &arr[14],
            system_program: &arr[15],
            token_program: &arr[16],
            associated_token_program: &arr[17],
            metadata_program: &arr[18],
        }
    }
}

pub const OPEN_POSITION_IX_DISCM: u8 = 77u8;

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct OpenPositionIxArgs {
    pub tick_lower_index: i32,
    pub tick_upper_index: i32,
    pub tick_array_lower_start_index: i32,
    pub tick_array_upper_start_index: i32,
    pub liquidity: u128,
    pub amount0_max: u64,
    pub amount1_max: u64,
}

#[derive(Clone, Debug, PartialEq)]
pub struct OpenPositionIxData(pub OpenPositionIxArgs);

impl From<OpenPositionIxArgs> for OpenPositionIxData {
    fn from(args: OpenPositionIxArgs) -> Self {
        Self(args)
    }
}

impl OpenPositionIxData {
    pub fn deserialize(buf: &[u8]) ->std::io::Result<Self> {
          let mut reader = buf;
        let mut maybe_discm_buf = [0u8; 1];
        reader.read_exact(&mut maybe_discm_buf)?;
        let maybe_discm = maybe_discm_buf[0];
        if maybe_discm != OPEN_POSITION_IX_DISCM {
            return Err(std::io::Error::new(
               std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    OPEN_POSITION_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(OpenPositionIxArgs::deserialize(&mut reader)?))
    }

    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&[OPEN_POSITION_IX_DISCM])?;
        self.0.serialize(&mut writer)
    }

    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>>  {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}

pub fn open_position_ix_with_program_id(
    program_id: Pubkey,
    keys: OpenPositionKeys,
    args: OpenPositionIxArgs,
) ->  std::io::Result<Instruction> {
    let metas: [AccountMeta; OPEN_POSITION_IX_ACCOUNTS_LEN] = keys.into();
    let data: OpenPositionIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}

pub fn open_position_ix(
    keys: OpenPositionKeys,
    args: OpenPositionIxArgs,
) ->  std::io::Result<Instruction> {
    open_position_ix_with_program_id(crate::ID, keys, args)
}

pub fn open_position_invoke_with_program_id(
    program_id: Pubkey,
    accounts: OpenPositionAccounts<'_, '_>,
    args: OpenPositionIxArgs,
) -> ProgramResult {
    let keys: OpenPositionKeys = accounts.into();
    let ix = open_position_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}

pub fn open_position_invoke(
    accounts: OpenPositionAccounts<'_, '_>,
    args: OpenPositionIxArgs,
) -> ProgramResult {
    open_position_invoke_with_program_id(crate::ID, accounts, args)
}

pub fn open_position_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: OpenPositionAccounts<'_, '_>,
    args: OpenPositionIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: OpenPositionKeys = accounts.into();
    let ix = open_position_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}

pub fn open_position_invoke_signed(
    accounts: OpenPositionAccounts<'_, '_>,
    args: OpenPositionIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    open_position_invoke_signed_with_program_id(crate::ID, accounts, args, seeds)
}

pub fn open_position_verify_account_keys(
    accounts: OpenPositionAccounts<'_, '_>,
    keys: OpenPositionKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.payer.key, keys.payer),
        (*accounts.position_nft_owner.key, keys.position_nft_owner),
        (*accounts.position_nft_mint.key, keys.position_nft_mint),
        (*accounts.position_nft_account.key, keys.position_nft_account),
        (*accounts.metadata_account.key, keys.metadata_account),
        (*accounts.pool_state.key, keys.pool_state),
        (*accounts.protocol_position.key, keys.protocol_position),
        (*accounts.tick_array_lower.key, keys.tick_array_lower),
        (*accounts.tick_array_upper.key, keys.tick_array_upper),
        (*accounts.personal_position.key, keys.personal_position),
        (*accounts.token_account0.key, keys.token_account0),
        (*accounts.token_account1.key, keys.token_account1),
        (*accounts.token_vault0.key, keys.token_vault0),
        (*accounts.token_vault1.key, keys.token_vault1),
        (*accounts.rent.key, keys.rent),
        (*accounts.system_program.key, keys.system_program),
        (*accounts.token_program.key, keys.token_program),
        (*accounts.associated_token_program.key, keys.associated_token_program),
        (*accounts.metadata_program.key, keys.metadata_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}

pub fn open_position_verify_writable_privileges<'me, 'info>(
    accounts: OpenPositionAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.payer,
        accounts.position_nft_mint,
        accounts.position_nft_account,
        accounts.metadata_account,
        accounts.pool_state,
        accounts.protocol_position,
        accounts.tick_array_lower,
        accounts.tick_array_upper,
        accounts.personal_position,
        accounts.token_account0,
        accounts.token_account1,
        accounts.token_vault0,
        accounts.token_vault1,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}

pub fn open_position_verify_signer_privileges<'me, 'info>(
    accounts: OpenPositionAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.payer, accounts.position_nft_mint] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}

pub fn open_position_verify_account_privileges<'me, 'info>(
    accounts: OpenPositionAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    open_position_verify_writable_privileges(accounts)?;
    open_position_verify_signer_privileges(accounts)?;
    Ok(())
}


pub const OPEN_POSITION_V2_IX_ACCOUNTS_LEN: usize = 22;

#[derive(Copy, Clone, Debug)]
pub struct OpenPositionV2Accounts<'me, 'info> {
    pub payer: &'me AccountInfo<'info>,
    pub position_nft_owner: &'me AccountInfo<'info>,
    pub position_nft_mint: &'me AccountInfo<'info>,
    pub position_nft_account: &'me AccountInfo<'info>,
    pub metadata_account: &'me AccountInfo<'info>,
    pub pool_state: &'me AccountInfo<'info>,
    pub protocol_position: &'me AccountInfo<'info>,
    pub tick_array_lower: &'me AccountInfo<'info>,
    pub tick_array_upper: &'me AccountInfo<'info>,
    pub personal_position: &'me AccountInfo<'info>,
    pub token_account0: &'me AccountInfo<'info>,
    pub token_account1: &'me AccountInfo<'info>,
    pub token_vault0: &'me AccountInfo<'info>,
    pub token_vault1: &'me AccountInfo<'info>,
    pub rent: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub associated_token_program: &'me AccountInfo<'info>,
    pub metadata_program: &'me AccountInfo<'info>,
    pub token_program2022: &'me AccountInfo<'info>,
    pub vault0_mint: &'me AccountInfo<'info>,
    pub vault1_mint: &'me AccountInfo<'info>,
}

#[derive(Copy,Clone, Debug, PartialEq)]
pub struct OpenPositionV2Keys {
    pub payer: Pubkey,
    pub position_nft_owner: Pubkey,
    pub position_nft_mint: Pubkey,
    pub position_nft_account: Pubkey,
    pub metadata_account: Pubkey,
    pub pool_state: Pubkey,
    pub protocol_position: Pubkey,
    pub tick_array_lower: Pubkey,
    pub tick_array_upper: Pubkey,
    pub personal_position: Pubkey,
    pub token_account0: Pubkey,
    pub token_account1: Pubkey,
    pub token_vault0: Pubkey,
    pub token_vault1: Pubkey,
    pub rent: Pubkey,
    pub system_program: Pubkey,
    pub token_program: Pubkey,
    pub associated_token_program: Pubkey,
    pub metadata_program: Pubkey,
    pub token_program2022: Pubkey,
    pub vault0_mint: Pubkey,
    pub vault1_mint: Pubkey,
}

impl From<OpenPositionV2Accounts<'_, '_>> for OpenPositionV2Keys {
    fn from(accounts: OpenPositionV2Accounts) -> Self {
        Self {
            payer: *accounts.payer.key,
            position_nft_owner: *accounts.position_nft_owner.key,
            position_nft_mint: *accounts.position_nft_mint.key,
            position_nft_account: *accounts.position_nft_account.key,
            metadata_account: *accounts.metadata_account.key,
            pool_state: *accounts.pool_state.key,
            protocol_position: *accounts.protocol_position.key,
            tick_array_lower: *accounts.tick_array_lower.key,
            tick_array_upper: *accounts.tick_array_upper.key,
            personal_position: *accounts.personal_position.key,
            token_account0: *accounts.token_account0.key,
            token_account1: *accounts.token_account1.key,
            token_vault0: *accounts.token_vault0.key,
            token_vault1: *accounts.token_vault1.key,
            rent: *accounts.rent.key,
            system_program: *accounts.system_program.key,
            token_program: *accounts.token_program.key,
            associated_token_program: *accounts.associated_token_program.key,
            metadata_program: *accounts.metadata_program.key,
            token_program2022: *accounts.token_program2022.key,
            vault0_mint: *accounts.vault0_mint.key,
            vault1_mint: *accounts.vault1_mint.key,
        }
    }
}

impl From<OpenPositionV2Keys> for [AccountMeta; OPEN_POSITION_V2_IX_ACCOUNTS_LEN] {
    fn from(keys: OpenPositionV2Keys) -> Self {
        [
            AccountMeta {
                pubkey: keys.payer,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.position_nft_owner,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.position_nft_mint,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.position_nft_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.metadata_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.pool_state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.protocol_position,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.tick_array_lower,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.tick_array_upper,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.personal_position,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_account0,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_account1,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_vault0,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_vault1,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.rent,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.system_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.associated_token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.metadata_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_program2022,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.vault0_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.vault1_mint,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}

impl From<[Pubkey; OPEN_POSITION_V2_IX_ACCOUNTS_LEN]> for OpenPositionV2Keys {
    fn from(pubkeys: [Pubkey; OPEN_POSITION_V2_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            payer: pubkeys[0],
            position_nft_owner: pubkeys[1],
            position_nft_mint: pubkeys[2],
            position_nft_account: pubkeys[3],
            metadata_account: pubkeys[4],
            pool_state: pubkeys[5],
            protocol_position: pubkeys[6],
            tick_array_lower: pubkeys[7],
            tick_array_upper: pubkeys[8],
            personal_position: pubkeys[9],
            token_account0: pubkeys[10],
            token_account1: pubkeys[11],
            token_vault0: pubkeys[12],
            token_vault1: pubkeys[13],
            rent: pubkeys[14],
            system_program: pubkeys[15],
            token_program: pubkeys[16],
            associated_token_program: pubkeys[17],
            metadata_program: pubkeys[18],
            token_program2022: pubkeys[19],
            vault0_mint: pubkeys[20],
            vault1_mint: pubkeys[21],
        }
    }
}

impl<'info> From<OpenPositionV2Accounts<'_, 'info>> for [AccountInfo<'info>; OPEN_POSITION_V2_IX_ACCOUNTS_LEN] {
    fn from(accounts: OpenPositionV2Accounts<'_, 'info>) -> Self {
        [
            accounts.payer.clone(),
            accounts.position_nft_owner.clone(),
            accounts.position_nft_mint.clone(),
            accounts.position_nft_account.clone(),
            accounts.metadata_account.clone(),
            accounts.pool_state.clone(),
            accounts.protocol_position.clone(),
            accounts.tick_array_lower.clone(),
            accounts.tick_array_upper.clone(),
            accounts.personal_position.clone(),
            accounts.token_account0.clone(),
            accounts.token_account1.clone(),
            accounts.token_vault0.clone(),
            accounts.token_vault1.clone(),
            accounts.rent.clone(),
            accounts.system_program.clone(),
            accounts.token_program.clone(),
            accounts.associated_token_program.clone(),
            accounts.metadata_program.clone(),
            accounts.token_program2022.clone(),
            accounts.vault0_mint.clone(),
            accounts.vault1_mint.clone(),
        ]
    }
}

impl<'me, 'info> From<&'me [AccountInfo<'info>; OPEN_POSITION_V2_IX_ACCOUNTS_LEN]> for OpenPositionV2Accounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; OPEN_POSITION_V2_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            payer: &arr[0],
            position_nft_owner: &arr[1],
            position_nft_mint: &arr[2],
            position_nft_account: &arr[3],
            metadata_account: &arr[4],
            pool_state: &arr[5],
            protocol_position: &arr[6],
            tick_array_lower: &arr[7],
            tick_array_upper: &arr[8],
            personal_position: &arr[9],
            token_account0: &arr[10],
            token_account1: &arr[11],
            token_vault0: &arr[12],
            token_vault1: &arr[13],
            rent: &arr[14],
            system_program: &arr[15],
            token_program: &arr[16],
            associated_token_program: &arr[17],
            metadata_program: &arr[18],
            token_program2022: &arr[19],
            vault0_mint: &arr[20],
            vault1_mint: &arr[21],
        }
    }
}

pub const OPEN_POSITION_V2_IX_DISCM: u8 = 77u8;

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct OpenPositionV2IxArgs {
    pub tick_lower_index: i32,
    pub tick_upper_index: i32,
    pub tick_array_lower_start_index: i32,
    pub tick_array_upper_start_index: i32,
    pub liquidity: u128,
    pub amount0_max: u64,
    pub amount1_max: u64,
    pub with_metadata: bool,
    pub base_flag: Option<bool>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct OpenPositionV2IxData(pub OpenPositionV2IxArgs);

impl From<OpenPositionV2IxArgs> for OpenPositionV2IxData {
    fn from(args: OpenPositionV2IxArgs) -> Self {
        Self(args)
    }
}

impl OpenPositionV2IxData {
    pub fn deserialize(buf: &[u8]) ->std::io::Result<Self> {
          let mut reader = buf;
        let mut maybe_discm_buf = [0u8; 1];
        reader.read_exact(&mut maybe_discm_buf)?;
        let maybe_discm = maybe_discm_buf[0];
        if maybe_discm != OPEN_POSITION_V2_IX_DISCM {
            return Err(std::io::Error::new(
               std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    OPEN_POSITION_V2_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(OpenPositionV2IxArgs::deserialize(&mut reader)?))
    }

    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&[OPEN_POSITION_V2_IX_DISCM])?;
        self.0.serialize(&mut writer)
    }

    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>>  {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}

pub fn open_position_v2_ix_with_program_id(
    program_id: Pubkey,
    keys: OpenPositionV2Keys,
    args: OpenPositionV2IxArgs,
) ->  std::io::Result<Instruction> {
    let metas: [AccountMeta; OPEN_POSITION_V2_IX_ACCOUNTS_LEN] = keys.into();
    let data: OpenPositionV2IxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}

pub fn open_position_v2_ix(
    keys: OpenPositionV2Keys,
    args: OpenPositionV2IxArgs,
) ->  std::io::Result<Instruction> {
    open_position_v2_ix_with_program_id(crate::ID, keys, args)
}

pub fn open_position_v2_invoke_with_program_id(
    program_id: Pubkey,
    accounts: OpenPositionV2Accounts<'_, '_>,
    args: OpenPositionV2IxArgs,
) -> ProgramResult {
    let keys: OpenPositionV2Keys = accounts.into();
    let ix = open_position_v2_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}

pub fn open_position_v2_invoke(
    accounts: OpenPositionV2Accounts<'_, '_>,
    args: OpenPositionV2IxArgs,
) -> ProgramResult {
    open_position_v2_invoke_with_program_id(crate::ID, accounts, args)
}

pub fn open_position_v2_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: OpenPositionV2Accounts<'_, '_>,
    args: OpenPositionV2IxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: OpenPositionV2Keys = accounts.into();
    let ix = open_position_v2_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}

pub fn open_position_v2_invoke_signed(
    accounts: OpenPositionV2Accounts<'_, '_>,
    args: OpenPositionV2IxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    open_position_v2_invoke_signed_with_program_id(crate::ID, accounts, args, seeds)
}

pub fn open_position_v2_verify_account_keys(
    accounts: OpenPositionV2Accounts<'_, '_>,
    keys: OpenPositionV2Keys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.payer.key, keys.payer),
        (*accounts.position_nft_owner.key, keys.position_nft_owner),
        (*accounts.position_nft_mint.key, keys.position_nft_mint),
        (*accounts.position_nft_account.key, keys.position_nft_account),
        (*accounts.metadata_account.key, keys.metadata_account),
        (*accounts.pool_state.key, keys.pool_state),
        (*accounts.protocol_position.key, keys.protocol_position),
        (*accounts.tick_array_lower.key, keys.tick_array_lower),
        (*accounts.tick_array_upper.key, keys.tick_array_upper),
        (*accounts.personal_position.key, keys.personal_position),
        (*accounts.token_account0.key, keys.token_account0),
        (*accounts.token_account1.key, keys.token_account1),
        (*accounts.token_vault0.key, keys.token_vault0),
        (*accounts.token_vault1.key, keys.token_vault1),
        (*accounts.rent.key, keys.rent),
        (*accounts.system_program.key, keys.system_program),
        (*accounts.token_program.key, keys.token_program),
        (*accounts.associated_token_program.key, keys.associated_token_program),
        (*accounts.metadata_program.key, keys.metadata_program),
        (*accounts.token_program2022.key, keys.token_program2022),
        (*accounts.vault0_mint.key, keys.vault0_mint),
        (*accounts.vault1_mint.key, keys.vault1_mint),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}

pub fn open_position_v2_verify_writable_privileges<'me, 'info>(
    accounts: OpenPositionV2Accounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.payer,
        accounts.position_nft_mint,
        accounts.position_nft_account,
        accounts.metadata_account,
        accounts.pool_state,
        accounts.protocol_position,
        accounts.tick_array_lower,
        accounts.tick_array_upper,
        accounts.personal_position,
        accounts.token_account0,
        accounts.token_account1,
        accounts.token_vault0,
        accounts.token_vault1,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}

pub fn open_position_v2_verify_signer_privileges<'me, 'info>(
    accounts: OpenPositionV2Accounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.payer, accounts.position_nft_mint] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}

pub fn open_position_v2_verify_account_privileges<'me, 'info>(
    accounts: OpenPositionV2Accounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    open_position_v2_verify_writable_privileges(accounts)?;
    open_position_v2_verify_signer_privileges(accounts)?;
    Ok(())
}


pub const OPEN_POSITION_WITH_TOKEN22_NFT_IX_ACCOUNTS_LEN: usize = 20;

#[derive(Copy, Clone, Debug)]
pub struct OpenPositionWithToken22NftAccounts<'me, 'info> {
    pub payer: &'me AccountInfo<'info>,
    pub position_nft_owner: &'me AccountInfo<'info>,
    pub position_nft_mint: &'me AccountInfo<'info>,
    pub position_nft_account: &'me AccountInfo<'info>,
    pub pool_state: &'me AccountInfo<'info>,
    pub protocol_position: &'me AccountInfo<'info>,
    pub tick_array_lower: &'me AccountInfo<'info>,
    pub tick_array_upper: &'me AccountInfo<'info>,
    pub personal_position: &'me AccountInfo<'info>,
    pub token_account0: &'me AccountInfo<'info>,
    pub token_account1: &'me AccountInfo<'info>,
    pub token_vault0: &'me AccountInfo<'info>,
    pub token_vault1: &'me AccountInfo<'info>,
    pub rent: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub associated_token_program: &'me AccountInfo<'info>,
    pub token_program2022: &'me AccountInfo<'info>,
    pub vault0_mint: &'me AccountInfo<'info>,
    pub vault1_mint: &'me AccountInfo<'info>,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct OpenPositionWithToken22NftKeys {
    pub payer: Pubkey,
    pub position_nft_owner: Pubkey,
    pub position_nft_mint: Pubkey,
    pub position_nft_account: Pubkey,
    pub pool_state: Pubkey,
    pub protocol_position: Pubkey,
    pub tick_array_lower: Pubkey,
    pub tick_array_upper: Pubkey,
    pub personal_position: Pubkey,
    pub token_account0: Pubkey,
    pub token_account1: Pubkey,
    pub token_vault0: Pubkey,
    pub token_vault1: Pubkey,
    pub rent: Pubkey,
    pub system_program: Pubkey,
    pub token_program: Pubkey,
    pub associated_token_program: Pubkey,
    pub token_program2022: Pubkey,
    pub vault0_mint: Pubkey,
    pub vault1_mint: Pubkey,
}

impl From<OpenPositionWithToken22NftAccounts<'_, '_>> for OpenPositionWithToken22NftKeys {
    fn from(accounts: OpenPositionWithToken22NftAccounts) -> Self {
        Self {
            payer: *accounts.payer.key,
            position_nft_owner: *accounts.position_nft_owner.key,
            position_nft_mint: *accounts.position_nft_mint.key,
            position_nft_account: *accounts.position_nft_account.key,
            pool_state: *accounts.pool_state.key,
            protocol_position: *accounts.protocol_position.key,
            tick_array_lower: *accounts.tick_array_lower.key,
            tick_array_upper: *accounts.tick_array_upper.key,
            personal_position: *accounts.personal_position.key,
            token_account0: *accounts.token_account0.key,
            token_account1: *accounts.token_account1.key,
            token_vault0: *accounts.token_vault0.key,
            token_vault1: *accounts.token_vault1.key,
            rent: *accounts.rent.key,
            system_program: *accounts.system_program.key,
            token_program: *accounts.token_program.key,
            associated_token_program: *accounts.associated_token_program.key,
            token_program2022: *accounts.token_program2022.key,
            vault0_mint: *accounts.vault0_mint.key,
            vault1_mint: *accounts.vault1_mint.key,
        }
    }
}

impl From<OpenPositionWithToken22NftKeys> for [AccountMeta; OPEN_POSITION_WITH_TOKEN22_NFT_IX_ACCOUNTS_LEN] {
    fn from(keys: OpenPositionWithToken22NftKeys) -> Self {
        [
            AccountMeta { pubkey: keys.payer, is_signer: true, is_writable: true },
            AccountMeta { pubkey: keys.position_nft_owner, is_signer: false, is_writable: false },
            AccountMeta { pubkey: keys.position_nft_mint, is_signer: true, is_writable: true },
            AccountMeta { pubkey: keys.position_nft_account, is_signer: false, is_writable: true },
            AccountMeta { pubkey: keys.pool_state, is_signer: false, is_writable: true },
            AccountMeta { pubkey: keys.protocol_position, is_signer: false, is_writable: true },
            AccountMeta { pubkey: keys.tick_array_lower, is_signer: false, is_writable: true },
            AccountMeta { pubkey: keys.tick_array_upper, is_signer: false, is_writable: true },
            AccountMeta { pubkey: keys.personal_position, is_signer: false, is_writable: true },
            AccountMeta { pubkey: keys.token_account0, is_signer: false, is_writable: true },
            AccountMeta { pubkey: keys.token_account1, is_signer: false, is_writable: true },
            AccountMeta { pubkey: keys.token_vault0, is_signer: false, is_writable: true },
            AccountMeta { pubkey: keys.token_vault1, is_signer: false, is_writable: true },
            AccountMeta { pubkey: keys.rent, is_signer: false, is_writable: false },
            AccountMeta { pubkey: keys.system_program, is_signer: false, is_writable: false },
            AccountMeta { pubkey: keys.token_program, is_signer: false, is_writable: false },
            AccountMeta { pubkey: keys.associated_token_program, is_signer: false, is_writable: false },
            AccountMeta { pubkey: keys.token_program2022, is_signer: false, is_writable: false },
            AccountMeta { pubkey: keys.vault0_mint, is_signer: false, is_writable: false },
            AccountMeta { pubkey: keys.vault1_mint, is_signer: false, is_writable: false },
        ]
    }
}

impl From<[Pubkey; OPEN_POSITION_WITH_TOKEN22_NFT_IX_ACCOUNTS_LEN]> for OpenPositionWithToken22NftKeys {
    fn from(pubkeys: [Pubkey; OPEN_POSITION_WITH_TOKEN22_NFT_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            payer: pubkeys[0],
            position_nft_owner: pubkeys[1],
            position_nft_mint: pubkeys[2],
            position_nft_account: pubkeys[3],
            pool_state: pubkeys[4],
            protocol_position: pubkeys[5],
            tick_array_lower: pubkeys[6],
            tick_array_upper: pubkeys[7],
            personal_position: pubkeys[8],
            token_account0: pubkeys[9],
            token_account1: pubkeys[10],
            token_vault0: pubkeys[11],
            token_vault1: pubkeys[12],
            rent: pubkeys[13],
            system_program: pubkeys[14],
            token_program: pubkeys[15],
            associated_token_program: pubkeys[16],
            token_program2022: pubkeys[17],
            vault0_mint: pubkeys[18],
            vault1_mint: pubkeys[19],
        }
    }
}

impl<'info> From<OpenPositionWithToken22NftAccounts<'_, 'info>> for [AccountInfo<'info>; OPEN_POSITION_WITH_TOKEN22_NFT_IX_ACCOUNTS_LEN] {
    fn from(accounts: OpenPositionWithToken22NftAccounts<'_, 'info>) -> Self {
        [
            accounts.payer.clone(),
            accounts.position_nft_owner.clone(),
            accounts.position_nft_mint.clone(),
            accounts.position_nft_account.clone(),
            accounts.pool_state.clone(),
            accounts.protocol_position.clone(),
            accounts.tick_array_lower.clone(),
            accounts.tick_array_upper.clone(),
            accounts.personal_position.clone(),
            accounts.token_account0.clone(),
            accounts.token_account1.clone(),
            accounts.token_vault0.clone(),
            accounts.token_vault1.clone(),
            accounts.rent.clone(),
            accounts.system_program.clone(),
            accounts.token_program.clone(),
            accounts.associated_token_program.clone(),
            accounts.token_program2022.clone(),
            accounts.vault0_mint.clone(),
            accounts.vault1_mint.clone(),
        ]
    }
}

impl<'me, 'info> From<&'me [AccountInfo<'info>; OPEN_POSITION_WITH_TOKEN22_NFT_IX_ACCOUNTS_LEN]> for OpenPositionWithToken22NftAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; OPEN_POSITION_WITH_TOKEN22_NFT_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            payer: &arr[0],
            position_nft_owner: &arr[1],
            position_nft_mint: &arr[2],
            position_nft_account: &arr[3],
            pool_state: &arr[4],
            protocol_position: &arr[5],
            tick_array_lower: &arr[6],
            tick_array_upper: &arr[7],
            personal_position: &arr[8],
            token_account0: &arr[9],
            token_account1: &arr[10],
            token_vault0: &arr[11],
            token_vault1: &arr[12],
            rent: &arr[13],
            system_program: &arr[14],
            token_program: &arr[15],
            associated_token_program: &arr[16],
            token_program2022: &arr[17],
            vault0_mint: &arr[18],
            vault1_mint: &arr[19],
        }
    }
}

// Placeholder discriminator (replace with actual discriminator for the instruction)
pub const OPEN_POSITION_WITH_TOKEN22_NFT_IX_DISCM: u8 = 16u8;

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct OpenPositionWithToken22NftIxArgs {
    pub tick_lower_index: i32,
    pub tick_upper_index: i32,
    pub tick_array_lower_start_index: i32,
    pub tick_array_upper_start_index: i32,
    pub liquidity: u128,
    pub amount0_max: u64,
    pub amount1_max: u64,
    pub with_metadata: bool,
    pub base_flag: Option<bool>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct OpenPositionWithToken22NftIxData(pub OpenPositionWithToken22NftIxArgs);

impl From<OpenPositionWithToken22NftIxArgs> for OpenPositionWithToken22NftIxData {
    fn from(args: OpenPositionWithToken22NftIxArgs) -> Self {
        Self(args)
    }
}

impl OpenPositionWithToken22NftIxData {
    pub fn deserialize(buf: &[u8]) ->std::io::Result<Self> {
          let mut reader = buf;
        let mut maybe_discm_buf = [0u8; 1];
        reader.read_exact(&mut maybe_discm_buf)?;
        let maybe_discm = maybe_discm_buf[0];
        if maybe_discm != OPEN_POSITION_WITH_TOKEN22_NFT_IX_DISCM {
            return Err(std::io::Error::new(
               std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    OPEN_POSITION_WITH_TOKEN22_NFT_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(OpenPositionWithToken22NftIxArgs::deserialize(&mut reader)?))
    }

    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&[OPEN_POSITION_WITH_TOKEN22_NFT_IX_DISCM])?;
        self.0.serialize(&mut writer)
    }

    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>>  {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}

pub fn open_position_with_token22_nft_ix_with_program_id(
    program_id: Pubkey,
    keys: OpenPositionWithToken22NftKeys,
    args: OpenPositionWithToken22NftIxArgs,
) ->  std::io::Result<Instruction> {
    let metas: [AccountMeta; OPEN_POSITION_WITH_TOKEN22_NFT_IX_ACCOUNTS_LEN] = keys.into();
    let data: OpenPositionWithToken22NftIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}

pub fn open_position_with_token22_nft_ix(
    keys: OpenPositionWithToken22NftKeys,
    args: OpenPositionWithToken22NftIxArgs,
) ->  std::io::Result<Instruction> {
    open_position_with_token22_nft_ix_with_program_id(crate::ID, keys, args)
}

pub fn open_position_with_token22_nft_invoke_with_program_id(
    program_id: Pubkey,
    accounts: OpenPositionWithToken22NftAccounts<'_, '_>,
    args: OpenPositionWithToken22NftIxArgs,
) -> ProgramResult {
    let keys: OpenPositionWithToken22NftKeys = accounts.into();
    let ix = open_position_with_token22_nft_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}

pub fn open_position_with_token22_nft_invoke(
    accounts: OpenPositionWithToken22NftAccounts<'_, '_>,
    args: OpenPositionWithToken22NftIxArgs,
) -> ProgramResult {
    open_position_with_token22_nft_invoke_with_program_id(crate::ID, accounts, args)
}

pub fn open_position_with_token22_nft_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: OpenPositionWithToken22NftAccounts<'_, '_>,
    args: OpenPositionWithToken22NftIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: OpenPositionWithToken22NftKeys = accounts.into();
    let ix = open_position_with_token22_nft_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}

pub fn open_position_with_token22_nft_invoke_signed(
    accounts: OpenPositionWithToken22NftAccounts<'_, '_>,
    args: OpenPositionWithToken22NftIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    open_position_with_token22_nft_invoke_signed_with_program_id(crate::ID, accounts, args, seeds)
}

pub fn open_position_with_token22_nft_verify_account_keys(
    accounts: OpenPositionWithToken22NftAccounts<'_, '_>,
    keys: OpenPositionWithToken22NftKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.payer.key, keys.payer),
        (*accounts.position_nft_owner.key, keys.position_nft_owner),
        (*accounts.position_nft_mint.key, keys.position_nft_mint),
        (*accounts.position_nft_account.key, keys.position_nft_account),
        (*accounts.pool_state.key, keys.pool_state),
        (*accounts.protocol_position.key, keys.protocol_position),
        (*accounts.tick_array_lower.key, keys.tick_array_lower),
        (*accounts.tick_array_upper.key, keys.tick_array_upper),
        (*accounts.personal_position.key, keys.personal_position),
        (*accounts.token_account0.key, keys.token_account0),
        (*accounts.token_account1.key, keys.token_account1),
        (*accounts.token_vault0.key, keys.token_vault0),
        (*accounts.token_vault1.key, keys.token_vault1),
        (*accounts.rent.key, keys.rent),
        (*accounts.system_program.key, keys.system_program),
        (*accounts.token_program.key, keys.token_program),
        (*accounts.associated_token_program.key, keys.associated_token_program),
        (*accounts.token_program2022.key, keys.token_program2022),
        (*accounts.vault0_mint.key, keys.vault0_mint),
        (*accounts.vault1_mint.key, keys.vault1_mint),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}

pub fn open_position_with_token22_nft_verify_writable_privileges<'me, 'info>(
    accounts: OpenPositionWithToken22NftAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.payer,
        accounts.position_nft_mint,
        accounts.position_nft_account,
        accounts.pool_state,
        accounts.protocol_position,
        accounts.tick_array_lower,
        accounts.tick_array_upper,
        accounts.personal_position,
        accounts.token_account0,
        accounts.token_account1,
        accounts.token_vault0,
        accounts.token_vault1,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}

pub fn open_position_with_token22_nft_verify_signer_privileges<'me, 'info>(
    accounts: OpenPositionWithToken22NftAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.payer, accounts.position_nft_mint] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}

pub fn open_position_with_token22_nft_verify_account_privileges<'me, 'info>(
    accounts: OpenPositionWithToken22NftAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    open_position_with_token22_nft_verify_writable_privileges(accounts)?;
    open_position_with_token22_nft_verify_signer_privileges(accounts)?;
    Ok(())
}


pub const CLOSE_POSITION_IX_ACCOUNTS_LEN: usize = 6;

#[derive(Copy, Clone, Debug)]
pub struct ClosePositionAccounts<'me, 'info> {
    pub nft_owner: &'me AccountInfo<'info>,
    pub position_nft_mint: &'me AccountInfo<'info>,
    pub position_nft_account: &'me AccountInfo<'info>,
    pub personal_position: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct ClosePositionKeys {
    pub nft_owner: Pubkey,
    pub position_nft_mint: Pubkey,
    pub position_nft_account: Pubkey,
    pub personal_position: Pubkey,
    pub system_program: Pubkey,
    pub token_program: Pubkey,
}

impl From<ClosePositionAccounts<'_, '_>> for ClosePositionKeys {
    fn from(accounts: ClosePositionAccounts) -> Self {
        Self {
            nft_owner: *accounts.nft_owner.key,
            position_nft_mint: *accounts.position_nft_mint.key,
            position_nft_account: *accounts.position_nft_account.key,
            personal_position: *accounts.personal_position.key,
            system_program: *accounts.system_program.key,
            token_program: *accounts.token_program.key,
        }
    }
}

impl From<ClosePositionKeys> for [AccountMeta; CLOSE_POSITION_IX_ACCOUNTS_LEN] {
    fn from(keys: ClosePositionKeys) -> Self {
        [
            AccountMeta { pubkey: keys.nft_owner, is_signer: true, is_writable: true },
            AccountMeta { pubkey: keys.position_nft_mint, is_signer: false, is_writable: true },
            AccountMeta { pubkey: keys.position_nft_account, is_signer: false, is_writable: true },
            AccountMeta { pubkey: keys.personal_position, is_signer: false, is_writable: true },
            AccountMeta { pubkey: keys.system_program, is_signer: false, is_writable: false },
            AccountMeta { pubkey: keys.token_program, is_signer: false, is_writable: false },
        ]
    }
}

impl From<[Pubkey; CLOSE_POSITION_IX_ACCOUNTS_LEN]> for ClosePositionKeys {
    fn from(pubkeys: [Pubkey; CLOSE_POSITION_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            nft_owner: pubkeys[0],
            position_nft_mint: pubkeys[1],
            position_nft_account: pubkeys[2],
            personal_position: pubkeys[3],
            system_program: pubkeys[4],
            token_program: pubkeys[5],
        }
    }
}

impl<'info> From<ClosePositionAccounts<'_, 'info>> for [AccountInfo<'info>; CLOSE_POSITION_IX_ACCOUNTS_LEN] {
    fn from(accounts: ClosePositionAccounts<'_, 'info>) -> Self {
        [
            accounts.nft_owner.clone(),
            accounts.position_nft_mint.clone(),
            accounts.position_nft_account.clone(),
            accounts.personal_position.clone(),
            accounts.system_program.clone(),
            accounts.token_program.clone(),
        ]
    }
}

impl<'me, 'info> From<&'me [AccountInfo<'info>; CLOSE_POSITION_IX_ACCOUNTS_LEN]> for ClosePositionAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; CLOSE_POSITION_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            nft_owner: &arr[0],
            position_nft_mint: &arr[1],
            position_nft_account: &arr[2],
            personal_position: &arr[3],
            system_program: &arr[4],
            token_program: &arr[5],
        }
    }
}

// Placeholder discriminator (replace with actual discriminator for the instruction)
pub const CLOSE_POSITION_IX_DISCM: u8 = 123u8;

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ClosePositionIxArgs {}

#[derive(Clone, Debug, PartialEq)]
pub struct ClosePositionIxData(pub ClosePositionIxArgs);

impl From<ClosePositionIxArgs> for ClosePositionIxData {
    fn from(args: ClosePositionIxArgs) -> Self {
        Self(args)
    }
}

impl ClosePositionIxData {
    pub fn deserialize(buf: &[u8]) ->std::io::Result<Self> {
          let mut reader = buf;
        let mut maybe_discm_buf = [0u8; 1];
        reader.read_exact(&mut maybe_discm_buf)?;
        let maybe_discm = maybe_discm_buf[0];
        if maybe_discm != CLOSE_POSITION_IX_DISCM {
            return Err(std::io::Error::new(
               std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    CLOSE_POSITION_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(ClosePositionIxArgs::deserialize(&mut reader)?))
    }

    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&[CLOSE_POSITION_IX_DISCM])?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>>  {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}

pub fn close_position_ix_with_program_id(
    program_id: Pubkey,
    keys: ClosePositionKeys,
    args: ClosePositionIxArgs,
) ->  std::io::Result<Instruction> {
    let metas: [AccountMeta; CLOSE_POSITION_IX_ACCOUNTS_LEN] = keys.into();
    let data: ClosePositionIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}

pub fn close_position_ix(
    keys: ClosePositionKeys,
    args: ClosePositionIxArgs,
) ->  std::io::Result<Instruction> {
    close_position_ix_with_program_id(crate::ID, keys, args)
}

pub fn close_position_invoke_with_program_id(
    program_id: Pubkey,
    accounts: ClosePositionAccounts<'_, '_>,
    args: ClosePositionIxArgs,
) -> ProgramResult {
    let keys: ClosePositionKeys = accounts.into();
    let ix = close_position_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}

pub fn close_position_invoke(
    accounts: ClosePositionAccounts<'_, '_>,
    args: ClosePositionIxArgs,
) -> ProgramResult {
    close_position_invoke_with_program_id(crate::ID, accounts, args)
}

pub fn close_position_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: ClosePositionAccounts<'_, '_>,
    args: ClosePositionIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: ClosePositionKeys = accounts.into();
    let ix = close_position_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}

pub fn close_position_invoke_signed(
    accounts: ClosePositionAccounts<'_, '_>,
    args: ClosePositionIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    close_position_invoke_signed_with_program_id(crate::ID, accounts, args, seeds)
}

pub fn close_position_verify_account_keys(
    accounts: ClosePositionAccounts<'_, '_>,
    keys: ClosePositionKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.nft_owner.key, keys.nft_owner),
        (*accounts.position_nft_mint.key, keys.position_nft_mint),
        (*accounts.position_nft_account.key, keys.position_nft_account),
        (*accounts.personal_position.key, keys.personal_position),
        (*accounts.system_program.key, keys.system_program),
        (*accounts.token_program.key, keys.token_program),
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
        accounts.nft_owner,
        accounts.position_nft_mint,
        accounts.position_nft_account,
        accounts.personal_position,
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
    for should_be_signer in [accounts.nft_owner] {
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

pub const INCREASE_LIQUIDITY_IX_ACCOUNTS_LEN: usize = 12;

#[derive(Copy, Clone, Debug)]
pub struct IncreaseLiquidityAccounts<'me, 'info> {
    pub nft_owner: &'me AccountInfo<'info>,
    pub nft_account: &'me AccountInfo<'info>,
    pub pool_state: &'me AccountInfo<'info>,
    pub protocol_position: &'me AccountInfo<'info>,
    pub personal_position: &'me AccountInfo<'info>,
    pub tick_array_lower: &'me AccountInfo<'info>,
    pub tick_array_upper: &'me AccountInfo<'info>,
    pub token_account_0: &'me AccountInfo<'info>,
    pub token_account_1: &'me AccountInfo<'info>,
    pub token_vault_0: &'me AccountInfo<'info>,
    pub token_vault_1: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct IncreaseLiquidityKeys {
    pub nft_owner: Pubkey,
    pub nft_account: Pubkey,
    pub pool_state: Pubkey,
    pub protocol_position: Pubkey,
    pub personal_position: Pubkey,
    pub tick_array_lower: Pubkey,
    pub tick_array_upper: Pubkey,
    pub token_account_0: Pubkey,
    pub token_account_1: Pubkey,
    pub token_vault_0: Pubkey,
    pub token_vault_1: Pubkey,
    pub token_program: Pubkey,
}

impl From<IncreaseLiquidityAccounts<'_, '_>> for IncreaseLiquidityKeys {
    fn from(accounts: IncreaseLiquidityAccounts) -> Self {
        Self {
            nft_owner: *accounts.nft_owner.key,
            nft_account: *accounts.nft_account.key,
            pool_state: *accounts.pool_state.key,
            protocol_position: *accounts.protocol_position.key,
            personal_position: *accounts.personal_position.key,
            tick_array_lower: *accounts.tick_array_lower.key,
            tick_array_upper: *accounts.tick_array_upper.key,
            token_account_0: *accounts.token_account_0.key,
            token_account_1: *accounts.token_account_1.key,
            token_vault_0: *accounts.token_vault_0.key,
            token_vault_1: *accounts.token_vault_1.key,
            token_program: *accounts.token_program.key,
        }
    }
}

impl From<IncreaseLiquidityKeys> for [AccountMeta; INCREASE_LIQUIDITY_IX_ACCOUNTS_LEN] {
    fn from(keys: IncreaseLiquidityKeys) -> Self {
        [
            AccountMeta { pubkey: keys.nft_owner, is_signer: true, is_writable: false },
            AccountMeta { pubkey: keys.nft_account, is_signer: false, is_writable: false },
            AccountMeta { pubkey: keys.pool_state, is_signer: false, is_writable: true },
            AccountMeta { pubkey: keys.protocol_position, is_signer: false, is_writable: true },
            AccountMeta { pubkey: keys.personal_position, is_signer: false, is_writable: true },
            AccountMeta { pubkey: keys.tick_array_lower, is_signer: false, is_writable: true },
            AccountMeta { pubkey: keys.tick_array_upper, is_signer: false, is_writable: true },
            AccountMeta { pubkey: keys.token_account_0, is_signer: false, is_writable: true },
            AccountMeta { pubkey: keys.token_account_1, is_signer: false, is_writable: true },
            AccountMeta { pubkey: keys.token_vault_0, is_signer: false, is_writable: true },
            AccountMeta { pubkey: keys.token_vault_1, is_signer: false, is_writable: true },
            AccountMeta { pubkey: keys.token_program, is_signer: false, is_writable: false },
        ]
    }
}

impl From<[Pubkey; INCREASE_LIQUIDITY_IX_ACCOUNTS_LEN]> for IncreaseLiquidityKeys {
    fn from(pubkeys: [Pubkey; INCREASE_LIQUIDITY_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            nft_owner: pubkeys[0],
            nft_account: pubkeys[1],
            pool_state: pubkeys[2],
            protocol_position: pubkeys[3],
            personal_position: pubkeys[4],
            tick_array_lower: pubkeys[5],
            tick_array_upper: pubkeys[6],
            token_account_0: pubkeys[7],
            token_account_1: pubkeys[8],
            token_vault_0: pubkeys[9],
            token_vault_1: pubkeys[10],
            token_program: pubkeys[11],
        }
    }
}

impl<'info> From<IncreaseLiquidityAccounts<'_, 'info>> for [AccountInfo<'info>; INCREASE_LIQUIDITY_IX_ACCOUNTS_LEN] {
    fn from(accounts: IncreaseLiquidityAccounts<'_, 'info>) -> Self {
        [
            accounts.nft_owner.clone(),
            accounts.nft_account.clone(),
            accounts.pool_state.clone(),
            accounts.protocol_position.clone(),
            accounts.personal_position.clone(),
            accounts.tick_array_lower.clone(),
            accounts.tick_array_upper.clone(),
            accounts.token_account_0.clone(),
            accounts.token_account_1.clone(),
            accounts.token_vault_0.clone(),
            accounts.token_vault_1.clone(),
            accounts.token_program.clone(),
        ]
    }
}

impl<'me, 'info> From<&'me [AccountInfo<'info>; INCREASE_LIQUIDITY_IX_ACCOUNTS_LEN]> for IncreaseLiquidityAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; INCREASE_LIQUIDITY_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            nft_owner: &arr[0],
            nft_account: &arr[1],
            pool_state: &arr[2],
            protocol_position: &arr[3],
            personal_position: &arr[4],
            tick_array_lower: &arr[5],
            tick_array_upper: &arr[6],
            token_account_0: &arr[7],
            token_account_1: &arr[8],
            token_vault_0: &arr[9],
            token_vault_1: &arr[10],
            token_program: &arr[11],
        }
    }
}

// Placeholder discriminator (replace with actual discriminator for the instruction)
pub const INCREASE_LIQUIDITY_IX_DISCM:  u8 = 133u8;

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct IncreaseLiquidityIxArgs {
    pub liquidity: u128,
    pub amount_0_max: u64,
    pub amount_1_max: u64,
}

#[derive(Clone, Debug, PartialEq)]
pub struct IncreaseLiquidityIxData(pub IncreaseLiquidityIxArgs);

impl From<IncreaseLiquidityIxArgs> for IncreaseLiquidityIxData {
    fn from(args: IncreaseLiquidityIxArgs) -> Self {
        Self(args)
    }
}

impl IncreaseLiquidityIxData {
    pub fn deserialize(buf: &[u8]) ->std::io::Result<Self> {
          let mut reader = buf;
        let mut maybe_discm_buf = [0u8; 1];
        reader.read_exact(&mut maybe_discm_buf)?;
        let maybe_discm = maybe_discm_buf[0];
        if maybe_discm != INCREASE_LIQUIDITY_IX_DISCM {
            return Err(std::io::Error::new(
               std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    INCREASE_LIQUIDITY_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(IncreaseLiquidityIxArgs::deserialize(&mut reader)?))
    }

    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&[INCREASE_LIQUIDITY_IX_DISCM])?;
        self.0.serialize(&mut writer)
    }

    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>>  {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}

pub fn increase_liquidity_ix_with_program_id(
    program_id: Pubkey,
    keys: IncreaseLiquidityKeys,
    args: IncreaseLiquidityIxArgs,
) ->  std::io::Result<Instruction> {
    let metas: [AccountMeta; INCREASE_LIQUIDITY_IX_ACCOUNTS_LEN] = keys.into();
    let data: IncreaseLiquidityIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}

pub fn increase_liquidity_ix(
    keys: IncreaseLiquidityKeys,
    args: IncreaseLiquidityIxArgs,
) ->  std::io::Result<Instruction> {
    increase_liquidity_ix_with_program_id(crate::ID, keys, args)
}

pub fn increase_liquidity_invoke_with_program_id(
    program_id: Pubkey,
    accounts: IncreaseLiquidityAccounts<'_, '_>,
    args: IncreaseLiquidityIxArgs,
) -> ProgramResult {
    let keys: IncreaseLiquidityKeys = accounts.into();
    let ix = increase_liquidity_ix_with_program_id(program_id, keys, args)?;
       invoke_instruction(&ix, accounts)
}

pub fn increase_liquidity_invoke(
    accounts: IncreaseLiquidityAccounts<'_, '_>,
    args: IncreaseLiquidityIxArgs,
) -> ProgramResult {
    increase_liquidity_invoke_with_program_id(crate::ID, accounts, args)
}

pub fn increase_liquidity_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: IncreaseLiquidityAccounts<'_, '_>,
    args: IncreaseLiquidityIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: IncreaseLiquidityKeys = accounts.into();
    let ix = increase_liquidity_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}

pub fn increase_liquidity_invoke_signed(
    accounts: IncreaseLiquidityAccounts<'_, '_>,
    args: IncreaseLiquidityIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    increase_liquidity_invoke_signed_with_program_id(crate::ID, accounts, args, seeds)
}

pub fn increase_liquidity_verify_account_keys(
    accounts: IncreaseLiquidityAccounts<'_, '_>,
    keys: IncreaseLiquidityKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.nft_owner.key, keys.nft_owner),
        (*accounts.nft_account.key, keys.nft_account),
        (*accounts.pool_state.key, keys.pool_state),
        (*accounts.protocol_position.key, keys.protocol_position),
        (*accounts.personal_position.key, keys.personal_position),
        (*accounts.tick_array_lower.key, keys.tick_array_lower),
        (*accounts.tick_array_upper.key, keys.tick_array_upper),
        (*accounts.token_account_0.key, keys.token_account_0),
        (*accounts.token_account_1.key, keys.token_account_1),
        (*accounts.token_vault_0.key, keys.token_vault_0),
        (*accounts.token_vault_1.key, keys.token_vault_1),
        (*accounts.token_program.key, keys.token_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}

pub fn increase_liquidity_verify_writable_privileges<'me, 'info>(
    accounts: IncreaseLiquidityAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.pool_state,
        accounts.protocol_position,
        accounts.personal_position,
        accounts.tick_array_lower,
        accounts.tick_array_upper,
        accounts.token_account_0,
        accounts.token_account_1,
        accounts.token_vault_0,
        accounts.token_vault_1,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}

pub fn increase_liquidity_verify_signer_privileges<'me, 'info>(
    accounts: IncreaseLiquidityAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.nft_owner] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}

pub fn increase_liquidity_verify_account_privileges<'me, 'info>(
    accounts: IncreaseLiquidityAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    increase_liquidity_verify_writable_privileges(accounts)?;
    increase_liquidity_verify_signer_privileges(accounts)?;
    Ok(())
}

pub const INCREASE_LIQUIDITY_V2_IX_ACCOUNTS_LEN: usize = 15;

#[derive(Copy, Clone, Debug)]
pub struct IncreaseLiquidityV2Accounts<'me, 'info> {
    pub nft_owner: &'me AccountInfo<'info>,
    pub nft_account: &'me AccountInfo<'info>,
    pub pool_state: &'me AccountInfo<'info>,
    pub protocol_position: &'me AccountInfo<'info>,
    pub personal_position: &'me AccountInfo<'info>,
    pub tick_array_lower: &'me AccountInfo<'info>,
    pub tick_array_upper: &'me AccountInfo<'info>,
    pub token_account0: &'me AccountInfo<'info>,
    pub token_account1: &'me AccountInfo<'info>,
    pub token_vault0: &'me AccountInfo<'info>,
    pub token_vault1: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub token_program2022: &'me AccountInfo<'info>,
    pub vault0_mint: &'me AccountInfo<'info>,
    pub vault1_mint: &'me AccountInfo<'info>,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct IncreaseLiquidityV2Keys {
    pub nft_owner: Pubkey,
    pub nft_account: Pubkey,
    pub pool_state: Pubkey,
    pub protocol_position: Pubkey,
    pub personal_position: Pubkey,
    pub tick_array_lower: Pubkey,
    pub tick_array_upper: Pubkey,
    pub token_account0: Pubkey,
    pub token_account1: Pubkey,
    pub token_vault0: Pubkey,
    pub token_vault1: Pubkey,
    pub token_program: Pubkey,
    pub token_program2022: Pubkey,
    pub vault0_mint: Pubkey,
    pub vault1_mint: Pubkey,
}

impl From<IncreaseLiquidityV2Accounts<'_, '_>> for IncreaseLiquidityV2Keys {
    fn from(accounts: IncreaseLiquidityV2Accounts) -> Self {
        Self {
            nft_owner: *accounts.nft_owner.key,
            nft_account: *accounts.nft_account.key,
            pool_state: *accounts.pool_state.key,
            protocol_position: *accounts.protocol_position.key,
            personal_position: *accounts.personal_position.key,
            tick_array_lower: *accounts.tick_array_lower.key,
            tick_array_upper: *accounts.tick_array_upper.key,
            token_account0: *accounts.token_account0.key,
            token_account1: *accounts.token_account1.key,
            token_vault0: *accounts.token_vault0.key,
            token_vault1: *accounts.token_vault1.key,
            token_program: *accounts.token_program.key,
            token_program2022: *accounts.token_program2022.key,
            vault0_mint: *accounts.vault0_mint.key,
            vault1_mint: *accounts.vault1_mint.key,
        }
    }
}

impl From<IncreaseLiquidityV2Keys> for [AccountMeta; INCREASE_LIQUIDITY_V2_IX_ACCOUNTS_LEN] {
    fn from(keys: IncreaseLiquidityV2Keys) -> Self {
        [
            AccountMeta { pubkey: keys.nft_owner, is_signer: true, is_writable: false },
            AccountMeta { pubkey: keys.nft_account, is_signer: false, is_writable: false },
            AccountMeta { pubkey: keys.pool_state, is_signer: false, is_writable: true },
            AccountMeta { pubkey: keys.protocol_position, is_signer: false, is_writable: true },
            AccountMeta { pubkey: keys.personal_position, is_signer: false, is_writable: true },
            AccountMeta { pubkey: keys.tick_array_lower, is_signer: false, is_writable: true },
            AccountMeta { pubkey: keys.tick_array_upper, is_signer: false, is_writable: true },
            AccountMeta { pubkey: keys.token_account0, is_signer: false, is_writable: true },
            AccountMeta { pubkey: keys.token_account1, is_signer: false, is_writable: true },
            AccountMeta { pubkey: keys.token_vault0, is_signer: false, is_writable: true },
            AccountMeta { pubkey: keys.token_vault1, is_signer: false, is_writable: true },
            AccountMeta { pubkey: keys.token_program, is_signer: false, is_writable: false },
            AccountMeta { pubkey: keys.token_program2022, is_signer: false, is_writable: false },
            AccountMeta { pubkey: keys.vault0_mint, is_signer: false, is_writable: false },
            AccountMeta { pubkey: keys.vault1_mint, is_signer: false, is_writable: false },
        ]
    }
}

impl From<[Pubkey; INCREASE_LIQUIDITY_V2_IX_ACCOUNTS_LEN]> for IncreaseLiquidityV2Keys {
    fn from(pubkeys: [Pubkey; INCREASE_LIQUIDITY_V2_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            nft_owner: pubkeys[0],
            nft_account: pubkeys[1],
            pool_state: pubkeys[2],
            protocol_position: pubkeys[3],
            personal_position: pubkeys[4],
            tick_array_lower: pubkeys[5],
            tick_array_upper: pubkeys[6],
            token_account0: pubkeys[7],
            token_account1: pubkeys[8],
            token_vault0: pubkeys[9],
            token_vault1: pubkeys[10],
            token_program: pubkeys[11],
            token_program2022: pubkeys[12],
            vault0_mint: pubkeys[13],
            vault1_mint: pubkeys[14],
        }
    }
}

impl<'info> From<IncreaseLiquidityV2Accounts<'_, 'info>> for [AccountInfo<'info>; INCREASE_LIQUIDITY_V2_IX_ACCOUNTS_LEN] {
    fn from(accounts: IncreaseLiquidityV2Accounts<'_, 'info>) -> Self {
        [
            accounts.nft_owner.clone(),
            accounts.nft_account.clone(),
            accounts.pool_state.clone(),
            accounts.protocol_position.clone(),
            accounts.personal_position.clone(),
            accounts.tick_array_lower.clone(),
            accounts.tick_array_upper.clone(),
            accounts.token_account0.clone(),
            accounts.token_account1.clone(),
            accounts.token_vault0.clone(),
            accounts.token_vault1.clone(),
            accounts.token_program.clone(),
            accounts.token_program2022.clone(),
            accounts.vault0_mint.clone(),
            accounts.vault1_mint.clone(),
        ]
    }
}

impl<'me, 'info> From<&'me [AccountInfo<'info>; INCREASE_LIQUIDITY_V2_IX_ACCOUNTS_LEN]> for IncreaseLiquidityV2Accounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; INCREASE_LIQUIDITY_V2_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            nft_owner: &arr[0],
            nft_account: &arr[1],
            pool_state: &arr[2],
            protocol_position: &arr[3],
            personal_position: &arr[4],
            tick_array_lower: &arr[5],
            tick_array_upper: &arr[6],
            token_account0: &arr[7],
            token_account1: &arr[8],
            token_vault0: &arr[9],
            token_vault1: &arr[10],
            token_program: &arr[11],
            token_program2022: &arr[12],
            vault0_mint: &arr[13],
            vault1_mint: &arr[14],
        }
    }
}

// Placeholder discriminator (replace with actual discriminator for the instruction)
pub const INCREASE_LIQUIDITY_V2_IX_DISCM:  u8 = 19u8;

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct IncreaseLiquidityV2IxArgs {
    pub liquidity: u128,
    pub amount0_max: u64,
    pub amount1_max: u64,
    pub base_flag: Option<bool>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct IncreaseLiquidityV2IxData(pub IncreaseLiquidityV2IxArgs);

impl From<IncreaseLiquidityV2IxArgs> for IncreaseLiquidityV2IxData {
    fn from(args: IncreaseLiquidityV2IxArgs) -> Self {
        Self(args)
    }
}

impl IncreaseLiquidityV2IxData {
    pub fn deserialize(buf: &[u8]) ->std::io::Result<Self> {
          let mut reader = buf;
        let mut maybe_discm_buf = [0u8; 1];
        reader.read_exact(&mut maybe_discm_buf)?;
        let maybe_discm = maybe_discm_buf[0];
        if maybe_discm != INCREASE_LIQUIDITY_V2_IX_DISCM {
            return Err(std::io::Error::new(
               std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    INCREASE_LIQUIDITY_V2_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(IncreaseLiquidityV2IxArgs::deserialize(&mut reader)?))
    }

    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&[INCREASE_LIQUIDITY_V2_IX_DISCM])?;
        self.0.serialize(&mut writer)
    }

    pub fn try_to_vec(&self) ->  std::io::Result<Vec<u8>>  {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}

pub fn increase_liquidity_v2_ix_with_program_id(
    program_id: Pubkey,
    keys: IncreaseLiquidityV2Keys,
    args: IncreaseLiquidityV2IxArgs,
) ->  std::io::Result<Instruction> {
    let metas: [AccountMeta; INCREASE_LIQUIDITY_V2_IX_ACCOUNTS_LEN] = keys.into();
    let data: IncreaseLiquidityV2IxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}

pub fn increase_liquidity_v2_ix(
    keys: IncreaseLiquidityV2Keys,
    args: IncreaseLiquidityV2IxArgs,
) ->  std::io::Result<Instruction> {
    increase_liquidity_v2_ix_with_program_id(crate::ID, keys, args)
}

pub fn increase_liquidity_v2_invoke_with_program_id(
    program_id: Pubkey,
    accounts: IncreaseLiquidityV2Accounts<'_, '_>,
    args: IncreaseLiquidityV2IxArgs,
) -> ProgramResult {
    let keys: IncreaseLiquidityV2Keys = accounts.into();
    let ix = increase_liquidity_v2_ix_with_program_id(program_id, keys, args)?;
       invoke_instruction(&ix, accounts)
}

pub fn increase_liquidity_v2_invoke(
    accounts: IncreaseLiquidityV2Accounts<'_, '_>,
    args: IncreaseLiquidityV2IxArgs,
) -> ProgramResult {
    increase_liquidity_v2_invoke_with_program_id(crate::ID, accounts, args)
}

pub fn increase_liquidity_v2_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: IncreaseLiquidityV2Accounts<'_, '_>,
    args: IncreaseLiquidityV2IxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: IncreaseLiquidityV2Keys = accounts.into();
    let ix = increase_liquidity_v2_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}

pub fn increase_liquidity_v2_invoke_signed(
    accounts: IncreaseLiquidityV2Accounts<'_, '_>,
    args: IncreaseLiquidityV2IxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    increase_liquidity_v2_invoke_signed_with_program_id(crate::ID, accounts, args, seeds)
}

pub fn increase_liquidity_v2_verify_account_keys(
    accounts: IncreaseLiquidityV2Accounts<'_, '_>,
    keys: IncreaseLiquidityV2Keys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.nft_owner.key, keys.nft_owner),
        (*accounts.nft_account.key, keys.nft_account),
        (*accounts.pool_state.key, keys.pool_state),
        (*accounts.protocol_position.key, keys.protocol_position),
        (*accounts.personal_position.key, keys.personal_position),
        (*accounts.tick_array_lower.key, keys.tick_array_lower),
        (*accounts.tick_array_upper.key, keys.tick_array_upper),
        (*accounts.token_account0.key, keys.token_account0),
        (*accounts.token_account1.key, keys.token_account1),
        (*accounts.token_vault0.key, keys.token_vault0),
        (*accounts.token_vault1.key, keys.token_vault1),
        (*accounts.token_program.key, keys.token_program),
        (*accounts.token_program2022.key, keys.token_program2022),
        (*accounts.vault0_mint.key, keys.vault0_mint),
        (*accounts.vault1_mint.key, keys.vault1_mint),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}

pub fn increase_liquidity_v2_verify_writable_privileges<'me, 'info>(
    accounts: IncreaseLiquidityV2Accounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.pool_state,
        accounts.protocol_position,
        accounts.personal_position,
        accounts.tick_array_lower,
        accounts.tick_array_upper,
        accounts.token_account0,
        accounts.token_account1,
        accounts.token_vault0,
        accounts.token_vault1,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}

pub fn increase_liquidity_v2_verify_signer_privileges<'me, 'info>(
    accounts: IncreaseLiquidityV2Accounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.nft_owner] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}

pub fn increase_liquidity_v2_verify_account_privileges<'me, 'info>(
    accounts: IncreaseLiquidityV2Accounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    increase_liquidity_v2_verify_writable_privileges(accounts)?;
    increase_liquidity_v2_verify_signer_privileges(accounts)?;
    Ok(())
}


pub const DECREASE_LIQUIDITY_IX_ACCOUNTS_LEN: usize = 12;

#[derive(Copy, Clone, Debug)]
pub struct DecreaseLiquidityAccounts<'me, 'info> {
    pub nft_owner: &'me AccountInfo<'info>,
    pub nft_account: &'me AccountInfo<'info>,
    pub personal_position: &'me AccountInfo<'info>,
    pub pool_state: &'me AccountInfo<'info>,
    pub protocol_position: &'me AccountInfo<'info>,
    pub token_vault_0: &'me AccountInfo<'info>,
    pub token_vault_1: &'me AccountInfo<'info>,
    pub tick_array_lower: &'me AccountInfo<'info>,
    pub tick_array_upper: &'me AccountInfo<'info>,
    pub recipient_token_account_0: &'me AccountInfo<'info>,
    pub recipient_token_account_1: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct DecreaseLiquidityKeys {
    pub nft_owner: Pubkey,
    pub nft_account: Pubkey,
    pub personal_position: Pubkey,
    pub pool_state: Pubkey,
    pub protocol_position: Pubkey,
    pub token_vault_0: Pubkey,
    pub token_vault_1: Pubkey,
    pub tick_array_lower: Pubkey,
    pub tick_array_upper: Pubkey,
    pub recipient_token_account_0: Pubkey,
    pub recipient_token_account_1: Pubkey,
    pub token_program: Pubkey,
}

impl From<DecreaseLiquidityAccounts<'_, '_>> for DecreaseLiquidityKeys {
    fn from(accounts: DecreaseLiquidityAccounts) -> Self {
        Self {
            nft_owner: *accounts.nft_owner.key,
            nft_account: *accounts.nft_account.key,
            personal_position: *accounts.personal_position.key,
            pool_state: *accounts.pool_state.key,
            protocol_position: *accounts.protocol_position.key,
            token_vault_0: *accounts.token_vault_0.key,
            token_vault_1: *accounts.token_vault_1.key,
            tick_array_lower: *accounts.tick_array_lower.key,
            tick_array_upper: *accounts.tick_array_upper.key,
            recipient_token_account_0: *accounts.recipient_token_account_0.key,
            recipient_token_account_1: *accounts.recipient_token_account_1.key,
            token_program: *accounts.token_program.key,
        }
    }
}

impl From<DecreaseLiquidityKeys> for [AccountMeta; DECREASE_LIQUIDITY_IX_ACCOUNTS_LEN] {
    fn from(keys: DecreaseLiquidityKeys) -> Self {
        [
            AccountMeta { pubkey: keys.nft_owner, is_signer: true, is_writable: false },
            AccountMeta { pubkey: keys.nft_account, is_signer: false, is_writable: false },
            AccountMeta { pubkey: keys.personal_position, is_signer: false, is_writable: true },
            AccountMeta { pubkey: keys.pool_state, is_signer: false, is_writable: true },
            AccountMeta { pubkey: keys.protocol_position, is_signer: false, is_writable: true },
            AccountMeta { pubkey: keys.token_vault_0, is_signer: false, is_writable: true },
            AccountMeta { pubkey: keys.token_vault_1, is_signer: false, is_writable: true },
            AccountMeta { pubkey: keys.tick_array_lower, is_signer: false, is_writable: true },
            AccountMeta { pubkey: keys.tick_array_upper, is_signer: false, is_writable: true },
            AccountMeta { pubkey: keys.recipient_token_account_0, is_signer: false, is_writable: true },
            AccountMeta { pubkey: keys.recipient_token_account_1, is_signer: false, is_writable: true },
            AccountMeta { pubkey: keys.token_program, is_signer: false, is_writable: false },
        ]
    }
}

impl From<[Pubkey; DECREASE_LIQUIDITY_IX_ACCOUNTS_LEN]> for DecreaseLiquidityKeys {
    fn from(pubkeys: [Pubkey; DECREASE_LIQUIDITY_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            nft_owner: pubkeys[0],
            nft_account: pubkeys[1],
            personal_position: pubkeys[2],
            pool_state: pubkeys[3],
            protocol_position: pubkeys[4],
            token_vault_0: pubkeys[5],
            token_vault_1: pubkeys[6],
            tick_array_lower: pubkeys[7],
            tick_array_upper: pubkeys[8],
            recipient_token_account_0: pubkeys[9],
            recipient_token_account_1: pubkeys[10],
            token_program: pubkeys[11],
        }
    }
}

impl<'info> From<DecreaseLiquidityAccounts<'_, 'info>> for [AccountInfo<'info>; DECREASE_LIQUIDITY_IX_ACCOUNTS_LEN] {
    fn from(accounts: DecreaseLiquidityAccounts<'_, 'info>) -> Self {
        [
            accounts.nft_owner.clone(),
            accounts.nft_account.clone(),
            accounts.personal_position.clone(),
            accounts.pool_state.clone(),
            accounts.protocol_position.clone(),
            accounts.token_vault_0.clone(),
            accounts.token_vault_1.clone(),
            accounts.tick_array_lower.clone(),
            accounts.tick_array_upper.clone(),
            accounts.recipient_token_account_0.clone(),
            accounts.recipient_token_account_1.clone(),
            accounts.token_program.clone(),
        ]
    }
}

impl<'me, 'info> From<&'me [AccountInfo<'info>; DECREASE_LIQUIDITY_IX_ACCOUNTS_LEN]> for DecreaseLiquidityAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; DECREASE_LIQUIDITY_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            nft_owner: &arr[0],
            nft_account: &arr[1],
            personal_position: &arr[2],
            pool_state: &arr[3],
            protocol_position: &arr[4],
            token_vault_0: &arr[5],
            token_vault_1: &arr[6],
            tick_array_lower: &arr[7],
            tick_array_upper: &arr[8],
            recipient_token_account_0: &arr[9],
            recipient_token_account_1: &arr[10],
            token_program: &arr[11],
        }
    }
}

// Placeholder discriminator (replace with actual discriminator for the instruction)
pub const DECREASE_LIQUIDITY_IX_DISCM:  u8 = 58u8;

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct DecreaseLiquidityIxArgs {
    pub liquidity: u128,
    pub amount_0_min: u64,
    pub amount_1_min: u64,
}

#[derive(Clone, Debug, PartialEq)]
pub struct DecreaseLiquidityIxData(pub DecreaseLiquidityIxArgs);

impl From<DecreaseLiquidityIxArgs> for DecreaseLiquidityIxData {
    fn from(args: DecreaseLiquidityIxArgs) -> Self {
        Self(args)
    }
}

impl DecreaseLiquidityIxData {
    pub fn deserialize(buf: &[u8]) ->std::io::Result<Self> {
          let mut reader = buf;
        let mut maybe_discm_buf = [0u8; 1];
        reader.read_exact(&mut maybe_discm_buf)?;
        let maybe_discm = maybe_discm_buf[0];
        if maybe_discm != DECREASE_LIQUIDITY_IX_DISCM {
            return Err(std::io::Error::new(
               std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    DECREASE_LIQUIDITY_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(DecreaseLiquidityIxArgs::deserialize(&mut reader)?))
    }

    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&[DECREASE_LIQUIDITY_IX_DISCM])?;
        self.0.serialize(&mut writer)
    }

    pub fn try_to_vec(&self) ->  std::io::Result<Vec<u8>>  {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}

pub fn decrease_liquidity_ix_with_program_id(
    program_id: Pubkey,
    keys: DecreaseLiquidityKeys,
    args: DecreaseLiquidityIxArgs,
) ->  std::io::Result<Instruction> {
    let metas: [AccountMeta; DECREASE_LIQUIDITY_IX_ACCOUNTS_LEN] = keys.into();
    let data: DecreaseLiquidityIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}

pub fn decrease_liquidity_ix(
    keys: DecreaseLiquidityKeys,
    args: DecreaseLiquidityIxArgs,
) ->  std::io::Result<Instruction> {
    decrease_liquidity_ix_with_program_id(crate::ID, keys, args)
}

pub fn decrease_liquidity_invoke_with_program_id(
    program_id: Pubkey,
    accounts: DecreaseLiquidityAccounts<'_, '_>,
    args: DecreaseLiquidityIxArgs,
) -> ProgramResult {
    let keys: DecreaseLiquidityKeys = accounts.into();
    let ix = decrease_liquidity_ix_with_program_id(program_id, keys, args)?;
       invoke_instruction(&ix, accounts)
}

pub fn decrease_liquidity_invoke(
    accounts: DecreaseLiquidityAccounts<'_, '_>,
    args: DecreaseLiquidityIxArgs,
) -> ProgramResult {
    decrease_liquidity_invoke_with_program_id(crate::ID, accounts, args)
}

pub fn decrease_liquidity_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: DecreaseLiquidityAccounts<'_, '_>,
    args: DecreaseLiquidityIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: DecreaseLiquidityKeys = accounts.into();
    let ix = decrease_liquidity_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}

pub fn decrease_liquidity_invoke_signed(
    accounts: DecreaseLiquidityAccounts<'_, '_>,
    args: DecreaseLiquidityIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    decrease_liquidity_invoke_signed_with_program_id(crate::ID, accounts, args, seeds)
}

pub fn decrease_liquidity_verify_account_keys(
    accounts: DecreaseLiquidityAccounts<'_, '_>,
    keys: DecreaseLiquidityKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.nft_owner.key, keys.nft_owner),
        (*accounts.nft_account.key, keys.nft_account),
        (*accounts.personal_position.key, keys.personal_position),
        (*accounts.pool_state.key, keys.pool_state),
        (*accounts.protocol_position.key, keys.protocol_position),
        (*accounts.token_vault_0.key, keys.token_vault_0),
        (*accounts.token_vault_1.key, keys.token_vault_1),
        (*accounts.tick_array_lower.key, keys.tick_array_lower),
        (*accounts.tick_array_upper.key, keys.tick_array_upper),
        (*accounts.recipient_token_account_0.key, keys.recipient_token_account_0),
        (*accounts.recipient_token_account_1.key, keys.recipient_token_account_1),
        (*accounts.token_program.key, keys.token_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}

pub fn decrease_liquidity_verify_writable_privileges<'me, 'info>(
    accounts: DecreaseLiquidityAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.personal_position,
        accounts.pool_state,
        accounts.protocol_position,
        accounts.token_vault_0,
        accounts.token_vault_1,
        accounts.tick_array_lower,
        accounts.tick_array_upper,
        accounts.recipient_token_account_0,
        accounts.recipient_token_account_1,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}

pub fn decrease_liquidity_verify_signer_privileges<'me, 'info>(
    accounts: DecreaseLiquidityAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.nft_owner] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}

pub fn decrease_liquidity_verify_account_privileges<'me, 'info>(
    accounts: DecreaseLiquidityAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    decrease_liquidity_verify_writable_privileges(accounts)?;
    decrease_liquidity_verify_signer_privileges(accounts)?;
    Ok(())
}

pub const DECREASE_LIQUIDITY_V2_IX_ACCOUNTS_LEN: usize = 16;

#[derive(Copy, Clone, Debug)]
pub struct DecreaseLiquidityV2Accounts<'me, 'info> {
    pub nft_owner: &'me AccountInfo<'info>,
    pub nft_account: &'me AccountInfo<'info>,
    pub personal_position: &'me AccountInfo<'info>,
    pub pool_state: &'me AccountInfo<'info>,
    pub protocol_position: &'me AccountInfo<'info>,
    pub token_vault0: &'me AccountInfo<'info>,
    pub token_vault1: &'me AccountInfo<'info>,
    pub tick_array_lower: &'me AccountInfo<'info>,
    pub tick_array_upper: &'me AccountInfo<'info>,
    pub recipient_token_account0: &'me AccountInfo<'info>,
    pub recipient_token_account1: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub token_program2022: &'me AccountInfo<'info>,
    pub memo_program: &'me AccountInfo<'info>,
    pub vault0_mint: &'me AccountInfo<'info>,
    pub vault1_mint: &'me AccountInfo<'info>,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct DecreaseLiquidityV2Keys {
    pub nft_owner: Pubkey,
    pub nft_account: Pubkey,
    pub personal_position: Pubkey,
    pub pool_state: Pubkey,
    pub protocol_position: Pubkey,
    pub token_vault0: Pubkey,
    pub token_vault1: Pubkey,
    pub tick_array_lower: Pubkey,
    pub tick_array_upper: Pubkey,
    pub recipient_token_account0: Pubkey,
    pub recipient_token_account1: Pubkey,
    pub token_program: Pubkey,
    pub token_program2022: Pubkey,
    pub memo_program: Pubkey,
    pub vault0_mint: Pubkey,
    pub vault1_mint: Pubkey,
}

impl From<DecreaseLiquidityV2Accounts<'_, '_>> for DecreaseLiquidityV2Keys {
    fn from(accounts: DecreaseLiquidityV2Accounts) -> Self {
        Self {
            nft_owner: *accounts.nft_owner.key,
            nft_account: *accounts.nft_account.key,
            personal_position: *accounts.personal_position.key,
            pool_state: *accounts.pool_state.key,
            protocol_position: *accounts.protocol_position.key,
            token_vault0: *accounts.token_vault0.key,
            token_vault1: *accounts.token_vault1.key,
            tick_array_lower: *accounts.tick_array_lower.key,
            tick_array_upper: *accounts.tick_array_upper.key,
            recipient_token_account0: *accounts.recipient_token_account0.key,
            recipient_token_account1: *accounts.recipient_token_account1.key,
            token_program: *accounts.token_program.key,
            token_program2022: *accounts.token_program2022.key,
            memo_program: *accounts.memo_program.key,
            vault0_mint: *accounts.vault0_mint.key,
            vault1_mint: *accounts.vault1_mint.key,
        }
    }
}

impl From<DecreaseLiquidityV2Keys> for [AccountMeta; DECREASE_LIQUIDITY_V2_IX_ACCOUNTS_LEN] {
    fn from(keys: DecreaseLiquidityV2Keys) -> Self {
        [
            AccountMeta { pubkey: keys.nft_owner, is_signer: true, is_writable: false },
            AccountMeta { pubkey: keys.nft_account, is_signer: false, is_writable: false },
            AccountMeta { pubkey: keys.personal_position, is_signer: false, is_writable: true },
            AccountMeta { pubkey: keys.pool_state, is_signer: false, is_writable: true },
            AccountMeta { pubkey: keys.protocol_position, is_signer: false, is_writable: true },
            AccountMeta { pubkey: keys.token_vault0, is_signer: false, is_writable: true },
            AccountMeta { pubkey: keys.token_vault1, is_signer: false, is_writable: true },
            AccountMeta { pubkey: keys.tick_array_lower, is_signer: false, is_writable: true },
            AccountMeta { pubkey: keys.tick_array_upper, is_signer: false, is_writable: true },
            AccountMeta { pubkey: keys.recipient_token_account0, is_signer: false, is_writable: true },
            AccountMeta { pubkey: keys.recipient_token_account1, is_signer: false, is_writable: true },
            AccountMeta { pubkey: keys.token_program, is_signer: false, is_writable: false },
            AccountMeta { pubkey: keys.token_program2022, is_signer: false, is_writable: false },
            AccountMeta { pubkey: keys.memo_program, is_signer: false, is_writable: false },
            AccountMeta { pubkey: keys.vault0_mint, is_signer: false, is_writable: false },
            AccountMeta { pubkey: keys.vault1_mint, is_signer: false, is_writable: false },
        ]
    }
}

impl From<[Pubkey; DECREASE_LIQUIDITY_V2_IX_ACCOUNTS_LEN]> for DecreaseLiquidityV2Keys {
    fn from(pubkeys: [Pubkey; DECREASE_LIQUIDITY_V2_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            nft_owner: pubkeys[0],
            nft_account: pubkeys[1],
            personal_position: pubkeys[2],
            pool_state: pubkeys[3],
            protocol_position: pubkeys[4],
            token_vault0: pubkeys[5],
            token_vault1: pubkeys[6],
            tick_array_lower: pubkeys[7],
            tick_array_upper: pubkeys[8],
            recipient_token_account0: pubkeys[9],
            recipient_token_account1: pubkeys[10],
            token_program: pubkeys[11],
            token_program2022: pubkeys[12],
            memo_program: pubkeys[13],
            vault0_mint: pubkeys[14],
            vault1_mint: pubkeys[15],
        }
    }
}

impl<'info> From<DecreaseLiquidityV2Accounts<'_, 'info>> for [AccountInfo<'info>; DECREASE_LIQUIDITY_V2_IX_ACCOUNTS_LEN] {
    fn from(accounts: DecreaseLiquidityV2Accounts<'_, 'info>) -> Self {
        [
            accounts.nft_owner.clone(),
            accounts.nft_account.clone(),
            accounts.personal_position.clone(),
            accounts.pool_state.clone(),
            accounts.protocol_position.clone(),
            accounts.token_vault0.clone(),
            accounts.token_vault1.clone(),
            accounts.tick_array_lower.clone(),
            accounts.tick_array_upper.clone(),
            accounts.recipient_token_account0.clone(),
            accounts.recipient_token_account1.clone(),
            accounts.token_program.clone(),
            accounts.token_program2022.clone(),
            accounts.memo_program.clone(),
            accounts.vault0_mint.clone(),
            accounts.vault1_mint.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; DECREASE_LIQUIDITY_V2_IX_ACCOUNTS_LEN]> for DecreaseLiquidityV2Accounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; DECREASE_LIQUIDITY_V2_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            nft_owner: &arr[0],
            nft_account: &arr[1],
            personal_position: &arr[2],
            pool_state: &arr[3],
            protocol_position: &arr[4],
            token_vault0: &arr[5],
            token_vault1: &arr[6],
            tick_array_lower: &arr[7],
            tick_array_upper: &arr[8],
            recipient_token_account0: &arr[9],
            recipient_token_account1: &arr[10],
            token_program: &arr[11],
            token_program2022: &arr[12],
            memo_program: &arr[13],
            vault0_mint: &arr[14],
            vault1_mint: &arr[15]
        }
    }
}

pub const DECREASE_LIQUIDITY_V2_IX_DISCM:u8 = 21u8;
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct DecreaseLiquidityV2IxArgs {
    pub liquidity: u128,
    pub amount0_min: u64,
    pub amount1_min: u64,
    pub base_flag: Option<bool>,
}
#[derive(Clone, Debug, PartialEq)]
pub struct DecreaseLiquidityV2IxData(pub DecreaseLiquidityV2IxArgs);
impl From<DecreaseLiquidityV2IxArgs> for DecreaseLiquidityV2IxData {
    fn from(args: DecreaseLiquidityV2IxArgs) -> Self {
        Self(args)
    }
}
impl DecreaseLiquidityV2IxData {
    pub fn deserialize(buf: &[u8]) ->std::io::Result<Self> {
          let mut reader = buf;
        let mut maybe_discm_buf = [0u8; 1];
        reader.read_exact(&mut maybe_discm_buf)?;
        let maybe_discm = maybe_discm_buf[0];
        if maybe_discm != DECREASE_LIQUIDITY_V2_IX_DISCM {
            return Err(std::io::Error::new(
               std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    DECREASE_LIQUIDITY_V2_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(DecreaseLiquidityV2IxArgs::deserialize(&mut reader)?))
    }
pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
    writer.write_all(&[DECREASE_LIQUIDITY_V2_IX_DISCM])?;
    self.0.serialize(&mut writer)
}

pub fn try_to_vec(&self) ->  std::io::Result<Vec<u8>>  {
    let mut data = Vec::new();
    self.serialize(&mut data)?;
    Ok(data)
}
}
pub fn decrease_liquidity_v2_ix_with_program_id(
    program_id: Pubkey,
    keys: DecreaseLiquidityV2Keys,
    args: DecreaseLiquidityV2IxArgs,
) ->  std::io::Result<Instruction> {
    let metas: [AccountMeta; DECREASE_LIQUIDITY_V2_IX_ACCOUNTS_LEN] = keys.into();
    let data: DecreaseLiquidityV2IxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn decrease_liquidity_v2_ix(
    keys: DecreaseLiquidityV2Keys,
    args: DecreaseLiquidityV2IxArgs,
) ->  std::io::Result<Instruction> {
    decrease_liquidity_v2_ix_with_program_id(crate::ID, keys, args)
}
pub fn decrease_liquidity_v2_invoke_with_program_id(
    program_id: Pubkey,
    accounts: DecreaseLiquidityV2Accounts<'_, '_>,
    args: DecreaseLiquidityV2IxArgs,
) -> ProgramResult {
    let keys: DecreaseLiquidityV2Keys = accounts.into();
    let ix = decrease_liquidity_v2_ix_with_program_id(program_id, keys, args)?;
       invoke_instruction(&ix, accounts)
}
pub fn decrease_liquidity_v2_invoke(
    accounts: DecreaseLiquidityV2Accounts<'_, '_>,
    args: DecreaseLiquidityV2IxArgs,
) -> ProgramResult {
    decrease_liquidity_v2_invoke_with_program_id(crate::ID, accounts, args)
}
pub fn decrease_liquidity_v2_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: DecreaseLiquidityV2Accounts<'_, '_>,
    args: DecreaseLiquidityV2IxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: DecreaseLiquidityV2Keys = accounts.into();
    let ix = decrease_liquidity_v2_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn decrease_liquidity_v2_invoke_signed(
    accounts: DecreaseLiquidityV2Accounts<'_, '_>,
    args: DecreaseLiquidityV2IxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    decrease_liquidity_v2_invoke_signed_with_program_id(crate::ID, accounts, args, seeds)
}
pub fn decrease_liquidity_v2_verify_account_keys(
    accounts: DecreaseLiquidityV2Accounts<'_, '_>,
    keys: DecreaseLiquidityV2Keys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.nft_owner.key, keys.nft_owner),
        (*accounts.nft_account.key, keys.nft_account),
        (*accounts.personal_position.key, keys.personal_position),
        (*accounts.pool_state.key, keys.pool_state),
        (*accounts.protocol_position.key, keys.protocol_position),
        (*accounts.token_vault0.key, keys.token_vault0),
        (*accounts.token_vault1.key, keys.token_vault1),
        (*accounts.tick_array_lower.key, keys.tick_array_lower),
        (*accounts.tick_array_upper.key, keys.tick_array_upper),
        (*accounts.recipient_token_account0.key, keys.recipient_token_account0),
        (*accounts.recipient_token_account1.key, keys.recipient_token_account1),
        (*accounts.token_program.key, keys.token_program),
        (*accounts.token_program2022.key, keys.token_program2022),
        (*accounts.memo_program.key, keys.memo_program),
        (*accounts.vault0_mint.key, keys.vault0_mint),
        (*accounts.vault1_mint.key, keys.vault1_mint),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn decrease_liquidity_v2_verify_writable_privileges<'me, 'info>(
    accounts: DecreaseLiquidityV2Accounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.personal_position,
        accounts.pool_state,
        accounts.protocol_position,
        accounts.token_vault0,
        accounts.token_vault1,
        accounts.tick_array_lower,
        accounts.tick_array_upper,
        accounts.recipient_token_account0,
        accounts.recipient_token_account1
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn decrease_liquidity_v2_verify_signer_privileges<'me, 'info>(
    accounts: DecreaseLiquidityV2Accounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    if !accounts.nft_owner.is_signer {
        return Err((
            accounts.nft_owner,
            ProgramError::MissingRequiredSignature,
        ));
    }
    Ok(())
}
pub fn decrease_liquidity_v2_verify_account_privileges<'me, 'info>(
    accounts: DecreaseLiquidityV2Accounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    decrease_liquidity_v2_verify_writable_privileges(accounts)?;
    decrease_liquidity_v2_verify_signer_privileges(accounts)?;
    Ok(())
}
// --- collectFee ---
pub const COLLECT_FEE_IX_ACCOUNTS_LEN: usize = 16;
#[derive(Copy, Clone, Debug)]
pub struct CollectFeeAccounts<'me, 'info> {
    pub position_nft_owner: &'me AccountInfo<'info>,
    pub position_nft_account: &'me AccountInfo<'info>,
    pub pool_state: &'me AccountInfo<'info>,
    pub protocol_position: &'me AccountInfo<'info>,
    pub tick_array_lower: &'me AccountInfo<'info>,
    pub tick_array_upper: &'me AccountInfo<'info>,
    pub personal_position: &'me AccountInfo<'info>,
    pub token_vault0: &'me AccountInfo<'info>,
    pub token_vault1: &'me AccountInfo<'info>,
    pub recipient_token_account0: &'me AccountInfo<'info>,
    pub recipient_token_account1: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub rent: &'me AccountInfo<'info>,
    pub associated_token_program: &'me AccountInfo<'info>,
    pub vault0_mint: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CollectFeeKeys {
    pub position_nft_owner: Pubkey,
    pub position_nft_account: Pubkey,
    pub pool_state: Pubkey,
    pub protocol_position: Pubkey,
    pub tick_array_lower: Pubkey,
    pub tick_array_upper: Pubkey,
    pub personal_position: Pubkey,
    pub token_vault0: Pubkey,
    pub token_vault1: Pubkey,
    pub recipient_token_account0: Pubkey,
    pub recipient_token_account1: Pubkey,
    pub token_program: Pubkey,
    pub system_program: Pubkey,
    pub rent: Pubkey,
    pub associated_token_program: Pubkey,
    pub vault0_mint: Pubkey,
}
impl From<CollectFeeAccounts<'_, '_>> for CollectFeeKeys {
    fn from(accounts: CollectFeeAccounts) -> Self {
        Self {
            position_nft_owner: *accounts.position_nft_owner.key,
            position_nft_account: *accounts.position_nft_account.key,
            pool_state: *accounts.pool_state.key,
            protocol_position: *accounts.protocol_position.key,
            tick_array_lower: *accounts.tick_array_lower.key,
            tick_array_upper: *accounts.tick_array_upper.key,
            personal_position: *accounts.personal_position.key,
            token_vault0: *accounts.token_vault0.key,
            token_vault1: *accounts.token_vault1.key,
            recipient_token_account0: *accounts.recipient_token_account0.key,
            recipient_token_account1: *accounts.recipient_token_account1.key,
            token_program: *accounts.token_program.key,
            system_program: *accounts.system_program.key,
            rent: *accounts.rent.key,
            associated_token_program: *accounts.associated_token_program.key,
            vault0_mint: *accounts.vault0_mint.key,
        }
    }
}
impl From<CollectFeeKeys> for [AccountMeta; COLLECT_FEE_IX_ACCOUNTS_LEN] {
    fn from(keys: CollectFeeKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.position_nft_owner,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.position_nft_account,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pool_state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.protocol_position,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.tick_array_lower,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.tick_array_upper,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.personal_position,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_vault0,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_vault1,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.recipient_token_account0,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.recipient_token_account1,
                is_signer: false,
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
                pubkey: keys.associated_token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.vault0_mint,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; COLLECT_FEE_IX_ACCOUNTS_LEN]> for CollectFeeKeys {
    fn from(pubkeys: [Pubkey; COLLECT_FEE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            position_nft_owner: pubkeys[0],
            position_nft_account: pubkeys[1],
            pool_state: pubkeys[2],
            protocol_position: pubkeys[3],
            tick_array_lower: pubkeys[4],
            tick_array_upper: pubkeys[5],
            personal_position: pubkeys[6],
            token_vault0: pubkeys[7],
            token_vault1: pubkeys[8],
            recipient_token_account0: pubkeys[9],
            recipient_token_account1: pubkeys[10],
            token_program: pubkeys[11],
            system_program: pubkeys[12],
            rent: pubkeys[13],
            associated_token_program: pubkeys[14],
            vault0_mint: pubkeys[15],
        }
    }
}
impl<'info> From<CollectFeeAccounts<'_, 'info>> for [AccountInfo<'info>; COLLECT_FEE_IX_ACCOUNTS_LEN] {
    fn from(accounts: CollectFeeAccounts<'_, 'info>) -> Self {
        [
            accounts.position_nft_owner.clone(),
            accounts.position_nft_account.clone(),
            accounts.pool_state.clone(),
            accounts.protocol_position.clone(),
            accounts.tick_array_lower.clone(),
            accounts.tick_array_upper.clone(),
            accounts.personal_position.clone(),
            accounts.token_vault0.clone(),
            accounts.token_vault1.clone(),
            accounts.recipient_token_account0.clone(),
            accounts.recipient_token_account1.clone(),
            accounts.token_program.clone(),
            accounts.system_program.clone(),
            accounts.rent.clone(),
            accounts.associated_token_program.clone(),
            accounts.vault0_mint.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; COLLECT_FEE_IX_ACCOUNTS_LEN]> for CollectFeeAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; COLLECT_FEE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            position_nft_owner: &arr[0],
            position_nft_account: &arr[1],
            pool_state: &arr[2],
            protocol_position: &arr[3],
            tick_array_lower: &arr[4],
            tick_array_upper: &arr[5],
            personal_position: &arr[6],
            token_vault0: &arr[7],
            token_vault1: &arr[8],
            recipient_token_account0: &arr[9],
            recipient_token_account1: &arr[10],
            token_program: &arr[11],
            system_program: &arr[12],
            rent: &arr[13],
            associated_token_program: &arr[14],
            vault0_mint: &arr[15],
        }
    }
}
pub const COLLECT_FEE_IX_DISCM:u8 = 22u8;

#[derive(Clone, Debug, PartialEq)]
pub struct CollectFeeIxData;
impl CollectFeeIxData {
    pub fn deserialize(_buf: &[u8]) ->std::io::Result<Self> {
        let mut reader = _buf;
        let mut maybe_discm_buf = [0u8; 1];
        reader.read_exact(&mut maybe_discm_buf)?;
        let maybe_discm = maybe_discm_buf[0];
        if maybe_discm != COLLECT_FEE_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    COLLECT_FEE_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self)
    }
pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
    writer.write_all(&[COLLECT_FEE_IX_DISCM])?;
    Ok(())
}

pub fn try_to_vec(&self) ->  std::io::Result<Vec<u8>>  {
    let mut data = Vec::new();
    self.serialize(&mut data)?;
    Ok(data)
}
}
pub fn collect_fee_ix_with_program_id(
    program_id: Pubkey,
    keys: CollectFeeKeys,
) ->  std::io::Result<Instruction> {
    let metas: [AccountMeta; COLLECT_FEE_IX_ACCOUNTS_LEN] = keys.into();
    let data = CollectFeeIxData;
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn collect_fee_ix(keys: CollectFeeKeys) ->  std::io::Result<Instruction> {
    collect_fee_ix_with_program_id(crate::ID, keys)
}
pub fn collect_fee_invoke_with_program_id(
    program_id: Pubkey,
    accounts: CollectFeeAccounts<'_, '_>,
) -> ProgramResult {
    let keys: CollectFeeKeys = accounts.into();
    let ix = collect_fee_ix_with_program_id(program_id, keys)?;
       invoke_instruction(&ix, accounts)
}
pub fn collect_fee_invoke(accounts: CollectFeeAccounts<'_, '_>) -> ProgramResult {
    collect_fee_invoke_with_program_id(crate::ID, accounts)
}
pub fn collect_fee_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: CollectFeeAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: CollectFeeKeys = accounts.into();
    let ix = collect_fee_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn collect_fee_invoke_signed(
    accounts: CollectFeeAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    collect_fee_invoke_signed_with_program_id(crate::ID, accounts, seeds)
}
pub fn collect_fee_verify_account_keys(
    accounts: CollectFeeAccounts<'_, '_>,
    keys: CollectFeeKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.position_nft_owner.key, keys.position_nft_owner),
        (*accounts.position_nft_account.key, keys.position_nft_account),
        (*accounts.pool_state.key, keys.pool_state),
        (*accounts.protocol_position.key, keys.protocol_position),
        (*accounts.tick_array_lower.key, keys.tick_array_lower),
        (*accounts.tick_array_upper.key, keys.tick_array_upper),
        (*accounts.personal_position.key, keys.personal_position),
        (*accounts.token_vault0.key, keys.token_vault0),
        (*accounts.token_vault1.key, keys.token_vault1),
        (*accounts.recipient_token_account0.key, keys.recipient_token_account0),
        (*accounts.recipient_token_account1.key, keys.recipient_token_account1),
        (*accounts.token_program.key, keys.token_program),
        (*accounts.system_program.key, keys.system_program),
        (*accounts.rent.key, keys.rent),
        (*accounts.associated_token_program.key, keys.associated_token_program),
        (*accounts.vault0_mint.key, keys.vault0_mint),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn collect_fee_verify_writable_privileges<'me, 'info>(
    accounts: CollectFeeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.pool_state,
        accounts.protocol_position,
        accounts.tick_array_lower,
        accounts.tick_array_upper,
        accounts.personal_position,
        accounts.token_vault0,
        accounts.token_vault1,
        accounts.recipient_token_account0,
        accounts.recipient_token_account1,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn collect_fee_verify_signer_privileges<'me, 'info>(
    accounts: CollectFeeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    if !accounts.position_nft_owner.is_signer {
        return Err((
            accounts.position_nft_owner,
            ProgramError::MissingRequiredSignature,
        ));
    }
    Ok(())
}
pub fn collect_fee_verify_account_privileges<'me, 'info>(
    accounts: CollectFeeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    collect_fee_verify_writable_privileges(accounts)?;
    collect_fee_verify_signer_privileges(accounts)?;
    Ok(())
}
// --- collectFeeV2 ---
pub const COLLECT_FEE_V2_IX_ACCOUNTS_LEN: usize = 17;
#[derive(Copy, Clone, Debug)]
pub struct CollectFeeV2Accounts<'me, 'info> {
    pub position_nft_owner: &'me AccountInfo<'info>,
    pub position_nft_account: &'me AccountInfo<'info>,
    pub pool_state: &'me AccountInfo<'info>,
    pub protocol_position: &'me AccountInfo<'info>,
    pub tick_array_lower: &'me AccountInfo<'info>,
    pub tick_array_upper: &'me AccountInfo<'info>,
    pub personal_position: &'me AccountInfo<'info>,
    pub token_vault0: &'me AccountInfo<'info>,
    pub token_vault1: &'me AccountInfo<'info>,
    pub recipient_token_account0: &'me AccountInfo<'info>,
    pub recipient_token_account1: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub rent: &'me AccountInfo<'info>,
    pub associated_token_program: &'me AccountInfo<'info>,
    pub token_program2022: &'me AccountInfo<'info>,
    pub vault0_mint: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CollectFeeV2Keys {
    pub position_nft_owner: Pubkey,
    pub position_nft_account: Pubkey,
    pub pool_state: Pubkey,
    pub protocol_position: Pubkey,
    pub tick_array_lower: Pubkey,
    pub tick_array_upper: Pubkey,
    pub personal_position: Pubkey,
    pub token_vault0: Pubkey,
    pub token_vault1: Pubkey,
    pub recipient_token_account0: Pubkey,
    pub recipient_token_account1: Pubkey,
    pub token_program: Pubkey,
    pub system_program: Pubkey,
    pub rent: Pubkey,
    pub associated_token_program: Pubkey,
    pub token_program2022: Pubkey,
    pub vault0_mint: Pubkey,
}
impl From<CollectFeeV2Accounts<'_, '_>> for CollectFeeV2Keys {
    fn from(accounts: CollectFeeV2Accounts) -> Self {
        Self {
            position_nft_owner: *accounts.position_nft_owner.key,
            position_nft_account: *accounts.position_nft_account.key,
            pool_state: *accounts.pool_state.key,
            protocol_position: *accounts.protocol_position.key,
            tick_array_lower: *accounts.tick_array_lower.key,
            tick_array_upper: *accounts.tick_array_upper.key,
            personal_position: *accounts.personal_position.key,
            token_vault0: *accounts.token_vault0.key,
            token_vault1: *accounts.token_vault1.key,
            recipient_token_account0: *accounts.recipient_token_account0.key,
            recipient_token_account1: *accounts.recipient_token_account1.key,
            token_program: *accounts.token_program.key,
            system_program: *accounts.system_program.key,
            rent: *accounts.rent.key,
            associated_token_program: *accounts.associated_token_program.key,
            token_program2022: *accounts.token_program2022.key,
            vault0_mint: *accounts.vault0_mint.key,
        }
    }
}
impl From<CollectFeeV2Keys> for [AccountMeta; COLLECT_FEE_V2_IX_ACCOUNTS_LEN] {
    fn from(keys: CollectFeeV2Keys) -> Self {
        [
            AccountMeta {
                pubkey: keys.position_nft_owner,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.position_nft_account,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pool_state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.protocol_position,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.tick_array_lower,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.tick_array_upper,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.personal_position,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_vault0,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_vault1,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.recipient_token_account0,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.recipient_token_account1,
                is_signer: false,
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
                pubkey: keys.associated_token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_program2022,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.vault0_mint,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; COLLECT_FEE_V2_IX_ACCOUNTS_LEN]> for CollectFeeV2Keys {
    fn from(pubkeys: [Pubkey; COLLECT_FEE_V2_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            position_nft_owner: pubkeys[0],
            position_nft_account: pubkeys[1],
            pool_state: pubkeys[2],
            protocol_position: pubkeys[3],
            tick_array_lower: pubkeys[4],
            tick_array_upper: pubkeys[5],
            personal_position: pubkeys[6],
            token_vault0: pubkeys[7],
            token_vault1: pubkeys[8],
            recipient_token_account0: pubkeys[9],
            recipient_token_account1: pubkeys[10],
            token_program: pubkeys[11],
            system_program: pubkeys[12],
            rent: pubkeys[13],
            associated_token_program: pubkeys[14],
            token_program2022: pubkeys[15],
            vault0_mint: pubkeys[16],
        }
    }
}
impl<'info> From<CollectFeeV2Accounts<'_, 'info>> for [AccountInfo<'info>; COLLECT_FEE_V2_IX_ACCOUNTS_LEN] {
    fn from(accounts: CollectFeeV2Accounts<'_, 'info>) -> Self {
        [
            accounts.position_nft_owner.clone(),
            accounts.position_nft_account.clone(),
            accounts.pool_state.clone(),
            accounts.protocol_position.clone(),
            accounts.tick_array_lower.clone(),
            accounts.tick_array_upper.clone(),
            accounts.personal_position.clone(),
            accounts.token_vault0.clone(),
            accounts.token_vault1.clone(),
            accounts.recipient_token_account0.clone(),
            accounts.recipient_token_account1.clone(),
            accounts.token_program.clone(),
            accounts.system_program.clone(),
            accounts.rent.clone(),
            accounts.associated_token_program.clone(),
            accounts.token_program2022.clone(),
            accounts.vault0_mint.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; COLLECT_FEE_V2_IX_ACCOUNTS_LEN]> for CollectFeeV2Accounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; COLLECT_FEE_V2_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            position_nft_owner: &arr[0],
            position_nft_account: &arr[1],
            pool_state: &arr[2],
            protocol_position: &arr[3],
            tick_array_lower: &arr[4],
            tick_array_upper: &arr[5],
            personal_position: &arr[6],
            token_vault0: &arr[7],
            token_vault1: &arr[8],
            recipient_token_account0: &arr[9],
            recipient_token_account1: &arr[10],
            token_program: &arr[11],
            system_program: &arr[12],
            rent: &arr[13],
            associated_token_program: &arr[14],
            token_program2022: &arr[15],
            vault0_mint: &arr[16],
        }
    }
}
pub const COLLECT_FEE_V2_IX_DISCM: u8 = 23u8;
#[derive(Clone, Debug, PartialEq)]
pub struct CollectFeeV2IxData;
impl CollectFeeV2IxData {
    pub fn deserialize(buf: &[u8]) ->std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm_buf = [0u8; 1];
        reader.read_exact(&mut maybe_discm_buf)?;
        let maybe_discm = maybe_discm_buf[0];
        if maybe_discm != COLLECT_FEE_V2_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    COLLECT_FEE_V2_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self)
    }
pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
    writer.write_all(&[COLLECT_FEE_V2_IX_DISCM])?;
    Ok(())
}

pub fn try_to_vec(&self) ->  std::io::Result<Vec<u8>>  {
    let mut data = Vec::new();
    self.serialize(&mut data)?;
    Ok(data)
}
}
pub fn collect_fee_v2_ix_with_program_id(
    program_id: Pubkey,
    keys: CollectFeeV2Keys,
) ->  std::io::Result<Instruction> {
    let metas: [AccountMeta; COLLECT_FEE_V2_IX_ACCOUNTS_LEN] = keys.into();
    let data = CollectFeeV2IxData;
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn collect_fee_v2_ix(keys: CollectFeeV2Keys) ->  std::io::Result<Instruction> {
    collect_fee_v2_ix_with_program_id(crate::ID, keys)
}
pub fn collect_fee_v2_invoke_with_program_id(
    program_id: Pubkey,
    accounts: CollectFeeV2Accounts<'_, '_>,
) -> ProgramResult {
    let keys: CollectFeeV2Keys = accounts.into();
    let ix = collect_fee_v2_ix_with_program_id(program_id, keys)?;
       invoke_instruction(&ix, accounts)
}
pub fn collect_fee_v2_invoke(accounts: CollectFeeV2Accounts<'_, '_>) -> ProgramResult {
    collect_fee_v2_invoke_with_program_id(crate::ID, accounts)
}
pub fn collect_fee_v2_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: CollectFeeV2Accounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: CollectFeeV2Keys = accounts.into();
    let ix = collect_fee_v2_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn collect_fee_v2_invoke_signed(
    accounts: CollectFeeV2Accounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    collect_fee_v2_invoke_signed_with_program_id(crate::ID, accounts, seeds)
}
pub fn collect_fee_v2_verify_account_keys(
    accounts: CollectFeeV2Accounts<'_, '_>,
    keys: CollectFeeV2Keys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.position_nft_owner.key, keys.position_nft_owner),
        (*accounts.position_nft_account.key, keys.position_nft_account),
        (*accounts.pool_state.key, keys.pool_state),
        (*accounts.protocol_position.key, keys.protocol_position),
        (*accounts.tick_array_lower.key, keys.tick_array_lower),
        (*accounts.tick_array_upper.key, keys.tick_array_upper),
        (*accounts.personal_position.key, keys.personal_position),
        (*accounts.token_vault0.key, keys.token_vault0),
        (*accounts.token_vault1.key, keys.token_vault1),
        (*accounts.recipient_token_account0.key, keys.recipient_token_account0),
        (*accounts.recipient_token_account1.key, keys.recipient_token_account1),
        (*accounts.token_program.key, keys.token_program),
        (*accounts.system_program.key, keys.system_program),
        (*accounts.rent.key, keys.rent),
        (*accounts.associated_token_program.key, keys.associated_token_program),
        (*accounts.token_program2022.key, keys.token_program2022),
        (*accounts.vault0_mint.key, keys.vault0_mint),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn collect_fee_v2_verify_writable_privileges<'me, 'info>(
    accounts: CollectFeeV2Accounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.pool_state,
        accounts.protocol_position,
        accounts.tick_array_lower,
        accounts.tick_array_upper,
        accounts.personal_position,
        accounts.token_vault0,
        accounts.token_vault1,
        accounts.recipient_token_account0,
        accounts.recipient_token_account1,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn collect_fee_v2_verify_signer_privileges<'me, 'info>(
    accounts: CollectFeeV2Accounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    if !accounts.position_nft_owner.is_signer {
        return Err((
            accounts.position_nft_owner,
            ProgramError::MissingRequiredSignature,
        ));
    }
    Ok(())
}
pub fn collect_fee_v2_verify_account_privileges<'me, 'info>(
    accounts: CollectFeeV2Accounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    collect_fee_v2_verify_writable_privileges(accounts)?;
    collect_fee_v2_verify_signer_privileges(accounts)?;
    Ok(())
}

pub const SWAP_IX_ACCOUNTS_LEN: usize = 10;
#[derive(Copy, Clone, Debug)]
pub struct SwapAccounts<'me, 'info> {
    pub payer: &'me AccountInfo<'info>,
    pub amm_config: &'me AccountInfo<'info>,
    pub pool_state: &'me AccountInfo<'info>,
    pub input_token_account: &'me AccountInfo<'info>,
    pub output_token_account: &'me AccountInfo<'info>,
    pub input_vault: &'me AccountInfo<'info>,
    pub output_vault: &'me AccountInfo<'info>,
    pub observation_state: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub tick_array: &'me AccountInfo<'info>,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SwapKeys {
    pub payer: Pubkey,
    pub amm_config: Pubkey,
    pub pool_state: Pubkey,
    pub input_token_account: Pubkey,
    pub output_token_account: Pubkey,
    pub input_vault: Pubkey,
    pub output_vault: Pubkey,
    pub observation_state: Pubkey,
    pub token_program: Pubkey,
    pub tick_array: Pubkey,
}

impl From<SwapAccounts<'_, '_>> for SwapKeys {
    fn from(accounts: SwapAccounts) -> Self {
        Self {
            payer: *accounts.payer.key,
            amm_config: *accounts.amm_config.key,
            pool_state: *accounts.pool_state.key,
            input_token_account: *accounts.input_token_account.key,
            output_token_account: *accounts.output_token_account.key,
            input_vault: *accounts.input_vault.key,
            output_vault: *accounts.output_vault.key,
            observation_state: *accounts.observation_state.key,
            token_program: *accounts.token_program.key,
            tick_array: *accounts.tick_array.key,
        }
    }
}

impl From<SwapKeys> for [AccountMeta; SWAP_IX_ACCOUNTS_LEN] {
    fn from(keys: SwapKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.payer,
                is_signer: true,
                is_writable: true, 
            },
            AccountMeta {
                pubkey: keys.amm_config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pool_state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.input_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.output_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.input_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.output_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.observation_state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.tick_array,
                is_signer: false,
                is_writable: true,
            },
        ]
    }
}

impl From<[Pubkey; SWAP_IX_ACCOUNTS_LEN]> for SwapKeys {
    fn from(pubkeys: [Pubkey; SWAP_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            payer: pubkeys[0],
            amm_config: pubkeys[1],
            pool_state: pubkeys[2],
            input_token_account: pubkeys[3],
            output_token_account: pubkeys[4],
            input_vault: pubkeys[5],
            output_vault: pubkeys[6],
            observation_state: pubkeys[7],
            token_program: pubkeys[8],
            tick_array: pubkeys[9],
        }
    }
}

impl<'info> From<SwapAccounts<'_, 'info>> for [AccountInfo<'info>; SWAP_IX_ACCOUNTS_LEN] {
    fn from(accounts: SwapAccounts<'_, 'info>) -> Self {
        [
            accounts.payer.clone(),
            accounts.amm_config.clone(),
            accounts.pool_state.clone(),
            accounts.input_token_account.clone(),
            accounts.output_token_account.clone(),
            accounts.input_vault.clone(),
            accounts.output_vault.clone(),
            accounts.observation_state.clone(),
            accounts.token_program.clone(),
            accounts.tick_array.clone(),
        ]
    }
}

impl<'me, 'info> From<&'me [AccountInfo<'info>; SWAP_IX_ACCOUNTS_LEN]> for SwapAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; SWAP_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            payer: &arr[0],
            amm_config: &arr[1],
            pool_state: &arr[2],
            input_token_account: &arr[3],
            output_token_account: &arr[4],
            input_vault: &arr[5],
            output_vault: &arr[6],
            observation_state: &arr[7],
            token_program: &arr[8],
            tick_array: &arr[9],
        }
    }
}

pub const SWAP_IX_DISCM: u8 = 248u8;

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SwapIxArgs {
    pub amount: u64,
    pub other_amount_threshold: u64,
    pub sqrt_price_limit_x64: u128,
    pub is_base_input: u8,
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
        let mut maybe_discm_buf = [0u8; 1];
        reader.read_exact(&mut maybe_discm_buf)?;
        let maybe_discm = maybe_discm_buf[0];
        if maybe_discm != SWAP_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {}. Received: {}",
                    SWAP_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(SwapIxArgs::deserialize(&mut reader)?))
    }

    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&[SWAP_IX_DISCM])?;
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
        (*accounts.payer.key, keys.payer),
        (*accounts.amm_config.key, keys.amm_config),
        (*accounts.pool_state.key, keys.pool_state),
        (*accounts.input_token_account.key, keys.input_token_account),
        (*accounts.output_token_account.key, keys.output_token_account),
        (*accounts.input_vault.key, keys.input_vault),
        (*accounts.output_vault.key, keys.output_vault),
        (*accounts.observation_state.key, keys.observation_state),
        (*accounts.token_program.key, keys.token_program),
        (*accounts.tick_array.key, keys.tick_array),
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
        accounts.pool_state,
        accounts.input_token_account,
        accounts.output_token_account,
        accounts.input_vault,
        accounts.output_vault,
        accounts.observation_state,
        accounts.tick_array,
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
    if !accounts.payer.is_signer {
        return Err((accounts.payer, ProgramError::MissingRequiredSignature));
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
pub const SWAP_V2_IX_ACCOUNTS_LEN: usize = 13;
#[derive(Copy, Clone, Debug)]
pub struct SwapV2Accounts<'me, 'info> {
    pub payer: &'me AccountInfo<'info>,
    pub amm_config: &'me AccountInfo<'info>,
    pub pool_state: &'me AccountInfo<'info>,
    pub input_token_account: &'me AccountInfo<'info>,
    pub output_token_account: &'me AccountInfo<'info>,
    pub input_vault: &'me AccountInfo<'info>,
    pub output_vault: &'me AccountInfo<'info>,
    pub observation_state: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub token_program2022: &'me AccountInfo<'info>,
    pub memo_program: &'me AccountInfo<'info>,
    pub input_vault_mint: &'me AccountInfo<'info>,
    pub output_vault_mint: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SwapV2Keys {
    pub payer: Pubkey,
    pub amm_config: Pubkey,
    pub pool_state: Pubkey,
    pub input_token_account: Pubkey,
    pub output_token_account: Pubkey,
    pub input_vault: Pubkey,
    pub output_vault: Pubkey,
    pub observation_state: Pubkey,
    pub token_program: Pubkey,
    pub token_program2022: Pubkey,
    pub memo_program: Pubkey,
    pub input_vault_mint: Pubkey,
    pub output_vault_mint: Pubkey,
}
impl From<SwapV2Accounts<'_, '_>> for SwapV2Keys {
    fn from(accounts: SwapV2Accounts) -> Self {
        Self {
            payer: *accounts.payer.key,
            amm_config: *accounts.amm_config.key,
            pool_state: *accounts.pool_state.key,
            input_token_account: *accounts.input_token_account.key,
            output_token_account: *accounts.output_token_account.key,
            input_vault: *accounts.input_vault.key,
            output_vault: *accounts.output_vault.key,
            observation_state: *accounts.observation_state.key,
            token_program: *accounts.token_program.key,
            token_program2022: *accounts.token_program2022.key,
            memo_program: *accounts.memo_program.key,
            input_vault_mint: *accounts.input_vault_mint.key,
            output_vault_mint: *accounts.output_vault_mint.key
        }
    }
}
impl From<SwapV2Keys> for [AccountMeta; SWAP_V2_IX_ACCOUNTS_LEN] {
    fn from(keys: SwapV2Keys) -> Self {
        [
            AccountMeta {
                pubkey: keys.payer,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.amm_config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pool_state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.input_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.output_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.input_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.output_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.observation_state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_program2022,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.memo_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.input_vault_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.output_vault_mint,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; SWAP_V2_IX_ACCOUNTS_LEN]> for SwapV2Keys {
    fn from(pubkeys: [Pubkey; SWAP_V2_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            payer: pubkeys[0],
            amm_config: pubkeys[1],
            pool_state: pubkeys[2],
            input_token_account: pubkeys[3],
            output_token_account: pubkeys[4],
            input_vault: pubkeys[5],
            output_vault: pubkeys[6],
            observation_state: pubkeys[7],
            token_program: pubkeys[8],
            token_program2022: pubkeys[9],
            memo_program: pubkeys[10],
            input_vault_mint: pubkeys[11],
            output_vault_mint: pubkeys[12]
        }
    }
}
impl<'info> From<SwapV2Accounts<'_, 'info>> for [AccountInfo<'info>; SWAP_V2_IX_ACCOUNTS_LEN] {
    fn from(accounts: SwapV2Accounts<'_, 'info>) -> Self {
        [
            accounts.payer.clone(),
            accounts.amm_config.clone(),
            accounts.pool_state.clone(),
            accounts.input_token_account.clone(),
            accounts.output_token_account.clone(),
            accounts.input_vault.clone(),
            accounts.output_vault.clone(),
            accounts.observation_state.clone(),
            accounts.token_program.clone(),
            accounts.token_program2022.clone(),
            accounts.memo_program.clone(),
            accounts.input_vault_mint.clone(),
            accounts.output_vault_mint.clone()
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; SWAP_V2_IX_ACCOUNTS_LEN]> for SwapV2Accounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; SWAP_V2_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            payer: &arr[0],
            amm_config: &arr[1],
            pool_state: &arr[2],
            input_token_account: &arr[3],
            output_token_account: &arr[4],
            input_vault: &arr[5],
            output_vault: &arr[6],
            observation_state: &arr[7],
            token_program: &arr[8],
            token_program2022: &arr[9],
            memo_program: &arr[10],
            input_vault_mint: &arr[11],
            output_vault_mint: &arr[12]
        }
    }
}
pub const SWAP_V2_IX_DISCM:  u8 = 43u8;
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SwapV2IxArgs {
    pub amount : u64,
    pub other_amount_threshold: u64,
    pub sqrt_price_limit_x64: u128,
    pub is_base_input: u8,
}
#[derive(Clone, Debug, PartialEq)]
pub struct SwapV2IxData(pub SwapV2IxArgs);
impl From<SwapV2IxArgs> for SwapV2IxData {
    fn from(args: SwapV2IxArgs) -> Self {
        Self(args)
    }
}
impl SwapV2IxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut buf = buf; 
        let mut maybe_discm_buf = [0u8; 1];
        buf.read_exact(&mut maybe_discm_buf)?;
        let maybe_discm = maybe_discm_buf[0];

        if maybe_discm != SWAP_V2_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    SWAP_V2_IX_DISCM, maybe_discm
                ),
            ));
        }

        Ok(Self(SwapV2IxArgs::deserialize(&mut buf)?))
    }
pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
    writer.write_all(&[SWAP_V2_IX_DISCM])?;
    self.0.serialize(&mut writer)
}

pub fn try_to_vec(&self) ->  std::io::Result<Vec<u8>>  {
      let mut data = Vec::new();
      self.serialize(&mut data)?;
      Ok(data)
 }
}
pub fn swap_v2_ix_with_program_id(
    program_id: Pubkey,
    keys: SwapV2Keys,
    args: SwapV2IxArgs,
) ->  std::io::Result<Instruction> {
    let metas: [AccountMeta; SWAP_V2_IX_ACCOUNTS_LEN] = keys.into();
    let data: SwapV2IxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn swap_v2_ix(keys: SwapV2Keys, args: SwapV2IxArgs) ->  std::io::Result<Instruction> {
    swap_v2_ix_with_program_id(crate::ID, keys, args)
}
pub fn swap_v2_invoke_with_program_id(
    program_id: Pubkey,
    accounts: SwapV2Accounts<'_, '_>,
    args: SwapV2IxArgs,
) -> ProgramResult {
    let keys: SwapV2Keys = accounts.into();
    let ix = swap_v2_ix_with_program_id(program_id, keys, args)?;
       invoke_instruction(&ix, accounts)
}
pub fn swap_v2_invoke(accounts: SwapV2Accounts<'_, '_>, args: SwapV2IxArgs) -> ProgramResult {
    swap_v2_invoke_with_program_id(crate::ID, accounts, args)
}
pub fn swap_v2_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: SwapV2Accounts<'_, '_>,
    args: SwapV2IxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: SwapV2Keys = accounts.into();
    let ix = swap_v2_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn swap_v2_invoke_signed(
    accounts: SwapV2Accounts<'_, '_>,
    args: SwapV2IxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    swap_v2_invoke_signed_with_program_id(crate::ID, accounts, args, seeds)
}
pub fn swap_v2_verify_account_keys(
    accounts: SwapV2Accounts<'_, '_>,
    keys: SwapV2Keys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.payer.key, keys.payer),
        (*accounts.amm_config.key, keys.amm_config),
        (*accounts.pool_state.key, keys.pool_state),
        (*accounts.input_token_account.key, keys.input_token_account),
        (*accounts.output_token_account.key, keys.output_token_account),
        (*accounts.input_vault.key, keys.input_vault),
        (*accounts.output_vault.key, keys.output_vault),
        (*accounts.observation_state.key, keys.observation_state),
        (*accounts.token_program.key, keys.token_program),
        (*accounts.token_program2022.key, keys.token_program2022),
        (*accounts.memo_program.key, keys.memo_program),
        (*accounts.input_vault_mint.key, keys.input_vault_mint),
        (*accounts.output_vault_mint.key, keys.output_vault_mint)
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn swap_v2_verify_writable_privileges<'me, 'info>(
    accounts: SwapV2Accounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.payer,
        accounts.pool_state,
        accounts.input_token_account,
        accounts.output_token_account,
        accounts.input_vault,
        accounts.output_vault,
        accounts.observation_state
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn swap_v2_verify_signer_privileges<'me, 'info>(
    accounts: SwapV2Accounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    if !accounts.payer.is_signer {
        return Err((accounts.payer, ProgramError::MissingRequiredSignature));
    }
    Ok(())
}
pub fn swap_v2_verify_account_privileges<'me, 'info>(
    accounts: SwapV2Accounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    swap_v2_verify_writable_privileges(accounts)?;
    swap_v2_verify_signer_privileges(accounts)?;
    Ok(())
}

pub const SWAP_ROUTER_BASE_IN_IX_ACCOUNTS_LEN: usize = 6;

#[derive(Copy, Clone, Debug)]
pub struct SwapRouterBaseInAccounts<'me, 'info> {
    pub payer: &'me AccountInfo<'info>,
    pub input_token_account: &'me AccountInfo<'info>,
    pub input_token_mint: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub token_program_2022: &'me AccountInfo<'info>,
    pub memo_program: &'me AccountInfo<'info>,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SwapRouterBaseInKeys {
    pub payer: Pubkey,
    pub input_token_account: Pubkey,
    pub input_token_mint: Pubkey,
    pub token_program: Pubkey,
    pub token_program_2022: Pubkey,
    pub memo_program: Pubkey,
}

impl From<SwapRouterBaseInAccounts<'_, '_>> for SwapRouterBaseInKeys {
    fn from(accounts: SwapRouterBaseInAccounts) -> Self {
        Self {
            payer: *accounts.payer.key,
            input_token_account: *accounts.input_token_account.key,
            input_token_mint: *accounts.input_token_mint.key,
            token_program: *accounts.token_program.key,
            token_program_2022: *accounts.token_program_2022.key,
            memo_program: *accounts.memo_program.key,
        }
    }
}

impl From<SwapRouterBaseInKeys> for [AccountMeta; SWAP_ROUTER_BASE_IN_IX_ACCOUNTS_LEN] {
    fn from(keys: SwapRouterBaseInKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.payer,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.input_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.input_token_mint,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_program_2022,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.memo_program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}

impl From<[Pubkey; SWAP_ROUTER_BASE_IN_IX_ACCOUNTS_LEN]> for SwapRouterBaseInKeys {
    fn from(pubkeys: [Pubkey; SWAP_ROUTER_BASE_IN_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            payer: pubkeys[0],
            input_token_account: pubkeys[1],
            input_token_mint: pubkeys[2],
            token_program: pubkeys[3],
            token_program_2022: pubkeys[4],
            memo_program: pubkeys[5],
        }
    }
}

impl<'info> From<SwapRouterBaseInAccounts<'_, 'info>> for [AccountInfo<'info>; SWAP_ROUTER_BASE_IN_IX_ACCOUNTS_LEN] {
    fn from(accounts: SwapRouterBaseInAccounts<'_, 'info>) -> Self {
        [
            accounts.payer.clone(),
            accounts.input_token_account.clone(),
            accounts.input_token_mint.clone(),
            accounts.token_program.clone(),
            accounts.token_program_2022.clone(),
            accounts.memo_program.clone(),
        ]
    }
}

impl<'me, 'info> From<&'me [AccountInfo<'info>; SWAP_ROUTER_BASE_IN_IX_ACCOUNTS_LEN]> for SwapRouterBaseInAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; SWAP_ROUTER_BASE_IN_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            payer: &arr[0],
            input_token_account: &arr[1],
            input_token_mint: &arr[2],
            token_program: &arr[3],
            token_program_2022: &arr[4],
            memo_program: &arr[5],
        }
    }
}

pub const SWAP_ROUTER_BASE_IN_IX_DISCM: u8 = 26u8;

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SwapRouterBaseInIxArgs {
    pub amount_in: u64,
    pub amount_out_minimum: u64,
}

#[derive(Clone, Debug, PartialEq)]
pub struct SwapRouterBaseInIxData(pub SwapRouterBaseInIxArgs);

impl From<SwapRouterBaseInIxArgs> for SwapRouterBaseInIxData {
    fn from(args: SwapRouterBaseInIxArgs) -> Self {
        Self(args)
    }
}

impl SwapRouterBaseInIxData {
    pub fn deserialize(buf: &[u8]) ->std::io::Result<Self> {
          let mut reader = buf;
        let mut maybe_discm_buf = [0u8; 1];
        reader.read_exact(&mut maybe_discm_buf)?;
        let maybe_discm = maybe_discm_buf[0];
        if maybe_discm != SWAP_ROUTER_BASE_IN_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discriminator does not match. Expected: {:?}. Received: {:?}",
                    SWAP_ROUTER_BASE_IN_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(SwapRouterBaseInIxArgs::deserialize(&mut reader)?))
    }

    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&[SWAP_ROUTER_BASE_IN_IX_DISCM])?;
        self.0.serialize(&mut writer)
    }

    pub fn try_to_vec(&self) ->  std::io::Result<Vec<u8>>  {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}

pub fn swap_router_base_in_ix_with_program_id(
    program_id: Pubkey,
    keys: SwapRouterBaseInKeys,
    args: SwapRouterBaseInIxArgs,
) ->  std::io::Result<Instruction> {
    let metas: [AccountMeta; SWAP_ROUTER_BASE_IN_IX_ACCOUNTS_LEN] = keys.into();
    let data: SwapRouterBaseInIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}

pub fn swap_router_base_in_ix(
    keys: SwapRouterBaseInKeys,
    args: SwapRouterBaseInIxArgs,
) ->  std::io::Result<Instruction> {
    swap_router_base_in_ix_with_program_id(crate::ID, keys, args)
}

pub fn swap_router_base_in_invoke_with_program_id(
    program_id: Pubkey,
    accounts: SwapRouterBaseInAccounts<'_, '_>,
    args: SwapRouterBaseInIxArgs,
) -> ProgramResult {
    let keys: SwapRouterBaseInKeys = accounts.into();
    let ix = swap_router_base_in_ix_with_program_id(program_id, keys, args)?;
       invoke_instruction(&ix, accounts)
}

pub fn swap_router_base_in_invoke(
    accounts: SwapRouterBaseInAccounts<'_, '_>,
    args: SwapRouterBaseInIxArgs,
) -> ProgramResult {
    swap_router_base_in_invoke_with_program_id(crate::ID, accounts, args)
}

pub fn swap_router_base_in_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: SwapRouterBaseInAccounts<'_, '_>,
    args: SwapRouterBaseInIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: SwapRouterBaseInKeys = accounts.into();
    let ix = swap_router_base_in_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}

pub fn swap_router_base_in_invoke_signed(
    accounts: SwapRouterBaseInAccounts<'_, '_>,
    args: SwapRouterBaseInIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    swap_router_base_in_invoke_signed_with_program_id(crate::ID, accounts, args, seeds)
}

pub fn swap_router_base_in_verify_account_keys(
    accounts: SwapRouterBaseInAccounts<'_, '_>,
    keys: SwapRouterBaseInKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.payer.key, keys.payer),
        (*accounts.input_token_account.key, keys.input_token_account),
        (*accounts.input_token_mint.key, keys.input_token_mint),
        (*accounts.token_program.key, keys.token_program),
        (*accounts.token_program_2022.key, keys.token_program_2022),
        (*accounts.memo_program.key, keys.memo_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}

pub fn swap_router_base_in_verify_writable_privileges<'me, 'info>(
    accounts: SwapRouterBaseInAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.input_token_account,
        accounts.input_token_mint,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}

pub fn swap_router_base_in_verify_signer_privileges<'me, 'info>(
    accounts: SwapRouterBaseInAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    if !accounts.payer.is_signer {
        return Err((accounts.payer, ProgramError::MissingRequiredSignature));
    }
    Ok(())
}

pub fn swap_router_base_in_verify_account_privileges<'me, 'info>(
    accounts: SwapRouterBaseInAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    swap_router_base_in_verify_writable_privileges(accounts)?;
    swap_router_base_in_verify_signer_privileges(accounts)?;
    Ok(())
}