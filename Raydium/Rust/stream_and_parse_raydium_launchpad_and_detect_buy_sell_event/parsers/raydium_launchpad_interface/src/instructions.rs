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

use typedefs::{PlatformParams,PlatformConfigParam};
use inflector::Inflector;
use std::io::Read;
use strum_macros::{Display, EnumString};

#[derive(Clone, Debug, PartialEq, EnumString, Display)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum RaydiumLaunchpadProgramIx {
    BuyExactIn(BuyExactInIxArgs),
    BuyExactOut(BuyExactOutIxArgs),
    ClaimPlatformFee(ClaimPlatformFeeIxArgs),
    ClaimVestedToken(ClaimVestedTokenIxArgs),
    CollectFee(CollectFeeIxArgs),
    CollectMigrateFee(CollectMigrateFeeIxArgs),
    CreateConfig(CreateConfigIxArgs),
    CreatePlatformConfig(CreatePlatformConfigIxArgs),
    CreateVestingAccount(CreateVestingAccountIxArgs),
    Initialize(InitializeIxArgs),
    MigrateToAmm(MigrateToAmmIxArgs),
    MigrateToCpswap(MigrateToCpswapIxArgs),
    SellExactIn(SellExactInIxArgs),
    SellExactOut(SellExactOutIxArgs),
    UpdateConfig(UpdateConfigIxArgs),
    UpdatePlatformConfig(UpdatePlatformConfigIxArgs),
}
impl RaydiumLaunchpadProgramIx {
        pub fn name(&self) -> &str {
        match self {
            Self::BuyExactIn(_) => "BuyExactIn",
            Self::BuyExactOut(_) => "BuyExactOut",
            Self::ClaimPlatformFee(_) => "ClaimPlatformFee",
            Self::ClaimVestedToken(_) => "ClaimVestedToken",
            Self::CollectFee(_) => "CollectFee",
            Self::CollectMigrateFee(_) => "CollectMigrateFee",
            Self::CreateConfig(_) => "CreateConfig",
            Self::CreatePlatformConfig(_) => "CreatePlatformConfig",
            Self::CreateVestingAccount(_) => "CreateVestingAccount",
            Self::Initialize(_) => "Initialize",
            Self::MigrateToAmm(_) => "MigrateToAmm",
            Self::MigrateToCpswap(_) => "MigrateToCpswap",
            Self::SellExactIn(_) => "SellExactIn",
            Self::SellExactOut(_) => "SellExactOut",
            Self::UpdateConfig(_) => "UpdateConfig",
            Self::UpdatePlatformConfig(_) => "UpdatePlatformConfig",
        }
    }

    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        match maybe_discm {
            BUY_EXACT_IN_IX_DISCM => {
                Ok(
                    Self::BuyExactIn(
                        BuyExactInIxArgs::deserialize(&mut reader)?,
                    ),
                )
            }
            BUY_EXACT_OUT_IX_DISCM => {
                Ok(
                    Self::BuyExactOut(
                        BuyExactOutIxArgs::deserialize(&mut reader)?,
                    ),
                )
            }
            CLAIM_PLATFORM_FEE_IX_DISCM => {
                Ok(
                    Self::ClaimPlatformFee(
                        ClaimPlatformFeeIxArgs::deserialize(&mut reader)?,
                    ),
                )
            }
            CLAIM_VESTED_TOKEN_IX_DISCM => {
                Ok(
                    Self::ClaimVestedToken(
                        ClaimVestedTokenIxArgs::deserialize(&mut reader)?,
                    ),
                )
            }
            COLLECT_FEE_IX_DISCM => {
                Ok(Self::CollectFee(CollectFeeIxArgs::deserialize(&mut reader)?))
            }
            COLLECT_MIGRATE_FEE_IX_DISCM => {
                Ok(Self::CollectMigrateFee(CollectMigrateFeeIxArgs::deserialize(&mut reader)?))
            }
            CREATE_CONFIG_IX_DISCM => {
                Ok(Self::CreateConfig(CreateConfigIxArgs::deserialize(&mut reader)?))
            }
            CREATE_PLATFORM_CONFIG_IX_DISCM => {
                Ok(Self::CreatePlatformConfig(CreatePlatformConfigIxArgs::deserialize(&mut reader)?))
            }
            CREATE_VESTING_ACCOUNT_IX_DISCM => {
                Ok(Self::CreateVestingAccount(CreateVestingAccountIxArgs::deserialize(&mut reader)?))
            }
            INITIALIZE_IX_DISCM => {
                Ok(Self::Initialize(InitializeIxArgs::deserialize(&mut reader)?))
            }
            MIGRATE_TO_AMM_IX_DISCM => {
                Ok(Self::MigrateToAmm(MigrateToAmmIxArgs::deserialize(&mut reader)?))
            }
            MIGRATE_TO_CPSWAP_IX_DISCM => {
                Ok(Self::MigrateToCpswap(MigrateToCpswapIxArgs::deserialize(&mut reader)?))
            }
            SELL_EXACT_IN_IX_DISCM => {
                Ok(Self::SellExactIn(SellExactInIxArgs::deserialize(&mut reader)?))
            }
            SELL_EXACT_OUT_IX_DISCM => {
                Ok(Self::SellExactOut(SellExactOutIxArgs::deserialize(&mut reader)?))
            }
            UPDATE_CONFIG_IX_DISCM => {
                Ok(Self::UpdateConfig(UpdateConfigIxArgs::deserialize(&mut reader)?))
            }
            UPDATE_PLATFORM_CONFIG_IX_DISCM => {
                Ok(Self::UpdatePlatformConfig(UpdatePlatformConfigIxArgs::deserialize(&mut reader)?))
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
            Self::BuyExactIn(args) => {
                writer.write_all(&BUY_EXACT_IN_IX_DISCM)?;
                args.serialize(&mut writer)
            }
            Self::BuyExactOut(args) => {
                writer.write_all(&BUY_EXACT_OUT_IX_DISCM)?;
                args.serialize(&mut writer)
            }
            Self::ClaimPlatformFee(args) => {
                writer.write_all(&CLAIM_PLATFORM_FEE_IX_DISCM)?;
                args.serialize(&mut writer)
            }
            Self::ClaimVestedToken(args) => {
                writer.write_all(&CLAIM_VESTED_TOKEN_IX_DISCM)?;
                args.serialize(&mut writer)
            }
            Self::CollectFee(args) => {
                writer.write_all(&COLLECT_FEE_IX_DISCM)?;
                args.serialize(&mut writer)
            }
            Self::CollectMigrateFee(args) => {
                writer.write_all(&COLLECT_MIGRATE_FEE_IX_DISCM)?;
                args.serialize(&mut writer)
            }
            Self::CreateConfig(args) => {
                writer.write_all(&CREATE_CONFIG_IX_DISCM)?;
                args.serialize(&mut writer)
            }
            Self::CreatePlatformConfig(args) => {
                writer.write_all(&CREATE_PLATFORM_CONFIG_IX_DISCM)?;
                args.serialize(&mut writer)
            }
            Self::CreateVestingAccount(args) => {
                writer.write_all(&CREATE_VESTING_ACCOUNT_IX_DISCM)?;
                args.serialize(&mut writer)
            }
            Self::Initialize(args) => {
                writer.write_all(&INITIALIZE_IX_DISCM)?;
                args.serialize(&mut writer)
            }
            Self::MigrateToAmm(args) => {
                writer.write_all(&MIGRATE_TO_AMM_IX_DISCM)?;
                args.serialize(&mut writer)
            }
            Self::MigrateToCpswap(args) => {
                writer.write_all(&MIGRATE_TO_CPSWAP_IX_DISCM)?;
                args.serialize(&mut writer)
            }
            Self::SellExactIn(args) => {
                writer.write_all(&SELL_EXACT_IN_IX_DISCM)?;
                args.serialize(&mut writer)
            }
            Self::SellExactOut(args) => {
                writer.write_all(&SELL_EXACT_OUT_IX_DISCM)?;
                args.serialize(&mut writer)
            }
            Self::UpdateConfig(args) => {
                writer.write_all(&UPDATE_CONFIG_IX_DISCM)?;
                args.serialize(&mut writer)
            }
            Self::UpdatePlatformConfig(args) => {
                writer.write_all(&UPDATE_PLATFORM_CONFIG_IX_DISCM)?;
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
pub const BUY_EXACT_IN_IX_ACCOUNTS_LEN: usize = 15;
#[derive(Copy, Clone, Debug)]
pub struct BuyExactInAccounts<'me, 'info> {
    pub payer: &'me AccountInfo<'info>,
    pub authority: &'me AccountInfo<'info>,
    pub global_config: &'me AccountInfo<'info>,
    pub platform_config: &'me AccountInfo<'info>,
    pub pool_state: &'me AccountInfo<'info>,
    pub user_base_token: &'me AccountInfo<'info>,
    pub user_quote_token: &'me AccountInfo<'info>,
    pub base_vault: &'me AccountInfo<'info>,
    pub quote_vault: &'me AccountInfo<'info>,
    pub base_token_mint: &'me AccountInfo<'info>,
    pub quote_token_mint: &'me AccountInfo<'info>,
    pub base_token_program: &'me AccountInfo<'info>,
    pub quote_token_program: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct BuyExactInKeys {
    pub payer: Pubkey,
    pub authority: Pubkey,
    pub global_config: Pubkey,
    pub platform_config: Pubkey,
    pub pool_state: Pubkey,
    pub user_base_token: Pubkey,
    pub user_quote_token: Pubkey,
    pub base_vault: Pubkey,
    pub quote_vault: Pubkey,
    pub base_token_mint: Pubkey,
    pub quote_token_mint: Pubkey,
    pub base_token_program: Pubkey,
    pub quote_token_program: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}

impl From<BuyExactInAccounts<'_, '_>> for BuyExactInKeys {
    fn from(accounts: BuyExactInAccounts) -> Self {
        Self {
            payer: *accounts.payer.key,
            authority: *accounts.authority.key,
            global_config: *accounts.global_config.key,
            platform_config: *accounts.platform_config.key,
            pool_state: *accounts.pool_state.key,
            user_base_token: *accounts.user_base_token.key,
            user_quote_token: *accounts.user_quote_token.key,
            base_vault: *accounts.base_vault.key,
            quote_vault: *accounts.quote_vault.key,
            base_token_mint: *accounts.base_token_mint.key,
            quote_token_mint: *accounts.quote_token_mint.key,
            base_token_program: *accounts.base_token_program.key,
            quote_token_program: *accounts.quote_token_program.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}

impl From<BuyExactInKeys> for [AccountMeta; BUY_EXACT_IN_IX_ACCOUNTS_LEN] {
    fn from(keys: BuyExactInKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.payer,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.global_config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.platform_config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pool_state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_base_token,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_quote_token,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.base_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.quote_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.base_token_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.quote_token_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.base_token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.quote_token_program,
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

impl From<[Pubkey; BUY_EXACT_IN_IX_ACCOUNTS_LEN]> for BuyExactInKeys {
    fn from(pubkeys: [Pubkey; BUY_EXACT_IN_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            payer: pubkeys[0],
            authority: pubkeys[1],
            global_config: pubkeys[2],
            platform_config: pubkeys[3],
            pool_state: pubkeys[4],
            user_base_token: pubkeys[5],
            user_quote_token: pubkeys[6],
            base_vault: pubkeys[7],
            quote_vault: pubkeys[8],
            base_token_mint: pubkeys[9],
            quote_token_mint: pubkeys[10],
            base_token_program: pubkeys[11],
            quote_token_program: pubkeys[12],
            event_authority: pubkeys[13],
            program: pubkeys[14],
        }
    }
}

impl<'info> From<BuyExactInAccounts<'_, 'info>> for [AccountInfo<'info>; BUY_EXACT_IN_IX_ACCOUNTS_LEN] {
    fn from(accounts: BuyExactInAccounts<'_, 'info>) -> Self {
        [
            accounts.payer.clone(),
            accounts.authority.clone(),
            accounts.global_config.clone(),
            accounts.platform_config.clone(),
            accounts.pool_state.clone(),
            accounts.user_base_token.clone(),
            accounts.user_quote_token.clone(),
            accounts.base_vault.clone(),
            accounts.quote_vault.clone(),
            accounts.base_token_mint.clone(),
            accounts.quote_token_mint.clone(),
            accounts.base_token_program.clone(),
            accounts.quote_token_program.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}

impl<'me, 'info> From<&'me [AccountInfo<'info>; BUY_EXACT_IN_IX_ACCOUNTS_LEN]> for BuyExactInAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; BUY_EXACT_IN_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            payer: &arr[0],
            authority: &arr[1],
            global_config: &arr[2],
            platform_config: &arr[3],
            pool_state: &arr[4],
            user_base_token: &arr[5],
            user_quote_token: &arr[6],
            base_vault: &arr[7],
            quote_vault: &arr[8],
            base_token_mint: &arr[9],
            quote_token_mint: &arr[10],
            base_token_program: &arr[11],
            quote_token_program: &arr[12],
            event_authority: &arr[13],
            program: &arr[14],
        }
    }
}

pub const BUY_EXACT_IN_IX_DISCM: [u8; 8] = [250, 234, 13, 123, 213, 156, 19, 236];

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct BuyExactInIxArgs {
    pub amount_in: u64,
    pub minimum_amount_out: u64,
    pub share_fee_rate: u64,
}

#[derive(Clone, Debug, PartialEq)]
pub struct BuyExactInIxData(pub BuyExactInIxArgs);

impl From<BuyExactInIxArgs> for BuyExactInIxData {
    fn from(args: BuyExactInIxArgs) -> Self {
        Self(args)
    }
}

impl BuyExactInIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != BUY_EXACT_IN_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    BUY_EXACT_IN_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(BuyExactInIxArgs::deserialize(&mut reader)?))
    }

    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&BUY_EXACT_IN_IX_DISCM)?;
        self.0.serialize(&mut writer)
    }

    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}

pub fn buy_exact_in_ix_with_program_id(
    program_id: Pubkey,
    keys: BuyExactInKeys,
    args: BuyExactInIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; BUY_EXACT_IN_IX_ACCOUNTS_LEN] = keys.into();
    let data: BuyExactInIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}

pub fn buy_exact_in_ix(
    keys: BuyExactInKeys,
    args: BuyExactInIxArgs,
) -> std::io::Result<Instruction> {
    buy_exact_in_ix_with_program_id(crate::ID, keys, args)
}

pub fn buy_exact_in_invoke_with_program_id(
    program_id: Pubkey,
    accounts: BuyExactInAccounts<'_, '_>,
    args: BuyExactInIxArgs,
) -> ProgramResult {
    let keys: BuyExactInKeys = accounts.into();
    let ix = buy_exact_in_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}

pub fn buy_exact_in_invoke(
    accounts: BuyExactInAccounts<'_, '_>,
    args: BuyExactInIxArgs,
) -> ProgramResult {
    buy_exact_in_invoke_with_program_id(crate::ID, accounts, args)
}

pub fn buy_exact_in_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: BuyExactInAccounts<'_, '_>,
    args: BuyExactInIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: BuyExactInKeys = accounts.into();
    let ix = buy_exact_in_ix_with_program_id(program_id,keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}

pub fn buy_exact_in_invoke_signed(
    accounts: BuyExactInAccounts<'_, '_>,
    args: BuyExactInIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    buy_exact_in_invoke_signed_with_program_id(
        crate::ID,
        accounts,
        args,
        seeds,
    )
}

pub fn buy_exact_in_verify_account_keys(
    accounts: BuyExactInAccounts<'_, '_>,
    keys: BuyExactInKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.payer.key, keys.payer),
        (*accounts.authority.key, keys.authority),
        (*accounts.global_config.key, keys.global_config),
        (*accounts.platform_config.key, keys.platform_config),
        (*accounts.pool_state.key, keys.pool_state),
        (*accounts.user_base_token.key, keys.user_base_token),
        (*accounts.user_quote_token.key, keys.user_quote_token),
        (*accounts.base_vault.key, keys.base_vault),
        (*accounts.quote_vault.key, keys.quote_vault),
        (*accounts.base_token_mint.key, keys.base_token_mint),
        (*accounts.quote_token_mint.key, keys.quote_token_mint),
        (*accounts.base_token_program.key, keys.base_token_program),
        (*accounts.quote_token_program.key, keys.quote_token_program),
        (*accounts.event_authority.key, keys.event_authority),
        (*accounts.program.key, keys.program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}

pub fn buy_exact_in_verify_writable_privileges<'me, 'info>(
    accounts: BuyExactInAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.pool_state,
        accounts.user_base_token,
        accounts.user_quote_token,
        accounts.base_vault,
        accounts.quote_vault,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}

pub fn buy_exact_in_verify_signer_privileges<'me, 'info>(
    accounts: BuyExactInAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.payer] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}

pub fn buy_exact_in_verify_account_privileges<'me, 'info>(
    accounts: BuyExactInAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    buy_exact_in_verify_writable_privileges(accounts)?;
    buy_exact_in_verify_signer_privileges(accounts)?;
    Ok(())
}

pub const BUY_EXACT_OUT_IX_ACCOUNTS_LEN: usize = 15;

#[derive(Copy, Clone, Debug)]
pub struct BuyExactOutAccounts<'me, 'info> {
    pub payer: &'me AccountInfo<'info>,
    pub authority: &'me AccountInfo<'info>,
    pub global_config: &'me AccountInfo<'info>,
    pub platform_config: &'me AccountInfo<'info>,
    pub pool_state: &'me AccountInfo<'info>,
    pub user_base_token: &'me AccountInfo<'info>,
    pub user_quote_token: &'me AccountInfo<'info>,
    pub base_vault: &'me AccountInfo<'info>,
    pub quote_vault: &'me AccountInfo<'info>,
    pub base_token_mint: &'me AccountInfo<'info>,
    pub quote_token_mint: &'me AccountInfo<'info>,
    pub base_token_program: &'me AccountInfo<'info>,
    pub quote_token_program: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct BuyExactOutKeys {
    pub payer: Pubkey,
    pub authority: Pubkey,
    pub global_config: Pubkey,
    pub platform_config: Pubkey,
    pub pool_state: Pubkey,
    pub user_base_token: Pubkey,
    pub user_quote_token: Pubkey,
    pub base_vault: Pubkey,
    pub quote_vault: Pubkey,
    pub base_token_mint: Pubkey,
    pub quote_token_mint: Pubkey,
    pub base_token_program: Pubkey,
    pub quote_token_program: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}

impl From<BuyExactOutAccounts<'_, '_>> for BuyExactOutKeys {
    fn from(accounts: BuyExactOutAccounts) -> Self {
        Self {
            payer: *accounts.payer.key,
            authority: *accounts.authority.key,
            global_config: *accounts.global_config.key,
            platform_config: *accounts.platform_config.key,
            pool_state: *accounts.pool_state.key,
            user_base_token: *accounts.user_base_token.key,
            user_quote_token: *accounts.user_quote_token.key,
            base_vault: *accounts.base_vault.key,
            quote_vault: *accounts.quote_vault.key,
            base_token_mint: *accounts.base_token_mint.key,
            quote_token_mint: *accounts.quote_token_mint.key,
            base_token_program: *accounts.base_token_program.key,
            quote_token_program: *accounts.quote_token_program.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}

impl From<BuyExactOutKeys> for [AccountMeta; BUY_EXACT_OUT_IX_ACCOUNTS_LEN] {
    fn from(keys: BuyExactOutKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.payer,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.global_config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.platform_config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pool_state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_base_token,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_quote_token,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.base_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.quote_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.base_token_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.quote_token_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.base_token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.quote_token_program,
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

impl From<[Pubkey; BUY_EXACT_OUT_IX_ACCOUNTS_LEN]> for BuyExactOutKeys {
    fn from(pubkeys: [Pubkey; BUY_EXACT_OUT_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            payer: pubkeys[0],
            authority: pubkeys[1],
            global_config: pubkeys[2],
            platform_config: pubkeys[3],
            pool_state: pubkeys[4],
            user_base_token: pubkeys[5],
            user_quote_token: pubkeys[6],
            base_vault: pubkeys[7],
            quote_vault: pubkeys[8],
            base_token_mint: pubkeys[9],
            quote_token_mint: pubkeys[10],
            base_token_program: pubkeys[11],
            quote_token_program: pubkeys[12],
            event_authority: pubkeys[13],
            program: pubkeys[14],
        }
    }
}

impl<'info> From<BuyExactOutAccounts<'_, 'info>> for [AccountInfo<'info>; BUY_EXACT_OUT_IX_ACCOUNTS_LEN] {
    fn from(accounts: BuyExactOutAccounts<'_, 'info>) -> Self {
        [
            accounts.payer.clone(),
            accounts.authority.clone(),
            accounts.global_config.clone(),
            accounts.platform_config.clone(),
            accounts.pool_state.clone(),
            accounts.user_base_token.clone(),
            accounts.user_quote_token.clone(),
            accounts.base_vault.clone(),
            accounts.quote_vault.clone(),
            accounts.base_token_mint.clone(),
            accounts.quote_token_mint.clone(),
            accounts.base_token_program.clone(),
            accounts.quote_token_program.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}

impl<'me, 'info> From<&'me [AccountInfo<'info>; BUY_EXACT_OUT_IX_ACCOUNTS_LEN]> for BuyExactOutAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; BUY_EXACT_OUT_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            payer: &arr[0],
            authority: &arr[1],
            global_config: &arr[2],
            platform_config: &arr[3],
            pool_state: &arr[4],
            user_base_token: &arr[5],
            user_quote_token: &arr[6],
            base_vault: &arr[7],
            quote_vault: &arr[8],
            base_token_mint: &arr[9],
            quote_token_mint: &arr[10],
            base_token_program: &arr[11],
            quote_token_program: &arr[12],
            event_authority: &arr[13],
            program: &arr[14],
        }
    }
}

pub const BUY_EXACT_OUT_IX_DISCM: [u8; 8] = [24, 211, 116, 40, 105, 3, 153, 56];

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct BuyExactOutIxArgs {
    pub amount_out: u64,
    pub maximum_amount_in: u64,
    pub share_fee_rate: u64,
}

#[derive(Clone, Debug, PartialEq)]
pub struct BuyExactOutIxData(pub BuyExactOutIxArgs);

impl From<BuyExactOutIxArgs> for BuyExactOutIxData {
    fn from(args: BuyExactOutIxArgs) -> Self {
        Self(args)
    }
}

impl BuyExactOutIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != BUY_EXACT_OUT_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    BUY_EXACT_OUT_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(BuyExactOutIxArgs::deserialize(&mut reader)?))
    }

    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&BUY_EXACT_OUT_IX_DISCM)?;
        self.0.serialize(&mut writer)
    }

    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}

pub fn buy_exact_out_ix_with_program_id(
    program_id: Pubkey,
    keys: BuyExactOutKeys,
    args: BuyExactOutIxArgs,
) -> std::io::Result<Instruction>{
    let metas: [AccountMeta; BUY_EXACT_OUT_IX_ACCOUNTS_LEN] = keys.into();
    let data: BuyExactOutIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}

pub fn buy_exact_out_ix(
    keys: BuyExactOutKeys,
    args: BuyExactOutIxArgs,
) -> std::io::Result<Instruction>{
    buy_exact_out_ix_with_program_id(crate::ID, keys, args)
}

pub fn buy_exact_out_invoke_with_program_id(
    program_id: Pubkey,
    accounts: BuyExactOutAccounts<'_, '_>,
    args: BuyExactOutIxArgs,
) -> ProgramResult {
    let keys: BuyExactOutKeys = accounts.into();
    let ix = buy_exact_out_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}

pub fn buy_exact_out_invoke(
    accounts: BuyExactOutAccounts<'_, '_>,
    args: BuyExactOutIxArgs,
) -> ProgramResult {
    buy_exact_out_invoke_with_program_id(crate::ID, accounts, args)
}

pub fn buy_exact_out_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: BuyExactOutAccounts<'_, '_>,
    args: BuyExactOutIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: BuyExactOutKeys = accounts.into();
    let ix = buy_exact_out_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}

pub fn buy_exact_out_invoke_signed(
    accounts: BuyExactOutAccounts<'_, '_>,
    args: BuyExactOutIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    buy_exact_out_invoke_signed_with_program_id(crate::ID, accounts, args, seeds)
}

pub fn buy_exact_out_verify_account_keys(
    accounts: BuyExactOutAccounts<'_, '_>,
    keys: BuyExactOutKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.payer.key, keys.payer),
        (*accounts.authority.key, keys.authority),
        (*accounts.global_config.key, keys.global_config),
        (*accounts.platform_config.key, keys.platform_config),
        (*accounts.pool_state.key, keys.pool_state),
        (*accounts.user_base_token.key, keys.user_base_token),
        (*accounts.user_quote_token.key, keys.user_quote_token),
        (*accounts.base_vault.key, keys.base_vault),
        (*accounts.quote_vault.key, keys.quote_vault),
        (*accounts.base_token_mint.key, keys.base_token_mint),
        (*accounts.quote_token_mint.key, keys.quote_token_mint),
        (*accounts.base_token_program.key, keys.base_token_program),
        (*accounts.quote_token_program.key, keys.quote_token_program),
        (*accounts.event_authority.key, keys.event_authority),
        (*accounts.program.key, keys.program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}

pub fn buy_exact_out_verify_writable_privileges<'me, 'info>(
    accounts: BuyExactOutAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.pool_state,
        accounts.user_base_token,
        accounts.user_quote_token,
        accounts.base_vault,
        accounts.quote_vault,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}

pub fn buy_exact_out_verify_signer_privileges<'me, 'info>(
    accounts: BuyExactOutAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.payer] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}

pub fn buy_exact_out_verify_account_privileges<'me, 'info>(
    accounts: BuyExactOutAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    buy_exact_out_verify_writable_privileges(accounts)?;
    buy_exact_out_verify_signer_privileges(accounts)?;
    Ok(())
}

pub const CLAIM_CREATOR_FEE_IX_ACCOUNTS_LEN: usize = 8;

#[derive(Copy, Clone, Debug)]
pub struct ClaimCreatorFeeAccounts<'me, 'info> {
    pub creator: &'me AccountInfo<'info>,
    pub fee_vault_authority: &'me AccountInfo<'info>,
    pub creator_fee_vault: &'me AccountInfo<'info>,
    pub recipient_token_account: &'me AccountInfo<'info>,
    pub quote_mint: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub associated_token_program: &'me AccountInfo<'info>,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct ClaimCreatorFeeKeys {
    pub creator: Pubkey,
    pub fee_vault_authority: Pubkey,
    pub creator_fee_vault: Pubkey,
    pub recipient_token_account: Pubkey,
    pub quote_mint: Pubkey,
    pub token_program: Pubkey,
    pub system_program: Pubkey,
    pub associated_token_program: Pubkey,
}

impl From<ClaimCreatorFeeAccounts<'_, '_>> for ClaimCreatorFeeKeys {
    fn from(accounts: ClaimCreatorFeeAccounts) -> Self {
        Self {
            creator: *accounts.creator.key,
            fee_vault_authority: *accounts.fee_vault_authority.key,
            creator_fee_vault: *accounts.creator_fee_vault.key,
            recipient_token_account: *accounts.recipient_token_account.key,
            quote_mint: *accounts.quote_mint.key,
            token_program: *accounts.token_program.key,
            system_program: *accounts.system_program.key,
            associated_token_program: *accounts.associated_token_program.key,
        }
    }
}

impl From<ClaimCreatorFeeKeys> for [AccountMeta; CLAIM_CREATOR_FEE_IX_ACCOUNTS_LEN] {
    fn from(keys: ClaimCreatorFeeKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.creator,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.fee_vault_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.creator_fee_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.recipient_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.quote_mint,
                is_signer: false,
                is_writable: false,
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
                pubkey: keys.associated_token_program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}

impl From<[Pubkey; CLAIM_CREATOR_FEE_IX_ACCOUNTS_LEN]> for ClaimCreatorFeeKeys {
    fn from(pubkeys: [Pubkey; CLAIM_CREATOR_FEE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            creator: pubkeys[0],
            fee_vault_authority: pubkeys[1],
            creator_fee_vault: pubkeys[2],
            recipient_token_account: pubkeys[3],
            quote_mint: pubkeys[4],
            token_program: pubkeys[5],
            system_program: pubkeys[6],
            associated_token_program: pubkeys[7],
        }
    }
}

impl<'info> From<ClaimCreatorFeeAccounts<'_, 'info>> for [AccountInfo<'info>; CLAIM_CREATOR_FEE_IX_ACCOUNTS_LEN] {
    fn from(accounts: ClaimCreatorFeeAccounts<'_, 'info>) -> Self {
        [
            accounts.creator.clone(),
            accounts.fee_vault_authority.clone(),
            accounts.creator_fee_vault.clone(),
            accounts.recipient_token_account.clone(),
            accounts.quote_mint.clone(),
            accounts.token_program.clone(),
            accounts.system_program.clone(),
            accounts.associated_token_program.clone(),
        ]
    }
}

impl<'me, 'info> From<&'me [AccountInfo<'info>; CLAIM_CREATOR_FEE_IX_ACCOUNTS_LEN]> for ClaimCreatorFeeAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; CLAIM_CREATOR_FEE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            creator: &arr[0],
            fee_vault_authority: &arr[1],
            creator_fee_vault: &arr[2],
            recipient_token_account: &arr[3],
            quote_mint: &arr[4],
            token_program: &arr[5],
            system_program: &arr[6],
            associated_token_program: &arr[7],
        }
    }
}

pub const CLAIM_CREATOR_FEE_IX_DISCM: [u8; 8] = [26, 97, 138, 203, 132, 171, 141, 252];

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ClaimCreatorFeeIxArgs {}

#[derive(Clone, Debug, PartialEq)]
pub struct ClaimCreatorFeeIxData(pub ClaimCreatorFeeIxArgs);

impl From<ClaimCreatorFeeIxArgs> for ClaimCreatorFeeIxData {
    fn from(args: ClaimCreatorFeeIxArgs) -> Self {
        Self(args)
    }
}

impl ClaimCreatorFeeIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != CLAIM_CREATOR_FEE_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    CLAIM_CREATOR_FEE_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(ClaimCreatorFeeIxArgs::deserialize(&mut reader)?))
    }

    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&CLAIM_CREATOR_FEE_IX_DISCM)?;
        self.0.serialize(&mut writer)
    }

    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}

pub fn claim_creator_fee_ix_with_program_id(
    program_id: Pubkey,
    keys: ClaimCreatorFeeKeys,
    args: ClaimCreatorFeeIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; CLAIM_CREATOR_FEE_IX_ACCOUNTS_LEN] = keys.into();
    let data: ClaimCreatorFeeIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}

pub fn claim_creator_fee_ix(
    keys: ClaimCreatorFeeKeys,
    args: ClaimCreatorFeeIxArgs,
) -> std::io::Result<Instruction> {
    claim_creator_fee_ix_with_program_id(crate::ID, keys, args)
}

pub fn claim_creator_fee_invoke_with_program_id(
    program_id: Pubkey,
    accounts: ClaimCreatorFeeAccounts<'_, '_>,
    args: ClaimCreatorFeeIxArgs,
) -> ProgramResult {
    let keys: ClaimCreatorFeeKeys = accounts.into();
    let ix = claim_creator_fee_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}

pub fn claim_creator_fee_invoke(
    accounts: ClaimCreatorFeeAccounts<'_, '_>,
    args: ClaimCreatorFeeIxArgs,
) -> ProgramResult {
    claim_creator_fee_invoke_with_program_id(crate::ID, accounts, args)
}

pub fn claim_creator_fee_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: ClaimCreatorFeeAccounts<'_, '_>,
    args: ClaimCreatorFeeIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: ClaimCreatorFeeKeys = accounts.into();
    let ix = claim_creator_fee_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}

pub fn claim_creator_fee_invoke_signed(
    accounts: ClaimCreatorFeeAccounts<'_, '_>,
    args: ClaimCreatorFeeIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    claim_creator_fee_invoke_signed_with_program_id(crate::ID, accounts, args, seeds)
}

pub fn claim_creator_fee_verify_account_keys(
    accounts: ClaimCreatorFeeAccounts<'_, '_>,
    keys: ClaimCreatorFeeKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.creator.key, keys.creator),
        (*accounts.fee_vault_authority.key, keys.fee_vault_authority),
        (*accounts.creator_fee_vault.key, keys.creator_fee_vault),
        (*accounts.recipient_token_account.key, keys.recipient_token_account),
        (*accounts.quote_mint.key, keys.quote_mint),
        (*accounts.token_program.key, keys.token_program),
        (*accounts.system_program.key, keys.system_program),
        (*accounts.associated_token_program.key, keys.associated_token_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}

pub fn claim_creator_fee_verify_writable_privileges<'me, 'info>(
    accounts: ClaimCreatorFeeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.creator,
        accounts.creator_fee_vault,
        accounts.recipient_token_account,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}

pub fn claim_creator_fee_verify_signer_privileges<'me, 'info>(
    accounts: ClaimCreatorFeeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.creator] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}

pub fn claim_creator_fee_verify_account_privileges<'me, 'info>(
    accounts: ClaimCreatorFeeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    claim_creator_fee_verify_writable_privileges(accounts)?;
    claim_creator_fee_verify_signer_privileges(accounts)?;
    Ok(())
}

pub const CLAIM_PLATFORM_FEE_IX_ACCOUNTS_LEN: usize = 10;
#[derive(Copy, Clone, Debug)]
pub struct ClaimPlatformFeeAccounts<'me, 'info> {
    pub platform_fee_wallet: &'me AccountInfo<'info>,
    pub authority: &'me AccountInfo<'info>,
    pub pool_state: &'me AccountInfo<'info>,
    pub platform_config: &'me AccountInfo<'info>,
    pub quote_vault: &'me AccountInfo<'info>,
    pub recipient_token_account: &'me AccountInfo<'info>,
    pub quote_mint: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub associated_token_program: &'me AccountInfo<'info>,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct ClaimPlatformFeeKeys {
    pub platform_fee_wallet: Pubkey,
    pub authority: Pubkey,
    pub pool_state: Pubkey,
    pub platform_config: Pubkey,
    pub quote_vault: Pubkey,
    pub recipient_token_account: Pubkey,
    pub quote_mint: Pubkey,
    pub token_program: Pubkey,
    pub system_program: Pubkey,
    pub associated_token_program: Pubkey,
}

impl From<ClaimPlatformFeeAccounts<'_, '_>> for ClaimPlatformFeeKeys {
    fn from(accounts: ClaimPlatformFeeAccounts) -> Self {
        Self {
            platform_fee_wallet: *accounts.platform_fee_wallet.key,
            authority: *accounts.authority.key,
            pool_state: *accounts.pool_state.key,
            platform_config: *accounts.platform_config.key,
            quote_vault: *accounts.quote_vault.key,
            recipient_token_account: *accounts.recipient_token_account.key,
            quote_mint: *accounts.quote_mint.key,
            token_program: *accounts.token_program.key,
            system_program: *accounts.system_program.key,
            associated_token_program: *accounts.associated_token_program.key,
        }
    }
}

impl From<ClaimPlatformFeeKeys> for [AccountMeta; CLAIM_PLATFORM_FEE_IX_ACCOUNTS_LEN] {
    fn from(keys: ClaimPlatformFeeKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.platform_fee_wallet,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pool_state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.platform_config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.quote_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.recipient_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.quote_mint,
                is_signer: false,
                is_writable: false,
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
                pubkey: keys.associated_token_program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}

impl From<[Pubkey; CLAIM_PLATFORM_FEE_IX_ACCOUNTS_LEN]> for ClaimPlatformFeeKeys {
    fn from(pubkeys: [Pubkey; CLAIM_PLATFORM_FEE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            platform_fee_wallet: pubkeys[0],
            authority: pubkeys[1],
            pool_state: pubkeys[2],
            platform_config: pubkeys[3],
            quote_vault: pubkeys[4],
            recipient_token_account: pubkeys[5],
            quote_mint: pubkeys[6],
            token_program: pubkeys[7],
            system_program: pubkeys[8],
            associated_token_program: pubkeys[9],
        }
    }
}

impl<'info> From<ClaimPlatformFeeAccounts<'_, 'info>> for [AccountInfo<'info>; CLAIM_PLATFORM_FEE_IX_ACCOUNTS_LEN] {
    fn from(accounts: ClaimPlatformFeeAccounts<'_, 'info>) -> Self {
        [
            accounts.platform_fee_wallet.clone(),
            accounts.authority.clone(),
            accounts.pool_state.clone(),
            accounts.platform_config.clone(),
            accounts.quote_vault.clone(),
            accounts.recipient_token_account.clone(),
            accounts.quote_mint.clone(),
            accounts.token_program.clone(),
            accounts.system_program.clone(),
            accounts.associated_token_program.clone(),
        ]
    }
}

impl<'me, 'info> From<&'me [AccountInfo<'info>; CLAIM_PLATFORM_FEE_IX_ACCOUNTS_LEN]> for ClaimPlatformFeeAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; CLAIM_PLATFORM_FEE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            platform_fee_wallet: &arr[0],
            authority: &arr[1],
            pool_state: &arr[2],
            platform_config: &arr[3],
            quote_vault: &arr[4],
            recipient_token_account: &arr[5],
            quote_mint: &arr[6],
            token_program: &arr[7],
            system_program: &arr[8],
            associated_token_program: &arr[9],
        }
    }
}

pub const CLAIM_PLATFORM_FEE_IX_DISCM: [u8; 8] = [156, 39, 208, 135, 76, 237, 61, 72];

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ClaimPlatformFeeIxArgs;

#[derive(Clone, Debug, PartialEq)]
pub struct ClaimPlatformFeeIxData(pub ClaimPlatformFeeIxArgs);

impl From<ClaimPlatformFeeIxArgs> for ClaimPlatformFeeIxData {
    fn from(args: ClaimPlatformFeeIxArgs) -> Self {
        Self(args)
    }
}

impl ClaimPlatformFeeIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != CLAIM_PLATFORM_FEE_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    CLAIM_PLATFORM_FEE_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(ClaimPlatformFeeIxArgs::deserialize(&mut reader)?))
    }

    pub fn  serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&CLAIM_PLATFORM_FEE_IX_DISCM)?;
        self.0.serialize(&mut writer)
    }

    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}

pub fn claim_platform_fee_ix_with_program_id(
    program_id: Pubkey,
    keys: ClaimPlatformFeeKeys,
    args: ClaimPlatformFeeIxArgs,
) -> std::io::Result<Instruction>{
    let metas: [AccountMeta; CLAIM_PLATFORM_FEE_IX_ACCOUNTS_LEN] = keys.into();
    let data: ClaimPlatformFeeIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}

pub fn claim_platform_fee_ix(
    keys: ClaimPlatformFeeKeys,
    args: ClaimPlatformFeeIxArgs,
) -> std::io::Result<Instruction>{
    claim_platform_fee_ix_with_program_id(crate::ID, keys, args)
}

pub fn claim_platform_fee_invoke_with_program_id(
    program_id: Pubkey,
    accounts: ClaimPlatformFeeAccounts<'_, '_>,
    args: ClaimPlatformFeeIxArgs,
) -> ProgramResult {
    let keys: ClaimPlatformFeeKeys = accounts.into();
    let ix = claim_platform_fee_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}

pub fn claim_platform_fee_invoke(
    accounts: ClaimPlatformFeeAccounts<'_, '_>,
    args: ClaimPlatformFeeIxArgs,
) -> ProgramResult {
    claim_platform_fee_invoke_with_program_id(crate::ID, accounts, args)
}

pub fn claim_platform_fee_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: ClaimPlatformFeeAccounts<'_, '_>,
    args: ClaimPlatformFeeIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: ClaimPlatformFeeKeys = accounts.into();
    let ix = claim_platform_fee_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}

pub fn claim_platform_fee_invoke_signed(
    accounts: ClaimPlatformFeeAccounts<'_, '_>,
    args: ClaimPlatformFeeIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    claim_platform_fee_invoke_signed_with_program_id(crate::ID, accounts, args, seeds)
}

pub fn claim_platform_fee_verify_account_keys(
    accounts: ClaimPlatformFeeAccounts<'_, '_>,
    keys: ClaimPlatformFeeKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.platform_fee_wallet.key, keys.platform_fee_wallet),
        (*accounts.authority.key, keys.authority),
        (*accounts.pool_state.key, keys.pool_state),
        (*accounts.platform_config.key, keys.platform_config),
        (*accounts.quote_vault.key, keys.quote_vault),
        (*accounts.recipient_token_account.key, keys.recipient_token_account),
        (*accounts.quote_mint.key, keys.quote_mint),
        (*accounts.token_program.key, keys.token_program),
        (*accounts.system_program.key, keys.system_program),
        (*accounts.associated_token_program.key, keys.associated_token_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}

pub fn claim_platform_fee_verify_writable_privileges<'me, 'info>(
    accounts: ClaimPlatformFeeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.platform_fee_wallet,
        accounts.pool_state,
        accounts.quote_vault,
        accounts.recipient_token_account,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}

pub fn claim_platform_fee_verify_signer_privileges<'me, 'info>(
    accounts: ClaimPlatformFeeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.platform_fee_wallet] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}

pub fn claim_platform_fee_verify_account_privileges<'me, 'info>(
    accounts: ClaimPlatformFeeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    claim_platform_fee_verify_writable_privileges(accounts)?;
    claim_platform_fee_verify_signer_privileges(accounts)?;
    Ok(())
}

pub const CLAIM_PLATFORM_FEE_FROM_VAULT_IX_ACCOUNTS_LEN: usize = 9;
#[derive(Copy, Clone, Debug)]
pub struct ClaimPlatformFeeFromVaultAccounts<'me, 'info> {
    pub platform_fee_wallet: &'me AccountInfo<'info>,
    pub fee_vault_authority: &'me AccountInfo<'info>,
    pub platform_config: &'me AccountInfo<'info>,
    pub platform_fee_vault: &'me AccountInfo<'info>,
    pub recipient_token_account: &'me AccountInfo<'info>,
    pub quote_mint: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub associated_token_program: &'me AccountInfo<'info>,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct ClaimPlatformFeeFromVaultKeys {
    pub platform_fee_wallet: Pubkey,
    pub fee_vault_authority: Pubkey,
    pub platform_config: Pubkey,
    pub platform_fee_vault: Pubkey,
    pub recipient_token_account: Pubkey,
    pub quote_mint: Pubkey,
    pub token_program: Pubkey,
    pub system_program: Pubkey,
    pub associated_token_program: Pubkey,
}

impl From<ClaimPlatformFeeFromVaultAccounts<'_, '_>> for ClaimPlatformFeeFromVaultKeys {
    fn from(accounts: ClaimPlatformFeeFromVaultAccounts) -> Self {
        Self {
            platform_fee_wallet: *accounts.platform_fee_wallet.key,
            fee_vault_authority: *accounts.fee_vault_authority.key,
            platform_config: *accounts.platform_config.key,
            platform_fee_vault: *accounts.platform_fee_vault.key,
            recipient_token_account: *accounts.recipient_token_account.key,
            quote_mint: *accounts.quote_mint.key,
            token_program: *accounts.token_program.key,
            system_program: *accounts.system_program.key,
            associated_token_program: *accounts.associated_token_program.key,
        }
    }
}

impl From<ClaimPlatformFeeFromVaultKeys> for [AccountMeta; CLAIM_PLATFORM_FEE_FROM_VAULT_IX_ACCOUNTS_LEN] {
    fn from(keys: ClaimPlatformFeeFromVaultKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.platform_fee_wallet,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.fee_vault_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.platform_config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.platform_fee_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.recipient_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.quote_mint,
                is_signer: false,
                is_writable: false,
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
                pubkey: keys.associated_token_program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}

impl From<[Pubkey; CLAIM_PLATFORM_FEE_FROM_VAULT_IX_ACCOUNTS_LEN]> for ClaimPlatformFeeFromVaultKeys {
    fn from(pubkeys: [Pubkey; CLAIM_PLATFORM_FEE_FROM_VAULT_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            platform_fee_wallet: pubkeys[0],
            fee_vault_authority: pubkeys[1],
            platform_config: pubkeys[2],
            platform_fee_vault: pubkeys[3],
            recipient_token_account: pubkeys[4],
            quote_mint: pubkeys[5],
            token_program: pubkeys[6],
            system_program: pubkeys[7],
            associated_token_program: pubkeys[8],
        }
    }
}

impl<'info> From<ClaimPlatformFeeFromVaultAccounts<'_, 'info>> for [AccountInfo<'info>; CLAIM_PLATFORM_FEE_FROM_VAULT_IX_ACCOUNTS_LEN] {
    fn from(accounts: ClaimPlatformFeeFromVaultAccounts<'_, 'info>) -> Self {
        [
            accounts.platform_fee_wallet.clone(),
            accounts.fee_vault_authority.clone(),
            accounts.platform_config.clone(),
            accounts.platform_fee_vault.clone(),
            accounts.recipient_token_account.clone(),
            accounts.quote_mint.clone(),
            accounts.token_program.clone(),
            accounts.system_program.clone(),
            accounts.associated_token_program.clone(),
        ]
    }
}

impl<'me, 'info> From<&'me [AccountInfo<'info>; CLAIM_PLATFORM_FEE_FROM_VAULT_IX_ACCOUNTS_LEN]> for ClaimPlatformFeeFromVaultAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; CLAIM_PLATFORM_FEE_FROM_VAULT_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            platform_fee_wallet: &arr[0],
            fee_vault_authority: &arr[1],
            platform_config: &arr[2],
            platform_fee_vault: &arr[3],
            recipient_token_account: &arr[4],
            quote_mint: &arr[5],
            token_program: &arr[6],
            system_program: &arr[7],
            associated_token_program: &arr[8],
        }
    }
}

pub const CLAIM_PLATFORM_FEE_FROM_VAULT_IX_DISCM: [u8; 8] = [117, 241, 198, 168, 248, 218, 80, 29];

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ClaimPlatformFeeFromVaultIxArgs {}

#[derive(Clone, Debug, PartialEq)]
pub struct ClaimPlatformFeeFromVaultIxData(pub ClaimPlatformFeeFromVaultIxArgs);

impl From<ClaimPlatformFeeFromVaultIxArgs> for ClaimPlatformFeeFromVaultIxData {
    fn from(args: ClaimPlatformFeeFromVaultIxArgs) -> Self {
        Self(args)
    }
}

impl ClaimPlatformFeeFromVaultIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != CLAIM_PLATFORM_FEE_FROM_VAULT_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    CLAIM_PLATFORM_FEE_FROM_VAULT_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(ClaimPlatformFeeFromVaultIxArgs::deserialize(&mut reader)?))
    }

    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&CLAIM_PLATFORM_FEE_FROM_VAULT_IX_DISCM)?;
        self.0.serialize(&mut writer)
    }

    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}

pub fn claim_platform_fee_from_vault_ix_with_program_id(
    program_id: Pubkey,
    keys: ClaimPlatformFeeFromVaultKeys,
    args: ClaimPlatformFeeFromVaultIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; CLAIM_PLATFORM_FEE_FROM_VAULT_IX_ACCOUNTS_LEN] = keys.into();
    let data: ClaimPlatformFeeFromVaultIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}

pub fn claim_platform_fee_from_vault_ix(
    keys: ClaimPlatformFeeFromVaultKeys,
    args: ClaimPlatformFeeFromVaultIxArgs,
) -> std::io::Result<Instruction> {
    claim_platform_fee_from_vault_ix_with_program_id(crate::ID, keys, args)
}

pub fn claim_platform_fee_from_vault_invoke_with_program_id(
    program_id: Pubkey,
    accounts: ClaimPlatformFeeFromVaultAccounts<'_, '_>,
    args: ClaimPlatformFeeFromVaultIxArgs,
) -> ProgramResult {
    let keys: ClaimPlatformFeeFromVaultKeys = accounts.into();
    let ix = claim_platform_fee_from_vault_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}

pub fn claim_platform_fee_from_vault_invoke(
    accounts: ClaimPlatformFeeFromVaultAccounts<'_, '_>,
    args: ClaimPlatformFeeFromVaultIxArgs,
) -> ProgramResult {
    claim_platform_fee_from_vault_invoke_with_program_id(crate::ID, accounts, args)
}

pub fn claim_platform_fee_from_vault_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: ClaimPlatformFeeFromVaultAccounts<'_, '_>,
    args: ClaimPlatformFeeFromVaultIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: ClaimPlatformFeeFromVaultKeys = accounts.into();
    let ix = claim_platform_fee_from_vault_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}

pub fn claim_platform_fee_from_vault_invoke_signed(
    accounts: ClaimPlatformFeeFromVaultAccounts<'_, '_>,
    args: ClaimPlatformFeeFromVaultIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    claim_platform_fee_from_vault_invoke_signed_with_program_id(crate::ID, accounts, args, seeds)
}

pub fn claim_platform_fee_from_vault_verify_account_keys(
    accounts: ClaimPlatformFeeFromVaultAccounts<'_, '_>,
    keys: ClaimPlatformFeeFromVaultKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.platform_fee_wallet.key, keys.platform_fee_wallet),
        (*accounts.fee_vault_authority.key, keys.fee_vault_authority),
        (*accounts.platform_config.key, keys.platform_config),
        (*accounts.platform_fee_vault.key, keys.platform_fee_vault),
        (*accounts.recipient_token_account.key, keys.recipient_token_account),
        (*accounts.quote_mint.key, keys.quote_mint),
        (*accounts.token_program.key, keys.token_program),
        (*accounts.system_program.key, keys.system_program),
        (*accounts.associated_token_program.key, keys.associated_token_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}

pub fn claim_platform_fee_from_vault_verify_writable_privileges<'me, 'info>(
    accounts: ClaimPlatformFeeFromVaultAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.platform_fee_wallet,
        accounts.platform_fee_vault,
        accounts.recipient_token_account,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}

pub fn claim_platform_fee_from_vault_verify_signer_privileges<'me, 'info>(
    accounts: ClaimPlatformFeeFromVaultAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.platform_fee_wallet] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}

pub fn claim_platform_fee_from_vault_verify_account_privileges<'me, 'info>(
    accounts: ClaimPlatformFeeFromVaultAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    claim_platform_fee_from_vault_verify_writable_privileges(accounts)?;
    claim_platform_fee_from_vault_verify_signer_privileges(accounts)?;
    Ok(())
}

pub const CLAIM_VESTED_TOKEN_IX_ACCOUNTS_LEN: usize = 10;

#[derive(Copy, Clone, Debug)]
pub struct ClaimVestedTokenAccounts<'me, 'info> {
    pub beneficiary: &'me AccountInfo<'info>,
    pub authority: &'me AccountInfo<'info>,
    pub pool_state: &'me AccountInfo<'info>,
    pub vesting_record: &'me AccountInfo<'info>,
    pub base_vault: &'me AccountInfo<'info>,
    pub user_base_token: &'me AccountInfo<'info>,
    pub base_token_mint: &'me AccountInfo<'info>,
    pub base_token_program: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub associated_token_program: &'me AccountInfo<'info>,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct ClaimVestedTokenKeys {
    pub beneficiary: Pubkey,
    pub authority: Pubkey,
    pub pool_state: Pubkey,
    pub vesting_record: Pubkey,
    pub base_vault: Pubkey,
    pub user_base_token: Pubkey,
    pub base_token_mint: Pubkey,
    pub base_token_program: Pubkey,
    pub system_program: Pubkey,
    pub associated_token_program: Pubkey,
}

impl From<ClaimVestedTokenAccounts<'_, '_>> for ClaimVestedTokenKeys {
    fn from(accounts: ClaimVestedTokenAccounts) -> Self {
        Self {
            beneficiary: *accounts.beneficiary.key,
            authority: *accounts.authority.key,
            pool_state: *accounts.pool_state.key,
            vesting_record: *accounts.vesting_record.key,
            base_vault: *accounts.base_vault.key,
            user_base_token: *accounts.user_base_token.key,
            base_token_mint: *accounts.base_token_mint.key,
            base_token_program: *accounts.base_token_program.key,
            system_program: *accounts.system_program.key,
            associated_token_program: *accounts.associated_token_program.key,
        }
    }
}

impl From<ClaimVestedTokenKeys> for [AccountMeta; CLAIM_VESTED_TOKEN_IX_ACCOUNTS_LEN] {
    fn from(keys: ClaimVestedTokenKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.beneficiary,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pool_state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.vesting_record,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.base_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_base_token,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.base_token_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.base_token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.system_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.associated_token_program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}

impl From<[Pubkey; CLAIM_VESTED_TOKEN_IX_ACCOUNTS_LEN]> for ClaimVestedTokenKeys {
    fn from(pubkeys: [Pubkey; CLAIM_VESTED_TOKEN_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            beneficiary: pubkeys[0],
            authority: pubkeys[1],
            pool_state: pubkeys[2],
            vesting_record: pubkeys[3],
            base_vault: pubkeys[4],
            user_base_token: pubkeys[5],
            base_token_mint: pubkeys[6],
            base_token_program: pubkeys[7],
            system_program: pubkeys[8],
            associated_token_program: pubkeys[9],
        }
    }
}

impl<'info> From<ClaimVestedTokenAccounts<'_, 'info>> for [AccountInfo<'info>; CLAIM_VESTED_TOKEN_IX_ACCOUNTS_LEN] {
    fn from(accounts: ClaimVestedTokenAccounts<'_, 'info>) -> Self {
        [
            accounts.beneficiary.clone(),
            accounts.authority.clone(),
            accounts.pool_state.clone(),
            accounts.vesting_record.clone(),
            accounts.base_vault.clone(),
            accounts.user_base_token.clone(),
            accounts.base_token_mint.clone(),
            accounts.base_token_program.clone(),
            accounts.system_program.clone(),
            accounts.associated_token_program.clone(),
        ]
    }
}

impl<'me, 'info> From<&'me [AccountInfo<'info>; CLAIM_VESTED_TOKEN_IX_ACCOUNTS_LEN]> for ClaimVestedTokenAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; CLAIM_VESTED_TOKEN_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            beneficiary: &arr[0],
            authority: &arr[1],
            pool_state: &arr[2],
            vesting_record: &arr[3],
            base_vault: &arr[4],
            user_base_token: &arr[5],
            base_token_mint: &arr[6],
            base_token_program: &arr[7],
            system_program: &arr[8],
            associated_token_program: &arr[9],
        }
    }
}

pub const CLAIM_VESTED_TOKEN_IX_DISCM: [u8; 8] = [49, 33, 104, 30, 189, 157, 79, 35];

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ClaimVestedTokenIxArgs;

#[derive(Clone, Debug, PartialEq)]
pub struct ClaimVestedTokenIxData(pub ClaimVestedTokenIxArgs);

impl From<ClaimVestedTokenIxArgs> for ClaimVestedTokenIxData {
    fn from(args: ClaimVestedTokenIxArgs) -> Self {
        Self(args)
    }
}

impl ClaimVestedTokenIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != CLAIM_VESTED_TOKEN_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    CLAIM_VESTED_TOKEN_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(ClaimVestedTokenIxArgs::deserialize(&mut reader)?))
    }

    pub fn  serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&CLAIM_VESTED_TOKEN_IX_DISCM)?;
        self.0.serialize(&mut writer)
    }

    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}

pub fn claim_vested_token_ix_with_program_id(
    program_id: Pubkey,
    keys: ClaimVestedTokenKeys,
    args: ClaimVestedTokenIxArgs,
) -> std::io::Result<Instruction>{
    let metas: [AccountMeta; CLAIM_VESTED_TOKEN_IX_ACCOUNTS_LEN] = keys.into();
    let data: ClaimVestedTokenIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}

pub fn claim_vested_token_ix(
    keys: ClaimVestedTokenKeys,
    args: ClaimVestedTokenIxArgs,
) -> std::io::Result<Instruction>{
    claim_vested_token_ix_with_program_id(crate::ID, keys, args)
}

pub fn claim_vested_token_invoke_with_program_id(
    program_id: Pubkey,
    accounts: ClaimVestedTokenAccounts<'_, '_>,
    args: ClaimVestedTokenIxArgs,
) -> ProgramResult {
    let keys: ClaimVestedTokenKeys = accounts.into();
    let ix = claim_vested_token_ix_with_program_id(program_id,keys, args)?;
    invoke_instruction(&ix, accounts)
}

pub fn claim_vested_token_invoke(
    accounts: ClaimVestedTokenAccounts<'_, '_>,
    args: ClaimVestedTokenIxArgs,
) -> ProgramResult {
    claim_vested_token_invoke_with_program_id(crate::ID, accounts, args)
}

pub fn claim_vested_token_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: ClaimVestedTokenAccounts<'_, '_>,
    args: ClaimVestedTokenIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: ClaimVestedTokenKeys = accounts.into();
    let ix = claim_vested_token_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}

pub fn claim_vested_token_invoke_signed(
    accounts: ClaimVestedTokenAccounts<'_, '_>,
    args: ClaimVestedTokenIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    claim_vested_token_invoke_signed_with_program_id(crate::ID, accounts, args, seeds)
}

pub fn claim_vested_token_verify_account_keys(
    accounts: ClaimVestedTokenAccounts<'_, '_>,
    keys: ClaimVestedTokenKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.beneficiary.key, keys.beneficiary),
        (*accounts.authority.key, keys.authority),
        (*accounts.pool_state.key, keys.pool_state),
        (*accounts.vesting_record.key, keys.vesting_record),
        (*accounts.base_vault.key, keys.base_vault),
        (*accounts.user_base_token.key, keys.user_base_token),
        (*accounts.base_token_mint.key, keys.base_token_mint),
        (*accounts.base_token_program.key, keys.base_token_program),
        (*accounts.system_program.key, keys.system_program),
        (*accounts.associated_token_program.key, keys.associated_token_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}

pub fn claim_vested_token_verify_writable_privileges<'me, 'info>(
    accounts: ClaimVestedTokenAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.beneficiary,
        accounts.pool_state,
        accounts.vesting_record,
        accounts.base_vault,
        accounts.user_base_token,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}

pub fn claim_vested_token_verify_signer_privileges<'me, 'info>(
    accounts: ClaimVestedTokenAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.beneficiary, accounts.user_base_token] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}

pub fn claim_vested_token_verify_account_privileges<'me, 'info>(
    accounts: ClaimVestedTokenAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    claim_vested_token_verify_writable_privileges(accounts)?;
    claim_vested_token_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const COLLECT_FEE_IX_ACCOUNTS_LEN: usize = 8;

#[derive(Copy, Clone, Debug)]
pub struct CollectFeeAccounts<'me, 'info> {
    pub owner: &'me AccountInfo<'info>,
    pub authority: &'me AccountInfo<'info>,
    pub pool_state: &'me AccountInfo<'info>,
    pub global_config: &'me AccountInfo<'info>,
    pub quote_vault: &'me AccountInfo<'info>,
    pub quote_mint: &'me AccountInfo<'info>,
    pub recipient_token_account: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CollectFeeKeys {
    pub owner: Pubkey,
    pub authority: Pubkey,
    pub pool_state: Pubkey,
    pub global_config: Pubkey,
    pub quote_vault: Pubkey,
    pub quote_mint: Pubkey,
    pub recipient_token_account: Pubkey,
    pub token_program: Pubkey,
}

impl From<CollectFeeAccounts<'_, '_>> for CollectFeeKeys {
    fn from(accounts: CollectFeeAccounts) -> Self {
        Self {
            owner: *accounts.owner.key,
            authority: *accounts.authority.key,
            pool_state: *accounts.pool_state.key,
            global_config: *accounts.global_config.key,
            quote_vault: *accounts.quote_vault.key,
            quote_mint: *accounts.quote_mint.key,
            recipient_token_account: *accounts.recipient_token_account.key,
            token_program: *accounts.token_program.key,
        }
    }
}

impl From<CollectFeeKeys> for [AccountMeta; COLLECT_FEE_IX_ACCOUNTS_LEN] {
    fn from(keys: CollectFeeKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.owner,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pool_state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.global_config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.quote_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.quote_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.recipient_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}

impl From<[Pubkey; COLLECT_FEE_IX_ACCOUNTS_LEN]> for CollectFeeKeys {
    fn from(pubkeys: [Pubkey; COLLECT_FEE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            owner: pubkeys[0],
            authority: pubkeys[1],
            pool_state: pubkeys[2],
            global_config: pubkeys[3],
            quote_vault: pubkeys[4],
            quote_mint: pubkeys[5],
            recipient_token_account: pubkeys[6],
            token_program: pubkeys[7],
        }
    }
}

impl<'info> From<CollectFeeAccounts<'_, 'info>> for [AccountInfo<'info>; COLLECT_FEE_IX_ACCOUNTS_LEN] {
    fn from(accounts: CollectFeeAccounts<'_, 'info>) -> Self {
        [
            accounts.owner.clone(),
            accounts.authority.clone(),
            accounts.pool_state.clone(),
            accounts.global_config.clone(),
            accounts.quote_vault.clone(),
            accounts.quote_mint.clone(),
            accounts.recipient_token_account.clone(),
            accounts.token_program.clone(),
        ]
    }
}

impl<'me, 'info> From<&'me [AccountInfo<'info>; COLLECT_FEE_IX_ACCOUNTS_LEN]> for CollectFeeAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; COLLECT_FEE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            owner: &arr[0],
            authority: &arr[1],
            pool_state: &arr[2],
            global_config: &arr[3],
            quote_vault: &arr[4],
            quote_mint: &arr[5],
            recipient_token_account: &arr[6],
            token_program: &arr[7],
        }
    }
}

pub const COLLECT_FEE_IX_DISCM: [u8; 8] = [60, 173, 247, 103, 4, 93, 130, 48];

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CollectFeeIxArgs;

#[derive(Clone, Debug, PartialEq)]
pub struct CollectFeeIxData(pub CollectFeeIxArgs);

impl From<CollectFeeIxArgs> for CollectFeeIxData {
    fn from(args: CollectFeeIxArgs) -> Self {
        Self(args)
    }
}

impl CollectFeeIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != COLLECT_FEE_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    COLLECT_FEE_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(CollectFeeIxArgs::deserialize(&mut reader)?))
    }

    pub fn  serialize<W: std::io::Write>(
        &self,
        mut writer: W) -> std::io::Result<()> {
            writer.write_all(&COLLECT_FEE_IX_DISCM)?;
        self.0.serialize(&mut writer)
    }

    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}

pub fn collect_fee_ix_with_program_id(
    program_id: Pubkey,
    keys: CollectFeeKeys,
    args: CollectFeeIxArgs,
) -> std::io::Result<Instruction>{
    let metas: [AccountMeta; COLLECT_FEE_IX_ACCOUNTS_LEN] = keys.into();
    let data: CollectFeeIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}

pub fn collect_fee_ix(
    keys: CollectFeeKeys,
    args: CollectFeeIxArgs,
) -> std::io::Result<Instruction>{
    collect_fee_ix_with_program_id(crate::ID, keys, args)
}

pub fn collect_fee_invoke_with_program_id(
    program_id: Pubkey,
    accounts: CollectFeeAccounts<'_, '_>,
    args: CollectFeeIxArgs,
) -> ProgramResult {
    let keys: CollectFeeKeys = accounts.into();
    let ix = collect_fee_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}

pub fn collect_fee_invoke(
    accounts: CollectFeeAccounts<'_, '_>,
    args: CollectFeeIxArgs,
) -> ProgramResult {
    collect_fee_invoke_with_program_id(crate::ID, accounts, args)
}

pub fn collect_fee_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: CollectFeeAccounts<'_, '_>,
    args: CollectFeeIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: CollectFeeKeys = accounts.into();
    let ix = collect_fee_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}

pub fn collect_fee_invoke_signed(
    accounts: CollectFeeAccounts<'_, '_>,
    args: CollectFeeIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    collect_fee_invoke_signed_with_program_id(crate::ID, accounts, args, seeds)
}

pub fn collect_fee_verify_account_keys(
    accounts: CollectFeeAccounts<'_, '_>,
    keys: CollectFeeKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.owner.key, keys.owner),
        (*accounts.authority.key, keys.authority),
        (*accounts.pool_state.key, keys.pool_state),
        (*accounts.global_config.key, keys.global_config),
        (*accounts.quote_vault.key, keys.quote_vault),
        (*accounts.quote_mint.key, keys.quote_mint),
        (*accounts.recipient_token_account.key, keys.recipient_token_account),
        (*accounts.token_program.key, keys.token_program),
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
        accounts.quote_vault,
        accounts.recipient_token_account,
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
    for should_be_signer in [accounts.owner] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
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
pub const COLLECT_MIGRATE_FEE_IX_ACCOUNTS_LEN: usize = 8;

#[derive(Copy, Clone, Debug)]
pub struct CollectMigrateFeeAccounts<'me, 'info> {
    pub owner: &'me AccountInfo<'info>,
    pub authority: &'me AccountInfo<'info>,
    pub pool_state: &'me AccountInfo<'info>,
    pub global_config: &'me AccountInfo<'info>,
    pub quote_vault: &'me AccountInfo<'info>,
    pub quote_mint: &'me AccountInfo<'info>,
    pub recipient_token_account: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CollectMigrateFeeKeys {
    pub owner: Pubkey,
    pub authority: Pubkey,
    pub pool_state: Pubkey,
    pub global_config: Pubkey,
    pub quote_vault: Pubkey,
    pub quote_mint: Pubkey,
    pub recipient_token_account: Pubkey,
    pub token_program: Pubkey,
}

impl From<CollectMigrateFeeAccounts<'_, '_>> for CollectMigrateFeeKeys {
    fn from(accounts: CollectMigrateFeeAccounts) -> Self {
        Self {
            owner: *accounts.owner.key,
            authority: *accounts.authority.key,
            pool_state: *accounts.pool_state.key,
            global_config: *accounts.global_config.key,
            quote_vault: *accounts.quote_vault.key,
            quote_mint: *accounts.quote_mint.key,
            recipient_token_account: *accounts.recipient_token_account.key,
            token_program: *accounts.token_program.key,
        }
    }
}

impl From<CollectMigrateFeeKeys> for [AccountMeta; COLLECT_MIGRATE_FEE_IX_ACCOUNTS_LEN] {
    fn from(keys: CollectMigrateFeeKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.owner,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pool_state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.global_config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.quote_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.quote_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.recipient_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}

impl From<[Pubkey; COLLECT_MIGRATE_FEE_IX_ACCOUNTS_LEN]> for CollectMigrateFeeKeys {
    fn from(pubkeys: [Pubkey; COLLECT_MIGRATE_FEE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            owner: pubkeys[0],
            authority: pubkeys[1],
            pool_state: pubkeys[2],
            global_config: pubkeys[3],
            quote_vault: pubkeys[4],
            quote_mint: pubkeys[5],
            recipient_token_account: pubkeys[6],
            token_program: pubkeys[7],
        }
    }
}

impl<'info> From<CollectMigrateFeeAccounts<'_, 'info>> for [AccountInfo<'info>; COLLECT_MIGRATE_FEE_IX_ACCOUNTS_LEN] {
    fn from(accounts: CollectMigrateFeeAccounts<'_, 'info>) -> Self {
        [
            accounts.owner.clone(),
            accounts.authority.clone(),
            accounts.pool_state.clone(),
            accounts.global_config.clone(),
            accounts.quote_vault.clone(),
            accounts.quote_mint.clone(),
            accounts.recipient_token_account.clone(),
            accounts.token_program.clone(),
        ]
    }
}

impl<'me, 'info> From<&'me [AccountInfo<'info>; COLLECT_MIGRATE_FEE_IX_ACCOUNTS_LEN]> for CollectMigrateFeeAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; COLLECT_MIGRATE_FEE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            owner: &arr[0],
            authority: &arr[1],
            pool_state: &arr[2],
            global_config: &arr[3],
            quote_vault: &arr[4],
            quote_mint: &arr[5],
            recipient_token_account: &arr[6],
            token_program: &arr[7],
        }
    }
}

pub const COLLECT_MIGRATE_FEE_IX_DISCM: [u8; 8] = [255, 186, 150, 223, 235, 118, 201, 186];

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CollectMigrateFeeIxArgs;

#[derive(Clone, Debug, PartialEq)]
pub struct CollectMigrateFeeIxData(pub CollectMigrateFeeIxArgs);

impl From<CollectMigrateFeeIxArgs> for CollectMigrateFeeIxData {
    fn from(args: CollectMigrateFeeIxArgs) -> Self {
        Self(args)
    }
}

impl CollectMigrateFeeIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != COLLECT_MIGRATE_FEE_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    COLLECT_MIGRATE_FEE_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(CollectMigrateFeeIxArgs::deserialize(&mut reader)?))
    }

    pub fn  serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&COLLECT_MIGRATE_FEE_IX_DISCM)?;
        self.0.serialize(&mut writer)
    }

    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}

pub fn collect_migrate_fee_ix_with_program_id(
    program_id: Pubkey,
    keys: CollectMigrateFeeKeys,
    args: CollectMigrateFeeIxArgs,
) -> std::io::Result<Instruction>{
    let metas: [AccountMeta; COLLECT_MIGRATE_FEE_IX_ACCOUNTS_LEN] = keys.into();
    let data: CollectMigrateFeeIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}

pub fn collect_migrate_fee_ix(
    keys: CollectMigrateFeeKeys,
    args: CollectMigrateFeeIxArgs,
) -> std::io::Result<Instruction>{
    collect_migrate_fee_ix_with_program_id(crate::ID, keys, args)
}

pub fn collect_migrate_fee_invoke_with_program_id(
    program_id: Pubkey,
    accounts: CollectMigrateFeeAccounts<'_, '_>,
    args: CollectMigrateFeeIxArgs,
) -> ProgramResult {
    let keys: CollectMigrateFeeKeys = accounts.into();
    let ix = collect_migrate_fee_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}

pub fn collect_migrate_fee_invoke(
    accounts: CollectMigrateFeeAccounts<'_, '_>,
    args: CollectMigrateFeeIxArgs,
) -> ProgramResult {
    collect_migrate_fee_invoke_with_program_id(crate::ID, accounts, args)
}

pub fn collect_migrate_fee_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: CollectMigrateFeeAccounts<'_, '_>,
    args: CollectMigrateFeeIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: CollectMigrateFeeKeys = accounts.into();
    let ix = collect_migrate_fee_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}

pub fn collect_migrate_fee_invoke_signed(
    accounts: CollectMigrateFeeAccounts<'_, '_>,
    args: CollectMigrateFeeIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    collect_migrate_fee_invoke_signed_with_program_id(crate::ID, accounts, args, seeds)
}

pub fn collect_migrate_fee_verify_account_keys(
    accounts: CollectMigrateFeeAccounts<'_, '_>,
    keys: CollectMigrateFeeKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.owner.key, keys.owner),
        (*accounts.authority.key, keys.authority),
        (*accounts.pool_state.key, keys.pool_state),
        (*accounts.global_config.key, keys.global_config),
        (*accounts.quote_vault.key, keys.quote_vault),
        (*accounts.quote_mint.key, keys.quote_mint),
        (*accounts.recipient_token_account.key, keys.recipient_token_account),
        (*accounts.token_program.key, keys.token_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}

pub fn collect_migrate_fee_verify_writable_privileges<'me, 'info>(
    accounts: CollectMigrateFeeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.pool_state,
        accounts.quote_vault,
        accounts.recipient_token_account,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}

pub fn collect_migrate_fee_verify_signer_privileges<'me, 'info>(
    accounts: CollectMigrateFeeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.owner] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}

pub fn collect_migrate_fee_verify_account_privileges<'me, 'info>(
    accounts: CollectMigrateFeeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    collect_migrate_fee_verify_writable_privileges(accounts)?;
    collect_migrate_fee_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const CREATE_CONFIG_IX_ACCOUNTS_LEN: usize = 8;

#[derive(Copy, Clone, Debug)]
pub struct CreateConfigAccounts<'me, 'info> {
    pub owner: &'me AccountInfo<'info>,
    pub global_config: &'me AccountInfo<'info>,
    pub quote_token_mint: &'me AccountInfo<'info>,
    pub protocol_fee_owner: &'me AccountInfo<'info>,
    pub migrate_fee_owner: &'me AccountInfo<'info>,
    pub migrate_to_amm_wallet: &'me AccountInfo<'info>,
    pub migrate_to_cpswap_wallet: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CreateConfigKeys {
    pub owner: Pubkey,
    pub global_config: Pubkey,
    pub quote_token_mint: Pubkey,
    pub protocol_fee_owner: Pubkey,
    pub migrate_fee_owner: Pubkey,
    pub migrate_to_amm_wallet: Pubkey,
    pub migrate_to_cpswap_wallet: Pubkey,
    pub system_program: Pubkey,
}

impl From<CreateConfigAccounts<'_, '_>> for CreateConfigKeys {
    fn from(accounts: CreateConfigAccounts) -> Self {
        Self {
            owner: *accounts.owner.key,
            global_config: *accounts.global_config.key,
            quote_token_mint: *accounts.quote_token_mint.key,
            protocol_fee_owner: *accounts.protocol_fee_owner.key,
            migrate_fee_owner: *accounts.migrate_fee_owner.key,
            migrate_to_amm_wallet: *accounts.migrate_to_amm_wallet.key,
            migrate_to_cpswap_wallet: *accounts.migrate_to_cpswap_wallet.key,
            system_program: *accounts.system_program.key,
        }
    }
}

impl From<CreateConfigKeys> for [AccountMeta; CREATE_CONFIG_IX_ACCOUNTS_LEN] {
    fn from(keys: CreateConfigKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.owner,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.global_config,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.quote_token_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.protocol_fee_owner,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.migrate_fee_owner,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.migrate_to_amm_wallet,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.migrate_to_cpswap_wallet,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.system_program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}

impl From<[Pubkey; CREATE_CONFIG_IX_ACCOUNTS_LEN]> for CreateConfigKeys {
    fn from(pubkeys: [Pubkey; CREATE_CONFIG_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            owner: pubkeys[0],
            global_config: pubkeys[1],
            quote_token_mint: pubkeys[2],
            protocol_fee_owner: pubkeys[3],
            migrate_fee_owner: pubkeys[4],
            migrate_to_amm_wallet: pubkeys[5],
            migrate_to_cpswap_wallet: pubkeys[6],
            system_program: pubkeys[7],
        }
    }
}

impl<'info> From<CreateConfigAccounts<'_, 'info>> for [AccountInfo<'info>; CREATE_CONFIG_IX_ACCOUNTS_LEN] {
    fn from(accounts: CreateConfigAccounts<'_, 'info>) -> Self {
        [
            accounts.owner.clone(),
            accounts.global_config.clone(),
            accounts.quote_token_mint.clone(),
            accounts.protocol_fee_owner.clone(),
            accounts.migrate_fee_owner.clone(),
            accounts.migrate_to_amm_wallet.clone(),
            accounts.migrate_to_cpswap_wallet.clone(),
            accounts.system_program.clone(),
        ]
    }
}

impl<'me, 'info> From<&'me [AccountInfo<'info>; CREATE_CONFIG_IX_ACCOUNTS_LEN]> for CreateConfigAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; CREATE_CONFIG_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            owner: &arr[0],
            global_config: &arr[1],
            quote_token_mint: &arr[2],
            protocol_fee_owner: &arr[3],
            migrate_fee_owner: &arr[4],
            migrate_to_amm_wallet: &arr[5],
            migrate_to_cpswap_wallet: &arr[6],
            system_program: &arr[7],
        }
    }
}

pub const CREATE_CONFIG_IX_DISCM: [u8; 8] = [201, 207, 243, 114, 75, 111, 47, 189];

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CreateConfigIxArgs {
    pub curve_type: u8,
    pub index: u16,
    pub migrate_fee: u64,
    pub trade_fee_rate: u64,
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
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    CREATE_CONFIG_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(CreateConfigIxArgs::deserialize(&mut reader)?))
    }

    pub fn  serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
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
) -> std::io::Result<Instruction>{
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
) -> std::io::Result<Instruction>{
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
        (*accounts.owner.key, keys.owner),
        (*accounts.global_config.key, keys.global_config),
        (*accounts.quote_token_mint.key, keys.quote_token_mint),
        (*accounts.protocol_fee_owner.key, keys.protocol_fee_owner),
        (*accounts.migrate_fee_owner.key, keys.migrate_fee_owner),
        (*accounts.migrate_to_amm_wallet.key, keys.migrate_to_amm_wallet),
        (*accounts.migrate_to_cpswap_wallet.key, keys.migrate_to_cpswap_wallet),
        (*accounts.system_program.key, keys.system_program),
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
    for should_be_writable in [accounts.owner, accounts.global_config] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}

pub fn create_config_verify_signer_privileges<'me, 'info>(
    accounts: CreateConfigAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.owner] {
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

pub const CREATE_PLATFORM_CONFIG_IX_ACCOUNTS_LEN: usize = 5;

#[derive(Copy, Clone, Debug)]
pub struct CreatePlatformConfigAccounts<'me, 'info> {
    pub platform_admin: &'me AccountInfo<'info>,
    pub platform_fee_wallet: &'me AccountInfo<'info>,
    pub platform_nft_wallet: &'me AccountInfo<'info>,
    pub platform_config: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CreatePlatformConfigKeys {
    pub platform_admin: Pubkey,
    pub platform_fee_wallet: Pubkey,
    pub platform_nft_wallet: Pubkey,
    pub platform_config: Pubkey,
    pub system_program: Pubkey,
}

impl From<CreatePlatformConfigAccounts<'_, '_>> for CreatePlatformConfigKeys {
    fn from(accounts: CreatePlatformConfigAccounts) -> Self {
        Self {
            platform_admin: *accounts.platform_admin.key,
            platform_fee_wallet: *accounts.platform_fee_wallet.key,
            platform_nft_wallet: *accounts.platform_nft_wallet.key,
            platform_config: *accounts.platform_config.key,
            system_program: *accounts.system_program.key,
        }
    }
}

impl From<CreatePlatformConfigKeys> for [AccountMeta; CREATE_PLATFORM_CONFIG_IX_ACCOUNTS_LEN] {
    fn from(keys: CreatePlatformConfigKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.platform_admin,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.platform_fee_wallet,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.platform_nft_wallet,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.platform_config,
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

impl From<[Pubkey; CREATE_PLATFORM_CONFIG_IX_ACCOUNTS_LEN]> for CreatePlatformConfigKeys {
    fn from(pubkeys: [Pubkey; CREATE_PLATFORM_CONFIG_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            platform_admin: pubkeys[0],
            platform_fee_wallet: pubkeys[1],
            platform_nft_wallet: pubkeys[2],
            platform_config: pubkeys[3],
            system_program: pubkeys[4],
        }
    }
}

impl<'info> From<CreatePlatformConfigAccounts<'_, 'info>> for [AccountInfo<'info>; CREATE_PLATFORM_CONFIG_IX_ACCOUNTS_LEN] {
    fn from(accounts: CreatePlatformConfigAccounts<'_, 'info>) -> Self {
        [
            accounts.platform_admin.clone(),
            accounts.platform_fee_wallet.clone(),
            accounts.platform_nft_wallet.clone(),
            accounts.platform_config.clone(),
            accounts.system_program.clone(),
        ]
    }
}

impl<'me, 'info> From<&'me [AccountInfo<'info>; CREATE_PLATFORM_CONFIG_IX_ACCOUNTS_LEN]> for CreatePlatformConfigAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; CREATE_PLATFORM_CONFIG_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            platform_admin: &arr[0],
            platform_fee_wallet: &arr[1],
            platform_nft_wallet: &arr[2],
            platform_config: &arr[3],
            system_program: &arr[4],
        }
    }
}

pub const CREATE_PLATFORM_CONFIG_IX_DISCM: [u8; 8] = [176, 90, 196, 175, 253, 113, 220, 20];

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CreatePlatformConfigIxArgs {
    pub platform_params: PlatformParams,
}

#[derive(Clone, Debug, PartialEq)]
pub struct CreatePlatformConfigIxData(pub CreatePlatformConfigIxArgs);

impl From<CreatePlatformConfigIxArgs> for CreatePlatformConfigIxData {
    fn from(args: CreatePlatformConfigIxArgs) -> Self {
        Self(args)
    }
}

impl CreatePlatformConfigIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != CREATE_PLATFORM_CONFIG_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    CREATE_PLATFORM_CONFIG_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(CreatePlatformConfigIxArgs::deserialize(&mut reader)?))
    }

    pub fn  serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&CREATE_PLATFORM_CONFIG_IX_DISCM)?;
        self.0.serialize(&mut writer)
    }

    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}

pub fn create_platform_config_ix_with_program_id(
    program_id: Pubkey,
    keys: CreatePlatformConfigKeys,
    args: CreatePlatformConfigIxArgs,
) -> std::io::Result<Instruction>{
    let metas: [AccountMeta; CREATE_PLATFORM_CONFIG_IX_ACCOUNTS_LEN] = keys.into();
    let data: CreatePlatformConfigIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}

pub fn create_platform_config_ix(
    keys: CreatePlatformConfigKeys,
    args: CreatePlatformConfigIxArgs,
) -> std::io::Result<Instruction>{
    create_platform_config_ix_with_program_id(crate::ID, keys, args)
}

pub fn create_platform_config_invoke_with_program_id(
    program_id: Pubkey,
    accounts: CreatePlatformConfigAccounts<'_, '_>,
    args: CreatePlatformConfigIxArgs,
) -> ProgramResult {
    let keys: CreatePlatformConfigKeys = accounts.into();
    let ix = create_platform_config_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}

pub fn create_platform_config_invoke(
    accounts: CreatePlatformConfigAccounts<'_, '_>,
    args: CreatePlatformConfigIxArgs,
) -> ProgramResult {
    create_platform_config_invoke_with_program_id(crate::ID, accounts, args)
}

pub fn create_platform_config_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: CreatePlatformConfigAccounts<'_, '_>,
    args: CreatePlatformConfigIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: CreatePlatformConfigKeys = accounts.into();
    let ix = create_platform_config_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}

pub fn create_platform_config_invoke_signed(
    accounts: CreatePlatformConfigAccounts<'_, '_>,
    args: CreatePlatformConfigIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    create_platform_config_invoke_signed_with_program_id(crate::ID, accounts, args, seeds)
}

pub fn create_platform_config_verify_account_keys(
    accounts: CreatePlatformConfigAccounts<'_, '_>,
    keys: CreatePlatformConfigKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.platform_admin.key, keys.platform_admin),
        (*accounts.platform_fee_wallet.key, keys.platform_fee_wallet),
        (*accounts.platform_nft_wallet.key, keys.platform_nft_wallet),
        (*accounts.platform_config.key, keys.platform_config),
        (*accounts.system_program.key, keys.system_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}

pub fn create_platform_config_verify_writable_privileges<'me, 'info>(
    accounts: CreatePlatformConfigAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.platform_admin, accounts.platform_config] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}

pub fn create_platform_config_verify_signer_privileges<'me, 'info>(
    accounts: CreatePlatformConfigAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.platform_admin] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}

pub fn create_platform_config_verify_account_privileges<'me, 'info>(
    accounts: CreatePlatformConfigAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    create_platform_config_verify_writable_privileges(accounts)?;
    create_platform_config_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const CREATE_VESTING_ACCOUNT_IX_ACCOUNTS_LEN: usize = 5;

#[derive(Copy, Clone, Debug)]
pub struct CreateVestingAccountAccounts<'me, 'info> {
    pub creator: &'me AccountInfo<'info>,
    pub beneficiary: &'me AccountInfo<'info>,
    pub pool_state: &'me AccountInfo<'info>,
    pub vesting_record: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CreateVestingAccountKeys {
    pub creator: Pubkey,
    pub beneficiary: Pubkey,
    pub pool_state: Pubkey,
    pub vesting_record: Pubkey,
    pub system_program: Pubkey,
}

impl From<CreateVestingAccountAccounts<'_, '_>> for CreateVestingAccountKeys {
    fn from(accounts: CreateVestingAccountAccounts) -> Self {
        Self {
            creator: *accounts.creator.key,
            beneficiary: *accounts.beneficiary.key,
            pool_state: *accounts.pool_state.key,
            vesting_record: *accounts.vesting_record.key,
            system_program: *accounts.system_program.key,
        }
    }
}

impl From<CreateVestingAccountKeys> for [AccountMeta; CREATE_VESTING_ACCOUNT_IX_ACCOUNTS_LEN] {
    fn from(keys: CreateVestingAccountKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.creator,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.beneficiary,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.pool_state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.vesting_record,
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

impl From<[Pubkey; CREATE_VESTING_ACCOUNT_IX_ACCOUNTS_LEN]> for CreateVestingAccountKeys {
    fn from(pubkeys: [Pubkey; CREATE_VESTING_ACCOUNT_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            creator: pubkeys[0],
            beneficiary: pubkeys[1],
            pool_state: pubkeys[2],
            vesting_record: pubkeys[3],
            system_program: pubkeys[4],
        }
    }
}

impl<'info> From<CreateVestingAccountAccounts<'_, 'info>> for [AccountInfo<'info>; CREATE_VESTING_ACCOUNT_IX_ACCOUNTS_LEN] {
    fn from(accounts: CreateVestingAccountAccounts<'_, 'info>) -> Self {
        [
            accounts.creator.clone(),
            accounts.beneficiary.clone(),
            accounts.pool_state.clone(),
            accounts.vesting_record.clone(),
            accounts.system_program.clone(),
        ]
    }
}

impl<'me, 'info> From<&'me [AccountInfo<'info>; CREATE_VESTING_ACCOUNT_IX_ACCOUNTS_LEN]> for CreateVestingAccountAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; CREATE_VESTING_ACCOUNT_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            creator: &arr[0],
            beneficiary: &arr[1],
            pool_state: &arr[2],
            vesting_record: &arr[3],
            system_program: &arr[4],
        }
    }
}

pub const CREATE_VESTING_ACCOUNT_IX_DISCM: [u8; 8] = [129, 178, 2, 13, 217, 172, 230, 218];

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CreateVestingAccountIxArgs {
    pub share_amount: u64,
}

#[derive(Clone, Debug, PartialEq)]
pub struct CreateVestingAccountIxData(pub CreateVestingAccountIxArgs);

impl From<CreateVestingAccountIxArgs> for CreateVestingAccountIxData {
    fn from(args: CreateVestingAccountIxArgs) -> Self {
        Self(args)
    }
}

impl CreateVestingAccountIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != CREATE_VESTING_ACCOUNT_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    CREATE_VESTING_ACCOUNT_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(CreateVestingAccountIxArgs::deserialize(&mut reader)?))
    }

    pub fn  serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&CREATE_VESTING_ACCOUNT_IX_DISCM)?;
        self.0.serialize(&mut writer)
    }

    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}

pub fn create_vesting_account_ix_with_program_id(
    program_id: Pubkey,
    keys: CreateVestingAccountKeys,
    args: CreateVestingAccountIxArgs,
) -> std::io::Result<Instruction>{
    let metas: [AccountMeta; CREATE_VESTING_ACCOUNT_IX_ACCOUNTS_LEN] = keys.into();
    let data: CreateVestingAccountIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}

pub fn create_vesting_account_ix(
    keys: CreateVestingAccountKeys,
    args: CreateVestingAccountIxArgs,
) -> std::io::Result<Instruction>{
    create_vesting_account_ix_with_program_id(crate::ID, keys, args)
}

pub fn create_vesting_account_invoke_with_program_id(
    program_id: Pubkey,
    accounts: CreateVestingAccountAccounts<'_, '_>,
    args: CreateVestingAccountIxArgs,
) -> ProgramResult {
    let keys: CreateVestingAccountKeys = accounts.into();
    let ix = create_vesting_account_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}

pub fn create_vesting_account_invoke(
    accounts: CreateVestingAccountAccounts<'_, '_>,
    args: CreateVestingAccountIxArgs,
) -> ProgramResult {
    create_vesting_account_invoke_with_program_id(crate::ID, accounts, args)
}

pub fn create_vesting_account_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: CreateVestingAccountAccounts<'_, '_>,
    args: CreateVestingAccountIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: CreateVestingAccountKeys = accounts.into();
    let ix = create_vesting_account_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}

pub fn create_vesting_account_invoke_signed(
    accounts: CreateVestingAccountAccounts<'_, '_>,
    args: CreateVestingAccountIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    create_vesting_account_invoke_signed_with_program_id(crate::ID, accounts, args, seeds)
}

pub fn create_vesting_account_verify_account_keys(
    accounts: CreateVestingAccountAccounts<'_, '_>,
    keys: CreateVestingAccountKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.creator.key, keys.creator),
        (*accounts.beneficiary.key, keys.beneficiary),
        (*accounts.pool_state.key, keys.pool_state),
        (*accounts.vesting_record.key, keys.vesting_record),
        (*accounts.system_program.key, keys.system_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}

pub fn create_vesting_account_verify_writable_privileges<'me, 'info>(
    accounts: CreateVestingAccountAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.creator,
        accounts.beneficiary,
        accounts.pool_state,
        accounts.vesting_record,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}

pub fn create_vesting_account_verify_signer_privileges<'me, 'info>(
    accounts: CreateVestingAccountAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.creator] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}

pub fn create_vesting_account_verify_account_privileges<'me, 'info>(
    accounts: CreateVestingAccountAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    create_vesting_account_verify_writable_privileges(accounts)?;
    create_vesting_account_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const INITIALIZE_IX_ACCOUNTS_LEN: usize = 18;

#[derive(Copy, Clone, Debug)]
pub struct InitializeAccounts<'me, 'info> {
    pub payer: &'me AccountInfo<'info>,
    pub creator: &'me AccountInfo<'info>,
    pub global_config: &'me AccountInfo<'info>,
    pub platform_config: &'me AccountInfo<'info>,
    pub authority: &'me AccountInfo<'info>,
    pub pool_state: &'me AccountInfo<'info>,
    pub base_mint: &'me AccountInfo<'info>,
    pub quote_mint: &'me AccountInfo<'info>,
    pub base_vault: &'me AccountInfo<'info>,
    pub quote_vault: &'me AccountInfo<'info>,
    pub metadata_account: &'me AccountInfo<'info>,
    pub base_token_program: &'me AccountInfo<'info>,
    pub quote_token_program: &'me AccountInfo<'info>,
    pub metadata_program: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub rent_program: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct InitializeKeys {
    pub payer: Pubkey,
    pub creator: Pubkey,
    pub global_config: Pubkey,
    pub platform_config: Pubkey,
    pub authority: Pubkey,
    pub pool_state: Pubkey,
    pub base_mint: Pubkey,
    pub quote_mint: Pubkey,
    pub base_vault: Pubkey,
    pub quote_vault: Pubkey,
    pub metadata_account: Pubkey,
    pub base_token_program: Pubkey,
    pub quote_token_program: Pubkey,
    pub metadata_program: Pubkey,
    pub system_program: Pubkey,
    pub rent_program: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}

impl From<InitializeAccounts<'_, '_>> for InitializeKeys {
    fn from(accounts: InitializeAccounts) -> Self {
        Self {
            payer: *accounts.payer.key,
            creator: *accounts.creator.key,
            global_config: *accounts.global_config.key,
            platform_config: *accounts.platform_config.key,
            authority: *accounts.authority.key,
            pool_state: *accounts.pool_state.key,
            base_mint: *accounts.base_mint.key,
            quote_mint: *accounts.quote_mint.key,
            base_vault: *accounts.base_vault.key,
            quote_vault: *accounts.quote_vault.key,
            metadata_account: *accounts.metadata_account.key,
            base_token_program: *accounts.base_token_program.key,
            quote_token_program: *accounts.quote_token_program.key,
            metadata_program: *accounts.metadata_program.key,
            system_program: *accounts.system_program.key,
            rent_program: *accounts.rent_program.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}

impl From<InitializeKeys> for [AccountMeta; INITIALIZE_IX_ACCOUNTS_LEN] {
    fn from(keys: InitializeKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.payer,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.creator,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.global_config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.platform_config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pool_state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.base_mint,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.quote_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.base_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.quote_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.metadata_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.base_token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.quote_token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.metadata_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.system_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.rent_program,
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

impl From<[Pubkey; INITIALIZE_IX_ACCOUNTS_LEN]> for InitializeKeys {
    fn from(pubkeys: [Pubkey; INITIALIZE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            payer: pubkeys[0],
            creator: pubkeys[1],
            global_config: pubkeys[2],
            platform_config: pubkeys[3],
            authority: pubkeys[4],
            pool_state: pubkeys[5],
            base_mint: pubkeys[6],
            quote_mint: pubkeys[7],
            base_vault: pubkeys[8],
            quote_vault: pubkeys[9],
            metadata_account: pubkeys[10],
            base_token_program: pubkeys[11],
            quote_token_program: pubkeys[12],
            metadata_program: pubkeys[13],
            system_program: pubkeys[14],
            rent_program: pubkeys[15],
            event_authority: pubkeys[16],
            program: pubkeys[17],
        }
    }
}

impl<'info> From<InitializeAccounts<'_, 'info>> for [AccountInfo<'info>; INITIALIZE_IX_ACCOUNTS_LEN] {
    fn from(accounts: InitializeAccounts<'_, 'info>) -> Self {
        [
            accounts.payer.clone(),
            accounts.creator.clone(),
            accounts.global_config.clone(),
            accounts.platform_config.clone(),
            accounts.authority.clone(),
            accounts.pool_state.clone(),
            accounts.base_mint.clone(),
            accounts.quote_mint.clone(),
            accounts.base_vault.clone(),
            accounts.quote_vault.clone(),
            accounts.metadata_account.clone(),
            accounts.base_token_program.clone(),
            accounts.quote_token_program.clone(),
            accounts.metadata_program.clone(),
            accounts.system_program.clone(),
            accounts.rent_program.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}

impl<'me, 'info> From<&'me [AccountInfo<'info>; INITIALIZE_IX_ACCOUNTS_LEN]> for InitializeAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; INITIALIZE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            payer: &arr[0],
            creator: &arr[1],
            global_config: &arr[2],
            platform_config: &arr[3],
            authority: &arr[4],
            pool_state: &arr[5],
            base_mint: &arr[6],
            quote_mint: &arr[7],
            base_vault: &arr[8],
            quote_vault: &arr[9],
            metadata_account: &arr[10],
            base_token_program: &arr[11],
            quote_token_program: &arr[12],
            metadata_program: &arr[13],
            system_program: &arr[14],
            rent_program: &arr[15],
            event_authority: &arr[16],
            program: &arr[17],
        }
    }
}

pub const INITIALIZE_IX_DISCM: [u8; 8] = [175, 175, 109, 31, 13, 152, 155, 237];

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InitializeIxArgs {
    pub base_mint_param: MintParams,
    pub curve_param: CurveParams,
    pub vesting_param: VestingParams,
}

#[derive(Clone, Debug, PartialEq)]
pub struct InitializeIxData(pub InitializeIxArgs);

impl From<InitializeIxArgs> for InitializeIxData {
    fn from(args: InitializeIxArgs) -> Self {
        Self(args)
    }
}

impl InitializeIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != INITIALIZE_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    INITIALIZE_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(InitializeIxArgs::deserialize(&mut reader)?))
    }

    pub fn  serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&INITIALIZE_IX_DISCM)?;
        self.0.serialize(&mut writer)
    }

    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}

pub fn initialize_ix_with_program_id(
    program_id: Pubkey,
    keys: InitializeKeys,
    args: InitializeIxArgs,
) -> std::io::Result<Instruction>{
    let metas: [AccountMeta; INITIALIZE_IX_ACCOUNTS_LEN] = keys.into();
    let data: InitializeIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}

pub fn initialize_ix(
    keys: InitializeKeys,
    args: InitializeIxArgs,
) -> std::io::Result<Instruction>{
    initialize_ix_with_program_id(crate::ID, keys, args)
}

pub fn initialize_invoke_with_program_id(
    program_id: Pubkey,
    accounts: InitializeAccounts<'_, '_>,
    args: InitializeIxArgs,
) -> ProgramResult {
    let keys: InitializeKeys = accounts.into();
    let ix = initialize_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}

pub fn initialize_invoke(
    accounts: InitializeAccounts<'_, '_>,
    args: InitializeIxArgs,
) -> ProgramResult {
    initialize_invoke_with_program_id(crate::ID, accounts, args)
}

pub fn initialize_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: InitializeAccounts<'_, '_>,
    args: InitializeIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: InitializeKeys = accounts.into();
    let ix = initialize_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}

pub fn initialize_invoke_signed(
    accounts: InitializeAccounts<'_, '_>,
    args: InitializeIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    initialize_invoke_signed_with_program_id(crate::ID, accounts, args, seeds)
}

pub fn initialize_verify_account_keys(
    accounts: InitializeAccounts<'_, '_>,
    keys: InitializeKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.payer.key, keys.payer),
        (*accounts.creator.key, keys.creator),
        (*accounts.global_config.key, keys.global_config),
        (*accounts.platform_config.key, keys.platform_config),
        (*accounts.authority.key, keys.authority),
        (*accounts.pool_state.key, keys.pool_state),
        (*accounts.base_mint.key, keys.base_mint),
        (*accounts.quote_mint.key, keys.quote_mint),
        (*accounts.base_vault.key, keys.base_vault),
        (*accounts.quote_vault.key, keys.quote_vault),
        (*accounts.metadata_account.key, keys.metadata_account),
        (*accounts.base_token_program.key, keys.base_token_program),
        (*accounts.quote_token_program.key, keys.quote_token_program),
        (*accounts.metadata_program.key, keys.metadata_program),
        (*accounts.system_program.key, keys.system_program),
        (*accounts.rent_program.key, keys.rent_program),
        (*accounts.event_authority.key, keys.event_authority),
        (*accounts.program.key, keys.program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}

pub fn initialize_verify_writable_privileges<'me, 'info>(
    accounts: InitializeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.payer,
        accounts.pool_state,
        accounts.base_mint,
        accounts.base_vault,
        accounts.quote_vault,
        accounts.metadata_account,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}

pub fn initialize_verify_signer_privileges<'me, 'info>(
    accounts: InitializeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.payer, accounts.base_mint] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}

pub fn initialize_verify_account_privileges<'me, 'info>(
    accounts: InitializeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    initialize_verify_writable_privileges(accounts)?;
    initialize_verify_signer_privileges(accounts)?;
    Ok(())
}

pub const INITIALIZE_V2_IX_ACCOUNTS_LEN: usize = 18;

#[derive(Copy, Clone, Debug)]
pub struct InitializeV2Accounts<'me, 'info> {
    pub payer: &'me AccountInfo<'info>,
    pub creator: &'me AccountInfo<'info>,
    pub global_config: &'me AccountInfo<'info>,
    pub platform_config: &'me AccountInfo<'info>,
    pub authority: &'me AccountInfo<'info>,
    pub pool_state: &'me AccountInfo<'info>,
    pub base_mint: &'me AccountInfo<'info>,
    pub quote_mint: &'me AccountInfo<'info>,
    pub base_vault: &'me AccountInfo<'info>,
    pub quote_vault: &'me AccountInfo<'info>,
    pub metadata_account: &'me AccountInfo<'info>,
    pub base_token_program: &'me AccountInfo<'info>,
    pub quote_token_program: &'me AccountInfo<'info>,
    pub metadata_program: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub rent_program: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct InitializeV2Keys {
    pub payer: Pubkey,
    pub creator: Pubkey,
    pub global_config: Pubkey,
    pub platform_config: Pubkey,
    pub authority: Pubkey,
    pub pool_state: Pubkey,
    pub base_mint: Pubkey,
    pub quote_mint: Pubkey,
    pub base_vault: Pubkey,
    pub quote_vault: Pubkey,
    pub metadata_account: Pubkey,
    pub base_token_program: Pubkey,
    pub quote_token_program: Pubkey,
    pub metadata_program: Pubkey,
    pub system_program: Pubkey,
    pub rent_program: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}

impl From<InitializeV2Accounts<'_, '_>> for InitializeV2Keys {
    fn from(accounts: InitializeV2Accounts) -> Self {
        Self {
            payer: *accounts.payer.key,
            creator: *accounts.creator.key,
            global_config: *accounts.global_config.key,
            platform_config: *accounts.platform_config.key,
            authority: *accounts.authority.key,
            pool_state: *accounts.pool_state.key,
            base_mint: *accounts.base_mint.key,
            quote_mint: *accounts.quote_mint.key,
            base_vault: *accounts.base_vault.key,
            quote_vault: *accounts.quote_vault.key,
            metadata_account: *accounts.metadata_account.key,
            base_token_program: *accounts.base_token_program.key,
            quote_token_program: *accounts.quote_token_program.key,
            metadata_program: *accounts.metadata_program.key,
            system_program: *accounts.system_program.key,
            rent_program: *accounts.rent_program.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}

impl From<InitializeV2Keys> for [AccountMeta; INITIALIZE_V2_IX_ACCOUNTS_LEN] {
    fn from(keys: InitializeV2Keys) -> Self {
        [
            AccountMeta {
                pubkey: keys.payer,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.creator,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.global_config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.platform_config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pool_state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.base_mint,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.quote_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.base_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.quote_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.metadata_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.base_token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.quote_token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.metadata_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.system_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.rent_program,
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

impl From<[Pubkey; INITIALIZE_V2_IX_ACCOUNTS_LEN]> for InitializeV2Keys {
    fn from(pubkeys: [Pubkey; INITIALIZE_V2_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            payer: pubkeys[0],
            creator: pubkeys[1],
            global_config: pubkeys[2],
            platform_config: pubkeys[3],
            authority: pubkeys[4],
            pool_state: pubkeys[5],
            base_mint: pubkeys[6],
            quote_mint: pubkeys[7],
            base_vault: pubkeys[8],
            quote_vault: pubkeys[9],
            metadata_account: pubkeys[10],
            base_token_program: pubkeys[11],
            quote_token_program: pubkeys[12],
            metadata_program: pubkeys[13],
            system_program: pubkeys[14],
            rent_program: pubkeys[15],
            event_authority: pubkeys[16],
            program: pubkeys[17],
        }
    }
}

impl<'info> From<InitializeV2Accounts<'_, 'info>> for [AccountInfo<'info>; INITIALIZE_V2_IX_ACCOUNTS_LEN] {
    fn from(accounts: InitializeV2Accounts<'_, 'info>) -> Self {
        [
            accounts.payer.clone(),
            accounts.creator.clone(),
            accounts.global_config.clone(),
            accounts.platform_config.clone(),
            accounts.authority.clone(),
            accounts.pool_state.clone(),
            accounts.base_mint.clone(),
            accounts.quote_mint.clone(),
            accounts.base_vault.clone(),
            accounts.quote_vault.clone(),
            accounts.metadata_account.clone(),
            accounts.base_token_program.clone(),
            accounts.quote_token_program.clone(),
            accounts.metadata_program.clone(),
            accounts.system_program.clone(),
            accounts.rent_program.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}

impl<'me, 'info> From<&'me [AccountInfo<'info>; INITIALIZE_V2_IX_ACCOUNTS_LEN]> for InitializeV2Accounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; INITIALIZE_V2_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            payer: &arr[0],
            creator: &arr[1],
            global_config: &arr[2],
            platform_config: &arr[3],
            authority: &arr[4],
            pool_state: &arr[5],
            base_mint: &arr[6],
            quote_mint: &arr[7],
            base_vault: &arr[8],
            quote_vault: &arr[9],
            metadata_account: &arr[10],
            base_token_program: &arr[11],
            quote_token_program: &arr[12],
            metadata_program: &arr[13],
            system_program: &arr[14],
            rent_program: &arr[15],
            event_authority: &arr[16],
            program: &arr[17],
        }
    }
}

pub const INITIALIZE_V2_IX_DISCM: [u8; 8] = [67, 153, 175, 39, 218, 16, 38, 32];

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InitializeV2IxArgs {
    pub base_mint_param: MintParams,
    pub curve_param: CurveParams,
    pub vesting_param: VestingParams,
    pub amm_fee_on: AmmCreatorFeeOn,
}

#[derive(Clone, Debug, PartialEq)]
pub struct InitializeV2IxData(pub InitializeV2IxArgs);

impl From<InitializeV2IxArgs> for InitializeV2IxData {
    fn from(args: InitializeV2IxArgs) -> Self {
        Self(args)
    }
}

impl InitializeV2IxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != INITIALIZE_V2_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    INITIALIZE_V2_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(InitializeV2IxArgs::deserialize(&mut reader)?))
    }

    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&INITIALIZE_V2_IX_DISCM)?;
        self.0.serialize(&mut writer)
    }

    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}

pub fn initialize_v2_ix_with_program_id(
    program_id: Pubkey,
    keys: InitializeV2Keys,
    args: InitializeV2IxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; INITIALIZE_V2_IX_ACCOUNTS_LEN] = keys.into();
    let data: InitializeV2IxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}

pub fn initialize_v2_ix(
    keys: InitializeV2Keys,
    args: InitializeV2IxArgs,
) -> std::io::Result<Instruction> {
    initialize_v2_ix_with_program_id(crate::ID, keys, args)
}

pub fn initialize_v2_invoke_with_program_id(
    program_id: Pubkey,
    accounts: InitializeV2Accounts<'_, '_>,
    args: InitializeV2IxArgs,
) -> ProgramResult {
    let keys: InitializeV2Keys = accounts.into();
    let ix = initialize_v2_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}

pub fn initialize_v2_invoke(
    accounts: InitializeV2Accounts<'_, '_>,
    args: InitializeV2IxArgs,
) -> ProgramResult {
    initialize_v2_invoke_with_program_id(crate::ID, accounts, args)
}

pub fn initialize_v2_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: InitializeV2Accounts<'_, '_>,
    args: InitializeV2IxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: InitializeV2Keys = accounts.into();
    let ix = initialize_v2_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}

pub fn initialize_v2_invoke_signed(
    accounts: InitializeV2Accounts<'_, '_>,
    args: InitializeV2IxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    initialize_v2_invoke_signed_with_program_id(crate::ID, accounts, args, seeds)
}

pub fn initialize_v2_verify_account_keys(
    accounts: InitializeV2Accounts<'_, '_>,
    keys: InitializeV2Keys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.payer.key, keys.payer),
        (*accounts.creator.key, keys.creator),
        (*accounts.global_config.key, keys.global_config),
        (*accounts.platform_config.key, keys.platform_config),
        (*accounts.authority.key, keys.authority),
        (*accounts.pool_state.key, keys.pool_state),
        (*accounts.base_mint.key, keys.base_mint),
        (*accounts.quote_mint.key, keys.quote_mint),
        (*accounts.base_vault.key, keys.base_vault),
        (*accounts.quote_vault.key, keys.quote_vault),
        (*accounts.metadata_account.key, keys.metadata_account),
        (*accounts.base_token_program.key, keys.base_token_program),
        (*accounts.quote_token_program.key, keys.quote_token_program),
        (*accounts.metadata_program.key, keys.metadata_program),
        (*accounts.system_program.key, keys.system_program),
        (*accounts.rent_program.key, keys.rent_program),
        (*accounts.event_authority.key, keys.event_authority),
        (*accounts.program.key, keys.program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}

pub fn initialize_v2_verify_writable_privileges<'me, 'info>(
    accounts: InitializeV2Accounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.payer,
        accounts.pool_state,
        accounts.base_mint,
        accounts.base_vault,
        accounts.quote_vault,
        accounts.metadata_account,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}

pub fn initialize_v2_verify_signer_privileges<'me, 'info>(
    accounts: InitializeV2Accounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.payer, accounts.base_mint] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}

pub fn initialize_v2_verify_account_privileges<'me, 'info>(
    accounts: InitializeV2Accounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    initialize_v2_verify_writable_privileges(accounts)?;
    initialize_v2_verify_signer_privileges(accounts)?;
    Ok(())
}

pub const INITIALIZE_WITH_TOKEN_2022_IX_ACCOUNTS_LEN: usize = 15;

#[derive(Copy, Clone, Debug)]
pub struct InitializeWithToken2022Accounts<'me, 'info> {
    pub payer: &'me AccountInfo<'info>,
    pub creator: &'me AccountInfo<'info>,
    pub global_config: &'me AccountInfo<'info>,
    pub platform_config: &'me AccountInfo<'info>,
    pub authority: &'me AccountInfo<'info>,
    pub pool_state: &'me AccountInfo<'info>,
    pub base_mint: &'me AccountInfo<'info>,
    pub quote_mint: &'me AccountInfo<'info>,
    pub base_vault: &'me AccountInfo<'info>,
    pub quote_vault: &'me AccountInfo<'info>,
    pub base_token_program: &'me AccountInfo<'info>,
    pub quote_token_program: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct InitializeWithToken2022Keys {
    pub payer: Pubkey,
    pub creator: Pubkey,
    pub global_config: Pubkey,
    pub platform_config: Pubkey,
    pub authority: Pubkey,
    pub pool_state: Pubkey,
    pub base_mint: Pubkey,
    pub quote_mint: Pubkey,
    pub base_vault: Pubkey,
    pub quote_vault: Pubkey,
    pub base_token_program: Pubkey,
    pub quote_token_program: Pubkey,
    pub system_program: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}

impl From<InitializeWithToken2022Accounts<'_, '_>> for InitializeWithToken2022Keys {
    fn from(accounts: InitializeWithToken2022Accounts) -> Self {
        Self {
            payer: *accounts.payer.key,
            creator: *accounts.creator.key,
            global_config: *accounts.global_config.key,
            platform_config: *accounts.platform_config.key,
            authority: *accounts.authority.key,
            pool_state: *accounts.pool_state.key,
            base_mint: *accounts.base_mint.key,
            quote_mint: *accounts.quote_mint.key,
            base_vault: *accounts.base_vault.key,
            quote_vault: *accounts.quote_vault.key,
            base_token_program: *accounts.base_token_program.key,
            quote_token_program: *accounts.quote_token_program.key,
            system_program: *accounts.system_program.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}

impl From<InitializeWithToken2022Keys> for [AccountMeta; INITIALIZE_WITH_TOKEN_2022_IX_ACCOUNTS_LEN] {
    fn from(keys: InitializeWithToken2022Keys) -> Self {
        [
            AccountMeta {
                pubkey: keys.payer,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.creator,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.global_config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.platform_config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pool_state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.base_mint,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.quote_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.base_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.quote_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.base_token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.quote_token_program,
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

impl From<[Pubkey; INITIALIZE_WITH_TOKEN_2022_IX_ACCOUNTS_LEN]> for InitializeWithToken2022Keys {
    fn from(pubkeys: [Pubkey; INITIALIZE_WITH_TOKEN_2022_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            payer: pubkeys[0],
            creator: pubkeys[1],
            global_config: pubkeys[2],
            platform_config: pubkeys[3],
            authority: pubkeys[4],
            pool_state: pubkeys[5],
            base_mint: pubkeys[6],
            quote_mint: pubkeys[7],
            base_vault: pubkeys[8],
            quote_vault: pubkeys[9],
            base_token_program: pubkeys[10],
            quote_token_program: pubkeys[11],
            system_program: pubkeys[12],
            event_authority: pubkeys[13],
            program: pubkeys[14],
        }
    }
}

impl<'info> From<InitializeWithToken2022Accounts<'_, 'info>> for [AccountInfo<'info>; INITIALIZE_WITH_TOKEN_2022_IX_ACCOUNTS_LEN] {
    fn from(accounts: InitializeWithToken2022Accounts<'_, 'info>) -> Self {
        [
            accounts.payer.clone(),
            accounts.creator.clone(),
            accounts.global_config.clone(),
            accounts.platform_config.clone(),
            accounts.authority.clone(),
            accounts.pool_state.clone(),
            accounts.base_mint.clone(),
            accounts.quote_mint.clone(),
            accounts.base_vault.clone(),
            accounts.quote_vault.clone(),
            accounts.base_token_program.clone(),
            accounts.quote_token_program.clone(),
            accounts.system_program.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}

impl<'me, 'info> From<&'me [AccountInfo<'info>; INITIALIZE_WITH_TOKEN_2022_IX_ACCOUNTS_LEN]> for InitializeWithToken2022Accounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; INITIALIZE_WITH_TOKEN_2022_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            payer: &arr[0],
            creator: &arr[1],
            global_config: &arr[2],
            platform_config: &arr[3],
            authority: &arr[4],
            pool_state: &arr[5],
            base_mint: &arr[6],
            quote_mint: &arr[7],
            base_vault: &arr[8],
            quote_vault: &arr[9],
            base_token_program: &arr[10],
            quote_token_program: &arr[11],
            system_program: &arr[12],
            event_authority: &arr[13],
            program: &arr[14],
        }
    }
}

pub const INITIALIZE_WITH_TOKEN_2022_IX_DISCM: [u8; 8] = [37, 190, 126, 222, 44, 154, 171, 17];

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InitializeWithToken2022IxArgs {
    pub base_mint_param: MintParams,
    pub curve_param: CurveParams,
    pub vesting_param: VestingParams,
    pub amm_fee_on: AmmCreatorFeeOn,
    pub transfer_fee_extension_param: Option<TransferFeeExtensionParams>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct InitializeWithToken2022IxData(pub InitializeWithToken2022IxArgs);

impl From<InitializeWithToken2022IxArgs> for InitializeWithToken2022IxData {
    fn from(args: InitializeWithToken2022IxArgs) -> Self {
        Self(args)
    }
}

impl InitializeWithToken2022IxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != INITIALIZE_WITH_TOKEN_2022_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    INITIALIZE_WITH_TOKEN_2022_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(InitializeWithToken2022IxArgs::deserialize(&mut reader)?))
    }

    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&INITIALIZE_WITH_TOKEN_2022_IX_DISCM)?;
        self.0.serialize(&mut writer)
    }

    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}

pub fn initialize_with_token_2022_ix_with_program_id(
    program_id: Pubkey,
    keys: InitializeWithToken2022Keys,
    args: InitializeWithToken2022IxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; INITIALIZE_WITH_TOKEN_2022_IX_ACCOUNTS_LEN] = keys.into();
    let data: InitializeWithToken2022IxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}

pub fn initialize_with_token_2022_ix(
    keys: InitializeWithToken2022Keys,
    args: InitializeWithToken2022IxArgs,
) -> std::io::Result<Instruction> {
    initialize_with_token_2022_ix_with_program_id(crate::ID, keys, args)
}

pub fn initialize_with_token_2022_invoke_with_program_id(
    program_id: Pubkey,
    accounts: InitializeWithToken2022Accounts<'_, '_>,
    args: InitializeWithToken2022IxArgs,
) -> ProgramResult {
    let keys: InitializeWithToken2022Keys = accounts.into();
    let ix = initialize_with_token_2022_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}

pub fn initialize_with_token_2022_invoke(
    accounts: InitializeWithToken2022Accounts<'_, '_>,
    args: InitializeWithToken2022IxArgs,
) -> ProgramResult {
    initialize_with_token_2022_invoke_with_program_id(crate::ID, accounts, args)
}

pub fn initialize_with_token_2022_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: InitializeWithToken2022Accounts<'_, '_>,
    args: InitializeWithToken2022IxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: InitializeWithToken2022Keys = accounts.into();
    let ix = initialize_with_token_2022_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}

pub fn initialize_with_token_2022_invoke_signed(
    accounts: InitializeWithToken2022Accounts<'_, '_>,
    args: InitializeWithToken2022IxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    initialize_with_token_2022_invoke_signed_with_program_id(crate::ID, accounts, args, seeds)
}

pub fn initialize_with_token_2022_verify_account_keys(
    accounts: InitializeWithToken2022Accounts<'_, '_>,
    keys: InitializeWithToken2022Keys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.payer.key, keys.payer),
        (*accounts.creator.key, keys.creator),
        (*accounts.global_config.key, keys.global_config),
        (*accounts.platform_config.key, keys.platform_config),
        (*accounts.authority.key, keys.authority),
        (*accounts.pool_state.key, keys.pool_state),
        (*accounts.base_mint.key, keys.base_mint),
        (*accounts.quote_mint.key, keys.quote_mint),
        (*accounts.base_vault.key, keys.base_vault),
        (*accounts.quote_vault.key, keys.quote_vault),
        (*accounts.base_token_program.key, keys.base_token_program),
        (*accounts.quote_token_program.key, keys.quote_token_program),
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

pub fn initialize_with_token_2022_verify_writable_privileges<'me, 'info>(
    accounts: InitializeWithToken2022Accounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.payer,
        accounts.pool_state,
        accounts.base_mint,
        accounts.base_vault,
        accounts.quote_vault,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}

pub fn initialize_with_token_2022_verify_signer_privileges<'me, 'info>(
    accounts: InitializeWithToken2022Accounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.payer, accounts.base_mint] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}

pub fn initialize_with_token_2022_verify_account_privileges<'me, 'info>(
    accounts: InitializeWithToken2022Accounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    initialize_with_token_2022_verify_writable_privileges(accounts)?;
    initialize_with_token_2022_verify_signer_privileges(accounts)?;
    Ok(())
}

pub const MIGRATE_TO_AMM_IX_ACCOUNTS_LEN: usize = 32;

#[derive(Copy, Clone, Debug)]
pub struct MigrateToAmmAccounts<'me, 'info> {
    pub payer: &'me AccountInfo<'info>,
    pub base_mint: &'me AccountInfo<'info>,
    pub quote_mint: &'me AccountInfo<'info>,
    pub openbook_program: &'me AccountInfo<'info>,
    pub market: &'me AccountInfo<'info>,
    pub request_queue: &'me AccountInfo<'info>,
    pub event_queue: &'me AccountInfo<'info>,
    pub bids: &'me AccountInfo<'info>,
    pub asks: &'me AccountInfo<'info>,
    pub market_vault_signer: &'me AccountInfo<'info>,
    pub market_base_vault: &'me AccountInfo<'info>,
    pub market_quote_vault: &'me AccountInfo<'info>,
    pub amm_program: &'me AccountInfo<'info>,
    pub amm_pool: &'me AccountInfo<'info>,
    pub amm_authority: &'me AccountInfo<'info>,
    pub amm_open_orders: &'me AccountInfo<'info>,
    pub amm_lp_mint: &'me AccountInfo<'info>,
    pub amm_base_vault: &'me AccountInfo<'info>,
    pub amm_quote_vault: &'me AccountInfo<'info>,
    pub amm_target_orders: &'me AccountInfo<'info>,
    pub amm_config: &'me AccountInfo<'info>,
    pub amm_create_fee_destination: &'me AccountInfo<'info>,
    pub authority: &'me AccountInfo<'info>,
    pub pool_state: &'me AccountInfo<'info>,
    pub global_config: &'me AccountInfo<'info>,
    pub base_vault: &'me AccountInfo<'info>,
    pub quote_vault: &'me AccountInfo<'info>,
    pub pool_lp_token: &'me AccountInfo<'info>,
    pub spl_token_program: &'me AccountInfo<'info>,
    pub associated_token_program: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub rent_program: &'me AccountInfo<'info>,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MigrateToAmmKeys {
    pub payer: Pubkey,
    pub base_mint: Pubkey,
    pub quote_mint: Pubkey,
    pub openbook_program: Pubkey,
    pub market: Pubkey,
    pub request_queue: Pubkey,
    pub event_queue: Pubkey,
    pub bids: Pubkey,
    pub asks: Pubkey,
    pub market_vault_signer: Pubkey,
    pub market_base_vault: Pubkey,
    pub market_quote_vault: Pubkey,
    pub amm_program: Pubkey,
    pub amm_pool: Pubkey,
    pub amm_authority: Pubkey,
    pub amm_open_orders: Pubkey,
    pub amm_lp_mint: Pubkey,
    pub amm_base_vault: Pubkey,
    pub amm_quote_vault: Pubkey,
    pub amm_target_orders: Pubkey,
    pub amm_config: Pubkey,
    pub amm_create_fee_destination: Pubkey,
    pub authority: Pubkey,
    pub pool_state: Pubkey,
    pub global_config: Pubkey,
    pub base_vault: Pubkey,
    pub quote_vault: Pubkey,
    pub pool_lp_token: Pubkey,
    pub spl_token_program: Pubkey,
    pub associated_token_program: Pubkey,
    pub system_program: Pubkey,
    pub rent_program: Pubkey,
}

impl From<MigrateToAmmAccounts<'_, '_>> for MigrateToAmmKeys {
    fn from(accounts: MigrateToAmmAccounts) -> Self {
        Self {
            payer: *accounts.payer.key,
            base_mint: *accounts.base_mint.key,
            quote_mint: *accounts.quote_mint.key,
            openbook_program: *accounts.openbook_program.key,
            market: *accounts.market.key,
            request_queue: *accounts.request_queue.key,
            event_queue: *accounts.event_queue.key,
            bids: *accounts.bids.key,
            asks: *accounts.asks.key,
            market_vault_signer: *accounts.market_vault_signer.key,
            market_base_vault: *accounts.market_base_vault.key,
            market_quote_vault: *accounts.market_quote_vault.key,
            amm_program: *accounts.amm_program.key,
            amm_pool: *accounts.amm_pool.key,
            amm_authority: *accounts.amm_authority.key,
            amm_open_orders: *accounts.amm_open_orders.key,
            amm_lp_mint: *accounts.amm_lp_mint.key,
            amm_base_vault: *accounts.amm_base_vault.key,
            amm_quote_vault: *accounts.amm_quote_vault.key,
            amm_target_orders: *accounts.amm_target_orders.key,
            amm_config: *accounts.amm_config.key,
            amm_create_fee_destination: *accounts.amm_create_fee_destination.key,
            authority: *accounts.authority.key,
            pool_state: *accounts.pool_state.key,
            global_config: *accounts.global_config.key,
            base_vault: *accounts.base_vault.key,
            quote_vault: *accounts.quote_vault.key,
            pool_lp_token: *accounts.pool_lp_token.key,
            spl_token_program: *accounts.spl_token_program.key,
            associated_token_program: *accounts.associated_token_program.key,
            system_program: *accounts.system_program.key,
            rent_program: *accounts.rent_program.key,
        }
    }
}

impl From<MigrateToAmmKeys> for [AccountMeta; MIGRATE_TO_AMM_IX_ACCOUNTS_LEN] {
    fn from(keys: MigrateToAmmKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.payer,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.base_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.quote_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.openbook_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.market,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.request_queue,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.event_queue,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.bids,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.asks,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.market_vault_signer,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.market_base_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.market_quote_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.amm_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.amm_pool,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.amm_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.amm_open_orders,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.amm_lp_mint,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.amm_base_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.amm_quote_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.amm_target_orders,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.amm_config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.amm_create_fee_destination,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.authority,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.pool_state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.global_config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.base_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.quote_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.pool_lp_token,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.spl_token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.associated_token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.system_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.rent_program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}

impl From<[Pubkey; MIGRATE_TO_AMM_IX_ACCOUNTS_LEN]> for MigrateToAmmKeys {
    fn from(pubkeys: [Pubkey; MIGRATE_TO_AMM_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            payer: pubkeys[0],
            base_mint: pubkeys[1],
            quote_mint: pubkeys[2],
            openbook_program: pubkeys[3],
            market: pubkeys[4],
            request_queue: pubkeys[5],
            event_queue: pubkeys[6],
            bids: pubkeys[2],
            asks: pubkeys[8],
            market_vault_signer: pubkeys[9],
            market_base_vault: pubkeys[10],
            market_quote_vault: pubkeys[11],
            amm_program: pubkeys[12],
            amm_pool: pubkeys[13],
            amm_authority: pubkeys[14],
            amm_open_orders: pubkeys[15],
            amm_lp_mint: pubkeys[16],
            amm_base_vault: pubkeys[17],
            amm_quote_vault: pubkeys[18],
            amm_target_orders: pubkeys[19],
            amm_config: pubkeys[20],
            amm_create_fee_destination: pubkeys[21],
            authority: pubkeys[22],
            pool_state: pubkeys[23],
            global_config: pubkeys[24],
            base_vault: pubkeys[25],
            quote_vault: pubkeys[26],
            pool_lp_token: pubkeys[27],
            spl_token_program: pubkeys[28],
            associated_token_program: pubkeys[29],
            system_program: pubkeys[30],
            rent_program: pubkeys[31],
        }
    }
}

impl<'info> From<MigrateToAmmAccounts<'_, 'info>> for [AccountInfo<'info>; MIGRATE_TO_AMM_IX_ACCOUNTS_LEN] {
    fn from(accounts: MigrateToAmmAccounts<'_, 'info>) -> Self {
        [
            accounts.payer.clone(),
            accounts.base_mint.clone(),
            accounts.quote_mint.clone(),
            accounts.openbook_program.clone(),
            accounts.market.clone(),
            accounts.request_queue.clone(),
            accounts.event_queue.clone(),
            accounts.bids.clone(),
            accounts.asks.clone(),
            accounts.market_vault_signer.clone(),
            accounts.market_base_vault.clone(),
            accounts.market_quote_vault.clone(),
            accounts.amm_program.clone(),
            accounts.amm_pool.clone(),
            accounts.amm_authority.clone(),
            accounts.amm_open_orders.clone(),
            accounts.amm_lp_mint.clone(),
            accounts.amm_base_vault.clone(),
            accounts.amm_quote_vault.clone(),
            accounts.amm_target_orders.clone(),
            accounts.amm_config.clone(),
            accounts.amm_create_fee_destination.clone(),
            accounts.authority.clone(),
            accounts.pool_state.clone(),
            accounts.global_config.clone(),
            accounts.base_vault.clone(),
            accounts.quote_vault.clone(),
            accounts.pool_lp_token.clone(),
            accounts.spl_token_program.clone(),
            accounts.associated_token_program.clone(),
            accounts.system_program.clone(),
            accounts.rent_program.clone(),
        ]
    }
}

impl<'me, 'info> From<&'me [AccountInfo<'info>; MIGRATE_TO_AMM_IX_ACCOUNTS_LEN]> for MigrateToAmmAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; MIGRATE_TO_AMM_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            payer: &arr[0],
            base_mint: &arr[1],
            quote_mint: &arr[2],
            openbook_program: &arr[3],
            market: &arr[4],
            request_queue: &arr[5],
            event_queue: &arr[6],
            bids: &arr[7],
            asks: &arr[8],
            market_vault_signer: &arr[9],
            market_base_vault: &arr[10],
            market_quote_vault: &arr[11],
            amm_program: &arr[12],
            amm_pool: &arr[13],
            amm_authority: &arr[14],
            amm_open_orders: &arr[15],
            amm_lp_mint: &arr[16],
            amm_base_vault: &arr[17],
            amm_quote_vault: &arr[18],
            amm_target_orders: &arr[19],
            amm_config: &arr[20],
            amm_create_fee_destination: &arr[21],
            authority: &arr[22],
            pool_state: &arr[23],
            global_config: &arr[24],
            base_vault: &arr[25],
            quote_vault: &arr[26],
            pool_lp_token: &arr[27],
            spl_token_program: &arr[28],
            associated_token_program: &arr[29],
            system_program: &arr[30],
            rent_program: &arr[31],
        }
    }
}

pub const MIGRATE_TO_AMM_IX_DISCM: [u8; 8] = [207, 82, 192, 145, 254, 207, 145, 223];

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct MigrateToAmmIxArgs {
    pub base_lot_size: u64,
    pub quote_lot_size: u64,
    pub market_vault_signer_nonce: u8,
}

#[derive(Clone, Debug, PartialEq)]
pub struct MigrateToAmmIxData(pub MigrateToAmmIxArgs);

impl From<MigrateToAmmIxArgs> for MigrateToAmmIxData {
    fn from(args: MigrateToAmmIxArgs) -> Self {
        Self(args)
    }
}

impl MigrateToAmmIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != MIGRATE_TO_AMM_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    MIGRATE_TO_AMM_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(MigrateToAmmIxArgs::deserialize(&mut reader)?))
    }

    pub fn  serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&MIGRATE_TO_AMM_IX_DISCM)?;
        self.0.serialize(&mut writer)
    }

    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}

pub fn migrate_to_amm_ix_with_program_id(
    program_id: Pubkey,
    keys: MigrateToAmmKeys,
    args: MigrateToAmmIxArgs,
) -> std::io::Result<Instruction>{
    let metas: [AccountMeta; MIGRATE_TO_AMM_IX_ACCOUNTS_LEN] = keys.into();
    let data: MigrateToAmmIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}

pub fn migrate_to_amm_ix(
    keys: MigrateToAmmKeys,
    args: MigrateToAmmIxArgs,
) -> std::io::Result<Instruction>{
    migrate_to_amm_ix_with_program_id(crate::ID, keys, args)
}

pub fn migrate_to_amm_invoke_with_program_id(
    program_id: Pubkey,
    accounts: MigrateToAmmAccounts<'_, '_>,
    args: MigrateToAmmIxArgs,
) -> ProgramResult {
    let keys: MigrateToAmmKeys = accounts.into();
    let ix = migrate_to_amm_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}

pub fn migrate_to_amm_invoke(
    accounts: MigrateToAmmAccounts<'_, '_>,
    args: MigrateToAmmIxArgs,
) -> ProgramResult {
    migrate_to_amm_invoke_with_program_id(crate::ID, accounts, args)
}

pub fn migrate_to_amm_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: MigrateToAmmAccounts<'_, '_>,
    args: MigrateToAmmIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: MigrateToAmmKeys = accounts.into();
    let ix = migrate_to_amm_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}

pub fn migrate_to_amm_invoke_signed(
    accounts: MigrateToAmmAccounts<'_, '_>,
    args: MigrateToAmmIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    migrate_to_amm_invoke_signed_with_program_id(crate::ID, accounts, args, seeds)
}

pub fn migrate_to_amm_verify_account_keys(
    accounts: MigrateToAmmAccounts<'_, '_>,
    keys: MigrateToAmmKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.payer.key, keys.payer),
        (*accounts.base_mint.key, keys.base_mint),
        (*accounts.quote_mint.key, keys.quote_mint),
        (*accounts.openbook_program.key, keys.openbook_program),
        (*accounts.market.key, keys.market),
        (*accounts.request_queue.key, keys.request_queue),
        (*accounts.event_queue.key, keys.event_queue),
        (*accounts.bids.key, keys.bids),
        (*accounts.asks.key, keys.asks),
        (*accounts.market_vault_signer.key, keys.market_vault_signer),
        (*accounts.market_base_vault.key, keys.market_base_vault),
        (*accounts.market_quote_vault.key, keys.market_quote_vault),
        (*accounts.amm_program.key, keys.amm_program),
        (*accounts.amm_pool.key, keys.amm_pool),
        (*accounts.amm_authority.key, keys.amm_authority),
        (*accounts.amm_open_orders.key, keys.amm_open_orders),
        (*accounts.amm_lp_mint.key, keys.amm_lp_mint),
        (*accounts.amm_base_vault.key, keys.amm_base_vault),
        (*accounts.amm_quote_vault.key, keys.amm_quote_vault),
        (*accounts.amm_target_orders.key, keys.amm_target_orders),
        (*accounts.amm_config.key, keys.amm_config),
        (*accounts.amm_create_fee_destination.key, keys.amm_create_fee_destination),
        (*accounts.authority.key, keys.authority),
        (*accounts.pool_state.key, keys.pool_state),
        (*accounts.global_config.key, keys.global_config),
        (*accounts.base_vault.key, keys.base_vault),
        (*accounts.quote_vault.key, keys.quote_vault),
        (*accounts.pool_lp_token.key, keys.pool_lp_token),
        (*accounts.spl_token_program.key, keys.spl_token_program),
        (*accounts.associated_token_program.key, keys.associated_token_program),
        (*accounts.system_program.key, keys.system_program),
        (*accounts.rent_program.key, keys.rent_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}

pub fn migrate_to_amm_verify_writable_privileges<'me, 'info>(
    accounts: MigrateToAmmAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.payer,
        accounts.market,
        accounts.request_queue,
        accounts.event_queue,
        accounts.bids,
        accounts.asks,
        accounts.market_base_vault,
        accounts.market_quote_vault,
        accounts.amm_pool,
        accounts.amm_open_orders,
        accounts.amm_lp_mint,
        accounts.amm_base_vault,
        accounts.amm_quote_vault,
        accounts.amm_target_orders,
        accounts.amm_create_fee_destination,
        accounts.authority,
        accounts.pool_state,
        accounts.base_vault,
        accounts.quote_vault,
        accounts.pool_lp_token,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}

pub fn migrate_to_amm_verify_signer_privileges<'me, 'info>(
    accounts: MigrateToAmmAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.payer] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}

pub fn migrate_to_amm_verify_account_privileges<'me, 'info>(
    accounts: MigrateToAmmAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    migrate_to_amm_verify_writable_privileges(accounts)?;
    migrate_to_amm_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const MIGRATE_TO_CPSWAP_IX_ACCOUNTS_LEN: usize = 28;

#[derive(Copy, Clone, Debug)]
pub struct MigrateToCpswapAccounts<'me, 'info> {
    pub payer: &'me AccountInfo<'info>,
    pub base_mint: &'me AccountInfo<'info>,
    pub quote_mint: &'me AccountInfo<'info>,
    pub platform_config: &'me AccountInfo<'info>,
    pub cpswap_program: &'me AccountInfo<'info>,
    pub cpswap_pool: &'me AccountInfo<'info>,
    pub cpswap_authority: &'me AccountInfo<'info>,
    pub cpswap_lp_mint: &'me AccountInfo<'info>,
    pub cpswap_base_vault: &'me AccountInfo<'info>,
    pub cpswap_quote_vault: &'me AccountInfo<'info>,
    pub cpswap_config: &'me AccountInfo<'info>,
    pub cpswap_create_pool_fee: &'me AccountInfo<'info>,
    pub cpswap_observation: &'me AccountInfo<'info>,
    pub lock_program: &'me AccountInfo<'info>,
    pub lock_authority: &'me AccountInfo<'info>,
    pub lock_lp_vault: &'me AccountInfo<'info>,
    pub authority: &'me AccountInfo<'info>,
    pub pool_state: &'me AccountInfo<'info>,
    pub global_config: &'me AccountInfo<'info>,
    pub base_vault: &'me AccountInfo<'info>,
    pub quote_vault: &'me AccountInfo<'info>,
    pub pool_lp_token: &'me AccountInfo<'info>,
    pub base_token_program: &'me AccountInfo<'info>,
    pub quote_token_program: &'me AccountInfo<'info>,
    pub associated_token_program: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub rent_program: &'me AccountInfo<'info>,
    pub metadata_program: &'me AccountInfo<'info>,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MigrateToCpswapKeys {
    pub payer: Pubkey,
    pub base_mint: Pubkey,
    pub quote_mint: Pubkey,
    pub platform_config: Pubkey,
    pub cpswap_program: Pubkey,
    pub cpswap_pool: Pubkey,
    pub cpswap_authority: Pubkey,
    pub cpswap_lp_mint: Pubkey,
    pub cpswap_base_vault: Pubkey,
    pub cpswap_quote_vault: Pubkey,
    pub cpswap_config: Pubkey,
    pub cpswap_create_pool_fee: Pubkey,
    pub cpswap_observation: Pubkey,
    pub lock_program: Pubkey,
    pub lock_authority: Pubkey,
    pub lock_lp_vault: Pubkey,
    pub authority: Pubkey,
    pub pool_state: Pubkey,
    pub global_config: Pubkey,
    pub base_vault: Pubkey,
    pub quote_vault: Pubkey,
    pub pool_lp_token: Pubkey,
    pub base_token_program: Pubkey,
    pub quote_token_program: Pubkey,
    pub associated_token_program: Pubkey,
    pub system_program: Pubkey,
    pub rent_program: Pubkey,
    pub metadata_program: Pubkey,
}

impl From<MigrateToCpswapAccounts<'_, '_>> for MigrateToCpswapKeys {
    fn from(accounts: MigrateToCpswapAccounts) -> Self {
        Self {
            payer: *accounts.payer.key,
            base_mint: *accounts.base_mint.key,
            quote_mint: *accounts.quote_mint.key,
            platform_config: *accounts.platform_config.key,
            cpswap_program: *accounts.cpswap_program.key,
            cpswap_pool: *accounts.cpswap_pool.key,
            cpswap_authority: *accounts.cpswap_authority.key,
            cpswap_lp_mint: *accounts.cpswap_lp_mint.key,
            cpswap_base_vault: *accounts.cpswap_base_vault.key,
            cpswap_quote_vault: *accounts.cpswap_quote_vault.key,
            cpswap_config: *accounts.cpswap_config.key,
            cpswap_create_pool_fee: *accounts.cpswap_create_pool_fee.key,
            cpswap_observation: *accounts.cpswap_observation.key,
            lock_program: *accounts.lock_program.key,
            lock_authority: *accounts.lock_authority.key,
            lock_lp_vault: *accounts.lock_lp_vault.key,
            authority: *accounts.authority.key,
            pool_state: *accounts.pool_state.key,
            global_config: *accounts.global_config.key,
            base_vault: *accounts.base_vault.key,
            quote_vault: *accounts.quote_vault.key,
            pool_lp_token: *accounts.pool_lp_token.key,
            base_token_program: *accounts.base_token_program.key,
            quote_token_program: *accounts.quote_token_program.key,
            associated_token_program: *accounts.associated_token_program.key,
            system_program: *accounts.system_program.key,
            rent_program: *accounts.rent_program.key,
            metadata_program: *accounts.metadata_program.key,
        }
    }
}

impl From<MigrateToCpswapKeys> for [AccountMeta; MIGRATE_TO_CPSWAP_IX_ACCOUNTS_LEN] {
    fn from(keys: MigrateToCpswapKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.payer,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.base_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.quote_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.platform_config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.cpswap_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.cpswap_pool,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.cpswap_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.cpswap_lp_mint,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.cpswap_base_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.cpswap_quote_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.cpswap_config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.cpswap_create_pool_fee,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.cpswap_observation,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.lock_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.lock_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.lock_lp_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.authority,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.pool_state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.global_config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.base_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.quote_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.pool_lp_token,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.base_token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.quote_token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.associated_token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.system_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.rent_program,
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

impl From<[Pubkey; MIGRATE_TO_CPSWAP_IX_ACCOUNTS_LEN]> for MigrateToCpswapKeys {
    fn from(pubkeys: [Pubkey; MIGRATE_TO_CPSWAP_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            payer: pubkeys[0],
            base_mint: pubkeys[1],
            quote_mint: pubkeys[2],
            platform_config: pubkeys[3],
            cpswap_program: pubkeys[4],
            cpswap_pool: pubkeys[5],
            cpswap_authority: pubkeys[6],
            cpswap_lp_mint: pubkeys[7],
            cpswap_base_vault: pubkeys[8],
            cpswap_quote_vault: pubkeys[9],
            cpswap_config: pubkeys[10],
            cpswap_create_pool_fee: pubkeys[11],
            cpswap_observation: pubkeys[12],
            lock_program: pubkeys[13],
            lock_authority: pubkeys[14],
            lock_lp_vault: pubkeys[15],
            authority: pubkeys[16],
            pool_state: pubkeys[17],
            global_config: pubkeys[18],
            base_vault: pubkeys[19],
            quote_vault: pubkeys[20],
            pool_lp_token: pubkeys[21],
            base_token_program: pubkeys[22],
            quote_token_program: pubkeys[23],
            associated_token_program: pubkeys[24],
            system_program: pubkeys[25],
            rent_program: pubkeys[26],
            metadata_program: pubkeys[27],
        }
    }
}

impl<'info> From<MigrateToCpswapAccounts<'_, 'info>> for [AccountInfo<'info>; MIGRATE_TO_CPSWAP_IX_ACCOUNTS_LEN] {
    fn from(accounts: MigrateToCpswapAccounts<'_, 'info>) -> Self {
        [
            accounts.payer.clone(),
            accounts.base_mint.clone(),
            accounts.quote_mint.clone(),
            accounts.platform_config.clone(),
            accounts.cpswap_program.clone(),
            accounts.cpswap_pool.clone(),
            accounts.cpswap_authority.clone(),
            accounts.cpswap_lp_mint.clone(),
            accounts.cpswap_base_vault.clone(),
            accounts.cpswap_quote_vault.clone(),
            accounts.cpswap_config.clone(),
            accounts.cpswap_create_pool_fee.clone(),
            accounts.cpswap_observation.clone(),
            accounts.lock_program.clone(),
            accounts.lock_authority.clone(),
            accounts.lock_lp_vault.clone(),
            accounts.authority.clone(),
            accounts.pool_state.clone(),
            accounts.global_config.clone(),
            accounts.base_vault.clone(),
            accounts.quote_vault.clone(),
            accounts.pool_lp_token.clone(),
            accounts.base_token_program.clone(),
            accounts.quote_token_program.clone(),
            accounts.associated_token_program.clone(),
            accounts.system_program.clone(),
            accounts.rent_program.clone(),
            accounts.metadata_program.clone(),
        ]
    }
}

impl<'me, 'info> From<&'me [AccountInfo<'info>; MIGRATE_TO_CPSWAP_IX_ACCOUNTS_LEN]> for MigrateToCpswapAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; MIGRATE_TO_CPSWAP_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            payer: &arr[0],
            base_mint: &arr[1],
            quote_mint: &arr[2],
            platform_config: &arr[3],
            cpswap_program: &arr[4],
            cpswap_pool: &arr[5],
            cpswap_authority: &arr[6],
            cpswap_lp_mint: &arr[7],
            cpswap_base_vault: &arr[8],
            cpswap_quote_vault: &arr[9],
            cpswap_config: &arr[10],
            cpswap_create_pool_fee: &arr[11],
            cpswap_observation: &arr[12],
            lock_program: &arr[13],
            lock_authority: &arr[14],
            lock_lp_vault: &arr[15],
            authority: &arr[16],
            pool_state: &arr[17],
            global_config: &arr[18],
            base_vault: &arr[19],
            quote_vault: &arr[20],
            pool_lp_token: &arr[21],
            base_token_program: &arr[22],
            quote_token_program: &arr[23],
            associated_token_program: &arr[24],
            system_program: &arr[25],
            rent_program: &arr[26],
            metadata_program: &arr[27],
        }
    }
}

pub const MIGRATE_TO_CPSWAP_IX_DISCM: [u8; 8] = [136, 92, 200, 103, 28, 218, 144, 140];

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct MigrateToCpswapIxArgs;

#[derive(Clone, Debug, PartialEq)]
pub struct MigrateToCpswapIxData(pub MigrateToCpswapIxArgs);

impl From<MigrateToCpswapIxArgs> for MigrateToCpswapIxData {
    fn from(args: MigrateToCpswapIxArgs) -> Self {
        Self(args)
    }
}

impl MigrateToCpswapIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != MIGRATE_TO_CPSWAP_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    MIGRATE_TO_CPSWAP_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(MigrateToCpswapIxArgs::deserialize(&mut reader)?))
    }

    pub fn  serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&MIGRATE_TO_CPSWAP_IX_DISCM)?;
        self.0.serialize(&mut writer)
    }

    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}

pub fn migrate_to_cpswap_ix_with_program_id(
    program_id: Pubkey,
    keys: MigrateToCpswapKeys,
    args: MigrateToCpswapIxArgs,
) -> std::io::Result<Instruction>{
    let metas: [AccountMeta; MIGRATE_TO_CPSWAP_IX_ACCOUNTS_LEN] = keys.into();
    let data: MigrateToCpswapIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}

pub fn migrate_to_cpswap_ix(
    keys: MigrateToCpswapKeys,
    args: MigrateToCpswapIxArgs,
) -> std::io::Result<Instruction>{
    migrate_to_cpswap_ix_with_program_id(crate::ID, keys, args)
}

pub fn migrate_to_cpswap_invoke_with_program_id(
    program_id: Pubkey,
    accounts: MigrateToCpswapAccounts<'_, '_>,
    args: MigrateToCpswapIxArgs,
) -> ProgramResult {
    let keys: MigrateToCpswapKeys = accounts.into();
    let ix = migrate_to_cpswap_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}

pub fn migrate_to_cpswap_invoke(
    accounts: MigrateToCpswapAccounts<'_, '_>,
    args: MigrateToCpswapIxArgs,
) -> ProgramResult {
    migrate_to_cpswap_invoke_with_program_id(crate::ID, accounts, args)
}

pub fn migrate_to_cpswap_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: MigrateToCpswapAccounts<'_, '_>,
    args: MigrateToCpswapIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: MigrateToCpswapKeys = accounts.into();
    let ix = migrate_to_cpswap_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}

pub fn migrate_to_cpswap_invoke_signed(
    accounts: MigrateToCpswapAccounts<'_, '_>,
    args: MigrateToCpswapIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    migrate_to_cpswap_invoke_signed_with_program_id(crate::ID, accounts, args, seeds)
}

pub fn migrate_to_cpswap_verify_account_keys(
    accounts: MigrateToCpswapAccounts<'_, '_>,
    keys: MigrateToCpswapKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.payer.key, keys.payer),
        (*accounts.base_mint.key, keys.base_mint),
        (*accounts.quote_mint.key, keys.quote_mint),
        (*accounts.platform_config.key, keys.platform_config),
        (*accounts.cpswap_program.key, keys.cpswap_program),
        (*accounts.cpswap_pool.key, keys.cpswap_pool),
        (*accounts.cpswap_authority.key, keys.cpswap_authority),
        (*accounts.cpswap_lp_mint.key, keys.cpswap_lp_mint),
        (*accounts.cpswap_base_vault.key, keys.cpswap_base_vault),
        (*accounts.cpswap_quote_vault.key, keys.cpswap_quote_vault),
        (*accounts.cpswap_config.key, keys.cpswap_config),
        (*accounts.cpswap_create_pool_fee.key, keys.cpswap_create_pool_fee),
        (*accounts.cpswap_observation.key, keys.cpswap_observation),
        (*accounts.lock_program.key, keys.lock_program),
        (*accounts.lock_authority.key, keys.lock_authority),
        (*accounts.lock_lp_vault.key, keys.lock_lp_vault),
        (*accounts.authority.key, keys.authority),
        (*accounts.pool_state.key, keys.pool_state),
        (*accounts.global_config.key, keys.global_config),
        (*accounts.base_vault.key, keys.base_vault),
        (*accounts.quote_vault.key, keys.quote_vault),
        (*accounts.pool_lp_token.key, keys.pool_lp_token),
        (*accounts.base_token_program.key, keys.base_token_program),
        (*accounts.quote_token_program.key, keys.quote_token_program),
        (*accounts.associated_token_program.key, keys.associated_token_program),
        (*accounts.system_program.key, keys.system_program),
        (*accounts.rent_program.key, keys.rent_program),
        (*accounts.metadata_program.key, keys.metadata_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}

pub fn migrate_to_cpswap_verify_writable_privileges<'me, 'info>(
    accounts: MigrateToCpswapAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.payer,
        accounts.cpswap_pool,
        accounts.cpswap_lp_mint,
        accounts.cpswap_base_vault,
        accounts.cpswap_quote_vault,
        accounts.cpswap_create_pool_fee,
        accounts.cpswap_observation,
        accounts.lock_lp_vault,
        accounts.authority,
        accounts.pool_state,
        accounts.base_vault,
        accounts.quote_vault,
        accounts.pool_lp_token,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}

pub fn migrate_to_cpswap_verify_signer_privileges<'me, 'info>(
    accounts: MigrateToCpswapAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.payer] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}

pub fn migrate_to_cpswap_verify_account_privileges<'me, 'info>(
    accounts: MigrateToCpswapAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    migrate_to_cpswap_verify_writable_privileges(accounts)?;
    migrate_to_cpswap_verify_signer_privileges(accounts)?;
    Ok(())
}

pub const REMOVE_PLATFORM_CURVE_PARAM_IX_ACCOUNTS_LEN: usize = 2;

#[derive(Copy, Clone, Debug)]
pub struct RemovePlatformCurveParamAccounts<'me, 'info> {
    pub platform_admin: &'me AccountInfo<'info>,
    pub platform_config: &'me AccountInfo<'info>,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct RemovePlatformCurveParamKeys {
    pub platform_admin: Pubkey,
    pub platform_config: Pubkey,
}

impl From<RemovePlatformCurveParamAccounts<'_, '_>> for RemovePlatformCurveParamKeys {
    fn from(accounts: RemovePlatformCurveParamAccounts) -> Self {
        Self {
            platform_admin: *accounts.platform_admin.key,
            platform_config: *accounts.platform_config.key,
        }
    }
}

impl From<RemovePlatformCurveParamKeys> for [AccountMeta; REMOVE_PLATFORM_CURVE_PARAM_IX_ACCOUNTS_LEN] {
    fn from(keys: RemovePlatformCurveParamKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.platform_admin,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.platform_config,
                is_signer: false,
                is_writable: true,
            },
        ]
    }
}

impl From<[Pubkey; REMOVE_PLATFORM_CURVE_PARAM_IX_ACCOUNTS_LEN]> for RemovePlatformCurveParamKeys {
    fn from(pubkeys: [Pubkey; REMOVE_PLATFORM_CURVE_PARAM_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            platform_admin: pubkeys[0],
            platform_config: pubkeys[1],
        }
    }
}

impl<'info> From<RemovePlatformCurveParamAccounts<'_, 'info>> for [AccountInfo<'info>; REMOVE_PLATFORM_CURVE_PARAM_IX_ACCOUNTS_LEN] {
    fn from(accounts: RemovePlatformCurveParamAccounts<'_, 'info>) -> Self {
        [
            accounts.platform_admin.clone(),
            accounts.platform_config.clone(),
        ]
    }
}

impl<'me, 'info> From<&'me [AccountInfo<'info>; REMOVE_PLATFORM_CURVE_PARAM_IX_ACCOUNTS_LEN]> for RemovePlatformCurveParamAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; REMOVE_PLATFORM_CURVE_PARAM_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            platform_admin: &arr[0],
            platform_config: &arr[1],
        }
    }
}

pub const REMOVE_PLATFORM_CURVE_PARAM_IX_DISCM: [u8; 8] = [27, 30, 62, 169, 93, 224, 24, 145];

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RemovePlatformCurveParamIxArgs {
    pub index: u8,
}

#[derive(Clone, Debug, PartialEq)]
pub struct RemovePlatformCurveParamIxData(pub RemovePlatformCurveParamIxArgs);

impl From<RemovePlatformCurveParamIxArgs> for RemovePlatformCurveParamIxData {
    fn from(args: RemovePlatformCurveParamIxArgs) -> Self {
        Self(args)
    }
}

impl RemovePlatformCurveParamIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != REMOVE_PLATFORM_CURVE_PARAM_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    REMOVE_PLATFORM_CURVE_PARAM_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(RemovePlatformCurveParamIxArgs::deserialize(&mut reader)?))
    }

    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&REMOVE_PLATFORM_CURVE_PARAM_IX_DISCM)?;
        self.0.serialize(&mut writer)
    }

    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}

pub fn remove_platform_curve_param_ix_with_program_id(
    program_id: Pubkey,
    keys: RemovePlatformCurveParamKeys,
    args: RemovePlatformCurveParamIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; REMOVE_PLATFORM_CURVE_PARAM_IX_ACCOUNTS_LEN] = keys.into();
    let data: RemovePlatformCurveParamIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}

pub fn remove_platform_curve_param_ix(
    keys: RemovePlatformCurveParamKeys,
    args: RemovePlatformCurveParamIxArgs,
) -> std::io::Result<Instruction> {
    remove_platform_curve_param_ix_with_program_id(crate::ID, keys, args)
}

pub fn remove_platform_curve_param_invoke_with_program_id(
    program_id: Pubkey,
    accounts: RemovePlatformCurveParamAccounts<'_, '_>,
    args: RemovePlatformCurveParamIxArgs,
) -> ProgramResult {
    let keys: RemovePlatformCurveParamKeys = accounts.into();
    let ix = remove_platform_curve_param_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}

pub fn remove_platform_curve_param_invoke(
    accounts: RemovePlatformCurveParamAccounts<'_, '_>,
    args: RemovePlatformCurveParamIxArgs,
) -> ProgramResult {
    remove_platform_curve_param_invoke_with_program_id(crate::ID, accounts, args)
}

pub fn remove_platform_curve_param_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: RemovePlatformCurveParamAccounts<'_, '_>,
    args: RemovePlatformCurveParamIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: RemovePlatformCurveParamKeys = accounts.into();
    let ix = remove_platform_curve_param_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}

pub fn remove_platform_curve_param_invoke_signed(
    accounts: RemovePlatformCurveParamAccounts<'_, '_>,
    args: RemovePlatformCurveParamIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    remove_platform_curve_param_invoke_signed_with_program_id(crate::ID, accounts, args, seeds)
}

pub fn remove_platform_curve_param_verify_account_keys(
    accounts: RemovePlatformCurveParamAccounts<'_, '_>,
    keys: RemovePlatformCurveParamKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.platform_admin.key, keys.platform_admin),
        (*accounts.platform_config.key, keys.platform_config),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}

pub fn remove_platform_curve_param_verify_writable_privileges<'me, 'info>(
    accounts: RemovePlatformCurveParamAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.platform_config] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}

pub fn remove_platform_curve_param_verify_signer_privileges<'me, 'info>(
    accounts: RemovePlatformCurveParamAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.platform_admin] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}

pub fn remove_platform_curve_param_verify_account_privileges<'me, 'info>(
    accounts: RemovePlatformCurveParamAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    remove_platform_curve_param_verify_writable_privileges(accounts)?;
    remove_platform_curve_param_verify_signer_privileges(accounts)?;
    Ok(())
}

pub const SELL_EXACT_IN_IX_ACCOUNTS_LEN: usize = 15;
#[derive(Copy, Clone, Debug)]
pub struct SellExactInAccounts<'me, 'info> {
    pub payer: &'me AccountInfo<'info>,
    pub authority: &'me AccountInfo<'info>,
    pub global_config: &'me AccountInfo<'info>,
    pub platform_config: &'me AccountInfo<'info>,
    pub pool_state: &'me AccountInfo<'info>,
    pub user_base_token: &'me AccountInfo<'info>,
    pub user_quote_token: &'me AccountInfo<'info>,
    pub base_vault: &'me AccountInfo<'info>,
    pub quote_vault: &'me AccountInfo<'info>,
    pub base_token_mint: &'me AccountInfo<'info>,
    pub quote_token_mint: &'me AccountInfo<'info>,
    pub base_token_program: &'me AccountInfo<'info>,
    pub quote_token_program: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SellExactInKeys {
    pub payer: Pubkey,
    pub authority: Pubkey,
    pub global_config: Pubkey,
    pub platform_config: Pubkey,
    pub pool_state: Pubkey,
    pub user_base_token: Pubkey,
    pub user_quote_token: Pubkey,
    pub base_vault: Pubkey,
    pub quote_vault: Pubkey,
    pub base_token_mint: Pubkey,
    pub quote_token_mint: Pubkey,
    pub base_token_program: Pubkey,
    pub quote_token_program: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}

impl From<SellExactInAccounts<'_, '_>> for SellExactInKeys {
    fn from(accounts: SellExactInAccounts) -> Self {
        Self {
            payer: *accounts.payer.key,
            authority: *accounts.authority.key,
            global_config: *accounts.global_config.key,
            platform_config: *accounts.platform_config.key,
            pool_state: *accounts.pool_state.key,
            user_base_token: *accounts.user_base_token.key,
            user_quote_token: *accounts.user_quote_token.key,
            base_vault: *accounts.base_vault.key,
            quote_vault: *accounts.quote_vault.key,
            base_token_mint: *accounts.base_token_mint.key,
            quote_token_mint: *accounts.quote_token_mint.key,
            base_token_program: *accounts.base_token_program.key,
            quote_token_program: *accounts.quote_token_program.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}

impl From<SellExactInKeys> for [AccountMeta; SELL_EXACT_IN_IX_ACCOUNTS_LEN] {
    fn from(keys: SellExactInKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.payer,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.global_config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.platform_config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pool_state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_base_token,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_quote_token,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.base_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.quote_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.base_token_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.quote_token_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.base_token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.quote_token_program,
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

impl From<[Pubkey; SELL_EXACT_IN_IX_ACCOUNTS_LEN]> for SellExactInKeys {
    fn from(pubkeys: [Pubkey; SELL_EXACT_IN_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            payer: pubkeys[0],
            authority: pubkeys[1],
            global_config: pubkeys[2],
            platform_config: pubkeys[3],
            pool_state: pubkeys[4],
            user_base_token: pubkeys[5],
            user_quote_token: pubkeys[6],
            base_vault: pubkeys[7],
            quote_vault: pubkeys[8],
            base_token_mint: pubkeys[9],
            quote_token_mint: pubkeys[10],
            base_token_program: pubkeys[11],
            quote_token_program: pubkeys[12],
            event_authority: pubkeys[13],
            program: pubkeys[14],
        }
    }
}

impl<'info> From<SellExactInAccounts<'_, 'info>> for [AccountInfo<'info>; SELL_EXACT_IN_IX_ACCOUNTS_LEN] {
    fn from(accounts: SellExactInAccounts<'_, 'info>) -> Self {
        [
            accounts.payer.clone(),
            accounts.authority.clone(),
            accounts.global_config.clone(),
            accounts.platform_config.clone(),
            accounts.pool_state.clone(),
            accounts.user_base_token.clone(),
            accounts.user_quote_token.clone(),
            accounts.base_vault.clone(),
            accounts.quote_vault.clone(),
            accounts.base_token_mint.clone(),
            accounts.quote_token_mint.clone(),
            accounts.base_token_program.clone(),
            accounts.quote_token_program.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}

impl<'me, 'info> From<&'me [AccountInfo<'info>; SELL_EXACT_IN_IX_ACCOUNTS_LEN]> for SellExactInAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; SELL_EXACT_IN_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            payer: &arr[0],
            authority: &arr[1],
            global_config: &arr[2],
            platform_config: &arr[3],
            pool_state: &arr[4],
            user_base_token: &arr[5],
            user_quote_token: &arr[6],
            base_vault: &arr[7],
            quote_vault: &arr[8],
            base_token_mint: &arr[9],
            quote_token_mint: &arr[10],
            base_token_program: &arr[11],
            quote_token_program: &arr[12],
            event_authority: &arr[13],
            program: &arr[14],
        }
    }
}

pub const SELL_EXACT_IN_IX_DISCM: [u8; 8] = [149, 39, 222, 155, 211, 124, 152, 26];

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SellExactInIxArgs {
    pub amount_in: u64,
    pub minimum_amount_out: u64,
    pub share_fee_rate: u64,
}

#[derive(Clone, Debug, PartialEq)]
pub struct SellExactInIxData(pub SellExactInIxArgs);

impl From<SellExactInIxArgs> for SellExactInIxData {
    fn from(args: SellExactInIxArgs) -> Self {
        Self(args)
    }
}

impl SellExactInIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != SELL_EXACT_IN_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    SELL_EXACT_IN_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(SellExactInIxArgs::deserialize(&mut reader)?))
    }

    pub fn  serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&SELL_EXACT_IN_IX_DISCM)?;
        self.0.serialize(&mut writer)
    }

    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}

pub fn sell_exact_in_ix_with_program_id(
    program_id: Pubkey,
    keys: SellExactInKeys,
    args: SellExactInIxArgs,
) -> std::io::Result<Instruction>{
    let metas: [AccountMeta; SELL_EXACT_IN_IX_ACCOUNTS_LEN] = keys.into();
    let data: SellExactInIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}

pub fn sell_exact_in_ix(
    keys: SellExactInKeys,
    args: SellExactInIxArgs,
) -> std::io::Result<Instruction>{
    sell_exact_in_ix_with_program_id(crate::ID, keys, args)
}

pub fn sell_exact_in_invoke_with_program_id(
    program_id: Pubkey,
    accounts: SellExactInAccounts<'_, '_>,
    args: SellExactInIxArgs,
) -> ProgramResult {
    let keys: SellExactInKeys = accounts.into();
    let ix = sell_exact_in_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}

pub fn sell_exact_in_invoke(
    accounts: SellExactInAccounts<'_, '_>,
    args: SellExactInIxArgs,
) -> ProgramResult {
    sell_exact_in_invoke_with_program_id(crate::ID, accounts, args)
}

pub fn sell_exact_in_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: SellExactInAccounts<'_, '_>,
    args: SellExactInIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: SellExactInKeys = accounts.into();
    let ix = sell_exact_in_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}

pub fn sell_exact_in_invoke_signed(
    accounts: SellExactInAccounts<'_, '_>,
    args: SellExactInIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    sell_exact_in_invoke_signed_with_program_id(crate::ID, accounts, args, seeds)
}

pub fn sell_exact_in_verify_account_keys(
    accounts: SellExactInAccounts<'_, '_>,
    keys: SellExactInKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.payer.key, keys.payer),
        (*accounts.authority.key, keys.authority),
        (*accounts.global_config.key, keys.global_config),
        (*accounts.platform_config.key, keys.platform_config),
        (*accounts.pool_state.key, keys.pool_state),
        (*accounts.user_base_token.key, keys.user_base_token),
        (*accounts.user_quote_token.key, keys.user_quote_token),
        (*accounts.base_vault.key, keys.base_vault),
        (*accounts.quote_vault.key, keys.quote_vault),
        (*accounts.base_token_mint.key, keys.base_token_mint),
        (*accounts.quote_token_mint.key, keys.quote_token_mint),
        (*accounts.base_token_program.key, keys.base_token_program),
        (*accounts.quote_token_program.key, keys.quote_token_program),
        (*accounts.event_authority.key, keys.event_authority),
        (*accounts.program.key, keys.program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}

pub fn sell_exact_in_verify_writable_privileges<'me, 'info>(
    accounts: SellExactInAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.payer,
        accounts.pool_state,
        accounts.user_base_token,
        accounts.user_quote_token,
        accounts.base_vault,
        accounts.quote_vault,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}

pub fn sell_exact_in_verify_signer_privileges<'me, 'info>(
    accounts: SellExactInAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.payer] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}

pub fn sell_exact_in_verify_account_privileges<'me, 'info>(
    accounts: SellExactInAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    sell_exact_in_verify_writable_privileges(accounts)?;
    sell_exact_in_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const SELL_EXACT_OUT_IX_ACCOUNTS_LEN: usize = 15;

#[derive(Copy, Clone, Debug)]
pub struct SellExactOutAccounts<'me, 'info> {
    pub payer: &'me AccountInfo<'info>,
    pub authority: &'me AccountInfo<'info>,
    pub global_config: &'me AccountInfo<'info>,
    pub platform_config: &'me AccountInfo<'info>,
    pub pool_state: &'me AccountInfo<'info>,
    pub user_base_token: &'me AccountInfo<'info>,
    pub user_quote_token: &'me AccountInfo<'info>,
    pub base_vault: &'me AccountInfo<'info>,
    pub quote_vault: &'me AccountInfo<'info>,
    pub base_token_mint: &'me AccountInfo<'info>,
    pub quote_token_mint: &'me AccountInfo<'info>,
    pub base_token_program: &'me AccountInfo<'info>,
    pub quote_token_program: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SellExactOutKeys {
    pub payer: Pubkey,
    pub authority: Pubkey,
    pub global_config: Pubkey,
    pub platform_config: Pubkey,
    pub pool_state: Pubkey,
    pub user_base_token: Pubkey,
    pub user_quote_token: Pubkey,
    pub base_vault: Pubkey,
    pub quote_vault: Pubkey,
    pub base_token_mint: Pubkey,
    pub quote_token_mint: Pubkey,
    pub base_token_program: Pubkey,
    pub quote_token_program: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}

impl From<SellExactOutAccounts<'_, '_>> for SellExactOutKeys {
    fn from(accounts: SellExactOutAccounts) -> Self {
        Self {
            payer: *accounts.payer.key,
            authority: *accounts.authority.key,
            global_config: *accounts.global_config.key,
            platform_config: *accounts.platform_config.key,
            pool_state: *accounts.pool_state.key,
            user_base_token: *accounts.user_base_token.key,
            user_quote_token: *accounts.user_quote_token.key,
            base_vault: *accounts.base_vault.key,
            quote_vault: *accounts.quote_vault.key,
            base_token_mint: *accounts.base_token_mint.key,
            quote_token_mint: *accounts.quote_token_mint.key,
            base_token_program: *accounts.base_token_program.key,
            quote_token_program: *accounts.quote_token_program.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}

impl From<SellExactOutKeys> for [AccountMeta; SELL_EXACT_OUT_IX_ACCOUNTS_LEN] {
    fn from(keys: SellExactOutKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.payer,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.global_config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.platform_config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pool_state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_base_token,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_quote_token,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.base_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.quote_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.base_token_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.quote_token_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.base_token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.quote_token_program,
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

impl From<[Pubkey; SELL_EXACT_OUT_IX_ACCOUNTS_LEN]> for SellExactOutKeys {
    fn from(pubkeys: [Pubkey; SELL_EXACT_OUT_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            payer: pubkeys[0],
            authority: pubkeys[1],
            global_config: pubkeys[2],
            platform_config: pubkeys[3],
            pool_state: pubkeys[4],
            user_base_token: pubkeys[5],
            user_quote_token: pubkeys[6],
            base_vault: pubkeys[7],
            quote_vault: pubkeys[8],
            base_token_mint: pubkeys[9],
            quote_token_mint: pubkeys[10],
            base_token_program: pubkeys[11],
            quote_token_program: pubkeys[12],
            event_authority: pubkeys[13],
            program: pubkeys[14],
        }
    }
}

impl<'info> From<SellExactOutAccounts<'_, 'info>> for [AccountInfo<'info>; SELL_EXACT_OUT_IX_ACCOUNTS_LEN] {
    fn from(accounts: SellExactOutAccounts<'_, 'info>) -> Self {
        [
            accounts.payer.clone(),
            accounts.authority.clone(),
            accounts.global_config.clone(),
            accounts.platform_config.clone(),
            accounts.pool_state.clone(),
            accounts.user_base_token.clone(),
            accounts.user_quote_token.clone(),
            accounts.base_vault.clone(),
            accounts.quote_vault.clone(),
            accounts.base_token_mint.clone(),
            accounts.quote_token_mint.clone(),
            accounts.base_token_program.clone(),
            accounts.quote_token_program.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}

impl<'me, 'info> From<&'me [AccountInfo<'info>; SELL_EXACT_OUT_IX_ACCOUNTS_LEN]> for SellExactOutAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; SELL_EXACT_OUT_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            payer: &arr[0],
            authority: &arr[1],
            global_config: &arr[2],
            platform_config: &arr[3],
            pool_state: &arr[4],
            user_base_token: &arr[5],
            user_quote_token: &arr[6],
            base_vault: &arr[7],
            quote_vault: &arr[8],
            base_token_mint: &arr[9],
            quote_token_mint: &arr[10],
            base_token_program: &arr[11],
            quote_token_program: &arr[12],
            event_authority: &arr[13],
            program: &arr[14],
        }
    }
}

pub const SELL_EXACT_OUT_IX_DISCM: [u8; 8] = [95, 200, 71, 34, 8, 9, 11, 166];

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SellExactOutIxArgs {
    pub amount_out: u64,
    pub maximum_amount_in: u64,
    pub share_fee_rate: u64,
}

#[derive(Clone, Debug, PartialEq)]
pub struct SellExactOutIxData(pub SellExactOutIxArgs);

impl From<SellExactOutIxArgs> for SellExactOutIxData {
    fn from(args: SellExactOutIxArgs) -> Self {
        Self(args)
    }
}

impl SellExactOutIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != SELL_EXACT_OUT_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    SELL_EXACT_OUT_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(SellExactOutIxArgs::deserialize(&mut reader)?))
    }

    pub fn  serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&SELL_EXACT_OUT_IX_DISCM)?;
        self.0.serialize(&mut writer)
    }

    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}

pub fn sell_exact_out_ix_with_program_id(
    program_id: Pubkey,
    keys: SellExactOutKeys,
    args: SellExactOutIxArgs,
) -> std::io::Result<Instruction>{
    let metas: [AccountMeta; SELL_EXACT_OUT_IX_ACCOUNTS_LEN] = keys.into();
    let data: SellExactOutIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}

pub fn sell_exact_out_ix(
    keys: SellExactOutKeys,
    args: SellExactOutIxArgs,
) -> std::io::Result<Instruction>{
    sell_exact_out_ix_with_program_id(crate::ID, keys, args)
}

pub fn sell_exact_out_invoke_with_program_id(
    program_id: Pubkey,
    accounts: SellExactOutAccounts<'_, '_>,
    args: SellExactOutIxArgs,
) -> ProgramResult {
    let keys: SellExactOutKeys = accounts.into();
    let ix = sell_exact_out_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}

pub fn sell_exact_out_invoke(
    accounts: SellExactOutAccounts<'_, '_>,
    args: SellExactOutIxArgs,
) -> ProgramResult {
    sell_exact_out_invoke_with_program_id(crate::ID, accounts, args)
}

pub fn sell_exact_out_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: SellExactOutAccounts<'_, '_>,
    args: SellExactOutIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: SellExactOutKeys = accounts.into();
    let ix = sell_exact_out_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}

pub fn sell_exact_out_invoke_signed(
    accounts: SellExactOutAccounts<'_, '_>,
    args: SellExactOutIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    sell_exact_out_invoke_signed_with_program_id(crate::ID, accounts, args, seeds)
}

pub fn sell_exact_out_verify_account_keys(
    accounts: SellExactOutAccounts<'_, '_>,
    keys: SellExactOutKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.payer.key, keys.payer),
        (*accounts.authority.key, keys.authority),
        (*accounts.global_config.key, keys.global_config),
        (*accounts.platform_config.key, keys.platform_config),
        (*accounts.pool_state.key, keys.pool_state),
        (*accounts.user_base_token.key, keys.user_base_token),
        (*accounts.user_quote_token.key, keys.user_quote_token),
        (*accounts.base_vault.key, keys.base_vault),
        (*accounts.quote_vault.key, keys.quote_vault),
        (*accounts.base_token_mint.key, keys.base_token_mint),
        (*accounts.quote_token_mint.key, keys.quote_token_mint),
        (*accounts.base_token_program.key, keys.base_token_program),
        (*accounts.quote_token_program.key, keys.quote_token_program),
        (*accounts.event_authority.key, keys.event_authority),
        (*accounts.program.key, keys.program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}

pub fn sell_exact_out_verify_writable_privileges<'me, 'info>(
    accounts: SellExactOutAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.payer,
        accounts.pool_state,
        accounts.user_base_token,
        accounts.user_quote_token,
        accounts.base_vault,
        accounts.quote_vault,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}

pub fn sell_exact_out_verify_signer_privileges<'me, 'info>(
    accounts: SellExactOutAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.payer] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}

pub fn sell_exact_out_verify_account_privileges<'me, 'info>(
    accounts: SellExactOutAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    sell_exact_out_verify_writable_privileges(accounts)?;
    sell_exact_out_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const UPDATE_CONFIG_IX_ACCOUNTS_LEN: usize = 2;

#[derive(Copy, Clone, Debug)]
pub struct UpdateConfigAccounts<'me, 'info> {
    pub owner: &'me AccountInfo<'info>,
    pub global_config: &'me AccountInfo<'info>,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct UpdateConfigKeys {
    pub owner: Pubkey,
    pub global_config: Pubkey,
}

impl From<UpdateConfigAccounts<'_, '_>> for UpdateConfigKeys {
    fn from(accounts: UpdateConfigAccounts) -> Self {
        Self {
            owner: *accounts.owner.key,
            global_config: *accounts.global_config.key,
        }
    }
}

impl From<UpdateConfigKeys> for [AccountMeta; UPDATE_CONFIG_IX_ACCOUNTS_LEN] {
    fn from(keys: UpdateConfigKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.owner,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.global_config,
                is_signer: false,
                is_writable: true,
            },
        ]
    }
}

impl From<[Pubkey; UPDATE_CONFIG_IX_ACCOUNTS_LEN]> for UpdateConfigKeys {
    fn from(pubkeys: [Pubkey; UPDATE_CONFIG_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            owner: pubkeys[0],
            global_config: pubkeys[1],
        }
    }
}

impl<'info> From<UpdateConfigAccounts<'_, 'info>> for [AccountInfo<'info>; UPDATE_CONFIG_IX_ACCOUNTS_LEN] {
    fn from(accounts: UpdateConfigAccounts<'_, 'info>) -> Self {
        [
            accounts.owner.clone(),
            accounts.global_config.clone(),
        ]
    }
}

impl<'me, 'info> From<&'me [AccountInfo<'info>; UPDATE_CONFIG_IX_ACCOUNTS_LEN]> for UpdateConfigAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; UPDATE_CONFIG_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            owner: &arr[0],
            global_config: &arr[1],
        }
    }
}

pub const UPDATE_CONFIG_IX_DISCM: [u8; 8] = [29, 158, 252, 191, 10, 83, 219, 99];

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct UpdateConfigIxArgs {
    pub param: u8,
    pub value: u64,
}

#[derive(Clone, Debug, PartialEq)]
pub struct UpdateConfigIxData(pub UpdateConfigIxArgs);

impl From<UpdateConfigIxArgs> for UpdateConfigIxData {
    fn from(args: UpdateConfigIxArgs) -> Self {
        Self(args)
    }
}

impl UpdateConfigIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != UPDATE_CONFIG_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    UPDATE_CONFIG_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(UpdateConfigIxArgs::deserialize(&mut reader)?))
    }

    pub fn  serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&UPDATE_CONFIG_IX_DISCM)?;
        self.0.serialize(&mut writer)
    }

    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}

pub fn update_config_ix_with_program_id(
    program_id: Pubkey,
    keys: UpdateConfigKeys,
    args: UpdateConfigIxArgs,
) -> std::io::Result<Instruction>{
    let metas: [AccountMeta; UPDATE_CONFIG_IX_ACCOUNTS_LEN] = keys.into();
    let data: UpdateConfigIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}

pub fn update_config_ix(
    keys: UpdateConfigKeys,
    args: UpdateConfigIxArgs,
) -> std::io::Result<Instruction>{
    update_config_ix_with_program_id(crate::ID, keys, args)
}

pub fn update_config_invoke_with_program_id(
    program_id: Pubkey,
    accounts: UpdateConfigAccounts<'_, '_>,
    args: UpdateConfigIxArgs,
) -> ProgramResult {
    let keys: UpdateConfigKeys = accounts.into();
    let ix = update_config_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}

pub fn update_config_invoke(
    accounts: UpdateConfigAccounts<'_, '_>,
    args: UpdateConfigIxArgs,
) -> ProgramResult {
    update_config_invoke_with_program_id(crate::ID, accounts, args)
}

pub fn update_config_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: UpdateConfigAccounts<'_, '_>,
    args: UpdateConfigIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: UpdateConfigKeys = accounts.into();
    let ix = update_config_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}

pub fn update_config_invoke_signed(
    accounts: UpdateConfigAccounts<'_, '_>,
    args: UpdateConfigIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    update_config_invoke_signed_with_program_id(crate::ID, accounts, args, seeds)
}

pub fn update_config_verify_account_keys(
    accounts: UpdateConfigAccounts<'_, '_>,
    keys: UpdateConfigKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.owner.key, keys.owner),
        (*accounts.global_config.key, keys.global_config),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}

pub fn update_config_verify_writable_privileges<'me, 'info>(
    accounts: UpdateConfigAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.global_config] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}

pub fn update_config_verify_signer_privileges<'me, 'info>(
    accounts: UpdateConfigAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.owner] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}

pub fn update_config_verify_account_privileges<'me, 'info>(
    accounts: UpdateConfigAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    update_config_verify_writable_privileges(accounts)?;
    update_config_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const UPDATE_PLATFORM_CONFIG_IX_ACCOUNTS_LEN: usize = 2;

#[derive(Copy, Clone, Debug)]
pub struct UpdatePlatformConfigAccounts<'me, 'info> {
    pub platform_admin: &'me AccountInfo<'info>,
    pub platform_config: &'me AccountInfo<'info>,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct UpdatePlatformConfigKeys {
    pub platform_admin: Pubkey,
    pub platform_config: Pubkey,
}

impl From<UpdatePlatformConfigAccounts<'_, '_>> for UpdatePlatformConfigKeys {
    fn from(accounts: UpdatePlatformConfigAccounts) -> Self {
        Self {
            platform_admin: *accounts.platform_admin.key,
            platform_config: *accounts.platform_config.key,
        }
    }
}

impl From<UpdatePlatformConfigKeys> for [AccountMeta; UPDATE_PLATFORM_CONFIG_IX_ACCOUNTS_LEN] {
    fn from(keys: UpdatePlatformConfigKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.platform_admin,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.platform_config,
                is_signer: false,
                is_writable: true,
            },
        ]
    }
}

impl From<[Pubkey; UPDATE_PLATFORM_CONFIG_IX_ACCOUNTS_LEN]> for UpdatePlatformConfigKeys {
    fn from(pubkeys: [Pubkey; UPDATE_PLATFORM_CONFIG_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            platform_admin: pubkeys[0],
            platform_config: pubkeys[1],
        }
    }
}

impl<'info> From<UpdatePlatformConfigAccounts<'_, 'info>> for [AccountInfo<'info>; UPDATE_PLATFORM_CONFIG_IX_ACCOUNTS_LEN] {
    fn from(accounts: UpdatePlatformConfigAccounts<'_, 'info>) -> Self {
        [
            accounts.platform_admin.clone(),
            accounts.platform_config.clone(),
        ]
    }
}

impl<'me, 'info> From<&'me [AccountInfo<'info>; UPDATE_PLATFORM_CONFIG_IX_ACCOUNTS_LEN]> for UpdatePlatformConfigAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; UPDATE_PLATFORM_CONFIG_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            platform_admin: &arr[0],
            platform_config: &arr[1],
        }
    }
}

pub const UPDATE_PLATFORM_CONFIG_IX_DISCM: [u8; 8] = [195, 60, 76, 129, 146, 45, 67, 143];

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct UpdatePlatformConfigIxArgs {
    pub param: PlatformConfigParam,
}

#[derive(Clone, Debug, PartialEq)]
pub struct UpdatePlatformConfigIxData(pub UpdatePlatformConfigIxArgs);

impl From<UpdatePlatformConfigIxArgs> for UpdatePlatformConfigIxData {
    fn from(args: UpdatePlatformConfigIxArgs) -> Self {
        Self(args)
    }
}

impl UpdatePlatformConfigIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != UPDATE_PLATFORM_CONFIG_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    UPDATE_PLATFORM_CONFIG_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(UpdatePlatformConfigIxArgs::deserialize(&mut reader)?))
    }

    pub fn  serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&UPDATE_PLATFORM_CONFIG_IX_DISCM)?;
        self.0.serialize(&mut writer)
    }

    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}

pub fn update_platform_config_ix_with_program_id(
    program_id: Pubkey,
    keys: UpdatePlatformConfigKeys,
    args: UpdatePlatformConfigIxArgs,
) -> std::io::Result<Instruction>{
    let metas: [AccountMeta; UPDATE_PLATFORM_CONFIG_IX_ACCOUNTS_LEN] = keys.into();
    let data: UpdatePlatformConfigIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}

pub fn update_platform_config_ix(
    keys: UpdatePlatformConfigKeys,
    args: UpdatePlatformConfigIxArgs,
) -> std::io::Result<Instruction>{
    update_platform_config_ix_with_program_id(crate::ID, keys, args)
}

pub fn update_platform_config_invoke_with_program_id(
    program_id: Pubkey,
    accounts: UpdatePlatformConfigAccounts<'_, '_>,
    args: UpdatePlatformConfigIxArgs,
) -> ProgramResult {
    let keys: UpdatePlatformConfigKeys = accounts.into();
    let ix = update_platform_config_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}

pub fn update_platform_config_invoke(
    accounts: UpdatePlatformConfigAccounts<'_, '_>,
    args: UpdatePlatformConfigIxArgs,
) -> ProgramResult {
    update_platform_config_invoke_with_program_id(crate::ID, accounts, args)
}

pub fn update_platform_config_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: UpdatePlatformConfigAccounts<'_, '_>,
    args: UpdatePlatformConfigIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: UpdatePlatformConfigKeys = accounts.into();
    let ix = update_platform_config_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}

pub fn update_platform_config_invoke_signed(
    accounts: UpdatePlatformConfigAccounts<'_, '_>,
    args: UpdatePlatformConfigIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    update_platform_config_invoke_signed_with_program_id(crate::ID, accounts, args, seeds)
}

pub fn update_platform_config_verify_account_keys(
    accounts: UpdatePlatformConfigAccounts<'_, '_>,
    keys: UpdatePlatformConfigKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.platform_admin.key, keys.platform_admin),
        (*accounts.platform_config.key, keys.platform_config),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}

pub fn update_platform_config_verify_writable_privileges<'me, 'info>(
    accounts: UpdatePlatformConfigAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.platform_config] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}

pub fn update_platform_config_verify_signer_privileges<'me, 'info>(
    accounts: UpdatePlatformConfigAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.platform_admin] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}

pub fn update_platform_config_verify_account_privileges<'me, 'info>(
    accounts: UpdatePlatformConfigAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    update_platform_config_verify_writable_privileges(accounts)?;
    update_platform_config_verify_signer_privileges(accounts)?;
    Ok(())
}