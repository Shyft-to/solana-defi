#[cfg(feature = "serde")]
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

#[derive(Clone, Debug, PartialEq, EnumString, Display)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum MeteoraDBCProgramIx {
    ClaimCreatorTradingFee(ClaimCreatorTradingFeeIxArgs),
    ClaimProtocolFee,
    ClaimTradingFee(ClaimTradingFeeIxArgs),
    CloseClaimFeeOperation,
    CreateClaimFeeOperation,
    CreateConfig(CreateConfigIxArgs),
    CreateLocker,
    CreatePartnerMetadata(CreatePartnerMetadataIxArgs),
    CreateVirtualPoolMetadata(CreateVirtualPoolMetadataIxArgs),
    CreatorWithdrawSurplus,
    InitializeVirtualPoolWithSplToken(InitializeVirtualPoolWithSplTokenIxArgs),
    InitializeVirtualPoolWithToken2022(InitializeVirtualPoolWithToken2022IxArgs),
    MigrateMeteoraDamm,
    MigrateMeteoraDammClaimLpToken,
    MigrateMeteoraDammLockLpToken,
    MigrationDammV2,
    MigrationDammV2CreateMetadata,
    MigrationMeteoraDammCreateMetadata,
    PartnerWithdrawSurplus,
    ProtocolWithdrawSurplus,
    Swap(SwapIxArgs),
    Swap2(Swap2IxArgs),
    TransferPoolCreator,
    WithdrawLeftover,
    WithdrawMigrationFee(WithdrawMigrationFeeIxArgs),
}
impl MeteoraDBCProgramIx {
     pub fn name(&self) -> String {
        self.to_string().to_camel_case()
     }
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        match maybe_discm {
            CLAIM_CREATOR_TRADING_FEE_IX_DISCM => {
                Ok(Self::ClaimCreatorTradingFee(
                    ClaimCreatorTradingFeeIxArgs::deserialize(&mut reader)?,
                ))
            }
            CLAIM_PROTOCOL_FEE_IX_DISCM => Ok(Self::ClaimProtocolFee),
            CLAIM_TRADING_FEE_IX_DISCM => {
                Ok(Self::ClaimTradingFee(
                    ClaimTradingFeeIxArgs::deserialize(&mut reader)?,
                ))
            }
            CLOSE_CLAIM_FEE_OPERATOR_IX_DISCM => Ok(Self::CloseClaimFeeOperation),
            CREATE_CLAIM_FEE_OPERATION_IX_DISCM => Ok(Self::CreateClaimFeeOperation),
            CREATE_CONFIG_IX_DISCM => Ok(Self::CreateConfig(
                CreateConfigIxArgs::deserialize(&mut reader)?,
            )),
            CREATE_LOCKER_IX_DISCM => Ok(Self::CreateLocker),
            CREATE_PARTNER_METADATA_IX_DISCM => Ok(Self::CreatePartnerMetadata(
                CreatePartnerMetadataIxArgs::deserialize(&mut reader)?,
            )),
            CREATE_VIRTUAL_POOL_METADATA_IX_DISCM => Ok(Self::CreateVirtualPoolMetadata(
                CreateVirtualPoolMetadataIxArgs::deserialize(&mut reader)?,
            )),
            CREATOR_WITHDRAW_SURPLUS_IX_DISCM => Ok(Self::CreatorWithdrawSurplus),
            INITIALIZE_VIRTUAL_POOL_WITH_SPL_TOKEN_IX_DISCM => Ok(
                Self::InitializeVirtualPoolWithSplToken(
                    InitializeVirtualPoolWithSplTokenIxArgs::deserialize(&mut reader)?,
                ),
            ),
            INITIALIZE_VIRTUAL_POOL_WITH_TOKEN_2022_IX_DISCM => Ok(
                Self::InitializeVirtualPoolWithToken2022(
                    InitializeVirtualPoolWithToken2022IxArgs::deserialize(&mut reader)?,
                ),
            ),
            MIGRATE_METEORA_DAMM_IX_DISCM => Ok(Self::MigrateMeteoraDamm),
            MIGRATE_METEORA_DAMM_CLAIM_LP_TOKEN_IX_DISCM => Ok(Self::MigrateMeteoraDammClaimLpToken),
            MIGRATE_METEORA_DAMM_LOCK_LP_TOKEN_IX_DISCM => Ok(Self::MigrateMeteoraDammLockLpToken),
            MIGRATION_DAMM_V2_IX_DISCM => Ok(Self::MigrationDammV2),
            MIGRATION_DAMM_V2_CREATE_METADATA_IX_DISCM => Ok(Self::MigrationDammV2CreateMetadata),
            MIGRATION_METEORA_DAMM_CREATE_METADATA_IX_DISCM => {
                Ok(Self::MigrationMeteoraDammCreateMetadata)
            }
            PARTNER_WITHDRAW_SURPLUS_IX_DISCM => Ok(Self::PartnerWithdrawSurplus),
            PROTOCOL_WITHDRAW_SURPLUS_IX_DISCM => Ok(Self::ProtocolWithdrawSurplus),
            SWAP_IX_DISCM => {
                Ok(
                    Self::Swap(
                        SwapIxArgs::deserialize(&mut reader)?,
                    )
                )
             }
            SWAP2_IX_DISCM => {
                Ok(
                    Self::Swap2(
                        Swap2IxArgs::deserialize(&mut reader)?,
                    )
                )
            }
            TRANSFER_POOL_CREATOR_IX_DISCM => Ok(Self::TransferPoolCreator),
            WITHDRAW_LEFTOVER_IX_DISCM => Ok(Self::WithdrawLeftover),
            WITHDRAW_MIGRATION_FEE_IX_DISCM => Ok(Self::WithdrawMigrationFee(
                WithdrawMigrationFeeIxArgs::deserialize(&mut reader)?,
            )),
            _ => Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("discm {:?} not found", maybe_discm),
            )),
        }
    }

    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        match self {
            Self::ClaimCreatorTradingFee(args) => {
                writer.write_all(&CLAIM_CREATOR_TRADING_FEE_IX_DISCM)?;
                args.serialize(&mut writer)
            }
            Self::ClaimProtocolFee => writer.write_all(&CLAIM_PROTOCOL_FEE_IX_DISCM),
            Self::ClaimTradingFee(args) => {
                writer.write_all(&CLAIM_TRADING_FEE_IX_DISCM)?;
                args.serialize(&mut writer)
            }
            Self::CloseClaimFeeOperation => writer.write_all(&CLOSE_CLAIM_FEE_OPERATOR_IX_DISCM),
            Self::CreateClaimFeeOperation => writer.write_all(&CREATE_CLAIM_FEE_OPERATION_IX_DISCM),
            Self::CreateConfig(args) => {
                writer.write_all(&CREATE_CONFIG_IX_DISCM)?;
                args.serialize(&mut writer)
            }
            Self::CreateLocker => writer.write_all(&CREATE_LOCKER_IX_DISCM),
            Self::CreatePartnerMetadata(args) => {
                writer.write_all(&CREATE_PARTNER_METADATA_IX_DISCM)?;
                args.serialize(&mut writer)
            }
            Self::CreateVirtualPoolMetadata(args) => {
                writer.write_all(&CREATE_VIRTUAL_POOL_METADATA_IX_DISCM)?;
                args.serialize(&mut writer)
            }
            Self::CreatorWithdrawSurplus => writer.write_all(&CREATOR_WITHDRAW_SURPLUS_IX_DISCM),
            Self::InitializeVirtualPoolWithSplToken(args) => {
                writer.write_all(&INITIALIZE_VIRTUAL_POOL_WITH_SPL_TOKEN_IX_DISCM)?;
                args.serialize(&mut writer)
            }
            Self::InitializeVirtualPoolWithToken2022(args) => {
                writer.write_all(&INITIALIZE_VIRTUAL_POOL_WITH_TOKEN_2022_IX_DISCM)?;
                args.serialize(&mut writer)
            }
            Self::MigrateMeteoraDamm => writer.write_all(&MIGRATE_METEORA_DAMM_IX_DISCM),
            Self::MigrateMeteoraDammClaimLpToken => {
                writer.write_all(&MIGRATE_METEORA_DAMM_CLAIM_LP_TOKEN_IX_DISCM)
            }
            Self::MigrateMeteoraDammLockLpToken => {
                writer.write_all(&MIGRATE_METEORA_DAMM_LOCK_LP_TOKEN_IX_DISCM)
            }
            Self::MigrationDammV2 => writer.write_all(&MIGRATION_DAMM_V2_IX_DISCM),
            Self::MigrationDammV2CreateMetadata => {
                writer.write_all(&MIGRATION_DAMM_V2_CREATE_METADATA_IX_DISCM)
            }
            Self::MigrationMeteoraDammCreateMetadata => {
                writer.write_all(&MIGRATION_METEORA_DAMM_CREATE_METADATA_IX_DISCM)
            }
            Self::PartnerWithdrawSurplus => writer.write_all(&PARTNER_WITHDRAW_SURPLUS_IX_DISCM),
            Self::ProtocolWithdrawSurplus => writer.write_all(&PROTOCOL_WITHDRAW_SURPLUS_IX_DISCM),
            Self::Swap(args) => {
                writer.write_all(&SWAP_IX_DISCM)?;
                args.serialize(&mut writer)
            }
            Self::Swap2(args) => {
                writer.write_all(&SWAP2_IX_DISCM)?;
                args.serialize(&mut writer)
            }
            Self::TransferPoolCreator => writer.write_all(&TRANSFER_POOL_CREATOR_IX_DISCM),
            Self::WithdrawLeftover => writer.write_all(&WITHDRAW_LEFTOVER_IX_DISCM),
            Self::WithdrawMigrationFee(args) => {
                writer.write_all(&WITHDRAW_MIGRATION_FEE_IX_DISCM)?;
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

pub const CLAIM_CREATOR_TRADING_FEE_IX_ACCOUNTS_LEN: usize = 13;
#[derive(Copy, Clone, Debug)]
pub struct ClaimCreatorTradingFeeAccounts<'me, 'info> {
    pub pool_authority: &'me AccountInfo<'info>,
    pub pool: &'me AccountInfo<'info>,
    pub token_a_account: &'me AccountInfo<'info>,
    pub token_b_account: &'me AccountInfo<'info>,
    pub base_vault: &'me AccountInfo<'info>,
    pub quote_vault: &'me AccountInfo<'info>,
    pub base_mint: &'me AccountInfo<'info>,
    pub quote_mint: &'me AccountInfo<'info>,
    pub creator: &'me AccountInfo<'info>,
    pub token_base_program: &'me AccountInfo<'info>,
    pub token_quote_program: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct ClaimCreatorTradingFeeKeys {
    pub pool_authority: Pubkey,
    pub pool: Pubkey,
    pub token_a_account: Pubkey,
    pub token_b_account: Pubkey,
    pub base_vault: Pubkey,
    pub quote_vault: Pubkey,
    pub base_mint: Pubkey,
    pub quote_mint: Pubkey,
    pub creator: Pubkey,
    pub token_base_program: Pubkey,
    pub token_quote_program: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}

impl From<ClaimCreatorTradingFeeAccounts<'_, '_>> for ClaimCreatorTradingFeeKeys {
    fn from(accounts: ClaimCreatorTradingFeeAccounts) -> Self {
        Self {
            pool_authority: *accounts.pool_authority.key,
            pool: *accounts.pool.key,
            token_a_account: *accounts.token_a_account.key,
            token_b_account: *accounts.token_b_account.key,
            base_vault: *accounts.base_vault.key,
            quote_vault: *accounts.quote_vault.key,
            base_mint: *accounts.base_mint.key,
            quote_mint: *accounts.quote_mint.key,
            creator: *accounts.creator.key,
            token_base_program: *accounts.token_base_program.key,
            token_quote_program: *accounts.token_quote_program.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}

impl From<ClaimCreatorTradingFeeKeys>
    for [AccountMeta; CLAIM_CREATOR_TRADING_FEE_IX_ACCOUNTS_LEN]
{
    fn from(keys: ClaimCreatorTradingFeeKeys) -> Self {
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
                pubkey: keys.creator,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_base_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_quote_program,
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

impl From<[Pubkey; CLAIM_CREATOR_TRADING_FEE_IX_ACCOUNTS_LEN]> for ClaimCreatorTradingFeeKeys {
    fn from(pubkeys: [Pubkey; CLAIM_CREATOR_TRADING_FEE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            pool_authority: pubkeys[0],
            pool: pubkeys[1],
            token_a_account: pubkeys[2],
            token_b_account: pubkeys[3],
            base_vault: pubkeys[4],
            quote_vault: pubkeys[5],
            base_mint: pubkeys[6],
            quote_mint: pubkeys[7],
            creator: pubkeys[8],
            token_base_program: pubkeys[9],
            token_quote_program: pubkeys[10],
            event_authority: pubkeys[11],
            program: pubkeys[12],
        }
    }
}

impl<'info> From<ClaimCreatorTradingFeeAccounts<'_, 'info>>
    for [AccountInfo<'info>; CLAIM_CREATOR_TRADING_FEE_IX_ACCOUNTS_LEN]
{
    fn from(accounts: ClaimCreatorTradingFeeAccounts<'_, 'info>) -> Self {
        [
            accounts.pool_authority.clone(),
            accounts.pool.clone(),
            accounts.token_a_account.clone(),
            accounts.token_b_account.clone(),
            accounts.base_vault.clone(),
            accounts.quote_vault.clone(),
            accounts.base_mint.clone(),
            accounts.quote_mint.clone(),
            accounts.creator.clone(),
            accounts.token_base_program.clone(),
            accounts.token_quote_program.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}

impl<'me, 'info>
    From<&'me [AccountInfo<'info>; CLAIM_CREATOR_TRADING_FEE_IX_ACCOUNTS_LEN]>
    for ClaimCreatorTradingFeeAccounts<'me, 'info>
{
    fn from(
        arr: &'me [AccountInfo<'info>; CLAIM_CREATOR_TRADING_FEE_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            pool_authority: &arr[0],
            pool: &arr[1],
            token_a_account: &arr[2],
            token_b_account: &arr[3],
            base_vault: &arr[4],
            quote_vault: &arr[5],
            base_mint: &arr[6],
            quote_mint: &arr[7],
            creator: &arr[8],
            token_base_program: &arr[9],
            token_quote_program: &arr[10],
            event_authority: &arr[11],
            program: &arr[12],
        }
    }
}

pub const CLAIM_CREATOR_TRADING_FEE_IX_DISCM: [u8; 8] = [82, 220, 250, 189, 3, 85, 107, 45];

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ClaimCreatorTradingFeeIxArgs {
    pub max_base_amount: u64,
    pub max_quote_amount: u64,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ClaimCreatorTradingFeeIxData(pub ClaimCreatorTradingFeeIxArgs);

impl From<ClaimCreatorTradingFeeIxArgs> for ClaimCreatorTradingFeeIxData {
    fn from(args: ClaimCreatorTradingFeeIxArgs) -> Self {
        Self(args)
    }
}

impl ClaimCreatorTradingFeeIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != CLAIM_CREATOR_TRADING_FEE_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    CLAIM_CREATOR_TRADING_FEE_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(ClaimCreatorTradingFeeIxArgs::deserialize(
            &mut reader,
        )?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&CLAIM_CREATOR_TRADING_FEE_IX_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}

pub fn claim_creator_trading_fee_ix_with_program_id(
    program_id: Pubkey,
    keys: ClaimCreatorTradingFeeKeys,
    args: ClaimCreatorTradingFeeIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; CLAIM_CREATOR_TRADING_FEE_IX_ACCOUNTS_LEN] = keys.into();
    let data: ClaimCreatorTradingFeeIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}

pub fn claim_creator_trading_fee_ix(
    keys: ClaimCreatorTradingFeeKeys,
    args: ClaimCreatorTradingFeeIxArgs,
) -> std::io::Result<Instruction> {
    claim_creator_trading_fee_ix_with_program_id(crate::ID, keys, args)
}

pub fn claim_creator_trading_fee_invoke_with_program_id(
    program_id: Pubkey,
    accounts: ClaimCreatorTradingFeeAccounts<'_, '_>,
    args: ClaimCreatorTradingFeeIxArgs,
) -> ProgramResult {
    let keys: ClaimCreatorTradingFeeKeys = accounts.into();
    let ix = claim_creator_trading_fee_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}

pub fn claim_creator_trading_fee_invoke(
    accounts: ClaimCreatorTradingFeeAccounts<'_, '_>,
    args: ClaimCreatorTradingFeeIxArgs,
) -> ProgramResult {
    claim_creator_trading_fee_invoke_with_program_id(crate::ID, accounts, args)
}

pub fn claim_creator_trading_fee_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: ClaimCreatorTradingFeeAccounts<'_, '_>,
    args: ClaimCreatorTradingFeeIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: ClaimCreatorTradingFeeKeys = accounts.into();
    let ix = claim_creator_trading_fee_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}

pub fn claim_creator_trading_fee_invoke_signed(
    accounts: ClaimCreatorTradingFeeAccounts<'_, '_>,
    args: ClaimCreatorTradingFeeIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    claim_creator_trading_fee_invoke_signed_with_program_id(crate::ID, accounts, args, seeds)
}

pub fn claim_creator_trading_fee_verify_account_keys(
    accounts: ClaimCreatorTradingFeeAccounts<'_, '_>,
    keys: ClaimCreatorTradingFeeKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.pool_authority.key, keys.pool_authority),
        (*accounts.pool.key, keys.pool),
        (*accounts.token_a_account.key, keys.token_a_account),
        (*accounts.token_b_account.key, keys.token_b_account),
        (*accounts.base_vault.key, keys.base_vault),
        (*accounts.quote_vault.key, keys.quote_vault),
        (*accounts.base_mint.key, keys.base_mint),
        (*accounts.quote_mint.key, keys.quote_mint),
        (*accounts.creator.key, keys.creator),
        (*accounts.token_base_program.key, keys.token_base_program),
        (*accounts.token_quote_program.key, keys.token_quote_program),
        (*accounts.event_authority.key, keys.event_authority),
        (*accounts.program.key, keys.program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}

pub fn claim_creator_trading_fee_verify_is_writable_privileges<'me, 'info>(
    accounts: ClaimCreatorTradingFeeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for acct in [
        accounts.pool,
        accounts.token_a_account,
        accounts.token_b_account,
        accounts.base_vault,
        accounts.quote_vault,
    ] {
        if !acct.is_writable {
            return Err((acct, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}

pub fn claim_creator_trading_fee_verify_is_signer_privileges<'me, 'info>(
    accounts: ClaimCreatorTradingFeeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    if !accounts.creator.is_signer {
        return Err((accounts.creator, ProgramError::MissingRequiredSignature));
    }
    Ok(())
}

pub fn claim_creator_trading_fee_verify_account_privileges<'me, 'info>(
    accounts: ClaimCreatorTradingFeeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    claim_creator_trading_fee_verify_is_writable_privileges(accounts)?;
    claim_creator_trading_fee_verify_is_signer_privileges(accounts)?;
    Ok(())
}
pub const CLAIM_PROTOCOL_FEE_IX_ACCOUNTS_LEN: usize = 15;

#[derive(Copy, Clone, Debug)]
pub struct ClaimProtocolFeeAccounts<'me, 'info> {
    pub pool_authority: &'me AccountInfo<'info>,
    pub config: &'me AccountInfo<'info>,
    pub pool: &'me AccountInfo<'info>,
    pub base_vault: &'me AccountInfo<'info>,
    pub quote_vault: &'me AccountInfo<'info>,
    pub base_mint: &'me AccountInfo<'info>,
    pub quote_mint: &'me AccountInfo<'info>,
    pub token_base_account: &'me AccountInfo<'info>,
    pub token_quote_account: &'me AccountInfo<'info>,
    pub claim_fee_operator: &'me AccountInfo<'info>,
    pub operator: &'me AccountInfo<'info>,
    pub token_base_program: &'me AccountInfo<'info>,
    pub token_quote_program: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct ClaimProtocolFeeKeys {
    pub pool_authority: Pubkey,
    pub config: Pubkey,
    pub pool: Pubkey,
    pub base_vault: Pubkey,
    pub quote_vault: Pubkey,
    pub base_mint: Pubkey,
    pub quote_mint: Pubkey,
    pub token_base_account: Pubkey,
    pub token_quote_account: Pubkey,
    pub claim_fee_operator: Pubkey,
    pub operator: Pubkey,
    pub token_base_program: Pubkey,
    pub token_quote_program: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}

impl From<ClaimProtocolFeeAccounts<'_, '_>> for ClaimProtocolFeeKeys {
    fn from(accounts: ClaimProtocolFeeAccounts) -> Self {
        Self {
            pool_authority: *accounts.pool_authority.key,
            config: *accounts.config.key,
            pool: *accounts.pool.key,
            base_vault: *accounts.base_vault.key,
            quote_vault: *accounts.quote_vault.key,
            base_mint: *accounts.base_mint.key,
            quote_mint: *accounts.quote_mint.key,
            token_base_account: *accounts.token_base_account.key,
            token_quote_account: *accounts.token_quote_account.key,
            claim_fee_operator: *accounts.claim_fee_operator.key,
            operator: *accounts.operator.key,
            token_base_program: *accounts.token_base_program.key,
            token_quote_program: *accounts.token_quote_program.key,
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
                 is_writable: false 
                },
            AccountMeta {
                 pubkey: keys.config,
                 is_signer: false,
                 is_writable: false
                 },
            AccountMeta { 
                pubkey: keys.pool,
                is_signer: false, 
                is_writable: true 
            },
            AccountMeta { 
                pubkey: keys.base_vault, 
                is_signer: false, 
                is_writable: true
             },
            AccountMeta { 
                pubkey: keys.quote_vault, 
                is_signer: false, 
                is_writable: true 
            },
            AccountMeta { 
                pubkey: keys.base_mint, 
                is_signer: false, 
                is_writable: false
             },
            AccountMeta { 
                pubkey: keys.quote_mint, 
                is_signer: false, 
                is_writable: false
             },
            AccountMeta { 
                pubkey: keys.token_base_account, 
                is_signer: false, 
                is_writable: true
             },
            AccountMeta {
                 pubkey: keys.token_quote_account, 
                 is_signer: false, 
                 is_writable: true 
                },
            AccountMeta { 
                pubkey: keys.claim_fee_operator, 
                is_signer: false, 
                is_writable: false 
            },
            AccountMeta { 
                pubkey: keys.operator,
                is_signer: true, 
                is_writable: false 
            },
            AccountMeta { 
                pubkey: keys.token_base_program, 
                is_signer: false, 
                is_writable: false
             },
            AccountMeta { 
                pubkey: keys.token_quote_program, 
                is_signer: false, 
                is_writable: false 
            },
            AccountMeta {
                 pubkey: keys.event_authority, 
                 is_signer: false, 
                 is_writable: false 
                },
            AccountMeta { 
                pubkey: keys.program, 
                is_signer: false, 
                is_writable: false
             },
        ]
    }
}

impl From<[Pubkey; CLAIM_PROTOCOL_FEE_IX_ACCOUNTS_LEN]> for ClaimProtocolFeeKeys {
    fn from(pubkeys: [Pubkey; CLAIM_PROTOCOL_FEE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            pool_authority: pubkeys[0],
            config: pubkeys[1],
            pool: pubkeys[2],
            base_vault: pubkeys[3],
            quote_vault: pubkeys[4],
            base_mint: pubkeys[5],
            quote_mint: pubkeys[6],
            token_base_account: pubkeys[7],
            token_quote_account: pubkeys[8],
            claim_fee_operator: pubkeys[9],
            operator: pubkeys[10],
            token_base_program: pubkeys[11],
            token_quote_program: pubkeys[12],
            event_authority: pubkeys[13],
            program: pubkeys[14],
        }
    }
}

impl<'info> From<ClaimProtocolFeeAccounts<'_, 'info>>
    for [AccountInfo<'info>; CLAIM_PROTOCOL_FEE_IX_ACCOUNTS_LEN]
{
    fn from(accounts: ClaimProtocolFeeAccounts<'_, 'info>) -> Self {
        [
            accounts.pool_authority.clone(),
            accounts.config.clone(),
            accounts.pool.clone(),
            accounts.base_vault.clone(),
            accounts.quote_vault.clone(),
            accounts.base_mint.clone(),
            accounts.quote_mint.clone(),
            accounts.token_base_account.clone(),
            accounts.token_quote_account.clone(),
            accounts.claim_fee_operator.clone(),
            accounts.operator.clone(),
            accounts.token_base_program.clone(),
            accounts.token_quote_program.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}

impl<'me, 'info> From<&'me [AccountInfo<'info>; CLAIM_PROTOCOL_FEE_IX_ACCOUNTS_LEN]>
    for ClaimProtocolFeeAccounts<'me, 'info>
{
    fn from(arr: &'me [AccountInfo<'info>; CLAIM_PROTOCOL_FEE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            pool_authority: &arr[0],
            config: &arr[1],
            pool: &arr[2],
            base_vault: &arr[3],
            quote_vault: &arr[4],
            base_mint: &arr[5],
            quote_mint: &arr[6],
            token_base_account: &arr[7],
            token_quote_account: &arr[8],
            claim_fee_operator: &arr[9],
            operator: &arr[10],
            token_base_program: &arr[11],
            token_quote_program: &arr[12],
            event_authority: &arr[13],
            program: &arr[14],
        }
    }
}

pub const CLAIM_PROTOCOL_FEE_IX_DISCM: [u8; 8] = [165, 228, 133, 48, 99, 249, 255, 33];
#[derive(Clone, Debug, PartialEq)]
pub struct ClaimProtocolFeeIxData;

impl ClaimProtocolFeeIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != CLAIM_PROTOCOL_FEE_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("discm does not match. Expected: {:?}. Received: {:?}", CLAIM_PROTOCOL_FEE_IX_DISCM, maybe_discm),
            ));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&CLAIM_PROTOCOL_FEE_IX_DISCM)
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
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; CLAIM_PROTOCOL_FEE_IX_ACCOUNTS_LEN] = keys.into();
    let data: ClaimProtocolFeeIxData = ClaimProtocolFeeIxData;
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}

pub fn claim_protocol_fee_ix(keys: ClaimProtocolFeeKeys) -> std::io::Result<Instruction> {
    claim_protocol_fee_ix_with_program_id(crate::ID, keys)
}

pub fn claim_protocol_fee_invoke_with_program_id(
    program_id: Pubkey,
    accounts: ClaimProtocolFeeAccounts<'_, '_>,
) -> ProgramResult {
    let keys: ClaimProtocolFeeKeys = accounts.into();
    let ix = claim_protocol_fee_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}

pub fn claim_protocol_fee_invoke(accounts: ClaimProtocolFeeAccounts<'_, '_>) -> ProgramResult {
    claim_protocol_fee_invoke_with_program_id(crate::ID, accounts)
}

pub fn claim_protocol_fee_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: ClaimProtocolFeeAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: ClaimProtocolFeeKeys = accounts.into();
    let ix = claim_protocol_fee_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}

pub fn claim_protocol_fee_invoke_signed(
    accounts: ClaimProtocolFeeAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    claim_protocol_fee_invoke_signed_with_program_id(crate::ID, accounts, seeds)
}

pub fn claim_protocol_fee_verify_account_keys(
    accounts: ClaimProtocolFeeAccounts<'_, '_>,
    keys: ClaimProtocolFeeKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.pool_authority.key, keys.pool_authority),
        (*accounts.config.key, keys.config),
        (*accounts.pool.key, keys.pool),
        (*accounts.base_vault.key, keys.base_vault),
        (*accounts.quote_vault.key, keys.quote_vault),
        (*accounts.base_mint.key, keys.base_mint),
        (*accounts.quote_mint.key, keys.quote_mint),
        (*accounts.token_base_account.key, keys.token_base_account),
        (*accounts.token_quote_account.key, keys.token_quote_account),
        (*accounts.claim_fee_operator.key, keys.claim_fee_operator),
        (*accounts.operator.key, keys.operator),
        (*accounts.token_base_program.key, keys.token_base_program),
        (*accounts.token_quote_program.key, keys.token_quote_program),
        (*accounts.event_authority.key, keys.event_authority),
        (*accounts.program.key, keys.program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}

pub fn claim_protocol_fee_verify_is_writable_privileges<'me, 'info>(
    accounts: ClaimProtocolFeeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for acct in [
        accounts.pool,
        accounts.base_vault,
        accounts.quote_vault,
        accounts.token_base_account,
        accounts.token_quote_account,
    ] {
        if !acct.is_writable {
            return Err((acct, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}

pub fn claim_protocol_fee_verify_is_signer_privileges<'me, 'info>(
    accounts: ClaimProtocolFeeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    if !accounts.operator.is_signer {
        return Err((accounts.operator, ProgramError::MissingRequiredSignature));
    }
    Ok(())
}

pub fn claim_protocol_fee_verify_account_privileges<'me, 'info>(
    accounts: ClaimProtocolFeeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    claim_protocol_fee_verify_is_writable_privileges(accounts)?;
    claim_protocol_fee_verify_is_signer_privileges(accounts)?;
    Ok(())
}

pub const CLAIM_TRADING_FEE_IX_ACCOUNTS_LEN: usize = 14;

#[derive(Copy, Clone, Debug)]
pub struct ClaimTradingFeeAccounts<'me, 'info> {
    pub pool_authority: &'me AccountInfo<'info>,
    pub config: &'me AccountInfo<'info>,
    pub pool: &'me AccountInfo<'info>,
    pub token_a_account: &'me AccountInfo<'info>,
    pub token_b_account: &'me AccountInfo<'info>,
    pub base_vault: &'me AccountInfo<'info>,
    pub quote_vault: &'me AccountInfo<'info>,
    pub base_mint: &'me AccountInfo<'info>,
    pub quote_mint: &'me AccountInfo<'info>,
    pub fee_claimer: &'me AccountInfo<'info>,
    pub token_base_program: &'me AccountInfo<'info>,
    pub token_quote_program: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct ClaimTradingFeeKeys {
    pub pool_authority: Pubkey,
    pub config: Pubkey,
    pub pool: Pubkey,
    pub token_a_account: Pubkey,
    pub token_b_account: Pubkey,
    pub base_vault: Pubkey,
    pub quote_vault: Pubkey,
    pub base_mint: Pubkey,
    pub quote_mint: Pubkey,
    pub fee_claimer: Pubkey,
    pub token_base_program: Pubkey,
    pub token_quote_program: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}

impl From<ClaimTradingFeeAccounts<'_, '_>> for ClaimTradingFeeKeys {
    fn from(accounts: ClaimTradingFeeAccounts) -> Self {
        Self {
            pool_authority: *accounts.pool_authority.key,
            config: *accounts.config.key,
            pool: *accounts.pool.key,
            token_a_account: *accounts.token_a_account.key,
            token_b_account: *accounts.token_b_account.key,
            base_vault: *accounts.base_vault.key,
            quote_vault: *accounts.quote_vault.key,
            base_mint: *accounts.base_mint.key,
            quote_mint: *accounts.quote_mint.key,
            fee_claimer: *accounts.fee_claimer.key,
            token_base_program: *accounts.token_base_program.key,
            token_quote_program: *accounts.token_quote_program.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}

impl From<ClaimTradingFeeKeys> for [AccountMeta; CLAIM_TRADING_FEE_IX_ACCOUNTS_LEN] {
    fn from(keys: ClaimTradingFeeKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.pool_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.config,
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
                pubkey: keys.fee_claimer,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_base_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_quote_program,
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

impl From<[Pubkey; CLAIM_TRADING_FEE_IX_ACCOUNTS_LEN]> for ClaimTradingFeeKeys {
    fn from(pubkeys: [Pubkey; CLAIM_TRADING_FEE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            pool_authority: pubkeys[0],
            config: pubkeys[1],
            pool: pubkeys[2],
            token_a_account: pubkeys[3],
            token_b_account: pubkeys[4],
            base_vault: pubkeys[5],
            quote_vault: pubkeys[6],
            base_mint: pubkeys[7],
            quote_mint: pubkeys[8],
            fee_claimer: pubkeys[9],
            token_base_program: pubkeys[10],
            token_quote_program: pubkeys[11],
            event_authority: pubkeys[12],
            program: pubkeys[13],
        }
    }
}

impl<'info> From<ClaimTradingFeeAccounts<'_, 'info>> for [AccountInfo<'info>; CLAIM_TRADING_FEE_IX_ACCOUNTS_LEN] {
    fn from(accounts: ClaimTradingFeeAccounts<'_, 'info>) -> Self {
        [
            accounts.pool_authority.clone(),
            accounts.config.clone(),
            accounts.pool.clone(),
            accounts.token_a_account.clone(),
            accounts.token_b_account.clone(),
            accounts.base_vault.clone(),
            accounts.quote_vault.clone(),
            accounts.base_mint.clone(),
            accounts.quote_mint.clone(),
            accounts.fee_claimer.clone(),
            accounts.token_base_program.clone(),
            accounts.token_quote_program.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}

impl<'me, 'info> From<&'me [AccountInfo<'info>; CLAIM_TRADING_FEE_IX_ACCOUNTS_LEN]> for ClaimTradingFeeAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; CLAIM_TRADING_FEE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            pool_authority: &arr[0],
            config: &arr[1],
            pool: &arr[2],
            token_a_account: &arr[3],
            token_b_account: &arr[4],
            base_vault: &arr[5],
            quote_vault: &arr[6],
            base_mint: &arr[7],
            quote_mint: &arr[8],
            fee_claimer: &arr[9],
            token_base_program: &arr[10],
            token_quote_program: &arr[11],
            event_authority: &arr[12],
            program: &arr[13],
        }
    }
}

pub const CLAIM_TRADING_FEE_IX_DISCM: [u8; 8] = [8, 236, 89, 49, 152, 125, 177, 81];

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ClaimTradingFeeIxArgs {
    pub max_amount_a: u64,
    pub max_amount_b: u64,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ClaimTradingFeeIxData(pub ClaimTradingFeeIxArgs);

impl From<ClaimTradingFeeIxArgs> for ClaimTradingFeeIxData {
    fn from(args: ClaimTradingFeeIxArgs) -> Self {
        Self(args)
    }
}

impl ClaimTradingFeeIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != CLAIM_TRADING_FEE_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    CLAIM_TRADING_FEE_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(ClaimTradingFeeIxArgs::deserialize(&mut reader)?))
    }
    
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&CLAIM_TRADING_FEE_IX_DISCM)?;
        self.0.serialize(&mut writer)
    }
    
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}

pub fn claim_trading_fee_ix_with_program_id(
    program_id: Pubkey,
    keys: ClaimTradingFeeKeys,
    args: ClaimTradingFeeIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; CLAIM_TRADING_FEE_IX_ACCOUNTS_LEN] = keys.into();
    let data: ClaimTradingFeeIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}

pub fn claim_trading_fee_ix(
    keys: ClaimTradingFeeKeys,
    args: ClaimTradingFeeIxArgs,
) -> std::io::Result<Instruction> {
    claim_trading_fee_ix_with_program_id(crate::ID, keys, args)
}

pub fn claim_trading_fee_invoke_with_program_id(
    program_id: Pubkey,
    accounts: ClaimTradingFeeAccounts<'_, '_>,
    args: ClaimTradingFeeIxArgs,
) -> ProgramResult {
    let keys: ClaimTradingFeeKeys = accounts.into();
    let ix = claim_trading_fee_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}

pub fn claim_trading_fee_invoke(
    accounts: ClaimTradingFeeAccounts<'_, '_>,
    args: ClaimTradingFeeIxArgs,
) -> ProgramResult {
    claim_trading_fee_invoke_with_program_id(crate::ID, accounts, args)
}

pub fn claim_trading_fee_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: ClaimTradingFeeAccounts<'_, '_>,
    args: ClaimTradingFeeIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: ClaimTradingFeeKeys = accounts.into();
       let ix = claim_trading_fee_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}

pub fn claim_trading_fee_invoke_signed(
    accounts: ClaimTradingFeeAccounts<'_, '_>,
    args: ClaimTradingFeeIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    claim_trading_fee_invoke_signed_with_program_id(crate::ID, accounts, args, seeds)
}

pub fn claim_trading_fee_verify_account_keys(
    accounts: ClaimTradingFeeAccounts<'_, '_>,
    keys: ClaimTradingFeeKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.pool_authority.key, keys.pool_authority),
        (*accounts.config.key, keys.config),
        (*accounts.pool.key, keys.pool),
        (*accounts.token_a_account.key, keys.token_a_account),
        (*accounts.token_b_account.key, keys.token_b_account),
        (*accounts.base_vault.key, keys.base_vault),
        (*accounts.quote_vault.key, keys.quote_vault),
        (*accounts.base_mint.key, keys.base_mint),
        (*accounts.quote_mint.key, keys.quote_mint),
        (*accounts.fee_claimer.key, keys.fee_claimer),
        (*accounts.token_base_program.key, keys.token_base_program),
        (*accounts.token_quote_program.key, keys.token_quote_program),
        (*accounts.event_authority.key, keys.event_authority),
        (*accounts.program.key, keys.program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}

pub fn claim_trading_fee_verify_is_writable_privileges<'me, 'info>(
    accounts: ClaimTradingFeeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for acct in [
        accounts.pool,
        accounts.token_a_account,
        accounts.token_b_account,
        accounts.base_vault,
        accounts.quote_vault,
    ] {
        if !acct.is_writable {
            return Err((acct, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}

pub fn claim_trading_fee_verify_is_signer_privileges<'me, 'info>(
    accounts: ClaimTradingFeeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    if !accounts.fee_claimer.is_signer {
        return Err((accounts.fee_claimer, ProgramError::MissingRequiredSignature));
    }
    Ok(())
}

pub fn claim_trading_fee_verify_account_privileges<'me, 'info>(
    accounts: ClaimTradingFeeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    claim_trading_fee_verify_is_writable_privileges(accounts)?;
    claim_trading_fee_verify_is_signer_privileges(accounts)?;
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

impl From<CloseClaimFeeOperatorKeys>
    for [AccountMeta; CLOSE_CLAIM_FEE_OPERATOR_IX_ACCOUNTS_LEN]
{
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

impl<'info> From<CloseClaimFeeOperatorAccounts<'_, 'info>>
    for [AccountInfo<'info>; CLOSE_CLAIM_FEE_OPERATOR_IX_ACCOUNTS_LEN]
{
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

impl<'me, 'info>
    From<&'me [AccountInfo<'info>; CLOSE_CLAIM_FEE_OPERATOR_IX_ACCOUNTS_LEN]>
    for CloseClaimFeeOperatorAccounts<'me, 'info>
{
    fn from(
        arr: &'me [AccountInfo<'info>; CLOSE_CLAIM_FEE_OPERATOR_IX_ACCOUNTS_LEN],
    ) -> Self {
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
pub struct CloseClaimFeeOperatorIxData;

impl From<CloseClaimFeeOperatorIxArgs> for CloseClaimFeeOperatorIxData {
    fn from(_args: CloseClaimFeeOperatorIxArgs) -> Self {
        Self
    }
}

impl CloseClaimFeeOperatorIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != CLOSE_CLAIM_FEE_OPERATOR_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    CLOSE_CLAIM_FEE_OPERATOR_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&CLOSE_CLAIM_FEE_OPERATOR_IX_DISCM)?;
        Ok(())
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
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; CLOSE_CLAIM_FEE_OPERATOR_IX_ACCOUNTS_LEN] = keys.into();
    let data: CloseClaimFeeOperatorIxData = CloseClaimFeeOperatorIxArgs {}.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}

pub fn close_claim_fee_operator_ix(
    keys: CloseClaimFeeOperatorKeys,
) -> std::io::Result<Instruction> {
    close_claim_fee_operator_ix_with_program_id(crate::ID, keys)
}

pub fn close_claim_fee_operator_invoke_with_program_id(
    program_id: Pubkey,
    accounts: CloseClaimFeeOperatorAccounts<'_, '_>,
) -> ProgramResult {
    let keys: CloseClaimFeeOperatorKeys = accounts.into();
    let ix = close_claim_fee_operator_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}

pub fn close_claim_fee_operator_invoke(
    accounts: CloseClaimFeeOperatorAccounts<'_, '_>,
) -> ProgramResult {
    close_claim_fee_operator_invoke_with_program_id(crate::ID, accounts)
}

pub fn close_claim_fee_operator_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: CloseClaimFeeOperatorAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: CloseClaimFeeOperatorKeys = accounts.into();
    let ix = close_claim_fee_operator_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}

pub fn close_claim_fee_operator_invoke_signed(
    accounts: CloseClaimFeeOperatorAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    close_claim_fee_operator_invoke_signed_with_program_id(crate::ID, accounts, seeds)
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

pub fn close_claim_fee_operator_verify_is_writable_privileges<'me, 'info>(
    accounts: CloseClaimFeeOperatorAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for acct in [
        accounts.claim_fee_operator,
        accounts.rent_receiver,
    ] {
        if !acct.is_writable {
            return Err((acct, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}

pub fn close_claim_fee_operator_verify_is_signer_privileges<'me, 'info>(
    accounts: CloseClaimFeeOperatorAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    if !accounts.admin.is_signer {
        return Err((accounts.admin, ProgramError::MissingRequiredSignature));
    }
    Ok(())
}

pub fn close_claim_fee_operator_verify_account_privileges<'me, 'info>(
    accounts: CloseClaimFeeOperatorAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    close_claim_fee_operator_verify_is_writable_privileges(accounts)?;
    close_claim_fee_operator_verify_is_signer_privileges(accounts)?;
    Ok(())
}
pub const CREATE_CLAIM_FEE_OPERATION_IX_ACCOUNTS_LEN: usize = 6;

#[derive(Copy, Clone, Debug)]
pub struct CreateClaimFeeOperationAccounts<'me, 'info> {
    pub claim_fee_operator: &'me AccountInfo<'info>,
    pub operator: &'me AccountInfo<'info>,
    pub admin: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CreateClaimFeeOperationKeys {
    pub claim_fee_operator: Pubkey,
    pub operator: Pubkey,
    pub admin: Pubkey,
    pub system_program: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}

impl From<CreateClaimFeeOperationAccounts<'_, '_>> for CreateClaimFeeOperationKeys {
    fn from(accounts: CreateClaimFeeOperationAccounts) -> Self {
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

impl From<CreateClaimFeeOperationKeys>
    for [AccountMeta; CREATE_CLAIM_FEE_OPERATION_IX_ACCOUNTS_LEN]
{
    fn from(keys: CreateClaimFeeOperationKeys) -> Self {
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

impl From<[Pubkey; CREATE_CLAIM_FEE_OPERATION_IX_ACCOUNTS_LEN]> for CreateClaimFeeOperationKeys {
    fn from(pubkeys: [Pubkey; CREATE_CLAIM_FEE_OPERATION_IX_ACCOUNTS_LEN]) -> Self {
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

impl<'info> From<CreateClaimFeeOperationAccounts<'_, 'info>>
    for [AccountInfo<'info>; CREATE_CLAIM_FEE_OPERATION_IX_ACCOUNTS_LEN]
{
    fn from(accounts: CreateClaimFeeOperationAccounts<'_, 'info>) -> Self {
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

impl<'me, 'info>
    From<&'me [AccountInfo<'info>; CREATE_CLAIM_FEE_OPERATION_IX_ACCOUNTS_LEN]>
    for CreateClaimFeeOperationAccounts<'me, 'info>
{
    fn from(
        arr: &'me [AccountInfo<'info>; CREATE_CLAIM_FEE_OPERATION_IX_ACCOUNTS_LEN],
    ) -> Self {
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

pub const CREATE_CLAIM_FEE_OPERATION_IX_DISCM: [u8; 8] = [169, 62, 207, 107, 58, 187, 162, 109];

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CreateClaimFeeOperationIxArgs {}

#[derive(Clone, Debug, PartialEq)]
pub struct CreateClaimFeeOperationIxData;

impl From<CreateClaimFeeOperationIxArgs> for CreateClaimFeeOperationIxData {
    fn from(_args: CreateClaimFeeOperationIxArgs) -> Self {
        Self
    }
}

impl CreateClaimFeeOperationIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != CREATE_CLAIM_FEE_OPERATION_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    CREATE_CLAIM_FEE_OPERATION_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&CREATE_CLAIM_FEE_OPERATION_IX_DISCM)?;
        Ok(())
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}

pub fn create_claim_fee_operation_ix_with_program_id(
    program_id: Pubkey,
    keys: CreateClaimFeeOperationKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; CREATE_CLAIM_FEE_OPERATION_IX_ACCOUNTS_LEN] = keys.into();
    let data: CreateClaimFeeOperationIxData = CreateClaimFeeOperationIxArgs {}.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}

pub fn create_claim_fee_operation_ix(
    keys: CreateClaimFeeOperationKeys,
) -> std::io::Result<Instruction> {
    create_claim_fee_operation_ix_with_program_id(crate::ID, keys)
}

pub fn create_claim_fee_operation_invoke_with_program_id(
    program_id: Pubkey,
    accounts: CreateClaimFeeOperationAccounts<'_, '_>,
) -> ProgramResult {
    let keys: CreateClaimFeeOperationKeys = accounts.into();
    let ix = create_claim_fee_operation_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}

pub fn create_claim_fee_operation_invoke(
    accounts: CreateClaimFeeOperationAccounts<'_, '_>,
) -> ProgramResult {
    create_claim_fee_operation_invoke_with_program_id(crate::ID, accounts)
}

pub fn create_claim_fee_operation_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: CreateClaimFeeOperationAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: CreateClaimFeeOperationKeys = accounts.into();
    let ix = create_claim_fee_operation_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}

pub fn create_claim_fee_operation_invoke_signed(
    accounts: CreateClaimFeeOperationAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    create_claim_fee_operation_invoke_signed_with_program_id(crate::ID, accounts, seeds)
}

pub fn create_claim_fee_operation_verify_account_keys(
    accounts: CreateClaimFeeOperationAccounts<'_, '_>,
    keys: CreateClaimFeeOperationKeys,
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

pub fn create_claim_fee_operation_verify_is_writable_privileges<'me, 'info>(
    accounts: CreateClaimFeeOperationAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for acct in [
        accounts.claim_fee_operator,
        accounts.admin,
    ] {
        if !acct.is_writable {
            return Err((acct, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}

pub fn create_claim_fee_operation_verify_is_signer_privileges<'me, 'info>(
    accounts: CreateClaimFeeOperationAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    if !accounts.admin.is_signer {
        return Err((accounts.admin, ProgramError::MissingRequiredSignature));
    }
    Ok(())
}

pub fn create_claim_fee_operator_verify_account_privileges<'me, 'info>(
    accounts: CreateClaimFeeOperationAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    create_claim_fee_operation_verify_is_writable_privileges(accounts)?;
    create_claim_fee_operation_verify_is_signer_privileges(accounts)?;
    Ok(())
}
pub const CREATE_CONFIG_IX_ACCOUNTS_LEN: usize = 8;

#[derive(Copy, Clone, Debug)]
pub struct CreateConfigAccounts<'me, 'info> {
    pub config: &'me AccountInfo<'info>,
    pub fee_claimer: &'me AccountInfo<'info>,
    pub leftover_receiver: &'me AccountInfo<'info>,
    pub quote_mint: &'me AccountInfo<'info>,
    pub payer: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CreateConfigKeys {
    pub config: Pubkey,
    pub fee_claimer: Pubkey,
    pub leftover_receiver: Pubkey,
    pub quote_mint: Pubkey,
    pub payer: Pubkey,
    pub system_program: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}

impl From<CreateConfigAccounts<'_, '_>> for CreateConfigKeys {
    fn from(accounts: CreateConfigAccounts) -> Self {
        Self {
            config: *accounts.config.key,
            fee_claimer: *accounts.fee_claimer.key,
            leftover_receiver: *accounts.leftover_receiver.key,
            quote_mint: *accounts.quote_mint.key,
            payer: *accounts.payer.key,
            system_program: *accounts.system_program.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}

impl From<CreateConfigKeys>
    for [AccountMeta; CREATE_CONFIG_IX_ACCOUNTS_LEN]
{
    fn from(keys: CreateConfigKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.config,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.fee_claimer,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.leftover_receiver,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.quote_mint,
                is_signer: false,
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

impl From<[Pubkey; CREATE_CONFIG_IX_ACCOUNTS_LEN]> for CreateConfigKeys {
    fn from(pubkeys: [Pubkey; CREATE_CONFIG_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            config: pubkeys[0],
            fee_claimer: pubkeys[1],
            leftover_receiver: pubkeys[2],
            quote_mint: pubkeys[3],
            payer: pubkeys[4],
            system_program: pubkeys[5],
            event_authority: pubkeys[6],
            program: pubkeys[7],
        }
    }
}

impl<'info> From<CreateConfigAccounts<'_, 'info>>
    for [AccountInfo<'info>; CREATE_CONFIG_IX_ACCOUNTS_LEN]
{
    fn from(accounts: CreateConfigAccounts<'_, 'info>) -> Self {
        [
            accounts.config.clone(),
            accounts.fee_claimer.clone(),
            accounts.leftover_receiver.clone(),
            accounts.quote_mint.clone(),
            accounts.payer.clone(),
            accounts.system_program.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}

impl<'me, 'info>
    From<&'me [AccountInfo<'info>; CREATE_CONFIG_IX_ACCOUNTS_LEN]>
    for CreateConfigAccounts<'me, 'info>
{
    fn from(
        arr: &'me [AccountInfo<'info>; CREATE_CONFIG_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            config: &arr[0],
            fee_claimer: &arr[1],
            leftover_receiver: &arr[2],
            quote_mint: &arr[3],
            payer: &arr[4],
            system_program: &arr[5],
            event_authority: &arr[6],
            program: &arr[7],
        }
    }
}

pub const CREATE_CONFIG_IX_DISCM: [u8; 8] = [201, 207, 243, 114, 75, 111, 47, 189];

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CreateConfigIxArgs {
    pub config_parameters: ConfigParameters,
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
        (*accounts.fee_claimer.key, keys.fee_claimer),
        (*accounts.leftover_receiver.key, keys.leftover_receiver),
        (*accounts.quote_mint.key, keys.quote_mint),
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

pub fn create_config_verify_is_writable_privileges<'me, 'info>(
    accounts: CreateConfigAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for acct in [
        accounts.config,
        accounts.payer,
    ] {
        if !acct.is_writable {
            return Err((acct, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}

pub fn create_config_verify_is_signer_privileges<'me, 'info>(
    accounts: CreateConfigAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for acct in [
        accounts.config,
        accounts.payer,
    ] {
        if !acct.is_signer {
            return Err((acct, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}

pub fn create_config_verify_account_privileges<'me, 'info>(
    accounts: CreateConfigAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    create_config_verify_is_writable_privileges(accounts)?;
    create_config_verify_is_signer_privileges(accounts)?;
    Ok(())
}
pub const CREATE_LOCKER_IX_ACCOUNTS_LEN: usize = 14;

#[derive(Copy, Clone, Debug)]
pub struct CreateLockerAccounts<'me, 'info> {
    pub virtual_pool: &'me AccountInfo<'info>,
    pub config: &'me AccountInfo<'info>,
    pub pool_authority: &'me AccountInfo<'info>,
    pub base_vault: &'me AccountInfo<'info>,
    pub base_mint: &'me AccountInfo<'info>,
    pub base: &'me AccountInfo<'info>,
    pub creator: &'me AccountInfo<'info>,
    pub escrow: &'me AccountInfo<'info>,
    pub escrow_token: &'me AccountInfo<'info>,
    pub payer: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub locker_program: &'me AccountInfo<'info>,
    pub locker_event_authority: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CreateLockerKeys {
    pub virtual_pool: Pubkey,
    pub config: Pubkey,
    pub pool_authority: Pubkey,
    pub base_vault: Pubkey,
    pub base_mint: Pubkey,
    pub base: Pubkey,
    pub creator: Pubkey,
    pub escrow: Pubkey,
    pub escrow_token: Pubkey,
    pub payer: Pubkey,
    pub token_program: Pubkey,
    pub locker_program: Pubkey,
    pub locker_event_authority: Pubkey,
    pub system_program: Pubkey,
}

impl From<CreateLockerAccounts<'_, '_>> for CreateLockerKeys {
    fn from(accounts: CreateLockerAccounts) -> Self {
        Self {
            virtual_pool: *accounts.virtual_pool.key,
            config: *accounts.config.key,
            pool_authority: *accounts.pool_authority.key,
            base_vault: *accounts.base_vault.key,
            base_mint: *accounts.base_mint.key,
            base: *accounts.base.key,
            creator: *accounts.creator.key,
            escrow: *accounts.escrow.key,
            escrow_token: *accounts.escrow_token.key,
            payer: *accounts.payer.key,
            token_program: *accounts.token_program.key,
            locker_program: *accounts.locker_program.key,
            locker_event_authority: *accounts.locker_event_authority.key,
            system_program: *accounts.system_program.key,
        }
    }
}

impl From<CreateLockerKeys> for [AccountMeta; CREATE_LOCKER_IX_ACCOUNTS_LEN] {
    fn from(keys: CreateLockerKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.virtual_pool,
                is_signer: false,
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
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.base_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.base_mint,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.base,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.creator,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.escrow,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.escrow_token,
                is_signer: false,
                is_writable: true,
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
                pubkey: keys.locker_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.locker_event_authority,
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

impl From<[Pubkey; CREATE_LOCKER_IX_ACCOUNTS_LEN]> for CreateLockerKeys {
    fn from(pubkeys: [Pubkey; CREATE_LOCKER_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            virtual_pool: pubkeys[0],
            config: pubkeys[1],
            pool_authority: pubkeys[2],
            base_vault: pubkeys[3],
            base_mint: pubkeys[4],
            base: pubkeys[5],
            creator: pubkeys[6],
            escrow: pubkeys[7],
            escrow_token: pubkeys[8],
            payer: pubkeys[9],
            token_program: pubkeys[10],
            locker_program: pubkeys[11],
            locker_event_authority: pubkeys[12],
            system_program: pubkeys[13],
        }
    }
}

impl<'info> From<CreateLockerAccounts<'_, 'info>> for [AccountInfo<'info>; CREATE_LOCKER_IX_ACCOUNTS_LEN] {
    fn from(accounts: CreateLockerAccounts<'_, 'info>) -> Self {
        [
            accounts.virtual_pool.clone(),
            accounts.config.clone(),
            accounts.pool_authority.clone(),
            accounts.base_vault.clone(),
            accounts.base_mint.clone(),
            accounts.base.clone(),
            accounts.creator.clone(),
            accounts.escrow.clone(),
            accounts.escrow_token.clone(),
            accounts.payer.clone(),
            accounts.token_program.clone(),
            accounts.locker_program.clone(),
            accounts.locker_event_authority.clone(),
            accounts.system_program.clone(),
        ]
    }
}

impl<'me, 'info> From<&'me [AccountInfo<'info>; CREATE_LOCKER_IX_ACCOUNTS_LEN]> for CreateLockerAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; CREATE_LOCKER_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            virtual_pool: &arr[0],
            config: &arr[1],
            pool_authority: &arr[2],
            base_vault: &arr[3],
            base_mint: &arr[4],
            base: &arr[5],
            creator: &arr[6],
            escrow: &arr[7],
            escrow_token: &arr[8],
            payer: &arr[9],
            token_program: &arr[10],
            locker_program: &arr[11],
            locker_event_authority: &arr[12],
            system_program: &arr[13],
        }
    }
}

pub const CREATE_LOCKER_IX_DISCM: [u8; 8] = [167, 90, 137, 154, 75, 47, 17, 84];

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CreateLockerIxArgs {}

#[derive(Clone, Debug, PartialEq)]
pub struct CreateLockerIxData(pub CreateLockerIxArgs);

impl From<CreateLockerIxArgs> for CreateLockerIxData {
    fn from(args: CreateLockerIxArgs) -> Self {
        Self(args)
    }
}

impl CreateLockerIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != CREATE_LOCKER_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    CREATE_LOCKER_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(CreateLockerIxArgs::deserialize(&mut reader)?))
    }
    
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&CREATE_LOCKER_IX_DISCM)?;
        self.0.serialize(&mut writer)
    }
    
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}

pub fn create_locker_ix_with_program_id(
    program_id: Pubkey,
    keys: CreateLockerKeys,
    args: CreateLockerIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; CREATE_LOCKER_IX_ACCOUNTS_LEN] = keys.into();
    let data: CreateLockerIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}

pub fn create_locker_ix(
    keys: CreateLockerKeys,
    args: CreateLockerIxArgs,
) -> std::io::Result<Instruction> {
    create_locker_ix_with_program_id(crate::ID, keys, args)
}

pub fn create_locker_invoke_with_program_id(
    program_id: Pubkey,
    accounts: CreateLockerAccounts<'_, '_>,
    args: CreateLockerIxArgs,
) -> ProgramResult {
    let keys: CreateLockerKeys = accounts.into();
    let ix = create_locker_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}

pub fn create_locker_invoke(
    accounts: CreateLockerAccounts<'_, '_>,
    args: CreateLockerIxArgs,
) -> ProgramResult {
    create_locker_invoke_with_program_id(crate::ID, accounts, args)
}

pub fn create_locker_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: CreateLockerAccounts<'_, '_>,
    args: CreateLockerIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: CreateLockerKeys = accounts.into();
    let ix = create_locker_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}

pub fn create_locker_invoke_signed(
    accounts: CreateLockerAccounts<'_, '_>,
    args: CreateLockerIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    create_locker_invoke_signed_with_program_id(crate::ID, accounts, args, seeds)
}

pub fn create_locker_verify_account_keys(
    accounts: CreateLockerAccounts<'_, '_>,
    keys: CreateLockerKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.virtual_pool.key, keys.virtual_pool),
        (*accounts.config.key, keys.config),
        (*accounts.pool_authority.key, keys.pool_authority),
        (*accounts.base_vault.key, keys.base_vault),
        (*accounts.base_mint.key, keys.base_mint),
        (*accounts.base.key, keys.base),
        (*accounts.creator.key, keys.creator),
        (*accounts.escrow.key, keys.escrow),
        (*accounts.escrow_token.key, keys.escrow_token),
        (*accounts.payer.key, keys.payer),
        (*accounts.token_program.key, keys.token_program),
        (*accounts.locker_program.key, keys.locker_program),
        (*accounts.locker_event_authority.key, keys.locker_event_authority),
        (*accounts.system_program.key, keys.system_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}

pub fn create_locker_verify_is_writable_privileges<'me, 'info>(
    accounts: CreateLockerAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for acct in [
        accounts.virtual_pool,
        accounts.pool_authority,
        accounts.base_vault,
        accounts.base_mint,
        accounts.base,
        accounts.escrow,
        accounts.escrow_token,
        accounts.payer,
    ] {
        if !acct.is_writable {
            return Err((acct, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}

pub fn create_locker_verify_is_signer_privileges<'me, 'info>(
    accounts: CreateLockerAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    if !accounts.payer.is_signer {
        return Err((accounts.payer, ProgramError::MissingRequiredSignature));
    }
    Ok(())
}

pub fn create_locker_verify_account_privileges<'me, 'info>(
    accounts: CreateLockerAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    create_locker_verify_is_writable_privileges(accounts)?;
    create_locker_verify_is_signer_privileges(accounts)?;
    Ok(())
}
pub const CREATE_PARTNER_METADATA_IX_ACCOUNTS_LEN: usize = 6;

#[derive(Copy, Clone, Debug)]
pub struct CreatePartnerMetadataAccounts<'me, 'info> {
    pub partner_metadata: &'me AccountInfo<'info>,
    pub payer: &'me AccountInfo<'info>,
    pub fee_claimer: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CreatePartnerMetadataKeys {
    pub partner_metadata: Pubkey,
    pub payer: Pubkey,
    pub fee_claimer: Pubkey,
    pub system_program: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}

impl From<CreatePartnerMetadataAccounts<'_, '_>> for CreatePartnerMetadataKeys {
    fn from(accounts: CreatePartnerMetadataAccounts) -> Self {
        Self {
            partner_metadata: *accounts.partner_metadata.key,
            payer: *accounts.payer.key,
            fee_claimer: *accounts.fee_claimer.key,
            system_program: *accounts.system_program.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}

impl From<CreatePartnerMetadataKeys> for [AccountMeta; CREATE_PARTNER_METADATA_IX_ACCOUNTS_LEN] {
    fn from(keys: CreatePartnerMetadataKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.partner_metadata,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.payer,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.fee_claimer,
                is_signer: true,
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

impl From<[Pubkey; CREATE_PARTNER_METADATA_IX_ACCOUNTS_LEN]> for CreatePartnerMetadataKeys {
    fn from(pubkeys: [Pubkey; CREATE_PARTNER_METADATA_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            partner_metadata: pubkeys[0],
            payer: pubkeys[1],
            fee_claimer: pubkeys[2],
            system_program: pubkeys[3],
            event_authority: pubkeys[4],
            program: pubkeys[5],
        }
    }
}

impl<'info> From<CreatePartnerMetadataAccounts<'_, 'info>> for [AccountInfo<'info>; CREATE_PARTNER_METADATA_IX_ACCOUNTS_LEN] {
    fn from(accounts: CreatePartnerMetadataAccounts<'_, 'info>) -> Self {
        [
            accounts.partner_metadata.clone(),
            accounts.payer.clone(),
            accounts.fee_claimer.clone(),
            accounts.system_program.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}

impl<'me, 'info> From<&'me [AccountInfo<'info>; CREATE_PARTNER_METADATA_IX_ACCOUNTS_LEN]> for CreatePartnerMetadataAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; CREATE_PARTNER_METADATA_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            partner_metadata: &arr[0],
            payer: &arr[1],
            fee_claimer: &arr[2],
            system_program: &arr[3],
            event_authority: &arr[4],
            program: &arr[5],
        }
    }
}

pub const CREATE_PARTNER_METADATA_IX_DISCM: [u8; 8] = [192, 168, 234, 191, 188, 226, 227, 255];


#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CreatePartnerMetadataIxArgs {
    pub metadata: CreatePartnerMetadataParameters,
}

#[derive(Clone, Debug, PartialEq)]
pub struct CreatePartnerMetadataIxData(pub CreatePartnerMetadataIxArgs);

impl From<CreatePartnerMetadataIxArgs> for CreatePartnerMetadataIxData {
    fn from(args: CreatePartnerMetadataIxArgs) -> Self {
        Self(args)
    }
}

impl CreatePartnerMetadataIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != CREATE_PARTNER_METADATA_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    CREATE_PARTNER_METADATA_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(CreatePartnerMetadataIxArgs::deserialize(&mut reader)?))
    }
    
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&CREATE_PARTNER_METADATA_IX_DISCM)?;
        self.0.serialize(&mut writer)
    }
    
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}

pub fn create_partner_metadata_ix_with_program_id(
    program_id: Pubkey,
    keys: CreatePartnerMetadataKeys,
    args: CreatePartnerMetadataIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; CREATE_PARTNER_METADATA_IX_ACCOUNTS_LEN] = keys.into();
    let data: CreatePartnerMetadataIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}

pub fn create_partner_metadata_ix(
    keys: CreatePartnerMetadataKeys,
    args: CreatePartnerMetadataIxArgs,
) -> std::io::Result<Instruction> {
    create_partner_metadata_ix_with_program_id(crate::ID, keys, args)
}

pub fn create_partner_metadata_invoke_with_program_id(
    program_id: Pubkey,
    accounts: CreatePartnerMetadataAccounts<'_, '_>,
    args: CreatePartnerMetadataIxArgs,
) -> ProgramResult {
    let keys: CreatePartnerMetadataKeys = accounts.into();
    let ix = create_partner_metadata_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}

pub fn create_partner_metadata_invoke(
    accounts: CreatePartnerMetadataAccounts<'_, '_>,
    args: CreatePartnerMetadataIxArgs,
) -> ProgramResult {
    create_partner_metadata_invoke_with_program_id(crate::ID, accounts, args)
}

pub fn create_partner_metadata_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: CreatePartnerMetadataAccounts<'_, '_>,
    args: CreatePartnerMetadataIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: CreatePartnerMetadataKeys = accounts.into();
    let ix = create_partner_metadata_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}

pub fn create_partner_metadata_invoke_signed(
    accounts: CreatePartnerMetadataAccounts<'_, '_>,
    args: CreatePartnerMetadataIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    create_partner_metadata_invoke_signed_with_program_id(crate::ID, accounts, args, seeds)
}

pub fn create_partner_metadata_verify_account_keys(
    accounts: CreatePartnerMetadataAccounts<'_, '_>,
    keys: CreatePartnerMetadataKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.partner_metadata.key, keys.partner_metadata),
        (*accounts.payer.key, keys.payer),
        (*accounts.fee_claimer.key, keys.fee_claimer),
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

pub fn create_partner_metadata_verify_is_writable_privileges<'me, 'info>(
    accounts: CreatePartnerMetadataAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for acct in [
        accounts.partner_metadata,
        accounts.payer,
    ] {
        if !acct.is_writable {
            return Err((acct, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}

pub fn create_partner_metadata_verify_is_signer_privileges<'me, 'info>(
    accounts: CreatePartnerMetadataAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for acct in [
        accounts.payer,
        accounts.fee_claimer,
    ] {
        if !acct.is_signer {
            return Err((acct, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}

pub fn create_partner_metadata_verify_account_privileges<'me, 'info>(
    accounts: CreatePartnerMetadataAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    create_partner_metadata_verify_is_writable_privileges(accounts)?;
    create_partner_metadata_verify_is_signer_privileges(accounts)?;
    Ok(())
}
pub const CREATE_VIRTUAL_POOL_METADATA_IX_ACCOUNTS_LEN: usize = 7;

#[derive(Copy, Clone, Debug)]
pub struct CreateVirtualPoolMetadataAccounts<'me, 'info> {
    pub virtual_pool: &'me AccountInfo<'info>,
    pub virtual_pool_metadata: &'me AccountInfo<'info>,
    pub creator: &'me AccountInfo<'info>,
    pub payer: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CreateVirtualPoolMetadataKeys {
    pub virtual_pool: Pubkey,
    pub virtual_pool_metadata: Pubkey,
    pub creator: Pubkey,
    pub payer: Pubkey,
    pub system_program: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}

impl From<CreateVirtualPoolMetadataAccounts<'_, '_>> for CreateVirtualPoolMetadataKeys {
    fn from(accounts: CreateVirtualPoolMetadataAccounts) -> Self {
        Self {
            virtual_pool: *accounts.virtual_pool.key,
            virtual_pool_metadata: *accounts.virtual_pool_metadata.key,
            creator: *accounts.creator.key,
            payer: *accounts.payer.key,
            system_program: *accounts.system_program.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}

impl From<CreateVirtualPoolMetadataKeys> for [AccountMeta; CREATE_VIRTUAL_POOL_METADATA_IX_ACCOUNTS_LEN] {
    fn from(keys: CreateVirtualPoolMetadataKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.virtual_pool,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.virtual_pool_metadata,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.creator,
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

impl From<[Pubkey; CREATE_VIRTUAL_POOL_METADATA_IX_ACCOUNTS_LEN]> for CreateVirtualPoolMetadataKeys {
    fn from(pubkeys: [Pubkey; CREATE_VIRTUAL_POOL_METADATA_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            virtual_pool: pubkeys[0],
            virtual_pool_metadata: pubkeys[1],
            creator: pubkeys[2],
            payer: pubkeys[3],
            system_program: pubkeys[4],
            event_authority: pubkeys[5],
            program: pubkeys[6],
        }
    }
}

impl<'info> From<CreateVirtualPoolMetadataAccounts<'_, 'info>> for [AccountInfo<'info>; CREATE_VIRTUAL_POOL_METADATA_IX_ACCOUNTS_LEN] {
    fn from(accounts: CreateVirtualPoolMetadataAccounts<'_, 'info>) -> Self {
        [
            accounts.virtual_pool.clone(),
            accounts.virtual_pool_metadata.clone(),
            accounts.creator.clone(),
            accounts.payer.clone(),
            accounts.system_program.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}

impl<'me, 'info> From<&'me [AccountInfo<'info>; CREATE_VIRTUAL_POOL_METADATA_IX_ACCOUNTS_LEN]> for CreateVirtualPoolMetadataAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; CREATE_VIRTUAL_POOL_METADATA_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            virtual_pool: &arr[0],
            virtual_pool_metadata: &arr[1],
            creator: &arr[2],
            payer: &arr[3],
            system_program: &arr[4],
            event_authority: &arr[5],
            program: &arr[6],
        }
    }
}

pub const CREATE_VIRTUAL_POOL_METADATA_IX_DISCM: [u8; 8] = [45, 97, 187, 103, 254, 109, 124, 134];
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CreateVirtualPoolMetadataIxArgs {
    pub metadata: CreateVirtualPoolMetadataParameters,
}

#[derive(Clone, Debug, PartialEq)]
pub struct CreateVirtualPoolMetadataIxData(pub CreateVirtualPoolMetadataIxArgs);

impl From<CreateVirtualPoolMetadataIxArgs> for CreateVirtualPoolMetadataIxData {
    fn from(args: CreateVirtualPoolMetadataIxArgs) -> Self {
        Self(args)
    }
}

impl CreateVirtualPoolMetadataIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != CREATE_VIRTUAL_POOL_METADATA_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    CREATE_VIRTUAL_POOL_METADATA_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(CreateVirtualPoolMetadataIxArgs::deserialize(&mut reader)?))
    }
    
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&CREATE_VIRTUAL_POOL_METADATA_IX_DISCM)?;
        self.0.serialize(&mut writer)
    }
    
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}

pub fn create_virtual_pool_metadata_ix_with_program_id(
    program_id: Pubkey,
    keys: CreateVirtualPoolMetadataKeys,
    args: CreateVirtualPoolMetadataIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; CREATE_VIRTUAL_POOL_METADATA_IX_ACCOUNTS_LEN] = keys.into();
    let data: CreateVirtualPoolMetadataIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}

pub fn create_virtual_pool_metadata_ix(
    keys: CreateVirtualPoolMetadataKeys,
    args: CreateVirtualPoolMetadataIxArgs,
) -> std::io::Result<Instruction> {
    create_virtual_pool_metadata_ix_with_program_id(crate::ID, keys, args)
}

pub fn create_virtual_pool_metadata_invoke_with_program_id(
    program_id: Pubkey,
    accounts: CreateVirtualPoolMetadataAccounts<'_, '_>,
    args: CreateVirtualPoolMetadataIxArgs,
) -> ProgramResult {
    let keys: CreateVirtualPoolMetadataKeys = accounts.into();
    let ix = create_virtual_pool_metadata_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}

pub fn create_virtual_pool_metadata_invoke(
    accounts: CreateVirtualPoolMetadataAccounts<'_, '_>,
    args: CreateVirtualPoolMetadataIxArgs,
) -> ProgramResult {
    create_virtual_pool_metadata_invoke_with_program_id(crate::ID, accounts, args)
}

pub fn create_virtual_pool_metadata_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: CreateVirtualPoolMetadataAccounts<'_, '_>,
    args: CreateVirtualPoolMetadataIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: CreateVirtualPoolMetadataKeys = accounts.into();
    let ix = create_virtual_pool_metadata_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}

pub fn create_virtual_pool_metadata_invoke_signed(
    accounts: CreateVirtualPoolMetadataAccounts<'_, '_>,
    args: CreateVirtualPoolMetadataIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    create_virtual_pool_metadata_invoke_signed_with_program_id(crate::ID, accounts, args, seeds)
}

pub fn create_virtual_pool_metadata_verify_account_keys(
    accounts: CreateVirtualPoolMetadataAccounts<'_, '_>,
    keys: CreateVirtualPoolMetadataKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.virtual_pool.key, keys.virtual_pool),
        (*accounts.virtual_pool_metadata.key, keys.virtual_pool_metadata),
        (*accounts.creator.key, keys.creator),
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

pub fn create_virtual_pool_metadata_verify_is_writable_privileges<'me, 'info>(
    accounts: CreateVirtualPoolMetadataAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for acct in [
        accounts.virtual_pool,
        accounts.virtual_pool_metadata,
        accounts.payer,
    ] {
        if !acct.is_writable {
            return Err((acct, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}

pub fn create_virtual_pool_metadata_verify_is_signer_privileges<'me, 'info>(
    accounts: CreateVirtualPoolMetadataAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for acct in [
        accounts.creator,
        accounts.payer,
    ] {
        if !acct.is_signer {
            return Err((acct, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}

pub fn create_virtual_pool_metadata_verify_account_privileges<'me, 'info>(
    accounts: CreateVirtualPoolMetadataAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    create_virtual_pool_metadata_verify_is_writable_privileges(accounts)?;
    create_virtual_pool_metadata_verify_is_signer_privileges(accounts)?;
    Ok(())
}
pub const CREATOR_WITHDRAW_SURPLUS_IX_ACCOUNTS_LEN: usize = 10;

#[derive(Copy, Clone, Debug)]
pub struct CreatorWithdrawSurplusAccounts<'me, 'info> {
    pub pool_authority: &'me AccountInfo<'info>,
    pub config: &'me AccountInfo<'info>,
    pub virtual_pool: &'me AccountInfo<'info>,
    pub token_quote_account: &'me AccountInfo<'info>,
    pub quote_vault: &'me AccountInfo<'info>,
    pub quote_mint: &'me AccountInfo<'info>,
    pub creator: &'me AccountInfo<'info>,
    pub token_quote_program: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CreatorWithdrawSurplusKeys {
    pub pool_authority: Pubkey,
    pub config: Pubkey,
    pub virtual_pool: Pubkey,
    pub token_quote_account: Pubkey,
    pub quote_vault: Pubkey,
    pub quote_mint: Pubkey,
    pub creator: Pubkey,
    pub token_quote_program: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}

impl From<CreatorWithdrawSurplusAccounts<'_, '_>> for CreatorWithdrawSurplusKeys {
    fn from(accounts: CreatorWithdrawSurplusAccounts) -> Self {
        Self {
            pool_authority: *accounts.pool_authority.key,
            config: *accounts.config.key,
            virtual_pool: *accounts.virtual_pool.key,
            token_quote_account: *accounts.token_quote_account.key,
            quote_vault: *accounts.quote_vault.key,
            quote_mint: *accounts.quote_mint.key,
            creator: *accounts.creator.key,
            token_quote_program: *accounts.token_quote_program.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}

impl From<CreatorWithdrawSurplusKeys> for [AccountMeta; CREATOR_WITHDRAW_SURPLUS_IX_ACCOUNTS_LEN] {
    fn from(keys: CreatorWithdrawSurplusKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.pool_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.virtual_pool,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_quote_account,
                is_signer: false,
                is_writable: true,
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
                pubkey: keys.creator,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_quote_program,
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

impl From<[Pubkey; CREATOR_WITHDRAW_SURPLUS_IX_ACCOUNTS_LEN]> for CreatorWithdrawSurplusKeys {
    fn from(pubkeys: [Pubkey; CREATOR_WITHDRAW_SURPLUS_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            pool_authority: pubkeys[0],
            config: pubkeys[1],
            virtual_pool: pubkeys[2],
            token_quote_account: pubkeys[3],
            quote_vault: pubkeys[4],
            quote_mint: pubkeys[5],
            creator: pubkeys[6],
            token_quote_program: pubkeys[7],
            event_authority: pubkeys[8],
            program: pubkeys[9],
        }
    }
}

impl<'info> From<CreatorWithdrawSurplusAccounts<'_, 'info>> for [AccountInfo<'info>; CREATOR_WITHDRAW_SURPLUS_IX_ACCOUNTS_LEN] {
    fn from(accounts: CreatorWithdrawSurplusAccounts<'_, 'info>) -> Self {
        [
            accounts.pool_authority.clone(),
            accounts.config.clone(),
            accounts.virtual_pool.clone(),
            accounts.token_quote_account.clone(),
            accounts.quote_vault.clone(),
            accounts.quote_mint.clone(),
            accounts.creator.clone(),
            accounts.token_quote_program.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}

impl<'me, 'info> From<&'me [AccountInfo<'info>; CREATOR_WITHDRAW_SURPLUS_IX_ACCOUNTS_LEN]> for CreatorWithdrawSurplusAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; CREATOR_WITHDRAW_SURPLUS_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            pool_authority: &arr[0],
            config: &arr[1],
            virtual_pool: &arr[2],
            token_quote_account: &arr[3],
            quote_vault: &arr[4],
            quote_mint: &arr[5],
            creator: &arr[6],
            token_quote_program: &arr[7],
            event_authority: &arr[8],
            program: &arr[9],
        }
    }
}

pub const CREATOR_WITHDRAW_SURPLUS_IX_DISCM: [u8; 8] = [165, 3, 137, 7, 28, 134, 76, 80];

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CreatorWithdrawSurplusIxArgs {}

#[derive(Clone, Debug, PartialEq)]
pub struct CreatorWithdrawSurplusIxData(pub CreatorWithdrawSurplusIxArgs);

impl From<CreatorWithdrawSurplusIxArgs> for CreatorWithdrawSurplusIxData {
    fn from(args: CreatorWithdrawSurplusIxArgs) -> Self {
        Self(args)
    }
}

impl CreatorWithdrawSurplusIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != CREATOR_WITHDRAW_SURPLUS_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    CREATOR_WITHDRAW_SURPLUS_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(CreatorWithdrawSurplusIxArgs::deserialize(&mut reader)?))
    }
    
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&CREATOR_WITHDRAW_SURPLUS_IX_DISCM)?;
        self.0.serialize(&mut writer)
    }
    
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}

pub fn creator_withdraw_surplus_ix_with_program_id(
    program_id: Pubkey,
    keys: CreatorWithdrawSurplusKeys,
    args: CreatorWithdrawSurplusIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; CREATOR_WITHDRAW_SURPLUS_IX_ACCOUNTS_LEN] = keys.into();
    let data: CreatorWithdrawSurplusIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}

pub fn creator_withdraw_surplus_ix(
    keys: CreatorWithdrawSurplusKeys,
    args: CreatorWithdrawSurplusIxArgs,
) -> std::io::Result<Instruction> {
    creator_withdraw_surplus_ix_with_program_id(crate::ID, keys, args)
}

pub fn creator_withdraw_surplus_invoke_with_program_id(
    program_id: Pubkey,
    accounts: CreatorWithdrawSurplusAccounts<'_, '_>,
    args: CreatorWithdrawSurplusIxArgs,
) -> ProgramResult {
    let keys: CreatorWithdrawSurplusKeys = accounts.into();
    let ix = creator_withdraw_surplus_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}

pub fn creator_withdraw_surplus_invoke(
    accounts: CreatorWithdrawSurplusAccounts<'_, '_>,
    args: CreatorWithdrawSurplusIxArgs,
) -> ProgramResult {
    creator_withdraw_surplus_invoke_with_program_id(crate::ID, accounts, args)
}

pub fn creator_withdraw_surplus_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: CreatorWithdrawSurplusAccounts<'_, '_>,
    args: CreatorWithdrawSurplusIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: CreatorWithdrawSurplusKeys = accounts.into();
    let ix = creator_withdraw_surplus_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}

pub fn creator_withdraw_surplus_invoke_signed(
    accounts: CreatorWithdrawSurplusAccounts<'_, '_>,
    args: CreatorWithdrawSurplusIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    creator_withdraw_surplus_invoke_signed_with_program_id(crate::ID, accounts, args, seeds)
}

pub fn creator_withdraw_surplus_verify_account_keys(
    accounts: CreatorWithdrawSurplusAccounts<'_, '_>,
    keys: CreatorWithdrawSurplusKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.pool_authority.key, keys.pool_authority),
        (*accounts.config.key, keys.config),
        (*accounts.virtual_pool.key, keys.virtual_pool),
        (*accounts.token_quote_account.key, keys.token_quote_account),
        (*accounts.quote_vault.key, keys.quote_vault),
        (*accounts.quote_mint.key, keys.quote_mint),
        (*accounts.creator.key, keys.creator),
        (*accounts.token_quote_program.key, keys.token_quote_program),
        (*accounts.event_authority.key, keys.event_authority),
        (*accounts.program.key, keys.program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}

pub fn creator_withdraw_surplus_verify_is_writable_privileges<'me, 'info>(
    accounts: CreatorWithdrawSurplusAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for acct in [
        accounts.virtual_pool,
        accounts.token_quote_account,
        accounts.quote_vault,
    ] {
        if !acct.is_writable {
            return Err((acct, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}

pub fn creator_withdraw_surplus_verify_is_signer_privileges<'me, 'info>(
    accounts: CreatorWithdrawSurplusAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    if !accounts.creator.is_signer {
        return Err((accounts.creator, ProgramError::MissingRequiredSignature));
    }
    Ok(())
}

pub fn creator_withdraw_surplus_verify_account_privileges<'me, 'info>(
    accounts: CreatorWithdrawSurplusAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    creator_withdraw_surplus_verify_is_writable_privileges(accounts)?;
    creator_withdraw_surplus_verify_is_signer_privileges(accounts)?;
    Ok(())
}
pub const INITIALIZE_VIRTUAL_POOL_WITH_SPL_TOKEN_IX_ACCOUNTS_LEN: usize = 16;

#[derive(Copy, Clone, Debug)]
pub struct InitializeVirtualPoolWithSplTokenAccounts<'me, 'info> {
    pub config: &'me AccountInfo<'info>,
    pub pool_authority: &'me AccountInfo<'info>,
    pub creator: &'me AccountInfo<'info>,
    pub base_mint: &'me AccountInfo<'info>,
    pub quote_mint: &'me AccountInfo<'info>,
    pub pool: &'me AccountInfo<'info>,
    pub base_vault: &'me AccountInfo<'info>,
    pub quote_vault: &'me AccountInfo<'info>,
    pub mint_metadata: &'me AccountInfo<'info>,
    pub metadata_program: &'me AccountInfo<'info>,
    pub payer: &'me AccountInfo<'info>,
    pub token_quote_program: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct InitializeVirtualPoolWithSplTokenKeys {
    pub config: Pubkey,
    pub pool_authority: Pubkey,
    pub creator: Pubkey,
    pub base_mint: Pubkey,
    pub quote_mint: Pubkey,
    pub pool: Pubkey,
    pub base_vault: Pubkey,
    pub quote_vault: Pubkey,
    pub mint_metadata: Pubkey,
    pub metadata_program: Pubkey,
    pub payer: Pubkey,
    pub token_quote_program: Pubkey,
    pub token_program: Pubkey,
    pub system_program: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}

impl From<InitializeVirtualPoolWithSplTokenAccounts<'_, '_>> for InitializeVirtualPoolWithSplTokenKeys {
    fn from(accounts: InitializeVirtualPoolWithSplTokenAccounts) -> Self {
        Self {
            config: *accounts.config.key,
            pool_authority: *accounts.pool_authority.key,
            creator: *accounts.creator.key,
            base_mint: *accounts.base_mint.key,
            quote_mint: *accounts.quote_mint.key,
            pool: *accounts.pool.key,
            base_vault: *accounts.base_vault.key,
            quote_vault: *accounts.quote_vault.key,
            mint_metadata: *accounts.mint_metadata.key,
            metadata_program: *accounts.metadata_program.key,
            payer: *accounts.payer.key,
            token_quote_program: *accounts.token_quote_program.key,
            token_program: *accounts.token_program.key,
            system_program: *accounts.system_program.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}

impl From<InitializeVirtualPoolWithSplTokenKeys>
    for [AccountMeta; INITIALIZE_VIRTUAL_POOL_WITH_SPL_TOKEN_IX_ACCOUNTS_LEN]
{
    fn from(keys: InitializeVirtualPoolWithSplTokenKeys) -> Self {
        [
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
                pubkey: keys.creator,
                is_signer: true,
                is_writable: false,
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
                pubkey: keys.pool,
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
                pubkey: keys.mint_metadata,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.metadata_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.payer,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_quote_program,
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

impl From<[Pubkey; INITIALIZE_VIRTUAL_POOL_WITH_SPL_TOKEN_IX_ACCOUNTS_LEN]> for InitializeVirtualPoolWithSplTokenKeys {
    fn from(pubkeys: [Pubkey; INITIALIZE_VIRTUAL_POOL_WITH_SPL_TOKEN_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            config: pubkeys[0],
            pool_authority: pubkeys[1],
            creator: pubkeys[2],
            base_mint: pubkeys[3],
            quote_mint: pubkeys[4],
            pool: pubkeys[5],
            base_vault: pubkeys[6],
            quote_vault: pubkeys[7],
            mint_metadata: pubkeys[8],
            metadata_program: pubkeys[9],
            payer: pubkeys[10],
            token_quote_program: pubkeys[11],
            token_program: pubkeys[12],
            system_program: pubkeys[13],
            event_authority: pubkeys[14],
            program: pubkeys[15],
        }
    }
}

impl<'info> From<InitializeVirtualPoolWithSplTokenAccounts<'_, 'info>>
    for [AccountInfo<'info>; INITIALIZE_VIRTUAL_POOL_WITH_SPL_TOKEN_IX_ACCOUNTS_LEN]
{
    fn from(accounts: InitializeVirtualPoolWithSplTokenAccounts<'_, 'info>) -> Self {
        [
            accounts.config.clone(),
            accounts.pool_authority.clone(),
            accounts.creator.clone(),
            accounts.base_mint.clone(),
            accounts.quote_mint.clone(),
            accounts.pool.clone(),
            accounts.base_vault.clone(),
            accounts.quote_vault.clone(),
            accounts.mint_metadata.clone(),
            accounts.metadata_program.clone(),
            accounts.payer.clone(),
            accounts.token_quote_program.clone(),
            accounts.token_program.clone(),
            accounts.system_program.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}

impl<'me, 'info>
    From<&'me [AccountInfo<'info>; INITIALIZE_VIRTUAL_POOL_WITH_SPL_TOKEN_IX_ACCOUNTS_LEN]>
    for InitializeVirtualPoolWithSplTokenAccounts<'me, 'info>
{
    fn from(
        arr: &'me [AccountInfo<'info>; INITIALIZE_VIRTUAL_POOL_WITH_SPL_TOKEN_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            config: &arr[0],
            pool_authority: &arr[1],
            creator: &arr[2],
            base_mint: &arr[3],
            quote_mint: &arr[4],
            pool: &arr[5],
            base_vault: &arr[6],
            quote_vault: &arr[7],
            mint_metadata: &arr[8],
            metadata_program: &arr[9],
            payer: &arr[10],
            token_quote_program: &arr[11],
            token_program: &arr[12],
            system_program: &arr[13],
            event_authority: &arr[14],
            program: &arr[15],
        }
    }
}

pub const INITIALIZE_VIRTUAL_POOL_WITH_SPL_TOKEN_IX_DISCM: [u8; 8] =
    [140, 85, 215, 176, 102, 54, 104, 79];

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InitializePoolParameters {
    // Define the parameters based on your actual requirements
    // Example fields:
    pub initial_price: u64,
    pub fee_rate: u64,
    pub creator_fee_rate: u64,
    // Add other parameters as needed
}

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InitializeVirtualPoolWithSplTokenIxArgs {
    pub params: InitializePoolParameters,
}

#[derive(Clone, Debug, PartialEq)]
pub struct InitializeVirtualPoolWithSplTokenIxData(pub InitializeVirtualPoolWithSplTokenIxArgs);

impl From<InitializeVirtualPoolWithSplTokenIxArgs> for InitializeVirtualPoolWithSplTokenIxData {
    fn from(args: InitializeVirtualPoolWithSplTokenIxArgs) -> Self {
        Self(args)
    }
}

impl InitializeVirtualPoolWithSplTokenIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != INITIALIZE_VIRTUAL_POOL_WITH_SPL_TOKEN_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    INITIALIZE_VIRTUAL_POOL_WITH_SPL_TOKEN_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(InitializeVirtualPoolWithSplTokenIxArgs::deserialize(
            &mut reader,
        )?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&INITIALIZE_VIRTUAL_POOL_WITH_SPL_TOKEN_IX_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}

pub fn initialize_virtual_pool_with_spl_token_ix_with_program_id(
    program_id: Pubkey,
    keys: InitializeVirtualPoolWithSplTokenKeys,
    args: InitializeVirtualPoolWithSplTokenIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; INITIALIZE_VIRTUAL_POOL_WITH_SPL_TOKEN_IX_ACCOUNTS_LEN] = keys.into();
    let data: InitializeVirtualPoolWithSplTokenIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}

pub fn initialize_virtual_pool_with_spl_token_ix(
    keys: InitializeVirtualPoolWithSplTokenKeys,
    args: InitializeVirtualPoolWithSplTokenIxArgs,
) -> std::io::Result<Instruction> {
    initialize_virtual_pool_with_spl_token_ix_with_program_id(crate::ID, keys, args)
}

pub fn initialize_virtual_pool_with_spl_token_invoke_with_program_id(
    program_id: Pubkey,
    accounts: InitializeVirtualPoolWithSplTokenAccounts<'_, '_>,
    args: InitializeVirtualPoolWithSplTokenIxArgs,
) -> ProgramResult {
    let keys: InitializeVirtualPoolWithSplTokenKeys = accounts.into();
    let ix = initialize_virtual_pool_with_spl_token_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}

pub fn initialize_virtual_pool_with_spl_token_invoke(
    accounts: InitializeVirtualPoolWithSplTokenAccounts<'_, '_>,
    args: InitializeVirtualPoolWithSplTokenIxArgs,
) -> ProgramResult {
    initialize_virtual_pool_with_spl_token_invoke_with_program_id(crate::ID, accounts, args)
}

pub fn initialize_virtual_pool_with_spl_token_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: InitializeVirtualPoolWithSplTokenAccounts<'_, '_>,
    args: InitializeVirtualPoolWithSplTokenIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: InitializeVirtualPoolWithSplTokenKeys = accounts.into();
    let ix = initialize_virtual_pool_with_spl_token_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}

pub fn initialize_virtual_pool_with_spl_token_invoke_signed(
    accounts: InitializeVirtualPoolWithSplTokenAccounts<'_, '_>,
    args: InitializeVirtualPoolWithSplTokenIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    initialize_virtual_pool_with_spl_token_invoke_signed_with_program_id(crate::ID, accounts, args, seeds)
}

pub fn initialize_virtual_pool_with_spl_token_verify_account_keys(
    accounts: InitializeVirtualPoolWithSplTokenAccounts<'_, '_>,
    keys: InitializeVirtualPoolWithSplTokenKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.config.key, keys.config),
        (*accounts.pool_authority.key, keys.pool_authority),
        (*accounts.creator.key, keys.creator),
        (*accounts.base_mint.key, keys.base_mint),
        (*accounts.quote_mint.key, keys.quote_mint),
        (*accounts.pool.key, keys.pool),
        (*accounts.base_vault.key, keys.base_vault),
        (*accounts.quote_vault.key, keys.quote_vault),
        (*accounts.mint_metadata.key, keys.mint_metadata),
        (*accounts.metadata_program.key, keys.metadata_program),
        (*accounts.payer.key, keys.payer),
        (*accounts.token_quote_program.key, keys.token_quote_program),
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

pub fn initialize_virtual_pool_with_spl_token_verify_is_writable_privileges<'me, 'info>(
    accounts: InitializeVirtualPoolWithSplTokenAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for acct in [
        accounts.base_mint,
        accounts.pool,
        accounts.base_vault,
        accounts.quote_vault,
        accounts.mint_metadata,
        accounts.payer,
    ] {
        if !acct.is_writable {
            return Err((acct, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}

pub fn initialize_virtual_pool_with_spl_token_verify_is_signer_privileges<'me, 'info>(
    accounts: InitializeVirtualPoolWithSplTokenAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for acct in [accounts.creator, accounts.base_mint, accounts.payer] {
        if !acct.is_signer {
            return Err((acct, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}

pub fn initialize_virtual_pool_with_spl_token_verify_account_privileges<'me, 'info>(
    accounts: InitializeVirtualPoolWithSplTokenAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    initialize_virtual_pool_with_spl_token_verify_is_writable_privileges(accounts)?;
    initialize_virtual_pool_with_spl_token_verify_is_signer_privileges(accounts)?;
    Ok(())
}
pub const INITIALIZE_VIRTUAL_POOL_WITH_TOKEN2022_IX_ACCOUNTS_LEN: usize = 14;

#[derive(Copy, Clone, Debug)]
pub struct InitializeVirtualPoolWithToken2022Accounts<'me, 'info> {
    pub config: &'me AccountInfo<'info>,
    pub pool_authority: &'me AccountInfo<'info>,
    pub creator: &'me AccountInfo<'info>,
    pub base_mint: &'me AccountInfo<'info>,
    pub quote_mint: &'me AccountInfo<'info>,
    pub pool: &'me AccountInfo<'info>,
    pub base_vault: &'me AccountInfo<'info>,
    pub quote_vault: &'me AccountInfo<'info>,
    pub payer: &'me AccountInfo<'info>,
    pub token_quote_program: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct InitializeVirtualPoolWithToken2022Keys {
    pub config: Pubkey,
    pub pool_authority: Pubkey,
    pub creator: Pubkey,
    pub base_mint: Pubkey,
    pub quote_mint: Pubkey,
    pub pool: Pubkey,
    pub base_vault: Pubkey,
    pub quote_vault: Pubkey,
    pub payer: Pubkey,
    pub token_quote_program: Pubkey,
    pub token_program: Pubkey,
    pub system_program: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}

impl From<InitializeVirtualPoolWithToken2022Accounts<'_, '_>> for InitializeVirtualPoolWithToken2022Keys {
    fn from(accounts: InitializeVirtualPoolWithToken2022Accounts) -> Self {
        Self {
            config: *accounts.config.key,
            pool_authority: *accounts.pool_authority.key,
            creator: *accounts.creator.key,
            base_mint: *accounts.base_mint.key,
            quote_mint: *accounts.quote_mint.key,
            pool: *accounts.pool.key,
            base_vault: *accounts.base_vault.key,
            quote_vault: *accounts.quote_vault.key,
            payer: *accounts.payer.key,
            token_quote_program: *accounts.token_quote_program.key,
            token_program: *accounts.token_program.key,
            system_program: *accounts.system_program.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}

impl From<InitializeVirtualPoolWithToken2022Keys>
    for [AccountMeta; INITIALIZE_VIRTUAL_POOL_WITH_TOKEN2022_IX_ACCOUNTS_LEN]
{
    fn from(keys: InitializeVirtualPoolWithToken2022Keys) -> Self {
        [
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
                pubkey: keys.creator,
                is_signer: true,
                is_writable: false,
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
                pubkey: keys.pool,
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
                pubkey: keys.payer,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_quote_program,
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

impl From<[Pubkey; INITIALIZE_VIRTUAL_POOL_WITH_TOKEN2022_IX_ACCOUNTS_LEN]> for InitializeVirtualPoolWithToken2022Keys {
    fn from(pubkeys: [Pubkey; INITIALIZE_VIRTUAL_POOL_WITH_TOKEN2022_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            config: pubkeys[0],
            pool_authority: pubkeys[1],
            creator: pubkeys[2],
            base_mint: pubkeys[3],
            quote_mint: pubkeys[4],
            pool: pubkeys[5],
            base_vault: pubkeys[6],
            quote_vault: pubkeys[7],
            payer: pubkeys[8],
            token_quote_program: pubkeys[9],
            token_program: pubkeys[10],
            system_program: pubkeys[11],
            event_authority: pubkeys[12],
            program: pubkeys[13],
        }
    }
}

impl<'info> From<InitializeVirtualPoolWithToken2022Accounts<'_, 'info>>
    for [AccountInfo<'info>; INITIALIZE_VIRTUAL_POOL_WITH_TOKEN2022_IX_ACCOUNTS_LEN]
{
    fn from(accounts: InitializeVirtualPoolWithToken2022Accounts<'_, 'info>) -> Self {
        [
            accounts.config.clone(),
            accounts.pool_authority.clone(),
            accounts.creator.clone(),
            accounts.base_mint.clone(),
            accounts.quote_mint.clone(),
            accounts.pool.clone(),
            accounts.base_vault.clone(),
            accounts.quote_vault.clone(),
            accounts.payer.clone(),
            accounts.token_quote_program.clone(),
            accounts.token_program.clone(),
            accounts.system_program.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}

impl<'me, 'info>
    From<&'me [AccountInfo<'info>; INITIALIZE_VIRTUAL_POOL_WITH_TOKEN2022_IX_ACCOUNTS_LEN]>
    for InitializeVirtualPoolWithToken2022Accounts<'me, 'info>
{
    fn from(
        arr: &'me [AccountInfo<'info>; INITIALIZE_VIRTUAL_POOL_WITH_TOKEN2022_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            config: &arr[0],
            pool_authority: &arr[1],
            creator: &arr[2],
            base_mint: &arr[3],
            quote_mint: &arr[4],
            pool: &arr[5],
            base_vault: &arr[6],
            quote_vault: &arr[7],
            payer: &arr[8],
            token_quote_program: &arr[9],
            token_program: &arr[10],
            system_program: &arr[11],
            event_authority: &arr[12],
            program: &arr[13],
        }
    }
}

pub const INITIALIZE_VIRTUAL_POOL_WITH_TOKEN_2022_IX_DISCM: [u8; 8] =
    [169, 118, 51, 78, 145, 110, 220, 155];

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InitializeVirtualPoolWithToken2022IxArgs {
    pub params: InitializePoolParameters,
}

#[derive(Clone, Debug, PartialEq)]
pub struct InitializeVirtualPoolWithToken2022IxData(pub InitializeVirtualPoolWithToken2022IxArgs);

impl From<InitializeVirtualPoolWithToken2022IxArgs> for InitializeVirtualPoolWithToken2022IxData {
    fn from(args: InitializeVirtualPoolWithToken2022IxArgs) -> Self {
        Self(args)
    }
}

impl InitializeVirtualPoolWithToken2022IxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != INITIALIZE_VIRTUAL_POOL_WITH_TOKEN_2022_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    INITIALIZE_VIRTUAL_POOL_WITH_TOKEN_2022_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(InitializeVirtualPoolWithToken2022IxArgs::deserialize(
            &mut reader,
        )?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&INITIALIZE_VIRTUAL_POOL_WITH_TOKEN_2022_IX_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}

pub fn initialize_virtual_pool_with_token2022_ix_with_program_id(
    program_id: Pubkey,
    keys: InitializeVirtualPoolWithToken2022Keys,
    args: InitializeVirtualPoolWithToken2022IxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; INITIALIZE_VIRTUAL_POOL_WITH_TOKEN2022_IX_ACCOUNTS_LEN] = keys.into();
    let data: InitializeVirtualPoolWithToken2022IxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}

pub fn initialize_virtual_pool_with_token2022_ix(
    keys: InitializeVirtualPoolWithToken2022Keys,
    args: InitializeVirtualPoolWithToken2022IxArgs,
) -> std::io::Result<Instruction> {
    initialize_virtual_pool_with_token2022_ix_with_program_id(crate::ID, keys, args)
}

pub fn initialize_virtual_pool_with_token2022_invoke_with_program_id(
    program_id: Pubkey,
    accounts: InitializeVirtualPoolWithToken2022Accounts<'_, '_>,
    args: InitializeVirtualPoolWithToken2022IxArgs,
) -> ProgramResult {
    let keys: InitializeVirtualPoolWithToken2022Keys = accounts.into();
    let ix = initialize_virtual_pool_with_token2022_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}

pub fn initialize_virtual_pool_with_token2022_invoke(
    accounts: InitializeVirtualPoolWithToken2022Accounts<'_, '_>,
    args: InitializeVirtualPoolWithToken2022IxArgs,
) -> ProgramResult {
    initialize_virtual_pool_with_token2022_invoke_with_program_id(crate::ID, accounts, args)
}

pub fn initialize_virtual_pool_with_token2022_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: InitializeVirtualPoolWithToken2022Accounts<'_, '_>,
    args: InitializeVirtualPoolWithToken2022IxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: InitializeVirtualPoolWithToken2022Keys = accounts.into();
    let ix = initialize_virtual_pool_with_token2022_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}

pub fn initialize_virtual_pool_with_token2022_invoke_signed(
    accounts: InitializeVirtualPoolWithToken2022Accounts<'_, '_>,
    args: InitializeVirtualPoolWithToken2022IxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    initialize_virtual_pool_with_token2022_invoke_signed_with_program_id(crate::ID, accounts, args, seeds)
}

pub fn initialize_virtual_pool_with_token2022_verify_account_keys(
    accounts: InitializeVirtualPoolWithToken2022Accounts<'_, '_>,
    keys: InitializeVirtualPoolWithToken2022Keys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.config.key, keys.config),
        (*accounts.pool_authority.key, keys.pool_authority),
        (*accounts.creator.key, keys.creator),
        (*accounts.base_mint.key, keys.base_mint),
        (*accounts.quote_mint.key, keys.quote_mint),
        (*accounts.pool.key, keys.pool),
        (*accounts.base_vault.key, keys.base_vault),
        (*accounts.quote_vault.key, keys.quote_vault),
        (*accounts.payer.key, keys.payer),
        (*accounts.token_quote_program.key, keys.token_quote_program),
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

pub fn initialize_virtual_pool_with_token2022_verify_is_writable_privileges<'me, 'info>(
    accounts: InitializeVirtualPoolWithToken2022Accounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for acct in [
        accounts.base_mint,
        accounts.pool,
        accounts.base_vault,
        accounts.quote_vault,
        accounts.payer,
    ] {
        if !acct.is_writable {
            return Err((acct, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}

pub fn initialize_virtual_pool_with_token2022_verify_is_signer_privileges<'me, 'info>(
    accounts: InitializeVirtualPoolWithToken2022Accounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for acct in [accounts.creator, accounts.base_mint, accounts.payer] {
        if !acct.is_signer {
            return Err((acct, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}

pub fn initialize_virtual_pool_with_token2022_verify_account_privileges<'me, 'info>(
    accounts: InitializeVirtualPoolWithToken2022Accounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    initialize_virtual_pool_with_token2022_verify_is_writable_privileges(accounts)?;
    initialize_virtual_pool_with_token2022_verify_is_signer_privileges(accounts)?;
    Ok(())
}
pub const MIGRATE_METEORA_DAMM_IX_ACCOUNTS_LEN: usize = 31;

#[derive(Copy, Clone, Debug)]
pub struct MigrateMeteoraDammAccounts<'me, 'info> {
    pub virtual_pool: &'me AccountInfo<'info>,
    pub migration_metadata: &'me AccountInfo<'info>,
    pub config: &'me AccountInfo<'info>,
    pub pool_authority: &'me AccountInfo<'info>,
    pub pool: &'me AccountInfo<'info>,
    pub damm_config: &'me AccountInfo<'info>,
    pub lp_mint: &'me AccountInfo<'info>,
    pub token_a_mint: &'me AccountInfo<'info>,
    pub token_b_mint: &'me AccountInfo<'info>,
    pub a_vault: &'me AccountInfo<'info>,
    pub b_vault: &'me AccountInfo<'info>,
    pub a_token_vault: &'me AccountInfo<'info>,
    pub b_token_vault: &'me AccountInfo<'info>,
    pub a_vault_lp_mint: &'me AccountInfo<'info>,
    pub b_vault_lp_mint: &'me AccountInfo<'info>,
    pub a_vault_lp: &'me AccountInfo<'info>,
    pub b_vault_lp: &'me AccountInfo<'info>,
    pub base_vault: &'me AccountInfo<'info>,
    pub quote_vault: &'me AccountInfo<'info>,
    pub virtual_pool_lp: &'me AccountInfo<'info>,
    pub protocol_token_a_fee: &'me AccountInfo<'info>,
    pub protocol_token_b_fee: &'me AccountInfo<'info>,
    pub payer: &'me AccountInfo<'info>,
    pub rent: &'me AccountInfo<'info>,
    pub mint_metadata: &'me AccountInfo<'info>,
    pub metadata_program: &'me AccountInfo<'info>,
    pub amm_program: &'me AccountInfo<'info>,
    pub vault_program: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub associated_token_program: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MigrateMeteoraDammKeys {
    pub virtual_pool: Pubkey,
    pub migration_metadata: Pubkey,
    pub config: Pubkey,
    pub pool_authority: Pubkey,
    pub pool: Pubkey,
    pub damm_config: Pubkey,
    pub lp_mint: Pubkey,
    pub token_a_mint: Pubkey,
    pub token_b_mint: Pubkey,
    pub a_vault: Pubkey,
    pub b_vault: Pubkey,
    pub a_token_vault: Pubkey,
    pub b_token_vault: Pubkey,
    pub a_vault_lp_mint: Pubkey,
    pub b_vault_lp_mint: Pubkey,
    pub a_vault_lp: Pubkey,
    pub b_vault_lp: Pubkey,
    pub base_vault: Pubkey,
    pub quote_vault: Pubkey,
    pub virtual_pool_lp: Pubkey,
    pub protocol_token_a_fee: Pubkey,
    pub protocol_token_b_fee: Pubkey,
    pub payer: Pubkey,
    pub rent: Pubkey,
    pub mint_metadata: Pubkey,
    pub metadata_program: Pubkey,
    pub amm_program: Pubkey,
    pub vault_program: Pubkey,
    pub token_program: Pubkey,
    pub associated_token_program: Pubkey,
    pub system_program: Pubkey,
}

impl From<MigrateMeteoraDammAccounts<'_, '_>> for MigrateMeteoraDammKeys {
    fn from(accounts: MigrateMeteoraDammAccounts) -> Self {
        Self {
            virtual_pool: *accounts.virtual_pool.key,
            migration_metadata: *accounts.migration_metadata.key,
            config: *accounts.config.key,
            pool_authority: *accounts.pool_authority.key,
            pool: *accounts.pool.key,
            damm_config: *accounts.damm_config.key,
            lp_mint: *accounts.lp_mint.key,
            token_a_mint: *accounts.token_a_mint.key,
            token_b_mint: *accounts.token_b_mint.key,
            a_vault: *accounts.a_vault.key,
            b_vault: *accounts.b_vault.key,
            a_token_vault: *accounts.a_token_vault.key,
            b_token_vault: *accounts.b_token_vault.key,
            a_vault_lp_mint: *accounts.a_vault_lp_mint.key,
            b_vault_lp_mint: *accounts.b_vault_lp_mint.key,
            a_vault_lp: *accounts.a_vault_lp.key,
            b_vault_lp: *accounts.b_vault_lp.key,
            base_vault: *accounts.base_vault.key,
            quote_vault: *accounts.quote_vault.key,
            virtual_pool_lp: *accounts.virtual_pool_lp.key,
            protocol_token_a_fee: *accounts.protocol_token_a_fee.key,
            protocol_token_b_fee: *accounts.protocol_token_b_fee.key,
            payer: *accounts.payer.key,
            rent: *accounts.rent.key,
            mint_metadata: *accounts.mint_metadata.key,
            metadata_program: *accounts.metadata_program.key,
            amm_program: *accounts.amm_program.key,
            vault_program: *accounts.vault_program.key,
            token_program: *accounts.token_program.key,
            associated_token_program: *accounts.associated_token_program.key,
            system_program: *accounts.system_program.key,
        }
    }
}

impl From<MigrateMeteoraDammKeys>
    for [AccountMeta; MIGRATE_METEORA_DAMM_IX_ACCOUNTS_LEN]
{
    fn from(keys: MigrateMeteoraDammKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.virtual_pool,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.migration_metadata,
                is_signer: false,
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
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.pool,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.damm_config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.lp_mint,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_a_mint,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_b_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.a_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.b_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.a_token_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.b_token_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.a_vault_lp_mint,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.b_vault_lp_mint,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.a_vault_lp,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.b_vault_lp,
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
                pubkey: keys.virtual_pool_lp,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.protocol_token_a_fee,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.protocol_token_b_fee,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.payer,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.rent,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.mint_metadata,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.metadata_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.amm_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.vault_program,
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
                pubkey: keys.system_program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}

impl From<[Pubkey; MIGRATE_METEORA_DAMM_IX_ACCOUNTS_LEN]> for MigrateMeteoraDammKeys {
    fn from(pubkeys: [Pubkey; MIGRATE_METEORA_DAMM_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            virtual_pool: pubkeys[0],
            migration_metadata: pubkeys[1],
            config: pubkeys[2],
            pool_authority: pubkeys[3],
            pool: pubkeys[4],
            damm_config: pubkeys[5],
            lp_mint: pubkeys[6],
            token_a_mint: pubkeys[7],
            token_b_mint: pubkeys[8],
            a_vault: pubkeys[9],
            b_vault: pubkeys[10],
            a_token_vault: pubkeys[11],
            b_token_vault: pubkeys[12],
            a_vault_lp_mint: pubkeys[13],
            b_vault_lp_mint: pubkeys[14],
            a_vault_lp: pubkeys[15],
            b_vault_lp: pubkeys[16],
            base_vault: pubkeys[17],
            quote_vault: pubkeys[18],
            virtual_pool_lp: pubkeys[19],
            protocol_token_a_fee: pubkeys[20],
            protocol_token_b_fee: pubkeys[21],
            payer: pubkeys[22],
            rent: pubkeys[23],
            mint_metadata: pubkeys[24],
            metadata_program: pubkeys[25],
            amm_program: pubkeys[26],
            vault_program: pubkeys[27],
            token_program: pubkeys[28],
            associated_token_program: pubkeys[29],
            system_program: pubkeys[30],
        }
    }
}

impl<'info> From<MigrateMeteoraDammAccounts<'_, 'info>>
    for [AccountInfo<'info>; MIGRATE_METEORA_DAMM_IX_ACCOUNTS_LEN]
{
    fn from(accounts: MigrateMeteoraDammAccounts<'_, 'info>) -> Self {
        [
            accounts.virtual_pool.clone(),
            accounts.migration_metadata.clone(),
            accounts.config.clone(),
            accounts.pool_authority.clone(),
            accounts.pool.clone(),
            accounts.damm_config.clone(),
            accounts.lp_mint.clone(),
            accounts.token_a_mint.clone(),
            accounts.token_b_mint.clone(),
            accounts.a_vault.clone(),
            accounts.b_vault.clone(),
            accounts.a_token_vault.clone(),
            accounts.b_token_vault.clone(),
            accounts.a_vault_lp_mint.clone(),
            accounts.b_vault_lp_mint.clone(),
            accounts.a_vault_lp.clone(),
            accounts.b_vault_lp.clone(),
            accounts.base_vault.clone(),
            accounts.quote_vault.clone(),
            accounts.virtual_pool_lp.clone(),
            accounts.protocol_token_a_fee.clone(),
            accounts.protocol_token_b_fee.clone(),
            accounts.payer.clone(),
            accounts.rent.clone(),
            accounts.mint_metadata.clone(),
            accounts.metadata_program.clone(),
            accounts.amm_program.clone(),
            accounts.vault_program.clone(),
            accounts.token_program.clone(),
            accounts.associated_token_program.clone(),
            accounts.system_program.clone(),
        ]
    }
}

impl<'me, 'info>
    From<&'me [AccountInfo<'info>; MIGRATE_METEORA_DAMM_IX_ACCOUNTS_LEN]>
    for MigrateMeteoraDammAccounts<'me, 'info>
{
    fn from(
        arr: &'me [AccountInfo<'info>; MIGRATE_METEORA_DAMM_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            virtual_pool: &arr[0],
            migration_metadata: &arr[1],
            config: &arr[2],
            pool_authority: &arr[3],
            pool: &arr[4],
            damm_config: &arr[5],
            lp_mint: &arr[6],
            token_a_mint: &arr[7],
            token_b_mint: &arr[8],
            a_vault: &arr[9],
            b_vault: &arr[10],
            a_token_vault: &arr[11],
            b_token_vault: &arr[12],
            a_vault_lp_mint: &arr[13],
            b_vault_lp_mint: &arr[14],
            a_vault_lp: &arr[15],
            b_vault_lp: &arr[16],
            base_vault: &arr[17],
            quote_vault: &arr[18],
            virtual_pool_lp: &arr[19],
            protocol_token_a_fee: &arr[20],
            protocol_token_b_fee: &arr[21],
            payer: &arr[22],
            rent: &arr[23],
            mint_metadata: &arr[24],
            metadata_program: &arr[25],
            amm_program: &arr[26],
            vault_program: &arr[27],
            token_program: &arr[28],
            associated_token_program: &arr[29],
            system_program: &arr[30],
        }
    }
}

pub const MIGRATE_METEORA_DAMM_IX_DISCM: [u8; 8] =
    [27, 1, 48, 22, 180, 63, 118, 217];

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct MigrateMeteoraDammIxArgs {
    // This instruction has no arguments according to the provided structure
}

#[derive(Clone, Debug, PartialEq)]
pub struct MigrateMeteoraDammIxData(pub MigrateMeteoraDammIxArgs);

impl From<MigrateMeteoraDammIxArgs> for MigrateMeteoraDammIxData {
    fn from(args: MigrateMeteoraDammIxArgs) -> Self {
        Self(args)
    }
}

impl MigrateMeteoraDammIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != MIGRATE_METEORA_DAMM_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    MIGRATE_METEORA_DAMM_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(MigrateMeteoraDammIxArgs::deserialize(
            &mut reader,
        )?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&MIGRATE_METEORA_DAMM_IX_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}

pub fn migrate_meteora_damm_ix_with_program_id(
    program_id: Pubkey,
    keys: MigrateMeteoraDammKeys,
    args: MigrateMeteoraDammIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; MIGRATE_METEORA_DAMM_IX_ACCOUNTS_LEN] = keys.into();
    let data: MigrateMeteoraDammIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}

pub fn migrate_meteora_damm_ix(
    keys: MigrateMeteoraDammKeys,
    args: MigrateMeteoraDammIxArgs,
) -> std::io::Result<Instruction> {
    migrate_meteora_damm_ix_with_program_id(crate::ID, keys, args)
}

pub fn migrate_meteora_damm_invoke_with_program_id(
    program_id: Pubkey,
    accounts: MigrateMeteoraDammAccounts<'_, '_>,
    args: MigrateMeteoraDammIxArgs,
) -> ProgramResult {
    let keys: MigrateMeteoraDammKeys = accounts.into();
    let ix = migrate_meteora_damm_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}

pub fn migrate_meteora_damm_invoke(
    accounts: MigrateMeteoraDammAccounts<'_, '_>,
    args: MigrateMeteoraDammIxArgs,
) -> ProgramResult {
    migrate_meteora_damm_invoke_with_program_id(crate::ID, accounts, args)
}

pub fn migrate_meteora_damm_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: MigrateMeteoraDammAccounts<'_, '_>,
    args: MigrateMeteoraDammIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: MigrateMeteoraDammKeys = accounts.into();
    let ix = migrate_meteora_damm_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}

pub fn migrate_meteora_damm_invoke_signed(
    accounts: MigrateMeteoraDammAccounts<'_, '_>,
    args: MigrateMeteoraDammIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    migrate_meteora_damm_invoke_signed_with_program_id(crate::ID, accounts, args, seeds)
}

pub fn migrate_meteora_damm_verify_account_keys(
    accounts: MigrateMeteoraDammAccounts<'_, '_>,
    keys: MigrateMeteoraDammKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.virtual_pool.key, keys.virtual_pool),
        (*accounts.migration_metadata.key, keys.migration_metadata),
        (*accounts.config.key, keys.config),
        (*accounts.pool_authority.key, keys.pool_authority),
        (*accounts.pool.key, keys.pool),
        (*accounts.damm_config.key, keys.damm_config),
        (*accounts.lp_mint.key, keys.lp_mint),
        (*accounts.token_a_mint.key, keys.token_a_mint),
        (*accounts.token_b_mint.key, keys.token_b_mint),
        (*accounts.a_vault.key, keys.a_vault),
        (*accounts.b_vault.key, keys.b_vault),
        (*accounts.a_token_vault.key, keys.a_token_vault),
        (*accounts.b_token_vault.key, keys.b_token_vault),
        (*accounts.a_vault_lp_mint.key, keys.a_vault_lp_mint),
        (*accounts.b_vault_lp_mint.key, keys.b_vault_lp_mint),
        (*accounts.a_vault_lp.key, keys.a_vault_lp),
        (*accounts.b_vault_lp.key, keys.b_vault_lp),
        (*accounts.base_vault.key, keys.base_vault),
        (*accounts.quote_vault.key, keys.quote_vault),
        (*accounts.virtual_pool_lp.key, keys.virtual_pool_lp),
        (*accounts.protocol_token_a_fee.key, keys.protocol_token_a_fee),
        (*accounts.protocol_token_b_fee.key, keys.protocol_token_b_fee),
        (*accounts.payer.key, keys.payer),
        (*accounts.rent.key, keys.rent),
        (*accounts.mint_metadata.key, keys.mint_metadata),
        (*accounts.metadata_program.key, keys.metadata_program),
        (*accounts.amm_program.key, keys.amm_program),
        (*accounts.vault_program.key, keys.vault_program),
        (*accounts.token_program.key, keys.token_program),
        (*accounts.associated_token_program.key, keys.associated_token_program),
        (*accounts.system_program.key, keys.system_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}

pub fn migrate_meteora_damm_verify_is_writable_privileges<'me, 'info>(
    accounts: MigrateMeteoraDammAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for acct in [
        accounts.virtual_pool,
        accounts.migration_metadata,
        accounts.pool_authority,
        accounts.pool,
        accounts.lp_mint,
        accounts.token_a_mint,
        accounts.a_vault,
        accounts.b_vault,
        accounts.a_token_vault,
        accounts.b_token_vault,
        accounts.a_vault_lp_mint,
        accounts.b_vault_lp_mint,
        accounts.a_vault_lp,
        accounts.b_vault_lp,
        accounts.base_vault,
        accounts.quote_vault,
        accounts.virtual_pool_lp,
        accounts.protocol_token_a_fee,
        accounts.protocol_token_b_fee,
        accounts.payer,
        accounts.mint_metadata,
    ] {
        if !acct.is_writable {
            return Err((acct, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}

pub fn migrate_meteora_damm_verify_is_signer_privileges<'me, 'info>(
    accounts: MigrateMeteoraDammAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    if !accounts.payer.is_signer {
        return Err((accounts.payer, ProgramError::MissingRequiredSignature));
    }
    Ok(())
}

pub fn migrate_meteora_damm_verify_account_privileges<'me, 'info>(
    accounts: MigrateMeteoraDammAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    migrate_meteora_damm_verify_is_writable_privileges(accounts)?;
    migrate_meteora_damm_verify_is_signer_privileges(accounts)?;
    Ok(())
}

pub const MIGRATE_METEORA_DAMM_CLAIM_LP_TOKEN_IX_ACCOUNTS_LEN: usize = 9;
#[derive(Copy, Clone, Debug)]
pub struct MigrateMeteoraDammClaimLpTokenAccounts<'me, 'info> {
    pub virtual_pool: &'me AccountInfo<'info>,
    pub migration_metadata: &'me AccountInfo<'info>,
    pub pool_authority: &'me AccountInfo<'info>,
    pub lp_mint: &'me AccountInfo<'info>,
    pub source_token: &'me AccountInfo<'info>,
    pub destination_token: &'me AccountInfo<'info>,
    pub owner: &'me AccountInfo<'info>,
    pub sender: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MigrateMeteoraDammClaimLpTokenKeys {
    pub virtual_pool: Pubkey,
    pub migration_metadata: Pubkey,
    pub pool_authority: Pubkey,
    pub lp_mint: Pubkey,
    pub source_token: Pubkey,
    pub destination_token: Pubkey,
    pub owner: Pubkey,
    pub sender: Pubkey,
    pub token_program: Pubkey,
}

impl From<MigrateMeteoraDammClaimLpTokenAccounts<'_, '_>>
    for MigrateMeteoraDammClaimLpTokenKeys
{
    fn from(accounts: MigrateMeteoraDammClaimLpTokenAccounts) -> Self {
        Self {
            virtual_pool: *accounts.virtual_pool.key,
            migration_metadata: *accounts.migration_metadata.key,
            pool_authority: *accounts.pool_authority.key,
            lp_mint: *accounts.lp_mint.key,
            source_token: *accounts.source_token.key,
            destination_token: *accounts.destination_token.key,
            owner: *accounts.owner.key,
            sender: *accounts.sender.key,
            token_program: *accounts.token_program.key,
        }
    }
}

impl From<MigrateMeteoraDammClaimLpTokenKeys>
    for [AccountMeta; MIGRATE_METEORA_DAMM_CLAIM_LP_TOKEN_IX_ACCOUNTS_LEN]
{
    fn from(keys: MigrateMeteoraDammClaimLpTokenKeys) -> Self {
        [
            AccountMeta { pubkey: keys.virtual_pool, is_signer: false, is_writable: false },
            AccountMeta { pubkey: keys.migration_metadata, is_signer: false, is_writable: true },
            AccountMeta { pubkey: keys.pool_authority, is_signer: false, is_writable: true },
            AccountMeta { pubkey: keys.lp_mint, is_signer: false, is_writable: false },
            AccountMeta { pubkey: keys.source_token, is_signer: false, is_writable: true },
            AccountMeta { pubkey: keys.destination_token, is_signer: false, is_writable: true },
            AccountMeta { pubkey: keys.owner, is_signer: false, is_writable: false },
            AccountMeta { pubkey: keys.sender, is_signer: true, is_writable: false },
            AccountMeta { pubkey: keys.token_program, is_signer: false, is_writable: false },
        ]
    }
}

impl From<[Pubkey; MIGRATE_METEORA_DAMM_CLAIM_LP_TOKEN_IX_ACCOUNTS_LEN]>
    for MigrateMeteoraDammClaimLpTokenKeys
{
    fn from(pubkeys: [Pubkey; MIGRATE_METEORA_DAMM_CLAIM_LP_TOKEN_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            virtual_pool: pubkeys[0],
            migration_metadata: pubkeys[1],
            pool_authority: pubkeys[2],
            lp_mint: pubkeys[3],
            source_token: pubkeys[4],
            destination_token: pubkeys[5],
            owner: pubkeys[6],
            sender: pubkeys[7],
            token_program: pubkeys[8],
        }
    }
}

impl<'info> From<MigrateMeteoraDammClaimLpTokenAccounts<'_, 'info>>
    for [AccountInfo<'info>; MIGRATE_METEORA_DAMM_CLAIM_LP_TOKEN_IX_ACCOUNTS_LEN]
{
    fn from(accounts: MigrateMeteoraDammClaimLpTokenAccounts<'_, 'info>) -> Self {
        [
            accounts.virtual_pool.clone(),
            accounts.migration_metadata.clone(),
            accounts.pool_authority.clone(),
            accounts.lp_mint.clone(),
            accounts.source_token.clone(),
            accounts.destination_token.clone(),
            accounts.owner.clone(),
            accounts.sender.clone(),
            accounts.token_program.clone(),
        ]
    }
}

impl<'me, 'info>
    From<&'me [AccountInfo<'info>; MIGRATE_METEORA_DAMM_CLAIM_LP_TOKEN_IX_ACCOUNTS_LEN]>
    for MigrateMeteoraDammClaimLpTokenAccounts<'me, 'info>
{
    fn from(arr: &'me [AccountInfo<'info>; MIGRATE_METEORA_DAMM_CLAIM_LP_TOKEN_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            virtual_pool: &arr[0],
            migration_metadata: &arr[1],
            pool_authority: &arr[2],
            lp_mint: &arr[3],
            source_token: &arr[4],
            destination_token: &arr[5],
            owner: &arr[6],
            sender: &arr[7],
            token_program: &arr[8],
        }
    }
}

pub const MIGRATE_METEORA_DAMM_CLAIM_LP_TOKEN_IX_DISCM: [u8; 8] =
    [139, 133, 2, 30, 91, 145, 127, 154];

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct MigrateMeteoraDammClaimLpTokenIxArgs {}

#[derive(Clone, Debug, PartialEq)]
pub struct MigrateMeteoraDammClaimLpTokenIxData(pub MigrateMeteoraDammClaimLpTokenIxArgs);

impl From<MigrateMeteoraDammClaimLpTokenIxArgs> for MigrateMeteoraDammClaimLpTokenIxData {
    fn from(args: MigrateMeteoraDammClaimLpTokenIxArgs) -> Self {
        Self(args)
    }
}

impl MigrateMeteoraDammClaimLpTokenIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != MIGRATE_METEORA_DAMM_CLAIM_LP_TOKEN_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    MIGRATE_METEORA_DAMM_CLAIM_LP_TOKEN_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(MigrateMeteoraDammClaimLpTokenIxArgs::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&MIGRATE_METEORA_DAMM_CLAIM_LP_TOKEN_IX_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}

pub fn migrate_meteora_damm_claim_lp_token_ix_with_program_id(
    program_id: Pubkey,
    keys: MigrateMeteoraDammClaimLpTokenKeys,
    args: MigrateMeteoraDammClaimLpTokenIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; MIGRATE_METEORA_DAMM_CLAIM_LP_TOKEN_IX_ACCOUNTS_LEN] = keys.into();
    let data: MigrateMeteoraDammClaimLpTokenIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}

pub fn migrate_meteora_damm_claim_lp_token_ix(
    keys: MigrateMeteoraDammClaimLpTokenKeys,
    args: MigrateMeteoraDammClaimLpTokenIxArgs,
) -> std::io::Result<Instruction> {
    migrate_meteora_damm_claim_lp_token_ix_with_program_id(crate::ID, keys, args)
}

pub fn migrate_meteora_damm_claim_lp_token_invoke_with_program_id(
    program_id: Pubkey,
    accounts: MigrateMeteoraDammClaimLpTokenAccounts<'_, '_>,
    args: MigrateMeteoraDammClaimLpTokenIxArgs,
) -> ProgramResult {
    let keys: MigrateMeteoraDammClaimLpTokenKeys = accounts.into();
    let ix = migrate_meteora_damm_claim_lp_token_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}

pub fn migrate_meteora_damm_claim_lp_token_invoke(
    accounts: MigrateMeteoraDammClaimLpTokenAccounts<'_, '_>,
    args: MigrateMeteoraDammClaimLpTokenIxArgs,
) -> ProgramResult {
    migrate_meteora_damm_claim_lp_token_invoke_with_program_id(crate::ID, accounts, args)
}

pub fn migrate_meteora_damm_claim_lp_token_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: MigrateMeteoraDammClaimLpTokenAccounts<'_, '_>,
    args: MigrateMeteoraDammClaimLpTokenIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: MigrateMeteoraDammClaimLpTokenKeys = accounts.into();
    let ix = migrate_meteora_damm_claim_lp_token_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}

pub fn migrate_meteora_damm_claim_lp_token_invoke_signed(
    accounts: MigrateMeteoraDammClaimLpTokenAccounts<'_, '_>,
    args: MigrateMeteoraDammClaimLpTokenIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    migrate_meteora_damm_claim_lp_token_invoke_signed_with_program_id(crate::ID, accounts, args, seeds)
}

pub fn migrate_meteora_damm_claim_lp_token_verify_account_keys(
    accounts: MigrateMeteoraDammClaimLpTokenAccounts<'_, '_>,
    keys: MigrateMeteoraDammClaimLpTokenKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.virtual_pool.key, keys.virtual_pool),
        (*accounts.migration_metadata.key, keys.migration_metadata),
        (*accounts.pool_authority.key, keys.pool_authority),
        (*accounts.lp_mint.key, keys.lp_mint),
        (*accounts.source_token.key, keys.source_token),
        (*accounts.destination_token.key, keys.destination_token),
        (*accounts.owner.key, keys.owner),
        (*accounts.sender.key, keys.sender),
        (*accounts.token_program.key, keys.token_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}

pub fn migrate_meteora_damm_claim_lp_token_verify_is_writable_privileges<'me, 'info>(
    accounts: MigrateMeteoraDammClaimLpTokenAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for acct in [
        accounts.migration_metadata,
        accounts.pool_authority,
        accounts.source_token,
        accounts.destination_token,
    ] {
        if !acct.is_writable {
            return Err((acct, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}

pub fn migrate_meteora_damm_claim_lp_token_verify_is_signer_privileges<'me, 'info>(
    accounts: MigrateMeteoraDammClaimLpTokenAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    if !accounts.sender.is_signer {
        return Err((accounts.sender, ProgramError::MissingRequiredSignature));
    }
    Ok(())
}

pub fn migrate_meteora_damm_claim_lp_token_verify_account_privileges<'me, 'info>(
    accounts: MigrateMeteoraDammClaimLpTokenAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    migrate_meteora_damm_claim_lp_token_verify_is_writable_privileges(accounts)?;
    migrate_meteora_damm_claim_lp_token_verify_is_signer_privileges(accounts)?;
    Ok(())
}

pub const MIGRATE_METEORA_DAMM_LOCK_LP_TOKEN_IX_ACCOUNTS_LEN: usize = 17;
#[derive(Copy, Clone, Debug)]
pub struct MigrateMeteoraDammLockLpTokenAccounts<'me, 'info> {
    pub virtual_pool: &'me AccountInfo<'info>,
    pub migration_metadata: &'me AccountInfo<'info>,
    pub pool_authority: &'me AccountInfo<'info>,
    pub pool: &'me AccountInfo<'info>,
    pub lp_mint: &'me AccountInfo<'info>,
    pub lock_escrow: &'me AccountInfo<'info>,
    pub owner: &'me AccountInfo<'info>,
    pub source_tokens: &'me AccountInfo<'info>,
    pub escrow_vault: &'me AccountInfo<'info>,
    pub amm_program: &'me AccountInfo<'info>,
    pub a_vault: &'me AccountInfo<'info>,
    pub b_vault: &'me AccountInfo<'info>,
    pub a_vault_lp: &'me AccountInfo<'info>,
    pub b_vault_lp: &'me AccountInfo<'info>,
    pub a_vault_lp_mint: &'me AccountInfo<'info>,
    pub b_vault_lp_mint: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MigrateMeteoraDammLockLpTokenKeys {
    pub virtual_pool: Pubkey,
    pub migration_metadata: Pubkey,
    pub pool_authority: Pubkey,
    pub pool: Pubkey,
    pub lp_mint: Pubkey,
    pub lock_escrow: Pubkey,
    pub owner: Pubkey,
    pub source_tokens: Pubkey,
    pub escrow_vault: Pubkey,
    pub amm_program: Pubkey,
    pub a_vault: Pubkey,
    pub b_vault: Pubkey,
    pub a_vault_lp: Pubkey,
    pub b_vault_lp: Pubkey,
    pub a_vault_lp_mint: Pubkey,
    pub b_vault_lp_mint: Pubkey,
    pub token_program: Pubkey,
}

impl From<MigrateMeteoraDammLockLpTokenAccounts<'_, '_>>
    for MigrateMeteoraDammLockLpTokenKeys
{
    fn from(accounts: MigrateMeteoraDammLockLpTokenAccounts) -> Self {
        Self {
            virtual_pool: *accounts.virtual_pool.key,
            migration_metadata: *accounts.migration_metadata.key,
            pool_authority: *accounts.pool_authority.key,
            pool: *accounts.pool.key,
            lp_mint: *accounts.lp_mint.key,
            lock_escrow: *accounts.lock_escrow.key,
            owner: *accounts.owner.key,
            source_tokens: *accounts.source_tokens.key,
            escrow_vault: *accounts.escrow_vault.key,
            amm_program: *accounts.amm_program.key,
            a_vault: *accounts.a_vault.key,
            b_vault: *accounts.b_vault.key,
            a_vault_lp: *accounts.a_vault_lp.key,
            b_vault_lp: *accounts.b_vault_lp.key,
            a_vault_lp_mint: *accounts.a_vault_lp_mint.key,
            b_vault_lp_mint: *accounts.b_vault_lp_mint.key,
            token_program: *accounts.token_program.key,
        }
    }
}

impl From<MigrateMeteoraDammLockLpTokenKeys>
    for [AccountMeta; MIGRATE_METEORA_DAMM_LOCK_LP_TOKEN_IX_ACCOUNTS_LEN]
{
    fn from(keys: MigrateMeteoraDammLockLpTokenKeys) -> Self {
        [
            AccountMeta { 
                pubkey: keys.virtual_pool,
                is_signer: false,
                is_writable: false
             },
            AccountMeta { 
                pubkey: keys.migration_metadata, 
                is_signer: false, 
                is_writable: true
             },
            AccountMeta { 
                pubkey: keys.pool_authority, 
                is_signer: false, 
                is_writable: true
             },
            AccountMeta { 
                pubkey: keys.pool, 
                is_signer: false, 
                is_writable: true
             },
            AccountMeta { 
                pubkey: keys.lp_mint, 
                is_signer: false, 
                is_writable: false 
            },
            AccountMeta { 
                pubkey: keys.lock_escrow, 
                is_signer: false, 
                is_writable: true 
            },
            AccountMeta { 
                pubkey: keys.owner, 
                is_signer: false, 
                is_writable: false
             },
            AccountMeta { 
                pubkey: keys.source_tokens, 
                is_signer: false, 
                is_writable: true
             },
            AccountMeta { 
                pubkey: keys.escrow_vault, 
                is_signer: false, 
                is_writable: true 
            },
            AccountMeta { 
                pubkey: keys.amm_program,
                is_signer: false, 
                is_writable: false
             },
            AccountMeta { 
                pubkey: keys.a_vault, 
                is_signer: false, 
                is_writable: false
             },
            AccountMeta { 
                pubkey: keys.b_vault, 
                is_signer: false, 
                is_writable: false 
            },
            AccountMeta { 
                pubkey: keys.a_vault_lp, 
                is_signer: false, 
                is_writable: false
             },
            AccountMeta { 
                pubkey: keys.b_vault_lp, 
                is_signer: false, 
                is_writable: false
             },
            AccountMeta { 
                pubkey: keys.a_vault_lp_mint, 
                is_signer: false, 
                is_writable: false
             },
            AccountMeta { 
                pubkey: keys.b_vault_lp_mint, 
                is_signer: false, 
                is_writable: false
             },
            AccountMeta { 
                pubkey: keys.token_program, 
                is_signer: false, 
                is_writable: false 
            },
        ]
    }
}

impl From<[Pubkey; MIGRATE_METEORA_DAMM_LOCK_LP_TOKEN_IX_ACCOUNTS_LEN]>
    for MigrateMeteoraDammLockLpTokenKeys
{
    fn from(pubkeys: [Pubkey; MIGRATE_METEORA_DAMM_LOCK_LP_TOKEN_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            virtual_pool: pubkeys[0],
            migration_metadata: pubkeys[1],
            pool_authority: pubkeys[2],
            pool: pubkeys[3],
            lp_mint: pubkeys[4],
            lock_escrow: pubkeys[5],
            owner: pubkeys[6],
            source_tokens: pubkeys[7],
            escrow_vault: pubkeys[8],
            amm_program: pubkeys[9],
            a_vault: pubkeys[10],
            b_vault: pubkeys[11],
            a_vault_lp: pubkeys[12],
            b_vault_lp: pubkeys[13],
            a_vault_lp_mint: pubkeys[14],
            b_vault_lp_mint: pubkeys[15],
            token_program: pubkeys[16],
        }
    }
}

impl<'info> From<MigrateMeteoraDammLockLpTokenAccounts<'_, 'info>>
    for [AccountInfo<'info>; MIGRATE_METEORA_DAMM_LOCK_LP_TOKEN_IX_ACCOUNTS_LEN]
{
    fn from(accounts: MigrateMeteoraDammLockLpTokenAccounts<'_, 'info>) -> Self {
        [
            accounts.virtual_pool.clone(),
            accounts.migration_metadata.clone(),
            accounts.pool_authority.clone(),
            accounts.pool.clone(),
            accounts.lp_mint.clone(),
            accounts.lock_escrow.clone(),
            accounts.owner.clone(),
            accounts.source_tokens.clone(),
            accounts.escrow_vault.clone(),
            accounts.amm_program.clone(),
            accounts.a_vault.clone(),
            accounts.b_vault.clone(),
            accounts.a_vault_lp.clone(),
            accounts.b_vault_lp.clone(),
            accounts.a_vault_lp_mint.clone(),
            accounts.b_vault_lp_mint.clone(),
            accounts.token_program.clone(),
        ]
    }
}

impl<'me, 'info>
    From<&'me [AccountInfo<'info>; MIGRATE_METEORA_DAMM_LOCK_LP_TOKEN_IX_ACCOUNTS_LEN]>
    for MigrateMeteoraDammLockLpTokenAccounts<'me, 'info>
{
    fn from(arr: &'me [AccountInfo<'info>; MIGRATE_METEORA_DAMM_LOCK_LP_TOKEN_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            virtual_pool: &arr[0],
            migration_metadata: &arr[1],
            pool_authority: &arr[2],
            pool: &arr[3],
            lp_mint: &arr[4],
            lock_escrow: &arr[5],
            owner: &arr[6],
            source_tokens: &arr[7],
            escrow_vault: &arr[8],
            amm_program: &arr[9],
            a_vault: &arr[10],
            b_vault: &arr[11],
            a_vault_lp: &arr[12],
            b_vault_lp: &arr[13],
            a_vault_lp_mint: &arr[14],
            b_vault_lp_mint: &arr[15],
            token_program: &arr[16],
        }
    }
}

pub const MIGRATE_METEORA_DAMM_LOCK_LP_TOKEN_IX_DISCM: [u8; 8] = [177, 55, 238, 157, 251, 88, 165, 42];

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct MigrateMeteoraDammLockLpTokenIxArgs {}

#[derive(Clone, Debug, PartialEq)]
pub struct MigrateMeteoraDammLockLpTokenIxData(pub MigrateMeteoraDammLockLpTokenIxArgs);

impl From<MigrateMeteoraDammLockLpTokenIxArgs> for MigrateMeteoraDammLockLpTokenIxData {
    fn from(args: MigrateMeteoraDammLockLpTokenIxArgs) -> Self {
        Self(args)
    }
}

impl MigrateMeteoraDammLockLpTokenIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != MIGRATE_METEORA_DAMM_LOCK_LP_TOKEN_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    MIGRATE_METEORA_DAMM_LOCK_LP_TOKEN_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(MigrateMeteoraDammLockLpTokenIxArgs::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&MIGRATE_METEORA_DAMM_LOCK_LP_TOKEN_IX_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}

pub fn migrate_meteora_damm_lock_lp_token_ix_with_program_id(
    program_id: Pubkey,
    keys: MigrateMeteoraDammLockLpTokenKeys,
    args: MigrateMeteoraDammLockLpTokenIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; MIGRATE_METEORA_DAMM_LOCK_LP_TOKEN_IX_ACCOUNTS_LEN] = keys.into();
    let data: MigrateMeteoraDammLockLpTokenIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}

pub fn migrate_meteora_damm_lock_lp_token_ix(
    keys: MigrateMeteoraDammLockLpTokenKeys,
    args: MigrateMeteoraDammLockLpTokenIxArgs,
) -> std::io::Result<Instruction> {
    migrate_meteora_damm_lock_lp_token_ix_with_program_id(crate::ID, keys, args)
}

pub fn migrate_meteora_damm_lock_lp_token_invoke_with_program_id(
    program_id: Pubkey,
    accounts: MigrateMeteoraDammLockLpTokenAccounts<'_, '_>,
    args: MigrateMeteoraDammLockLpTokenIxArgs,
) -> ProgramResult {
    let keys: MigrateMeteoraDammLockLpTokenKeys = accounts.into();
    let ix = migrate_meteora_damm_lock_lp_token_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}

pub fn migrate_meteora_damm_lock_lp_token_invoke(
    accounts: MigrateMeteoraDammLockLpTokenAccounts<'_, '_>,
    args: MigrateMeteoraDammLockLpTokenIxArgs,
) -> ProgramResult {
    migrate_meteora_damm_lock_lp_token_invoke_with_program_id(crate::ID, accounts, args)
}

pub fn migrate_meteora_damm_lock_lp_token_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: MigrateMeteoraDammLockLpTokenAccounts<'_, '_>,
    args: MigrateMeteoraDammLockLpTokenIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: MigrateMeteoraDammLockLpTokenKeys = accounts.into();
    let ix = migrate_meteora_damm_lock_lp_token_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}

pub fn migrate_meteora_damm_lock_lp_token_invoke_signed(
    accounts: MigrateMeteoraDammLockLpTokenAccounts<'_, '_>,
    args: MigrateMeteoraDammLockLpTokenIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    migrate_meteora_damm_lock_lp_token_invoke_signed_with_program_id(crate::ID, accounts, args, seeds)
}

pub fn migrate_meteora_damm_lock_lp_token_verify_account_keys(
    accounts: MigrateMeteoraDammLockLpTokenAccounts<'_, '_>,
    keys: MigrateMeteoraDammLockLpTokenKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.virtual_pool.key, keys.virtual_pool),
        (*accounts.migration_metadata.key, keys.migration_metadata),
        (*accounts.pool_authority.key, keys.pool_authority),
        (*accounts.pool.key, keys.pool),
        (*accounts.lp_mint.key, keys.lp_mint),
        (*accounts.lock_escrow.key, keys.lock_escrow),
        (*accounts.owner.key, keys.owner),
        (*accounts.source_tokens.key, keys.source_tokens),
        (*accounts.escrow_vault.key, keys.escrow_vault),
        (*accounts.amm_program.key, keys.amm_program),
        (*accounts.a_vault.key, keys.a_vault),
        (*accounts.b_vault.key, keys.b_vault),
        (*accounts.a_vault_lp.key, keys.a_vault_lp),
        (*accounts.b_vault_lp.key, keys.b_vault_lp),
        (*accounts.a_vault_lp_mint.key, keys.a_vault_lp_mint),
        (*accounts.b_vault_lp_mint.key, keys.b_vault_lp_mint),
        (*accounts.token_program.key, keys.token_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}

pub fn migrate_meteora_damm_lock_lp_token_verify_is_writable_privileges<'me, 'info>(
    accounts: MigrateMeteoraDammLockLpTokenAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for acct in [
        accounts.migration_metadata,
        accounts.pool_authority,
        accounts.pool,
        accounts.lock_escrow,
        accounts.source_tokens,
        accounts.escrow_vault,
    ] {
        if !acct.is_writable {
            return Err((acct, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}

pub fn migrate_meteora_damm_lock_lp_token_verify_is_signer_privileges<'me, 'info>(
    accounts: MigrateMeteoraDammLockLpTokenAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    Ok(())
}

pub fn migrate_meteora_damm_lock_lp_token_verify_account_privileges<'me, 'info>(
    accounts: MigrateMeteoraDammLockLpTokenAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    migrate_meteora_damm_lock_lp_token_verify_is_writable_privileges(accounts)?;
    migrate_meteora_damm_lock_lp_token_verify_is_signer_privileges(accounts)?;
    Ok(())
}

pub const MIGRATION_DAMM_V2_IX_ACCOUNTS_LEN: usize = 25;
#[derive(Copy, Clone, Debug)]
pub struct MigrationDammV2Accounts<'me, 'info> {
    pub virtual_pool: &'me AccountInfo<'info>,
    pub migration_metadata: &'me AccountInfo<'info>,
    pub config: &'me AccountInfo<'info>,
    pub pool_authority: &'me AccountInfo<'info>,
    pub pool: &'me AccountInfo<'info>,
    pub first_position_nft_mint: &'me AccountInfo<'info>,
    pub first_position_nft_account: &'me AccountInfo<'info>,
    pub first_position: &'me AccountInfo<'info>,
    pub second_position_nft_mint: &'me AccountInfo<'info>,
    pub second_position_nft_account: &'me AccountInfo<'info>,
    pub second_position: &'me AccountInfo<'info>,
    pub damm_pool_authority: &'me AccountInfo<'info>,
    pub amm_program: &'me AccountInfo<'info>,
    pub base_mint: &'me AccountInfo<'info>,
    pub quote_mint: &'me AccountInfo<'info>,
    pub token_a_vault: &'me AccountInfo<'info>,
    pub token_b_vault: &'me AccountInfo<'info>,
    pub base_vault: &'me AccountInfo<'info>,
    pub quote_vault: &'me AccountInfo<'info>,
    pub payer: &'me AccountInfo<'info>,
    pub token_base_program: &'me AccountInfo<'info>,
    pub token_quote_program: &'me AccountInfo<'info>,
    pub token_2022_program: &'me AccountInfo<'info>,
    pub damm_event_authority: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MigrationDammV2Keys {
    pub virtual_pool: Pubkey,
    pub migration_metadata: Pubkey,
    pub config: Pubkey,
    pub pool_authority: Pubkey,
    pub pool: Pubkey,
    pub first_position_nft_mint: Pubkey,
    pub first_position_nft_account: Pubkey,
    pub first_position: Pubkey,
    pub second_position_nft_mint: Pubkey,
    pub second_position_nft_account: Pubkey,
    pub second_position: Pubkey,
    pub damm_pool_authority: Pubkey,
    pub amm_program: Pubkey,
    pub base_mint: Pubkey,
    pub quote_mint: Pubkey,
    pub token_a_vault: Pubkey,
    pub token_b_vault: Pubkey,
    pub base_vault: Pubkey,
    pub quote_vault: Pubkey,
    pub payer: Pubkey,
    pub token_base_program: Pubkey,
    pub token_quote_program: Pubkey,
    pub token_2022_program: Pubkey,
    pub damm_event_authority: Pubkey,
    pub system_program: Pubkey,
}

impl From<MigrationDammV2Accounts<'_, '_>> for MigrationDammV2Keys {
    fn from(accounts: MigrationDammV2Accounts) -> Self {
        Self {
            virtual_pool: *accounts.virtual_pool.key,
            migration_metadata: *accounts.migration_metadata.key,
            config: *accounts.config.key,
            pool_authority: *accounts.pool_authority.key,
            pool: *accounts.pool.key,
            first_position_nft_mint: *accounts.first_position_nft_mint.key,
            first_position_nft_account: *accounts.first_position_nft_account.key,
            first_position: *accounts.first_position.key,
            second_position_nft_mint: *accounts.second_position_nft_mint.key,
            second_position_nft_account: *accounts.second_position_nft_account.key,
            second_position: *accounts.second_position.key,
            damm_pool_authority: *accounts.damm_pool_authority.key,
            amm_program: *accounts.amm_program.key,
            base_mint: *accounts.base_mint.key,
            quote_mint: *accounts.quote_mint.key,
            token_a_vault: *accounts.token_a_vault.key,
            token_b_vault: *accounts.token_b_vault.key,
            base_vault: *accounts.base_vault.key,
            quote_vault: *accounts.quote_vault.key,
            payer: *accounts.payer.key,
            token_base_program: *accounts.token_base_program.key,
            token_quote_program: *accounts.token_quote_program.key,
            token_2022_program: *accounts.token_2022_program.key,
            damm_event_authority: *accounts.damm_event_authority.key,
            system_program: *accounts.system_program.key,
        }
    }
}

impl From<MigrationDammV2Keys> for [AccountMeta; MIGRATION_DAMM_V2_IX_ACCOUNTS_LEN] {
    fn from(keys: MigrationDammV2Keys) -> Self {
        [
            AccountMeta {
                pubkey: keys.virtual_pool,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.migration_metadata,
                is_signer: false,
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
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.pool,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.first_position_nft_mint,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.first_position_nft_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.first_position,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.second_position_nft_mint,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.second_position_nft_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.second_position,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.damm_pool_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.amm_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.base_mint,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.quote_mint,
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
                pubkey: keys.payer,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_base_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_quote_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_2022_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.damm_event_authority,
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

impl From<[Pubkey; MIGRATION_DAMM_V2_IX_ACCOUNTS_LEN]> for MigrationDammV2Keys {
    fn from(pubkeys: [Pubkey; MIGRATION_DAMM_V2_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            virtual_pool: pubkeys[0],
            migration_metadata: pubkeys[1],
            config: pubkeys[2],
            pool_authority: pubkeys[3],
            pool: pubkeys[4],
            first_position_nft_mint: pubkeys[5],
            first_position_nft_account: pubkeys[6],
            first_position: pubkeys[7],
            second_position_nft_mint: pubkeys[8],
            second_position_nft_account: pubkeys[9],
            second_position: pubkeys[10],
            damm_pool_authority: pubkeys[11],
            amm_program: pubkeys[12],
            base_mint: pubkeys[13],
            quote_mint: pubkeys[14],
            token_a_vault: pubkeys[15],
            token_b_vault: pubkeys[16],
            base_vault: pubkeys[17],
            quote_vault: pubkeys[18],
            payer: pubkeys[19],
            token_base_program: pubkeys[20],
            token_quote_program: pubkeys[21],
            token_2022_program: pubkeys[22],
            damm_event_authority: pubkeys[23],
            system_program: pubkeys[24],
        }
    }
}

impl<'info> From<MigrationDammV2Accounts<'_, 'info>> for [AccountInfo<'info>; MIGRATION_DAMM_V2_IX_ACCOUNTS_LEN] {
    fn from(accounts: MigrationDammV2Accounts<'_, 'info>) -> Self {
        [
            accounts.virtual_pool.clone(),
            accounts.migration_metadata.clone(),
            accounts.config.clone(),
            accounts.pool_authority.clone(),
            accounts.pool.clone(),
            accounts.first_position_nft_mint.clone(),
            accounts.first_position_nft_account.clone(),
            accounts.first_position.clone(),
            accounts.second_position_nft_mint.clone(),
            accounts.second_position_nft_account.clone(),
            accounts.second_position.clone(),
            accounts.damm_pool_authority.clone(),
            accounts.amm_program.clone(),
            accounts.base_mint.clone(),
            accounts.quote_mint.clone(),
            accounts.token_a_vault.clone(),
            accounts.token_b_vault.clone(),
            accounts.base_vault.clone(),
            accounts.quote_vault.clone(),
            accounts.payer.clone(),
            accounts.token_base_program.clone(),
            accounts.token_quote_program.clone(),
            accounts.token_2022_program.clone(),
            accounts.damm_event_authority.clone(),
            accounts.system_program.clone(),
        ]
    }
}

impl<'me, 'info> From<&'me [AccountInfo<'info>; MIGRATION_DAMM_V2_IX_ACCOUNTS_LEN]> for MigrationDammV2Accounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; MIGRATION_DAMM_V2_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            virtual_pool: &arr[0],
            migration_metadata: &arr[1],
            config: &arr[2],
            pool_authority: &arr[3],
            pool: &arr[4],
            first_position_nft_mint: &arr[5],
            first_position_nft_account: &arr[6],
            first_position: &arr[7],
            second_position_nft_mint: &arr[8],
            second_position_nft_account: &arr[9],
            second_position: &arr[10],
            damm_pool_authority: &arr[11],
            amm_program: &arr[12],
            base_mint: &arr[13],
            quote_mint: &arr[14],
            token_a_vault: &arr[15],
            token_b_vault: &arr[16],
            base_vault: &arr[17],
            quote_vault: &arr[18],
            payer: &arr[19],
            token_base_program: &arr[20],
            token_quote_program: &arr[21],
            token_2022_program: &arr[22],
            damm_event_authority: &arr[23],
            system_program: &arr[24],
        }
    }
}

pub const MIGRATION_DAMM_V2_IX_DISCM: [u8; 8] = [156, 169, 230, 103, 53, 228, 80, 64];

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct MigrationDammV2IxArgs {}

#[derive(Clone, Debug, PartialEq)]
pub struct MigrationDammV2IxData(pub MigrationDammV2IxArgs);

impl From<MigrationDammV2IxArgs> for MigrationDammV2IxData {
    fn from(args: MigrationDammV2IxArgs) -> Self {
        Self(args)
    }
}

impl MigrationDammV2IxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != MIGRATION_DAMM_V2_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    MIGRATION_DAMM_V2_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(MigrationDammV2IxArgs::deserialize(&mut reader)?))
    }

    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&MIGRATION_DAMM_V2_IX_DISCM)?;
        self.0.serialize(&mut writer)
    }

    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}

pub fn migration_damm_v2_ix_with_program_id(
    program_id: Pubkey,
    keys: MigrationDammV2Keys,
    args: MigrationDammV2IxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; MIGRATION_DAMM_V2_IX_ACCOUNTS_LEN] = keys.into();
    let data: MigrationDammV2IxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}

pub fn migration_damm_v2_ix(
    keys: MigrationDammV2Keys,
    args: MigrationDammV2IxArgs,
) -> std::io::Result<Instruction> {
    migration_damm_v2_ix_with_program_id(crate::ID, keys, args)
}

pub fn migration_damm_v2_invoke_with_program_id(
    program_id: Pubkey,
    accounts: MigrationDammV2Accounts<'_, '_>,
    args: MigrationDammV2IxArgs,
) -> ProgramResult {
    let keys: MigrationDammV2Keys = accounts.into();
    let ix = migration_damm_v2_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}

pub fn migration_damm_v2_invoke(
    accounts: MigrationDammV2Accounts<'_, '_>,
    args: MigrationDammV2IxArgs,
) -> ProgramResult {
    migration_damm_v2_invoke_with_program_id(crate::ID, accounts, args)
}

pub fn migration_damm_v2_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: MigrationDammV2Accounts<'_, '_>,
    args: MigrationDammV2IxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: MigrationDammV2Keys = accounts.into();
    let ix = migration_damm_v2_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}

pub fn migration_damm_v2_invoke_signed(
    accounts: MigrationDammV2Accounts<'_, '_>,
    args: MigrationDammV2IxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    migration_damm_v2_invoke_signed_with_program_id(crate::ID, accounts, args, seeds)
}

pub fn migration_damm_v2_verify_account_keys(
    accounts: MigrationDammV2Accounts<'_, '_>,
    keys: MigrationDammV2Keys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.virtual_pool.key, keys.virtual_pool),
        (*accounts.migration_metadata.key, keys.migration_metadata),
        (*accounts.config.key, keys.config),
        (*accounts.pool_authority.key, keys.pool_authority),
        (*accounts.pool.key, keys.pool),
        (*accounts.first_position_nft_mint.key, keys.first_position_nft_mint),
        (*accounts.first_position_nft_account.key, keys.first_position_nft_account),
        (*accounts.first_position.key, keys.first_position),
        (*accounts.second_position_nft_mint.key, keys.second_position_nft_mint),
        (*accounts.second_position_nft_account.key, keys.second_position_nft_account),
        (*accounts.second_position.key, keys.second_position),
        (*accounts.damm_pool_authority.key, keys.damm_pool_authority),
        (*accounts.amm_program.key, keys.amm_program),
        (*accounts.base_mint.key, keys.base_mint),
        (*accounts.quote_mint.key, keys.quote_mint),
        (*accounts.token_a_vault.key, keys.token_a_vault),
        (*accounts.token_b_vault.key, keys.token_b_vault),
        (*accounts.base_vault.key, keys.base_vault),
        (*accounts.quote_vault.key, keys.quote_vault),
        (*accounts.payer.key, keys.payer),
        (*accounts.token_base_program.key, keys.token_base_program),
        (*accounts.token_quote_program.key, keys.token_quote_program),
        (*accounts.token_2022_program.key, keys.token_2022_program),
        (*accounts.damm_event_authority.key, keys.damm_event_authority),
        (*accounts.system_program.key, keys.system_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}

pub fn migration_damm_v2_verify_is_writable_privileges<'me, 'info>(
    accounts: MigrationDammV2Accounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.virtual_pool,
        accounts.pool_authority,
        accounts.pool,
        accounts.first_position_nft_mint,
        accounts.first_position_nft_account,
        accounts.first_position,
        accounts.base_mint,
        accounts.quote_mint,
        accounts.token_a_vault,
        accounts.token_b_vault,
        accounts.base_vault,
        accounts.quote_vault,
        accounts.payer,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}

pub fn migration_damm_v2_verify_is_signer_privileges<'me, 'info>(
    accounts: MigrationDammV2Accounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    if !accounts.payer.is_signer {
        return Err((accounts.payer, ProgramError::MissingRequiredSignature));
    }
    Ok(())
}

pub fn migration_damm_v2_verify_account_privileges<'me, 'info>(
    accounts: MigrationDammV2Accounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    migration_damm_v2_verify_is_writable_privileges(accounts)?;
    migration_damm_v2_verify_is_signer_privileges(accounts)?;
    Ok(())
}
pub const MIGRATION_DAMM_V2_CREATE_METADATA_IX_ACCOUNTS_LEN: usize = 7;

#[derive(Copy, Clone, Debug)]
pub struct MigrationDammV2CreateMetadataAccounts<'me, 'info> {
    pub virtual_pool: &'me AccountInfo<'info>,
    pub config: &'me AccountInfo<'info>,
    pub migration_metadata: &'me AccountInfo<'info>,
    pub payer: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MigrationDammV2CreateMetadataKeys {
    pub virtual_pool: Pubkey,
    pub config: Pubkey,
    pub migration_metadata: Pubkey,
    pub payer: Pubkey,
    pub system_program: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}

impl From<MigrationDammV2CreateMetadataAccounts<'_, '_>> for MigrationDammV2CreateMetadataKeys {
    fn from(accounts: MigrationDammV2CreateMetadataAccounts) -> Self {
        Self {
            virtual_pool: *accounts.virtual_pool.key,
            config: *accounts.config.key,
            migration_metadata: *accounts.migration_metadata.key,
            payer: *accounts.payer.key,
            system_program: *accounts.system_program.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}

impl From<MigrationDammV2CreateMetadataKeys> for [AccountMeta; MIGRATION_DAMM_V2_CREATE_METADATA_IX_ACCOUNTS_LEN] {
    fn from(keys: MigrationDammV2CreateMetadataKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.virtual_pool,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.migration_metadata,
                is_signer: false,
                is_writable: true,
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

impl From<[Pubkey; MIGRATION_DAMM_V2_CREATE_METADATA_IX_ACCOUNTS_LEN]> for MigrationDammV2CreateMetadataKeys {
    fn from(pubkeys: [Pubkey; MIGRATION_DAMM_V2_CREATE_METADATA_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            virtual_pool: pubkeys[0],
            config: pubkeys[1],
            migration_metadata: pubkeys[2],
            payer: pubkeys[3],
            system_program: pubkeys[4],
            event_authority: pubkeys[5],
            program: pubkeys[6],
        }
    }
}

impl<'info> From<MigrationDammV2CreateMetadataAccounts<'_, 'info>>
    for [AccountInfo<'info>; MIGRATION_DAMM_V2_CREATE_METADATA_IX_ACCOUNTS_LEN]
{
    fn from(accounts: MigrationDammV2CreateMetadataAccounts<'_, 'info>) -> Self {
        [
            accounts.virtual_pool.clone(),
            accounts.config.clone(),
            accounts.migration_metadata.clone(),
            accounts.payer.clone(),
            accounts.system_program.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}

impl<'me, 'info>
    From<&'me [AccountInfo<'info>; MIGRATION_DAMM_V2_CREATE_METADATA_IX_ACCOUNTS_LEN]>
    for MigrationDammV2CreateMetadataAccounts<'me, 'info>
{
    fn from(arr: &'me [AccountInfo<'info>; MIGRATION_DAMM_V2_CREATE_METADATA_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            virtual_pool: &arr[0],
            config: &arr[1],
            migration_metadata: &arr[2],
            payer: &arr[3],
            system_program: &arr[4],
            event_authority: &arr[5],
            program: &arr[6],
        }
    }
}

pub const MIGRATION_DAMM_V2_CREATE_METADATA_IX_DISCM: [u8; 8] = [109, 189, 19, 36, 195, 183, 222, 82];

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct MigrationDammV2CreateMetadataIxArgs {}

#[derive(Clone, Debug, PartialEq)]
pub struct MigrationDammV2CreateMetadataIxData(pub MigrationDammV2CreateMetadataIxArgs);

impl From<MigrationDammV2CreateMetadataIxArgs> for MigrationDammV2CreateMetadataIxData {
    fn from(args: MigrationDammV2CreateMetadataIxArgs) -> Self {
        Self(args)
    }
}

impl MigrationDammV2CreateMetadataIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != MIGRATION_DAMM_V2_CREATE_METADATA_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    MIGRATION_DAMM_V2_CREATE_METADATA_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(MigrationDammV2CreateMetadataIxArgs::deserialize(&mut reader)?))
    }
    
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&MIGRATION_DAMM_V2_CREATE_METADATA_IX_DISCM)?;
        self.0.serialize(&mut writer)
    }
    
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}

pub fn migration_damm_v2_create_metadata_ix_with_program_id(
    program_id: Pubkey,
    keys: MigrationDammV2CreateMetadataKeys,
    args: MigrationDammV2CreateMetadataIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; MIGRATION_DAMM_V2_CREATE_METADATA_IX_ACCOUNTS_LEN] = keys.into();
    let data: MigrationDammV2CreateMetadataIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}

pub fn migration_damm_v2_create_metadata_ix(
    keys: MigrationDammV2CreateMetadataKeys,
    args: MigrationDammV2CreateMetadataIxArgs,
) -> std::io::Result<Instruction> {
    migration_damm_v2_create_metadata_ix_with_program_id(crate::ID, keys, args)
}

pub fn migration_damm_v2_create_metadata_invoke_with_program_id(
    program_id: Pubkey,
    accounts: MigrationDammV2CreateMetadataAccounts<'_, '_>,
    args: MigrationDammV2CreateMetadataIxArgs,
) -> ProgramResult {
    let keys: MigrationDammV2CreateMetadataKeys = accounts.into();
    let ix = migration_damm_v2_create_metadata_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}

pub fn migration_damm_v2_create_metadata_invoke(
    accounts: MigrationDammV2CreateMetadataAccounts<'_, '_>,
    args: MigrationDammV2CreateMetadataIxArgs,
) -> ProgramResult {
    migration_damm_v2_create_metadata_invoke_with_program_id(crate::ID, accounts, args)
}

pub fn migration_damm_v2_create_metadata_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: MigrationDammV2CreateMetadataAccounts<'_, '_>,
    args: MigrationDammV2CreateMetadataIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: MigrationDammV2CreateMetadataKeys = accounts.into();
    let ix = migration_damm_v2_create_metadata_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}

pub fn migration_damm_v2_create_metadata_invoke_signed(
    accounts: MigrationDammV2CreateMetadataAccounts<'_, '_>,
    args: MigrationDammV2CreateMetadataIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    migration_damm_v2_create_metadata_invoke_signed_with_program_id(crate::ID, accounts, args, seeds)
}

pub fn migration_damm_v2_create_metadata_verify_account_keys(
    accounts: MigrationDammV2CreateMetadataAccounts<'_, '_>,
    keys: MigrationDammV2CreateMetadataKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.virtual_pool.key, keys.virtual_pool),
        (*accounts.config.key, keys.config),
        (*accounts.migration_metadata.key, keys.migration_metadata),
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

pub fn migration_damm_v2_create_metadata_verify_is_writable_privileges<'me, 'info>(
    accounts: MigrationDammV2CreateMetadataAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for acct in [
        accounts.migration_metadata,
        accounts.payer,
    ] {
        if !acct.is_writable {
            return Err((acct, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}

pub fn migration_damm_v2_create_metadata_verify_is_signer_privileges<'me, 'info>(
    accounts: MigrationDammV2CreateMetadataAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    if !accounts.payer.is_signer {
        return Err((accounts.payer, ProgramError::MissingRequiredSignature));
    }
    Ok(())
}

pub fn migration_damm_v2_create_metadata_verify_account_privileges<'me, 'info>(
    accounts: MigrationDammV2CreateMetadataAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    migration_damm_v2_create_metadata_verify_is_writable_privileges(accounts)?;
    migration_damm_v2_create_metadata_verify_is_signer_privileges(accounts)?;
    Ok(())
}
pub const MIGRATION_METEORA_DAMM_CREATE_METADATA_IX_ACCOUNTS_LEN: usize = 7;

#[derive(Copy, Clone, Debug)]
pub struct MigrationMeteoraDammCreateMetadataAccounts<'me, 'info> {
    pub virtual_pool: &'me AccountInfo<'info>,
    pub config: &'me AccountInfo<'info>,
    pub migration_metadata: &'me AccountInfo<'info>,
    pub payer: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MigrationMeteoraDammCreateMetadataKeys {
    pub virtual_pool: Pubkey,
    pub config: Pubkey,
    pub migration_metadata: Pubkey,
    pub payer: Pubkey,
    pub system_program: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}

impl From<MigrationMeteoraDammCreateMetadataAccounts<'_, '_>> for MigrationMeteoraDammCreateMetadataKeys {
    fn from(accounts: MigrationMeteoraDammCreateMetadataAccounts) -> Self {
        Self {
            virtual_pool: *accounts.virtual_pool.key,
            config: *accounts.config.key,
            migration_metadata: *accounts.migration_metadata.key,
            payer: *accounts.payer.key,
            system_program: *accounts.system_program.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}

impl From<MigrationMeteoraDammCreateMetadataKeys> for [AccountMeta; MIGRATION_METEORA_DAMM_CREATE_METADATA_IX_ACCOUNTS_LEN] {
    fn from(keys: MigrationMeteoraDammCreateMetadataKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.virtual_pool,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.migration_metadata,
                is_signer: false,
                is_writable: true,
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

impl From<[Pubkey; MIGRATION_METEORA_DAMM_CREATE_METADATA_IX_ACCOUNTS_LEN]> for MigrationMeteoraDammCreateMetadataKeys {
    fn from(pubkeys: [Pubkey; MIGRATION_METEORA_DAMM_CREATE_METADATA_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            virtual_pool: pubkeys[0],
            config: pubkeys[1],
            migration_metadata: pubkeys[2],
            payer: pubkeys[3],
            system_program: pubkeys[4],
            event_authority: pubkeys[5],
            program: pubkeys[6],
        }
    }
}

impl<'info> From<MigrationMeteoraDammCreateMetadataAccounts<'_, 'info>>
    for [AccountInfo<'info>; MIGRATION_METEORA_DAMM_CREATE_METADATA_IX_ACCOUNTS_LEN]
{
    fn from(accounts: MigrationMeteoraDammCreateMetadataAccounts<'_, 'info>) -> Self {
        [
            accounts.virtual_pool.clone(),
            accounts.config.clone(),
            accounts.migration_metadata.clone(),
            accounts.payer.clone(),
            accounts.system_program.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}

impl<'me, 'info>
    From<&'me [AccountInfo<'info>; MIGRATION_METEORA_DAMM_CREATE_METADATA_IX_ACCOUNTS_LEN]>
    for MigrationMeteoraDammCreateMetadataAccounts<'me, 'info>
{
    fn from(arr: &'me [AccountInfo<'info>; MIGRATION_METEORA_DAMM_CREATE_METADATA_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            virtual_pool: &arr[0],
            config: &arr[1],
            migration_metadata: &arr[2],
            payer: &arr[3],
            system_program: &arr[4],
            event_authority: &arr[5],
            program: &arr[6],
        }
    }
}

pub const MIGRATION_METEORA_DAMM_CREATE_METADATA_IX_DISCM: [u8; 8] = [47, 94, 126, 115, 221, 226, 194, 133];

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct MigrationMeteoraDammCreateMetadataIxArgs {}

#[derive(Clone, Debug, PartialEq)]
pub struct MigrationMeteoraDammCreateMetadataIxData(pub MigrationMeteoraDammCreateMetadataIxArgs);

impl From<MigrationMeteoraDammCreateMetadataIxArgs> for MigrationMeteoraDammCreateMetadataIxData {
    fn from(args: MigrationMeteoraDammCreateMetadataIxArgs) -> Self {
        Self(args)
    }
}

impl MigrationMeteoraDammCreateMetadataIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != MIGRATION_METEORA_DAMM_CREATE_METADATA_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    MIGRATION_METEORA_DAMM_CREATE_METADATA_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(MigrationMeteoraDammCreateMetadataIxArgs::deserialize(&mut reader)?))
    }
    
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&MIGRATION_METEORA_DAMM_CREATE_METADATA_IX_DISCM)?;
        self.0.serialize(&mut writer)
    }
    
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}

pub fn migration_meteora_damm_create_metadata_ix_with_program_id(
    program_id: Pubkey,
    keys: MigrationMeteoraDammCreateMetadataKeys,
    args: MigrationMeteoraDammCreateMetadataIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; MIGRATION_METEORA_DAMM_CREATE_METADATA_IX_ACCOUNTS_LEN] = keys.into();
    let data: MigrationMeteoraDammCreateMetadataIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}

pub fn migration_meteora_damm_create_metadata_ix(
    keys: MigrationMeteoraDammCreateMetadataKeys,
    args: MigrationMeteoraDammCreateMetadataIxArgs,
) -> std::io::Result<Instruction> {
    migration_meteora_damm_create_metadata_ix_with_program_id(crate::ID, keys, args)
}

pub fn migration_meteora_damm_create_metadata_invoke_with_program_id(
    program_id: Pubkey,
    accounts: MigrationMeteoraDammCreateMetadataAccounts<'_, '_>,
    args: MigrationMeteoraDammCreateMetadataIxArgs,
) -> ProgramResult {
    let keys: MigrationMeteoraDammCreateMetadataKeys = accounts.into();
    let ix = migration_meteora_damm_create_metadata_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}

pub fn migration_meteora_damm_create_metadata_invoke(
    accounts: MigrationMeteoraDammCreateMetadataAccounts<'_, '_>,
    args: MigrationMeteoraDammCreateMetadataIxArgs,
) -> ProgramResult {
    migration_meteora_damm_create_metadata_invoke_with_program_id(crate::ID, accounts, args)
}

pub fn migration_meteora_damm_create_metadata_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: MigrationMeteoraDammCreateMetadataAccounts<'_, '_>,
    args: MigrationMeteoraDammCreateMetadataIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: MigrationMeteoraDammCreateMetadataKeys = accounts.into();
    let ix = migration_meteora_damm_create_metadata_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}

pub fn migration_meteora_damm_create_metadata_invoke_signed(
    accounts: MigrationMeteoraDammCreateMetadataAccounts<'_, '_>,
    args: MigrationMeteoraDammCreateMetadataIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    migration_meteora_damm_create_metadata_invoke_signed_with_program_id(crate::ID, accounts, args, seeds)
}

pub fn migration_meteora_damm_create_metadata_verify_account_keys(
    accounts: MigrationMeteoraDammCreateMetadataAccounts<'_, '_>,
    keys: MigrationMeteoraDammCreateMetadataKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.virtual_pool.key, keys.virtual_pool),
        (*accounts.config.key, keys.config),
        (*accounts.migration_metadata.key, keys.migration_metadata),
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

pub fn migration_meteora_damm_create_metadata_verify_is_writable_privileges<'me, 'info>(
    accounts: MigrationMeteoraDammCreateMetadataAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for acct in [
        accounts.migration_metadata,
        accounts.payer,
    ] {
        if !acct.is_writable {
            return Err((acct, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}

pub fn migration_meteora_damm_create_metadata_verify_is_signer_privileges<'me, 'info>(
    accounts: MigrationMeteoraDammCreateMetadataAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    if !accounts.payer.is_signer {
        return Err((accounts.payer, ProgramError::MissingRequiredSignature));
    }
    Ok(())
}

pub fn migration_meteora_damm_create_metadata_verify_account_privileges<'me, 'info>(
    accounts: MigrationMeteoraDammCreateMetadataAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    migration_meteora_damm_create_metadata_verify_is_writable_privileges(accounts)?;
    migration_meteora_damm_create_metadata_verify_is_signer_privileges(accounts)?;
    Ok(())
}
pub const PARTNER_WITHDRAW_SURPLUS_IX_ACCOUNTS_LEN: usize = 10;

#[derive(Copy, Clone, Debug)]
pub struct PartnerWithdrawSurplusAccounts<'me, 'info> {
    pub pool_authority: &'me AccountInfo<'info>,
    pub config: &'me AccountInfo<'info>,
    pub virtual_pool: &'me AccountInfo<'info>,
    pub token_quote_account: &'me AccountInfo<'info>,
    pub quote_vault: &'me AccountInfo<'info>,
    pub quote_mint: &'me AccountInfo<'info>,
    pub fee_claimer: &'me AccountInfo<'info>,
    pub token_quote_program: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct PartnerWithdrawSurplusKeys {
    pub pool_authority: Pubkey,
    pub config: Pubkey,
    pub virtual_pool: Pubkey,
    pub token_quote_account: Pubkey,
    pub quote_vault: Pubkey,
    pub quote_mint: Pubkey,
    pub fee_claimer: Pubkey,
    pub token_quote_program: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}

impl From<PartnerWithdrawSurplusAccounts<'_, '_>> for PartnerWithdrawSurplusKeys {
    fn from(accounts: PartnerWithdrawSurplusAccounts) -> Self {
        Self {
            pool_authority: *accounts.pool_authority.key,
            config: *accounts.config.key,
            virtual_pool: *accounts.virtual_pool.key,
            token_quote_account: *accounts.token_quote_account.key,
            quote_vault: *accounts.quote_vault.key,
            quote_mint: *accounts.quote_mint.key,
            fee_claimer: *accounts.fee_claimer.key,
            token_quote_program: *accounts.token_quote_program.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}

impl From<PartnerWithdrawSurplusKeys> for [AccountMeta; PARTNER_WITHDRAW_SURPLUS_IX_ACCOUNTS_LEN] {
    fn from(keys: PartnerWithdrawSurplusKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.pool_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.virtual_pool,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_quote_account,
                is_signer: false,
                is_writable: true,
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
                pubkey: keys.fee_claimer,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_quote_program,
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

impl From<[Pubkey; PARTNER_WITHDRAW_SURPLUS_IX_ACCOUNTS_LEN]> for PartnerWithdrawSurplusKeys {
    fn from(pubkeys: [Pubkey; PARTNER_WITHDRAW_SURPLUS_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            pool_authority: pubkeys[0],
            config: pubkeys[1],
            virtual_pool: pubkeys[2],
            token_quote_account: pubkeys[3],
            quote_vault: pubkeys[4],
            quote_mint: pubkeys[5],
            fee_claimer: pubkeys[6],
            token_quote_program: pubkeys[7],
            event_authority: pubkeys[8],
            program: pubkeys[9],
        }
    }
}

impl<'info> From<PartnerWithdrawSurplusAccounts<'_, 'info>>
    for [AccountInfo<'info>; PARTNER_WITHDRAW_SURPLUS_IX_ACCOUNTS_LEN]
{
    fn from(accounts: PartnerWithdrawSurplusAccounts<'_, 'info>) -> Self {
        [
            accounts.pool_authority.clone(),
            accounts.config.clone(),
            accounts.virtual_pool.clone(),
            accounts.token_quote_account.clone(),
            accounts.quote_vault.clone(),
            accounts.quote_mint.clone(),
            accounts.fee_claimer.clone(),
            accounts.token_quote_program.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}

impl<'me, 'info>
    From<&'me [AccountInfo<'info>; PARTNER_WITHDRAW_SURPLUS_IX_ACCOUNTS_LEN]>
    for PartnerWithdrawSurplusAccounts<'me, 'info>
{
    fn from(arr: &'me [AccountInfo<'info>; PARTNER_WITHDRAW_SURPLUS_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            pool_authority: &arr[0],
            config: &arr[1],
            virtual_pool: &arr[2],
            token_quote_account: &arr[3],
            quote_vault: &arr[4],
            quote_mint: &arr[5],
            fee_claimer: &arr[6],
            token_quote_program: &arr[7],
            event_authority: &arr[8],
            program: &arr[9],
        }
    }
}

pub const PARTNER_WITHDRAW_SURPLUS_IX_DISCM: [u8; 8] = [168, 173, 72, 100, 201, 98, 38, 92];

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct PartnerWithdrawSurplusIxArgs {}

#[derive(Clone, Debug, PartialEq)]
pub struct PartnerWithdrawSurplusIxData(pub PartnerWithdrawSurplusIxArgs);

impl From<PartnerWithdrawSurplusIxArgs> for PartnerWithdrawSurplusIxData {
    fn from(args: PartnerWithdrawSurplusIxArgs) -> Self {
        Self(args)
    }
}

impl PartnerWithdrawSurplusIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != PARTNER_WITHDRAW_SURPLUS_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    PARTNER_WITHDRAW_SURPLUS_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(PartnerWithdrawSurplusIxArgs::deserialize(&mut reader)?))
    }
    
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&PARTNER_WITHDRAW_SURPLUS_IX_DISCM)?;
        self.0.serialize(&mut writer)
    }
    
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}

pub fn partner_withdraw_surplus_ix_with_program_id(
    program_id: Pubkey,
    keys: PartnerWithdrawSurplusKeys,
    args: PartnerWithdrawSurplusIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; PARTNER_WITHDRAW_SURPLUS_IX_ACCOUNTS_LEN] = keys.into();
    let data: PartnerWithdrawSurplusIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}

pub fn partner_withdraw_surplus_ix(
    keys: PartnerWithdrawSurplusKeys,
    args: PartnerWithdrawSurplusIxArgs,
) -> std::io::Result<Instruction> {
    partner_withdraw_surplus_ix_with_program_id(crate::ID, keys, args)
}

pub fn partner_withdraw_surplus_invoke_with_program_id(
    program_id: Pubkey,
    accounts: PartnerWithdrawSurplusAccounts<'_, '_>,
    args: PartnerWithdrawSurplusIxArgs,
) -> ProgramResult {
    let keys: PartnerWithdrawSurplusKeys = accounts.into();
    let ix = partner_withdraw_surplus_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}

pub fn partner_withdraw_surplus_invoke(
    accounts: PartnerWithdrawSurplusAccounts<'_, '_>,
    args: PartnerWithdrawSurplusIxArgs,
) -> ProgramResult {
    partner_withdraw_surplus_invoke_with_program_id(crate::ID, accounts, args)
}

pub fn partner_withdraw_surplus_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: PartnerWithdrawSurplusAccounts<'_, '_>,
    args: PartnerWithdrawSurplusIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: PartnerWithdrawSurplusKeys = accounts.into();
    let ix = partner_withdraw_surplus_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}

pub fn partner_withdraw_surplus_invoke_signed(
    accounts: PartnerWithdrawSurplusAccounts<'_, '_>,
    args: PartnerWithdrawSurplusIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    partner_withdraw_surplus_invoke_signed_with_program_id(crate::ID, accounts, args, seeds)
}

pub fn partner_withdraw_surplus_verify_account_keys(
    accounts: PartnerWithdrawSurplusAccounts<'_, '_>,
    keys: PartnerWithdrawSurplusKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.pool_authority.key, keys.pool_authority),
        (*accounts.config.key, keys.config),
        (*accounts.virtual_pool.key, keys.virtual_pool),
        (*accounts.token_quote_account.key, keys.token_quote_account),
        (*accounts.quote_vault.key, keys.quote_vault),
        (*accounts.quote_mint.key, keys.quote_mint),
        (*accounts.fee_claimer.key, keys.fee_claimer),
        (*accounts.token_quote_program.key, keys.token_quote_program),
        (*accounts.event_authority.key, keys.event_authority),
        (*accounts.program.key, keys.program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}

pub fn partner_withdraw_surplus_verify_is_writable_privileges<'me, 'info>(
    accounts: PartnerWithdrawSurplusAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for acct in [
        accounts.virtual_pool,
        accounts.token_quote_account,
        accounts.quote_vault,
    ] {
        if !acct.is_writable {
            return Err((acct, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}

pub fn partner_withdraw_surplus_verify_is_signer_privileges<'me, 'info>(
    accounts: PartnerWithdrawSurplusAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    if !accounts.fee_claimer.is_signer {
        return Err((accounts.fee_claimer, ProgramError::MissingRequiredSignature));
    }
    Ok(())
}

pub fn partner_withdraw_surplus_verify_account_privileges<'me, 'info>(
    accounts: PartnerWithdrawSurplusAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    partner_withdraw_surplus_verify_is_writable_privileges(accounts)?;
    partner_withdraw_surplus_verify_is_signer_privileges(accounts)?;
    Ok(())
}

pub const PROTOCOL_WITHDRAW_SURPLUS_IX_ACCOUNTS_LEN: usize = 8;
#[derive(Copy, Clone, Debug)]
pub struct ProtocolWithdrawSurplusAccounts<'me, 'info> {
    pub pool_authority: &'me AccountInfo<'info>,
    pub config: &'me AccountInfo<'info>,
    pub virtual_pool: &'me AccountInfo<'info>,
    pub token_quote_account: &'me AccountInfo<'info>,
    pub quote_vault: &'me AccountInfo<'info>,
    pub quote_mint: &'me AccountInfo<'info>,
    pub token_quote_program: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct ProtocolWithdrawSurplusKeys {
    pub pool_authority: Pubkey,
    pub config: Pubkey,
    pub virtual_pool: Pubkey,
    pub token_quote_account: Pubkey,
    pub quote_vault: Pubkey,
    pub quote_mint: Pubkey,
    pub token_quote_program: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}

impl From<ProtocolWithdrawSurplusAccounts<'_, '_>> for ProtocolWithdrawSurplusKeys {
    fn from(accounts: ProtocolWithdrawSurplusAccounts) -> Self {
        Self {
            pool_authority: *accounts.pool_authority.key,
            config: *accounts.config.key,
            virtual_pool: *accounts.virtual_pool.key,
            token_quote_account: *accounts.token_quote_account.key,
            quote_vault: *accounts.quote_vault.key,
            quote_mint: *accounts.quote_mint.key,
            token_quote_program: *accounts.token_quote_program.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}

impl From<ProtocolWithdrawSurplusKeys> for [AccountMeta; PROTOCOL_WITHDRAW_SURPLUS_IX_ACCOUNTS_LEN] {
    fn from(keys: ProtocolWithdrawSurplusKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.pool_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.virtual_pool,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_quote_account,
                is_signer: false,
                is_writable: true,
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
                pubkey: keys.token_quote_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.event_authority,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}

impl From<[Pubkey; PROTOCOL_WITHDRAW_SURPLUS_IX_ACCOUNTS_LEN]> for ProtocolWithdrawSurplusKeys {
    fn from(pubkeys: [Pubkey; PROTOCOL_WITHDRAW_SURPLUS_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            pool_authority: pubkeys[0],
            config: pubkeys[1],
            virtual_pool: pubkeys[2],
            token_quote_account: pubkeys[3],
            quote_vault: pubkeys[4],
            quote_mint: pubkeys[5],
            token_quote_program: pubkeys[6],
            event_authority: pubkeys[7],
            program: Pubkey::default(), // Note: program is not in the account metas array
        }
    }
}

impl<'info> From<ProtocolWithdrawSurplusAccounts<'_, 'info>>
    for [AccountInfo<'info>; PROTOCOL_WITHDRAW_SURPLUS_IX_ACCOUNTS_LEN]
{
    fn from(accounts: ProtocolWithdrawSurplusAccounts<'_, 'info>) -> Self {
        [
            accounts.pool_authority.clone(),
            accounts.config.clone(),
            accounts.virtual_pool.clone(),
            accounts.token_quote_account.clone(),
            accounts.quote_vault.clone(),
            accounts.quote_mint.clone(),
            accounts.token_quote_program.clone(),
            accounts.event_authority.clone(),
        ]
    }
}

impl<'me, 'info>
    From<&'me [AccountInfo<'info>; PROTOCOL_WITHDRAW_SURPLUS_IX_ACCOUNTS_LEN]>
    for ProtocolWithdrawSurplusAccounts<'me, 'info>
{
    fn from(arr: &'me [AccountInfo<'info>; PROTOCOL_WITHDRAW_SURPLUS_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            pool_authority: &arr[0],
            config: &arr[1],
            virtual_pool: &arr[2],
            token_quote_account: &arr[3],
            quote_vault: &arr[4],
            quote_mint: &arr[5],
            token_quote_program: &arr[6],
            event_authority: &arr[7],
            program: &arr[7], // Using event_authority as placeholder, needs proper handling
        }
    }
}

pub const PROTOCOL_WITHDRAW_SURPLUS_IX_DISCM: [u8; 8] = [54, 136, 225, 138, 172, 182, 214, 167];

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ProtocolWithdrawSurplusIxArgs {}

#[derive(Clone, Debug, PartialEq)]
pub struct ProtocolWithdrawSurplusIxData(pub ProtocolWithdrawSurplusIxArgs);

impl From<ProtocolWithdrawSurplusIxArgs> for ProtocolWithdrawSurplusIxData {
    fn from(args: ProtocolWithdrawSurplusIxArgs) -> Self {
        Self(args)
    }
}

impl ProtocolWithdrawSurplusIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != PROTOCOL_WITHDRAW_SURPLUS_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    PROTOCOL_WITHDRAW_SURPLUS_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(ProtocolWithdrawSurplusIxArgs::deserialize(&mut reader)?))
    }

    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&PROTOCOL_WITHDRAW_SURPLUS_IX_DISCM)?;
        self.0.serialize(&mut writer)
    }

    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}

pub fn protocol_withdraw_surplus_ix_with_program_id(
    program_id: Pubkey,
    keys: ProtocolWithdrawSurplusKeys,
    args: ProtocolWithdrawSurplusIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; PROTOCOL_WITHDRAW_SURPLUS_IX_ACCOUNTS_LEN] = keys.into();
    let data: ProtocolWithdrawSurplusIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}

pub fn protocol_withdraw_surplus_ix(
    keys: ProtocolWithdrawSurplusKeys,
    args: ProtocolWithdrawSurplusIxArgs,
) -> std::io::Result<Instruction> {
    protocol_withdraw_surplus_ix_with_program_id(crate::ID, keys, args)
}

pub fn protocol_withdraw_surplus_invoke_with_program_id(
    program_id: Pubkey,
    accounts: ProtocolWithdrawSurplusAccounts<'_, '_>,
    args: ProtocolWithdrawSurplusIxArgs,
) -> ProgramResult {
    let keys: ProtocolWithdrawSurplusKeys = accounts.into();
    let ix = protocol_withdraw_surplus_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}

pub fn protocol_withdraw_surplus_invoke(
    accounts: ProtocolWithdrawSurplusAccounts<'_, '_>,
    args: ProtocolWithdrawSurplusIxArgs,
) -> ProgramResult {
    protocol_withdraw_surplus_invoke_with_program_id(crate::ID, accounts, args)
}

pub fn protocol_withdraw_surplus_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: ProtocolWithdrawSurplusAccounts<'_, '_>,
    args: ProtocolWithdrawSurplusIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: ProtocolWithdrawSurplusKeys = accounts.into();
    let ix = protocol_withdraw_surplus_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}

pub fn protocol_withdraw_surplus_invoke_signed(
    accounts: ProtocolWithdrawSurplusAccounts<'_, '_>,
    args: ProtocolWithdrawSurplusIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    protocol_withdraw_surplus_invoke_signed_with_program_id(crate::ID, accounts, args, seeds)
}

pub fn protocol_withdraw_surplus_verify_account_keys(
    accounts: ProtocolWithdrawSurplusAccounts<'_, '_>,
    keys: ProtocolWithdrawSurplusKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.pool_authority.key, keys.pool_authority),
        (*accounts.config.key, keys.config),
        (*accounts.virtual_pool.key, keys.virtual_pool),
        (*accounts.token_quote_account.key, keys.token_quote_account),
        (*accounts.quote_vault.key, keys.quote_vault),
        (*accounts.quote_mint.key, keys.quote_mint),
        (*accounts.token_quote_program.key, keys.token_quote_program),
        (*accounts.event_authority.key, keys.event_authority),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}

pub fn protocol_withdraw_surplus_verify_is_writable_privileges<'me, 'info>(
    accounts: ProtocolWithdrawSurplusAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for acct in [
        accounts.virtual_pool,
        accounts.token_quote_account,
        accounts.quote_vault,
    ] {
        if !acct.is_writable {
            return Err((acct, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}

pub fn protocol_withdraw_surplus_verify_is_signer_privileges<'me, 'info>(
    _accounts: ProtocolWithdrawSurplusAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    Ok(())
}

pub fn protocol_withdraw_surplus_verify_account_privileges<'me, 'info>(
    accounts: ProtocolWithdrawSurplusAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    protocol_withdraw_surplus_verify_is_writable_privileges(accounts)?;
    protocol_withdraw_surplus_verify_is_signer_privileges(accounts)?;
    Ok(())
}

pub const SWAP_IX_ACCOUNTS_LEN: usize = 15;
#[derive(Copy, Clone, Debug)]
pub struct SwapAccounts<'me, 'info> {
    pub pool_authority: &'me AccountInfo<'info>,
    pub config: &'me AccountInfo<'info>,
    pub pool: &'me AccountInfo<'info>,
    pub input_token_account: &'me AccountInfo<'info>,
    pub output_token_account: &'me AccountInfo<'info>,
    pub base_vault: &'me AccountInfo<'info>,
    pub quote_vault: &'me AccountInfo<'info>,
    pub base_mint: &'me AccountInfo<'info>,
    pub quote_mint: &'me AccountInfo<'info>,
    pub payer: &'me AccountInfo<'info>,
    pub token_base_program: &'me AccountInfo<'info>,
    pub token_quote_program: &'me AccountInfo<'info>,
    pub referral_token_account: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SwapKeys {
    pub pool_authority: Pubkey,
    pub config: Pubkey,
    pub pool: Pubkey,
    pub input_token_account: Pubkey,
    pub output_token_account: Pubkey,
    pub base_vault: Pubkey,
    pub quote_vault: Pubkey,
    pub base_mint: Pubkey,
    pub quote_mint: Pubkey,
    pub payer: Pubkey,
    pub token_base_program: Pubkey,
    pub token_quote_program: Pubkey,
    pub referral_token_account: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}

impl From<SwapAccounts<'_, '_>> for SwapKeys {
    fn from(accounts: SwapAccounts) -> Self {
        Self {
            pool_authority: *accounts.pool_authority.key,
            config: *accounts.config.key,
            pool: *accounts.pool.key,
            input_token_account: *accounts.input_token_account.key,
            output_token_account: *accounts.output_token_account.key,
            base_vault: *accounts.base_vault.key,
            quote_vault: *accounts.quote_vault.key,
            base_mint: *accounts.base_mint.key,
            quote_mint: *accounts.quote_mint.key,
            payer: *accounts.payer.key,
            token_base_program: *accounts.token_base_program.key,
            token_quote_program: *accounts.token_quote_program.key,
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
                pubkey: keys.config,
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
                pubkey: keys.payer,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_base_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_quote_program,
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
            config: pubkeys[1],
            pool: pubkeys[2],
            input_token_account: pubkeys[3],
            output_token_account: pubkeys[4],
            base_vault: pubkeys[5],
            quote_vault: pubkeys[6],
            base_mint: pubkeys[7],
            quote_mint: pubkeys[8],
            payer: pubkeys[9],
            token_base_program: pubkeys[10],
            token_quote_program: pubkeys[11],
            referral_token_account: pubkeys[12],
            event_authority: pubkeys[13],
            program: pubkeys[14],
        }
    }
}

impl<'info> From<SwapAccounts<'_, 'info>> for [AccountInfo<'info>; SWAP_IX_ACCOUNTS_LEN] {
    fn from(accounts: SwapAccounts<'_, 'info>) -> Self {
        [
            accounts.pool_authority.clone(),
            accounts.config.clone(),
            accounts.pool.clone(),
            accounts.input_token_account.clone(),
            accounts.output_token_account.clone(),
            accounts.base_vault.clone(),
            accounts.quote_vault.clone(),
            accounts.base_mint.clone(),
            accounts.quote_mint.clone(),
            accounts.payer.clone(),
            accounts.token_base_program.clone(),
            accounts.token_quote_program.clone(),
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
            config: &arr[1],
            pool: &arr[2],
            input_token_account: &arr[3],
            output_token_account: &arr[4],
            base_vault: &arr[5],
            quote_vault: &arr[6],
            base_mint: &arr[7],
            quote_mint: &arr[8],
            payer: &arr[9],
            token_base_program: &arr[10],
            token_quote_program: &arr[11],
            referral_token_account: &arr[12],
            event_authority: &arr[13],
            program: &arr[14],
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
            return Err(
                std::io::Error::new(
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

pub fn swap_ix(
    keys: SwapKeys,
    args: SwapIxArgs,
) -> std::io::Result<Instruction> {
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

pub fn swap_invoke(
    accounts: SwapAccounts<'_, '_>,
    args: SwapIxArgs,
) -> ProgramResult {
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
        (*accounts.config.key, keys.config),
        (*accounts.pool.key, keys.pool),
        (*accounts.input_token_account.key, keys.input_token_account),
        (*accounts.output_token_account.key, keys.output_token_account),
        (*accounts.base_vault.key, keys.base_vault),
        (*accounts.quote_vault.key, keys.quote_vault),
        (*accounts.base_mint.key, keys.base_mint),
        (*accounts.quote_mint.key, keys.quote_mint),
        (*accounts.payer.key, keys.payer),
        (*accounts.token_base_program.key, keys.token_base_program),
        (*accounts.token_quote_program.key, keys.token_quote_program),
        (*accounts.referral_token_account.key, keys.referral_token_account),
        (*accounts.event_authority.key, keys.event_authority),
        (*accounts.program.key, keys.program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}

pub fn swap_verify_is_writable_privileges<'me, 'info>(
    accounts: SwapAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for acct in [
        accounts.pool,
        accounts.input_token_account,
        accounts.output_token_account,
        accounts.base_vault,
        accounts.quote_vault,
    ] {
        if !acct.is_writable {
            return Err((acct, ProgramError::InvalidAccountData));
        }
    }
    
    Ok(())
}

pub fn swap_verify_is_signer_privileges<'me, 'info>(
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
    swap_verify_is_writable_privileges(accounts)?;
    swap_verify_is_signer_privileges(accounts)?;
    Ok(())
}

pub const SWAP2_IX_ACCOUNTS_LEN: usize = 15;
#[derive(Copy, Clone, Debug)]
pub struct Swap2Accounts<'me, 'info> {
    pub pool_authority: &'me AccountInfo<'info>,
    pub config: &'me AccountInfo<'info>,
    pub pool: &'me AccountInfo<'info>,
    pub input_token_account: &'me AccountInfo<'info>,
    pub output_token_account: &'me AccountInfo<'info>,
    pub base_vault: &'me AccountInfo<'info>,
    pub quote_vault: &'me AccountInfo<'info>,
    pub base_mint: &'me AccountInfo<'info>,
    pub quote_mint: &'me AccountInfo<'info>,
    pub payer: &'me AccountInfo<'info>,
    pub token_base_program: &'me AccountInfo<'info>,
    pub token_quote_program: &'me AccountInfo<'info>,
    pub referral_token_account:&'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Swap2Keys {
    pub pool_authority: Pubkey,
    pub config: Pubkey,
    pub pool: Pubkey,
    pub input_token_account: Pubkey,
    pub output_token_account: Pubkey,
    pub base_vault: Pubkey,
    pub quote_vault: Pubkey,
    pub base_mint: Pubkey,
    pub quote_mint: Pubkey,
    pub payer: Pubkey,
    pub token_base_program: Pubkey,
    pub token_quote_program: Pubkey,
    pub referral_token_account: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}

impl From<Swap2Accounts<'_, '_>> for Swap2Keys {
    fn from(accounts: Swap2Accounts) -> Self {
        Self {
            pool_authority: *accounts.pool_authority.key,
            config: *accounts.config.key,
            pool: *accounts.pool.key,
            input_token_account: *accounts.input_token_account.key,
            output_token_account: *accounts.output_token_account.key,
            base_vault: *accounts.base_vault.key,
            quote_vault: *accounts.quote_vault.key,
            base_mint: *accounts.base_mint.key,
            quote_mint: *accounts.quote_mint.key,
            payer: *accounts.payer.key,
            token_base_program: *accounts.token_base_program.key,
            token_quote_program: *accounts.token_quote_program.key,
            referral_token_account: *accounts.referral_token_account.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}

impl From<Swap2Keys> for [AccountMeta; SWAP2_IX_ACCOUNTS_LEN] {
    fn from(keys: Swap2Keys) -> Self {
        let mut metas = vec![
            AccountMeta { pubkey: keys.pool_authority, is_signer: false, is_writable: false },
            AccountMeta { pubkey: keys.config, is_signer: false, is_writable: false },
            AccountMeta { pubkey: keys.pool, is_signer: false, is_writable: true },
            AccountMeta { pubkey: keys.input_token_account, is_signer: false, is_writable: true },
            AccountMeta { pubkey: keys.output_token_account, is_signer: false, is_writable: true },
            AccountMeta { pubkey: keys.base_vault, is_signer: false, is_writable: true },
            AccountMeta { pubkey: keys.quote_vault, is_signer: false, is_writable: true },
            AccountMeta { pubkey: keys.base_mint, is_signer: false, is_writable: false },
            AccountMeta { pubkey: keys.quote_mint, is_signer: false, is_writable: false },
            AccountMeta { pubkey: keys.payer, is_signer: true, is_writable: false },
            AccountMeta { pubkey: keys.token_base_program, is_signer: false, is_writable: false },
            AccountMeta { pubkey: keys.token_quote_program, is_signer: false, is_writable: false },
            AccountMeta { 
                pubkey: keys.referral_token_account, 
                is_signer: false, 
                is_writable: true 
            },
            AccountMeta { pubkey: keys.event_authority, is_signer: false, is_writable: false },
            AccountMeta { pubkey: keys.program, is_signer: false, is_writable: false },
        ];
        metas.try_into().unwrap()
    }
}

impl From<[Pubkey; SWAP2_IX_ACCOUNTS_LEN]> for Swap2Keys {
    fn from(pubkeys: [Pubkey; SWAP2_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            pool_authority: pubkeys[0],
            config: pubkeys[1],
            pool: pubkeys[2],
            input_token_account: pubkeys[3],
            output_token_account: pubkeys[4],
            base_vault: pubkeys[5],
            quote_vault: pubkeys[6],
            base_mint: pubkeys[7],
            quote_mint: pubkeys[8],
            payer: pubkeys[9],
            token_base_program: pubkeys[10],
            token_quote_program: pubkeys[11],
            referral_token_account: pubkeys[12],
            event_authority: pubkeys[13],
            program: pubkeys[14],
        }
    }
}

impl<'info> From<Swap2Accounts<'_, 'info>> for [AccountInfo<'info>; SWAP2_IX_ACCOUNTS_LEN] {
    fn from(accounts: Swap2Accounts<'_, 'info>) -> Self {
        let mut arr = vec![
            accounts.pool_authority.clone(),
            accounts.config.clone(),
            accounts.pool.clone(),
            accounts.input_token_account.clone(),
            accounts.output_token_account.clone(),
            accounts.base_vault.clone(),
            accounts.quote_vault.clone(),
            accounts.base_mint.clone(),
            accounts.quote_mint.clone(),
            accounts.payer.clone(),
            accounts.token_base_program.clone(),
            accounts.token_quote_program.clone(),
            accounts.referral_token_account.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ];
        arr.try_into().unwrap()
    }
}

impl<'me, 'info> From<&'me [AccountInfo<'info>; SWAP2_IX_ACCOUNTS_LEN]> for Swap2Accounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; SWAP2_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            pool_authority: &arr[0],
            config: &arr[1],
            pool: &arr[2],
            input_token_account: &arr[3],
            output_token_account: &arr[4],
            base_vault: &arr[5],
            quote_vault: &arr[6],
            base_mint: &arr[7],
            quote_mint: &arr[8],
            payer: &arr[9],
            token_base_program: &arr[10],
            token_quote_program: &arr[11],
            referral_token_account: &arr[12],
            event_authority: &arr[13],
            program: &arr[14],
        }
    }
}

pub const SWAP2_IX_DISCM: [u8; 8] = [65, 75, 63, 76, 235, 91, 91, 136];

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Swap2IxArgs {
    params: SwapParameters2,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Swap2IxData(pub Swap2IxArgs);

impl From<Swap2IxArgs> for Swap2IxData {
    fn from(args: Swap2IxArgs) -> Self {
        Self(args)
    }
}

impl Swap2IxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != SWAP2_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    SWAP2_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(Swap2IxArgs::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&SWAP2_IX_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}

pub fn swap2_ix_with_program_id(
    program_id: Pubkey,
    keys: Swap2Keys,
    args: Swap2IxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; SWAP2_IX_ACCOUNTS_LEN] = keys.into();
    let data: Swap2IxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}

pub fn swap2_ix(
    keys: Swap2Keys,
    args: Swap2IxArgs,
) -> std::io::Result<Instruction> {
    swap2_ix_with_program_id(crate::ID, keys, args)
}

pub fn swap2_invoke_with_program_id(
    program_id: Pubkey,
    accounts: Swap2Accounts<'_, '_>,
    args: Swap2IxArgs,
) -> ProgramResult {
    let keys: Swap2Keys = accounts.into();
    let ix = swap2_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}

pub fn swap2_invoke(
    accounts: Swap2Accounts<'_, '_>,
    args: Swap2IxArgs,
) -> ProgramResult {
    swap2_invoke_with_program_id(crate::ID, accounts, args)
}

pub fn swap2_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: Swap2Accounts<'_, '_>,
    args: Swap2IxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: Swap2Keys = accounts.into();
    let ix = swap2_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}

pub fn swap2_invoke_signed(
    accounts: Swap2Accounts<'_, '_>,
    args: Swap2IxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    swap2_invoke_signed_with_program_id(crate::ID, accounts, args, seeds)
}

pub fn swap2_verify_account_keys(
    accounts: Swap2Accounts<'_, '_>,
    keys: Swap2Keys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.pool_authority.key, keys.pool_authority),
        (*accounts.config.key, keys.config),
        (*accounts.pool.key, keys.pool),
        (*accounts.input_token_account.key, keys.input_token_account),
        (*accounts.output_token_account.key, keys.output_token_account),
        (*accounts.base_vault.key, keys.base_vault),
        (*accounts.quote_vault.key, keys.quote_vault),
        (*accounts.base_mint.key, keys.base_mint),
        (*accounts.quote_mint.key, keys.quote_mint),
        (*accounts.payer.key, keys.payer),
        (*accounts.token_base_program.key, keys.token_base_program),
        (*accounts.token_quote_program.key, keys.token_quote_program),
        (*accounts.referral_token_account.key, keys.referral_token_account),
        (*accounts.event_authority.key, keys.event_authority),
        (*accounts.program.key, keys.program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}

pub fn swap2_verify_is_writable_privileges<'me, 'info>(
    accounts: Swap2Accounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for acct in [
        accounts.pool,
        accounts.input_token_account,
        accounts.output_token_account,
        accounts.base_vault,
        accounts.quote_vault,
    ] {
        if !acct.is_writable {
            return Err((acct, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}

pub fn swap2_verify_is_signer_privileges<'me, 'info>(
    accounts: Swap2Accounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    if !accounts.payer.is_signer {
        return Err((accounts.payer, ProgramError::MissingRequiredSignature));
    }
    Ok(())
}

pub fn swap2_verify_account_privileges<'me, 'info>(
    accounts: Swap2Accounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    swap2_verify_is_writable_privileges(accounts)?;
    swap2_verify_is_signer_privileges(accounts)?;
    Ok(())
}
pub const TRANSFER_POOL_CREATOR_IX_ACCOUNTS_LEN: usize = 6;

#[derive(Copy, Clone, Debug)]
pub struct TransferPoolCreatorAccounts<'me, 'info> {
    pub virtual_pool: &'me AccountInfo<'info>,
    pub config: &'me AccountInfo<'info>,
    pub creator: &'me AccountInfo<'info>,
    pub new_creator: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct TransferPoolCreatorKeys {
    pub virtual_pool: Pubkey,
    pub config: Pubkey,
    pub creator: Pubkey,
    pub new_creator: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}

impl From<TransferPoolCreatorAccounts<'_, '_>> for TransferPoolCreatorKeys {
    fn from(accounts: TransferPoolCreatorAccounts) -> Self {
        Self {
            virtual_pool: *accounts.virtual_pool.key,
            config: *accounts.config.key,
            creator: *accounts.creator.key,
            new_creator: *accounts.new_creator.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}

impl From<TransferPoolCreatorKeys> for [AccountMeta; TRANSFER_POOL_CREATOR_IX_ACCOUNTS_LEN] {
    fn from(keys: TransferPoolCreatorKeys) -> Self {
        [
            AccountMeta { pubkey: keys.virtual_pool, is_signer: false, is_writable: true },
            AccountMeta { pubkey: keys.config, is_signer: false, is_writable: false },
            AccountMeta { pubkey: keys.creator, is_signer: true, is_writable: false },
            AccountMeta { pubkey: keys.new_creator, is_signer: false, is_writable: false },
            AccountMeta { pubkey: keys.event_authority, is_signer: false, is_writable: false },
            AccountMeta { pubkey: keys.program, is_signer: false, is_writable: false },
        ]
    }
}

impl From<[Pubkey; TRANSFER_POOL_CREATOR_IX_ACCOUNTS_LEN]> for TransferPoolCreatorKeys {
    fn from(pubkeys: [Pubkey; TRANSFER_POOL_CREATOR_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            virtual_pool: pubkeys[0],
            config: pubkeys[1],
            creator: pubkeys[2],
            new_creator: pubkeys[3],
            event_authority: pubkeys[4],
            program: pubkeys[5],
        }
    }
}

impl<'info> From<TransferPoolCreatorAccounts<'_, 'info>> for [AccountInfo<'info>; TRANSFER_POOL_CREATOR_IX_ACCOUNTS_LEN] {
    fn from(accounts: TransferPoolCreatorAccounts<'_, 'info>) -> Self {
        [
            accounts.virtual_pool.clone(),
            accounts.config.clone(),
            accounts.creator.clone(),
            accounts.new_creator.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}

impl<'me, 'info> From<&'me [AccountInfo<'info>; TRANSFER_POOL_CREATOR_IX_ACCOUNTS_LEN]> for TransferPoolCreatorAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; TRANSFER_POOL_CREATOR_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            virtual_pool: &arr[0],
            config: &arr[1],
            creator: &arr[2],
            new_creator: &arr[3],
            event_authority: &arr[4],
            program: &arr[5],
        }
    }
}

pub const TRANSFER_POOL_CREATOR_IX_DISCM: [u8; 8] = [20, 7, 169, 33, 58, 147, 166, 33];

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct TransferPoolCreatorIxArgs {}

#[derive(Clone, Debug, PartialEq)]
pub struct TransferPoolCreatorIxData(pub TransferPoolCreatorIxArgs);
impl From<TransferPoolCreatorIxArgs> for TransferPoolCreatorIxData {
    fn from(args: TransferPoolCreatorIxArgs) -> Self {
        Self(args)
    }
}
impl TransferPoolCreatorIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != TRANSFER_POOL_CREATOR_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    TRANSFER_POOL_CREATOR_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(TransferPoolCreatorIxArgs::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&TRANSFER_POOL_CREATOR_IX_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}

pub fn transfer_pool_creator_ix_with_program_id(
    program_id: Pubkey,
    keys: TransferPoolCreatorKeys,
    args: TransferPoolCreatorIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; TRANSFER_POOL_CREATOR_IX_ACCOUNTS_LEN] = keys.into();
    let data: TransferPoolCreatorIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}

pub fn transfer_pool_creator_ix(
    keys: TransferPoolCreatorKeys,
    args: TransferPoolCreatorIxArgs,
) -> std::io::Result<Instruction> {
    transfer_pool_creator_ix_with_program_id(crate::ID, keys, args)
}

pub fn transfer_pool_creator_invoke_with_program_id(
    program_id: Pubkey,
    accounts: TransferPoolCreatorAccounts<'_, '_>,
    args: TransferPoolCreatorIxArgs,
) -> ProgramResult {
    let keys: TransferPoolCreatorKeys = accounts.into();
    let ix = transfer_pool_creator_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}

pub fn transfer_pool_creator_invoke(
    accounts: TransferPoolCreatorAccounts<'_, '_>,
    args: TransferPoolCreatorIxArgs,
) -> ProgramResult {
    transfer_pool_creator_invoke_with_program_id(crate::ID, accounts, args)
}

pub fn transfer_pool_creator_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: TransferPoolCreatorAccounts<'_, '_>,
    args: TransferPoolCreatorIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: TransferPoolCreatorKeys = accounts.into();
    let ix = transfer_pool_creator_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}

pub fn transfer_pool_creator_invoke_signed(
    accounts: TransferPoolCreatorAccounts<'_, '_>,
    args: TransferPoolCreatorIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    transfer_pool_creator_invoke_signed_with_program_id(crate::ID, accounts, args, seeds)
}

pub fn transfer_pool_creator_verify_account_keys(
    accounts: TransferPoolCreatorAccounts<'_, '_>,
    keys: TransferPoolCreatorKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.virtual_pool.key, keys.virtual_pool),
        (*accounts.config.key, keys.config),
        (*accounts.creator.key, keys.creator),
        (*accounts.new_creator.key, keys.new_creator),
        (*accounts.event_authority.key, keys.event_authority),
        (*accounts.program.key, keys.program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}

pub fn transfer_pool_creator_verify_is_writable_privileges<'me, 'info>(
    accounts: TransferPoolCreatorAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    if !accounts.virtual_pool.is_writable {
        return Err((accounts.virtual_pool, ProgramError::InvalidAccountData));
    }
    Ok(())
}

pub fn transfer_pool_creator_verify_is_signer_privileges<'me, 'info>(
    accounts: TransferPoolCreatorAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    if !accounts.creator.is_signer {
        return Err((accounts.creator, ProgramError::MissingRequiredSignature));
    }
    Ok(())
}

pub fn transfer_pool_creator_verify_account_privileges<'me, 'info>(
    accounts: TransferPoolCreatorAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    transfer_pool_creator_verify_is_writable_privileges(accounts)?;
    transfer_pool_creator_verify_is_signer_privileges(accounts)?;
    Ok(())
}
pub const WITHDRAW_LEFTOVER_IX_ACCOUNTS_LEN: usize = 10;

#[derive(Copy, Clone, Debug)]
pub struct WithdrawLeftoverAccounts<'me, 'info> {
    pub pool_authority: &'me AccountInfo<'info>,
    pub config: &'me AccountInfo<'info>,
    pub virtual_pool: &'me AccountInfo<'info>,
    pub token_base_account: &'me AccountInfo<'info>,
    pub base_vault: &'me AccountInfo<'info>,
    pub base_mint: &'me AccountInfo<'info>,
    pub leftover_receiver: &'me AccountInfo<'info>,
    pub token_base_program: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct WithdrawLeftoverKeys {
    pub pool_authority: Pubkey,
    pub config: Pubkey,
    pub virtual_pool: Pubkey,
    pub token_base_account: Pubkey,
    pub base_vault: Pubkey,
    pub base_mint: Pubkey,
    pub leftover_receiver: Pubkey,
    pub token_base_program: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}

impl From<WithdrawLeftoverAccounts<'_, '_>> for WithdrawLeftoverKeys {
    fn from(accounts: WithdrawLeftoverAccounts) -> Self {
        Self {
            pool_authority: *accounts.pool_authority.key,
            config: *accounts.config.key,
            virtual_pool: *accounts.virtual_pool.key,
            token_base_account: *accounts.token_base_account.key,
            base_vault: *accounts.base_vault.key,
            base_mint: *accounts.base_mint.key,
            leftover_receiver: *accounts.leftover_receiver.key,
            token_base_program: *accounts.token_base_program.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}

impl From<WithdrawLeftoverKeys> for [AccountMeta; WITHDRAW_LEFTOVER_IX_ACCOUNTS_LEN] {
    fn from(keys: WithdrawLeftoverKeys) -> Self {
        [
            AccountMeta { pubkey: keys.pool_authority, is_signer: false, is_writable: false },
            AccountMeta { pubkey: keys.config, is_signer: false, is_writable: false },
            AccountMeta { pubkey: keys.virtual_pool, is_signer: false, is_writable: true },
            AccountMeta { pubkey: keys.token_base_account, is_signer: false, is_writable: true },
            AccountMeta { pubkey: keys.base_vault, is_signer: false, is_writable: true },
            AccountMeta { pubkey: keys.base_mint, is_signer: false, is_writable: false },
            AccountMeta { pubkey: keys.leftover_receiver, is_signer: false, is_writable: false },
            AccountMeta { pubkey: keys.token_base_program, is_signer: false, is_writable: false },
            AccountMeta { pubkey: keys.event_authority, is_signer: false, is_writable: false },
            AccountMeta { pubkey: keys.program, is_signer: false, is_writable: false },
        ]
    }
}

impl From<[Pubkey; WITHDRAW_LEFTOVER_IX_ACCOUNTS_LEN]> for WithdrawLeftoverKeys {
    fn from(pubkeys: [Pubkey; WITHDRAW_LEFTOVER_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            pool_authority: pubkeys[0],
            config: pubkeys[1],
            virtual_pool: pubkeys[2],
            token_base_account: pubkeys[3],
            base_vault: pubkeys[4],
            base_mint: pubkeys[5],
            leftover_receiver: pubkeys[6],
            token_base_program: pubkeys[7],
            event_authority: pubkeys[8],
            program: pubkeys[9],
        }
    }
}

impl<'info> From<WithdrawLeftoverAccounts<'_, 'info>> for [AccountInfo<'info>; WITHDRAW_LEFTOVER_IX_ACCOUNTS_LEN] {
    fn from(accounts: WithdrawLeftoverAccounts<'_, 'info>) -> Self {
        [
            accounts.pool_authority.clone(),
            accounts.config.clone(),
            accounts.virtual_pool.clone(),
            accounts.token_base_account.clone(),
            accounts.base_vault.clone(),
            accounts.base_mint.clone(),
            accounts.leftover_receiver.clone(),
            accounts.token_base_program.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}

impl<'me, 'info> From<&'me [AccountInfo<'info>; WITHDRAW_LEFTOVER_IX_ACCOUNTS_LEN]> for WithdrawLeftoverAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; WITHDRAW_LEFTOVER_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            pool_authority: &arr[0],
            config: &arr[1],
            virtual_pool: &arr[2],
            token_base_account: &arr[3],
            base_vault: &arr[4],
            base_mint: &arr[5],
            leftover_receiver: &arr[6],
            token_base_program: &arr[7],
            event_authority: &arr[8],
            program: &arr[9],
        }
    }
}

pub const WITHDRAW_LEFTOVER_IX_DISCM: [u8; 8] = [20, 198, 202, 237, 235, 243, 183, 66];

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct WithdrawLeftoverIxArgs {}

#[derive(Clone, Debug, PartialEq)]
pub struct WithdrawLeftoverIxData(pub WithdrawLeftoverIxArgs);

impl From<WithdrawLeftoverIxArgs> for WithdrawLeftoverIxData {
    fn from(args: WithdrawLeftoverIxArgs) -> Self {
        Self(args)
    }
}

impl WithdrawLeftoverIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != WITHDRAW_LEFTOVER_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    WITHDRAW_LEFTOVER_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(WithdrawLeftoverIxArgs::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&WITHDRAW_LEFTOVER_IX_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}

pub fn withdraw_leftover_ix_with_program_id(
    program_id: Pubkey,
    keys: WithdrawLeftoverKeys,
    args: WithdrawLeftoverIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; WITHDRAW_LEFTOVER_IX_ACCOUNTS_LEN] = keys.into();
    let data: WithdrawLeftoverIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}

pub fn withdraw_leftover_ix(
    keys: WithdrawLeftoverKeys,
    args: WithdrawLeftoverIxArgs,
) -> std::io::Result<Instruction> {
    withdraw_leftover_ix_with_program_id(crate::ID, keys, args)
}

pub fn withdraw_leftover_invoke_with_program_id(
    program_id: Pubkey,
    accounts: WithdrawLeftoverAccounts<'_, '_>,
    args: WithdrawLeftoverIxArgs,
) -> ProgramResult {
    let keys: WithdrawLeftoverKeys = accounts.into();
    let ix = withdraw_leftover_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}

pub fn withdraw_leftover_invoke(
    accounts: WithdrawLeftoverAccounts<'_, '_>,
    args: WithdrawLeftoverIxArgs,
) -> ProgramResult {
    withdraw_leftover_invoke_with_program_id(crate::ID, accounts, args)
}

pub fn withdraw_leftover_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: WithdrawLeftoverAccounts<'_, '_>,
    args: WithdrawLeftoverIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: WithdrawLeftoverKeys = accounts.into();
    let ix = withdraw_leftover_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}

pub fn withdraw_leftover_invoke_signed(
    accounts: WithdrawLeftoverAccounts<'_, '_>,
    args: WithdrawLeftoverIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    withdraw_leftover_invoke_signed_with_program_id(crate::ID, accounts, args, seeds)
}

pub fn withdraw_leftover_verify_account_keys(
    accounts: WithdrawLeftoverAccounts<'_, '_>,
    keys: WithdrawLeftoverKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.pool_authority.key, keys.pool_authority),
        (*accounts.config.key, keys.config),
        (*accounts.virtual_pool.key, keys.virtual_pool),
        (*accounts.token_base_account.key, keys.token_base_account),
        (*accounts.base_vault.key, keys.base_vault),
        (*accounts.base_mint.key, keys.base_mint),
        (*accounts.leftover_receiver.key, keys.leftover_receiver),
        (*accounts.token_base_program.key, keys.token_base_program),
        (*accounts.event_authority.key, keys.event_authority),
        (*accounts.program.key, keys.program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}

pub fn withdraw_leftover_verify_is_writable_privileges<'me, 'info>(
    accounts: WithdrawLeftoverAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for acct in [
        accounts.virtual_pool,
        accounts.token_base_account,
        accounts.base_vault,
    ] {
        if !acct.is_writable {
            return Err((acct, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}

pub fn withdraw_leftover_verify_is_signer_privileges<'me, 'info>(
    accounts: WithdrawLeftoverAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    Ok(())
}

pub fn withdraw_leftover_verify_account_privileges<'me, 'info>(
    accounts: WithdrawLeftoverAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    withdraw_leftover_verify_is_writable_privileges(accounts)?;
    withdraw_leftover_verify_is_signer_privileges(accounts)?;
    Ok(())
}
pub const WITHDRAW_MIGRATION_FEE_IX_ACCOUNTS_LEN: usize = 10;

#[derive(Copy, Clone, Debug)]
pub struct WithdrawMigrationFeeAccounts<'me, 'info> {
    pub pool_authority: &'me AccountInfo<'info>,
    pub config: &'me AccountInfo<'info>,
    pub virtual_pool: &'me AccountInfo<'info>,
    pub token_quote_account: &'me AccountInfo<'info>,
    pub quote_vault: &'me AccountInfo<'info>,
    pub quote_mint: &'me AccountInfo<'info>,
    pub sender: &'me AccountInfo<'info>,
    pub token_quote_program: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct WithdrawMigrationFeeKeys {
    pub pool_authority: Pubkey,
    pub config: Pubkey,
    pub virtual_pool: Pubkey,
    pub token_quote_account: Pubkey,
    pub quote_vault: Pubkey,
    pub quote_mint: Pubkey,
    pub sender: Pubkey,
    pub token_quote_program: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}

impl From<WithdrawMigrationFeeAccounts<'_, '_>> for WithdrawMigrationFeeKeys {
    fn from(accounts: WithdrawMigrationFeeAccounts) -> Self {
        Self {
            pool_authority: *accounts.pool_authority.key,
            config: *accounts.config.key,
            virtual_pool: *accounts.virtual_pool.key,
            token_quote_account: *accounts.token_quote_account.key,
            quote_vault: *accounts.quote_vault.key,
            quote_mint: *accounts.quote_mint.key,
            sender: *accounts.sender.key,
            token_quote_program: *accounts.token_quote_program.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}

impl From<WithdrawMigrationFeeKeys> for [AccountMeta; WITHDRAW_MIGRATION_FEE_IX_ACCOUNTS_LEN] {
    fn from(keys: WithdrawMigrationFeeKeys) -> Self {
        [
            AccountMeta { pubkey: keys.pool_authority, is_signer: false, is_writable: false },
            AccountMeta { pubkey: keys.config, is_signer: false, is_writable: false },
            AccountMeta { pubkey: keys.virtual_pool, is_signer: false, is_writable: true },
            AccountMeta { pubkey: keys.token_quote_account, is_signer: false, is_writable: true },
            AccountMeta { pubkey: keys.quote_vault, is_signer: false, is_writable: true },
            AccountMeta { pubkey: keys.quote_mint, is_signer: false, is_writable: false },
            AccountMeta { pubkey: keys.sender, is_signer: true, is_writable: false },
            AccountMeta { pubkey: keys.token_quote_program, is_signer: false, is_writable: false },
            AccountMeta { pubkey: keys.event_authority, is_signer: false, is_writable: false },
            AccountMeta { pubkey: keys.program, is_signer: false, is_writable: false },
        ]
    }
}

impl From<[Pubkey; WITHDRAW_MIGRATION_FEE_IX_ACCOUNTS_LEN]> for WithdrawMigrationFeeKeys {
    fn from(pubkeys: [Pubkey; WITHDRAW_MIGRATION_FEE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            pool_authority: pubkeys[0],
            config: pubkeys[1],
            virtual_pool: pubkeys[2],
            token_quote_account: pubkeys[3],
            quote_vault: pubkeys[4],
            quote_mint: pubkeys[5],
            sender: pubkeys[6],
            token_quote_program: pubkeys[7],
            event_authority: pubkeys[8],
            program: pubkeys[9],
        }
    }
}

impl<'info> From<WithdrawMigrationFeeAccounts<'_, 'info>> for [AccountInfo<'info>; WITHDRAW_MIGRATION_FEE_IX_ACCOUNTS_LEN] {
    fn from(accounts: WithdrawMigrationFeeAccounts<'_, 'info>) -> Self {
        [
            accounts.pool_authority.clone(),
            accounts.config.clone(),
            accounts.virtual_pool.clone(),
            accounts.token_quote_account.clone(),
            accounts.quote_vault.clone(),
            accounts.quote_mint.clone(),
            accounts.sender.clone(),
            accounts.token_quote_program.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}

impl<'me, 'info> From<&'me [AccountInfo<'info>; WITHDRAW_MIGRATION_FEE_IX_ACCOUNTS_LEN]> for WithdrawMigrationFeeAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; WITHDRAW_MIGRATION_FEE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            pool_authority: &arr[0],
            config: &arr[1],
            virtual_pool: &arr[2],
            token_quote_account: &arr[3],
            quote_vault: &arr[4],
            quote_mint: &arr[5],
            sender: &arr[6],
            token_quote_program: &arr[7],
            event_authority: &arr[8],
            program: &arr[9],
        }
    }
}

pub const WITHDRAW_MIGRATION_FEE_IX_DISCM: [u8; 8] = [237, 142, 45, 23, 129, 6, 222, 162];

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq,Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct WithdrawMigrationFeeIxArgs {
    pub flag: u8,
}

#[derive(Clone, Debug, PartialEq)]
pub struct WithdrawMigrationFeeIxData(pub WithdrawMigrationFeeIxArgs);

impl From<WithdrawMigrationFeeIxArgs> for WithdrawMigrationFeeIxData {
    fn from(args: WithdrawMigrationFeeIxArgs) -> Self {
        Self(args)
    }
}

impl WithdrawMigrationFeeIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != WITHDRAW_MIGRATION_FEE_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    WITHDRAW_MIGRATION_FEE_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(WithdrawMigrationFeeIxArgs::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&WITHDRAW_MIGRATION_FEE_IX_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}

pub fn withdraw_migration_fee_ix_with_program_id(
    program_id: Pubkey,
    keys: WithdrawMigrationFeeKeys,
    args: WithdrawMigrationFeeIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; WITHDRAW_MIGRATION_FEE_IX_ACCOUNTS_LEN] = keys.into();
    let data: WithdrawMigrationFeeIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}

pub fn withdraw_migration_fee_ix(
    keys: WithdrawMigrationFeeKeys,
    args: WithdrawMigrationFeeIxArgs,
) -> std::io::Result<Instruction> {
    withdraw_migration_fee_ix_with_program_id(crate::ID, keys, args)
}

pub fn withdraw_migration_fee_invoke_with_program_id(
    program_id: Pubkey,
    accounts: WithdrawMigrationFeeAccounts<'_, '_>,
    args: WithdrawMigrationFeeIxArgs,
) -> ProgramResult {
    let keys: WithdrawMigrationFeeKeys = accounts.into();
    let ix = withdraw_migration_fee_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}

pub fn withdraw_migration_fee_invoke(
    accounts: WithdrawMigrationFeeAccounts<'_, '_>,
    args: WithdrawMigrationFeeIxArgs,
) -> ProgramResult {
    withdraw_migration_fee_invoke_with_program_id(crate::ID, accounts, args)
}

pub fn withdraw_migration_fee_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: WithdrawMigrationFeeAccounts<'_, '_>,
    args: WithdrawMigrationFeeIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: WithdrawMigrationFeeKeys = accounts.into();
    let ix = withdraw_migration_fee_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}

pub fn withdraw_migration_fee_invoke_signed(
    accounts: WithdrawMigrationFeeAccounts<'_, '_>,
    args: WithdrawMigrationFeeIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    withdraw_migration_fee_invoke_signed_with_program_id(crate::ID, accounts, args, seeds)
}

pub fn withdraw_migration_fee_verify_account_keys(
    accounts: WithdrawMigrationFeeAccounts<'_, '_>,
    keys: WithdrawMigrationFeeKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.pool_authority.key, keys.pool_authority),
        (*accounts.config.key, keys.config),
        (*accounts.virtual_pool.key, keys.virtual_pool),
        (*accounts.token_quote_account.key, keys.token_quote_account),
        (*accounts.quote_vault.key, keys.quote_vault),
        (*accounts.quote_mint.key, keys.quote_mint),
        (*accounts.sender.key, keys.sender),
        (*accounts.token_quote_program.key, keys.token_quote_program),
        (*accounts.event_authority.key, keys.event_authority),
        (*accounts.program.key, keys.program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}

pub fn withdraw_migration_fee_verify_is_writable_privileges<'me, 'info>(
    accounts: WithdrawMigrationFeeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for acct in [
        accounts.virtual_pool,
        accounts.token_quote_account,
        accounts.quote_vault,
    ] {
        if !acct.is_writable {
            return Err((acct, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}

pub fn withdraw_migration_fee_verify_is_signer_privileges<'me, 'info>(
    accounts: WithdrawMigrationFeeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    if !accounts.sender.is_signer {
        return Err((accounts.sender, ProgramError::MissingRequiredSignature));
    }
    Ok(())
}

pub fn withdraw_migration_fee_verify_account_privileges<'me, 'info>(
    accounts: WithdrawMigrationFeeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    withdraw_migration_fee_verify_is_writable_privileges(accounts)?;
    withdraw_migration_fee_verify_is_signer_privileges(accounts)?;
    Ok(())
}