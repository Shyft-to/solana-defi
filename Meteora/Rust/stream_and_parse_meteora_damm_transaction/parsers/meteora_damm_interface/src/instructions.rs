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
use typedefs::{AddLiquidityParameters,InitializeCustomizablePoolParameters};
use strum_macros::{Display, EnumString};

#[derive(Clone, Debug, PartialEq, EnumString, Display)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum MeteoraDammProgramIx {
    AddLiquidity(AddLiquidityIxArgs),
    ClaimPartnerFee(ClaimPartnerFeeIxArgs),
    ClaimPositionFee,
    ClaimProtocolFee,
    ClaimReward(ClaimRewardIxArgs),
    CloseClaimFeeOperator,
    CloseConfig,
    ClosePosition,
    CreateClaimFeeOperator,
    CreateConfig(CreateConfigIxArgs),
    CreateDynamicConfig(CreateDynamicConfigIxArgs),
    CreatePosition,
    CreateTokenBadge,
    FundReward(FundRewardIxArgs),
    InitializeCustomizablePool(InitializeCustomizablePoolIxArgs),
    InitializePool(InitializePoolIxArgs),
    InitializePoolWithDynamicConfig(InitializePoolWithDynamicConfigIxArgs),
    InitializeReward(InitializeRewardIxArgs),
    LockPosition(LockPositionIxArgs),
    PermanentLockPosition(PermanentLockPositionIxArgs),
    RefreshVesting,
    RemoveAllLiquidity(RemoveAllLiquidityIxArgs),
    RemoveLiquidity(RemoveLiquidityIxArgs),
    SetPoolStatus(SetPoolStatusIxArgs),
    Swap(SwapIxArgs),
    UpdateRewardDuration(UpdateRewardDurationIxArgs),
    UpdateRewardFunder(UpdateRewardFunderIxArgs),
    WithdrawIneligibleReward(WithdrawIneligibleRewardIxArgs),
}

impl MeteoraDammProgramIx {
    pub fn name(&self) -> String {
        self.to_string().to_camel_case()
    }

    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        
        match maybe_discm {
            ADD_LIQUIDITY_IX_DISCM => { 
                Ok(
                    Self::AddLiquidity(AddLiquidityIxArgs::deserialize(&mut reader)?
                ),
             )
           }
            CLAIM_PARTNER_FEE_IX_DISCM => {
                Ok(Self::ClaimPartnerFee(ClaimPartnerFeeIxArgs::deserialize(&mut reader)?
            ),
          )
         }
            CLAIM_POSITION_FEE_IX_DISCM => {Ok(Self::ClaimPositionFee)}
            CLAIM_PROTOCOL_FEE_IX_DISCM => {Ok(Self::ClaimProtocolFee)}
            CLAIM_REWARD_IX_DISCM => {
                 Ok(Self::ClaimReward(ClaimRewardIxArgs::deserialize(&mut reader)?
                ),
             )
            } 
            CLOSE_CLAIM_FEE_OPERATOR_IX_DISCM => {Ok(Self::CloseClaimFeeOperator)}
            CLOSE_CONFIG_IX_DISCM => {Ok(Self::CloseConfig)}
            CLOSE_POSITION_IX_DISCM => {Ok(Self::ClosePosition)}
            CREATE_CLAIM_FEE_OPERATOR_IX_DISCM => {Ok(Self::CreateClaimFeeOperator)}
            CREATE_CONFIG_IX_DISCM => { 
                Ok(Self::CreateConfig(CreateConfigIxArgs::deserialize(&mut reader)?
              ),
             )
            } 
            CREATE_DYNAMIC_CONFIG_IX_DISCM => { 
                Ok(Self::CreateDynamicConfig(CreateDynamicConfigIxArgs::deserialize(&mut reader)?
             ),
            )
           }
            CREATE_POSITION_IX_DISCM => {Ok(Self::CreatePosition)}
            CREATE_TOKEN_BADGE_IX_DISCM => {Ok(Self::CreateTokenBadge)}
            FUND_REWARD_IX_DISCM => {
                 Ok(Self::FundReward(FundRewardIxArgs::deserialize(&mut reader)?
                ),
              )
            }
            INITIALIZE_CUSTOMIZABLE_POOL_IX_DISCM => {
                 Ok(Self::InitializeCustomizablePool(InitializeCustomizablePoolIxArgs::deserialize(&mut reader)?
                ),
              )
            }
            INITIALIZE_POOL_IX_DISCM =>  {
                Ok(Self::InitializePool(InitializePoolIxArgs::deserialize(&mut reader)?
              ),
             )
            }
            INITIALIZE_POOL_WITH_DYNAMIC_CONFIG_IX_DISCM =>{
                 Ok(Self::InitializePoolWithDynamicConfig(InitializePoolWithDynamicConfigIxArgs::deserialize(&mut reader)?
                ),
              )
            }
            INITIALIZE_REWARD_IX_DISCM => {
                Ok(Self::InitializeReward(InitializeRewardIxArgs::deserialize(&mut reader)?
              ),
             )
            }
            LOCK_POSITION_IX_DISCM => {
                Ok(Self::LockPosition(LockPositionIxArgs::deserialize(&mut reader)?
               ),
              )
            }
            PERMANENT_LOCK_POSITION_IX_DISCM => {
                Ok(Self::PermanentLockPosition(PermanentLockPositionIxArgs::deserialize(&mut reader)?
              ),
             )
            }
            REFRESH_VESTING_IX_DISCM => {Ok(Self::RefreshVesting)},
            REMOVE_ALL_LIQUIDITY_IX_DISCM => {
                 Ok(Self::RemoveAllLiquidity(RemoveAllLiquidityIxArgs::deserialize(&mut reader)?
                ),
               )
            }
            REMOVE_LIQUIDITY_IX_DISCM => {
                Ok(Self::RemoveLiquidity(RemoveLiquidityIxArgs::deserialize(&mut reader)?
              ),
             )
            }
            SET_POOL_STATUS_IX_DISCM =>{ 
                Ok(Self::SetPoolStatus(SetPoolStatusIxArgs::deserialize(&mut reader)?
               ),
              )
            }
            SWAP_IX_DISCM => {
                Ok(Self::Swap(SwapIxArgs::deserialize(&mut reader)?
               ),
             )
            }
            UPDATE_REWARD_DURATION_IX_DISCM =>{
             Ok(Self::UpdateRewardDuration(UpdateRewardDurationIxArgs::deserialize(&mut reader)?
              ),
             )
            }
            UPDATE_REWARD_FUNDER_IX_DISCM =>{
                 Ok(Self::UpdateRewardFunder(UpdateRewardFunderIxArgs::deserialize(&mut reader)?
                ),
               )
            }
            WITHDRAW_INELIGIBLE_REWARD_IX_DISCM => {
            Ok(Self::WithdrawIneligibleReward(WithdrawIneligibleRewardIxArgs::deserialize(&mut reader)?
            ),
           )
         }
            _ => { Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("discm {:?} not found", maybe_discm),
            ),
          )
          }
        }
    }

    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        match self {
            Self::AddLiquidity(args) => {
                writer.write_all(&ADD_LIQUIDITY_IX_DISCM)?;
                args.serialize(&mut writer)
            }
            Self::ClaimPartnerFee(args) => {
                writer.write_all(&CLAIM_PARTNER_FEE_IX_DISCM)?;
                args.serialize(&mut writer)
            }
            Self::ClaimPositionFee => {writer.write_all(&CLAIM_POSITION_FEE_IX_DISCM)}
            Self::ClaimProtocolFee => {writer.write_all(&CLAIM_PROTOCOL_FEE_IX_DISCM)}
            Self::ClaimReward(args) => {
                writer.write_all(&CLAIM_REWARD_IX_DISCM)?;
                args.serialize(&mut writer)
            }
            Self::CloseClaimFeeOperator => {writer.write_all(&CLOSE_CLAIM_FEE_OPERATOR_IX_DISCM)}
            Self::CloseConfig => {writer.write_all(&CLOSE_CONFIG_IX_DISCM)}
            Self::ClosePosition => {writer.write_all(&CLOSE_POSITION_IX_DISCM)}
            Self::CreateClaimFeeOperator => {writer.write_all(&CREATE_CLAIM_FEE_OPERATOR_IX_DISCM)}
            Self::CreateConfig(args) => {
                writer.write_all(&CREATE_CONFIG_IX_DISCM)?;
                args.serialize(&mut writer)
            }
            Self::CreateDynamicConfig(args) => {
                writer.write_all(&CREATE_DYNAMIC_CONFIG_IX_DISCM)?;
                args.serialize(&mut writer)
            }
            Self::CreatePosition => {writer.write_all(&CREATE_POSITION_IX_DISCM)}
            Self::CreateTokenBadge => {writer.write_all(&CREATE_TOKEN_BADGE_IX_DISCM)}
            Self::FundReward(args) => {
                writer.write_all(&FUND_REWARD_IX_DISCM)?;
                args.serialize(&mut writer)
            }
            Self::InitializeCustomizablePool(args) => {
                writer.write_all(&INITIALIZE_CUSTOMIZABLE_POOL_IX_DISCM)?;
                args.serialize(&mut writer)
            }
            Self::InitializePool(args) => {
                writer.write_all(&INITIALIZE_POOL_IX_DISCM)?;
                args.serialize(&mut writer)
            }
            Self::InitializePoolWithDynamicConfig(args) => {
                writer.write_all(&INITIALIZE_POOL_WITH_DYNAMIC_CONFIG_IX_DISCM)?;
                args.serialize(&mut writer)
            }
            Self::InitializeReward(args) => {
                writer.write_all(&INITIALIZE_REWARD_IX_DISCM)?;
                args.serialize(&mut writer)
            }
            Self::LockPosition(args) => {
                writer.write_all(&LOCK_POSITION_IX_DISCM)?;
                args.serialize(&mut writer)
            }
            Self::PermanentLockPosition(args) => {
                writer.write_all(&PERMANENT_LOCK_POSITION_IX_DISCM)?;
                args.serialize(&mut writer)
            }
            Self::RefreshVesting => {writer.write_all(&REFRESH_VESTING_IX_DISCM)}
            Self::RemoveAllLiquidity(args) => {
                writer.write_all(&REMOVE_ALL_LIQUIDITY_IX_DISCM)?;
                args.serialize(&mut writer)
            }
            Self::RemoveLiquidity(args) => {
                writer.write_all(&REMOVE_LIQUIDITY_IX_DISCM)?;
                args.serialize(&mut writer)
            }
            Self::SetPoolStatus(args) => {
                writer.write_all(&SET_POOL_STATUS_IX_DISCM)?;
                args.serialize(&mut writer)
            }
            Self::Swap(args) => {
                writer.write_all(&SWAP_IX_DISCM)?;
                args.serialize(&mut writer)
            }
            Self::UpdateRewardDuration(args) => {
                writer.write_all(&UPDATE_REWARD_DURATION_IX_DISCM)?;
                args.serialize(&mut writer)
            }
            Self::UpdateRewardFunder(args) => {
                writer.write_all(&UPDATE_REWARD_FUNDER_IX_DISCM)?;
                args.serialize(&mut writer)
            }
            Self::WithdrawIneligibleReward(args) => {
                writer.write_all(&WITHDRAW_INELIGIBLE_REWARD_IX_DISCM)?;
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
pub const ADD_LIQUIDITY_IX_ACCOUNTS_LEN: usize = 14;
#[derive(Copy, Clone, Debug)]
pub struct AddLiquidityAccounts<'me, 'info> {
    pub pool: &'me AccountInfo<'info>,
    pub position: &'me AccountInfo<'info>,
    pub token_a_account: &'me AccountInfo<'info>,
    pub token_b_account: &'me AccountInfo<'info>,
    pub token_a_vault: &'me AccountInfo<'info>,
    pub token_b_vault: &'me AccountInfo<'info>,
    pub token_a_mint: &'me AccountInfo<'info>,
    pub token_b_mint: &'me AccountInfo<'info>,
    pub position_nft_account: &'me AccountInfo<'info>,
    pub owner: &'me AccountInfo<'info>,
    pub token_a_program: &'me AccountInfo<'info>,
    pub token_b_program: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct AddLiquidityKeys {
    pub pool: Pubkey,
    pub position: Pubkey,
    pub token_a_account: Pubkey,
    pub token_b_account: Pubkey,
    pub token_a_vault: Pubkey,
    pub token_b_vault: Pubkey,
    pub token_a_mint: Pubkey,
    pub token_b_mint: Pubkey,
    pub position_nft_account: Pubkey,
    pub owner: Pubkey,
    pub token_a_program: Pubkey,
    pub token_b_program: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<AddLiquidityAccounts<'_, '_>> for AddLiquidityKeys {
    fn from(accounts: AddLiquidityAccounts) -> Self {
        Self {
            pool: *accounts.pool.key,
            position: *accounts.position.key,
            token_a_account: *accounts.token_a_account.key,
            token_b_account: *accounts.token_b_account.key,
            token_a_vault: *accounts.token_a_vault.key,
            token_b_vault: *accounts.token_b_vault.key,
            token_a_mint: *accounts.token_a_mint.key,
            token_b_mint: *accounts.token_b_mint.key,
            position_nft_account: *accounts.position_nft_account.key,
            owner: *accounts.owner.key,
            token_a_program: *accounts.token_a_program.key,
            token_b_program: *accounts.token_b_program.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<AddLiquidityKeys> for [AccountMeta; ADD_LIQUIDITY_IX_ACCOUNTS_LEN] {
    fn from(keys: AddLiquidityKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.pool,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.position,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_a_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_b_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_a_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_b_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_a_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_b_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.position_nft_account,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.owner,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_a_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_b_program,
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
            pool: pubkeys[0],
            position: pubkeys[1],
            token_a_account: pubkeys[2],
            token_b_account: pubkeys[3],
            token_a_vault: pubkeys[4],
            token_b_vault: pubkeys[5],
            token_a_mint: pubkeys[6],
            token_b_mint: pubkeys[7],
            position_nft_account: pubkeys[8],
            owner: pubkeys[9],
            token_a_program: pubkeys[10],
            token_b_program: pubkeys[11],
            event_authority: pubkeys[12],
            program: pubkeys[13],
        }
    }
}
impl<'info> From<AddLiquidityAccounts<'_, 'info>> for [AccountInfo<'info>; ADD_LIQUIDITY_IX_ACCOUNTS_LEN] {
    fn from(accounts: AddLiquidityAccounts<'_, 'info>) -> Self {
        [
            accounts.pool.clone(),
            accounts.position.clone(),
            accounts.token_a_account.clone(),
            accounts.token_b_account.clone(),
            accounts.token_a_vault.clone(),
            accounts.token_b_vault.clone(),
            accounts.token_a_mint.clone(),
            accounts.token_b_mint.clone(),
            accounts.position_nft_account.clone(),
            accounts.owner.clone(),
            accounts.token_a_program.clone(),
            accounts.token_b_program.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; ADD_LIQUIDITY_IX_ACCOUNTS_LEN]> for AddLiquidityAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; ADD_LIQUIDITY_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            pool: &arr[0],
            position: &arr[1],
            token_a_account: &arr[2],
            token_b_account: &arr[3],
            token_a_vault: &arr[4],
            token_b_vault: &arr[5],
            token_a_mint: &arr[6],
            token_b_mint: &arr[7],
            position_nft_account: &arr[8],
            owner: &arr[9],
            token_a_program: &arr[10],
            token_b_program: &arr[11],
            event_authority: &arr[12],
            program: &arr[13],
        }
    }
}
pub const ADD_LIQUIDITY_IX_DISCM: [u8; 8] = [181, 157, 89, 67, 143, 182, 52, 72];
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct AddLiquidityIxArgs {
    pub params: AddLiquidityParameters,
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
        (*accounts.pool.key, keys.pool),
        (*accounts.position.key, keys.position),
        (*accounts.token_a_account.key, keys.token_a_account),
        (*accounts.token_b_account.key, keys.token_b_account),
        (*accounts.token_a_vault.key, keys.token_a_vault),
        (*accounts.token_b_vault.key, keys.token_b_vault),
        (*accounts.token_a_mint.key, keys.token_a_mint),
        (*accounts.token_b_mint.key, keys.token_b_mint),
        (*accounts.position_nft_account.key, keys.position_nft_account),
        (*accounts.owner.key, keys.owner),
        (*accounts.token_a_program.key, keys.token_a_program),
        (*accounts.token_b_program.key, keys.token_b_program),
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
        accounts.pool,
        accounts.position,
        accounts.token_a_account,
        accounts.token_b_account,
        accounts.token_a_vault,
        accounts.token_b_vault,
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
    for should_be_signer in [accounts.owner] {
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
pub const CLAIM_PARTNER_FEE_IX_ACCOUNTS_LEN: usize = 13;
#[derive(Copy, Clone, Debug)]
pub struct ClaimPartnerFeeAccounts<'me, 'info> {
    pub pool_authority: &'me AccountInfo<'info>,
    pub pool: &'me AccountInfo<'info>,
    pub token_a_account: &'me AccountInfo<'info>,
    pub token_b_account: &'me AccountInfo<'info>,
    pub token_a_vault: &'me AccountInfo<'info>,
    pub token_b_vault: &'me AccountInfo<'info>,
    pub token_a_mint: &'me AccountInfo<'info>,
    pub token_b_mint: &'me AccountInfo<'info>,
    pub partner: &'me AccountInfo<'info>,
    pub token_a_program: &'me AccountInfo<'info>,
    pub token_b_program: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct ClaimPartnerFeeKeys {
    pub pool_authority: Pubkey,
    pub pool: Pubkey,
    pub token_a_account: Pubkey,
    pub token_b_account: Pubkey,
    pub token_a_vault: Pubkey,
    pub token_b_vault: Pubkey,
    pub token_a_mint: Pubkey,
    pub token_b_mint: Pubkey,
    pub partner: Pubkey,
    pub token_a_program: Pubkey,
    pub token_b_program: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<ClaimPartnerFeeAccounts<'_, '_>> for ClaimPartnerFeeKeys {
    fn from(accounts: ClaimPartnerFeeAccounts) -> Self {
        Self {
            pool_authority: *accounts.pool_authority.key,
            pool: *accounts.pool.key,
            token_a_account: *accounts.token_a_account.key,
            token_b_account: *accounts.token_b_account.key,
            token_a_vault: *accounts.token_a_vault.key,
            token_b_vault: *accounts.token_b_vault.key,
            token_a_mint: *accounts.token_a_mint.key,
            token_b_mint: *accounts.token_b_mint.key,
            partner: *accounts.partner.key,
            token_a_program: *accounts.token_a_program.key,
            token_b_program: *accounts.token_b_program.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<ClaimPartnerFeeKeys> for [AccountMeta; CLAIM_PARTNER_FEE_IX_ACCOUNTS_LEN] {
    fn from(keys: ClaimPartnerFeeKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.pool_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pool,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_a_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_b_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_a_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_b_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_a_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_b_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.partner,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_a_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_b_program,
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
impl From<[Pubkey; CLAIM_PARTNER_FEE_IX_ACCOUNTS_LEN]> for ClaimPartnerFeeKeys {
    fn from(pubkeys: [Pubkey; CLAIM_PARTNER_FEE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            pool_authority: pubkeys[0],
            pool: pubkeys[1],
            token_a_account: pubkeys[2],
            token_b_account: pubkeys[3],
            token_a_vault: pubkeys[4],
            token_b_vault: pubkeys[5],
            token_a_mint: pubkeys[6],
            token_b_mint: pubkeys[7],
            partner: pubkeys[8],
            token_a_program: pubkeys[9],
            token_b_program: pubkeys[10],
            event_authority: pubkeys[11],
            program: pubkeys[12],
        }
    }
}
impl<'info> From<ClaimPartnerFeeAccounts<'_, 'info>> for [AccountInfo<'info>; CLAIM_PARTNER_FEE_IX_ACCOUNTS_LEN] {
    fn from(accounts: ClaimPartnerFeeAccounts<'_, 'info>) -> Self {
        [
            accounts.pool_authority.clone(),
            accounts.pool.clone(),
            accounts.token_a_account.clone(),
            accounts.token_b_account.clone(),
            accounts.token_a_vault.clone(),
            accounts.token_b_vault.clone(),
            accounts.token_a_mint.clone(),
            accounts.token_b_mint.clone(),
            accounts.partner.clone(),
            accounts.token_a_program.clone(),
            accounts.token_b_program.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; CLAIM_PARTNER_FEE_IX_ACCOUNTS_LEN]> for ClaimPartnerFeeAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; CLAIM_PARTNER_FEE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            pool_authority: &arr[0],
            pool: &arr[1],
            token_a_account: &arr[2],
            token_b_account: &arr[3],
            token_a_vault: &arr[4],
            token_b_vault: &arr[5],
            token_a_mint: &arr[6],
            token_b_mint: &arr[7],
            partner: &arr[8],
            token_a_program: &arr[9],
            token_b_program: &arr[10],
            event_authority: &arr[11],
            program: &arr[12],
        }
    }
}
pub const CLAIM_PARTNER_FEE_IX_DISCM: [u8; 8] = [97, 206, 39, 105, 94, 94, 126, 148];
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ClaimPartnerFeeIxArgs {
    pub max_amount_a: u64,
    pub max_amount_b: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct ClaimPartnerFeeIxData(pub ClaimPartnerFeeIxArgs);
impl From<ClaimPartnerFeeIxArgs> for ClaimPartnerFeeIxData {
    fn from(args: ClaimPartnerFeeIxArgs) -> Self {
        Self(args)
    }
}
impl ClaimPartnerFeeIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != CLAIM_PARTNER_FEE_IX_DISCM {
            return Err(
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!(
                        "discm does not match. Expected: {:?}. Received: {:?}",
                        CLAIM_PARTNER_FEE_IX_DISCM, maybe_discm
                    ),
                ),
            );
        }
        Ok(Self(ClaimPartnerFeeIxArgs::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&CLAIM_PARTNER_FEE_IX_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn claim_partner_fee_ix_with_program_id(
    program_id: Pubkey,
    keys: ClaimPartnerFeeKeys,
    args: ClaimPartnerFeeIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; CLAIM_PARTNER_FEE_IX_ACCOUNTS_LEN] = keys.into();
    let data: ClaimPartnerFeeIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn claim_partner_fee_ix(
    keys: ClaimPartnerFeeKeys,
    args: ClaimPartnerFeeIxArgs,
) -> std::io::Result<Instruction> {
    claim_partner_fee_ix_with_program_id(crate::ID, keys, args)
}
pub fn claim_partner_fee_invoke_with_program_id(
    program_id: Pubkey,
    accounts: ClaimPartnerFeeAccounts<'_, '_>,
    args: ClaimPartnerFeeIxArgs,
) -> ProgramResult {
    let keys: ClaimPartnerFeeKeys = accounts.into();
    let ix = claim_partner_fee_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn claim_partner_fee_invoke(
    accounts: ClaimPartnerFeeAccounts<'_, '_>,
    args: ClaimPartnerFeeIxArgs,
) -> ProgramResult {
    claim_partner_fee_invoke_with_program_id(crate::ID, accounts, args)
}
pub fn claim_partner_fee_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: ClaimPartnerFeeAccounts<'_, '_>,
    args: ClaimPartnerFeeIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: ClaimPartnerFeeKeys = accounts.into();
    let ix = claim_partner_fee_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn claim_partner_fee_invoke_signed(
    accounts: ClaimPartnerFeeAccounts<'_, '_>,
    args: ClaimPartnerFeeIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    claim_partner_fee_invoke_signed_with_program_id(crate::ID, accounts, args, seeds)
}
pub fn claim_partner_fee_verify_account_keys(
    accounts: ClaimPartnerFeeAccounts<'_, '_>,
    keys: ClaimPartnerFeeKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.pool_authority.key, keys.pool_authority),
        (*accounts.pool.key, keys.pool),
        (*accounts.token_a_account.key, keys.token_a_account),
        (*accounts.token_b_account.key, keys.token_b_account),
        (*accounts.token_a_vault.key, keys.token_a_vault),
        (*accounts.token_b_vault.key, keys.token_b_vault),
        (*accounts.token_a_mint.key, keys.token_a_mint),
        (*accounts.token_b_mint.key, keys.token_b_mint),
        (*accounts.partner.key, keys.partner),
        (*accounts.token_a_program.key, keys.token_a_program),
        (*accounts.token_b_program.key, keys.token_b_program),
        (*accounts.event_authority.key, keys.event_authority),
        (*accounts.program.key, keys.program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn claim_partner_fee_verify_writable_privileges<'me, 'info>(
    accounts: ClaimPartnerFeeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.pool,
        accounts.token_a_account,
        accounts.token_b_account,
        accounts.token_a_vault,
        accounts.token_b_vault,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn claim_partner_fee_verify_signer_privileges<'me, 'info>(
    accounts: ClaimPartnerFeeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.partner] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn claim_partner_fee_verify_account_privileges<'me, 'info>(
    accounts: ClaimPartnerFeeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    claim_partner_fee_verify_writable_privileges(accounts)?;
    claim_partner_fee_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const CLAIM_POSITION_FEE_IX_ACCOUNTS_LEN: usize = 15;
#[derive(Copy, Clone, Debug)]
pub struct ClaimPositionFeeAccounts<'me, 'info> {
    pub pool_authority: &'me AccountInfo<'info>,
    pub pool: &'me AccountInfo<'info>,
    pub position: &'me AccountInfo<'info>,
    pub token_a_account: &'me AccountInfo<'info>,
    pub token_b_account: &'me AccountInfo<'info>,
    pub token_a_vault: &'me AccountInfo<'info>,
    pub token_b_vault: &'me AccountInfo<'info>,
    pub token_a_mint: &'me AccountInfo<'info>,
    pub token_b_mint: &'me AccountInfo<'info>,
    pub position_nft_account: &'me AccountInfo<'info>,
    pub owner: &'me AccountInfo<'info>,
    pub token_a_program: &'me AccountInfo<'info>,
    pub token_b_program: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct ClaimPositionFeeKeys {
    pub pool_authority: Pubkey,
    pub pool: Pubkey,
    pub position: Pubkey,
    pub token_a_account: Pubkey,
    pub token_b_account: Pubkey,
    pub token_a_vault: Pubkey,
    pub token_b_vault: Pubkey,
    pub token_a_mint: Pubkey,
    pub token_b_mint: Pubkey,
    pub position_nft_account: Pubkey,
    pub owner: Pubkey,
    pub token_a_program: Pubkey,
    pub token_b_program: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<ClaimPositionFeeAccounts<'_, '_>> for ClaimPositionFeeKeys {
    fn from(accounts: ClaimPositionFeeAccounts) -> Self {
        Self {
            pool_authority: *accounts.pool_authority.key,
            pool: *accounts.pool.key,
            position: *accounts.position.key,
            token_a_account: *accounts.token_a_account.key,
            token_b_account: *accounts.token_b_account.key,
            token_a_vault: *accounts.token_a_vault.key,
            token_b_vault: *accounts.token_b_vault.key,
            token_a_mint: *accounts.token_a_mint.key,
            token_b_mint: *accounts.token_b_mint.key,
            position_nft_account: *accounts.position_nft_account.key,
            owner: *accounts.owner.key,
            token_a_program: *accounts.token_a_program.key,
            token_b_program: *accounts.token_b_program.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<ClaimPositionFeeKeys> for [AccountMeta; CLAIM_POSITION_FEE_IX_ACCOUNTS_LEN] {
    fn from(keys: ClaimPositionFeeKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.pool_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pool,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.position,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_a_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_b_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_a_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_b_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_a_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_b_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.position_nft_account,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.owner,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_a_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_b_program,
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
impl From<[Pubkey; CLAIM_POSITION_FEE_IX_ACCOUNTS_LEN]> for ClaimPositionFeeKeys {
    fn from(pubkeys: [Pubkey; CLAIM_POSITION_FEE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            pool_authority: pubkeys[0],
            pool: pubkeys[1],
            position: pubkeys[2],
            token_a_account: pubkeys[3],
            token_b_account: pubkeys[4],
            token_a_vault: pubkeys[5],
            token_b_vault: pubkeys[6],
            token_a_mint: pubkeys[7],
            token_b_mint: pubkeys[8],
            position_nft_account: pubkeys[9],
            owner: pubkeys[10],
            token_a_program: pubkeys[11],
            token_b_program: pubkeys[12],
            event_authority: pubkeys[13],
            program: pubkeys[14],
        }
    }
}
impl<'info> From<ClaimPositionFeeAccounts<'_, 'info>> for [AccountInfo<'info>; CLAIM_POSITION_FEE_IX_ACCOUNTS_LEN] {
    fn from(accounts: ClaimPositionFeeAccounts<'_, 'info>) -> Self {
        [
            accounts.pool_authority.clone(),
            accounts.pool.clone(),
            accounts.position.clone(),
            accounts.token_a_account.clone(),
            accounts.token_b_account.clone(),
            accounts.token_a_vault.clone(),
            accounts.token_b_vault.clone(),
            accounts.token_a_mint.clone(),
            accounts.token_b_mint.clone(),
            accounts.position_nft_account.clone(),
            accounts.owner.clone(),
            accounts.token_a_program.clone(),
            accounts.token_b_program.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; CLAIM_POSITION_FEE_IX_ACCOUNTS_LEN]> for ClaimPositionFeeAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; CLAIM_POSITION_FEE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            pool_authority: &arr[0],
            pool: &arr[1],
            position: &arr[2],
            token_a_account: &arr[3],
            token_b_account: &arr[4],
            token_a_vault: &arr[5],
            token_b_vault: &arr[6],
            token_a_mint: &arr[7],
            token_b_mint: &arr[8],
            position_nft_account: &arr[9],
            owner: &arr[10],
            token_a_program: &arr[11],
            token_b_program: &arr[12],
            event_authority: &arr[13],
            program: &arr[14],
        }
    }
}
pub const CLAIM_POSITION_FEE_IX_DISCM: [u8; 8] = [180, 38, 154, 17, 133, 33, 162, 211];
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ClaimPositionFeeIxArgs {}
#[derive(Clone, Debug, PartialEq)]
pub struct ClaimPositionFeeIxData(pub ClaimPositionFeeIxArgs);
impl From<ClaimPositionFeeIxArgs> for ClaimPositionFeeIxData {
    fn from(args: ClaimPositionFeeIxArgs) -> Self {
        Self(args)
    }
}
impl ClaimPositionFeeIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != CLAIM_POSITION_FEE_IX_DISCM {
            return Err(
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!(
                        "discm does not match. Expected: {:?}. Received: {:?}",
                        CLAIM_POSITION_FEE_IX_DISCM, maybe_discm
                    ),
                ),
            );
        }
        Ok(Self(ClaimPositionFeeIxArgs::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&CLAIM_POSITION_FEE_IX_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn claim_position_fee_ix_with_program_id(
    program_id: Pubkey,
    keys: ClaimPositionFeeKeys,
    args: ClaimPositionFeeIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; CLAIM_POSITION_FEE_IX_ACCOUNTS_LEN] = keys.into();
    let data: ClaimPositionFeeIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn claim_position_fee_ix(
    keys: ClaimPositionFeeKeys,
    args: ClaimPositionFeeIxArgs,
) -> std::io::Result<Instruction> {
    claim_position_fee_ix_with_program_id(crate::ID, keys, args)
}
pub fn claim_position_fee_invoke_with_program_id(
    program_id: Pubkey,
    accounts: ClaimPositionFeeAccounts<'_, '_>,
    args: ClaimPositionFeeIxArgs,
) -> ProgramResult {
    let keys: ClaimPositionFeeKeys = accounts.into();
    let ix = claim_position_fee_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn claim_position_fee_invoke(
    accounts: ClaimPositionFeeAccounts<'_, '_>,
    args: ClaimPositionFeeIxArgs,
) -> ProgramResult {
    claim_position_fee_invoke_with_program_id(crate::ID, accounts, args)
}
pub fn claim_position_fee_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: ClaimPositionFeeAccounts<'_, '_>,
    args: ClaimPositionFeeIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: ClaimPositionFeeKeys = accounts.into();
    let ix = claim_position_fee_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn claim_position_fee_invoke_signed(
    accounts: ClaimPositionFeeAccounts<'_, '_>,
    args: ClaimPositionFeeIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    claim_position_fee_invoke_signed_with_program_id(crate::ID, accounts, args, seeds)
}
pub fn claim_position_fee_verify_account_keys(
    accounts: ClaimPositionFeeAccounts<'_, '_>,
    keys: ClaimPositionFeeKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.pool_authority.key, keys.pool_authority),
        (*accounts.pool.key, keys.pool),
        (*accounts.position.key, keys.position),
        (*accounts.token_a_account.key, keys.token_a_account),
        (*accounts.token_b_account.key, keys.token_b_account),
        (*accounts.token_a_vault.key, keys.token_a_vault),
        (*accounts.token_b_vault.key, keys.token_b_vault),
        (*accounts.token_a_mint.key, keys.token_a_mint),
        (*accounts.token_b_mint.key, keys.token_b_mint),
        (*accounts.position_nft_account.key, keys.position_nft_account),
        (*accounts.owner.key, keys.owner),
        (*accounts.token_a_program.key, keys.token_a_program),
        (*accounts.token_b_program.key, keys.token_b_program),
        (*accounts.event_authority.key, keys.event_authority),
        (*accounts.program.key, keys.program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn claim_position_fee_verify_writable_privileges<'me, 'info>(
    accounts: ClaimPositionFeeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.position,
        accounts.token_a_account,
        accounts.token_b_account,
        accounts.token_a_vault,
        accounts.token_b_vault,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn claim_position_fee_verify_signer_privileges<'me, 'info>(
    accounts: ClaimPositionFeeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.owner] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn claim_position_fee_verify_account_privileges<'me, 'info>(
    accounts: ClaimPositionFeeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    claim_position_fee_verify_writable_privileges(accounts)?;
    claim_position_fee_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const CLAIM_PROTOCOL_FEE_IX_ACCOUNTS_LEN: usize = 14;
#[derive(Copy, Clone, Debug)]
pub struct ClaimProtocolFeeAccounts<'me, 'info> {
    pub pool_authority: &'me AccountInfo<'info>,
    pub pool: &'me AccountInfo<'info>,
    pub token_a_vault: &'me AccountInfo<'info>,
    pub token_b_vault: &'me AccountInfo<'info>,
    pub token_a_mint: &'me AccountInfo<'info>,
    pub token_b_mint: &'me AccountInfo<'info>,
    pub token_a_account: &'me AccountInfo<'info>,
    pub token_b_account: &'me AccountInfo<'info>,
    pub claim_fee_operator: &'me AccountInfo<'info>,
    pub operator: &'me AccountInfo<'info>,
    pub token_a_program: &'me AccountInfo<'info>,
    pub token_b_program: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct ClaimProtocolFeeKeys {
    pub pool_authority: Pubkey,
    pub pool: Pubkey,
    pub token_a_vault: Pubkey,
    pub token_b_vault: Pubkey,
    pub token_a_mint: Pubkey,
    pub token_b_mint: Pubkey,
    pub token_a_account: Pubkey,
    pub token_b_account: Pubkey,
    pub claim_fee_operator: Pubkey,
    pub operator: Pubkey,
    pub token_a_program: Pubkey,
    pub token_b_program: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<ClaimProtocolFeeAccounts<'_, '_>> for ClaimProtocolFeeKeys {
    fn from(accounts: ClaimProtocolFeeAccounts) -> Self {
        Self {
            pool_authority: *accounts.pool_authority.key,
            pool: *accounts.pool.key,
            token_a_vault: *accounts.token_a_vault.key,
            token_b_vault: *accounts.token_b_vault.key,
            token_a_mint: *accounts.token_a_mint.key,
            token_b_mint: *accounts.token_b_mint.key,
            token_a_account: *accounts.token_a_account.key,
            token_b_account: *accounts.token_b_account.key,
            claim_fee_operator: *accounts.claim_fee_operator.key,
            operator: *accounts.operator.key,
            token_a_program: *accounts.token_a_program.key,
            token_b_program: *accounts.token_b_program.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<ClaimProtocolFeeKeys> for [AccountMeta; CLAIM_PROTOCOL_FEE_IX_ACCOUNTS_LEN] {
    fn from(keys: ClaimProtocolFeeKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.pool_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pool,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_a_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_b_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_a_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_b_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_a_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_b_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.claim_fee_operator,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.operator,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_a_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_b_program,
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
impl From<[Pubkey; CLAIM_PROTOCOL_FEE_IX_ACCOUNTS_LEN]> for ClaimProtocolFeeKeys {
    fn from(pubkeys: [Pubkey; CLAIM_PROTOCOL_FEE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            pool_authority: pubkeys[0],
            pool: pubkeys[1],
            token_a_vault: pubkeys[2],
            token_b_vault: pubkeys[3],
            token_a_mint: pubkeys[4],
            token_b_mint: pubkeys[5],
            token_a_account: pubkeys[6],
            token_b_account: pubkeys[7],
            claim_fee_operator: pubkeys[8],
            operator: pubkeys[9],
            token_a_program: pubkeys[10],
            token_b_program: pubkeys[11],
            event_authority: pubkeys[12],
            program: pubkeys[13],
        }
    }
}
impl<'info> From<ClaimProtocolFeeAccounts<'_, 'info>> for [AccountInfo<'info>; CLAIM_PROTOCOL_FEE_IX_ACCOUNTS_LEN] {
    fn from(accounts: ClaimProtocolFeeAccounts<'_, 'info>) -> Self {
        [
            accounts.pool_authority.clone(),
            accounts.pool.clone(),
            accounts.token_a_vault.clone(),
            accounts.token_b_vault.clone(),
            accounts.token_a_mint.clone(),
            accounts.token_b_mint.clone(),
            accounts.token_a_account.clone(),
            accounts.token_b_account.clone(),
            accounts.claim_fee_operator.clone(),
            accounts.operator.clone(),
            accounts.token_a_program.clone(),
            accounts.token_b_program.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; CLAIM_PROTOCOL_FEE_IX_ACCOUNTS_LEN]> for ClaimProtocolFeeAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; CLAIM_PROTOCOL_FEE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            pool_authority: &arr[0],
            pool: &arr[1],
            token_a_vault: &arr[2],
            token_b_vault: &arr[3],
            token_a_mint: &arr[4],
            token_b_mint: &arr[5],
            token_a_account: &arr[6],
            token_b_account: &arr[7],
            claim_fee_operator: &arr[8],
            operator: &arr[9],
            token_a_program: &arr[10],
            token_b_program: &arr[11],
            event_authority: &arr[12],
            program: &arr[13],
        }
    }
}
pub const CLAIM_PROTOCOL_FEE_IX_DISCM: [u8; 8] = [165, 228, 133, 48, 99, 249, 255, 33];
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ClaimProtocolFeeIxArgs {}
#[derive(Clone, Debug, PartialEq)]
pub struct ClaimProtocolFeeIxData(pub ClaimProtocolFeeIxArgs);
impl From<ClaimProtocolFeeIxArgs> for ClaimProtocolFeeIxData {
    fn from(args: ClaimProtocolFeeIxArgs) -> Self {
        Self(args)
    }
}
impl ClaimProtocolFeeIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != CLAIM_PROTOCOL_FEE_IX_DISCM {
            return Err(
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!(
                        "discm does not match. Expected: {:?}. Received: {:?}",
                        CLAIM_PROTOCOL_FEE_IX_DISCM, maybe_discm
                    ),
                ),
            );
        }
        Ok(Self(ClaimProtocolFeeIxArgs::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&CLAIM_PROTOCOL_FEE_IX_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn claim_protocol_fee_ix_with_program_id(
    program_id: Pubkey,
    keys: ClaimProtocolFeeKeys,
    args: ClaimProtocolFeeIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; CLAIM_PROTOCOL_FEE_IX_ACCOUNTS_LEN] = keys.into();
    let data: ClaimProtocolFeeIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn claim_protocol_fee_ix(
    keys: ClaimProtocolFeeKeys,
    args: ClaimProtocolFeeIxArgs,
) -> std::io::Result<Instruction> {
    claim_protocol_fee_ix_with_program_id(crate::ID, keys, args)
}
pub fn claim_protocol_fee_invoke_with_program_id(
    program_id: Pubkey,
    accounts: ClaimProtocolFeeAccounts<'_, '_>,
    args: ClaimProtocolFeeIxArgs,
) -> ProgramResult {
    let keys: ClaimProtocolFeeKeys = accounts.into();
    let ix = claim_protocol_fee_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn claim_protocol_fee_invoke(
    accounts: ClaimProtocolFeeAccounts<'_, '_>,
    args: ClaimProtocolFeeIxArgs,
) -> ProgramResult {
    claim_protocol_fee_invoke_with_program_id(crate::ID, accounts, args)
}
pub fn claim_protocol_fee_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: ClaimProtocolFeeAccounts<'_, '_>,
    args: ClaimProtocolFeeIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: ClaimProtocolFeeKeys = accounts.into();
    let ix = claim_protocol_fee_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn claim_protocol_fee_invoke_signed(
    accounts: ClaimProtocolFeeAccounts<'_, '_>,
    args: ClaimProtocolFeeIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    claim_protocol_fee_invoke_signed_with_program_id(crate::ID, accounts, args, seeds)
}
pub fn claim_protocol_fee_verify_account_keys(
    accounts: ClaimProtocolFeeAccounts<'_, '_>,
    keys: ClaimProtocolFeeKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.pool_authority.key, keys.pool_authority),
        (*accounts.pool.key, keys.pool),
        (*accounts.token_a_vault.key, keys.token_a_vault),
        (*accounts.token_b_vault.key, keys.token_b_vault),
        (*accounts.token_a_mint.key, keys.token_a_mint),
        (*accounts.token_b_mint.key, keys.token_b_mint),
        (*accounts.token_a_account.key, keys.token_a_account),
        (*accounts.token_b_account.key, keys.token_b_account),
        (*accounts.claim_fee_operator.key, keys.claim_fee_operator),
        (*accounts.operator.key, keys.operator),
        (*accounts.token_a_program.key, keys.token_a_program),
        (*accounts.token_b_program.key, keys.token_b_program),
        (*accounts.event_authority.key, keys.event_authority),
        (*accounts.program.key, keys.program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn claim_protocol_fee_verify_writable_privileges<'me, 'info>(
    accounts: ClaimProtocolFeeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.pool,
        accounts.token_a_vault,
        accounts.token_b_vault,
        accounts.token_a_account,
        accounts.token_b_account,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn claim_protocol_fee_verify_signer_privileges<'me, 'info>(
    accounts: ClaimProtocolFeeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.operator] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn claim_protocol_fee_verify_account_privileges<'me, 'info>(
    accounts: ClaimProtocolFeeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    claim_protocol_fee_verify_writable_privileges(accounts)?;
    claim_protocol_fee_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const CLAIM_REWARD_IX_ACCOUNTS_LEN: usize = 11;
#[derive(Copy, Clone, Debug)]
pub struct ClaimRewardAccounts<'me, 'info> {
    pub pool_authority: &'me AccountInfo<'info>,
    pub pool: &'me AccountInfo<'info>,
    pub position: &'me AccountInfo<'info>,
    pub reward_vault: &'me AccountInfo<'info>,
    pub reward_mint: &'me AccountInfo<'info>,
    pub user_token_account: &'me AccountInfo<'info>,
    pub position_nft_account: &'me AccountInfo<'info>,
    pub owner: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct ClaimRewardKeys {
    pub pool_authority: Pubkey,
    pub pool: Pubkey,
    pub position: Pubkey,
    pub reward_vault: Pubkey,
    pub reward_mint: Pubkey,
    pub user_token_account: Pubkey,
    pub position_nft_account: Pubkey,
    pub owner: Pubkey,
    pub token_program: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<ClaimRewardAccounts<'_, '_>> for ClaimRewardKeys {
    fn from(accounts: ClaimRewardAccounts) -> Self {
        Self {
            pool_authority: *accounts.pool_authority.key,
            pool: *accounts.pool.key,
            position: *accounts.position.key,
            reward_vault: *accounts.reward_vault.key,
            reward_mint: *accounts.reward_mint.key,
            user_token_account: *accounts.user_token_account.key,
            position_nft_account: *accounts.position_nft_account.key,
            owner: *accounts.owner.key,
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
                pubkey: keys.pool_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pool,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.position,
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
                pubkey: keys.user_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.position_nft_account,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.owner,
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
impl From<[Pubkey; CLAIM_REWARD_IX_ACCOUNTS_LEN]> for ClaimRewardKeys {
    fn from(pubkeys: [Pubkey; CLAIM_REWARD_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            pool_authority: pubkeys[0],
            pool: pubkeys[1],
            position: pubkeys[2],
            reward_vault: pubkeys[3],
            reward_mint: pubkeys[4],
            user_token_account: pubkeys[5],
            position_nft_account: pubkeys[6],
            owner: pubkeys[7],
            token_program: pubkeys[8],
            event_authority: pubkeys[9],
            program: pubkeys[10],
        }
    }
}
impl<'info> From<ClaimRewardAccounts<'_, 'info>> for [AccountInfo<'info>; CLAIM_REWARD_IX_ACCOUNTS_LEN] {
    fn from(accounts: ClaimRewardAccounts<'_, 'info>) -> Self {
        [
            accounts.pool_authority.clone(),
            accounts.pool.clone(),
            accounts.position.clone(),
            accounts.reward_vault.clone(),
            accounts.reward_mint.clone(),
            accounts.user_token_account.clone(),
            accounts.position_nft_account.clone(),
            accounts.owner.clone(),
            accounts.token_program.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; CLAIM_REWARD_IX_ACCOUNTS_LEN]> for ClaimRewardAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; CLAIM_REWARD_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            pool_authority: &arr[0],
            pool: &arr[1],
            position: &arr[2],
            reward_vault: &arr[3],
            reward_mint: &arr[4],
            user_token_account: &arr[5],
            position_nft_account: &arr[6],
            owner: &arr[7],
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
    pub reward_index: u8,
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
        (*accounts.pool_authority.key, keys.pool_authority),
        (*accounts.pool.key, keys.pool),
        (*accounts.position.key, keys.position),
        (*accounts.reward_vault.key, keys.reward_vault),
        (*accounts.reward_mint.key, keys.reward_mint),
        (*accounts.user_token_account.key, keys.user_token_account),
        (*accounts.position_nft_account.key, keys.position_nft_account),
        (*accounts.owner.key, keys.owner),
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
        accounts.pool,
        accounts.position,
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
    for should_be_signer in [accounts.owner] {
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
pub const CLOSE_CLAIM_FEE_OPERATOR_IX_ACCOUNTS_LEN: usize = 5;
#[derive(Copy, Clone, Debug)]
pub struct CloseClaimFeeOperatorAccounts<'me, 'info> {
    pub claim_fee_operator: &'me AccountInfo<'info>,
    pub rent_receiver: &'me AccountInfo<'info>,
    pub admin: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CloseClaimFeeOperatorKeys {
    pub claim_fee_operator: Pubkey,
    pub rent_receiver: Pubkey,
    pub admin: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<CloseClaimFeeOperatorAccounts<'_, '_>> for CloseClaimFeeOperatorKeys {
    fn from(accounts: CloseClaimFeeOperatorAccounts) -> Self {
        Self {
            claim_fee_operator: *accounts.claim_fee_operator.key,
            rent_receiver: *accounts.rent_receiver.key,
            admin: *accounts.admin.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<CloseClaimFeeOperatorKeys> for [AccountMeta; CLOSE_CLAIM_FEE_OPERATOR_IX_ACCOUNTS_LEN] {
    fn from(keys: CloseClaimFeeOperatorKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.claim_fee_operator,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.rent_receiver,
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
impl From<[Pubkey; CLOSE_CLAIM_FEE_OPERATOR_IX_ACCOUNTS_LEN]> for CloseClaimFeeOperatorKeys {
    fn from(pubkeys: [Pubkey; CLOSE_CLAIM_FEE_OPERATOR_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            claim_fee_operator: pubkeys[0],
            rent_receiver: pubkeys[1],
            admin: pubkeys[2],
            event_authority: pubkeys[3],
            program: pubkeys[4],
        }
    }
}
impl<'info> From<CloseClaimFeeOperatorAccounts<'_, 'info>> for [AccountInfo<'info>; CLOSE_CLAIM_FEE_OPERATOR_IX_ACCOUNTS_LEN] {
    fn from(accounts: CloseClaimFeeOperatorAccounts<'_, 'info>) -> Self {
        [
            accounts.claim_fee_operator.clone(),
            accounts.rent_receiver.clone(),
            accounts.admin.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; CLOSE_CLAIM_FEE_OPERATOR_IX_ACCOUNTS_LEN]> for CloseClaimFeeOperatorAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; CLOSE_CLAIM_FEE_OPERATOR_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            claim_fee_operator: &arr[0],
            rent_receiver: &arr[1],
            admin: &arr[2],
            event_authority: &arr[3],
            program: &arr[4],
        }
    }
}
pub const CLOSE_CLAIM_FEE_OPERATOR_IX_DISCM: [u8; 8] = [38, 134, 82, 216, 95, 124, 17, 99];
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CloseClaimFeeOperatorIxArgs {}
#[derive(Clone, Debug, PartialEq)]
pub struct CloseClaimFeeOperatorIxData(pub CloseClaimFeeOperatorIxArgs);
impl From<CloseClaimFeeOperatorIxArgs> for CloseClaimFeeOperatorIxData {
    fn from(args: CloseClaimFeeOperatorIxArgs) -> Self {
        Self(args)
    }
}
impl CloseClaimFeeOperatorIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != CLOSE_CLAIM_FEE_OPERATOR_IX_DISCM {
            return Err(
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!(
                        "discm does not match. Expected: {:?}. Received: {:?}",
                        CLOSE_CLAIM_FEE_OPERATOR_IX_DISCM, maybe_discm
                    ),
                ),
            );
        }
        Ok(Self(CloseClaimFeeOperatorIxArgs::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&CLOSE_CLAIM_FEE_OPERATOR_IX_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn close_claim_fee_operator_ix_with_program_id(
    program_id: Pubkey,
    keys: CloseClaimFeeOperatorKeys,
    args: CloseClaimFeeOperatorIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; CLOSE_CLAIM_FEE_OPERATOR_IX_ACCOUNTS_LEN] = keys.into();
    let data: CloseClaimFeeOperatorIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn close_claim_fee_operator_ix(
    keys: CloseClaimFeeOperatorKeys,
    args: CloseClaimFeeOperatorIxArgs,
) -> std::io::Result<Instruction> {
    close_claim_fee_operator_ix_with_program_id(crate::ID, keys, args)
}
pub fn close_claim_fee_operator_invoke_with_program_id(
    program_id: Pubkey,
    accounts: CloseClaimFeeOperatorAccounts<'_, '_>,
    args: CloseClaimFeeOperatorIxArgs,
) -> ProgramResult {
    let keys: CloseClaimFeeOperatorKeys = accounts.into();
    let ix = close_claim_fee_operator_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn close_claim_fee_operator_invoke(
    accounts: CloseClaimFeeOperatorAccounts<'_, '_>,
    args: CloseClaimFeeOperatorIxArgs,
) -> ProgramResult {
    close_claim_fee_operator_invoke_with_program_id(crate::ID, accounts, args)
}
pub fn close_claim_fee_operator_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: CloseClaimFeeOperatorAccounts<'_, '_>,
    args: CloseClaimFeeOperatorIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: CloseClaimFeeOperatorKeys = accounts.into();
    let ix = close_claim_fee_operator_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn close_claim_fee_operator_invoke_signed(
    accounts: CloseClaimFeeOperatorAccounts<'_, '_>,
    args: CloseClaimFeeOperatorIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    close_claim_fee_operator_invoke_signed_with_program_id(crate::ID, accounts, args, seeds)
}
pub fn close_claim_fee_operator_verify_account_keys(
    accounts: CloseClaimFeeOperatorAccounts<'_, '_>,
    keys: CloseClaimFeeOperatorKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.claim_fee_operator.key, keys.claim_fee_operator),
        (*accounts.rent_receiver.key, keys.rent_receiver),
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
pub fn close_claim_fee_operator_verify_writable_privileges<'me, 'info>(
    accounts: CloseClaimFeeOperatorAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.claim_fee_operator,
        accounts.rent_receiver,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn close_claim_fee_operator_verify_signer_privileges<'me, 'info>(
    accounts: CloseClaimFeeOperatorAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.admin] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn close_claim_fee_operator_verify_account_privileges<'me, 'info>(
    accounts: CloseClaimFeeOperatorAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    close_claim_fee_operator_verify_writable_privileges(accounts)?;
    close_claim_fee_operator_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const CLOSE_CONFIG_IX_ACCOUNTS_LEN: usize = 5;
#[derive(Copy, Clone, Debug)]
pub struct CloseConfigAccounts<'me, 'info> {
    pub config: &'me AccountInfo<'info>,
    pub admin: &'me AccountInfo<'info>,
    pub rent_receiver: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CloseConfigKeys {
    pub config: Pubkey,
    pub admin: Pubkey,
    pub rent_receiver: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<CloseConfigAccounts<'_, '_>> for CloseConfigKeys {
    fn from(accounts: CloseConfigAccounts) -> Self {
        Self {
            config: *accounts.config.key,
            admin: *accounts.admin.key,
            rent_receiver: *accounts.rent_receiver.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<CloseConfigKeys> for [AccountMeta; CLOSE_CONFIG_IX_ACCOUNTS_LEN] {
    fn from(keys: CloseConfigKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.config,
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
impl From<[Pubkey; CLOSE_CONFIG_IX_ACCOUNTS_LEN]> for CloseConfigKeys {
    fn from(pubkeys: [Pubkey; CLOSE_CONFIG_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            config: pubkeys[0],
            admin: pubkeys[1],
            rent_receiver: pubkeys[2],
            event_authority: pubkeys[3],
            program: pubkeys[4],
        }
    }
}
impl<'info> From<CloseConfigAccounts<'_, 'info>> for [AccountInfo<'info>; CLOSE_CONFIG_IX_ACCOUNTS_LEN] {
    fn from(accounts: CloseConfigAccounts<'_, 'info>) -> Self {
        [
            accounts.config.clone(),
            accounts.admin.clone(),
            accounts.rent_receiver.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; CLOSE_CONFIG_IX_ACCOUNTS_LEN]> for CloseConfigAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; CLOSE_CONFIG_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            config: &arr[0],
            admin: &arr[1],
            rent_receiver: &arr[2],
            event_authority: &arr[3],
            program: &arr[4],
        }
    }
}
pub const CLOSE_CONFIG_IX_DISCM: [u8; 8] = [145, 9, 72, 157, 95, 125, 61, 85];
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CloseConfigIxArgs {}
#[derive(Clone, Debug, PartialEq)]
pub struct CloseConfigIxData(pub CloseConfigIxArgs);
impl From<CloseConfigIxArgs> for CloseConfigIxData {
    fn from(args: CloseConfigIxArgs) -> Self {
        Self(args)
    }
}
impl CloseConfigIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != CLOSE_CONFIG_IX_DISCM {
            return Err(
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!(
                        "discm does not match. Expected: {:?}. Received: {:?}",
                        CLOSE_CONFIG_IX_DISCM, maybe_discm
                    ),
                ),
            );
        }
        Ok(Self(CloseConfigIxArgs::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&CLOSE_CONFIG_IX_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn close_config_ix_with_program_id(
    program_id: Pubkey,
    keys: CloseConfigKeys,
    args: CloseConfigIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; CLOSE_CONFIG_IX_ACCOUNTS_LEN] = keys.into();
    let data: CloseConfigIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn close_config_ix(
    keys: CloseConfigKeys,
    args: CloseConfigIxArgs,
) -> std::io::Result<Instruction> {
    close_config_ix_with_program_id(crate::ID, keys, args)
}
pub fn close_config_invoke_with_program_id(
    program_id: Pubkey,
    accounts: CloseConfigAccounts<'_, '_>,
    args: CloseConfigIxArgs,
) -> ProgramResult {
    let keys: CloseConfigKeys = accounts.into();
    let ix = close_config_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn close_config_invoke(
    accounts: CloseConfigAccounts<'_, '_>,
    args: CloseConfigIxArgs,
) -> ProgramResult {
    close_config_invoke_with_program_id(crate::ID, accounts, args)
}
pub fn close_config_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: CloseConfigAccounts<'_, '_>,
    args: CloseConfigIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: CloseConfigKeys = accounts.into();
    let ix = close_config_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn close_config_invoke_signed(
    accounts: CloseConfigAccounts<'_, '_>,
    args: CloseConfigIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    close_config_invoke_signed_with_program_id(crate::ID, accounts, args, seeds)
}
pub fn close_config_verify_account_keys(
    accounts: CloseConfigAccounts<'_, '_>,
    keys: CloseConfigKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.config.key, keys.config),
        (*accounts.admin.key, keys.admin),
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
pub fn close_config_verify_writable_privileges<'me, 'info>(
    accounts: CloseConfigAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.config,
        accounts.admin,
        accounts.rent_receiver,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn close_config_verify_signer_privileges<'me, 'info>(
    accounts: CloseConfigAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.admin] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn close_config_verify_account_privileges<'me, 'info>(
    accounts: CloseConfigAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    close_config_verify_writable_privileges(accounts)?;
    close_config_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const CLOSE_POSITION_IX_ACCOUNTS_LEN: usize = 10;
#[derive(Copy, Clone, Debug)]
pub struct ClosePositionAccounts<'me, 'info> {
    pub position_nft_mint: &'me AccountInfo<'info>,
    pub position_nft_account: &'me AccountInfo<'info>,
    pub pool: &'me AccountInfo<'info>,
    pub position: &'me AccountInfo<'info>,
    pub pool_authority: &'me AccountInfo<'info>,
    pub rent_receiver: &'me AccountInfo<'info>,
    pub owner: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct ClosePositionKeys {
    pub position_nft_mint: Pubkey,
    pub position_nft_account: Pubkey,
    pub pool: Pubkey,
    pub position: Pubkey,
    pub pool_authority: Pubkey,
    pub rent_receiver: Pubkey,
    pub owner: Pubkey,
    pub token_program: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<ClosePositionAccounts<'_, '_>> for ClosePositionKeys {
    fn from(accounts: ClosePositionAccounts) -> Self {
        Self {
            position_nft_mint: *accounts.position_nft_mint.key,
            position_nft_account: *accounts.position_nft_account.key,
            pool: *accounts.pool.key,
            position: *accounts.position.key,
            pool_authority: *accounts.pool_authority.key,
            rent_receiver: *accounts.rent_receiver.key,
            owner: *accounts.owner.key,
            token_program: *accounts.token_program.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<ClosePositionKeys> for [AccountMeta; CLOSE_POSITION_IX_ACCOUNTS_LEN] {
    fn from(keys: ClosePositionKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.position_nft_mint,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.position_nft_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.pool,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.position,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.pool_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.rent_receiver,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.owner,
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
impl From<[Pubkey; CLOSE_POSITION_IX_ACCOUNTS_LEN]> for ClosePositionKeys {
    fn from(pubkeys: [Pubkey; CLOSE_POSITION_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            position_nft_mint: pubkeys[0],
            position_nft_account: pubkeys[1],
            pool: pubkeys[2],
            position: pubkeys[3],
            pool_authority: pubkeys[4],
            rent_receiver: pubkeys[5],
            owner: pubkeys[6],
            token_program: pubkeys[7],
            event_authority: pubkeys[8],
            program: pubkeys[9],
        }
    }
}
impl<'info> From<ClosePositionAccounts<'_, 'info>> for [AccountInfo<'info>; CLOSE_POSITION_IX_ACCOUNTS_LEN] {
    fn from(accounts: ClosePositionAccounts<'_, 'info>) -> Self {
        [
            accounts.position_nft_mint.clone(),
            accounts.position_nft_account.clone(),
            accounts.pool.clone(),
            accounts.position.clone(),
            accounts.pool_authority.clone(),
            accounts.rent_receiver.clone(),
            accounts.owner.clone(),
            accounts.token_program.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; CLOSE_POSITION_IX_ACCOUNTS_LEN]> for ClosePositionAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; CLOSE_POSITION_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            position_nft_mint: &arr[0],
            position_nft_account: &arr[1],
            pool: &arr[2],
            position: &arr[3],
            pool_authority: &arr[4],
            rent_receiver: &arr[5],
            owner: &arr[6],
            token_program: &arr[7],
            event_authority: &arr[8],
            program: &arr[9],
        }
    }
}
pub const CLOSE_POSITION_IX_DISCM: [u8; 8] = [123, 134, 81, 0, 49, 68, 98, 98];
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
        Ok(Self(ClosePositionIxArgs::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&CLOSE_POSITION_IX_DISCM)?;
        self.0.serialize(&mut writer)
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
    args: ClosePositionIxArgs,
) -> std::io::Result<Instruction> {
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
) -> std::io::Result<Instruction> {
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
        (*accounts.position_nft_mint.key, keys.position_nft_mint),
        (*accounts.position_nft_account.key, keys.position_nft_account),
        (*accounts.pool.key, keys.pool),
        (*accounts.position.key, keys.position),
        (*accounts.pool_authority.key, keys.pool_authority),
        (*accounts.rent_receiver.key, keys.rent_receiver),
        (*accounts.owner.key, keys.owner),
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
pub fn close_position_verify_writable_privileges<'me, 'info>(
    accounts: ClosePositionAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.position_nft_mint,
        accounts.position_nft_account,
        accounts.pool,
        accounts.position,
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
    for should_be_signer in [accounts.owner] {
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
pub const CREATE_CLAIM_FEE_OPERATOR_IX_ACCOUNTS_LEN: usize = 6;
#[derive(Copy, Clone, Debug)]
pub struct CreateClaimFeeOperatorAccounts<'me, 'info> {
    pub claim_fee_operator: &'me AccountInfo<'info>,
    pub operator: &'me AccountInfo<'info>,
    pub admin: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CreateClaimFeeOperatorKeys {
    pub claim_fee_operator: Pubkey,
    pub operator: Pubkey,
    pub admin: Pubkey,
    pub system_program: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<CreateClaimFeeOperatorAccounts<'_, '_>> for CreateClaimFeeOperatorKeys {
    fn from(accounts: CreateClaimFeeOperatorAccounts) -> Self {
        Self {
            claim_fee_operator: *accounts.claim_fee_operator.key,
            operator: *accounts.operator.key,
            admin: *accounts.admin.key,
            system_program: *accounts.system_program.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<CreateClaimFeeOperatorKeys> for [AccountMeta; CREATE_CLAIM_FEE_OPERATOR_IX_ACCOUNTS_LEN] {
    fn from(keys: CreateClaimFeeOperatorKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.claim_fee_operator,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.operator,
                is_signer: false,
                is_writable: false,
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
impl From<[Pubkey; CREATE_CLAIM_FEE_OPERATOR_IX_ACCOUNTS_LEN]> for CreateClaimFeeOperatorKeys {
    fn from(pubkeys: [Pubkey; CREATE_CLAIM_FEE_OPERATOR_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            claim_fee_operator: pubkeys[0],
            operator: pubkeys[1],
            admin: pubkeys[2],
            system_program: pubkeys[3],
            event_authority: pubkeys[4],
            program: pubkeys[5],
        }
    }
}
impl<'info> From<CreateClaimFeeOperatorAccounts<'_, 'info>> for [AccountInfo<'info>; CREATE_CLAIM_FEE_OPERATOR_IX_ACCOUNTS_LEN] {
    fn from(accounts: CreateClaimFeeOperatorAccounts<'_, 'info>) -> Self {
        [
            accounts.claim_fee_operator.clone(),
            accounts.operator.clone(),
            accounts.admin.clone(),
            accounts.system_program.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; CREATE_CLAIM_FEE_OPERATOR_IX_ACCOUNTS_LEN]> for CreateClaimFeeOperatorAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; CREATE_CLAIM_FEE_OPERATOR_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            claim_fee_operator: &arr[0],
            operator: &arr[1],
            admin: &arr[2],
            system_program: &arr[3],
            event_authority: &arr[4],
            program: &arr[5],
        }
    }
}
pub const CREATE_CLAIM_FEE_OPERATOR_IX_DISCM: [u8; 8] = [169, 62, 207, 107, 58, 187, 162, 109];
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CreateClaimFeeOperatorIxArgs {}
#[derive(Clone, Debug, PartialEq)]
pub struct CreateClaimFeeOperatorIxData(pub CreateClaimFeeOperatorIxArgs);
impl From<CreateClaimFeeOperatorIxArgs> for CreateClaimFeeOperatorIxData {
    fn from(args: CreateClaimFeeOperatorIxArgs) -> Self {
        Self(args)
    }
}
impl CreateClaimFeeOperatorIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != CREATE_CLAIM_FEE_OPERATOR_IX_DISCM {
            return Err(
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!(
                        "discm does not match. Expected: {:?}. Received: {:?}",
                        CREATE_CLAIM_FEE_OPERATOR_IX_DISCM, maybe_discm
                    ),
                ),
            );
        }
        Ok(Self(CreateClaimFeeOperatorIxArgs::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&CREATE_CLAIM_FEE_OPERATOR_IX_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn create_claim_fee_operator_ix_with_program_id(
    program_id: Pubkey,
    keys: CreateClaimFeeOperatorKeys,
    args: CreateClaimFeeOperatorIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; CREATE_CLAIM_FEE_OPERATOR_IX_ACCOUNTS_LEN] = keys.into();
    let data: CreateClaimFeeOperatorIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn create_claim_fee_operator_ix(
    keys: CreateClaimFeeOperatorKeys,
    args: CreateClaimFeeOperatorIxArgs,
) -> std::io::Result<Instruction> {
    create_claim_fee_operator_ix_with_program_id(crate::ID, keys, args)
}
pub fn create_claim_fee_operator_invoke_with_program_id(
    program_id: Pubkey,
    accounts: CreateClaimFeeOperatorAccounts<'_, '_>,
    args: CreateClaimFeeOperatorIxArgs,
) -> ProgramResult {
    let keys: CreateClaimFeeOperatorKeys = accounts.into();
    let ix = create_claim_fee_operator_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn create_claim_fee_operator_invoke(
    accounts: CreateClaimFeeOperatorAccounts<'_, '_>,
    args: CreateClaimFeeOperatorIxArgs,
) -> ProgramResult {
    create_claim_fee_operator_invoke_with_program_id(crate::ID, accounts, args)
}
pub fn create_claim_fee_operator_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: CreateClaimFeeOperatorAccounts<'_, '_>,
    args: CreateClaimFeeOperatorIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: CreateClaimFeeOperatorKeys = accounts.into();
    let ix = create_claim_fee_operator_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn create_claim_fee_operator_invoke_signed(
    accounts: CreateClaimFeeOperatorAccounts<'_, '_>,
    args: CreateClaimFeeOperatorIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    create_claim_fee_operator_invoke_signed_with_program_id(crate::ID, accounts, args, seeds)
}
pub fn create_claim_fee_operator_verify_account_keys(
    accounts: CreateClaimFeeOperatorAccounts<'_, '_>,
    keys: CreateClaimFeeOperatorKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.claim_fee_operator.key, keys.claim_fee_operator),
        (*accounts.operator.key, keys.operator),
        (*accounts.admin.key, keys.admin),
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
pub fn create_claim_fee_operator_verify_writable_privileges<'me, 'info>(
    accounts: CreateClaimFeeOperatorAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.claim_fee_operator,
        accounts.admin,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn create_claim_fee_operator_verify_signer_privileges<'me, 'info>(
    accounts: CreateClaimFeeOperatorAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.admin] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn create_claim_fee_operator_verify_account_privileges<'me, 'info>(
    accounts: CreateClaimFeeOperatorAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    create_claim_fee_operator_verify_writable_privileges(accounts)?;
    create_claim_fee_operator_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const CREATE_CONFIG_IX_ACCOUNTS_LEN: usize = 5;
#[derive(Copy, Clone, Debug)]
pub struct CreateConfigAccounts<'me, 'info> {
    pub config: &'me AccountInfo<'info>,
    pub admin: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CreateConfigKeys {
    pub config: Pubkey,
    pub admin: Pubkey,
    pub system_program: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<CreateConfigAccounts<'_, '_>> for CreateConfigKeys {
    fn from(accounts: CreateConfigAccounts) -> Self {
        Self {
            config: *accounts.config.key,
            admin: *accounts.admin.key,
            system_program: *accounts.system_program.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<CreateConfigKeys> for [AccountMeta; CREATE_CONFIG_IX_ACCOUNTS_LEN] {
    fn from(keys: CreateConfigKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.config,
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
impl From<[Pubkey; CREATE_CONFIG_IX_ACCOUNTS_LEN]> for CreateConfigKeys {
    fn from(pubkeys: [Pubkey; CREATE_CONFIG_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            config: pubkeys[0],
            admin: pubkeys[1],
            system_program: pubkeys[2],
            event_authority: pubkeys[3],
            program: pubkeys[4],
        }
    }
}
impl<'info> From<CreateConfigAccounts<'_, 'info>> for [AccountInfo<'info>; CREATE_CONFIG_IX_ACCOUNTS_LEN] {
    fn from(accounts: CreateConfigAccounts<'_, 'info>) -> Self {
        [
            accounts.config.clone(),
            accounts.admin.clone(),
            accounts.system_program.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; CREATE_CONFIG_IX_ACCOUNTS_LEN]> for CreateConfigAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; CREATE_CONFIG_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            config: &arr[0],
            admin: &arr[1],
            system_program: &arr[2],
            event_authority: &arr[3],
            program: &arr[4],
        }
    }
}
pub const CREATE_CONFIG_IX_DISCM: [u8; 8] = [201, 207, 243, 114, 75, 111, 47, 189];
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CreateConfigIxArgs {
    pub index: u64,
    pub config_parameters: StaticConfigParameters,
}
#[derive(Clone, Debug, PartialEq)]
pub struct CreateConfigIxData(pub CreateConfigIxArgs);
impl From<CreateConfigIxArgs> for CreateConfigIxData {
    fn from(args: CreateConfigIxArgs) -> Self {
        Self(args)
    }
}
impl CreateConfigIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != CREATE_CONFIG_IX_DISCM {
            return Err(
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!(
                        "discm does not match. Expected: {:?}. Received: {:?}",
                        CREATE_CONFIG_IX_DISCM, maybe_discm
                    ),
                ),
            );
        }
        Ok(Self(CreateConfigIxArgs::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&CREATE_CONFIG_IX_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn create_config_ix_with_program_id(
    program_id: Pubkey,
    keys: CreateConfigKeys,
    args: CreateConfigIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; CREATE_CONFIG_IX_ACCOUNTS_LEN] = keys.into();
    let data: CreateConfigIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn create_config_ix(
    keys: CreateConfigKeys,
    args: CreateConfigIxArgs,
) -> std::io::Result<Instruction> {
    create_config_ix_with_program_id(crate::ID, keys, args)
}
pub fn create_config_invoke_with_program_id(
    program_id: Pubkey,
    accounts: CreateConfigAccounts<'_, '_>,
    args: CreateConfigIxArgs,
) -> ProgramResult {
    let keys: CreateConfigKeys = accounts.into();
    let ix = create_config_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn create_config_invoke(
    accounts: CreateConfigAccounts<'_, '_>,
    args: CreateConfigIxArgs,
) -> ProgramResult {
    create_config_invoke_with_program_id(crate::ID, accounts, args)
}
pub fn create_config_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: CreateConfigAccounts<'_, '_>,
    args: CreateConfigIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: CreateConfigKeys = accounts.into();
    let ix = create_config_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn create_config_invoke_signed(
    accounts: CreateConfigAccounts<'_, '_>,
    args: CreateConfigIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    create_config_invoke_signed_with_program_id(crate::ID, accounts, args, seeds)
}
pub fn create_config_verify_account_keys(
    accounts: CreateConfigAccounts<'_, '_>,
    keys: CreateConfigKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.config.key, keys.config),
        (*accounts.admin.key, keys.admin),
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
pub fn create_config_verify_writable_privileges<'me, 'info>(
    accounts: CreateConfigAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.config,
        accounts.admin,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn create_config_verify_signer_privileges<'me, 'info>(
    accounts: CreateConfigAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.admin] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn create_config_verify_account_privileges<'me, 'info>(
    accounts: CreateConfigAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    create_config_verify_writable_privileges(accounts)?;
    create_config_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const CREATE_DYNAMIC_CONFIG_IX_ACCOUNTS_LEN: usize = 5;
#[derive(Copy, Clone, Debug)]
pub struct CreateDynamicConfigAccounts<'me, 'info> {
    pub config: &'me AccountInfo<'info>,
    pub admin: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CreateDynamicConfigKeys {
    pub config: Pubkey,
    pub admin: Pubkey,
    pub system_program: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<CreateDynamicConfigAccounts<'_, '_>> for CreateDynamicConfigKeys {
    fn from(accounts: CreateDynamicConfigAccounts) -> Self {
        Self {
            config: *accounts.config.key,
            admin: *accounts.admin.key,
            system_program: *accounts.system_program.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<CreateDynamicConfigKeys> for [AccountMeta; CREATE_DYNAMIC_CONFIG_IX_ACCOUNTS_LEN] {
    fn from(keys: CreateDynamicConfigKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.config,
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
impl From<[Pubkey; CREATE_DYNAMIC_CONFIG_IX_ACCOUNTS_LEN]> for CreateDynamicConfigKeys {
    fn from(pubkeys: [Pubkey; CREATE_DYNAMIC_CONFIG_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            config: pubkeys[0],
            admin: pubkeys[1],
            system_program: pubkeys[2],
            event_authority: pubkeys[3],
            program: pubkeys[4],
        }
    }
}
impl<'info> From<CreateDynamicConfigAccounts<'_, 'info>> for [AccountInfo<'info>; CREATE_DYNAMIC_CONFIG_IX_ACCOUNTS_LEN] {
    fn from(accounts: CreateDynamicConfigAccounts<'_, 'info>) -> Self {
        [
            accounts.config.clone(),
            accounts.admin.clone(),
            accounts.system_program.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; CREATE_DYNAMIC_CONFIG_IX_ACCOUNTS_LEN]> for CreateDynamicConfigAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; CREATE_DYNAMIC_CONFIG_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            config: &arr[0],
            admin: &arr[1],
            system_program: &arr[2],
            event_authority: &arr[3],
            program: &arr[4],
        }
    }
}
pub const CREATE_DYNAMIC_CONFIG_IX_DISCM: [u8; 8] = [81, 251, 122, 78, 66, 57, 208, 82];
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CreateDynamicConfigIxArgs {
    pub index: u64,
    pub config_parameters: DynamicConfigParameters,
}
#[derive(Clone, Debug, PartialEq)]
pub struct CreateDynamicConfigIxData(pub CreateDynamicConfigIxArgs);
impl From<CreateDynamicConfigIxArgs> for CreateDynamicConfigIxData {
    fn from(args: CreateDynamicConfigIxArgs) -> Self {
        Self(args)
    }
}
impl CreateDynamicConfigIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != CREATE_DYNAMIC_CONFIG_IX_DISCM {
            return Err(
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!(
                        "discm does not match. Expected: {:?}. Received: {:?}",
                        CREATE_DYNAMIC_CONFIG_IX_DISCM, maybe_discm
                    ),
                ),
            );
        }
        Ok(Self(CreateDynamicConfigIxArgs::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&CREATE_DYNAMIC_CONFIG_IX_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn create_dynamic_config_ix_with_program_id(
    program_id: Pubkey,
    keys: CreateDynamicConfigKeys,
    args: CreateDynamicConfigIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; CREATE_DYNAMIC_CONFIG_IX_ACCOUNTS_LEN] = keys.into();
    let data: CreateDynamicConfigIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn create_dynamic_config_ix(
    keys: CreateDynamicConfigKeys,
    args: CreateDynamicConfigIxArgs,
) -> std::io::Result<Instruction> {
    create_dynamic_config_ix_with_program_id(crate::ID, keys, args)
}
pub fn create_dynamic_config_invoke_with_program_id(
    program_id: Pubkey,
    accounts: CreateDynamicConfigAccounts<'_, '_>,
    args: CreateDynamicConfigIxArgs,
) -> ProgramResult {
    let keys: CreateDynamicConfigKeys = accounts.into();
    let ix = create_dynamic_config_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn create_dynamic_config_invoke(
    accounts: CreateDynamicConfigAccounts<'_, '_>,
    args: CreateDynamicConfigIxArgs,
) -> ProgramResult {
    create_dynamic_config_invoke_with_program_id(crate::ID, accounts, args)
}
pub fn create_dynamic_config_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: CreateDynamicConfigAccounts<'_, '_>,
    args: CreateDynamicConfigIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: CreateDynamicConfigKeys = accounts.into();
    let ix = create_dynamic_config_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn create_dynamic_config_invoke_signed(
    accounts: CreateDynamicConfigAccounts<'_, '_>,
    args: CreateDynamicConfigIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    create_dynamic_config_invoke_signed_with_program_id(crate::ID, accounts, args, seeds)
}
pub fn create_dynamic_config_verify_account_keys(
    accounts: CreateDynamicConfigAccounts<'_, '_>,
    keys: CreateDynamicConfigKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.config.key, keys.config),
        (*accounts.admin.key, keys.admin),
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
pub fn create_dynamic_config_verify_writable_privileges<'me, 'info>(
    accounts: CreateDynamicConfigAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.config,
        accounts.admin,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn create_dynamic_config_verify_signer_privileges<'me, 'info>(
    accounts: CreateDynamicConfigAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.admin] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn create_dynamic_config_verify_account_privileges<'me, 'info>(
    accounts: CreateDynamicConfigAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    create_dynamic_config_verify_writable_privileges(accounts)?;
    create_dynamic_config_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const CREATE_POSITION_IX_ACCOUNTS_LEN: usize = 11;
#[derive(Copy, Clone, Debug)]
pub struct CreatePositionAccounts<'me, 'info> {
    pub owner: &'me AccountInfo<'info>,
    pub position_nft_mint: &'me AccountInfo<'info>,
    pub position_nft_account: &'me AccountInfo<'info>,
    pub pool: &'me AccountInfo<'info>,
    pub position: &'me AccountInfo<'info>,
    pub pool_authority: &'me AccountInfo<'info>,
    pub payer: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CreatePositionKeys {
    pub owner: Pubkey,
    pub position_nft_mint: Pubkey,
    pub position_nft_account: Pubkey,
    pub pool: Pubkey,
    pub position: Pubkey,
    pub pool_authority: Pubkey,
    pub payer: Pubkey,
    pub token_program: Pubkey,
    pub system_program: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<CreatePositionAccounts<'_, '_>> for CreatePositionKeys {
    fn from(accounts: CreatePositionAccounts) -> Self {
        Self {
            owner: *accounts.owner.key,
            position_nft_mint: *accounts.position_nft_mint.key,
            position_nft_account: *accounts.position_nft_account.key,
            pool: *accounts.pool.key,
            position: *accounts.position.key,
            pool_authority: *accounts.pool_authority.key,
            payer: *accounts.payer.key,
            token_program: *accounts.token_program.key,
            system_program: *accounts.system_program.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<CreatePositionKeys> for [AccountMeta; CREATE_POSITION_IX_ACCOUNTS_LEN] {
    fn from(keys: CreatePositionKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.owner,
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
                pubkey: keys.pool,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.position,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.pool_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.payer,
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
impl From<[Pubkey; CREATE_POSITION_IX_ACCOUNTS_LEN]> for CreatePositionKeys {
    fn from(pubkeys: [Pubkey; CREATE_POSITION_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            owner: pubkeys[0],
            position_nft_mint: pubkeys[1],
            position_nft_account: pubkeys[2],
            pool: pubkeys[3],
            position: pubkeys[4],
            pool_authority: pubkeys[5],
            payer: pubkeys[6],
            token_program: pubkeys[7],
            system_program: pubkeys[8],
            event_authority: pubkeys[9],
            program: pubkeys[10],
        }
    }
}
impl<'info> From<CreatePositionAccounts<'_, 'info>> for [AccountInfo<'info>; CREATE_POSITION_IX_ACCOUNTS_LEN] {
    fn from(accounts: CreatePositionAccounts<'_, 'info>) -> Self {
        [
            accounts.owner.clone(),
            accounts.position_nft_mint.clone(),
            accounts.position_nft_account.clone(),
            accounts.pool.clone(),
            accounts.position.clone(),
            accounts.pool_authority.clone(),
            accounts.payer.clone(),
            accounts.token_program.clone(),
            accounts.system_program.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; CREATE_POSITION_IX_ACCOUNTS_LEN]> for CreatePositionAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; CREATE_POSITION_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            owner: &arr[0],
            position_nft_mint: &arr[1],
            position_nft_account: &arr[2],
            pool: &arr[3],
            position: &arr[4],
            pool_authority: &arr[5],
            payer: &arr[6],
            token_program: &arr[7],
            system_program: &arr[8],
            event_authority: &arr[9],
            program: &arr[10],
        }
    }
}
pub const CREATE_POSITION_IX_DISCM: [u8; 8] = [48, 215, 197, 153, 96, 203, 180, 133];
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CreatePositionIxArgs {}
#[derive(Clone, Debug, PartialEq)]
pub struct CreatePositionIxData(pub CreatePositionIxArgs);
impl From<CreatePositionIxArgs> for CreatePositionIxData {
    fn from(args: CreatePositionIxArgs) -> Self {
        Self(args)
    }
}
impl CreatePositionIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != CREATE_POSITION_IX_DISCM {
            return Err(
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!(
                        "discm does not match. Expected: {:?}. Received: {:?}",
                        CREATE_POSITION_IX_DISCM, maybe_discm
                    ),
                ),
            );
        }
        Ok(Self(CreatePositionIxArgs::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&CREATE_POSITION_IX_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn create_position_ix_with_program_id(
    program_id: Pubkey,
    keys: CreatePositionKeys,
    args: CreatePositionIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; CREATE_POSITION_IX_ACCOUNTS_LEN] = keys.into();
    let data: CreatePositionIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn create_position_ix(
    keys: CreatePositionKeys,
    args: CreatePositionIxArgs,
) -> std::io::Result<Instruction> {
    create_position_ix_with_program_id(crate::ID, keys, args)
}
pub fn create_position_invoke_with_program_id(
    program_id: Pubkey,
    accounts: CreatePositionAccounts<'_, '_>,
    args: CreatePositionIxArgs,
) -> ProgramResult {
    let keys: CreatePositionKeys = accounts.into();
    let ix = create_position_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn create_position_invoke(
    accounts: CreatePositionAccounts<'_, '_>,
    args: CreatePositionIxArgs,
) -> ProgramResult {
    create_position_invoke_with_program_id(crate::ID, accounts, args)
}
pub fn create_position_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: CreatePositionAccounts<'_, '_>,
    args: CreatePositionIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: CreatePositionKeys = accounts.into();
    let ix = create_position_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn create_position_invoke_signed(
    accounts: CreatePositionAccounts<'_, '_>,
    args: CreatePositionIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    create_position_invoke_signed_with_program_id(crate::ID, accounts, args, seeds)
}
pub fn create_position_verify_account_keys(
    accounts: CreatePositionAccounts<'_, '_>,
    keys: CreatePositionKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.owner.key, keys.owner),
        (*accounts.position_nft_mint.key, keys.position_nft_mint),
        (*accounts.position_nft_account.key, keys.position_nft_account),
        (*accounts.pool.key, keys.pool),
        (*accounts.position.key, keys.position),
        (*accounts.pool_authority.key, keys.pool_authority),
        (*accounts.payer.key, keys.payer),
        (*accounts.token_program.key, keys.token_program),
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
pub fn create_position_verify_writable_privileges<'me, 'info>(
    accounts: CreatePositionAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.position_nft_mint,
        accounts.position_nft_account,
        accounts.pool,
        accounts.position,
        accounts.payer,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn create_position_verify_signer_privileges<'me, 'info>(
    accounts: CreatePositionAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [
        accounts.position_nft_mint,
        accounts.payer,
    ] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn create_position_verify_account_privileges<'me, 'info>(
    accounts: CreatePositionAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    create_position_verify_writable_privileges(accounts)?;
    create_position_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const CREATE_TOKEN_BADGE_IX_ACCOUNTS_LEN: usize = 6;
#[derive(Copy, Clone, Debug)]
pub struct CreateTokenBadgeAccounts<'me, 'info> {
    pub token_badge: &'me AccountInfo<'info>,
    pub token_mint: &'me AccountInfo<'info>,
    pub admin: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CreateTokenBadgeKeys {
    pub token_badge: Pubkey,
    pub token_mint: Pubkey,
    pub admin: Pubkey,
    pub system_program: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<CreateTokenBadgeAccounts<'_, '_>> for CreateTokenBadgeKeys {
    fn from(accounts: CreateTokenBadgeAccounts) -> Self {
        Self {
            token_badge: *accounts.token_badge.key,
            token_mint: *accounts.token_mint.key,
            admin: *accounts.admin.key,
            system_program: *accounts.system_program.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<CreateTokenBadgeKeys> for [AccountMeta; CREATE_TOKEN_BADGE_IX_ACCOUNTS_LEN] {
    fn from(keys: CreateTokenBadgeKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.token_badge,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_mint,
                is_signer: false,
                is_writable: false,
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
impl From<[Pubkey; CREATE_TOKEN_BADGE_IX_ACCOUNTS_LEN]> for CreateTokenBadgeKeys {
    fn from(pubkeys: [Pubkey; CREATE_TOKEN_BADGE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            token_badge: pubkeys[0],
            token_mint: pubkeys[1],
            admin: pubkeys[2],
            system_program: pubkeys[3],
            event_authority: pubkeys[4],
            program: pubkeys[5],
        }
    }
}
impl<'info> From<CreateTokenBadgeAccounts<'_, 'info>> for [AccountInfo<'info>; CREATE_TOKEN_BADGE_IX_ACCOUNTS_LEN] {
    fn from(accounts: CreateTokenBadgeAccounts<'_, 'info>) -> Self {
        [
            accounts.token_badge.clone(),
            accounts.token_mint.clone(),
            accounts.admin.clone(),
            accounts.system_program.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; CREATE_TOKEN_BADGE_IX_ACCOUNTS_LEN]> for CreateTokenBadgeAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; CREATE_TOKEN_BADGE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            token_badge: &arr[0],
            token_mint: &arr[1],
            admin: &arr[2],
            system_program: &arr[3],
            event_authority: &arr[4],
            program: &arr[5],
        }
    }
}
pub const CREATE_TOKEN_BADGE_IX_DISCM: [u8; 8] = [88, 206, 0, 91, 60, 175, 151, 118];
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CreateTokenBadgeIxArgs {}
#[derive(Clone, Debug, PartialEq)]
pub struct CreateTokenBadgeIxData(pub CreateTokenBadgeIxArgs);
impl From<CreateTokenBadgeIxArgs> for CreateTokenBadgeIxData {
    fn from(args: CreateTokenBadgeIxArgs) -> Self {
        Self(args)
    }
}
impl CreateTokenBadgeIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != CREATE_TOKEN_BADGE_IX_DISCM {
            return Err(
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!(
                        "discm does not match. Expected: {:?}. Received: {:?}",
                        CREATE_TOKEN_BADGE_IX_DISCM, maybe_discm
                    ),
                ),
            );
        }
        Ok(Self(CreateTokenBadgeIxArgs::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&CREATE_TOKEN_BADGE_IX_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn create_token_badge_ix_with_program_id(
    program_id: Pubkey,
    keys: CreateTokenBadgeKeys,
    args: CreateTokenBadgeIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; CREATE_TOKEN_BADGE_IX_ACCOUNTS_LEN] = keys.into();
    let data: CreateTokenBadgeIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn create_token_badge_ix(
    keys: CreateTokenBadgeKeys,
    args: CreateTokenBadgeIxArgs,
) -> std::io::Result<Instruction> {
    create_token_badge_ix_with_program_id(crate::ID, keys, args)
}
pub fn create_token_badge_invoke_with_program_id(
    program_id: Pubkey,
    accounts: CreateTokenBadgeAccounts<'_, '_>,
    args: CreateTokenBadgeIxArgs,
) -> ProgramResult {
    let keys: CreateTokenBadgeKeys = accounts.into();
    let ix = create_token_badge_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn create_token_badge_invoke(
    accounts: CreateTokenBadgeAccounts<'_, '_>,
    args: CreateTokenBadgeIxArgs,
) -> ProgramResult {
    create_token_badge_invoke_with_program_id(crate::ID, accounts, args)
}
pub fn create_token_badge_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: CreateTokenBadgeAccounts<'_, '_>,
    args: CreateTokenBadgeIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: CreateTokenBadgeKeys = accounts.into();
    let ix = create_token_badge_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn create_token_badge_invoke_signed(
    accounts: CreateTokenBadgeAccounts<'_, '_>,
    args: CreateTokenBadgeIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    create_token_badge_invoke_signed_with_program_id(crate::ID, accounts, args, seeds)
}
pub fn create_token_badge_verify_account_keys(
    accounts: CreateTokenBadgeAccounts<'_, '_>,
    keys: CreateTokenBadgeKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.token_badge.key, keys.token_badge),
        (*accounts.token_mint.key, keys.token_mint),
        (*accounts.admin.key, keys.admin),
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
pub fn create_token_badge_verify_writable_privileges<'me, 'info>(
    accounts: CreateTokenBadgeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.token_badge,
        accounts.admin,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn create_token_badge_verify_signer_privileges<'me, 'info>(
    accounts: CreateTokenBadgeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.admin] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn create_token_badge_verify_account_privileges<'me, 'info>(
    accounts: CreateTokenBadgeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    create_token_badge_verify_writable_privileges(accounts)?;
    create_token_badge_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const FUND_REWARD_IX_ACCOUNTS_LEN: usize = 8;

#[derive(Copy, Clone, Debug)]
pub struct FundRewardAccounts<'me, 'info> {
    pub pool: &'me AccountInfo<'info>,
    pub reward_vault: &'me AccountInfo<'info>,
    pub reward_mint: &'me AccountInfo<'info>,
    pub funder_token_account: &'me AccountInfo<'info>,
    pub funder: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct FundRewardKeys {
    pub pool: Pubkey,
    pub reward_vault: Pubkey,
    pub reward_mint: Pubkey,
    pub funder_token_account: Pubkey,
    pub funder: Pubkey,
    pub token_program: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}

impl From<FundRewardAccounts<'_, '_>> for FundRewardKeys {
    fn from(accounts: FundRewardAccounts) -> Self {
        Self {
            pool: *accounts.pool.key,
            reward_vault: *accounts.reward_vault.key,
            reward_mint: *accounts.reward_mint.key,
            funder_token_account: *accounts.funder_token_account.key,
            funder: *accounts.funder.key,
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
                pubkey: keys.pool,
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
            pool: pubkeys[0],
            reward_vault: pubkeys[1],
            reward_mint: pubkeys[2],
            funder_token_account: pubkeys[3],
            funder: pubkeys[4],
            token_program: pubkeys[5],
            event_authority: pubkeys[6],
            program: pubkeys[7],
        }
    }
}

impl<'info> From<FundRewardAccounts<'_, 'info>> for [AccountInfo<'info>; FUND_REWARD_IX_ACCOUNTS_LEN] {
    fn from(accounts: FundRewardAccounts<'_, 'info>) -> Self {
        [
            accounts.pool.clone(),
            accounts.reward_vault.clone(),
            accounts.reward_mint.clone(),
            accounts.funder_token_account.clone(),
            accounts.funder.clone(),
            accounts.token_program.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}

impl<'me, 'info> From<&'me [AccountInfo<'info>; FUND_REWARD_IX_ACCOUNTS_LEN]> for FundRewardAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; FUND_REWARD_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            pool: &arr[0],
            reward_vault: &arr[1],
            reward_mint: &arr[2],
            funder_token_account: &arr[3],
            funder: &arr[4],
            token_program: &arr[5],
            event_authority: &arr[6],
            program: &arr[7],
        }
    }
}

pub const FUND_REWARD_IX_DISCM: [u8; 8] = [188, 50, 249, 165, 93, 151, 38, 63];

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FundRewardIxArgs {
    pub reward_index: u8,
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
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    FUND_REWARD_IX_DISCM, maybe_discm
                ),
            ));
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
        (*accounts.pool.key, keys.pool),
        (*accounts.reward_vault.key, keys.reward_vault),
        (*accounts.reward_mint.key, keys.reward_mint),
        (*accounts.funder_token_account.key, keys.funder_token_account),
        (*accounts.funder.key, keys.funder),
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
        accounts.pool,
        accounts.reward_vault,
        accounts.funder_token_account,
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

pub const INITIALIZE_CUSTOMIZABLE_POOL_IX_ACCOUNTS_LEN: usize = 19;

#[derive(Copy, Clone, Debug)]
pub struct InitializeCustomizablePoolAccounts<'me, 'info> {
    pub creator: &'me AccountInfo<'info>,
    pub position_nft_mint: &'me AccountInfo<'info>,
    pub position_nft_account: &'me AccountInfo<'info>,
    pub payer: &'me AccountInfo<'info>,
    pub pool_authority: &'me AccountInfo<'info>,
    pub pool: &'me AccountInfo<'info>,
    pub position: &'me AccountInfo<'info>,
    pub token_a_mint: &'me AccountInfo<'info>,
    pub token_b_mint: &'me AccountInfo<'info>,
    pub token_a_vault: &'me AccountInfo<'info>,
    pub token_b_vault: &'me AccountInfo<'info>,
    pub payer_token_a: &'me AccountInfo<'info>,
    pub payer_token_b: &'me AccountInfo<'info>,
    pub token_a_program: &'me AccountInfo<'info>,
    pub token_b_program: &'me AccountInfo<'info>,
    pub token_2022_program: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct InitializeCustomizablePoolKeys {
    pub creator: Pubkey,
    pub position_nft_mint: Pubkey,
    pub position_nft_account: Pubkey,
    pub payer: Pubkey,
    pub pool_authority: Pubkey,
    pub pool: Pubkey,
    pub position: Pubkey,
    pub token_a_mint: Pubkey,
    pub token_b_mint: Pubkey,
    pub token_a_vault: Pubkey,
    pub token_b_vault: Pubkey,
    pub payer_token_a: Pubkey,
    pub payer_token_b: Pubkey,
    pub token_a_program: Pubkey,
    pub token_b_program: Pubkey,
    pub token_2022_program: Pubkey,
    pub system_program: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}

impl From<InitializeCustomizablePoolAccounts<'_, '_>> for InitializeCustomizablePoolKeys {
    fn from(accounts: InitializeCustomizablePoolAccounts) -> Self {
        Self {
            creator: *accounts.creator.key,
            position_nft_mint: *accounts.position_nft_mint.key,
            position_nft_account: *accounts.position_nft_account.key,
            payer: *accounts.payer.key,
            pool_authority: *accounts.pool_authority.key,
            pool: *accounts.pool.key,
            position: *accounts.position.key,
            token_a_mint: *accounts.token_a_mint.key,
            token_b_mint: *accounts.token_b_mint.key,
            token_a_vault: *accounts.token_a_vault.key,
            token_b_vault: *accounts.token_b_vault.key,
            payer_token_a: *accounts.payer_token_a.key,
            payer_token_b: *accounts.payer_token_b.key,
            token_a_program: *accounts.token_a_program.key,
            token_b_program: *accounts.token_b_program.key,
            token_2022_program: *accounts.token_2022_program.key,
            system_program: *accounts.system_program.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}

impl From<InitializeCustomizablePoolKeys> for [AccountMeta; INITIALIZE_CUSTOMIZABLE_POOL_IX_ACCOUNTS_LEN] {
    fn from(keys: InitializeCustomizablePoolKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.creator,
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
                pubkey: keys.payer,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.pool_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pool,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.position,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_a_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_b_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_a_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_b_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.payer_token_a,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.payer_token_b,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_a_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_b_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_2022_program,
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

impl From<[Pubkey; INITIALIZE_CUSTOMIZABLE_POOL_IX_ACCOUNTS_LEN]> for InitializeCustomizablePoolKeys {
    fn from(pubkeys: [Pubkey; INITIALIZE_CUSTOMIZABLE_POOL_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            creator: pubkeys[0],
            position_nft_mint: pubkeys[1],
            position_nft_account: pubkeys[2],
            payer: pubkeys[3],
            pool_authority: pubkeys[4],
            pool: pubkeys[5],
            position: pubkeys[6],
            token_a_mint: pubkeys[7],
            token_b_mint: pubkeys[8],
            token_a_vault: pubkeys[9],
            token_b_vault: pubkeys[10],
            payer_token_a: pubkeys[11],
            payer_token_b: pubkeys[12],
            token_a_program: pubkeys[13],
            token_b_program: pubkeys[14],
            token_2022_program: pubkeys[15],
            system_program: pubkeys[16],
            event_authority: pubkeys[17],
            program: pubkeys[18],
        }
    }
}

impl<'info> From<InitializeCustomizablePoolAccounts<'_, 'info>> for [AccountInfo<'info>; INITIALIZE_CUSTOMIZABLE_POOL_IX_ACCOUNTS_LEN] {
    fn from(accounts: InitializeCustomizablePoolAccounts<'_, 'info>) -> Self {
        [
            accounts.creator.clone(),
            accounts.position_nft_mint.clone(),
            accounts.position_nft_account.clone(),
            accounts.payer.clone(),
            accounts.pool_authority.clone(),
            accounts.pool.clone(),
            accounts.position.clone(),
            accounts.token_a_mint.clone(),
            accounts.token_b_mint.clone(),
            accounts.token_a_vault.clone(),
            accounts.token_b_vault.clone(),
            accounts.payer_token_a.clone(),
            accounts.payer_token_b.clone(),
            accounts.token_a_program.clone(),
            accounts.token_b_program.clone(),
            accounts.token_2022_program.clone(),
            accounts.system_program.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}

impl<'me, 'info> From<&'me [AccountInfo<'info>; INITIALIZE_CUSTOMIZABLE_POOL_IX_ACCOUNTS_LEN]> for InitializeCustomizablePoolAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; INITIALIZE_CUSTOMIZABLE_POOL_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            creator: &arr[0],
            position_nft_mint: &arr[1],
            position_nft_account: &arr[2],
            payer: &arr[3],
            pool_authority: &arr[4],
            pool: &arr[5],
            position: &arr[6],
            token_a_mint: &arr[7],
            token_b_mint: &arr[8],
            token_a_vault: &arr[9],
            token_b_vault: &arr[10],
            payer_token_a: &arr[11],
            payer_token_b: &arr[12],
            token_a_program: &arr[13],
            token_b_program: &arr[14],
            token_2022_program: &arr[15],
            system_program: &arr[16],
            event_authority: &arr[17],
            program: &arr[18],
        }
    }
}

pub const INITIALIZE_CUSTOMIZABLE_POOL_IX_DISCM: [u8; 8] = [20, 161, 241, 24, 189, 221, 180, 2];

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq,Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InitializeCustomizablePoolIxArgs {
    pub params: InitializeCustomizablePoolParameters, // Placeholder for the actual struct
}

#[derive(Clone, Debug, PartialEq)]
pub struct InitializeCustomizablePoolIxData(pub InitializeCustomizablePoolIxArgs);

impl From<InitializeCustomizablePoolIxArgs> for InitializeCustomizablePoolIxData {
    fn from(args: InitializeCustomizablePoolIxArgs) -> Self {
        Self(args)
    }
}

impl InitializeCustomizablePoolIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != INITIALIZE_CUSTOMIZABLE_POOL_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    INITIALIZE_CUSTOMIZABLE_POOL_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(InitializeCustomizablePoolIxArgs::deserialize(&mut reader)?))
    }

    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&INITIALIZE_CUSTOMIZABLE_POOL_IX_DISCM)?;
        self.0.serialize(&mut writer)
    }

    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}

pub fn initialize_customizable_pool_ix_with_program_id(
    program_id: Pubkey,
    keys: InitializeCustomizablePoolKeys,
    args: InitializeCustomizablePoolIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; INITIALIZE_CUSTOMIZABLE_POOL_IX_ACCOUNTS_LEN] = keys.into();
    let data: InitializeCustomizablePoolIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}

pub fn initialize_customizable_pool_ix(
    keys: InitializeCustomizablePoolKeys,
    args: InitializeCustomizablePoolIxArgs,
) -> std::io::Result<Instruction> {
    initialize_customizable_pool_ix_with_program_id(crate::ID, keys, args)
}

pub fn initialize_customizable_pool_invoke_with_program_id(
    program_id: Pubkey,
    accounts: InitializeCustomizablePoolAccounts<'_, '_>,
    args: InitializeCustomizablePoolIxArgs,
) -> ProgramResult {
    let keys: InitializeCustomizablePoolKeys = accounts.into();
    let ix = initialize_customizable_pool_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}

pub fn initialize_customizable_pool_invoke(
    accounts: InitializeCustomizablePoolAccounts<'_, '_>,
    args: InitializeCustomizablePoolIxArgs,
) -> ProgramResult {
    initialize_customizable_pool_invoke_with_program_id(crate::ID, accounts, args)
}

pub fn initialize_customizable_pool_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: InitializeCustomizablePoolAccounts<'_, '_>,
    args: InitializeCustomizablePoolIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: InitializeCustomizablePoolKeys = accounts.into();
    let ix = initialize_customizable_pool_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}

pub fn initialize_customizable_pool_invoke_signed(
    accounts: InitializeCustomizablePoolAccounts<'_, '_>,
    args: InitializeCustomizablePoolIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    initialize_customizable_pool_invoke_signed_with_program_id(crate::ID, accounts, args, seeds)
}

pub fn initialize_customizable_pool_verify_account_keys(
    accounts: InitializeCustomizablePoolAccounts<'_, '_>,
    keys: InitializeCustomizablePoolKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.creator.key, keys.creator),
        (*accounts.position_nft_mint.key, keys.position_nft_mint),
        (*accounts.position_nft_account.key, keys.position_nft_account),
        (*accounts.payer.key, keys.payer),
        (*accounts.pool_authority.key, keys.pool_authority),
        (*accounts.pool.key, keys.pool),
        (*accounts.position.key, keys.position),
        (*accounts.token_a_mint.key, keys.token_a_mint),
        (*accounts.token_b_mint.key, keys.token_b_mint),
        (*accounts.token_a_vault.key, keys.token_a_vault),
        (*accounts.token_b_vault.key, keys.token_b_vault),
        (*accounts.payer_token_a.key, keys.payer_token_a),
        (*accounts.payer_token_b.key, keys.payer_token_b),
        (*accounts.token_a_program.key, keys.token_a_program),
        (*accounts.token_b_program.key, keys.token_b_program),
        (*accounts.token_2022_program.key, keys.token_2022_program),
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

pub fn initialize_customizable_pool_verify_writable_privileges<'me, 'info>(
    accounts: InitializeCustomizablePoolAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.position_nft_mint,
        accounts.position_nft_account,
        accounts.payer,
        accounts.pool,
        accounts.position,
        accounts.token_a_vault,
        accounts.token_b_vault,
        accounts.payer_token_a,
        accounts.payer_token_b,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}

pub fn initialize_customizable_pool_verify_signer_privileges<'me, 'info>(
    accounts: InitializeCustomizablePoolAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.position_nft_mint, accounts.payer] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}

pub fn initialize_customizable_pool_verify_account_privileges<'me, 'info>(
    accounts: InitializeCustomizablePoolAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    initialize_customizable_pool_verify_writable_privileges(accounts)?;
    initialize_customizable_pool_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const INITIALIZE_POOL_IX_ACCOUNTS_LEN: usize = 20;

#[derive(Copy, Clone, Debug)]
pub struct InitializePoolAccounts<'me, 'info> {
    pub creator: &'me AccountInfo<'info>,
    pub position_nft_mint: &'me AccountInfo<'info>,
    pub position_nft_account: &'me AccountInfo<'info>,
    pub payer: &'me AccountInfo<'info>,
    pub config: &'me AccountInfo<'info>,
    pub pool_authority: &'me AccountInfo<'info>,
    pub pool: &'me AccountInfo<'info>,
    pub position: &'me AccountInfo<'info>,
    pub token_a_mint: &'me AccountInfo<'info>,
    pub token_b_mint: &'me AccountInfo<'info>,
    pub token_a_vault: &'me AccountInfo<'info>,
    pub token_b_vault: &'me AccountInfo<'info>,
    pub payer_token_a: &'me AccountInfo<'info>,
    pub payer_token_b: &'me AccountInfo<'info>,
    pub token_a_program: &'me AccountInfo<'info>,
    pub token_b_program: &'me AccountInfo<'info>,
    pub token_2022_program: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct InitializePoolKeys {
    pub creator: Pubkey,
    pub position_nft_mint: Pubkey,
    pub position_nft_account: Pubkey,
    pub payer: Pubkey,
    pub config: Pubkey,
    pub pool_authority: Pubkey,
    pub pool: Pubkey,
    pub position: Pubkey,
    pub token_a_mint: Pubkey,
    pub token_b_mint: Pubkey,
    pub token_a_vault: Pubkey,
    pub token_b_vault: Pubkey,
    pub payer_token_a: Pubkey,
    pub payer_token_b: Pubkey,
    pub token_a_program: Pubkey,
    pub token_b_program: Pubkey,
    pub token_2022_program: Pubkey,
    pub system_program: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}

impl From<InitializePoolAccounts<'_, '_>> for InitializePoolKeys {
    fn from(accounts: InitializePoolAccounts) -> Self {
        Self {
            creator: *accounts.creator.key,
            position_nft_mint: *accounts.position_nft_mint.key,
            position_nft_account: *accounts.position_nft_account.key,
            payer: *accounts.payer.key,
            config: *accounts.config.key,
            pool_authority: *accounts.pool_authority.key,
            pool: *accounts.pool.key,
            position: *accounts.position.key,
            token_a_mint: *accounts.token_a_mint.key,
            token_b_mint: *accounts.token_b_mint.key,
            token_a_vault: *accounts.token_a_vault.key,
            token_b_vault: *accounts.token_b_vault.key,
            payer_token_a: *accounts.payer_token_a.key,
            payer_token_b: *accounts.payer_token_b.key,
            token_a_program: *accounts.token_a_program.key,
            token_b_program: *accounts.token_b_program.key,
            token_2022_program: *accounts.token_2022_program.key,
            system_program: *accounts.system_program.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}

impl From<InitializePoolKeys> for [AccountMeta; INITIALIZE_POOL_IX_ACCOUNTS_LEN] {
    fn from(keys: InitializePoolKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.creator,
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
                pubkey: keys.payer,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pool_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pool,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.position,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_a_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_b_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_a_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_b_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.payer_token_a,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.payer_token_b,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_a_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_b_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_2022_program,
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

impl From<[Pubkey; INITIALIZE_POOL_IX_ACCOUNTS_LEN]> for InitializePoolKeys {
    fn from(pubkeys: [Pubkey; INITIALIZE_POOL_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            creator: pubkeys[0],
            position_nft_mint: pubkeys[1],
            position_nft_account: pubkeys[2],
            payer: pubkeys[3],
            config: pubkeys[4],
            pool_authority: pubkeys[5],
            pool: pubkeys[6],
            position: pubkeys[7],
            token_a_mint: pubkeys[8],
            token_b_mint: pubkeys[9],
            token_a_vault: pubkeys[10],
            token_b_vault: pubkeys[11],
            payer_token_a: pubkeys[12],
            payer_token_b: pubkeys[13],
            token_a_program: pubkeys[14],
            token_b_program: pubkeys[15],
            token_2022_program: pubkeys[16],
            system_program: pubkeys[17],
            event_authority: pubkeys[18],
            program: pubkeys[19],
        }
    }
}

impl<'info> From<InitializePoolAccounts<'_, 'info>> for [AccountInfo<'info>; INITIALIZE_POOL_IX_ACCOUNTS_LEN] {
    fn from(accounts: InitializePoolAccounts<'_, 'info>) -> Self {
        [
            accounts.creator.clone(),
            accounts.position_nft_mint.clone(),
            accounts.position_nft_account.clone(),
            accounts.payer.clone(),
            accounts.config.clone(),
            accounts.pool_authority.clone(),
            accounts.pool.clone(),
            accounts.position.clone(),
            accounts.token_a_mint.clone(),
            accounts.token_b_mint.clone(),
            accounts.token_a_vault.clone(),
            accounts.token_b_vault.clone(),
            accounts.payer_token_a.clone(),
            accounts.payer_token_b.clone(),
            accounts.token_a_program.clone(),
            accounts.token_b_program.clone(),
            accounts.token_2022_program.clone(),
            accounts.system_program.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}

impl<'me, 'info> From<&'me [AccountInfo<'info>; INITIALIZE_POOL_IX_ACCOUNTS_LEN]> for InitializePoolAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; INITIALIZE_POOL_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            creator: &arr[0],
            position_nft_mint: &arr[1],
            position_nft_account: &arr[2],
            payer: &arr[3],
            config: &arr[4],
            pool_authority: &arr[5],
            pool: &arr[6],
            position: &arr[7],
            token_a_mint: &arr[8],
            token_b_mint: &arr[9],
            token_a_vault: &arr[10],
            token_b_vault: &arr[11],
            payer_token_a: &arr[12],
            payer_token_b: &arr[13],
            token_a_program: &arr[14],
            token_b_program: &arr[15],
            token_2022_program: &arr[16],
            system_program: &arr[17],
            event_authority: &arr[18],
            program: &arr[19],
        }
    }
}

pub const INITIALIZE_POOL_IX_DISCM: [u8; 8] = [95, 180, 10, 172, 84, 174, 232, 40];

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InitializePoolIxArgs {
    pub params: InitializePoolParameters,
}


#[derive(Clone, Debug, PartialEq)]
pub struct InitializePoolIxData(pub InitializePoolIxArgs);

impl From<InitializePoolIxArgs> for InitializePoolIxData {
    fn from(args: InitializePoolIxArgs) -> Self {
        Self(args)
    }
}

impl InitializePoolIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != INITIALIZE_POOL_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    INITIALIZE_POOL_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(InitializePoolIxArgs::deserialize(&mut reader)?))
    }

    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&INITIALIZE_POOL_IX_DISCM)?;
        self.0.serialize(&mut writer)
    }

    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}

pub fn initialize_pool_ix_with_program_id(
    program_id: Pubkey,
    keys: InitializePoolKeys,
    args: InitializePoolIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; INITIALIZE_POOL_IX_ACCOUNTS_LEN] = keys.into();
    let data: InitializePoolIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}

pub fn initialize_pool_ix(
    keys: InitializePoolKeys,
    args: InitializePoolIxArgs,
) -> std::io::Result<Instruction> {
    initialize_pool_ix_with_program_id(crate::ID, keys, args)
}

pub fn initialize_pool_invoke_with_program_id(
    program_id: Pubkey,
    accounts: InitializePoolAccounts<'_, '_>,
    args: InitializePoolIxArgs,
) -> ProgramResult {
    let keys: InitializePoolKeys = accounts.into();
    let ix = initialize_pool_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}

pub fn initialize_pool_invoke(
    accounts: InitializePoolAccounts<'_, '_>,
    args: InitializePoolIxArgs,
) -> ProgramResult {
    initialize_pool_invoke_with_program_id(crate::ID, accounts, args)
}

pub fn initialize_pool_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: InitializePoolAccounts<'_, '_>,
    args: InitializePoolIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: InitializePoolKeys = accounts.into();
    let ix = initialize_pool_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}

pub fn initialize_pool_invoke_signed(
    accounts: InitializePoolAccounts<'_, '_>,
    args: InitializePoolIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    initialize_pool_invoke_signed_with_program_id(crate::ID, accounts, args, seeds)
}

pub fn initialize_pool_verify_account_keys(
    accounts: InitializePoolAccounts<'_, '_>,
    keys: InitializePoolKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.creator.key, keys.creator),
        (*accounts.position_nft_mint.key, keys.position_nft_mint),
        (*accounts.position_nft_account.key, keys.position_nft_account),
        (*accounts.payer.key, keys.payer),
        (*accounts.config.key, keys.config),
        (*accounts.pool_authority.key, keys.pool_authority),
        (*accounts.pool.key, keys.pool),
        (*accounts.position.key, keys.position),
        (*accounts.token_a_mint.key, keys.token_a_mint),
        (*accounts.token_b_mint.key, keys.token_b_mint),
        (*accounts.token_a_vault.key, keys.token_a_vault),
        (*accounts.token_b_vault.key, keys.token_b_vault),
        (*accounts.payer_token_a.key, keys.payer_token_a),
        (*accounts.payer_token_b.key, keys.payer_token_b),
        (*accounts.token_a_program.key, keys.token_a_program),
        (*accounts.token_b_program.key, keys.token_b_program),
        (*accounts.token_2022_program.key, keys.token_2022_program),
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

pub fn initialize_pool_verify_writable_privileges<'me, 'info>(
    accounts: InitializePoolAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.position_nft_mint,
        accounts.position_nft_account,
        accounts.payer,
        accounts.pool,
        accounts.position,
        accounts.token_a_vault,
        accounts.token_b_vault,
        accounts.payer_token_a,
        accounts.payer_token_b,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}

pub fn initialize_pool_verify_signer_privileges<'me, 'info>(
    accounts: InitializePoolAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.position_nft_mint, accounts.payer] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}

pub fn initialize_pool_verify_account_privileges<'me, 'info>(
    accounts: InitializePoolAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    initialize_pool_verify_writable_privileges(accounts)?;
    initialize_pool_verify_signer_privileges(accounts)?;
    Ok(())
}

pub const INITIALIZE_POOL_WITH_DYNAMIC_CONFIG_IX_ACCOUNTS_LEN: usize = 21;

#[derive(Copy, Clone, Debug)]
pub struct InitializePoolWithDynamicConfigAccounts<'me, 'info> {
    pub creator: &'me AccountInfo<'info>,
    pub position_nft_mint: &'me AccountInfo<'info>,
    pub position_nft_account: &'me AccountInfo<'info>,
    pub payer: &'me AccountInfo<'info>,
    pub pool_creator_authority: &'me AccountInfo<'info>,
    pub config: &'me AccountInfo<'info>,
    pub pool_authority: &'me AccountInfo<'info>,
    pub pool: &'me AccountInfo<'info>,
    pub position: &'me AccountInfo<'info>,
    pub token_a_mint: &'me AccountInfo<'info>,
    pub token_b_mint: &'me AccountInfo<'info>,
    pub token_a_vault: &'me AccountInfo<'info>,
    pub token_b_vault: &'me AccountInfo<'info>,
    pub payer_token_a: &'me AccountInfo<'info>,
    pub payer_token_b: &'me AccountInfo<'info>,
    pub token_a_program: &'me AccountInfo<'info>,
    pub token_b_program: &'me AccountInfo<'info>,
    pub token_2022_program: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct InitializePoolWithDynamicConfigKeys {
    pub creator: Pubkey,
    pub position_nft_mint: Pubkey,
    pub position_nft_account: Pubkey,
    pub payer: Pubkey,
    pub pool_creator_authority: Pubkey,
    pub config: Pubkey,
    pub pool_authority: Pubkey,
    pub pool: Pubkey,
    pub position: Pubkey,
    pub token_a_mint: Pubkey,
    pub token_b_mint: Pubkey,
    pub token_a_vault: Pubkey,
    pub token_b_vault: Pubkey,
    pub payer_token_a: Pubkey,
    pub payer_token_b: Pubkey,
    pub token_a_program: Pubkey,
    pub token_b_program: Pubkey,
    pub token_2022_program: Pubkey,
    pub system_program: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}

impl From<InitializePoolWithDynamicConfigAccounts<'_, '_>> for InitializePoolWithDynamicConfigKeys {
    fn from(accounts: InitializePoolWithDynamicConfigAccounts) -> Self {
        Self {
            creator: *accounts.creator.key,
            position_nft_mint: *accounts.position_nft_mint.key,
            position_nft_account: *accounts.position_nft_account.key,
            payer: *accounts.payer.key,
            pool_creator_authority: *accounts.pool_creator_authority.key,
            config: *accounts.config.key,
            pool_authority: *accounts.pool_authority.key,
            pool: *accounts.pool.key,
            position: *accounts.position.key,
            token_a_mint: *accounts.token_a_mint.key,
            token_b_mint: *accounts.token_b_mint.key,
            token_a_vault: *accounts.token_a_vault.key,
            token_b_vault: *accounts.token_b_vault.key,
            payer_token_a: *accounts.payer_token_a.key,
            payer_token_b: *accounts.payer_token_b.key,
            token_a_program: *accounts.token_a_program.key,
            token_b_program: *accounts.token_b_program.key,
            token_2022_program: *accounts.token_2022_program.key,
            system_program: *accounts.system_program.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}

impl From<InitializePoolWithDynamicConfigKeys> for [AccountMeta; INITIALIZE_POOL_WITH_DYNAMIC_CONFIG_IX_ACCOUNTS_LEN] {
    fn from(keys: InitializePoolWithDynamicConfigKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.creator,
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
                pubkey: keys.payer,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.pool_creator_authority,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pool_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pool,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.position,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_a_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_b_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_a_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_b_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.payer_token_a,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.payer_token_b,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_a_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_b_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_2022_program,
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

impl From<[Pubkey; INITIALIZE_POOL_WITH_DYNAMIC_CONFIG_IX_ACCOUNTS_LEN]> for InitializePoolWithDynamicConfigKeys {
    fn from(pubkeys: [Pubkey; INITIALIZE_POOL_WITH_DYNAMIC_CONFIG_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            creator: pubkeys[0],
            position_nft_mint: pubkeys[1],
            position_nft_account: pubkeys[2],
            payer: pubkeys[3],
            pool_creator_authority: pubkeys[4],
            config: pubkeys[5],
            pool_authority: pubkeys[6],
            pool: pubkeys[7],
            position: pubkeys[8],
            token_a_mint: pubkeys[9],
            token_b_mint: pubkeys[10],
            token_a_vault: pubkeys[11],
            token_b_vault: pubkeys[12],
            payer_token_a: pubkeys[13],
            payer_token_b: pubkeys[14],
            token_a_program: pubkeys[15],
            token_b_program: pubkeys[16],
            token_2022_program: pubkeys[17],
            system_program: pubkeys[18],
            event_authority: pubkeys[19],
            program: pubkeys[20],
        }
    }
}

impl<'info> From<InitializePoolWithDynamicConfigAccounts<'_, 'info>> for [AccountInfo<'info>; INITIALIZE_POOL_WITH_DYNAMIC_CONFIG_IX_ACCOUNTS_LEN] {
    fn from(accounts: InitializePoolWithDynamicConfigAccounts<'_, 'info>) -> Self {
        [
            accounts.creator.clone(),
            accounts.position_nft_mint.clone(),
            accounts.position_nft_account.clone(),
            accounts.payer.clone(),
            accounts.pool_creator_authority.clone(),
            accounts.config.clone(),
            accounts.pool_authority.clone(),
            accounts.pool.clone(),
            accounts.position.clone(),
            accounts.token_a_mint.clone(),
            accounts.token_b_mint.clone(),
            accounts.token_a_vault.clone(),
            accounts.token_b_vault.clone(),
            accounts.payer_token_a.clone(),
            accounts.payer_token_b.clone(),
            accounts.token_a_program.clone(),
            accounts.token_b_program.clone(),
            accounts.token_2022_program.clone(),
            accounts.system_program.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}

impl<'me, 'info> From<&'me [AccountInfo<'info>; INITIALIZE_POOL_WITH_DYNAMIC_CONFIG_IX_ACCOUNTS_LEN]> for InitializePoolWithDynamicConfigAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; INITIALIZE_POOL_WITH_DYNAMIC_CONFIG_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            creator: &arr[0],
            position_nft_mint: &arr[1],
            position_nft_account: &arr[2],
            payer: &arr[3],
            pool_creator_authority: &arr[4],
            config: &arr[5],
            pool_authority: &arr[6],
            pool: &arr[7],
            position: &arr[8],
            token_a_mint: &arr[9],
            token_b_mint: &arr[10],
            token_a_vault: &arr[11],
            token_b_vault: &arr[12],
            payer_token_a: &arr[13],
            payer_token_b: &arr[14],
            token_a_program: &arr[15],
            token_b_program: &arr[16],
            token_2022_program: &arr[17],
            system_program: &arr[18],
            event_authority: &arr[19],
            program: &arr[20],
        }
    }
}

pub const INITIALIZE_POOL_WITH_DYNAMIC_CONFIG_IX_DISCM: [u8; 8] = [149, 82, 72, 197, 253, 252, 68, 15];

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InitializePoolWithDynamicConfigIxArgs {
    pub params: InitializeCustomizablePoolParameters,
}

#[derive(Clone, Debug, PartialEq)]
pub struct InitializePoolWithDynamicConfigIxData(pub InitializePoolWithDynamicConfigIxArgs);

impl From<InitializePoolWithDynamicConfigIxArgs> for InitializePoolWithDynamicConfigIxData {
    fn from(args: InitializePoolWithDynamicConfigIxArgs) -> Self {
        Self(args)
    }
}

impl InitializePoolWithDynamicConfigIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != INITIALIZE_POOL_WITH_DYNAMIC_CONFIG_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    INITIALIZE_POOL_WITH_DYNAMIC_CONFIG_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(InitializePoolWithDynamicConfigIxArgs::deserialize(&mut reader)?))
    }

    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&INITIALIZE_POOL_WITH_DYNAMIC_CONFIG_IX_DISCM)?;
        self.0.serialize(&mut writer)
    }

    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}

pub fn initialize_pool_with_dynamic_config_ix_with_program_id(
    program_id: Pubkey,
    keys: InitializePoolWithDynamicConfigKeys,
    args: InitializePoolWithDynamicConfigIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; INITIALIZE_POOL_WITH_DYNAMIC_CONFIG_IX_ACCOUNTS_LEN] = keys.into();
    let data: InitializePoolWithDynamicConfigIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}

pub fn initialize_pool_with_dynamic_config_ix(
    keys: InitializePoolWithDynamicConfigKeys,
    args: InitializePoolWithDynamicConfigIxArgs,
) -> std::io::Result<Instruction> {
    initialize_pool_with_dynamic_config_ix_with_program_id(crate::ID, keys, args)
}

pub fn initialize_pool_with_dynamic_config_invoke_with_program_id(
    program_id: Pubkey,
    accounts: InitializePoolWithDynamicConfigAccounts<'_, '_>,
    args: InitializePoolWithDynamicConfigIxArgs,
) -> ProgramResult {
    let keys: InitializePoolWithDynamicConfigKeys = accounts.into();
    let ix = initialize_pool_with_dynamic_config_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}

pub fn initialize_pool_with_dynamic_config_invoke(
    accounts: InitializePoolWithDynamicConfigAccounts<'_, '_>,
    args: InitializePoolWithDynamicConfigIxArgs,
) -> ProgramResult {
    initialize_pool_with_dynamic_config_invoke_with_program_id(crate::ID, accounts, args)
}

pub fn initialize_pool_with_dynamic_config_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: InitializePoolWithDynamicConfigAccounts<'_, '_>,
    args: InitializePoolWithDynamicConfigIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: InitializePoolWithDynamicConfigKeys = accounts.into();
    let ix = initialize_pool_with_dynamic_config_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}

pub fn initialize_pool_with_dynamic_config_invoke_signed(
    accounts: InitializePoolWithDynamicConfigAccounts<'_, '_>,
    args: InitializePoolWithDynamicConfigIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    initialize_pool_with_dynamic_config_invoke_signed_with_program_id(crate::ID, accounts, args, seeds)
}

pub fn initialize_pool_with_dynamic_config_verify_account_keys(
    accounts: InitializePoolWithDynamicConfigAccounts<'_, '_>,
    keys: InitializePoolWithDynamicConfigKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.creator.key, keys.creator),
        (*accounts.position_nft_mint.key, keys.position_nft_mint),
        (*accounts.position_nft_account.key, keys.position_nft_account),
        (*accounts.payer.key, keys.payer),
        (*accounts.pool_creator_authority.key, keys.pool_creator_authority),
        (*accounts.config.key, keys.config),
        (*accounts.pool_authority.key, keys.pool_authority),
        (*accounts.pool.key, keys.pool),
        (*accounts.position.key, keys.position),
        (*accounts.token_a_mint.key, keys.token_a_mint),
        (*accounts.token_b_mint.key, keys.token_b_mint),
        (*accounts.token_a_vault.key, keys.token_a_vault),
        (*accounts.token_b_vault.key, keys.token_b_vault),
        (*accounts.payer_token_a.key, keys.payer_token_a),
        (*accounts.payer_token_b.key, keys.payer_token_b),
        (*accounts.token_a_program.key, keys.token_a_program),
        (*accounts.token_b_program.key, keys.token_b_program),
        (*accounts.token_2022_program.key, keys.token_2022_program),
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

pub fn initialize_pool_with_dynamic_config_verify_writable_privileges<'me, 'info>(
    accounts: InitializePoolWithDynamicConfigAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.position_nft_mint,
        accounts.position_nft_account,
        accounts.payer,
        accounts.pool,
        accounts.position,
        accounts.token_a_vault,
        accounts.token_b_vault,
        accounts.payer_token_a,
        accounts.payer_token_b,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}

pub fn initialize_pool_with_dynamic_config_verify_signer_privileges<'me, 'info>(
    accounts: InitializePoolWithDynamicConfigAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [
        accounts.position_nft_mint,
        accounts.payer,
        accounts.pool_creator_authority,
    ] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}

pub fn initialize_pool_with_dynamic_config_verify_account_privileges<'me, 'info>(
    accounts: InitializePoolWithDynamicConfigAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    initialize_pool_with_dynamic_config_verify_writable_privileges(accounts)?;
    initialize_pool_with_dynamic_config_verify_signer_privileges(accounts)?;
    Ok(())
}

pub const INITIALIZE_REWARD_IX_ACCOUNTS_LEN: usize = 9;

#[derive(Copy, Clone, Debug)]
pub struct InitializeRewardAccounts<'me, 'info> {
    pub pool_authority: &'me AccountInfo<'info>,
    pub pool: &'me AccountInfo<'info>,
    pub reward_vault: &'me AccountInfo<'info>,
    pub reward_mint: &'me AccountInfo<'info>,
    pub admin: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct InitializeRewardKeys {
    pub pool_authority: Pubkey,
    pub pool: Pubkey,
    pub reward_vault: Pubkey,
    pub reward_mint: Pubkey,
    pub admin: Pubkey,
    pub token_program: Pubkey,
    pub system_program: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}

impl From<InitializeRewardAccounts<'_, '_>> for InitializeRewardKeys {
    fn from(accounts: InitializeRewardAccounts) -> Self {
        Self {
            pool_authority: *accounts.pool_authority.key,
            pool: *accounts.pool.key,
            reward_vault: *accounts.reward_vault.key,
            reward_mint: *accounts.reward_mint.key,
            admin: *accounts.admin.key,
            token_program: *accounts.token_program.key,
            system_program: *accounts.system_program.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}

impl From<InitializeRewardKeys> for [AccountMeta; INITIALIZE_REWARD_IX_ACCOUNTS_LEN] {
    fn from(keys: InitializeRewardKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.pool_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pool,
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
            pool_authority: pubkeys[0],
            pool: pubkeys[1],
            reward_vault: pubkeys[2],
            reward_mint: pubkeys[3],
            admin: pubkeys[4],
            token_program: pubkeys[5],
            system_program: pubkeys[6],
            event_authority: pubkeys[7],
            program: pubkeys[8],
        }
    }
}

impl<'info> From<InitializeRewardAccounts<'_, 'info>> for [AccountInfo<'info>; INITIALIZE_REWARD_IX_ACCOUNTS_LEN] {
    fn from(accounts: InitializeRewardAccounts<'_, 'info>) -> Self {
        [
            accounts.pool_authority.clone(),
            accounts.pool.clone(),
            accounts.reward_vault.clone(),
            accounts.reward_mint.clone(),
            accounts.admin.clone(),
            accounts.token_program.clone(),
            accounts.system_program.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}

impl<'me, 'info> From<&'me [AccountInfo<'info>; INITIALIZE_REWARD_IX_ACCOUNTS_LEN]> for InitializeRewardAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; INITIALIZE_REWARD_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            pool_authority: &arr[0],
            pool: &arr[1],
            reward_vault: &arr[2],
            reward_mint: &arr[3],
            admin: &arr[4],
            token_program: &arr[5],
            system_program: &arr[6],
            event_authority: &arr[7],
            program: &arr[8],
        }
    }
}

pub const INITIALIZE_REWARD_IX_DISCM: [u8; 8] = [95, 135, 192, 196, 242, 129, 230, 68];

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq,Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InitializeRewardIxArgs {
    pub reward_index: u8,
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
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    INITIALIZE_REWARD_IX_DISCM, maybe_discm
                ),
            ));
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
        (*accounts.pool_authority.key, keys.pool_authority),
        (*accounts.pool.key, keys.pool),
        (*accounts.reward_vault.key, keys.reward_vault),
        (*accounts.reward_mint.key, keys.reward_mint),
        (*accounts.admin.key, keys.admin),
        (*accounts.token_program.key, keys.token_program),
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

pub fn initialize_reward_verify_writable_privileges<'me, 'info>(
    accounts: InitializeRewardAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.pool, accounts.reward_vault, accounts.admin] {
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
pub const LOCK_POSITION_IX_ACCOUNTS_LEN: usize = 9;

#[derive(Copy, Clone, Debug)]
pub struct LockPositionAccounts<'me, 'info> {
    pub pool: &'me AccountInfo<'info>,
    pub position: &'me AccountInfo<'info>,
    pub vesting: &'me AccountInfo<'info>,
    pub position_nft_account: &'me AccountInfo<'info>,
    pub owner: &'me AccountInfo<'info>,
    pub payer: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LockPositionKeys {
    pub pool: Pubkey,
    pub position: Pubkey,
    pub vesting: Pubkey,
    pub position_nft_account: Pubkey,
    pub owner: Pubkey,
    pub payer: Pubkey,
    pub system_program: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}

impl From<LockPositionAccounts<'_, '_>> for LockPositionKeys {
    fn from(accounts: LockPositionAccounts) -> Self {
        Self {
            pool: *accounts.pool.key,
            position: *accounts.position.key,
            vesting: *accounts.vesting.key,
            position_nft_account: *accounts.position_nft_account.key,
            owner: *accounts.owner.key,
            payer: *accounts.payer.key,
            system_program: *accounts.system_program.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}

impl From<LockPositionKeys> for [AccountMeta; LOCK_POSITION_IX_ACCOUNTS_LEN] {
    fn from(keys: LockPositionKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.pool,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.position,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.vesting,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.position_nft_account,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.owner,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.payer,
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

impl From<[Pubkey; LOCK_POSITION_IX_ACCOUNTS_LEN]> for LockPositionKeys {
    fn from(pubkeys: [Pubkey; LOCK_POSITION_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            pool: pubkeys[0],
            position: pubkeys[1],
            vesting: pubkeys[2],
            position_nft_account: pubkeys[3],
            owner: pubkeys[4],
            payer: pubkeys[5],
            system_program: pubkeys[6],
            event_authority: pubkeys[7],
            program: pubkeys[8],
        }
    }
}

impl<'info> From<LockPositionAccounts<'_, 'info>> for [AccountInfo<'info>; LOCK_POSITION_IX_ACCOUNTS_LEN] {
    fn from(accounts: LockPositionAccounts<'_, 'info>) -> Self {
        [
            accounts.pool.clone(),
            accounts.position.clone(),
            accounts.vesting.clone(),
            accounts.position_nft_account.clone(),
            accounts.owner.clone(),
            accounts.payer.clone(),
            accounts.system_program.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}

impl<'me, 'info> From<&'me [AccountInfo<'info>; LOCK_POSITION_IX_ACCOUNTS_LEN]> for LockPositionAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; LOCK_POSITION_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            pool: &arr[0],
            position: &arr[1],
            vesting: &arr[2],
            position_nft_account: &arr[3],
            owner: &arr[4],
            payer: &arr[5],
            system_program: &arr[6],
            event_authority: &arr[7],
            program: &arr[8],
        }
    }
}

pub const LOCK_POSITION_IX_DISCM: [u8; 8] = [227, 62, 2, 252, 247, 10, 171, 185];

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct LockPositionIxArgs {
    pub params: VestingParameters,
}


#[derive(Clone, Debug, PartialEq)]
pub struct LockPositionIxData(pub LockPositionIxArgs);

impl From<LockPositionIxArgs> for LockPositionIxData {
    fn from(args: LockPositionIxArgs) -> Self {
        Self(args)
    }
}

impl LockPositionIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != LOCK_POSITION_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    LOCK_POSITION_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(LockPositionIxArgs::deserialize(&mut reader)?))
    }

    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&LOCK_POSITION_IX_DISCM)?;
        self.0.serialize(&mut writer)
    }

    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}

pub fn lock_position_ix_with_program_id(
    program_id: Pubkey,
    keys: LockPositionKeys,
    args: LockPositionIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; LOCK_POSITION_IX_ACCOUNTS_LEN] = keys.into();
    let data: LockPositionIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}

pub fn lock_position_ix(
    keys: LockPositionKeys,
    args: LockPositionIxArgs,
) -> std::io::Result<Instruction> {
    lock_position_ix_with_program_id(crate::ID, keys, args)
}

pub fn lock_position_invoke_with_program_id(
    program_id: Pubkey,
    accounts: LockPositionAccounts<'_, '_>,
    args: LockPositionIxArgs,
) -> ProgramResult {
    let keys: LockPositionKeys = accounts.into();
    let ix = lock_position_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}

pub fn lock_position_invoke(
    accounts: LockPositionAccounts<'_, '_>,
    args: LockPositionIxArgs,
) -> ProgramResult {
    lock_position_invoke_with_program_id(crate::ID, accounts, args)
}

pub fn lock_position_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: LockPositionAccounts<'_, '_>,
    args: LockPositionIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: LockPositionKeys = accounts.into();
    let ix = lock_position_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}

pub fn lock_position_invoke_signed(
    accounts: LockPositionAccounts<'_, '_>,
    args: LockPositionIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    lock_position_invoke_signed_with_program_id(crate::ID, accounts, args, seeds)
}

pub fn lock_position_verify_account_keys(
    accounts: LockPositionAccounts<'_, '_>,
    keys: LockPositionKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.pool.key, keys.pool),
        (*accounts.position.key, keys.position),
        (*accounts.vesting.key, keys.vesting),
        (*accounts.position_nft_account.key, keys.position_nft_account),
        (*accounts.owner.key, keys.owner),
        (*accounts.payer.key, keys.payer),
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

pub fn lock_position_verify_writable_privileges<'me, 'info>(
    accounts: LockPositionAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.position, accounts.vesting, accounts.payer] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}

pub fn lock_position_verify_signer_privileges<'me, 'info>(
    accounts: LockPositionAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.vesting, accounts.owner, accounts.payer] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}

pub fn lock_position_verify_account_privileges<'me, 'info>(
    accounts: LockPositionAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    lock_position_verify_writable_privileges(accounts)?;
    lock_position_verify_signer_privileges(accounts)?;
    Ok(())
}

pub const PERMANENT_LOCK_POSITION_IX_ACCOUNTS_LEN: usize = 6;
#[derive(Copy, Clone, Debug)]
pub struct PermanentLockPositionAccounts<'me, 'info> {
    pub pool: &'me AccountInfo<'info>,
    pub position: &'me AccountInfo<'info>,
    pub position_nft_account: &'me AccountInfo<'info>,
    pub owner: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct PermanentLockPositionKeys {
    pub pool: Pubkey,
    pub position: Pubkey,
    pub position_nft_account: Pubkey,
    pub owner: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}

impl From<PermanentLockPositionAccounts<'_, '_>> for PermanentLockPositionKeys {
    fn from(accounts: PermanentLockPositionAccounts) -> Self {
        Self {
            pool: *accounts.pool.key,
            position: *accounts.position.key,
            position_nft_account: *accounts.position_nft_account.key,
            owner: *accounts.owner.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}

impl From<PermanentLockPositionKeys> for [AccountMeta; PERMANENT_LOCK_POSITION_IX_ACCOUNTS_LEN] {
    fn from(keys: PermanentLockPositionKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.pool,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.position,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.position_nft_account,
                is_signer: false,
                is_writable: false,
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

impl From<[Pubkey; PERMANENT_LOCK_POSITION_IX_ACCOUNTS_LEN]> for PermanentLockPositionKeys {
    fn from(pubkeys: [Pubkey; PERMANENT_LOCK_POSITION_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            pool: pubkeys[0],
            position: pubkeys[1],
            position_nft_account: pubkeys[2],
            owner: pubkeys[3],
            event_authority: pubkeys[4],
            program: pubkeys[5],
        }
    }
}

impl<'info> From<PermanentLockPositionAccounts<'_, 'info>> for [AccountInfo<'info>; PERMANENT_LOCK_POSITION_IX_ACCOUNTS_LEN] {
    fn from(accounts: PermanentLockPositionAccounts<'_, 'info>) -> Self {
        [
            accounts.pool.clone(),
            accounts.position.clone(),
            accounts.position_nft_account.clone(),
            accounts.owner.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}

impl<'me, 'info> From<&'me [AccountInfo<'info>; PERMANENT_LOCK_POSITION_IX_ACCOUNTS_LEN]> for PermanentLockPositionAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; PERMANENT_LOCK_POSITION_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            pool: &arr[0],
            position: &arr[1],
            position_nft_account: &arr[2],
            owner: &arr[3],
            event_authority: &arr[4],
            program: &arr[5],
        }
    }
}

pub const PERMANENT_LOCK_POSITION_IX_DISCM: [u8; 8] = [165, 176, 125, 6, 231, 171, 186, 213];

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct PermanentLockPositionIxArgs {
    pub permanent_lock_liquidity: u128,
}

#[derive(Clone, Debug, PartialEq)]
pub struct PermanentLockPositionIxData(pub PermanentLockPositionIxArgs);

impl From<PermanentLockPositionIxArgs> for PermanentLockPositionIxData {
    fn from(args: PermanentLockPositionIxArgs) -> Self {
        Self(args)
    }
}

impl PermanentLockPositionIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != PERMANENT_LOCK_POSITION_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    PERMANENT_LOCK_POSITION_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(PermanentLockPositionIxArgs::deserialize(&mut reader)?))
    }

    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&PERMANENT_LOCK_POSITION_IX_DISCM)?;
        self.0.serialize(&mut writer)
    }

    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}

pub fn permanent_lock_position_ix_with_program_id(
    program_id: Pubkey,
    keys: PermanentLockPositionKeys,
    args: PermanentLockPositionIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; PERMANENT_LOCK_POSITION_IX_ACCOUNTS_LEN] = keys.into();
    let data: PermanentLockPositionIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}

pub fn permanent_lock_position_ix(
    keys: PermanentLockPositionKeys,
    args: PermanentLockPositionIxArgs,
) -> std::io::Result<Instruction> {
    permanent_lock_position_ix_with_program_id(crate::ID, keys, args)
}

pub fn permanent_lock_position_invoke_with_program_id(
    program_id: Pubkey,
    accounts: PermanentLockPositionAccounts<'_, '_>,
    args: PermanentLockPositionIxArgs,
) -> ProgramResult {
    let keys: PermanentLockPositionKeys = accounts.into();
    let ix = permanent_lock_position_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}

pub fn permanent_lock_position_invoke(
    accounts: PermanentLockPositionAccounts<'_, '_>,
    args: PermanentLockPositionIxArgs,
) -> ProgramResult {
    permanent_lock_position_invoke_with_program_id(crate::ID, accounts, args)
}

pub fn permanent_lock_position_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: PermanentLockPositionAccounts<'_, '_>,
    args: PermanentLockPositionIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: PermanentLockPositionKeys = accounts.into();
    let ix = permanent_lock_position_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}

pub fn permanent_lock_position_invoke_signed(
    accounts: PermanentLockPositionAccounts<'_, '_>,
    args: PermanentLockPositionIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    permanent_lock_position_invoke_signed_with_program_id(crate::ID, accounts, args, seeds)
}

pub fn permanent_lock_position_verify_account_keys(
    accounts: PermanentLockPositionAccounts<'_, '_>,
    keys: PermanentLockPositionKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.pool.key, keys.pool),
        (*accounts.position.key, keys.position),
        (*accounts.position_nft_account.key, keys.position_nft_account),
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

pub fn permanent_lock_position_verify_writable_privileges<'me, 'info>(
    accounts: PermanentLockPositionAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.pool, accounts.position] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}

pub fn permanent_lock_position_verify_signer_privileges<'me, 'info>(
    accounts: PermanentLockPositionAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.owner] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}

pub fn permanent_lock_position_verify_account_privileges<'me, 'info>(
    accounts: PermanentLockPositionAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    permanent_lock_position_verify_writable_privileges(accounts)?;
    permanent_lock_position_verify_signer_privileges(accounts)?;
    Ok(())
}

pub const REFRESH_VESTING_IX_ACCOUNTS_LEN: usize = 4;

#[derive(Copy, Clone, Debug)]
pub struct RefreshVestingAccounts<'me, 'info> {
    pub pool: &'me AccountInfo<'info>,
    pub position: &'me AccountInfo<'info>,
    pub position_nft_account: &'me AccountInfo<'info>,
    pub owner: &'me AccountInfo<'info>,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct RefreshVestingKeys {
    pub pool: Pubkey,
    pub position: Pubkey,
    pub position_nft_account: Pubkey,
    pub owner: Pubkey,
}

impl From<RefreshVestingAccounts<'_, '_>> for RefreshVestingKeys {
    fn from(accounts: RefreshVestingAccounts) -> Self {
        Self {
            pool: *accounts.pool.key,
            position: *accounts.position.key,
            position_nft_account: *accounts.position_nft_account.key,
            owner: *accounts.owner.key,
        }
    }
}

impl From<RefreshVestingKeys> for [AccountMeta; REFRESH_VESTING_IX_ACCOUNTS_LEN] {
    fn from(keys: RefreshVestingKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.pool,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.position,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.position_nft_account,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.owner,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}

impl From<[Pubkey; REFRESH_VESTING_IX_ACCOUNTS_LEN]> for RefreshVestingKeys {
    fn from(pubkeys: [Pubkey; REFRESH_VESTING_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            pool: pubkeys[0],
            position: pubkeys[1],
            position_nft_account: pubkeys[2],
            owner: pubkeys[3],
        }
    }
}

impl<'info> From<RefreshVestingAccounts<'_, 'info>> for [AccountInfo<'info>; REFRESH_VESTING_IX_ACCOUNTS_LEN] {
    fn from(accounts: RefreshVestingAccounts<'_, 'info>) -> Self {
        [
            accounts.pool.clone(),
            accounts.position.clone(),
            accounts.position_nft_account.clone(),
            accounts.owner.clone(),
        ]
    }
}

impl<'me, 'info> From<&'me [AccountInfo<'info>; REFRESH_VESTING_IX_ACCOUNTS_LEN]> for RefreshVestingAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; REFRESH_VESTING_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            pool: &arr[0],
            position: &arr[1],
            position_nft_account: &arr[2],
            owner: &arr[3],
        }
    }
}

pub const REFRESH_VESTING_IX_DISCM: [u8; 8] = [9, 94, 216, 14, 116, 204, 247, 0];

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RefreshVestingIxArgs {}

#[derive(Clone, Debug, PartialEq)]
pub struct RefreshVestingIxData(pub RefreshVestingIxArgs);

impl From<RefreshVestingIxArgs> for RefreshVestingIxData {
    fn from(args: RefreshVestingIxArgs) -> Self {
        Self(args)
    }
}

impl RefreshVestingIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != REFRESH_VESTING_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    REFRESH_VESTING_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(RefreshVestingIxArgs::deserialize(&mut reader)?))
    }

    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&REFRESH_VESTING_IX_DISCM)?;
        self.0.serialize(&mut writer)
    }

    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}

pub fn refresh_vesting_ix_with_program_id(
    program_id: Pubkey,
    keys: RefreshVestingKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; REFRESH_VESTING_IX_ACCOUNTS_LEN] = keys.into();
    let data: RefreshVestingIxData = RefreshVestingIxArgs {}.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}

pub fn refresh_vesting_ix(keys: RefreshVestingKeys) -> std::io::Result<Instruction> {
    refresh_vesting_ix_with_program_id(crate::ID, keys)
}

pub fn refresh_vesting_invoke_with_program_id(
    program_id: Pubkey,
    accounts: RefreshVestingAccounts<'_, '_>,
) -> ProgramResult {
    let keys: RefreshVestingKeys = accounts.into();
    let ix = refresh_vesting_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}

pub fn refresh_vesting_invoke(accounts: RefreshVestingAccounts<'_, '_>) -> ProgramResult {
    refresh_vesting_invoke_with_program_id(crate::ID, accounts)
}

pub fn refresh_vesting_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: RefreshVestingAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: RefreshVestingKeys = accounts.into();
    let ix = refresh_vesting_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}

pub fn refresh_vesting_invoke_signed(
    accounts: RefreshVestingAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    refresh_vesting_invoke_signed_with_program_id(crate::ID, accounts, seeds)
}

pub fn refresh_vesting_verify_account_keys(
    accounts: RefreshVestingAccounts<'_, '_>,
    keys: RefreshVestingKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.pool.key, keys.pool),
        (*accounts.position.key, keys.position),
        (*accounts.position_nft_account.key, keys.position_nft_account),
        (*accounts.owner.key, keys.owner),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}

pub fn refresh_vesting_verify_writable_privileges<'me, 'info>(
    accounts: RefreshVestingAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.position] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}

pub fn refresh_vesting_verify_signer_privileges<'me, 'info>(
    accounts: RefreshVestingAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    // No signers specified in the JSON
    Ok(())
}

pub fn refresh_vesting_verify_account_privileges<'me, 'info>(
    accounts: RefreshVestingAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    refresh_vesting_verify_writable_privileges(accounts)?;
    refresh_vesting_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const REMOVE_ALL_LIQUIDITY_IX_ACCOUNTS_LEN: usize = 15;

#[derive(Copy, Clone, Debug)]
pub struct RemoveAllLiquidityAccounts<'me, 'info> {
    pub pool_authority: &'me AccountInfo<'info>,
    pub pool: &'me AccountInfo<'info>,
    pub position: &'me AccountInfo<'info>,
    pub token_a_account: &'me AccountInfo<'info>,
    pub token_b_account: &'me AccountInfo<'info>,
    pub token_a_vault: &'me AccountInfo<'info>,
    pub token_b_vault: &'me AccountInfo<'info>,
    pub token_a_mint: &'me AccountInfo<'info>,
    pub token_b_mint: &'me AccountInfo<'info>,
    pub position_nft_account: &'me AccountInfo<'info>,
    pub owner: &'me AccountInfo<'info>,
    pub token_a_program: &'me AccountInfo<'info>,
    pub token_b_program: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct RemoveAllLiquidityKeys {
    pub pool_authority: Pubkey,
    pub pool: Pubkey,
    pub position: Pubkey,
    pub token_a_account: Pubkey,
    pub token_b_account: Pubkey,
    pub token_a_vault: Pubkey,
    pub token_b_vault: Pubkey,
    pub token_a_mint: Pubkey,
    pub token_b_mint: Pubkey,
    pub position_nft_account: Pubkey,
    pub owner: Pubkey,
    pub token_a_program: Pubkey,
    pub token_b_program: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}

impl From<RemoveAllLiquidityAccounts<'_, '_>> for RemoveAllLiquidityKeys {
    fn from(accounts: RemoveAllLiquidityAccounts) -> Self {
        Self {
            pool_authority: *accounts.pool_authority.key,
            pool: *accounts.pool.key,
            position: *accounts.position.key,
            token_a_account: *accounts.token_a_account.key,
            token_b_account: *accounts.token_b_account.key,
            token_a_vault: *accounts.token_a_vault.key,
            token_b_vault: *accounts.token_b_vault.key,
            token_a_mint: *accounts.token_a_mint.key,
            token_b_mint: *accounts.token_b_mint.key,
            position_nft_account: *accounts.position_nft_account.key,
            owner: *accounts.owner.key,
            token_a_program: *accounts.token_a_program.key,
            token_b_program: *accounts.token_b_program.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}

impl From<RemoveAllLiquidityKeys> for [AccountMeta; REMOVE_ALL_LIQUIDITY_IX_ACCOUNTS_LEN] {
    fn from(keys: RemoveAllLiquidityKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.pool_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pool,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.position,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_a_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_b_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_a_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_b_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_a_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_b_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.position_nft_account,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.owner,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_a_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_b_program,
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
            pool_authority: pubkeys[0],
            pool: pubkeys[1],
            position: pubkeys[2],
            token_a_account: pubkeys[3],
            token_b_account: pubkeys[4],
            token_a_vault: pubkeys[5],
            token_b_vault: pubkeys[6],
            token_a_mint: pubkeys[7],
            token_b_mint: pubkeys[8],
            position_nft_account: pubkeys[9],
            owner: pubkeys[10],
            token_a_program: pubkeys[11],
            token_b_program: pubkeys[12],
            event_authority: pubkeys[13],
            program: pubkeys[14],
        }
    }
}

impl<'info> From<RemoveAllLiquidityAccounts<'_, 'info>> for [AccountInfo<'info>; REMOVE_ALL_LIQUIDITY_IX_ACCOUNTS_LEN] {
    fn from(accounts: RemoveAllLiquidityAccounts<'_, 'info>) -> Self {
        [
            accounts.pool_authority.clone(),
            accounts.pool.clone(),
            accounts.position.clone(),
            accounts.token_a_account.clone(),
            accounts.token_b_account.clone(),
            accounts.token_a_vault.clone(),
            accounts.token_b_vault.clone(),
            accounts.token_a_mint.clone(),
            accounts.token_b_mint.clone(),
            accounts.position_nft_account.clone(),
            accounts.owner.clone(),
            accounts.token_a_program.clone(),
            accounts.token_b_program.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}

impl<'me, 'info> From<&'me [AccountInfo<'info>; REMOVE_ALL_LIQUIDITY_IX_ACCOUNTS_LEN]> for RemoveAllLiquidityAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; REMOVE_ALL_LIQUIDITY_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            pool_authority: &arr[0],
            pool: &arr[1],
            position: &arr[2],
            token_a_account: &arr[3],
            token_b_account: &arr[4],
            token_a_vault: &arr[5],
            token_b_vault: &arr[6],
            token_a_mint: &arr[7],
            token_b_mint: &arr[8],
            position_nft_account: &arr[9],
            owner: &arr[10],
            token_a_program: &arr[11],
            token_b_program: &arr[12],
            event_authority: &arr[13],
            program: &arr[14],
        }
    }
}

pub const REMOVE_ALL_LIQUIDITY_IX_DISCM: [u8; 8] = [10, 51, 61, 35, 112, 105, 24, 85];

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RemoveAllLiquidityIxArgs {
    pub token_a_amount_threshold: u64,
    pub token_b_amount_threshold: u64,
}

#[derive(Clone, Debug, PartialEq)]
pub struct RemoveAllLiquidityIxData(pub RemoveAllLiquidityIxArgs);

impl From<RemoveAllLiquidityIxArgs> for RemoveAllLiquidityIxData {
    fn from(args: RemoveAllLiquidityIxArgs) -> Self {
        Self(args)
    }
}

impl RemoveAllLiquidityIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != REMOVE_ALL_LIQUIDITY_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    REMOVE_ALL_LIQUIDITY_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(RemoveAllLiquidityIxArgs::deserialize(&mut reader)?))
    }

    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&REMOVE_ALL_LIQUIDITY_IX_DISCM)?;
        self.0.serialize(&mut writer)
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
    args: RemoveAllLiquidityIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; REMOVE_ALL_LIQUIDITY_IX_ACCOUNTS_LEN] = keys.into();
    let data: RemoveAllLiquidityIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}

pub fn remove_all_liquidity_ix(
    keys: RemoveAllLiquidityKeys,
    args: RemoveAllLiquidityIxArgs,
) -> std::io::Result<Instruction> {
    remove_all_liquidity_ix_with_program_id(crate::ID, keys, args)
}

pub fn remove_all_liquidity_invoke_with_program_id(
    program_id: Pubkey,
    accounts: RemoveAllLiquidityAccounts<'_, '_>,
    args: RemoveAllLiquidityIxArgs,
) -> ProgramResult {
    let keys: RemoveAllLiquidityKeys = accounts.into();
    let ix = remove_all_liquidity_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}

pub fn remove_all_liquidity_invoke(
    accounts: RemoveAllLiquidityAccounts<'_, '_>,
    args: RemoveAllLiquidityIxArgs,
) -> ProgramResult {
    remove_all_liquidity_invoke_with_program_id(crate::ID, accounts, args)
}

pub fn remove_all_liquidity_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: RemoveAllLiquidityAccounts<'_, '_>,
    args: RemoveAllLiquidityIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: RemoveAllLiquidityKeys = accounts.into();
    let ix = remove_all_liquidity_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}

pub fn remove_all_liquidity_invoke_signed(
    accounts: RemoveAllLiquidityAccounts<'_, '_>,
    args: RemoveAllLiquidityIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    remove_all_liquidity_invoke_signed_with_program_id(crate::ID, accounts, args, seeds)
}

pub fn remove_all_liquidity_verify_account_keys(
    accounts: RemoveAllLiquidityAccounts<'_, '_>,
    keys: RemoveAllLiquidityKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.pool_authority.key, keys.pool_authority),
        (*accounts.pool.key, keys.pool),
        (*accounts.position.key, keys.position),
        (*accounts.token_a_account.key, keys.token_a_account),
        (*accounts.token_b_account.key, keys.token_b_account),
        (*accounts.token_a_vault.key, keys.token_a_vault),
        (*accounts.token_b_vault.key, keys.token_b_vault),
        (*accounts.token_a_mint.key, keys.token_a_mint),
        (*accounts.token_b_mint.key, keys.token_b_mint),
        (*accounts.position_nft_account.key, keys.position_nft_account),
        (*accounts.owner.key, keys.owner),
        (*accounts.token_a_program.key, keys.token_a_program),
        (*accounts.token_b_program.key, keys.token_b_program),
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
        accounts.pool,
        accounts.position,
        accounts.token_a_account,
        accounts.token_b_account,
        accounts.token_a_vault,
        accounts.token_b_vault,
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
    for should_be_signer in [accounts.owner] {
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

pub const REMOVE_LIQUIDITY_IX_ACCOUNTS_LEN: usize = 15;

#[derive(Copy, Clone, Debug)]
pub struct RemoveLiquidityAccounts<'me, 'info> {
    pub pool_authority: &'me AccountInfo<'info>,
    pub pool: &'me AccountInfo<'info>,
    pub position: &'me AccountInfo<'info>,
    pub token_a_account: &'me AccountInfo<'info>,
    pub token_b_account: &'me AccountInfo<'info>,
    pub token_a_vault: &'me AccountInfo<'info>,
    pub token_b_vault: &'me AccountInfo<'info>,
    pub token_a_mint: &'me AccountInfo<'info>,
    pub token_b_mint: &'me AccountInfo<'info>,
    pub position_nft_account: &'me AccountInfo<'info>,
    pub owner: &'me AccountInfo<'info>,
    pub token_a_program: &'me AccountInfo<'info>,
    pub token_b_program: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct RemoveLiquidityKeys {
    pub pool_authority: Pubkey,
    pub pool: Pubkey,
    pub position: Pubkey,
    pub token_a_account: Pubkey,
    pub token_b_account: Pubkey,
    pub token_a_vault: Pubkey,
    pub token_b_vault: Pubkey,
    pub token_a_mint: Pubkey,
    pub token_b_mint: Pubkey,
    pub position_nft_account: Pubkey,
    pub owner: Pubkey,
    pub token_a_program: Pubkey,
    pub token_b_program: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}

impl From<RemoveLiquidityAccounts<'_, '_>> for RemoveLiquidityKeys {
    fn from(accounts: RemoveLiquidityAccounts) -> Self {
        Self {
            pool_authority: *accounts.pool_authority.key,
            pool: *accounts.pool.key,
            position: *accounts.position.key,
            token_a_account: *accounts.token_a_account.key,
            token_b_account: *accounts.token_b_account.key,
            token_a_vault: *accounts.token_a_vault.key,
            token_b_vault: *accounts.token_b_vault.key,
            token_a_mint: *accounts.token_a_mint.key,
            token_b_mint: *accounts.token_b_mint.key,
            position_nft_account: *accounts.position_nft_account.key,
            owner: *accounts.owner.key,
            token_a_program: *accounts.token_a_program.key,
            token_b_program: *accounts.token_b_program.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}

impl From<RemoveLiquidityKeys> for [AccountMeta; REMOVE_LIQUIDITY_IX_ACCOUNTS_LEN] {
    fn from(keys: RemoveLiquidityKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.pool_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pool,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.position,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_a_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_b_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_a_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_b_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_a_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_b_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.position_nft_account,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.owner,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_a_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_b_program,
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
            pool_authority: pubkeys[0],
            pool: pubkeys[1],
            position: pubkeys[2],
            token_a_account: pubkeys[3],
            token_b_account: pubkeys[4],
            token_a_vault: pubkeys[5],
            token_b_vault: pubkeys[6],
            token_a_mint: pubkeys[7],
            token_b_mint: pubkeys[8],
            position_nft_account: pubkeys[9],
            owner: pubkeys[10],
            token_a_program: pubkeys[11],
            token_b_program: pubkeys[12],
            event_authority: pubkeys[13],
            program: pubkeys[14],
        }
    }
}

impl<'info> From<RemoveLiquidityAccounts<'_, 'info>> for [AccountInfo<'info>; REMOVE_LIQUIDITY_IX_ACCOUNTS_LEN] {
    fn from(accounts: RemoveLiquidityAccounts<'_, 'info>) -> Self {
        [
            accounts.pool_authority.clone(),
            accounts.pool.clone(),
            accounts.position.clone(),
            accounts.token_a_account.clone(),
            accounts.token_b_account.clone(),
            accounts.token_a_vault.clone(),
            accounts.token_b_vault.clone(),
            accounts.token_a_mint.clone(),
            accounts.token_b_mint.clone(),
            accounts.position_nft_account.clone(),
            accounts.owner.clone(),
            accounts.token_a_program.clone(),
            accounts.token_b_program.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}

impl<'me, 'info> From<&'me [AccountInfo<'info>; REMOVE_LIQUIDITY_IX_ACCOUNTS_LEN]> for RemoveLiquidityAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; REMOVE_LIQUIDITY_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            pool_authority: &arr[0],
            pool: &arr[1],
            position: &arr[2],
            token_a_account: &arr[3],
            token_b_account: &arr[4],
            token_a_vault: &arr[5],
            token_b_vault: &arr[6],
            token_a_mint: &arr[7],
            token_b_mint: &arr[8],
            position_nft_account: &arr[9],
            owner: &arr[10],
            token_a_program: &arr[11],
            token_b_program: &arr[12],
            event_authority: &arr[13],
            program: &arr[14],
        }
    }
}

pub const REMOVE_LIQUIDITY_IX_DISCM: [u8; 8] = [80, 85, 209, 72, 24, 206, 177, 108];

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RemoveLiquidityIxArgs {
    pub params: RemoveLiquidityParameters,
}

// Placeholder struct (replace with actual definition)
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RemoveLiquidityParameters {
    // Define fields as needed based on program requirements
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
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    REMOVE_LIQUIDITY_IX_DISCM, maybe_discm
                ),
            ));
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
        (*accounts.pool_authority.key, keys.pool_authority),
        (*accounts.pool.key, keys.pool),
        (*accounts.position.key, keys.position),
        (*accounts.token_a_account.key, keys.token_a_account),
        (*accounts.token_b_account.key, keys.token_b_account),
        (*accounts.token_a_vault.key, keys.token_a_vault),
        (*accounts.token_b_vault.key, keys.token_b_vault),
        (*accounts.token_a_mint.key, keys.token_a_mint),
        (*accounts.token_b_mint.key, keys.token_b_mint),
        (*accounts.position_nft_account.key, keys.position_nft_account),
        (*accounts.owner.key, keys.owner),
        (*accounts.token_a_program.key, keys.token_a_program),
        (*accounts.token_b_program.key, keys.token_b_program),
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
        accounts.pool,
        accounts.position,
        accounts.token_a_account,
        accounts.token_b_account,
        accounts.token_a_vault,
        accounts.token_b_vault,
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
    for should_be_signer in [accounts.owner] {
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
pub const SET_POOL_STATUS_IX_ACCOUNTS_LEN: usize = 4;

#[derive(Copy, Clone, Debug)]
pub struct SetPoolStatusAccounts<'me, 'info> {
    pub pool: &'me AccountInfo<'info>,
    pub admin: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SetPoolStatusKeys {
    pub pool: Pubkey,
    pub admin: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}

impl From<SetPoolStatusAccounts<'_, '_>> for SetPoolStatusKeys {
    fn from(accounts: SetPoolStatusAccounts) -> Self {
        Self {
            pool: *accounts.pool.key,
            admin: *accounts.admin.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}

impl From<SetPoolStatusKeys> for [AccountMeta; SET_POOL_STATUS_IX_ACCOUNTS_LEN] {
    fn from(keys: SetPoolStatusKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.pool,
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

impl From<[Pubkey; SET_POOL_STATUS_IX_ACCOUNTS_LEN]> for SetPoolStatusKeys {
    fn from(pubkeys: [Pubkey; SET_POOL_STATUS_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            pool: pubkeys[0],
            admin: pubkeys[1],
            event_authority: pubkeys[2],
            program: pubkeys[3],
        }
    }
}

impl<'info> From<SetPoolStatusAccounts<'_, 'info>> for [AccountInfo<'info>; SET_POOL_STATUS_IX_ACCOUNTS_LEN] {
    fn from(accounts: SetPoolStatusAccounts<'_, 'info>) -> Self {
        [
            accounts.pool.clone(),
            accounts.admin.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}

impl<'me, 'info> From<&'me [AccountInfo<'info>; SET_POOL_STATUS_IX_ACCOUNTS_LEN]> for SetPoolStatusAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; SET_POOL_STATUS_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            pool: &arr[0],
            admin: &arr[1],
            event_authority: &arr[2],
            program: &arr[3],
        }
    }
}

pub const SET_POOL_STATUS_IX_DISCM: [u8; 8] = [112, 87, 135, 223, 83, 204, 132, 53];

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SetPoolStatusIxArgs {
    pub status: u8,
}

#[derive(Clone, Debug, PartialEq)]
pub struct SetPoolStatusIxData(pub SetPoolStatusIxArgs);

impl From<SetPoolStatusIxArgs> for SetPoolStatusIxData {
    fn from(args: SetPoolStatusIxArgs) -> Self {
        Self(args)
    }
}

impl SetPoolStatusIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != SET_POOL_STATUS_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    SET_POOL_STATUS_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(SetPoolStatusIxArgs::deserialize(&mut reader)?))
    }

    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&SET_POOL_STATUS_IX_DISCM)?;
        self.0.serialize(&mut writer)
    }

    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}

pub fn set_pool_status_ix_with_program_id(
    program_id: Pubkey,
    keys: SetPoolStatusKeys,
    args: SetPoolStatusIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; SET_POOL_STATUS_IX_ACCOUNTS_LEN] = keys.into();
    let data: SetPoolStatusIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}

pub fn set_pool_status_ix(
    keys: SetPoolStatusKeys,
    args: SetPoolStatusIxArgs,
) -> std::io::Result<Instruction> {
    set_pool_status_ix_with_program_id(crate::ID, keys, args)
}

pub fn set_pool_status_invoke_with_program_id(
    program_id: Pubkey,
    accounts: SetPoolStatusAccounts<'_, '_>,
    args: SetPoolStatusIxArgs,
) -> ProgramResult {
    let keys: SetPoolStatusKeys = accounts.into();
    let ix = set_pool_status_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}

pub fn set_pool_status_invoke(
    accounts: SetPoolStatusAccounts<'_, '_>,
    args: SetPoolStatusIxArgs,
) -> ProgramResult {
    set_pool_status_invoke_with_program_id(crate::ID, accounts, args)
}

pub fn set_pool_status_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: SetPoolStatusAccounts<'_, '_>,
    args: SetPoolStatusIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: SetPoolStatusKeys = accounts.into();
    let ix = set_pool_status_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}

pub fn set_pool_status_invoke_signed(
    accounts: SetPoolStatusAccounts<'_, '_>,
    args: SetPoolStatusIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    set_pool_status_invoke_signed_with_program_id(crate::ID, accounts, args, seeds)
}

pub fn set_pool_status_verify_account_keys(
    accounts: SetPoolStatusAccounts<'_, '_>,
    keys: SetPoolStatusKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.pool.key, keys.pool),
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

pub fn set_pool_status_verify_writable_privileges<'me, 'info>(
    accounts: SetPoolStatusAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.pool] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}

pub fn set_pool_status_verify_signer_privileges<'me, 'info>(
    accounts: SetPoolStatusAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.admin] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}

pub fn set_pool_status_verify_account_privileges<'me, 'info>(
    accounts: SetPoolStatusAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    set_pool_status_verify_writable_privileges(accounts)?;
    set_pool_status_verify_signer_privileges(accounts)?;
    Ok(())
}

pub const SWAP_IX_ACCOUNTS_LEN: usize = 14;

#[derive(Copy, Clone, Debug)]
pub struct SwapAccounts<'me, 'info> {
    pub pool_authority: &'me AccountInfo<'info>,
    pub pool: &'me AccountInfo<'info>,
    pub input_token_account: &'me AccountInfo<'info>,
    pub output_token_account: &'me AccountInfo<'info>,
    pub token_a_vault: &'me AccountInfo<'info>,
    pub token_b_vault: &'me AccountInfo<'info>,
    pub token_a_mint: &'me AccountInfo<'info>,
    pub token_b_mint: &'me AccountInfo<'info>,
    pub payer: &'me AccountInfo<'info>,
    pub token_a_program: &'me AccountInfo<'info>,
    pub token_b_program: &'me AccountInfo<'info>,
    pub referral_token_account: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SwapKeys {
    pub pool_authority: Pubkey,
    pub pool: Pubkey,
    pub input_token_account: Pubkey,
    pub output_token_account: Pubkey,
    pub token_a_vault: Pubkey,
    pub token_b_vault: Pubkey,
    pub token_a_mint: Pubkey,
    pub token_b_mint: Pubkey,
    pub payer: Pubkey,
    pub token_a_program: Pubkey,
    pub token_b_program: Pubkey,
    pub referral_token_account: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}

impl From<SwapAccounts<'_, '_>> for SwapKeys {
    fn from(accounts: SwapAccounts) -> Self {
        Self {
            pool_authority: *accounts.pool_authority.key,
            pool: *accounts.pool.key,
            input_token_account: *accounts.input_token_account.key,
            output_token_account: *accounts.output_token_account.key,
            token_a_vault: *accounts.token_a_vault.key,
            token_b_vault: *accounts.token_b_vault.key,
            token_a_mint: *accounts.token_a_mint.key,
            token_b_mint: *accounts.token_b_mint.key,
            payer: *accounts.payer.key,
            token_a_program: *accounts.token_a_program.key,
            token_b_program: *accounts.token_b_program.key,
            referral_token_account: *accounts.referral_token_account.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}

impl From<SwapKeys> for [AccountMeta; SWAP_IX_ACCOUNTS_LEN] {
    fn from(keys: SwapKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.pool_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pool,
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
                pubkey: keys.token_a_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_b_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_a_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_b_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.payer,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_a_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_b_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.referral_token_account,
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

impl From<[Pubkey; SWAP_IX_ACCOUNTS_LEN]> for SwapKeys {
    fn from(pubkeys: [Pubkey; SWAP_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            pool_authority: pubkeys[0],
            pool: pubkeys[1],
            input_token_account: pubkeys[2],
            output_token_account: pubkeys[3],
            token_a_vault: pubkeys[4],
            token_b_vault: pubkeys[5],
            token_a_mint: pubkeys[6],
            token_b_mint: pubkeys[7],
            payer: pubkeys[8],
            token_a_program: pubkeys[9],
            token_b_program: pubkeys[10],
            referral_token_account: pubkeys[11],
            event_authority: pubkeys[12],
            program: pubkeys[13],
        }
    }
}

impl<'info> From<SwapAccounts<'_, 'info>> for [AccountInfo<'info>; SWAP_IX_ACCOUNTS_LEN] {
    fn from(accounts: SwapAccounts<'_, 'info>) -> Self {
        [
            accounts.pool_authority.clone(),
            accounts.pool.clone(),
            accounts.input_token_account.clone(),
            accounts.output_token_account.clone(),
            accounts.token_a_vault.clone(),
            accounts.token_b_vault.clone(),
            accounts.token_a_mint.clone(),
            accounts.token_b_mint.clone(),
            accounts.payer.clone(),
            accounts.token_a_program.clone(),
            accounts.token_b_program.clone(),
            accounts.referral_token_account.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}

impl<'me, 'info> From<&'me [AccountInfo<'info>; SWAP_IX_ACCOUNTS_LEN]> for SwapAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; SWAP_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            pool_authority: &arr[0],
            pool: &arr[1],
            input_token_account: &arr[2],
            output_token_account: &arr[3],
            token_a_vault: &arr[4],
            token_b_vault: &arr[5],
            token_a_mint: &arr[6],
            token_b_mint: &arr[7],
            payer: &arr[8],
            token_a_program: &arr[9],
            token_b_program: &arr[10],
            referral_token_account: &arr[11],
            event_authority: &arr[12],
            program: &arr[13],
        }
    }
}

pub const SWAP_IX_DISCM: [u8; 8] = [248, 198, 158, 145, 225, 117, 135, 200];

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SwapIxArgs {
    pub params: SwapParameters,
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
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    SWAP_IX_DISCM, maybe_discm
                ),
            ));
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
        (*accounts.pool_authority.key, keys.pool_authority),
        (*accounts.pool.key, keys.pool),
        (*accounts.input_token_account.key, keys.input_token_account),
        (*accounts.output_token_account.key, keys.output_token_account),
        (*accounts.token_a_vault.key, keys.token_a_vault),
        (*accounts.token_b_vault.key, keys.token_b_vault),
        (*accounts.token_a_mint.key, keys.token_a_mint),
        (*accounts.token_b_mint.key, keys.token_b_mint),
        (*accounts.payer.key, keys.payer),
        (*accounts.token_a_program.key, keys.token_a_program),
        (*accounts.token_b_program.key, keys.token_b_program),
        (
            *accounts.referral_token_account.key,
            keys.referral_token_account,
        ),
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
        accounts.pool,
        accounts.input_token_account,
        accounts.output_token_account,
        accounts.token_a_vault,
        accounts.token_b_vault,
        accounts.referral_token_account,
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
    for should_be_signer in [accounts.payer] {
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

pub const UPDATE_REWARD_DURATION_IX_ACCOUNTS_LEN: usize = 4;

#[derive(Copy, Clone, Debug)]
pub struct UpdateRewardDurationAccounts<'me, 'info> {
    pub pool: &'me AccountInfo<'info>,
    pub admin: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct UpdateRewardDurationKeys {
    pub pool: Pubkey,
    pub admin: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}

impl From<UpdateRewardDurationAccounts<'_, '_>> for UpdateRewardDurationKeys {
    fn from(accounts: UpdateRewardDurationAccounts) -> Self {
        Self {
            pool: *accounts.pool.key,
            admin: *accounts.admin.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}

impl From<UpdateRewardDurationKeys> for [AccountMeta; UPDATE_REWARD_DURATION_IX_ACCOUNTS_LEN] {
    fn from(keys: UpdateRewardDurationKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.pool,
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

impl From<[Pubkey; UPDATE_REWARD_DURATION_IX_ACCOUNTS_LEN]> for UpdateRewardDurationKeys {
    fn from(pubkeys: [Pubkey; UPDATE_REWARD_DURATION_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            pool: pubkeys[0],
            admin: pubkeys[1],
            event_authority: pubkeys[2],
            program: pubkeys[3],
        }
    }
}

impl<'info> From<UpdateRewardDurationAccounts<'_, 'info>> for [AccountInfo<'info>; UPDATE_REWARD_DURATION_IX_ACCOUNTS_LEN] {
    fn from(accounts: UpdateRewardDurationAccounts<'_, 'info>) -> Self {
        [
            accounts.pool.clone(),
            accounts.admin.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}

impl<'me, 'info> From<&'me [AccountInfo<'info>; UPDATE_REWARD_DURATION_IX_ACCOUNTS_LEN]> for UpdateRewardDurationAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; UPDATE_REWARD_DURATION_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            pool: &arr[0],
            admin: &arr[1],
            event_authority: &arr[2],
            program: &arr[3],
        }
    }
}

pub const UPDATE_REWARD_DURATION_IX_DISCM: [u8; 8] = [138, 174, 196, 169, 213, 235, 254, 107];

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct UpdateRewardDurationIxArgs {
    pub reward_index: u8,
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
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    UPDATE_REWARD_DURATION_IX_DISCM, maybe_discm
                ),
            ));
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
    update_reward_duration_invoke_signed_with_program_id(crate::ID, accounts, args, seeds)
}

pub fn update_reward_duration_verify_account_keys(
    accounts: UpdateRewardDurationAccounts<'_, '_>,
    keys: UpdateRewardDurationKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.pool.key, keys.pool),
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

pub fn update_reward_duration_verify_writable_privileges<'me, 'info>(
    accounts: UpdateRewardDurationAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.pool] {
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

pub const UPDATE_REWARD_FUNDER_IX_ACCOUNTS_LEN: usize = 4;

#[derive(Copy, Clone, Debug)]
pub struct UpdateRewardFunderAccounts<'me, 'info> {
    pub pool: &'me AccountInfo<'info>,
    pub admin: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct UpdateRewardFunderKeys {
    pub pool: Pubkey,
    pub admin: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}

impl From<UpdateRewardFunderAccounts<'_, '_>> for UpdateRewardFunderKeys {
    fn from(accounts: UpdateRewardFunderAccounts) -> Self {
        Self {
            pool: *accounts.pool.key,
            admin: *accounts.admin.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}

impl From<UpdateRewardFunderKeys> for [AccountMeta; UPDATE_REWARD_FUNDER_IX_ACCOUNTS_LEN] {
    fn from(keys: UpdateRewardFunderKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.pool,
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
            pool: pubkeys[0],
            admin: pubkeys[1],
            event_authority: pubkeys[2],
            program: pubkeys[3],
        }
    }
}

impl<'info> From<UpdateRewardFunderAccounts<'_, 'info>> for [AccountInfo<'info>; UPDATE_REWARD_FUNDER_IX_ACCOUNTS_LEN] {
    fn from(accounts: UpdateRewardFunderAccounts<'_, 'info>) -> Self {
        [
            accounts.pool.clone(),
            accounts.admin.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}

impl<'me, 'info> From<&'me [AccountInfo<'info>; UPDATE_REWARD_FUNDER_IX_ACCOUNTS_LEN]> for UpdateRewardFunderAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; UPDATE_REWARD_FUNDER_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            pool: &arr[0],
            admin: &arr[1],
            event_authority: &arr[2],
            program: &arr[3],
        }
    }
}

pub const UPDATE_REWARD_FUNDER_IX_DISCM: [u8; 8] = [211, 28, 48, 32, 215, 160, 35, 23];

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq,Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct UpdateRewardFunderIxArgs {
    pub reward_index: u8,
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
            return Err(std::io::Error::new(
               std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    UPDATE_REWARD_FUNDER_IX_DISCM, maybe_discm
                ),
            ));
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
        (*accounts.pool.key, keys.pool),
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
    for should_be_writable in [accounts.pool] {
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

pub const WITHDRAW_INELIGIBLE_REWARD_IX_ACCOUNTS_LEN: usize = 9;

#[derive(Copy, Clone, Debug)]
pub struct WithdrawIneligibleRewardAccounts<'me, 'info> {
    pub pool_authority: &'me AccountInfo<'info>,
    pub pool: &'me AccountInfo<'info>,
    pub reward_vault: &'me AccountInfo<'info>,
    pub reward_mint: &'me AccountInfo<'info>,
    pub funder_token_account: &'me AccountInfo<'info>,
    pub funder: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct WithdrawIneligibleRewardKeys {
    pub pool_authority: Pubkey,
    pub pool: Pubkey,
    pub reward_vault: Pubkey,
    pub reward_mint: Pubkey,
    pub funder_token_account: Pubkey,
    pub funder: Pubkey,
    pub token_program: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}

impl From<WithdrawIneligibleRewardAccounts<'_, '_>> for WithdrawIneligibleRewardKeys {
    fn from(accounts: WithdrawIneligibleRewardAccounts) -> Self {
        Self {
            pool_authority: *accounts.pool_authority.key,
            pool: *accounts.pool.key,
            reward_vault: *accounts.reward_vault.key,
            reward_mint: *accounts.reward_mint.key,
            funder_token_account: *accounts.funder_token_account.key,
            funder: *accounts.funder.key,
            token_program: *accounts.token_program.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}

impl From<WithdrawIneligibleRewardKeys> for [AccountMeta; WITHDRAW_INELIGIBLE_REWARD_IX_ACCOUNTS_LEN] {
    fn from(keys: WithdrawIneligibleRewardKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.pool_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pool,
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

impl From<[Pubkey; WITHDRAW_INELIGIBLE_REWARD_IX_ACCOUNTS_LEN]> for WithdrawIneligibleRewardKeys {
    fn from(pubkeys: [Pubkey; WITHDRAW_INELIGIBLE_REWARD_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            pool_authority: pubkeys[0],
            pool: pubkeys[1],
            reward_vault: pubkeys[2],
            reward_mint: pubkeys[3],
            funder_token_account: pubkeys[4],
            funder: pubkeys[5],
            token_program: pubkeys[6],
            event_authority: pubkeys[7],
            program: pubkeys[8],
        }
    }
}

impl<'info> From<WithdrawIneligibleRewardAccounts<'_, 'info>> for [AccountInfo<'info>; WITHDRAW_INELIGIBLE_REWARD_IX_ACCOUNTS_LEN] {
    fn from(accounts: WithdrawIneligibleRewardAccounts<'_, 'info>) -> Self {
        [
            accounts.pool_authority.clone(),
            accounts.pool.clone(),
            accounts.reward_vault.clone(),
            accounts.reward_mint.clone(),
            accounts.funder_token_account.clone(),
            accounts.funder.clone(),
            accounts.token_program.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}

impl<'me, 'info> From<&'me [AccountInfo<'info>; WITHDRAW_INELIGIBLE_REWARD_IX_ACCOUNTS_LEN]> for WithdrawIneligibleRewardAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; WITHDRAW_INELIGIBLE_REWARD_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            pool_authority: &arr[0],
            pool: &arr[1],
            reward_vault: &arr[2],
            reward_mint: &arr[3],
            funder_token_account: &arr[4],
            funder: &arr[5],
            token_program: &arr[6],
            event_authority: &arr[7],
            program: &arr[8],
        }
    }
}

pub const WITHDRAW_INELIGIBLE_REWARD_IX_DISCM: [u8; 8] = [148, 206, 42, 195, 247, 49, 103, 8];

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct WithdrawIneligibleRewardIxArgs {
    pub reward_index: u8,
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
            return Err(std::io::Error::new(
               std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    WITHDRAW_INELIGIBLE_REWARD_IX_DISCM, maybe_discm
                ),
            ));
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
    withdraw_ineligible_reward_invoke_signed_with_program_id(crate::ID, accounts, args, seeds)
}

pub fn withdraw_ineligible_reward_verify_account_keys(
    accounts: WithdrawIneligibleRewardAccounts<'_, '_>,
    keys: WithdrawIneligibleRewardKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.pool_authority.key, keys.pool_authority),
        (*accounts.pool.key, keys.pool),
        (*accounts.reward_vault.key, keys.reward_vault),
        (*accounts.reward_mint.key, keys.reward_mint),
        (*accounts.funder_token_account.key, keys.funder_token_account),
        (*accounts.funder.key, keys.funder),
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
        accounts.pool,
        accounts.reward_vault,
        accounts.funder_token_account,
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