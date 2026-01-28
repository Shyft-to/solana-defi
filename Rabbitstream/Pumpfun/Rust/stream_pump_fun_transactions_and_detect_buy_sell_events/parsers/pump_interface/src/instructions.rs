#[cfg(feature = "serde")]
use crate::*;
use borsh::{BorshDeserialize, BorshSerialize};
use inflector::Inflector;
use solana_program::{
    account_info::AccountInfo, entrypoint::ProgramResult,
    instruction::{AccountMeta, Instruction},
    program::{invoke, invoke_signed},
    pubkey::Pubkey, program_error::ProgramError,
};
use std::io::Read;
use strum_macros::{Display, EnumString};



#[derive(Clone, Debug, PartialEq, EnumString, Display)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum PumpProgramIx {
    AdminSetCreator(AdminSetCreatorIxArgs),
    AdminSetIdlAuthority(AdminSetIdlAuthorityIxArgs),
    AdminUpdateTokenIncentives(AdminUpdateTokenIncentivesIxArgs),
    Buy(BuyIxArgs),
    BuyExactSolIn(BuyExactSolInIxArgs),
    ClaimTokenIncentives,
    CloseUserVolumeAccumulator,
    CollectCreatorFee,
    Create(CreateIxArgs),
    CreateV2(CreateV2IxArgs),
    DistributeCreatorFees, 
    ExtendAccount,
    GetMinimumDistributableFee,
    InitUserVolumeAccumulator,
    Initialize,
    Migrate,
    MigrateBondingCurveCreator, 
    Sell(SellIxArgs),
    SetCreator(SetCreatorIxArgs),
    SetMayhemVirtualParams,
    SetMetaPlexCreator,
    SetParams(SetParamsIxArgs),
    SetReservedFeeRecipients(SetReservedFeeRecipientsIxArgs), 
    SyncUserVolumeAccumlator,
    ToggleCreateV2(ToggleCreateV2IxArgs),
    ToggleMayhemMode(ToggleMayhemModeIxArgs),
    UpdateGlobalAuthority,
}

impl PumpProgramIx {
    pub fn name(&self) -> String {
        self.to_string().to_camel_case()
    }
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        match maybe_discm {
            ADMIN_SET_CREATOR_IX_DISCM => Ok(Self::AdminSetCreator(AdminSetCreatorIxArgs::deserialize(&mut reader)?)),
            ADMIN_SET_IDL_AUTHORITY_IX_DISCM => Ok(Self::AdminSetIdlAuthority(AdminSetIdlAuthorityIxArgs::deserialize(&mut reader)?)),
            ADMIN_UPDATE_TOKEN_INCENTIVES_IX_DISCM => Ok(Self::AdminUpdateTokenIncentives(AdminUpdateTokenIncentivesIxArgs::deserialize(&mut reader)?)),
            BUY_IX_DISCM => Ok(Self::Buy(BuyIxArgs::deserialize(&mut reader)?)),
            BUY_EXACT_SOL_IN_IX_DISCM => Ok(Self::BuyExactSolIn(BuyExactSolInIxArgs::deserialize(&mut reader)?)),
            CLAIM_TOKEN_INCENTIVES_IX_DISCM => Ok(Self::ClaimTokenIncentives),
            CLOSE_USER_VOLUME_ACCUMULATOR_IX_DISCM => Ok(Self::CloseUserVolumeAccumulator),
            COLLECT_CREATOR_FEE_IX_DISCM => Ok(Self::CollectCreatorFee),
            CREATE_IX_DISCM => Ok(Self::Create(CreateIxArgs::deserialize(&mut reader)?)),
            CREATEV2_IX_DISCM => Ok(Self::CreateV2(CreateV2IxArgs::deserialize(&mut reader)?)),
            DISTRIBUTE_CREATOR_FEES_IX_DISCM => Ok(Self::DistributeCreatorFees),
            EXTEND_ACCOUNT_IX_DISCM => Ok(Self::ExtendAccount),
            GET_MINIMUM_DISTRIBUTABLE_FEE_IX_DISCM => Ok(Self::GetMinimumDistributableFee),
            INIT_USER_VOLUME_ACCUMULATOR_IX_DISCM => Ok(Self::InitUserVolumeAccumulator),
            INITIALIZE_IX_DISCM => Ok(Self::Initialize),
            MIGRATE_IX_DISCM => Ok(Self::Migrate),
            MIGRATE_BONDING_CURVE_CREATOR_IX_DISCM => Ok(Self::MigrateBondingCurveCreator),
            SELL_IX_DISCM => Ok(Self::Sell(SellIxArgs::deserialize(&mut reader)?)),
            SET_CREATOR_IX_DISCM => Ok(Self::SetCreator(SetCreatorIxArgs::deserialize(&mut reader)?)),
            SET_MAYHEM_VIRTUAL_PARAMS_IX_DISCM => Ok(Self::SetMayhemVirtualParams),
            SET_METAPLEX_CREATOR_IX_DISCM => Ok(Self::SetMetaPlexCreator),
            SET_PARAMS_IX_DISCM => Ok(Self::SetParams(SetParamsIxArgs::deserialize(&mut reader)?)),
            SET_RESERVED_FEE_RECIPIENTS_IX_DISCM => Ok(Self::SetReservedFeeRecipients(SetReservedFeeRecipientsIxArgs::deserialize(&mut reader)?)),
            SYNC_USER_VOLUME_ACCUMULATOR_IX_DISCM => Ok(Self::SyncUserVolumeAccumlator),
            TOGGLE_CREATEV2_IX_DISCM => Ok(Self::ToggleCreateV2(ToggleCreateV2IxArgs::deserialize(&mut reader)?)),
            TOGGLE_MAYHEM_MODE_IX_DISCM => Ok(Self::ToggleMayhemMode(ToggleMayhemModeIxArgs::deserialize(&mut reader)?)),
            UPDATE_GLOBAL_AUTHORITY_IX_DISCM => Ok(Self::UpdateGlobalAuthority),
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
            Self::AdminSetCreator(args) => {
                writer.write_all(&ADMIN_SET_CREATOR_IX_DISCM)?;
                args.serialize(&mut writer)
            }
            Self::AdminSetIdlAuthority(args) => {
                writer.write_all(&ADMIN_SET_IDL_AUTHORITY_IX_DISCM)?;
                args.serialize(&mut writer)
            }
            Self::AdminUpdateTokenIncentives(args) => {
                writer.write_all(&ADMIN_UPDATE_TOKEN_INCENTIVES_IX_DISCM)?;
                args.serialize(&mut writer)
            }
             Self::Buy(args) => {
                writer.write_all(&BUY_IX_DISCM)?;
                args.serialize(&mut writer)
            }
            Self::BuyExactSolIn(args) => {
                writer.write_all(&BUY_EXACT_SOL_IN_IX_DISCM)?;
                args.serialize(&mut writer)
            }
            Self::ClaimTokenIncentives => writer.write_all(&CLAIM_TOKEN_INCENTIVES_IX_DISCM),
            Self::CloseUserVolumeAccumulator => writer.write_all(&CLOSE_USER_VOLUME_ACCUMULATOR_IX_DISCM),
            Self::CollectCreatorFee => writer.write_all(&COLLECT_CREATOR_FEE_IX_DISCM),
            Self::Create(args) => {
                writer.write_all(&CREATE_IX_DISCM)?;
                args.serialize(&mut writer)
            }
            Self::CreateV2(args) => {
                writer.write_all(&CREATEV2_IX_DISCM)?;
                args.serialize(&mut writer)
            }
            Self::DistributeCreatorFees => writer.write_all(&DISTRIBUTE_CREATOR_FEES_IX_DISCM),
            Self::ExtendAccount => writer.write_all(&EXTEND_ACCOUNT_IX_DISCM),
            Self::GetMinimumDistributableFee => writer.write_all(&GET_MINIMUM_DISTRIBUTABLE_FEE_IX_DISCM),
            Self::InitUserVolumeAccumulator => writer.write_all(&INIT_USER_VOLUME_ACCUMULATOR_IX_DISCM),
            Self::Initialize => writer.write_all(&INITIALIZE_IX_DISCM),
            Self::Migrate =>  writer.write_all(&MIGRATE_IX_DISCM),
            Self::MigrateBondingCurveCreator => writer.write_all(&MIGRATE_BONDING_CURVE_CREATOR_IX_DISCM),
             Self::Sell(args) => {
                writer.write_all(&SELL_IX_DISCM)?;
                args.serialize(&mut writer)
            }
            Self::SetCreator(args) => {
                writer.write_all(&SET_CREATOR_IX_DISCM)?;
                args.serialize(&mut writer)
            }
            Self::SetMayhemVirtualParams => writer.write_all(&SET_MAYHEM_VIRTUAL_PARAMS_IX_DISCM),
            Self::SetMetaPlexCreator => writer.write_all(&SET_METAPLEX_CREATOR_IX_DISCM),

            Self::SetParams(args) => {
                writer.write_all(&SET_PARAMS_IX_DISCM)?;
                args.serialize(&mut writer)
            }
            Self::SetReservedFeeRecipients(args) => {
                writer.write_all(&SET_RESERVED_FEE_RECIPIENTS_IX_DISCM)?;
                args.serialize(&mut writer)
            }
            Self::SyncUserVolumeAccumlator => writer.write_all(&SYNC_USER_VOLUME_ACCUMULATOR_IX_DISCM),
            Self::ToggleCreateV2(args) => {
                writer.write_all(&TOGGLE_CREATEV2_IX_DISCM)?;
                args.serialize(&mut writer)
            }
            Self::ToggleMayhemMode(args) => {
                writer.write_all(&TOGGLE_MAYHEM_MODE_IX_DISCM)?;
                args.serialize(&mut writer)
            }
            Self::UpdateGlobalAuthority => writer.write_all(&UPDATE_GLOBAL_AUTHORITY_IX_DISCM),
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

pub const ADMIN_SET_CREATOR_IX_ACCOUNTS_LEN: usize = 6;
#[derive(Copy, Clone, Debug)]
pub struct AdminSetCreatorAccounts<'me, 'info> {
    pub admin_set_creator_authority: &'me AccountInfo<'info>,
    pub global: &'me AccountInfo<'info>,
    pub mint: &'me AccountInfo<'info>,
    pub bonding_curve: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct AdminSetCreatorKeys {
    pub admin_set_creator_authority: Pubkey,
    pub global : Pubkey,
    pub mint : Pubkey,
    pub bonding_curve: Pubkey,
    pub event_authority : Pubkey,
    pub program : Pubkey,
}

impl From<AdminSetCreatorAccounts<'_, '_>> for AdminSetCreatorKeys {
    fn from(accounts: AdminSetCreatorAccounts) -> Self {
        Self {
            admin_set_creator_authority : *accounts.admin_set_creator_authority.key,
            global : *accounts.global.key,
            mint : *accounts.mint.key,
            bonding_curve : *accounts.bonding_curve.key,
            event_authority : *accounts.event_authority.key,
            program : *accounts.program.key 
        }
    }
}
impl From<AdminSetCreatorKeys> for [AccountMeta; ADMIN_SET_CREATOR_IX_ACCOUNTS_LEN] {
    fn from(keys: AdminSetCreatorKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.admin_set_creator_authority,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.global,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.bonding_curve,
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
impl From<[Pubkey; ADMIN_SET_CREATOR_IX_ACCOUNTS_LEN]> for AdminSetCreatorKeys {
    fn from(pubkeys: [Pubkey; ADMIN_SET_CREATOR_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            admin_set_creator_authority: pubkeys[0],
            global: pubkeys[1],
            mint: pubkeys[2],
            bonding_curve: pubkeys[3],
            event_authority: pubkeys[4],
            program: pubkeys[5],
        }
    }
}

impl<'info> From<AdminSetCreatorAccounts<'_, 'info>> for [AccountInfo<'info>; ADMIN_SET_CREATOR_IX_ACCOUNTS_LEN] {
    fn from(accounts: AdminSetCreatorAccounts<'_, 'info>) -> Self {
        [
            accounts.admin_set_creator_authority.clone(),
            accounts.global.clone(),
            accounts.mint.clone(),
            accounts.bonding_curve.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; ADMIN_SET_CREATOR_IX_ACCOUNTS_LEN]>
for AdminSetCreatorAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; ADMIN_SET_CREATOR_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            admin_set_creator_authority: &arr[0],
            global: &arr[1],
            mint: &arr[2],
            bonding_curve: &arr[3],
            event_authority: &arr[4],
            program: &arr[5],
        }
    }
}
pub const ADMIN_SET_CREATOR_IX_DISCM: [u8; 8] = [69, 25, 171, 142, 57, 239, 13, 4];
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct AdminSetCreatorIxArgs {
    pub creator: Pubkey,
}
#[derive(Clone, Debug, PartialEq)]
pub struct AdminSetCreatorIxData(pub AdminSetCreatorIxArgs);
impl From<AdminSetCreatorIxArgs> for AdminSetCreatorIxData {
    fn from(args: AdminSetCreatorIxArgs) -> Self {
        Self(args)
    }
}
impl AdminSetCreatorIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != ADMIN_SET_CREATOR_IX_DISCM {
            return Err(
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!(
                        "discm does not match. Expected: {:?}. Received: {:?}",
                        ADMIN_SET_CREATOR_IX_DISCM, maybe_discm
                    ),
                ),
            );
        }
        Ok(Self(AdminSetCreatorIxArgs::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&ADMIN_SET_CREATOR_IX_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn admin_set_creator_authority_ix_with_program_id(
    program_id: Pubkey,
    keys: AdminSetCreatorKeys,
    args: AdminSetCreatorIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; ADMIN_SET_CREATOR_IX_ACCOUNTS_LEN] = keys.into();
    let data: AdminSetCreatorIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn admin_set_creator_authority_ix(keys: AdminSetCreatorKeys, args: AdminSetCreatorIxArgs) -> std::io::Result<Instruction> {
    admin_set_creator_authority_ix_with_program_id(crate::ID, keys, args)
}
pub fn admin_set_creator_authority_invoke_with_program_id(
    program_id: Pubkey,
    accounts: AdminSetCreatorAccounts<'_, '_>,
    args: AdminSetCreatorIxArgs,
) -> ProgramResult {
    let keys: AdminSetCreatorKeys = accounts.into();
    let ix = admin_set_creator_authority_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn admin_set_creator_authority_invoke(accounts: AdminSetCreatorAccounts<'_, '_>, args: AdminSetCreatorIxArgs) -> ProgramResult {
    admin_set_creator_authority_invoke_with_program_id(crate::ID, accounts, args)
}
pub fn admin_set_creator_authority_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: AdminSetCreatorAccounts<'_, '_>,
    args: AdminSetCreatorIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: AdminSetCreatorKeys = accounts.into();
    let ix = admin_set_creator_authority_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn admin_set_creator_authority_invoke_signed(
    accounts: AdminSetCreatorAccounts<'_, '_>,
    args: AdminSetCreatorIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    admin_set_creator_authority_invoke_signed_with_program_id(crate::ID, accounts, args, seeds)
}
pub fn admin_set_creator_authority_verify_account_keys(
    accounts: AdminSetCreatorAccounts<'_, '_>,
    keys: AdminSetCreatorKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.admin_set_creator_authority.key, keys.admin_set_creator_authority),
        (*accounts.global.key, keys.global),
        (*accounts.mint.key, keys.mint),
        (*accounts.bonding_curve.key, keys.bonding_curve),
        (*accounts.event_authority.key, keys.event_authority),
        (*accounts.program.key, keys.program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn admin_set_creator_authority_verify_writable_privileges<'me, 'info>(
    accounts: AdminSetCreatorAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.admin_set_creator_authority,
        accounts.bonding_curve,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn admin_set_creator_authority_verify_signer_privileges<'me, 'info>(
    accounts: AdminSetCreatorAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.admin_set_creator_authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn admin_set_creator_authority_verify_account_privileges<'me, 'info>(
    accounts: AdminSetCreatorAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    admin_set_creator_authority_verify_writable_privileges(accounts)?;
    admin_set_creator_authority_verify_signer_privileges(accounts)?;
    Ok(())
}


pub const ADMIN_SET_IDL_AUTHORITY_IX_ACCOUNTS_LEN: usize = 7;
#[derive(Copy, Clone, Debug)]
pub struct AdminSetIdlAuthorityAccounts<'me, 'info> {
    pub authority: &'me AccountInfo<'info>,
    pub global: &'me AccountInfo<'info>,
    pub idl_account: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub program_signer: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program : &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct AdminSetIdlAuthorityKeys {
    pub authority: Pubkey,
    pub global : Pubkey,
    pub idl_account : Pubkey,
    pub system_program: Pubkey,
    pub program_signer: Pubkey,
    pub event_authority : Pubkey,
    pub program : Pubkey,
}

impl From<AdminSetIdlAuthorityAccounts<'_, '_>> for AdminSetIdlAuthorityKeys {
    fn from(accounts: AdminSetIdlAuthorityAccounts) -> Self {
        Self {
            authority : *accounts.authority.key,
            global : *accounts.global.key,
            idl_account : *accounts.idl_account.key,
            system_program : *accounts.system_program.key,
            program_signer : *accounts.program_signer.key,
            event_authority : *accounts.event_authority.key,
            program : *accounts.program.key 
        }
    }
}
impl From<AdminSetIdlAuthorityKeys> for [AccountMeta; ADMIN_SET_IDL_AUTHORITY_IX_ACCOUNTS_LEN] {
    fn from(keys: AdminSetIdlAuthorityKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.authority,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.global,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.idl_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.system_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey : keys.program_signer,
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
impl From<[Pubkey; ADMIN_SET_IDL_AUTHORITY_IX_ACCOUNTS_LEN]> for AdminSetIdlAuthorityKeys {
    fn from(pubkeys: [Pubkey; ADMIN_SET_IDL_AUTHORITY_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            authority: pubkeys[0],
            global: pubkeys[1],
            idl_account: pubkeys[2],
            system_program: pubkeys[3],
            program_signer: pubkeys[4],
            event_authority: pubkeys[5],
            program: pubkeys[6],
        }
    }
}

impl<'info> From<AdminSetIdlAuthorityAccounts<'_, 'info>> for [AccountInfo<'info>; ADMIN_SET_IDL_AUTHORITY_IX_ACCOUNTS_LEN] {
    fn from(accounts: AdminSetIdlAuthorityAccounts<'_, 'info>) -> Self {
        [
            accounts.authority.clone(),
            accounts.global.clone(),
            accounts.idl_account.clone(),
            accounts.system_program.clone(),
            accounts.program_signer.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; ADMIN_SET_IDL_AUTHORITY_IX_ACCOUNTS_LEN]>
for AdminSetIdlAuthorityAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; ADMIN_SET_IDL_AUTHORITY_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            authority: &arr[0],
            global: &arr[1],
            idl_account: &arr[2],
            system_program: &arr[3],
            program_signer: &arr[4],
            event_authority: &arr[5],
            program: &arr[6],
        }
    }
}

pub const ADMIN_SET_IDL_AUTHORITY_IX_DISCM: [u8; 8] = [8, 217, 96, 231, 144, 104, 192, 5];
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct AdminSetIdlAuthorityIxArgs {
    pub idl_authority: Pubkey,
}
#[derive(Clone, Debug, PartialEq)]
pub struct AdminSetIdlAuthorityIxData(pub AdminSetIdlAuthorityIxArgs);
impl From<AdminSetIdlAuthorityIxArgs> for AdminSetIdlAuthorityIxData {
    fn from(args: AdminSetIdlAuthorityIxArgs) -> Self {
        Self(args)
    }
}
impl AdminSetIdlAuthorityIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != ADMIN_SET_IDL_AUTHORITY_IX_DISCM {
            return Err(
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!(
                        "discm does not match. Expected: {:?}. Received: {:?}",
                        ADMIN_SET_IDL_AUTHORITY_IX_DISCM, maybe_discm
                    ),
                ),
            );
        }
        Ok(Self(AdminSetIdlAuthorityIxArgs::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&ADMIN_SET_IDL_AUTHORITY_IX_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn admin_set_idl_authority_authority_ix_with_program_id(
    program_id: Pubkey,
    keys: AdminSetIdlAuthorityKeys,
    args: AdminSetIdlAuthorityIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; ADMIN_SET_IDL_AUTHORITY_IX_ACCOUNTS_LEN] = keys.into();
    let data: AdminSetIdlAuthorityIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn admin_set_idl_authority_authority_ix(keys: AdminSetIdlAuthorityKeys, args: AdminSetIdlAuthorityIxArgs) -> std::io::Result<Instruction> {
    admin_set_idl_authority_authority_ix_with_program_id(crate::ID, keys, args)
}
pub fn admin_set_idl_authority_authority_invoke_with_program_id(
    program_id: Pubkey,
    accounts: AdminSetIdlAuthorityAccounts<'_, '_>,
    args: AdminSetIdlAuthorityIxArgs,
) -> ProgramResult {
    let keys: AdminSetIdlAuthorityKeys = accounts.into();
    let ix = admin_set_idl_authority_authority_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn admin_set_idl_authority_authority_invoke(
    accounts: AdminSetIdlAuthorityAccounts<'_, '_>,
    args: AdminSetIdlAuthorityIxArgs) -> ProgramResult {
    admin_set_idl_authority_authority_invoke_with_program_id(crate::ID, accounts, args)
}
pub fn admin_set_idl_authority_authority_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: AdminSetIdlAuthorityAccounts<'_, '_>,
    args: AdminSetIdlAuthorityIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: AdminSetIdlAuthorityKeys = accounts.into();
    let ix = admin_set_idl_authority_authority_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn admin_set_idl_authority_authority_invoke_signed(
    accounts: AdminSetIdlAuthorityAccounts<'_, '_>,
    args: AdminSetIdlAuthorityIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    admin_set_idl_authority_authority_invoke_signed_with_program_id(crate::ID, accounts, args, seeds)
}
pub fn admin_set_idl_authority_authority_verify_account_keys(
    accounts: AdminSetIdlAuthorityAccounts<'_, '_>,
    keys: AdminSetIdlAuthorityKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.authority.key, keys.authority),
        (*accounts.global.key, keys.global),
        (*accounts.idl_account.key, keys.idl_account),
        (*accounts.system_program.key, keys.system_program),
        (*accounts.program_signer.key, keys.program_signer),
        (*accounts.event_authority.key, keys.event_authority),
        (*accounts.program.key, keys.program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn admin_set_idl_authority_authority_verify_writable_privileges<'me, 'info>(
    accounts: AdminSetIdlAuthorityAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.authority,
        accounts.idl_account,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn admin_set_idl_authority_authority_verify_signer_privileges<'me, 'info>(
    accounts: AdminSetIdlAuthorityAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn admin_set_idl_authority_authority_verify_account_privileges<'me, 'info>(
    accounts: AdminSetIdlAuthorityAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    admin_set_idl_authority_authority_verify_writable_privileges(accounts)?;
    admin_set_idl_authority_authority_verify_signer_privileges(accounts)?;
    Ok(())
}

pub const ADMIN_UPDATE_TOKEN_INCENTIVES_IX_ACCOUNTS_LEN: usize = 10;

#[derive(Copy, Clone, Debug)]
pub struct AdminUpdateTokenIncentivesAccounts<'me, 'info> {
    pub authority: &'me AccountInfo<'info>,
    pub global: &'me AccountInfo<'info>,
    pub global_volume_accumulator: &'me AccountInfo<'info>,
    pub mint: &'me AccountInfo<'info>,
    pub global_incentive_token_account: &'me AccountInfo<'info>,
    pub associated_token_program: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct AdminUpdateTokenIncentivesKeys {
    pub authority: Pubkey,
    pub global: Pubkey,
    pub global_volume_accumulator: Pubkey,
    pub mint: Pubkey,
    pub global_incentive_token_account: Pubkey,
    pub associated_token_program: Pubkey,
    pub system_program: Pubkey,
    pub token_program: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}

impl From<AdminUpdateTokenIncentivesAccounts<'_, '_>> for AdminUpdateTokenIncentivesKeys {
    fn from(accounts: AdminUpdateTokenIncentivesAccounts) -> Self {
        Self {
            authority: *accounts.authority.key,
            global: *accounts.global.key,
            global_volume_accumulator: *accounts.global_volume_accumulator.key,
            mint: *accounts.mint.key,
            global_incentive_token_account: *accounts.global_incentive_token_account.key,
            associated_token_program: *accounts.associated_token_program.key,
            system_program: *accounts.system_program.key,
            token_program: *accounts.token_program.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}

impl From<AdminUpdateTokenIncentivesKeys> for [AccountMeta; ADMIN_UPDATE_TOKEN_INCENTIVES_IX_ACCOUNTS_LEN] {
    fn from(keys: AdminUpdateTokenIncentivesKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.authority,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.global,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.global_volume_accumulator,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.global_incentive_token_account,
                is_signer: false,
                is_writable: true,
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

impl From<[Pubkey; ADMIN_UPDATE_TOKEN_INCENTIVES_IX_ACCOUNTS_LEN]> for AdminUpdateTokenIncentivesKeys {
    fn from(pubkeys: [Pubkey; ADMIN_UPDATE_TOKEN_INCENTIVES_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            authority: pubkeys[0],
            global: pubkeys[1],
            global_volume_accumulator: pubkeys[2],
            mint: pubkeys[3],
            global_incentive_token_account: pubkeys[4],
            associated_token_program: pubkeys[5],
            system_program: pubkeys[6],
            token_program: pubkeys[7],
            event_authority: pubkeys[8],
            program: pubkeys[9],
        }
    }
}

impl<'info> From<AdminUpdateTokenIncentivesAccounts<'_, 'info>> for [AccountInfo<'info>; ADMIN_UPDATE_TOKEN_INCENTIVES_IX_ACCOUNTS_LEN] {
    fn from(accounts: AdminUpdateTokenIncentivesAccounts<'_, 'info>) -> Self {
        [
            accounts.authority.clone(),
            accounts.global.clone(),
            accounts.global_volume_accumulator.clone(),
            accounts.mint.clone(),
            accounts.global_incentive_token_account.clone(),
            accounts.associated_token_program.clone(),
            accounts.system_program.clone(),
            accounts.token_program.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}

impl<'me, 'info> From<&'me [AccountInfo<'info>; ADMIN_UPDATE_TOKEN_INCENTIVES_IX_ACCOUNTS_LEN]> for AdminUpdateTokenIncentivesAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; ADMIN_UPDATE_TOKEN_INCENTIVES_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            authority: &arr[0],
            global: &arr[1],
            global_volume_accumulator: &arr[2],
            mint: &arr[3],
            global_incentive_token_account: &arr[4],
            associated_token_program: &arr[5],
            system_program: &arr[6],
            token_program: &arr[7],
            event_authority: &arr[8],
            program: &arr[9],
        }
    }
}

pub const ADMIN_UPDATE_TOKEN_INCENTIVES_IX_DISCM: [u8; 8] = [209, 11, 115, 87, 213, 23, 124, 204];

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct AdminUpdateTokenIncentivesIxArgs {
    pub start_time: i64,
    pub end_time: i64,
    pub seconds_in_a_day: i64,
    pub day_number: u64,
    pub pump_token_supply_per_day: u64,
}

#[derive(Clone, Debug, PartialEq)]
pub struct AdminUpdateTokenIncentivesIxData(pub AdminUpdateTokenIncentivesIxArgs);

impl From<AdminUpdateTokenIncentivesIxArgs> for AdminUpdateTokenIncentivesIxData {
    fn from(args: AdminUpdateTokenIncentivesIxArgs) -> Self {
        Self(args)
    }
}

impl AdminUpdateTokenIncentivesIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != ADMIN_UPDATE_TOKEN_INCENTIVES_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    ADMIN_UPDATE_TOKEN_INCENTIVES_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(AdminUpdateTokenIncentivesIxArgs::deserialize(&mut reader)?))
    }

    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&ADMIN_UPDATE_TOKEN_INCENTIVES_IX_DISCM)?;
        self.0.serialize(&mut writer)
    }

    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}

pub fn admin_update_token_incentives_ix_with_program_id(
    program_id: Pubkey,
    keys: AdminUpdateTokenIncentivesKeys,
    args: AdminUpdateTokenIncentivesIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; ADMIN_UPDATE_TOKEN_INCENTIVES_IX_ACCOUNTS_LEN] = keys.into();
    let data: AdminUpdateTokenIncentivesIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}

pub fn admin_update_token_incentives_ix(
    keys: AdminUpdateTokenIncentivesKeys,
    args: AdminUpdateTokenIncentivesIxArgs,
) -> std::io::Result<Instruction> {
    admin_update_token_incentives_ix_with_program_id(crate::ID, keys, args)
}

pub fn admin_update_token_incentives_invoke_with_program_id(
    program_id: Pubkey,
    accounts: AdminUpdateTokenIncentivesAccounts<'_, '_>,
    args: AdminUpdateTokenIncentivesIxArgs,
) -> ProgramResult {
    let keys: AdminUpdateTokenIncentivesKeys = accounts.into();
    let ix = admin_update_token_incentives_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}

pub fn admin_update_token_incentives_invoke(
    accounts: AdminUpdateTokenIncentivesAccounts<'_, '_>,
    args: AdminUpdateTokenIncentivesIxArgs,
) -> ProgramResult {
    admin_update_token_incentives_invoke_with_program_id(crate::ID, accounts, args)
}

pub fn admin_update_token_incentives_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: AdminUpdateTokenIncentivesAccounts<'_, '_>,
    args: AdminUpdateTokenIncentivesIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: AdminUpdateTokenIncentivesKeys = accounts.into();
    let ix = admin_update_token_incentives_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}

pub fn admin_update_token_incentives_invoke_signed(
    accounts: AdminUpdateTokenIncentivesAccounts<'_, '_>,
    args: AdminUpdateTokenIncentivesIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    admin_update_token_incentives_invoke_signed_with_program_id(crate::ID, accounts, args, seeds)
}

pub fn admin_update_token_incentives_verify_account_keys(
    accounts: AdminUpdateTokenIncentivesAccounts<'_, '_>,
    keys: AdminUpdateTokenIncentivesKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.authority.key, keys.authority),
        (*accounts.global.key, keys.global),
        (*accounts.global_volume_accumulator.key, keys.global_volume_accumulator),
        (*accounts.mint.key, keys.mint),
        (*accounts.global_incentive_token_account.key, keys.global_incentive_token_account),
        (*accounts.associated_token_program.key, keys.associated_token_program),
        (*accounts.system_program.key, keys.system_program),
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

pub fn admin_update_token_incentives_verify_writable_privileges<'me, 'info>(
    accounts: AdminUpdateTokenIncentivesAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.authority,
        accounts.global_volume_accumulator,
        accounts.global_incentive_token_account,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}

pub fn admin_update_token_incentives_verify_signer_privileges<'me, 'info>(
    accounts: AdminUpdateTokenIncentivesAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}

pub fn admin_update_token_incentives_verify_account_privileges<'me, 'info>(
    accounts: AdminUpdateTokenIncentivesAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    admin_update_token_incentives_verify_writable_privileges(accounts)?;
    admin_update_token_incentives_verify_signer_privileges(accounts)?;
    Ok(())
}

pub const BUY_IX_ACCOUNTS_LEN: usize = 16;
#[derive(Copy, Clone, Debug)]
pub struct BuyAccounts<'me, 'info> {
    pub global: &'me AccountInfo<'info>,
    pub fee_recipient: &'me AccountInfo<'info>,
    pub mint: &'me AccountInfo<'info>,
    pub bonding_curve: &'me AccountInfo<'info>,
    pub associated_bonding_curve: &'me AccountInfo<'info>,
    pub associated_user: &'me AccountInfo<'info>,
    pub user: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub creator_vault: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
    pub global_volume_accumulator :  &'me AccountInfo<'info>,
    pub user_volume_accumulator: &'me AccountInfo<'info>,
    pub fee_config: &'me AccountInfo<'info>,
    pub fee_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct BuyKeys {
    pub global: Pubkey,
    pub fee_recipient: Pubkey,
    pub mint: Pubkey,
    pub bonding_curve: Pubkey,
    pub associated_bonding_curve: Pubkey,
    pub associated_user: Pubkey,
    pub user: Pubkey,
    pub system_program: Pubkey,
    pub token_program: Pubkey,
    pub creator_vault: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
    pub global_volume_accumulator: Pubkey,
    pub user_volume_accumulator: Pubkey,
    pub fee_config: Pubkey,
    pub fee_program: Pubkey,
}
impl From<BuyAccounts<'_, '_>> for BuyKeys {
    fn from(accounts: BuyAccounts) -> Self {
        Self {
            global: *accounts.global.key,
            fee_recipient: *accounts.fee_recipient.key,
            mint: *accounts.mint.key,
            bonding_curve: *accounts.bonding_curve.key,
            associated_bonding_curve: *accounts.associated_bonding_curve.key,
            associated_user: *accounts.associated_user.key,
            user: *accounts.user.key,
            system_program: *accounts.system_program.key,
            token_program: *accounts.token_program.key,
            creator_vault: *accounts.creator_vault.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
            global_volume_accumulator: *accounts.global_volume_accumulator.key,
            user_volume_accumulator: *accounts.user_volume_accumulator.key,
            fee_config: *accounts.fee_config.key,
            fee_program: *accounts.fee_program.key,
        }
    }
}
impl From<BuyKeys> for [AccountMeta; BUY_IX_ACCOUNTS_LEN] {
    fn from(keys: BuyKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.global,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.fee_recipient,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.bonding_curve,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.associated_bonding_curve,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.associated_user,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user,
                is_signer: true,
                is_writable: true,
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
                pubkey: keys.creator_vault,
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
            AccountMeta {
                pubkey: keys.global_volume_accumulator,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_volume_accumulator,
                is_signer: false,
                is_writable: true,
            }, 
            AccountMeta {
                pubkey: keys.fee_config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.fee_program,
                is_signer: false,
                is_writable: false,
            }
        ]
    }
}
impl From<[Pubkey; BUY_IX_ACCOUNTS_LEN]> for BuyKeys {
    fn from(pubkeys: [Pubkey; BUY_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            global: pubkeys[0],
            fee_recipient: pubkeys[1],
            mint: pubkeys[2],
            bonding_curve: pubkeys[3],
            associated_bonding_curve: pubkeys[4],
            associated_user: pubkeys[5],
            user: pubkeys[6],
            system_program: pubkeys[7],
            token_program: pubkeys[8],
            creator_vault: pubkeys[9],
            event_authority: pubkeys[10],
            program: pubkeys[11],
            global_volume_accumulator: pubkeys[12],
            user_volume_accumulator: pubkeys[13],
            fee_config: pubkeys[14],
            fee_program: pubkeys[15],
        }
    }
}
impl<'info> From<BuyAccounts<'_, 'info>> for [AccountInfo<'info>; BUY_IX_ACCOUNTS_LEN] {
    fn from(accounts: BuyAccounts<'_, 'info>) -> Self {
        [
            accounts.global.clone(),
            accounts.fee_recipient.clone(),
            accounts.mint.clone(),
            accounts.bonding_curve.clone(),
            accounts.associated_bonding_curve.clone(),
            accounts.associated_user.clone(),
            accounts.user.clone(),
            accounts.system_program.clone(),
            accounts.token_program.clone(),
            accounts.creator_vault.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
            accounts.global_volume_accumulator.clone(),
            accounts.user_volume_accumulator.clone(),
            accounts.fee_config.clone(),
            accounts.fee_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; BUY_IX_ACCOUNTS_LEN]>
for BuyAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; BUY_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            global: &arr[0],
            fee_recipient: &arr[1],
            mint: &arr[2],
            bonding_curve: &arr[3],
            associated_bonding_curve: &arr[4],
            associated_user: &arr[5],
            user: &arr[6],
            system_program: &arr[7],
            token_program: &arr[8],
            creator_vault: &arr[9],
            event_authority: &arr[10],
            program: &arr[11],
            global_volume_accumulator: &arr[12],
            user_volume_accumulator: &arr[13],
            fee_config: &arr[14],
            fee_program: &arr[15],
        }
    }
}

pub const BUY_IX_DISCM: [u8; 8] = [102, 6, 61, 18, 1, 218, 235, 234];
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct BuyIxArgs {
    pub amount: u64,
    pub max_sol_cost: u64,
    // pub track_volume: Option<OptionBool>,
}
#[derive(Clone, Debug, PartialEq)]
pub struct BuyIxData(pub BuyIxArgs);
impl From<BuyIxArgs> for BuyIxData {
    fn from(args: BuyIxArgs) -> Self {
        Self(args)
    }
}
impl BuyIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != BUY_IX_DISCM {
            return Err(
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!(
                        "discm does not match. Expected: {:?}. Received: {:?}",
                        BUY_IX_DISCM, maybe_discm
                    ),
                ),
            );
        }
        Ok(Self(BuyIxArgs::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&BUY_IX_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn buy_ix_with_program_id(
    program_id: Pubkey,
    keys: BuyKeys,
    args: BuyIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; BUY_IX_ACCOUNTS_LEN] = keys.into();
    let data: BuyIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn buy_ix(keys: BuyKeys, args: BuyIxArgs) -> std::io::Result<Instruction> {
    buy_ix_with_program_id(crate::ID, keys, args)
}
pub fn buy_invoke_with_program_id(
    program_id: Pubkey,
    accounts: BuyAccounts<'_, '_>,
    args: BuyIxArgs,
) -> ProgramResult {
    let keys: BuyKeys = accounts.into();
    let ix = buy_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn buy_invoke(accounts: BuyAccounts<'_, '_>, args: BuyIxArgs) -> ProgramResult {
    buy_invoke_with_program_id(crate::ID, accounts, args)
}
pub fn buy_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: BuyAccounts<'_, '_>,
    args: BuyIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: BuyKeys = accounts.into();
    let ix = buy_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn buy_invoke_signed(
    accounts: BuyAccounts<'_, '_>,
    args: BuyIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    buy_invoke_signed_with_program_id(crate::ID, accounts, args, seeds)
}
pub fn buy_verify_account_keys(
    accounts: BuyAccounts<'_, '_>,
    keys: BuyKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.global.key, keys.global),
        (*accounts.fee_recipient.key, keys.fee_recipient),
        (*accounts.mint.key, keys.mint),
        (*accounts.bonding_curve.key, keys.bonding_curve),
        (*accounts.associated_bonding_curve.key, keys.associated_bonding_curve),
        (*accounts.associated_user.key, keys.associated_user),
        (*accounts.user.key, keys.user),
        (*accounts.system_program.key, keys.system_program),
        (*accounts.token_program.key, keys.token_program),
        (*accounts.creator_vault.key, keys.creator_vault),
        (*accounts.event_authority.key, keys.event_authority),
        (*accounts.program.key, keys.program),
        (*accounts.global_volume_accumulator.key, keys.global_volume_accumulator),
        (*accounts.user_volume_accumulator.key, keys.user_volume_accumulator),
        (*accounts.fee_config.key, keys.fee_config),
        (*accounts.fee_program.key, keys.fee_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn buy_verify_writable_privileges<'me, 'info>(
    accounts: BuyAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.fee_recipient,
        accounts.bonding_curve,
        accounts.associated_bonding_curve,
        accounts.associated_user,
        accounts.user,
        accounts.creator_vault,
        accounts.global_volume_accumulator,
        accounts.user_volume_accumulator,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn buy_verify_signer_privileges<'me, 'info>(
    accounts: BuyAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.user] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn buy_verify_account_privileges<'me, 'info>(
    accounts: BuyAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    buy_verify_writable_privileges(accounts)?;
    buy_verify_signer_privileges(accounts)?;
    Ok(())
}

pub const BUY_EXACT_SOL_IN_IX_ACCOUNTS_LEN: usize = 16;
#[derive(Copy, Clone, Debug)]
pub struct BuyExactSolInAccounts<'me, 'info> {
    pub global: &'me AccountInfo<'info>,
    pub fee_recipient: &'me AccountInfo<'info>,
    pub mint: &'me AccountInfo<'info>,
    pub bonding_curve: &'me AccountInfo<'info>,
    pub associated_bonding_curve: &'me AccountInfo<'info>,
    pub associated_user: &'me AccountInfo<'info>,
    pub user: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub creator_vault: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
    pub global_volume_accumulator :  &'me AccountInfo<'info>,
    pub user_volume_accumulator: &'me AccountInfo<'info>,
    pub fee_config: &'me AccountInfo<'info>,
    pub fee_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct BuyExactSolInKeys {
    pub global: Pubkey,
    pub fee_recipient: Pubkey,
    pub mint: Pubkey,
    pub bonding_curve: Pubkey,
    pub associated_bonding_curve: Pubkey,
    pub associated_user: Pubkey,
    pub user: Pubkey,
    pub system_program: Pubkey,
    pub token_program: Pubkey,
    pub creator_vault: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
    pub global_volume_accumulator: Pubkey,
    pub user_volume_accumulator: Pubkey,
    pub fee_config: Pubkey,
    pub fee_program: Pubkey,
}
impl From<BuyExactSolInAccounts<'_, '_>> for BuyExactSolInKeys {
    fn from(accounts: BuyExactSolInAccounts) -> Self {
        Self {
            global: *accounts.global.key,
            fee_recipient: *accounts.fee_recipient.key,
            mint: *accounts.mint.key,
            bonding_curve: *accounts.bonding_curve.key,
            associated_bonding_curve: *accounts.associated_bonding_curve.key,
            associated_user: *accounts.associated_user.key,
            user: *accounts.user.key,
            system_program: *accounts.system_program.key,
            token_program: *accounts.token_program.key,
            creator_vault: *accounts.creator_vault.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
            global_volume_accumulator: *accounts.global_volume_accumulator.key,
            user_volume_accumulator: *accounts.user_volume_accumulator.key,
            fee_config: *accounts.fee_config.key,
            fee_program: *accounts.fee_program.key,
        }
    }
}
impl From<BuyExactSolInKeys> for [AccountMeta; BUY_EXACT_SOL_IN_IX_ACCOUNTS_LEN] {
    fn from(keys: BuyExactSolInKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.global,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.fee_recipient,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.bonding_curve,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.associated_bonding_curve,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.associated_user,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user,
                is_signer: true,
                is_writable: true,
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
                pubkey: keys.creator_vault,
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
            AccountMeta {
                pubkey: keys.global_volume_accumulator,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_volume_accumulator,
                is_signer: false,
                is_writable: true,
            }, 
            AccountMeta {
                pubkey: keys.fee_config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.fee_program,
                is_signer: false,
                is_writable: false,
            }
        ]
    }
}
impl From<[Pubkey; BUY_EXACT_SOL_IN_IX_ACCOUNTS_LEN]> for BuyExactSolInKeys {
    fn from(pubkeys: [Pubkey; BUY_EXACT_SOL_IN_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            global: pubkeys[0],
            fee_recipient: pubkeys[1],
            mint: pubkeys[2],
            bonding_curve: pubkeys[3],
            associated_bonding_curve: pubkeys[4],
            associated_user: pubkeys[5],
            user: pubkeys[6],
            system_program: pubkeys[7],
            token_program: pubkeys[8],
            creator_vault: pubkeys[9],
            event_authority: pubkeys[10],
            program: pubkeys[11],
            global_volume_accumulator: pubkeys[12],
            user_volume_accumulator: pubkeys[13],
            fee_config: pubkeys[14],
            fee_program: pubkeys[15],
        }
    }
}
impl<'info> From<BuyExactSolInAccounts<'_, 'info>> for [AccountInfo<'info>; BUY_EXACT_SOL_IN_IX_ACCOUNTS_LEN] {
    fn from(accounts: BuyExactSolInAccounts<'_, 'info>) -> Self {
        [
            accounts.global.clone(),
            accounts.fee_recipient.clone(),
            accounts.mint.clone(),
            accounts.bonding_curve.clone(),
            accounts.associated_bonding_curve.clone(),
            accounts.associated_user.clone(),
            accounts.user.clone(),
            accounts.system_program.clone(),
            accounts.token_program.clone(),
            accounts.creator_vault.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
            accounts.global_volume_accumulator.clone(),
            accounts.user_volume_accumulator.clone(),
            accounts.fee_config.clone(),
            accounts.fee_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; BUY_EXACT_SOL_IN_IX_ACCOUNTS_LEN]>
for BuyExactSolInAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; BUY_EXACT_SOL_IN_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            global: &arr[0],
            fee_recipient: &arr[1],
            mint: &arr[2],
            bonding_curve: &arr[3],
            associated_bonding_curve: &arr[4],
            associated_user: &arr[5],
            user: &arr[6],
            system_program: &arr[7],
            token_program: &arr[8],
            creator_vault: &arr[9],
            event_authority: &arr[10],
            program: &arr[11],
            global_volume_accumulator: &arr[12],
            user_volume_accumulator: &arr[13],
            fee_config: &arr[14],
            fee_program: &arr[15],
        }
    }
}
pub const BUY_EXACT_SOL_IN_IX_DISCM: [u8; 8] = [56, 252, 116, 8, 158, 223, 205, 95];
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct BuyExactSolInIxArgs {
    pub spendable_sol_in: u64,
    pub min_tokens_out: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct BuyExactSolInIxData(pub BuyExactSolInIxArgs);
impl From<BuyExactSolInIxArgs> for BuyExactSolInIxData {
    fn from(args: BuyExactSolInIxArgs) -> Self {
        Self(args)
    }
}
impl BuyExactSolInIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != BUY_EXACT_SOL_IN_IX_DISCM {
            return Err(
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!(
                        "discm does not match. Expected: {:?}. Received: {:?}",
                        BUY_EXACT_SOL_IN_IX_DISCM, maybe_discm
                    ),
                ),
            );
        }
        Ok(Self(BuyExactSolInIxArgs::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&BUY_EXACT_SOL_IN_IX_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn buy_exact_sol_in_ix_with_program_id(
    program_id: Pubkey,
    keys: BuyExactSolInKeys,
    args: BuyExactSolInIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; BUY_EXACT_SOL_IN_IX_ACCOUNTS_LEN] = keys.into();
    let data: BuyExactSolInIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn buy_exact_sol_in_ix(keys: BuyExactSolInKeys, args: BuyExactSolInIxArgs) -> std::io::Result<Instruction> {
    buy_exact_sol_in_ix_with_program_id(crate::ID, keys, args)
}
pub fn buy_exact_sol_in_invoke_with_program_id(
    program_id: Pubkey,
    accounts: BuyExactSolInAccounts<'_, '_>,
    args: BuyExactSolInIxArgs,
) -> ProgramResult {
    let keys: BuyExactSolInKeys = accounts.into();
    let ix = buy_exact_sol_in_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn buy_exact_sol_in_invoke(accounts: BuyExactSolInAccounts<'_, '_>, args: BuyExactSolInIxArgs) -> ProgramResult {
    buy_exact_sol_in_invoke_with_program_id(crate::ID, accounts, args)
}
pub fn buy_exact_sol_in_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: BuyExactSolInAccounts<'_, '_>,
    args: BuyExactSolInIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: BuyExactSolInKeys = accounts.into();
    let ix = buy_exact_sol_in_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn buy_exact_sol_in_invoke_signed(
    accounts: BuyExactSolInAccounts<'_, '_>,
    args: BuyExactSolInIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    buy_exact_sol_in_invoke_signed_with_program_id(crate::ID, accounts, args, seeds)
}
pub fn buy_exact_sol_in_verify_account_keys(
    accounts: BuyExactSolInAccounts<'_, '_>,
    keys: BuyExactSolInKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.global.key, keys.global),
        (*accounts.fee_recipient.key, keys.fee_recipient),
        (*accounts.mint.key, keys.mint),
        (*accounts.bonding_curve.key, keys.bonding_curve),
        (*accounts.associated_bonding_curve.key, keys.associated_bonding_curve),
        (*accounts.associated_user.key, keys.associated_user),
        (*accounts.user.key, keys.user),
        (*accounts.system_program.key, keys.system_program),
        (*accounts.token_program.key, keys.token_program),
        (*accounts.creator_vault.key, keys.creator_vault),
        (*accounts.event_authority.key, keys.event_authority),
        (*accounts.program.key, keys.program),
        (*accounts.global_volume_accumulator.key, keys.global_volume_accumulator),
        (*accounts.user_volume_accumulator.key, keys.user_volume_accumulator),
        (*accounts.fee_config.key, keys.fee_config),
        (*accounts.fee_program.key, keys.fee_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn buy_exact_sol_in_verify_writable_privileges<'me, 'info>(
    accounts: BuyExactSolInAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.fee_recipient,
        accounts.bonding_curve,
        accounts.associated_bonding_curve,
        accounts.associated_user,
        accounts.user,
        accounts.creator_vault,
        accounts.global_volume_accumulator,
        accounts.user_volume_accumulator,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn buy_exact_sol_in_verify_signer_privileges<'me, 'info>(
    accounts: BuyExactSolInAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.user] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn buy_exact_sol_in_verify_account_privileges<'me, 'info>(
    accounts: BuyExactSolInAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    buy_exact_sol_in_verify_writable_privileges(accounts)?;
    buy_exact_sol_in_verify_signer_privileges(accounts)?;
    Ok(())
}


pub const CLAIM_TOKEN_INCENTIVES_IX_ACCOUNTS_LEN: usize = 12;
#[derive(Copy, Clone, Debug)]
pub struct ClaimTokenIncentivesAccounts<'me, 'info> {
    pub user: &'me AccountInfo<'info>,
    pub user_ata: &'me AccountInfo<'info>,
    pub global_volume_accumulator: &'me AccountInfo<'info>,
    pub global_incentive_token_account: &'me AccountInfo<'info>,
    pub user_volume_accumulator: &'me AccountInfo<'info>,
    pub mint: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub associated_token_program: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
    pub payer: &'me AccountInfo<'info>,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct ClaimTokenIncentivesKeys {
    pub user: Pubkey,
    pub user_ata: Pubkey,
    pub global_volume_accumulator: Pubkey,
    pub global_incentive_token_account: Pubkey,
    pub user_volume_accumulator: Pubkey,
    pub mint: Pubkey,
    pub token_program: Pubkey,
    pub system_program: Pubkey,
    pub associated_token_program: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
    pub payer: Pubkey,
}

impl From<ClaimTokenIncentivesAccounts<'_, '_>> for ClaimTokenIncentivesKeys {
    fn from(accounts: ClaimTokenIncentivesAccounts) -> Self {
        Self {
            user: *accounts.user.key,
            user_ata: *accounts.user_ata.key,
            global_volume_accumulator: *accounts.global_volume_accumulator.key,
            global_incentive_token_account: *accounts.global_incentive_token_account.key,
            user_volume_accumulator: *accounts.user_volume_accumulator.key,
            mint: *accounts.mint.key,
            token_program: *accounts.token_program.key,
            system_program: *accounts.system_program.key,
            associated_token_program: *accounts.associated_token_program.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
            payer: *accounts.payer.key,
        }
    }
}

impl From<ClaimTokenIncentivesKeys> for [AccountMeta; CLAIM_TOKEN_INCENTIVES_IX_ACCOUNTS_LEN] {
    fn from(keys: ClaimTokenIncentivesKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.user,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.user_ata,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.global_volume_accumulator,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.global_incentive_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_volume_accumulator,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.mint,
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
            AccountMeta {
                pubkey: keys.payer,
                is_signer: true,
                is_writable: true,
            },
        ]
    }
}

impl From<[Pubkey; CLAIM_TOKEN_INCENTIVES_IX_ACCOUNTS_LEN]> for ClaimTokenIncentivesKeys {
    fn from(pubkeys: [Pubkey; CLAIM_TOKEN_INCENTIVES_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            user: pubkeys[0],
            user_ata: pubkeys[1],
            global_volume_accumulator: pubkeys[2],
            global_incentive_token_account: pubkeys[3],
            user_volume_accumulator: pubkeys[4],
            mint: pubkeys[5],
            token_program: pubkeys[6],
            system_program: pubkeys[7],
            associated_token_program: pubkeys[8],
            event_authority: pubkeys[9],
            program: pubkeys[10],
            payer: pubkeys[11],
        }
    }
}

impl<'info> From<ClaimTokenIncentivesAccounts<'_, 'info>> for [AccountInfo<'info>; CLAIM_TOKEN_INCENTIVES_IX_ACCOUNTS_LEN] {
    fn from(accounts: ClaimTokenIncentivesAccounts<'_, 'info>) -> Self {
        [
            accounts.user.clone(),
            accounts.user_ata.clone(),
            accounts.global_volume_accumulator.clone(),
            accounts.global_incentive_token_account.clone(),
            accounts.user_volume_accumulator.clone(),
            accounts.mint.clone(),
            accounts.token_program.clone(),
            accounts.system_program.clone(),
            accounts.associated_token_program.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
            accounts.payer.clone(),
        ]
    }
}

impl<'me, 'info> From<&'me [AccountInfo<'info>; CLAIM_TOKEN_INCENTIVES_IX_ACCOUNTS_LEN]> for ClaimTokenIncentivesAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; CLAIM_TOKEN_INCENTIVES_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            user: &arr[0],
            user_ata: &arr[1],
            global_volume_accumulator: &arr[2],
            global_incentive_token_account: &arr[3],
            user_volume_accumulator: &arr[4],
            mint: &arr[5],
            token_program: &arr[6],
            system_program: &arr[7],
            associated_token_program: &arr[8],
            event_authority: &arr[9],
            program: &arr[10],
            payer: &arr[11],
        }
    }
}

pub const CLAIM_TOKEN_INCENTIVES_IX_DISCM: [u8; 8] = [16, 4, 71, 28, 204, 1, 40, 27];

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ClaimTokenIncentivesIxArgs {}

#[derive(Clone, Debug, PartialEq)]
pub struct ClaimTokenIncentivesIxData(pub ClaimTokenIncentivesIxArgs);

impl From<ClaimTokenIncentivesIxArgs> for ClaimTokenIncentivesIxData {
    fn from(args: ClaimTokenIncentivesIxArgs) -> Self {
        Self(args)
    }
}

impl ClaimTokenIncentivesIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != CLAIM_TOKEN_INCENTIVES_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    CLAIM_TOKEN_INCENTIVES_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(ClaimTokenIncentivesIxArgs::deserialize(&mut reader)?))
    }

    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&CLAIM_TOKEN_INCENTIVES_IX_DISCM)?;
        self.0.serialize(&mut writer)
    }

    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}

pub fn claim_token_incentives_ix_with_program_id(
    program_id: Pubkey,
    keys: ClaimTokenIncentivesKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; CLAIM_TOKEN_INCENTIVES_IX_ACCOUNTS_LEN] = keys.into();
    let data: ClaimTokenIncentivesIxData = ClaimTokenIncentivesIxArgs {}.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}

pub fn claim_token_incentives_ix(
    keys: ClaimTokenIncentivesKeys,
) -> std::io::Result<Instruction> {
    claim_token_incentives_ix_with_program_id(crate::ID, keys)
}

pub fn claim_token_incentives_invoke_with_program_id(
    program_id: Pubkey,
    accounts: ClaimTokenIncentivesAccounts<'_, '_>,
) -> ProgramResult {
    let keys: ClaimTokenIncentivesKeys = accounts.into();
    let ix = claim_token_incentives_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}

pub fn claim_token_incentives_invoke(
    accounts: ClaimTokenIncentivesAccounts<'_, '_>,
) -> ProgramResult {
    claim_token_incentives_invoke_with_program_id(crate::ID, accounts)
}

pub fn claim_token_incentives_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: ClaimTokenIncentivesAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: ClaimTokenIncentivesKeys = accounts.into();
    let ix = claim_token_incentives_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}

pub fn claim_token_incentives_invoke_signed(
    accounts: ClaimTokenIncentivesAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    claim_token_incentives_invoke_signed_with_program_id(crate::ID, accounts, seeds)
}

pub fn claim_token_incentives_verify_account_keys(
    accounts: ClaimTokenIncentivesAccounts<'_, '_>,
    keys: ClaimTokenIncentivesKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.user.key, keys.user),
        (*accounts.user_ata.key, keys.user_ata),
        (*accounts.global_volume_accumulator.key, keys.global_volume_accumulator),
        (*accounts.global_incentive_token_account.key, keys.global_incentive_token_account),
        (*accounts.user_volume_accumulator.key, keys.user_volume_accumulator),
        (*accounts.mint.key, keys.mint),
        (*accounts.token_program.key, keys.token_program),
        (*accounts.system_program.key, keys.system_program),
        (*accounts.associated_token_program.key, keys.associated_token_program),
        (*accounts.event_authority.key, keys.event_authority),
        (*accounts.program.key, keys.program),
        (*accounts.payer.key, keys.payer),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}

pub fn claim_token_incentives_verify_writable_privileges<'me, 'info>(
    accounts: ClaimTokenIncentivesAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.user_ata,
        accounts.global_incentive_token_account,
        accounts.user_volume_accumulator,
        accounts.payer,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}

pub fn claim_token_incentives_verify_signer_privileges<'me, 'info>(
    accounts: ClaimTokenIncentivesAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.payer] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}

pub fn claim_token_incentives_verify_account_privileges<'me, 'info>(
    accounts: ClaimTokenIncentivesAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    claim_token_incentives_verify_writable_privileges(accounts)?;
    claim_token_incentives_verify_signer_privileges(accounts)?;
    Ok(())
}

pub const CLOSE_USER_VOLUME_ACCUMULATOR_IX_ACCOUNTS_LEN: usize = 4;

#[derive(Copy, Clone, Debug)]
pub struct CloseUserVolumeAccumulatorAccounts<'me, 'info> {
    pub user: &'me AccountInfo<'info>,
    pub user_volume_accumulator: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CloseUserVolumeAccumulatorKeys {
    pub user: Pubkey,
    pub user_volume_accumulator: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}

impl From<CloseUserVolumeAccumulatorAccounts<'_, '_>> for CloseUserVolumeAccumulatorKeys {
    fn from(accounts: CloseUserVolumeAccumulatorAccounts) -> Self {
        Self {
            user: *accounts.user.key,
            user_volume_accumulator: *accounts.user_volume_accumulator.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}

impl From<CloseUserVolumeAccumulatorKeys> for [AccountMeta; CLOSE_USER_VOLUME_ACCUMULATOR_IX_ACCOUNTS_LEN] {
    fn from(keys: CloseUserVolumeAccumulatorKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.user,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_volume_accumulator,
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

impl From<[Pubkey; CLOSE_USER_VOLUME_ACCUMULATOR_IX_ACCOUNTS_LEN]> for CloseUserVolumeAccumulatorKeys {
    fn from(pubkeys: [Pubkey; CLOSE_USER_VOLUME_ACCUMULATOR_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            user: pubkeys[0],
            user_volume_accumulator: pubkeys[1],
            event_authority: pubkeys[2],
            program: pubkeys[3],
        }
    }
}

impl<'info> From<CloseUserVolumeAccumulatorAccounts<'_, 'info>> for [AccountInfo<'info>; CLOSE_USER_VOLUME_ACCUMULATOR_IX_ACCOUNTS_LEN] {
    fn from(accounts: CloseUserVolumeAccumulatorAccounts<'_, 'info>) -> Self {
        [
            accounts.user.clone(),
            accounts.user_volume_accumulator.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}

impl<'me, 'info> From<&'me [AccountInfo<'info>; CLOSE_USER_VOLUME_ACCUMULATOR_IX_ACCOUNTS_LEN]> for CloseUserVolumeAccumulatorAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; CLOSE_USER_VOLUME_ACCUMULATOR_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            user: &arr[0],
            user_volume_accumulator: &arr[1],
            event_authority: &arr[2],
            program: &arr[3],
        }
    }
}

pub const CLOSE_USER_VOLUME_ACCUMULATOR_IX_DISCM: [u8; 8] = [249, 69, 164, 218, 150, 103, 84, 138];

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CloseUserVolumeAccumulatorIxArgs {}

#[derive(Clone, Debug, PartialEq)]
pub struct CloseUserVolumeAccumulatorIxData(pub CloseUserVolumeAccumulatorIxArgs);

impl From<CloseUserVolumeAccumulatorIxArgs> for CloseUserVolumeAccumulatorIxData {
    fn from(args: CloseUserVolumeAccumulatorIxArgs) -> Self {
        Self(args)
    }
}

impl CloseUserVolumeAccumulatorIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != CLOSE_USER_VOLUME_ACCUMULATOR_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    CLOSE_USER_VOLUME_ACCUMULATOR_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(CloseUserVolumeAccumulatorIxArgs::deserialize(&mut reader)?))
    }

    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&CLOSE_USER_VOLUME_ACCUMULATOR_IX_DISCM)?;
        self.0.serialize(&mut writer)
    }

    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}

pub fn close_user_volume_accumulator_ix_with_program_id(
    program_id: Pubkey,
    keys: CloseUserVolumeAccumulatorKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; CLOSE_USER_VOLUME_ACCUMULATOR_IX_ACCOUNTS_LEN] = keys.into();
    let data: CloseUserVolumeAccumulatorIxData = CloseUserVolumeAccumulatorIxArgs {}.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}

pub fn close_user_volume_accumulator_ix(
    keys: CloseUserVolumeAccumulatorKeys,
) -> std::io::Result<Instruction> {
    close_user_volume_accumulator_ix_with_program_id(crate::ID, keys)
}

pub fn close_user_volume_accumulator_invoke_with_program_id(
    program_id: Pubkey,
    accounts: CloseUserVolumeAccumulatorAccounts<'_, '_>,
) -> ProgramResult {
    let keys: CloseUserVolumeAccumulatorKeys = accounts.into();
    let ix = close_user_volume_accumulator_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}

pub fn close_user_volume_accumulator_invoke(
    accounts: CloseUserVolumeAccumulatorAccounts<'_, '_>,
) -> ProgramResult {
    close_user_volume_accumulator_invoke_with_program_id(crate::ID, accounts)
}

pub fn close_user_volume_accumulator_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: CloseUserVolumeAccumulatorAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: CloseUserVolumeAccumulatorKeys = accounts.into();
    let ix = close_user_volume_accumulator_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}

pub fn close_user_volume_accumulator_invoke_signed(
    accounts: CloseUserVolumeAccumulatorAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    close_user_volume_accumulator_invoke_signed_with_program_id(crate::ID, accounts, seeds)
}

pub fn close_user_volume_accumulator_verify_account_keys(
    accounts: CloseUserVolumeAccumulatorAccounts<'_, '_>,
    keys: CloseUserVolumeAccumulatorKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.user.key, keys.user),
        (*accounts.user_volume_accumulator.key, keys.user_volume_accumulator),
        (*accounts.event_authority.key, keys.event_authority),
        (*accounts.program.key, keys.program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}

pub fn close_user_volume_accumulator_verify_writable_privileges<'me, 'info>(
    accounts: CloseUserVolumeAccumulatorAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.user, accounts.user_volume_accumulator] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}

pub fn close_user_volume_accumulator_verify_signer_privileges<'me, 'info>(
    accounts: CloseUserVolumeAccumulatorAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.user] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}

pub fn close_user_volume_accumulator_verify_account_privileges<'me, 'info>(
    accounts: CloseUserVolumeAccumulatorAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    close_user_volume_accumulator_verify_writable_privileges(accounts)?;
    close_user_volume_accumulator_verify_signer_privileges(accounts)?;
    Ok(())
}

pub const COLLECT_CREATOR_FEE_IX_ACCOUNTS_LEN: usize = 5;
#[derive(Copy, Clone, Debug)]
pub struct CollectCreatorFeeAccounts<'me,'info> {
    pub creator : &'me AccountInfo<'info>,
    pub creator_vault : &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program : &'me AccountInfo<'info>
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CollectCreatorFeeKeys {
    pub creator : Pubkey,
    pub creator_vault: Pubkey,
    pub system_program : Pubkey,
    pub event_authority : Pubkey,
    pub program : Pubkey,
}
impl From<CollectCreatorFeeAccounts<'_, '_>> for CollectCreatorFeeKeys {
    fn from(accounts: CollectCreatorFeeAccounts) -> Self {
        Self {
            creator : *accounts.creator.key,
            creator_vault : *accounts.creator_vault.key,
            system_program : *accounts.system_program.key,
            event_authority : *accounts.event_authority.key,
            program : *accounts.program.key,
        }
    }
}
impl From<CollectCreatorFeeKeys> for [AccountMeta; COLLECT_CREATOR_FEE_IX_ACCOUNTS_LEN] {
    fn from(keys: CollectCreatorFeeKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.creator,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.creator_vault,
                is_signer: false,
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
impl From<[Pubkey; COLLECT_CREATOR_FEE_IX_ACCOUNTS_LEN]> for CollectCreatorFeeKeys {
    fn from(pubkeys: [Pubkey; COLLECT_CREATOR_FEE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            creator : pubkeys[0],
            creator_vault : pubkeys[1],
            system_program : pubkeys[2],
            event_authority : pubkeys[3],
            program : pubkeys[4],
        }
    }
}
impl<'info> From<CollectCreatorFeeAccounts<'_, 'info>>
for [AccountInfo<'info>; COLLECT_CREATOR_FEE_IX_ACCOUNTS_LEN] {
    fn from(accounts: CollectCreatorFeeAccounts<'_, 'info>) -> Self {
        [
            accounts.creator.clone(),
            accounts.creator_vault.clone(),
            accounts.system_program.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; COLLECT_CREATOR_FEE_IX_ACCOUNTS_LEN]>
for CollectCreatorFeeAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; COLLECT_CREATOR_FEE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            creator : &arr[0],
            creator_vault : &arr[1],
            system_program : &arr[2],
            event_authority : &arr[3],
            program : &arr[4],
        }
    }
}
pub const COLLECT_CREATOR_FEE_IX_DISCM: [u8; 8] = [20,22,86,123,198,28,219,132];
#[derive(Clone, Debug, PartialEq)]
pub struct CollectCreatorFeeIxData;
impl CollectCreatorFeeIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != COLLECT_CREATOR_FEE_IX_DISCM {
            return Err(
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!(
                        "discm does not match. Expected: {:?}. Received: {:?}",
                        COLLECT_CREATOR_FEE_IX_DISCM, maybe_discm
                    ),
                ),
            );
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&COLLECT_CREATOR_FEE_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}

pub fn collect_creator_fee_ix_with_program_id(
    program_id: Pubkey,
    keys: CollectCreatorFeeKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; COLLECT_CREATOR_FEE_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: CollectCreatorFeeIxData.try_to_vec()?,
    })
}
pub fn collect_creator_fee_ix(keys: CollectCreatorFeeKeys) -> std::io::Result<Instruction> {
    collect_creator_fee_ix_with_program_id(crate::ID, keys)
}
pub fn collect_creator_fee_invoke_with_program_id(
    program_id: Pubkey,
    accounts: CollectCreatorFeeAccounts<'_, '_>,
) -> ProgramResult {
    let keys: CollectCreatorFeeKeys = accounts.into();
    let ix = collect_creator_fee_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn collect_creator_fee_invoke(accounts: CollectCreatorFeeAccounts<'_, '_>) -> ProgramResult {
    collect_creator_fee_invoke_with_program_id(crate::ID, accounts)
}
pub fn collect_creator_fee_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: CollectCreatorFeeAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: CollectCreatorFeeKeys = accounts.into();
    let ix = collect_creator_fee_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn collect_creator_fee_invoke_signed(
    accounts: CollectCreatorFeeAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    collect_creator_fee_invoke_signed_with_program_id(crate::ID, accounts, seeds)
}
pub fn collect_creator_fee_verify_account_keys(
    accounts: CollectCreatorFeeAccounts<'_, '_>,
    keys: CollectCreatorFeeKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.creator.key, keys.creator),
        (*accounts.creator_vault.key, keys.creator_vault),
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
pub fn collect_creator_fee_verify_writable_privileges<'me, 'info>(
    accounts: CollectCreatorFeeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.creator, accounts.creator_vault] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn collect_creator_fee_verify_signer_privileges<'me, 'info>(
    accounts: CollectCreatorFeeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.creator] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn collect_creator_fee_verify_account_privileges<'me, 'info>(
    accounts: CollectCreatorFeeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    collect_creator_fee_verify_writable_privileges(accounts)?;
    collect_creator_fee_verify_signer_privileges(accounts)?;
    Ok(())
}

pub const CREATE_IX_ACCOUNTS_LEN: usize = 14;
#[derive(Copy, Clone, Debug)]
pub struct CreateAccounts<'me, 'info>{
    pub mint : &'me AccountInfo<'info>,
    pub mint_authority : &'me AccountInfo<'info>,
    pub bonding_curve : &'me AccountInfo<'info>,
    pub associated_bonding_curve: &'me AccountInfo<'info>,
    pub global : &'me AccountInfo<'info>,
    pub mpl_token_metadata : &'me AccountInfo<'info>,
    pub metadata : &'me AccountInfo<'info>,
    pub user:&'me AccountInfo<'info>,
    pub system_program : &'me AccountInfo<'info>,
    pub token_program : &'me AccountInfo<'info>,
    pub associated_token_program: &'me AccountInfo<'info>,
    pub rent :&'me AccountInfo<'info>,
    pub event_authority : &'me AccountInfo<'info>,
    pub program : &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CreateKeys {
    pub mint : Pubkey,
    pub mint_authority: Pubkey,
    pub bonding_curve : Pubkey,
    pub associated_bonding_curve : Pubkey,
    pub global : Pubkey,
    pub mpl_token_metadata: Pubkey,
    pub metadata : Pubkey,
    pub user: Pubkey,
    pub system_program: Pubkey,
    pub token_program : Pubkey,
    pub associated_token_program: Pubkey,
    pub rent : Pubkey,
    pub event_authority: Pubkey,
    pub program : Pubkey
}
impl From<CreateAccounts<'_, '_>> for CreateKeys {
    fn from(accounts: CreateAccounts) -> Self {
        Self {
            mint: *accounts.mint.key,
            mint_authority: *accounts.mint_authority.key,
            bonding_curve: *accounts.bonding_curve.key,
            associated_bonding_curve: *accounts.associated_bonding_curve.key,
            global: *accounts.global.key,
            mpl_token_metadata: *accounts.mpl_token_metadata.key,
            metadata: *accounts.metadata.key,
            user: *accounts.user.key,
            system_program: *accounts.system_program.key,
            token_program: *accounts.token_program.key,
            associated_token_program: *accounts.associated_token_program.key,
            rent: *accounts.rent.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<CreateKeys> for [AccountMeta; CREATE_IX_ACCOUNTS_LEN] {
    fn from(keys: CreateKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.mint,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.mint_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.bonding_curve,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.associated_bonding_curve,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.global,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.mpl_token_metadata,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.metadata,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user,
                is_signer: true,
                is_writable: true,
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
                pubkey: keys.rent,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.event_authority,
                is_signer: false,
                is_writable: false
            },
             AccountMeta {
                 pubkey : keys.program ,
                 is_signer : false ,
                 is_writable : false
             }
        ]
    }
}
impl From<[Pubkey; CREATE_IX_ACCOUNTS_LEN]> for CreateKeys {
    fn from(pubkeys: [Pubkey; CREATE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            mint: pubkeys[0],
            mint_authority: pubkeys[1],
            bonding_curve: pubkeys[2],
            associated_bonding_curve: pubkeys[3],
            global: pubkeys[4],
            mpl_token_metadata: pubkeys[5],
            metadata: pubkeys[6],
            user: pubkeys[7],
            system_program: pubkeys[8],
            token_program: pubkeys[9],
            associated_token_program: pubkeys[10],
            rent: pubkeys[11],
            event_authority: pubkeys[12],
            program: pubkeys[13]
        }
    }
}
impl<'info> From<CreateAccounts<'_, 'info>> for [AccountInfo<'info>; CREATE_IX_ACCOUNTS_LEN] {
    fn from(accounts: CreateAccounts<'_, 'info>) -> Self {
        [
            accounts.mint.clone(),
            accounts.mint_authority.clone(),
            accounts.bonding_curve.clone(),
            accounts.associated_bonding_curve.clone(),
            accounts.global.clone(),
            accounts.mpl_token_metadata.clone(),
            accounts.metadata.clone(),
            accounts.user.clone(),
            accounts.system_program.clone(),
            accounts.token_program.clone(),
            accounts.associated_token_program.clone(),
            accounts.rent.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone()
        ]
    }
}

impl<'me, 'info> From<&'me [AccountInfo<'info>; CREATE_IX_ACCOUNTS_LEN]>
for CreateAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; CREATE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            mint: &arr[0],
            mint_authority: &arr[1],
            bonding_curve: &arr[2],
            associated_bonding_curve: &arr[3],
            global: &arr[4],
            mpl_token_metadata: &arr[5],
            metadata: &arr[6],
            user: &arr[7],
            system_program: &arr[8],
            token_program: &arr[9],
            associated_token_program: &arr[10],
            rent: &arr[11],
            event_authority: &arr[12],
            program : &arr[13]
        }
    }
}
pub const CREATE_IX_DISCM: [u8; 8] = [24,30,200,40,5,28,7,119];
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CreateIxArgs {
    pub name: String,
    pub symbol: String,
    pub uri: String,
    pub creator: Pubkey,
}
#[derive(Clone, Debug, PartialEq)]
pub struct CreateIxData(pub CreateIxArgs);
impl From<CreateIxArgs> for CreateIxData {
    fn from(args: CreateIxArgs) -> Self {
        Self(args)
    }
}
impl CreateIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != CREATE_IX_DISCM {
            return Err(
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!(
                        "discm does not match. Expected: {:?}. Received: {:?}",
                        CREATE_IX_DISCM, maybe_discm
                    ),
                ),
            );
        }
        Ok(Self(CreateIxArgs::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&CREATE_IX_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn create_ix_with_program_id(
    program_id: Pubkey,
    keys: CreateKeys,
    args: CreateIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; CREATE_IX_ACCOUNTS_LEN] = keys.into();
    let data: CreateIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn create_ix(keys: CreateKeys, args: CreateIxArgs) -> std::io::Result<Instruction> {
    create_ix_with_program_id(crate::ID, keys, args)
}
pub fn create_invoke_with_program_id(
    program_id: Pubkey,
    accounts: CreateAccounts<'_, '_>,
    args: CreateIxArgs,
) -> ProgramResult {
    let keys: CreateKeys = accounts.into();
    let ix = create_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn create_invoke(accounts: CreateAccounts<'_, '_>, args: CreateIxArgs) -> ProgramResult {
    create_invoke_with_program_id(crate::ID, accounts, args)
}
pub fn create_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: CreateAccounts<'_, '_>,
    args: CreateIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: CreateKeys = accounts.into();
    let ix = create_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn create_invoke_signed(
    accounts: CreateAccounts<'_, '_>,
    args: CreateIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    create_invoke_signed_with_program_id(crate::ID, accounts, args, seeds)
}
pub fn create_verify_account_keys(
    accounts: CreateAccounts<'_, '_>,
    keys: CreateKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.mint.key, keys.mint),
        (*accounts.mint_authority.key, keys.mint_authority),
        (*accounts.bonding_curve.key, keys.bonding_curve),
        (*accounts.associated_bonding_curve.key, keys.associated_bonding_curve),
        (*accounts.global.key, keys.global),
        (*accounts.mpl_token_metadata.key, keys.mpl_token_metadata),
        (*accounts.metadata.key, keys.metadata),
        (*accounts.user.key, keys.user),
        (*accounts.system_program.key, keys.system_program),
        (*accounts.token_program.key, keys.token_program),
        (*accounts.associated_token_program.key, keys.associated_token_program),
        (*accounts.rent.key, keys.rent),
        (*accounts.event_authority.key, keys.event_authority),
        (*accounts.program.key, keys.program)
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn create_verify_writable_privileges<'me, 'info>(
    accounts: CreateAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.mint,
        accounts.bonding_curve,
        accounts.associated_bonding_curve,
        accounts.metadata,
        accounts.user,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn create_verify_signer_privileges<'me, 'info>(
    accounts: CreateAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.mint, accounts.user] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn create_verify_account_privileges<'me, 'info>(
    accounts: CreateAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    create_verify_writable_privileges(accounts)?;
    create_verify_signer_privileges(accounts)?;
    Ok(())
}

pub const CREATEV2_IX_ACCOUNTS_LEN: usize = 16;
#[derive(Copy, Clone, Debug)]
pub struct CreateV2Accounts<'me, 'info> {
    pub mint: &'me AccountInfo<'info>,
    pub mint_authority : &'me AccountInfo<'info>,
    pub bonding_curve: &'me AccountInfo<'info>,
    pub associated_bonding_curve: &'me AccountInfo<'info>,
    pub global: &'me AccountInfo<'info>,
    pub user: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub associated_token_program: &'me AccountInfo<'info>,
    pub mayhem_program_id : &'me AccountInfo<'info>,
    pub global_params: &'me AccountInfo<'info>,
    pub sol_vault: &'me AccountInfo<'info>,
    pub mayhem_state: &'me AccountInfo<'info>,
    pub mayhem_token_vault: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CreateV2Keys {
    pub mint: Pubkey,
    pub mint_authority: Pubkey,
    pub bonding_curve: Pubkey,
    pub associated_bonding_curve: Pubkey,
    pub global: Pubkey,
    pub user: Pubkey,
    pub system_program: Pubkey,
    pub token_program: Pubkey,
    pub associated_token_program: Pubkey,
    pub mayhem_program_id: Pubkey,
    pub global_params: Pubkey,
    pub sol_vault: Pubkey,
    pub mayhem_state: Pubkey,
    pub mayhem_token_vault: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}

impl From<CreateV2Accounts<'_,'_>> for CreateV2Keys {
   fn from(accounts: CreateV2Accounts) -> Self {
    Self {
        mint: *accounts.mint.key,
        mint_authority: *accounts.mint_authority.key,
        bonding_curve: *accounts.bonding_curve.key,
        associated_bonding_curve: *accounts.associated_bonding_curve.key,
        global: *accounts.global.key,
        user: *accounts.user.key,
        system_program: *accounts.system_program.key,
        token_program: *accounts.token_program.key,
        associated_token_program: *accounts.associated_token_program.key,
        mayhem_program_id: *accounts.mayhem_program_id.key,
        global_params: *accounts.global_params.key,
        sol_vault: *accounts.sol_vault.key,
        mayhem_state: *accounts.mayhem_state.key,
        mayhem_token_vault: *accounts.mayhem_token_vault.key,
        event_authority: *accounts.event_authority.key,
        program: *accounts.program.key,
    }
   }
}

impl From<CreateV2Keys> for [AccountMeta; CREATEV2_IX_ACCOUNTS_LEN] {
    fn from(keys: CreateV2Keys) -> Self {
        [
            AccountMeta {
                pubkey: keys.mint,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.mint_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.bonding_curve,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.associated_bonding_curve,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.global,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.user,
                is_signer: true,
                is_writable: true,
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
                pubkey: keys.mayhem_program_id,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.global_params,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.sol_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.mayhem_state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.mayhem_token_vault,
                is_signer: false,
                is_writable: true
            },
            AccountMeta {
                 pubkey : keys.event_authority,
                 is_signer : false ,
                 is_writable : false
            },
            AccountMeta {
                pubkey: keys.program,
                is_signer: false,
                is_writable: false
            }  
        ]
    }
}
impl From<[Pubkey; CREATEV2_IX_ACCOUNTS_LEN]> for CreateV2Keys {
    fn from(pubkeys: [Pubkey; CREATEV2_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            mint: pubkeys[0],
            mint_authority: pubkeys[1],
            bonding_curve: pubkeys[2],
            associated_bonding_curve: pubkeys[3],
            global: pubkeys[4],
            user: pubkeys[5],
            system_program: pubkeys[6],
            token_program: pubkeys[7],
            associated_token_program: pubkeys[8],
            mayhem_program_id: pubkeys[9],
            global_params: pubkeys[10],
            sol_vault: pubkeys[11],
            mayhem_state: pubkeys[12],
            mayhem_token_vault: pubkeys[13],
            event_authority: pubkeys[14],
            program: pubkeys[15],
        }
    }
}
impl<'info> From<CreateV2Accounts<'_, 'info>> for [AccountInfo<'info>; CREATEV2_IX_ACCOUNTS_LEN] {
    fn from(accounts: CreateV2Accounts<'_, 'info>) -> Self {
        [
            accounts.mint.clone(),
            accounts.mint_authority.clone(),
            accounts.bonding_curve.clone(),
            accounts.associated_bonding_curve.clone(),
            accounts.global.clone(),
            accounts.user.clone(),
            accounts.system_program.clone(),
            accounts.token_program.clone(),
            accounts.associated_token_program.clone(),
            accounts.mayhem_program_id.clone(),
            accounts.global_params.clone(),
            accounts.sol_vault.clone(),
            accounts.mayhem_state.clone(),
            accounts.mayhem_token_vault.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone()
        ]
    }
}

impl<'me, 'info> From<&'me [AccountInfo<'info>; CREATEV2_IX_ACCOUNTS_LEN]>
for CreateV2Accounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; CREATEV2_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            mint: &arr[0],
            mint_authority: &arr[1],
            bonding_curve: &arr[2],
            associated_bonding_curve: &arr[3],
            global: &arr[4],
            user: &arr[5],
            system_program: &arr[6],
            token_program: &arr[7],
            associated_token_program: &arr[8],
            mayhem_program_id: &arr[9],
            global_params: &arr[10],
            sol_vault: &arr[11],
            mayhem_state: &arr[12],
            mayhem_token_vault : &arr[13],
            event_authority: &arr[14],
            program: &arr[15],
        }
    }
}
pub const CREATEV2_IX_DISCM: [u8; 8] = [214,144,76,236,95,139,49,180];
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CreateV2IxArgs {
    pub name: String,
    pub symbol: String,
    pub uri: String,
    pub creator: Pubkey,
    pub is_mayhem_mode: bool,
}
#[derive(Clone, Debug, PartialEq)]
pub struct CreateV2IxData(pub CreateV2IxArgs);
impl From<CreateV2IxArgs> for CreateV2IxData {
    fn from(args: CreateV2IxArgs) -> Self {
        Self(args)
    }
}
impl CreateV2IxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != CREATEV2_IX_DISCM {
            return Err(
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!(
                        "discm does not match. Expected: {:?}. Received: {:?}",
                        CREATEV2_IX_DISCM, maybe_discm
                    ),
                ),
            );
        }
        Ok(Self(CreateV2IxArgs::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&CREATEV2_IX_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn createv2_ix_with_program_id(
    program_id: Pubkey,
    keys: CreateV2Keys,
    args: CreateV2IxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; CREATEV2_IX_ACCOUNTS_LEN] = keys.into();
    let data: CreateV2IxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn createv2_ix(keys: CreateV2Keys, args: CreateV2IxArgs) -> std::io::Result<Instruction> {
    createv2_ix_with_program_id(crate::ID, keys, args)
}
pub fn createv2_invoke_with_program_id(
    program_id: Pubkey,
    accounts: CreateV2Accounts<'_, '_>,
    args: CreateV2IxArgs,
) -> ProgramResult {
    let keys: CreateV2Keys = accounts.into();
    let ix = createv2_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn createv2_invoke(accounts: CreateV2Accounts<'_, '_>, args: CreateV2IxArgs) -> ProgramResult {
    createv2_invoke_with_program_id(crate::ID, accounts, args)
}
pub fn createv2_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: CreateV2Accounts<'_, '_>,
    args: CreateV2IxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: CreateV2Keys = accounts.into();
    let ix = createv2_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn createv2_invoke_signed(
    accounts: CreateV2Accounts<'_, '_>,
    args: CreateV2IxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    createv2_invoke_signed_with_program_id(crate::ID, accounts, args, seeds)
}
pub fn createv2_verify_account_keys(
    accounts: CreateV2Accounts<'_, '_>,
    keys: CreateV2Keys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.mint.key, keys.mint),
        (*accounts.mint_authority.key, keys.mint_authority),
        (*accounts.bonding_curve.key, keys.bonding_curve),
        (*accounts.associated_bonding_curve.key, keys.associated_bonding_curve),
        (*accounts.global.key, keys.global),
        (*accounts.user.key, keys.user),
        (*accounts.system_program.key, keys.system_program),
        (*accounts.token_program.key, keys.token_program),
        (*accounts.associated_token_program.key, keys.associated_token_program),
        (*accounts.mayhem_program_id.key, keys.mayhem_program_id),
        (*accounts.global_params.key, keys.global_params),
        (*accounts.sol_vault.key, keys.sol_vault),
        (*accounts.mayhem_state.key, keys.mayhem_state),
        (*accounts.mayhem_token_vault.key, keys.mayhem_token_vault),
        (*accounts.event_authority.key, keys.event_authority),
        (*accounts.program.key, keys.program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn createv2_verify_writable_privileges<'me, 'info>(
    accounts: CreateV2Accounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.mint,
        accounts.bonding_curve,
        accounts.associated_bonding_curve,
        accounts.mayhem_program_id,
        accounts.sol_vault,
        accounts.mayhem_state,
        accounts.mayhem_token_vault,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn createv2_verify_signer_privileges<'me, 'info>(
    accounts: CreateV2Accounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.mint, accounts.user] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn createv2_verify_account_privileges<'me, 'info>(
    accounts: CreateV2Accounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    createv2_verify_writable_privileges(accounts)?;
    createv2_verify_signer_privileges(accounts)?;
    Ok(())
}

pub const DISTRIBUTE_CREATOR_FEES_IX_ACCOUNTS_LEN: usize = 7; 
#[derive(Copy, Clone, Debug)]
pub struct DistributeCreatorFeesAccounts<'me, 'info> {
    pub mint: &'me AccountInfo<'info>,
    pub bonding_curve: &'me AccountInfo<'info>,
    pub sharing_config: &'me AccountInfo<'info>,
    pub creator_vault: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct DistributeCreatorFeesKeys {
    pub mint: Pubkey,
    pub bonding_curve: Pubkey,
    pub sharing_config: Pubkey,
    pub creator_vault: Pubkey,
    pub system_program: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}

impl From<DistributeCreatorFeesAccounts<'_, '_>> for DistributeCreatorFeesKeys {
    fn from(accounts: DistributeCreatorFeesAccounts) -> Self {
        Self {
            mint: *accounts.mint.key,
            bonding_curve: *accounts.bonding_curve.key,
            sharing_config: *accounts.sharing_config.key,
            creator_vault: *accounts.creator_vault.key,
            system_program: *accounts.system_program.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}

impl From<DistributeCreatorFeesKeys> for [AccountMeta; DISTRIBUTE_CREATOR_FEES_IX_ACCOUNTS_LEN] {
    fn from(keys: DistributeCreatorFeesKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.bonding_curve,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.sharing_config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.creator_vault,
                is_signer: false,
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
            }
        ]
    }
}

impl From<[Pubkey; DISTRIBUTE_CREATOR_FEES_IX_ACCOUNTS_LEN]> for DistributeCreatorFeesKeys {
    fn from(pubkeys: [Pubkey; DISTRIBUTE_CREATOR_FEES_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            mint: pubkeys[0],
            bonding_curve: pubkeys[1],
            sharing_config: pubkeys[2],
            creator_vault: pubkeys[3],
            system_program: pubkeys[4],
            event_authority: pubkeys[5],
            program: pubkeys[6],
        }
    }
}

impl<'info> From<DistributeCreatorFeesAccounts<'_, 'info>> for [AccountInfo<'info>; DISTRIBUTE_CREATOR_FEES_IX_ACCOUNTS_LEN] {
    fn from(accounts: DistributeCreatorFeesAccounts<'_, 'info>) -> Self {
        [
            accounts.mint.clone(),
            accounts.bonding_curve.clone(),
            accounts.sharing_config.clone(),
            accounts.creator_vault.clone(),
            accounts.system_program.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}

impl<'me, 'info> From<&'me [AccountInfo<'info>; DISTRIBUTE_CREATOR_FEES_IX_ACCOUNTS_LEN]> for DistributeCreatorFeesAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; DISTRIBUTE_CREATOR_FEES_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            mint: &arr[0],
            bonding_curve: &arr[1],
            sharing_config: &arr[2],
            creator_vault: &arr[3],
            system_program: &arr[4],
            event_authority: &arr[5],
            program: &arr[6]
        }
    }
}

pub const DISTRIBUTE_CREATOR_FEES_IX_DISCM: [u8; 8] = [165, 114, 103, 0, 121, 206, 247, 81];
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq, Default)]
pub struct DistributeCreatorFeesIxArgs;

#[derive(Clone, Debug, PartialEq)]
pub struct DistributeCreatorFeesIxData(pub DistributeCreatorFeesIxArgs);

impl From<DistributeCreatorFeesIxArgs> for DistributeCreatorFeesIxData {
    fn from(args: DistributeCreatorFeesIxArgs) -> Self {
        Self(args)
    }
}

impl DistributeCreatorFeesIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != DISTRIBUTE_CREATOR_FEES_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    DISTRIBUTE_CREATOR_FEES_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(DistributeCreatorFeesIxArgs::deserialize(&mut reader)?))
    }

    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&DISTRIBUTE_CREATOR_FEES_IX_DISCM)?;
        self.0.serialize(&mut writer)
    }

    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}

pub fn distribute_creator_fees_ix_with_program_id(
    program_id: Pubkey,
    keys: DistributeCreatorFeesKeys,
    args: DistributeCreatorFeesIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; DISTRIBUTE_CREATOR_FEES_IX_ACCOUNTS_LEN] = keys.into();
    let data: DistributeCreatorFeesIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}

pub fn distribute_creator_fees_ix(keys: DistributeCreatorFeesKeys, args: DistributeCreatorFeesIxArgs) -> std::io::Result<Instruction> {
    distribute_creator_fees_ix_with_program_id(crate::ID, keys, args)
}

pub fn distribute_creator_fees_invoke_with_program_id(
    program_id: Pubkey,
    accounts: DistributeCreatorFeesAccounts<'_, '_>,
    args: DistributeCreatorFeesIxArgs,
) -> ProgramResult {
    let keys: DistributeCreatorFeesKeys = accounts.into();
    let ix = distribute_creator_fees_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}

pub fn distribute_creator_fees_invoke(accounts: DistributeCreatorFeesAccounts<'_, '_>, args: DistributeCreatorFeesIxArgs) -> ProgramResult {
    distribute_creator_fees_invoke_with_program_id(crate::ID, accounts, args)
}

pub fn distribute_creator_fees_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: DistributeCreatorFeesAccounts<'_, '_>,
    args: DistributeCreatorFeesIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: DistributeCreatorFeesKeys = accounts.into();
    let ix = distribute_creator_fees_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}

pub fn distribute_creator_fees_invoke_signed(
    accounts: DistributeCreatorFeesAccounts<'_, '_>,
    args: DistributeCreatorFeesIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    distribute_creator_fees_invoke_signed_with_program_id(crate::ID, accounts, args, seeds)
}
pub fn distribute_creator_fees_verify_account_keys(
    accounts: DistributeCreatorFeesAccounts<'_, '_>,
    keys: DistributeCreatorFeesKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.mint.key, keys.mint),
        (*accounts.bonding_curve.key, keys.bonding_curve),
        (*accounts.sharing_config.key, keys.sharing_config),
        (*accounts.creator_vault.key, keys.creator_vault),
        (*accounts.system_program.key, keys.system_program),
        (*accounts.event_authority.key, keys.event_authority),
        (*accounts.program.key, keys.program)
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}

pub fn distribute_creator_fees_verify_writable_privileges<'me, 'info>(
    accounts: DistributeCreatorFeesAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    if !accounts.creator_vault.is_writable {
        return Err((accounts.creator_vault, ProgramError::InvalidAccountData));
    }
    Ok(())
}

pub fn distribute_creator_fees_verify_signer_privileges<'me, 'info>(
    accounts: DistributeCreatorFeesAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    Ok(())
}

pub fn distribute_creator_fees_verify_account_privileges<'me, 'info>(
    accounts: DistributeCreatorFeesAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    distribute_creator_fees_verify_writable_privileges(accounts)?;
    distribute_creator_fees_verify_signer_privileges(accounts)?;
    Ok(())
}

pub const EXTEND_ACCOUNT_IX_ACCOUNTS_LEN: usize = 5;
#[derive(Copy, Clone, Debug)]
pub struct ExtendAccountAccounts<'me, 'info> {
    pub account: &'me AccountInfo<'info>,
    pub user: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct ExtendAccountKeys {
    pub account: Pubkey,
    pub user: Pubkey,
    pub system_program: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}

impl From<ExtendAccountAccounts<'_, '_>> for ExtendAccountKeys {
    fn from(accounts: ExtendAccountAccounts) -> Self {
        Self {
            account: *accounts.account.key,
            user: *accounts.user.key,
            system_program: *accounts.system_program.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}

impl From<ExtendAccountKeys> for [AccountMeta; EXTEND_ACCOUNT_IX_ACCOUNTS_LEN] {
    fn from(keys: ExtendAccountKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user,
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

impl From<[Pubkey; EXTEND_ACCOUNT_IX_ACCOUNTS_LEN]> for ExtendAccountKeys {
    fn from(pubkeys: [Pubkey; EXTEND_ACCOUNT_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            account: pubkeys[0],
            user: pubkeys[1],
            system_program: pubkeys[2],
            event_authority: pubkeys[3],
            program: pubkeys[4],
        }
    }
}

impl<'info> From<ExtendAccountAccounts<'_, 'info>> for [AccountInfo<'info>; EXTEND_ACCOUNT_IX_ACCOUNTS_LEN] {
    fn from(accounts: ExtendAccountAccounts<'_, 'info>) -> Self {
        [
            accounts.account.clone(),
            accounts.user.clone(),
            accounts.system_program.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}

impl<'me, 'info> From<&'me [AccountInfo<'info>; EXTEND_ACCOUNT_IX_ACCOUNTS_LEN]> for ExtendAccountAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; EXTEND_ACCOUNT_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            account: &arr[0],
            user: &arr[1],
            system_program: &arr[2],
            event_authority: &arr[3],
            program: &arr[4],
        }
    }
}

pub const EXTEND_ACCOUNT_IX_DISCM: [u8; 8] = [234, 102, 194, 203, 150, 72, 62, 229];
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq, Default)]
pub struct ExtendAccountIxArgs;

#[derive(Clone, Debug, PartialEq)]
pub struct ExtendAccountIxData(pub ExtendAccountIxArgs);

impl From<ExtendAccountIxArgs> for ExtendAccountIxData {
    fn from(args: ExtendAccountIxArgs) -> Self {
        Self(args)
    }
}

impl ExtendAccountIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != EXTEND_ACCOUNT_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    EXTEND_ACCOUNT_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(ExtendAccountIxArgs::deserialize(&mut reader)?))
    }

    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&EXTEND_ACCOUNT_IX_DISCM)?;
        self.0.serialize(&mut writer)
    }

    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}

pub fn extend_account_ix_with_program_id(
    program_id: Pubkey,
    keys: ExtendAccountKeys,
    args: ExtendAccountIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; EXTEND_ACCOUNT_IX_ACCOUNTS_LEN] = keys.into();
    let data: ExtendAccountIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}

pub fn extend_account_ix(keys: ExtendAccountKeys, args: ExtendAccountIxArgs) -> std::io::Result<Instruction> {
    extend_account_ix_with_program_id(crate::ID, keys, args)
}

pub fn extend_account_invoke_with_program_id(
    program_id: Pubkey,
    accounts: ExtendAccountAccounts<'_, '_>,
    args: ExtendAccountIxArgs,
) -> ProgramResult {
    let keys: ExtendAccountKeys = accounts.into();
    let ix = extend_account_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}

pub fn extend_account_invoke(accounts: ExtendAccountAccounts<'_, '_>, args: ExtendAccountIxArgs) -> ProgramResult {
    extend_account_invoke_with_program_id(crate::ID, accounts, args)
}

pub fn extend_account_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: ExtendAccountAccounts<'_, '_>,
    args: ExtendAccountIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: ExtendAccountKeys = accounts.into();
    let ix = extend_account_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}

pub fn extend_account_invoke_signed(
    accounts: ExtendAccountAccounts<'_, '_>,
    args: ExtendAccountIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    extend_account_invoke_signed_with_program_id(crate::ID, accounts, args, seeds)
}

pub fn extend_account_verify_account_keys(
    accounts: ExtendAccountAccounts<'_, '_>,
    keys: ExtendAccountKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.account.key, keys.account),
        (*accounts.user.key, keys.user),
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

pub fn extend_account_verify_writable_privileges<'me, 'info>(
    accounts: ExtendAccountAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    if !accounts.account.is_writable {
        return Err((accounts.account, ProgramError::InvalidAccountData));
    }
    Ok(())
}

pub fn extend_account_verify_signer_privileges<'me, 'info>(
    accounts: ExtendAccountAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    if !accounts.user.is_signer {
        return Err((accounts.user, ProgramError::MissingRequiredSignature));
    }
    Ok(())
}

pub fn extend_account_verify_account_privileges<'me, 'info>(
    accounts: ExtendAccountAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    extend_account_verify_writable_privileges(accounts)?;
    extend_account_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const GET_MINIMUM_DISTRIBUTABLE_FEE_IX_ACCOUNTS_LEN: usize = 4;
#[derive(Copy, Clone, Debug)]
pub struct GetMinimumDistributableFeeAccounts<'me, 'info> {
    pub mint: &'me AccountInfo<'info>,
    pub bonding_curve: &'me AccountInfo<'info>,
    pub sharing_config: &'me AccountInfo<'info>,
    pub creator_vault: &'me AccountInfo<'info>,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GetMinimumDistributableFeeKeys {
    pub mint: Pubkey,
    pub bonding_curve: Pubkey,
    pub sharing_config: Pubkey,
    pub creator_vault: Pubkey
}

impl From<GetMinimumDistributableFeeAccounts<'_, '_>> for GetMinimumDistributableFeeKeys {
    fn from(accounts: GetMinimumDistributableFeeAccounts) -> Self {
        Self {
            mint: *accounts.mint.key,
            bonding_curve: *accounts.bonding_curve.key,
            sharing_config: *accounts.sharing_config.key,
            creator_vault: *accounts.creator_vault.key,
        }
    }
}

impl From<GetMinimumDistributableFeeKeys> for [AccountMeta; GET_MINIMUM_DISTRIBUTABLE_FEE_IX_ACCOUNTS_LEN] {
    fn from(keys: GetMinimumDistributableFeeKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.bonding_curve,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.sharing_config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.creator_vault,
                is_signer: false,
                is_writable: true,
            }
        ]
    }
}

impl From<[Pubkey; GET_MINIMUM_DISTRIBUTABLE_FEE_IX_ACCOUNTS_LEN]> for GetMinimumDistributableFeeKeys {
    fn from(pubkeys: [Pubkey; GET_MINIMUM_DISTRIBUTABLE_FEE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            mint: pubkeys[0],
            bonding_curve: pubkeys[1],
            sharing_config: pubkeys[2],
            creator_vault: pubkeys[3],
        }
    }
}

impl<'info> From<GetMinimumDistributableFeeAccounts<'_, 'info>> for [AccountInfo<'info>; GET_MINIMUM_DISTRIBUTABLE_FEE_IX_ACCOUNTS_LEN] {
    fn from(accounts: GetMinimumDistributableFeeAccounts<'_, 'info>) -> Self {
        [
            accounts.mint.clone(),
            accounts.bonding_curve.clone(),
            accounts.sharing_config.clone(),
            accounts.creator_vault.clone(),
        ]
    }
}

impl<'me, 'info> From<&'me [AccountInfo<'info>; GET_MINIMUM_DISTRIBUTABLE_FEE_IX_ACCOUNTS_LEN]> for GetMinimumDistributableFeeAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; GET_MINIMUM_DISTRIBUTABLE_FEE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            mint: &arr[0],
            bonding_curve: &arr[1],
            sharing_config: &arr[2],
            creator_vault: &arr[3]
        }
    }
}
pub const GET_MINIMUM_DISTRIBUTABLE_FEE_IX_DISCM: [u8; 8] = [117, 225, 127, 202, 134, 95, 68, 35];
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq, Default)]
pub struct GetMinimumDistributableFeeIxArgs;

#[derive(Clone, Debug, PartialEq)]
pub struct GetMinimumDistributableFeeIxData(pub GetMinimumDistributableFeeIxArgs);

impl From<GetMinimumDistributableFeeIxArgs> for GetMinimumDistributableFeeIxData {
    fn from(args: GetMinimumDistributableFeeIxArgs) -> Self {
        Self(args)
    }
}

impl GetMinimumDistributableFeeIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != GET_MINIMUM_DISTRIBUTABLE_FEE_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    GET_MINIMUM_DISTRIBUTABLE_FEE_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(GetMinimumDistributableFeeIxArgs::deserialize(&mut reader)?))
    }

    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&GET_MINIMUM_DISTRIBUTABLE_FEE_IX_DISCM)?;
        self.0.serialize(&mut writer)
    }

    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}

pub fn get_minimum_distributable_fee_ix_with_program_id(
    program_id: Pubkey,
    keys: GetMinimumDistributableFeeKeys,
    args: GetMinimumDistributableFeeIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; GET_MINIMUM_DISTRIBUTABLE_FEE_IX_ACCOUNTS_LEN] = keys.into();
    let data: GetMinimumDistributableFeeIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn get_minimum_distributable_fee_ix(keys: GetMinimumDistributableFeeKeys, args: GetMinimumDistributableFeeIxArgs) -> std::io::Result<Instruction> {
    get_minimum_distributable_fee_ix_with_program_id(crate::ID, keys, args)
}
pub fn get_minimum_distributable_fee_invoke_with_program_id(
    program_id: Pubkey,
    accounts: GetMinimumDistributableFeeAccounts<'_, '_>,
    args: GetMinimumDistributableFeeIxArgs,
) -> ProgramResult {
    let keys: GetMinimumDistributableFeeKeys = accounts.into();
    let ix = get_minimum_distributable_fee_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn get_minimum_distributable_fee_invoke(accounts: GetMinimumDistributableFeeAccounts<'_, '_>, args: GetMinimumDistributableFeeIxArgs) -> ProgramResult {
    get_minimum_distributable_fee_invoke_with_program_id(crate::ID, accounts, args)
}

pub fn get_minimum_distributable_fee_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: GetMinimumDistributableFeeAccounts<'_, '_>,
    args: GetMinimumDistributableFeeIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: GetMinimumDistributableFeeKeys = accounts.into();
    let ix = get_minimum_distributable_fee_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}

pub fn get_minimum_distributable_fee_invoke_signed(
    accounts: GetMinimumDistributableFeeAccounts<'_, '_>,
    args: GetMinimumDistributableFeeIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    get_minimum_distributable_fee_invoke_signed_with_program_id(crate::ID, accounts, args, seeds)
}
pub fn get_minimum_distributable_fee_verify_account_keys(
    accounts: GetMinimumDistributableFeeAccounts<'_, '_>,
    keys: GetMinimumDistributableFeeKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.mint.key, keys.mint),
        (*accounts.bonding_curve.key, keys.bonding_curve),
        (*accounts.sharing_config.key, keys.sharing_config),
        (*accounts.creator_vault.key, keys.creator_vault),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}

pub fn get_minimum_distributable_fee_verify_writable_privileges<'me, 'info>(
    accounts: GetMinimumDistributableFeeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    if !accounts.creator_vault.is_writable {
        return Err((accounts.creator_vault, ProgramError::InvalidAccountData));
    }
    Ok(())
}

pub fn get_minimum_distributable_fee_verify_signer_privileges<'me, 'info>(
    accounts: GetMinimumDistributableFeeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    Ok(())
}

pub fn get_minimum_distributable_fee_verify_account_privileges<'me, 'info>(
    accounts: GetMinimumDistributableFeeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    get_minimum_distributable_fee_verify_writable_privileges(accounts)?;
    get_minimum_distributable_fee_verify_signer_privileges(accounts)?;
    Ok(())
}

pub const INIT_USER_VOLUME_ACCUMULATOR_IX_ACCOUNTS_LEN: usize = 6;

#[derive(Copy, Clone, Debug)]
pub struct InitUserVolumeAccumulatorAccounts<'me, 'info> {
    pub payer: &'me AccountInfo<'info>,
    pub user: &'me AccountInfo<'info>,
    pub user_volume_accumulator: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct InitUserVolumeAccumulatorKeys {
    pub payer: Pubkey,
    pub user: Pubkey,
    pub user_volume_accumulator: Pubkey,
    pub system_program: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}

impl From<InitUserVolumeAccumulatorAccounts<'_, '_>> for InitUserVolumeAccumulatorKeys {
    fn from(accounts: InitUserVolumeAccumulatorAccounts) -> Self {
        Self {
            payer: *accounts.payer.key,
            user: *accounts.user.key,
            user_volume_accumulator: *accounts.user_volume_accumulator.key,
            system_program: *accounts.system_program.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}

impl From<InitUserVolumeAccumulatorKeys> for [AccountMeta; INIT_USER_VOLUME_ACCUMULATOR_IX_ACCOUNTS_LEN] {
    fn from(keys: InitUserVolumeAccumulatorKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.payer,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.user_volume_accumulator,
                is_signer: false,
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

impl From<[Pubkey; INIT_USER_VOLUME_ACCUMULATOR_IX_ACCOUNTS_LEN]> for InitUserVolumeAccumulatorKeys {
    fn from(pubkeys: [Pubkey; INIT_USER_VOLUME_ACCUMULATOR_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            payer: pubkeys[0],
            user: pubkeys[1],
            user_volume_accumulator: pubkeys[2],
            system_program: pubkeys[3],
            event_authority: pubkeys[4],
            program: pubkeys[5],
        }
    }
}

impl<'info> From<InitUserVolumeAccumulatorAccounts<'_, 'info>> for [AccountInfo<'info>; INIT_USER_VOLUME_ACCUMULATOR_IX_ACCOUNTS_LEN] {
    fn from(accounts: InitUserVolumeAccumulatorAccounts<'_, 'info>) -> Self {
        [
            accounts.payer.clone(),
            accounts.user.clone(),
            accounts.user_volume_accumulator.clone(),
            accounts.system_program.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}

impl<'me, 'info> From<&'me [AccountInfo<'info>; INIT_USER_VOLUME_ACCUMULATOR_IX_ACCOUNTS_LEN]> for InitUserVolumeAccumulatorAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; INIT_USER_VOLUME_ACCUMULATOR_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            payer: &arr[0],
            user: &arr[1],
            user_volume_accumulator: &arr[2],
            system_program: &arr[3],
            event_authority: &arr[4],
            program: &arr[5],
        }
    }
}

pub const INIT_USER_VOLUME_ACCUMULATOR_IX_DISCM: [u8; 8] = [94, 6, 202, 115, 255, 96, 232, 183];

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InitUserVolumeAccumulatorIxArgs {}

#[derive(Clone, Debug, PartialEq)]
pub struct InitUserVolumeAccumulatorIxData(pub InitUserVolumeAccumulatorIxArgs);

impl From<InitUserVolumeAccumulatorIxArgs> for InitUserVolumeAccumulatorIxData {
    fn from(args: InitUserVolumeAccumulatorIxArgs) -> Self {
        Self(args)
    }
}

impl InitUserVolumeAccumulatorIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != INIT_USER_VOLUME_ACCUMULATOR_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    INIT_USER_VOLUME_ACCUMULATOR_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(InitUserVolumeAccumulatorIxArgs::deserialize(&mut reader)?))
    }

    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&INIT_USER_VOLUME_ACCUMULATOR_IX_DISCM)?;
        self.0.serialize(&mut writer)
    }

    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}

pub fn init_user_volume_accumulator_ix_with_program_id(
    program_id: Pubkey,
    keys: InitUserVolumeAccumulatorKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; INIT_USER_VOLUME_ACCUMULATOR_IX_ACCOUNTS_LEN] = keys.into();
    let data: InitUserVolumeAccumulatorIxData = InitUserVolumeAccumulatorIxArgs {}.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}

pub fn init_user_volume_accumulator_ix(
    keys: InitUserVolumeAccumulatorKeys,
) -> std::io::Result<Instruction> {
    init_user_volume_accumulator_ix_with_program_id(crate::ID, keys)
}

pub fn init_user_volume_accumulator_invoke_with_program_id(
    program_id: Pubkey,
    accounts: InitUserVolumeAccumulatorAccounts<'_, '_>,
) -> ProgramResult {
    let keys: InitUserVolumeAccumulatorKeys = accounts.into();
    let ix = init_user_volume_accumulator_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}

pub fn init_user_volume_accumulator_invoke(
    accounts: InitUserVolumeAccumulatorAccounts<'_, '_>,
) -> ProgramResult {
    init_user_volume_accumulator_invoke_with_program_id(crate::ID, accounts)
}

pub fn init_user_volume_accumulator_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: InitUserVolumeAccumulatorAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: InitUserVolumeAccumulatorKeys = accounts.into();
    let ix = init_user_volume_accumulator_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}

pub fn init_user_volume_accumulator_invoke_signed(
    accounts: InitUserVolumeAccumulatorAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    init_user_volume_accumulator_invoke_signed_with_program_id(crate::ID, accounts, seeds)
}

pub fn init_user_volume_accumulator_verify_account_keys(
    accounts: InitUserVolumeAccumulatorAccounts<'_, '_>,
    keys: InitUserVolumeAccumulatorKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.payer.key, keys.payer),
        (*accounts.user.key, keys.user),
        (*accounts.user_volume_accumulator.key, keys.user_volume_accumulator),
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

pub fn init_user_volume_accumulator_verify_writable_privileges<'me, 'info>(
    accounts: InitUserVolumeAccumulatorAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.payer, accounts.user_volume_accumulator] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}

pub fn init_user_volume_accumulator_verify_signer_privileges<'me, 'info>(
    accounts: InitUserVolumeAccumulatorAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.payer] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}

pub fn init_user_volume_accumulator_verify_account_privileges<'me, 'info>(
    accounts: InitUserVolumeAccumulatorAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    init_user_volume_accumulator_verify_writable_privileges(accounts)?;
    init_user_volume_accumulator_verify_signer_privileges(accounts)?;
    Ok(())
}

pub const INITIALIZE_IX_ACCOUNTS_LEN: usize = 3;
#[derive(Copy, Clone, Debug)]
pub struct InitializeAccounts<'me, 'info> {
    pub global: &'me AccountInfo<'info>,
    pub user: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct InitializeKeys {
    pub global: Pubkey,
    pub user: Pubkey,
    pub system_program: Pubkey,
}
impl From<InitializeAccounts<'_, '_>> for InitializeKeys {
    fn from(accounts: InitializeAccounts) -> Self {
        Self {
            global: *accounts.global.key,
            user: *accounts.user.key,
            system_program: *accounts.system_program.key,
        }
    }
}
impl From<InitializeKeys> for [AccountMeta; INITIALIZE_IX_ACCOUNTS_LEN] {
    fn from(keys: InitializeKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.global,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user,
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
impl From<[Pubkey; INITIALIZE_IX_ACCOUNTS_LEN]> for InitializeKeys {
    fn from(pubkeys: [Pubkey; INITIALIZE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            global: pubkeys[0],
            user: pubkeys[1],
            system_program: pubkeys[2],
        }
    }
}
impl<'info> From<InitializeAccounts<'_, 'info>>
for [AccountInfo<'info>; INITIALIZE_IX_ACCOUNTS_LEN] {
    fn from(accounts: InitializeAccounts<'_, 'info>) -> Self {
        [accounts.global.clone(), accounts.user.clone(), accounts.system_program.clone()]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; INITIALIZE_IX_ACCOUNTS_LEN]>
for InitializeAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; INITIALIZE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            global: &arr[0],
            user: &arr[1],
            system_program: &arr[2],
        }
    }
}
pub const INITIALIZE_IX_DISCM: [u8; 8] = [175, 175, 109, 31, 13, 152, 155, 237];
#[derive(Clone, Debug, PartialEq)]
pub struct InitializeIxData;
impl InitializeIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != INITIALIZE_IX_DISCM {
            return Err(
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!(
                        "discm does not match. Expected: {:?}. Received: {:?}",
                        INITIALIZE_IX_DISCM, maybe_discm
                    ),
                ),
            );
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&INITIALIZE_IX_DISCM)
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
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; INITIALIZE_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: InitializeIxData.try_to_vec()?,
    })
}
pub fn initialize_ix(keys: InitializeKeys) -> std::io::Result<Instruction> {
    initialize_ix_with_program_id(crate::ID, keys)
}
pub fn initialize_invoke_with_program_id(
    program_id: Pubkey,
    accounts: InitializeAccounts<'_, '_>,
) -> ProgramResult {
    let keys: InitializeKeys = accounts.into();
    let ix = initialize_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn initialize_invoke(accounts: InitializeAccounts<'_, '_>) -> ProgramResult {
    initialize_invoke_with_program_id(crate::ID, accounts)
}
pub fn initialize_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: InitializeAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: InitializeKeys = accounts.into();
    let ix = initialize_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn initialize_invoke_signed(
    accounts: InitializeAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    initialize_invoke_signed_with_program_id(crate::ID, accounts, seeds)
}
pub fn initialize_verify_account_keys(
    accounts: InitializeAccounts<'_, '_>,
    keys: InitializeKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.global.key, keys.global),
        (*accounts.user.key, keys.user),
        (*accounts.system_program.key, keys.system_program),
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
    for should_be_writable in [accounts.global, accounts.user] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn initialize_verify_signer_privileges<'me, 'info>(
    accounts: InitializeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.user] {
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

pub const MIGRATE_IX_ACCOUNTS_LEN: usize = 24;

#[derive(Copy, Clone, Debug)]
pub struct MigrateAccounts<'me, 'info> {
    pub global: &'me AccountInfo<'info>,
    pub withdraw_authority: &'me AccountInfo<'info>,
    pub mint: &'me AccountInfo<'info>,
    pub bonding_curve: &'me AccountInfo<'info>,
    pub associated_bonding_curve: &'me AccountInfo<'info>,
    pub user: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub pump_amm: &'me AccountInfo<'info>,
    pub pool: &'me AccountInfo<'info>,
    pub pool_authority: &'me AccountInfo<'info>,
    pub pool_authority_mint_account: &'me AccountInfo<'info>,
    pub pool_authority_wsol_account: &'me AccountInfo<'info>,
    pub amm_global_config: &'me AccountInfo<'info>,
    pub wsol_mint: &'me AccountInfo<'info>,
    pub lp_mint: &'me AccountInfo<'info>,
    pub user_pool_token_account: &'me AccountInfo<'info>,
    pub pool_base_token_account: &'me AccountInfo<'info>,
    pub pool_quote_token_account: &'me AccountInfo<'info>,
    pub token_2022_program: &'me AccountInfo<'info>,
    pub associated_token_program: &'me AccountInfo<'info>,
    pub pump_amm_event_authority: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MigrateKeys {
    pub global: Pubkey,
    pub withdraw_authority: Pubkey,
    pub mint: Pubkey,
    pub bonding_curve: Pubkey,
    pub associated_bonding_curve: Pubkey,
    pub user: Pubkey,
    pub system_program: Pubkey,
    pub token_program: Pubkey,
    pub pump_amm: Pubkey,
    pub pool: Pubkey,
    pub pool_authority: Pubkey,
    pub pool_authority_mint_account: Pubkey,
    pub pool_authority_wsol_account: Pubkey,
    pub amm_global_config: Pubkey,
    pub wsol_mint: Pubkey,
    pub lp_mint: Pubkey,
    pub user_pool_token_account: Pubkey,
    pub pool_base_token_account: Pubkey,
    pub pool_quote_token_account: Pubkey,
    pub token_2022_program: Pubkey,
    pub associated_token_program: Pubkey,
    pub pump_amm_event_authority: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}

impl From<MigrateAccounts<'_, '_>> for MigrateKeys {
    fn from(accounts: MigrateAccounts) -> Self {
        Self {
            global: *accounts.global.key,
            withdraw_authority: *accounts.withdraw_authority.key,
            mint: *accounts.mint.key,
            bonding_curve: *accounts.bonding_curve.key,
            associated_bonding_curve: *accounts.associated_bonding_curve.key,
            user: *accounts.user.key,
            system_program: *accounts.system_program.key,
            token_program: *accounts.token_program.key,
            pump_amm: *accounts.pump_amm.key,
            pool: *accounts.pool.key,
            pool_authority: *accounts.pool_authority.key,
            pool_authority_mint_account: *accounts.pool_authority_mint_account.key,
            pool_authority_wsol_account: *accounts.pool_authority_wsol_account.key,
            amm_global_config: *accounts.amm_global_config.key,
            wsol_mint: *accounts.wsol_mint.key,
            lp_mint: *accounts.lp_mint.key,
            user_pool_token_account: *accounts.user_pool_token_account.key,
            pool_base_token_account: *accounts.pool_base_token_account.key,
            pool_quote_token_account: *accounts.pool_quote_token_account.key,
            token_2022_program: *accounts.token_2022_program.key,
            associated_token_program: *accounts.associated_token_program.key,
            pump_amm_event_authority: *accounts.pump_amm_event_authority.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<MigrateKeys> for [AccountMeta; MIGRATE_IX_ACCOUNTS_LEN] {
    fn from(keys: MigrateKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.global,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.withdraw_authority,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.bonding_curve,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.associated_bonding_curve,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user,
                is_signer: true,
                is_writable: true,
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
                pubkey: keys.pump_amm,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pool,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.pool_authority,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.pool_authority_mint_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.pool_authority_wsol_account,
                is_signer: false,
                is_writable: true
            },
             AccountMeta {
                 pubkey : keys.amm_global_config ,
                 is_signer : false ,
                 is_writable : false
             },
             AccountMeta {
                 pubkey : keys.wsol_mint ,
                 is_signer : false ,
                 is_writable : false
             },
             AccountMeta {
                 pubkey : keys.lp_mint ,
                 is_signer : false ,
                 is_writable : true
             },
             AccountMeta {
                 pubkey : keys.user_pool_token_account ,
                 is_signer : false ,
                 is_writable : true
             },
             AccountMeta {
                 pubkey : keys.pool_base_token_account ,
                 is_signer : false ,
                 is_writable : true
             },
             AccountMeta {
                 pubkey : keys.pool_quote_token_account ,
                 is_signer : false ,
                 is_writable : true
             },
             AccountMeta {
                 pubkey : keys.token_2022_program ,
                 is_signer : false ,
                 is_writable : false
             },
             AccountMeta {
                 pubkey : keys.associated_token_program ,
                 is_signer : false ,
                 is_writable : false
             },
             AccountMeta {
                 pubkey : keys.pump_amm_event_authority ,
                 is_signer : false ,
                 is_writable : false
             },
             AccountMeta {
                 pubkey : keys.event_authority ,
                 is_signer : false ,
                 is_writable : false
             },
             AccountMeta {
                 pubkey : keys.program ,
                 is_signer : false ,
                 is_writable : false
             },
        ]
    }
}

impl From<[Pubkey; MIGRATE_IX_ACCOUNTS_LEN]> for MigrateKeys {
    fn from(pubkeys: [Pubkey; MIGRATE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            global: pubkeys[0],
            withdraw_authority: pubkeys[1],
            mint: pubkeys[2],
            bonding_curve: pubkeys[3],
            associated_bonding_curve: pubkeys[4],
            user: pubkeys[5],
            system_program: pubkeys[6],
            token_program: pubkeys[7],
            pump_amm: pubkeys[8],
            pool: pubkeys[9],
            pool_authority: pubkeys[10],
            pool_authority_mint_account: pubkeys[11],
            pool_authority_wsol_account: pubkeys[12],
            amm_global_config: pubkeys[13],
            wsol_mint: pubkeys[14],
            lp_mint: pubkeys[15],
            user_pool_token_account: pubkeys[16],
            pool_base_token_account: pubkeys[17],
            pool_quote_token_account: pubkeys[18],
            token_2022_program: pubkeys[19],
            associated_token_program: pubkeys[20],
            pump_amm_event_authority: pubkeys[21],
            event_authority: pubkeys[22],
            program: pubkeys[23],
        }
    }
}

impl<'info> From<MigrateAccounts<'_, 'info>> for [AccountInfo<'info>; MIGRATE_IX_ACCOUNTS_LEN] {
    fn from(accounts: MigrateAccounts<'_, 'info>) -> Self {
        [
            accounts.global.clone(),
            accounts.withdraw_authority.clone(),
            accounts.mint.clone(),
            accounts.bonding_curve.clone(),
            accounts.associated_bonding_curve.clone(),
            accounts.user.clone(),
            accounts.system_program.clone(),
            accounts.token_program.clone(),
            accounts.pump_amm.clone(),
            accounts.pool.clone(),
            accounts.pool_authority.clone(),
            accounts.pool_authority_mint_account.clone(),
            accounts.pool_authority_wsol_account.clone(),
            accounts.amm_global_config.clone(),
            accounts.wsol_mint.clone(),
            accounts.lp_mint.clone(),
            accounts.user_pool_token_account.clone(),
            accounts.pool_base_token_account.clone(),
            accounts.pool_quote_token_account.clone(),
            accounts.token_2022_program.clone(),
            accounts.associated_token_program.clone(),
            accounts.pump_amm_event_authority.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}

impl<'me, 'info> From<&'me [AccountInfo<'info>; MIGRATE_IX_ACCOUNTS_LEN]> for MigrateAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; MIGRATE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            global: &arr[0],
            withdraw_authority: &arr[1],
            mint: &arr[2],
            bonding_curve: &arr[3],
            associated_bonding_curve: &arr[4],
            user: &arr[5],
            system_program: &arr[6],
            token_program: &arr[7],
            pump_amm: &arr[8],
            pool: &arr[9],
            pool_authority: &arr[10],
            pool_authority_mint_account: &arr[11],
            pool_authority_wsol_account: &arr[12],
            amm_global_config: &arr[13],
            wsol_mint: &arr[14],
            lp_mint: &arr[15],
            user_pool_token_account: &arr[16],
            pool_base_token_account: &arr[17],
            pool_quote_token_account: &arr[18],
            token_2022_program: &arr[19],
            associated_token_program: &arr[20],
            pump_amm_event_authority: &arr[21],
            event_authority: &arr[22],
            program: &arr[23],
        }
    }
}

pub const MIGRATE_IX_DISCM: [u8; 8] = [155, 234, 231, 146, 236, 158, 162, 30];

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq, Default)]
pub struct MigrateIxArgs;

#[derive(Clone, Debug, PartialEq)]
pub struct MigrateIxData(pub MigrateIxArgs);

impl From<MigrateIxArgs> for MigrateIxData {
    fn from(args: MigrateIxArgs) -> Self {
        Self(args)
    }
}

impl MigrateIxData {
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&MIGRATE_IX_DISCM)?;
        self.0.serialize(&mut writer)
    }

    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }

    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut discm = [0u8; 8];
        reader.read_exact(&mut discm)?;
        if discm != MIGRATE_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discriminator mismatch. Expected {:?}, got {:?}",
                    MIGRATE_IX_DISCM, discm
                ),
            ));
        }
        Ok(Self(MigrateIxArgs::deserialize(&mut reader)?))
    }
}

pub fn migrate_ix_with_program_id(
    program_id: Pubkey,
    keys: MigrateKeys,
    args: MigrateIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; MIGRATE_IX_ACCOUNTS_LEN] = keys.into();
    let data: MigrateIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}

pub fn migrate_ix(keys: MigrateKeys, args: MigrateIxArgs) -> std::io::Result<Instruction> {
    migrate_ix_with_program_id(crate::ID, keys, args)
}

pub fn migrate_invoke_with_program_id(
    program_id: Pubkey,
    accounts: MigrateAccounts<'_, '_>,
    args: MigrateIxArgs,
) -> ProgramResult {
    let ix = migrate_ix_with_program_id(program_id, accounts.into(), args)?;
    invoke_instruction(&ix, accounts)
}

pub fn migrate_invoke(accounts: MigrateAccounts<'_, '_>, args: MigrateIxArgs) -> ProgramResult {
    migrate_invoke_with_program_id(crate::ID, accounts, args)
}

pub fn migrate_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: MigrateAccounts<'_, '_>,
    args: MigrateIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = migrate_ix_with_program_id(program_id, accounts.into(), args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}

pub fn migrate_invoke_signed(
    accounts: MigrateAccounts<'_, '_>,
    args: MigrateIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    migrate_invoke_signed_with_program_id(crate::ID, accounts, args, seeds)
}
pub fn migrate_verify_account_keys(
    accounts: MigrateAccounts<'_, '_>,
    keys: MigrateKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.global.key, keys.global),
        (*accounts.withdraw_authority.key, keys.withdraw_authority),
        (*accounts.mint.key, keys.mint),
        (*accounts.bonding_curve.key, keys.bonding_curve),
        (*accounts.associated_bonding_curve.key, keys.associated_bonding_curve),
        (*accounts.user.key, keys.user),
        (*accounts.system_program.key, keys.system_program),
        (*accounts.token_program.key, keys.token_program),
        (*accounts.pump_amm.key, keys.pump_amm),
        (*accounts.pool.key, keys.pool),
        (*accounts.pool_authority.key, keys.pool_authority),
        (*accounts.pool_authority_mint_account.key, keys.pool_authority_mint_account),
        (*accounts.pool_authority_wsol_account.key, keys.pool_authority_wsol_account),
        (*accounts.amm_global_config.key, keys.amm_global_config),
        (*accounts.wsol_mint.key, keys.wsol_mint),
        (*accounts.lp_mint.key, keys.lp_mint),
        (*accounts.user_pool_token_account.key, keys.user_pool_token_account),
        (*accounts.pool_base_token_account.key, keys.pool_base_token_account),
        (*accounts.pool_quote_token_account.key, keys.pool_quote_token_account),
        (*accounts.token_2022_program.key, keys.token_2022_program),
        (*accounts.associated_token_program.key, keys.associated_token_program),
        (*accounts.pump_amm_event_authority.key, keys.pump_amm_event_authority),
        (*accounts.event_authority.key, keys.event_authority),
        (*accounts.program.key, keys.program)
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn migrate_verify_writable_privileges<'me, 'info>(
    accounts: MigrateAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.global,
        accounts.withdraw_authority,
        accounts.bonding_curve,
        accounts.associated_bonding_curve,
        accounts.user,
        accounts.pool,
        accounts.pool_authority,
        accounts.pool_authority_mint_account,
        accounts.pool_authority_wsol_account,
        accounts.lp_mint,
        accounts.user_pool_token_account,
        accounts.pool_base_token_account,
        accounts.pool_quote_token_account
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn migrate_verify_signer_privileges<'me, 'info>(
    accounts: MigrateAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.withdraw_authority, accounts.user] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn migrate_verify_account_privileges<'me, 'info>(
    accounts: MigrateAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    migrate_verify_writable_privileges(accounts)?;
    migrate_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const MIGRATE_BONDING_CURVE_CREATOR_IX_ACCOUNTS_LEN: usize = 5; 
#[derive(Copy, Clone, Debug)]
pub struct MigrateBondingCurveCreatorAccounts<'me, 'info> {
    pub mint: &'me AccountInfo<'info>,
    pub bonding_curve: &'me AccountInfo<'info>,
    pub sharing_config: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MigrateBondingCurveCreatorKeys {
    pub mint: Pubkey,
    pub bonding_curve: Pubkey,
    pub sharing_config: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}

impl From<MigrateBondingCurveCreatorAccounts<'_, '_>> for MigrateBondingCurveCreatorKeys {
    fn from(accounts: MigrateBondingCurveCreatorAccounts) -> Self {
        Self {
            mint: *accounts.mint.key,
            bonding_curve: *accounts.bonding_curve.key,
            sharing_config: *accounts.sharing_config.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}

impl From<MigrateBondingCurveCreatorKeys> for [AccountMeta; MIGRATE_BONDING_CURVE_CREATOR_IX_ACCOUNTS_LEN] {
    fn from(keys: MigrateBondingCurveCreatorKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.bonding_curve,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.sharing_config,
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
            }
        ]
    }
}
impl From<[Pubkey; MIGRATE_BONDING_CURVE_CREATOR_IX_ACCOUNTS_LEN]> for MigrateBondingCurveCreatorKeys {
    fn from(pubkeys: [Pubkey; MIGRATE_BONDING_CURVE_CREATOR_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            mint: pubkeys[0],
            bonding_curve: pubkeys[1],
            sharing_config: pubkeys[2],
            event_authority: pubkeys[3],
            program: pubkeys[4],
        }
    }
}
impl<'info> From<MigrateBondingCurveCreatorAccounts<'_, 'info>> for [AccountInfo<'info>; MIGRATE_BONDING_CURVE_CREATOR_IX_ACCOUNTS_LEN] {
    fn from(accounts: MigrateBondingCurveCreatorAccounts<'_, 'info>) -> Self {
        [
            accounts.mint.clone(),
            accounts.bonding_curve.clone(),
            accounts.sharing_config.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; MIGRATE_BONDING_CURVE_CREATOR_IX_ACCOUNTS_LEN]> for MigrateBondingCurveCreatorAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; MIGRATE_BONDING_CURVE_CREATOR_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            mint: &arr[0],
            bonding_curve: &arr[1],
            sharing_config: &arr[2],
            event_authority: &arr[3],
            program: &arr[4]
        }
    }
}
pub const MIGRATE_BONDING_CURVE_CREATOR_IX_DISCM: [u8; 8] = [87, 124, 52, 191, 52, 38, 214, 232];
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq, Default)]
pub struct MigrateBondingCurveCreatorIxArgs;

#[derive(Clone, Debug, PartialEq)]
pub struct MigrateBondingCurveCreatorIxData(pub MigrateBondingCurveCreatorIxArgs);

impl From<MigrateBondingCurveCreatorIxArgs> for MigrateBondingCurveCreatorIxData {
    fn from(args: MigrateBondingCurveCreatorIxArgs) -> Self {
        Self(args)
    }
}

impl MigrateBondingCurveCreatorIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != MIGRATE_BONDING_CURVE_CREATOR_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    MIGRATE_BONDING_CURVE_CREATOR_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(MigrateBondingCurveCreatorIxArgs::deserialize(&mut reader)?))
    }

    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&MIGRATE_BONDING_CURVE_CREATOR_IX_DISCM)?;
        self.0.serialize(&mut writer)
    }

    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}

pub fn migrate_bonding_curve_creator_ix_with_program_id(
    program_id: Pubkey,
    keys: MigrateBondingCurveCreatorKeys,
    args: MigrateBondingCurveCreatorIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; MIGRATE_BONDING_CURVE_CREATOR_IX_ACCOUNTS_LEN] = keys.into();
    let data: MigrateBondingCurveCreatorIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}

pub fn migrate_bonding_curve_creator_ix(keys: MigrateBondingCurveCreatorKeys, args: MigrateBondingCurveCreatorIxArgs) -> std::io::Result<Instruction> {
    migrate_bonding_curve_creator_ix_with_program_id(crate::ID, keys, args)
}

pub fn migrate_bonding_curve_creator_invoke_with_program_id(
    program_id: Pubkey,
    accounts: MigrateBondingCurveCreatorAccounts<'_, '_>,
    args: MigrateBondingCurveCreatorIxArgs,
) -> ProgramResult {
    let keys: MigrateBondingCurveCreatorKeys = accounts.into();
    let ix = migrate_bonding_curve_creator_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}

pub fn migrate_bonding_curve_creator_invoke(accounts: MigrateBondingCurveCreatorAccounts<'_, '_>, args: MigrateBondingCurveCreatorIxArgs) -> ProgramResult {
    migrate_bonding_curve_creator_invoke_with_program_id(crate::ID, accounts, args)
}

pub fn migrate_bonding_curve_creator_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: MigrateBondingCurveCreatorAccounts<'_, '_>,
    args: MigrateBondingCurveCreatorIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: MigrateBondingCurveCreatorKeys = accounts.into();
    let ix = migrate_bonding_curve_creator_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}

pub fn migrate_bonding_curve_creator_invoke_signed(
    accounts: MigrateBondingCurveCreatorAccounts<'_, '_>,
    args: MigrateBondingCurveCreatorIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    migrate_bonding_curve_creator_invoke_signed_with_program_id(crate::ID, accounts, args, seeds)
}
pub fn migrate_bonding_curve_creator_verify_account_keys(
    accounts: MigrateBondingCurveCreatorAccounts<'_, '_>,
    keys: MigrateBondingCurveCreatorKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.mint.key, keys.mint),
        (*accounts.bonding_curve.key, keys.bonding_curve),
        (*accounts.sharing_config.key, keys.sharing_config),
        (*accounts.event_authority.key, keys.event_authority),
        (*accounts.program.key, keys.program)
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}

pub fn migrate_bonding_curve_creator_verify_writable_privileges<'me, 'info>(
    accounts: MigrateBondingCurveCreatorAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    if !accounts.bonding_curve.is_writable {
        return Err((accounts.bonding_curve, ProgramError::InvalidAccountData));
    }
    Ok(())
}

pub fn migrate_bonding_curve_creator_verify_signer_privileges<'me, 'info>(
    accounts: MigrateBondingCurveCreatorAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    Ok(())
}

pub fn migrate_bonding_curve_creator_verify_account_privileges<'me, 'info>(
    accounts: MigrateBondingCurveCreatorAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    migrate_bonding_curve_creator_verify_writable_privileges(accounts)?;
    migrate_bonding_curve_creator_verify_signer_privileges(accounts)?;
    Ok(())
}

pub const SELL_IX_ACCOUNTS_LEN: usize = 14;

#[derive(Copy, Clone, Debug)]
pub struct SellAccounts<'me, 'info> {
    pub global: &'me AccountInfo<'info>,
    pub fee_recipient: &'me AccountInfo<'info>,
    pub mint: &'me AccountInfo<'info>,
    pub bonding_curve: &'me AccountInfo<'info>,
    pub associated_bonding_curve: &'me AccountInfo<'info>,
    pub associated_user: &'me AccountInfo<'info>,
    pub user: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub creator_vault: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
    pub fee_config: &'me AccountInfo<'info>,
    pub fee_program: &'me AccountInfo<'info>,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SellKeys {
    pub global: Pubkey,
    pub fee_recipient: Pubkey,
    pub mint: Pubkey,
    pub bonding_curve: Pubkey,
    pub associated_bonding_curve: Pubkey,
    pub associated_user: Pubkey,
    pub user: Pubkey,
    pub system_program: Pubkey,
    pub creator_vault: Pubkey,
    pub token_program: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
    pub fee_config: Pubkey,
    pub fee_program: Pubkey,
}

impl From<SellAccounts<'_, '_>> for SellKeys {
    fn from(accounts: SellAccounts) -> Self {
        Self {
            global: *accounts.global.key,
            fee_recipient: *accounts.fee_recipient.key,
            mint: *accounts.mint.key,
            bonding_curve: *accounts.bonding_curve.key,
            associated_bonding_curve: *accounts.associated_bonding_curve.key,
            associated_user: *accounts.associated_user.key,
            user: *accounts.user.key,
            system_program: *accounts.system_program.key,
            creator_vault: *accounts.creator_vault.key,
            token_program: *accounts.token_program.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
            fee_config: *accounts.fee_config.key,
            fee_program: *accounts.fee_program.key,
        }
    }
}

impl From<SellKeys> for [AccountMeta; SELL_IX_ACCOUNTS_LEN] {
    fn from(keys: SellKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.global,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.fee_recipient,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.bonding_curve,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.associated_bonding_curve,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.associated_user,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.system_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.creator_vault,
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
            AccountMeta {
                pubkey: keys.fee_config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.fee_program,
                is_signer: false,
                is_writable: false,
            }
        ]
    }
}

impl From<[Pubkey; SELL_IX_ACCOUNTS_LEN]> for SellKeys {
    fn from(pubkeys: [Pubkey; SELL_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            global: pubkeys[0],
            fee_recipient: pubkeys[1],
            mint: pubkeys[2],
            bonding_curve: pubkeys[3],
            associated_bonding_curve: pubkeys[4],
            associated_user: pubkeys[5],
            user: pubkeys[6],
            system_program: pubkeys[7],
            creator_vault: pubkeys[8],
            token_program: pubkeys[9],
            event_authority: pubkeys[10],
            program: pubkeys[11],
            fee_config: pubkeys[12],
            fee_program: pubkeys[13],
        }
    }
}

impl<'info> From<SellAccounts<'_, 'info>> for [AccountInfo<'info>; SELL_IX_ACCOUNTS_LEN] {
    fn from(accounts: SellAccounts<'_, 'info>) -> Self {
        [
            accounts.global.clone(),
            accounts.fee_recipient.clone(),
            accounts.mint.clone(),
            accounts.bonding_curve.clone(),
            accounts.associated_bonding_curve.clone(),
            accounts.associated_user.clone(),
            accounts.user.clone(),
            accounts.system_program.clone(),
            accounts.creator_vault.clone(),
            accounts.token_program.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
            accounts.fee_config.clone(),
            accounts.fee_program.clone(),
        ]
    }
}

impl<'me, 'info> From<&'me [AccountInfo<'info>; SELL_IX_ACCOUNTS_LEN]> for SellAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; SELL_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            global: &arr[0],
            fee_recipient: &arr[1],
            mint: &arr[2],
            bonding_curve: &arr[3],
            associated_bonding_curve: &arr[4],
            associated_user: &arr[5],
            user: &arr[6],
            system_program: &arr[7],
            creator_vault: &arr[8],
            token_program: &arr[9],
            event_authority: &arr[10],
            program: &arr[11],
            fee_config: &arr[12],
            fee_program: &arr[13],
        }
    }
}

pub const SELL_IX_DISCM: [u8; 8] = [51, 230, 133, 164, 1, 127, 131, 173];

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SellIxArgs {
    pub amount: u64,
    pub min_sol_output: u64,
}

#[derive(Clone, Debug, PartialEq)]
pub struct SellIxData(pub SellIxArgs);

impl From<SellIxArgs> for SellIxData {
    fn from(args: SellIxArgs) -> Self {
        Self(args)
    }
}

impl SellIxData {
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&SELL_IX_DISCM)?;
        self.0.serialize(&mut writer)
    }

    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }

    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut discm = [0u8; 8];
        reader.read_exact(&mut discm)?;
        if discm != SELL_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discriminator mismatch. Expected {:?}, got {:?}",
                    SELL_IX_DISCM, discm
                ),
            ));
        }
        Ok(Self(SellIxArgs::deserialize(&mut reader)?))
    }
}

pub fn sell_ix_with_program_id(
    program_id: Pubkey,
    keys: SellKeys,
    args: SellIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; SELL_IX_ACCOUNTS_LEN] = keys.into();
    let data: SellIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}

pub fn sell_ix(keys: SellKeys, args: SellIxArgs) -> std::io::Result<Instruction> {
    sell_ix_with_program_id(crate::ID, keys, args)
}

pub fn sell_invoke_with_program_id(
    program_id: Pubkey,
    accounts: SellAccounts<'_, '_>,
    args: SellIxArgs,
) -> ProgramResult {
    let ix = sell_ix_with_program_id(program_id, accounts.into(), args)?;
    invoke_instruction(&ix, accounts)
}

pub fn sell_invoke(accounts: SellAccounts<'_, '_>, args: SellIxArgs) -> ProgramResult {
    sell_invoke_with_program_id(crate::ID, accounts, args)
}

pub fn sell_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: SellAccounts<'_,'_>,
    args: SellIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = sell_ix_with_program_id(program_id, accounts.into(), args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}

pub fn sell_invoke_signed(
    accounts: SellAccounts<'_, '_>,
    args: SellIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    sell_invoke_signed_with_program_id(crate::ID, accounts, args, seeds)
}

pub fn sell_verify_account_keys(
    accounts: SellAccounts<'_, '_>,
    keys: SellKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.global.key, keys.global),
        (*accounts.fee_recipient.key, keys.fee_recipient),
        (*accounts.mint.key, keys.mint),
        (*accounts.bonding_curve.key, keys.bonding_curve),
        (*accounts.associated_bonding_curve.key, keys.associated_bonding_curve),
        (*accounts.associated_user.key, keys.associated_user),
        (*accounts.user.key, keys.user),
        (*accounts.system_program.key, keys.system_program),
        (*accounts.creator_vault.key, keys.creator_vault),
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

pub fn sell_verify_writable_privileges<'me, 'info>(
    accounts: SellAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.fee_recipient,
        accounts.bonding_curve,
        accounts.associated_bonding_curve,
        accounts.associated_user,
        accounts.user,
        accounts.creator_vault,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}

pub fn sell_verify_signer_privileges<'me, 'info>(
    accounts: SellAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.user] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}

pub fn sell_verify_account_privileges<'me, 'info>(
    accounts: SellAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    sell_verify_writable_privileges(accounts)?;
    sell_verify_signer_privileges(accounts)?;
    Ok(())
}

pub const SET_CREATOR_IX_ACCOUNTS_LEN: usize = 7;

#[derive(Copy, Clone, Debug)]
pub struct SetCreatorAccounts<'me, 'info> {
    pub set_creator_authority: &'me AccountInfo<'info>,
    pub global: &'me AccountInfo<'info>,
    pub mint: &'me AccountInfo<'info>,
    pub metadata: &'me AccountInfo<'info>,
    pub bonding_curve: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SetCreatorKeys {
    pub set_creator_authority: Pubkey,
    pub global: Pubkey,
    pub mint: Pubkey,
    pub metadata: Pubkey,
    pub bonding_curve: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}

impl From<SetCreatorAccounts<'_, '_>> for SetCreatorKeys {
    fn from(accounts: SetCreatorAccounts) -> Self {
        Self {
            set_creator_authority: *accounts.set_creator_authority.key,
            global: *accounts.global.key,
            mint: *accounts.mint.key,
            metadata: *accounts.metadata.key,
            bonding_curve: *accounts.bonding_curve.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}

impl From<SetCreatorKeys> for [AccountMeta; SET_CREATOR_IX_ACCOUNTS_LEN] {
    fn from(keys: SetCreatorKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.set_creator_authority,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.global,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.metadata,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.bonding_curve,
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

impl From<[Pubkey; SET_CREATOR_IX_ACCOUNTS_LEN]> for SetCreatorKeys {
    fn from(pubkeys: [Pubkey; SET_CREATOR_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            set_creator_authority: pubkeys[0],
            global: pubkeys[1],
            mint: pubkeys[2],
            metadata: pubkeys[3],
            bonding_curve: pubkeys[4],
            event_authority: pubkeys[5],
            program: pubkeys[6],
        }
    }
}

impl<'info> From<SetCreatorAccounts<'_, 'info>> for [AccountInfo<'info>; SET_CREATOR_IX_ACCOUNTS_LEN] {
    fn from(accounts: SetCreatorAccounts<'_, 'info>) -> Self {
        [
            accounts.set_creator_authority.clone(),
            accounts.global.clone(),
            accounts.mint.clone(),
            accounts.metadata.clone(),
            accounts.bonding_curve.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}

impl<'me, 'info> From<&'me [AccountInfo<'info>; SET_CREATOR_IX_ACCOUNTS_LEN]> for SetCreatorAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; SET_CREATOR_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            set_creator_authority: &arr[0],
            global: &arr[1],
            mint: &arr[2],
            metadata: &arr[3],
            bonding_curve: &arr[4],
            event_authority: &arr[5],
            program: &arr[6],
        }
    }
}

pub const SET_CREATOR_IX_DISCM: [u8; 8] = [254, 148, 255, 112, 207, 142, 170, 165];

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SetCreatorIxArgs {
    pub creator: Pubkey,
}

#[derive(Clone, Debug, PartialEq)]
pub struct SetCreatorIxData(pub SetCreatorIxArgs);

impl From<SetCreatorIxArgs> for SetCreatorIxData {
    fn from(args: SetCreatorIxArgs) -> Self {
        Self(args)
    }
}

impl SetCreatorIxData {
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&SET_CREATOR_IX_DISCM)?;
        self.0.serialize(&mut writer)
    }

    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }

    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut discm = [0u8; 8];
        reader.read_exact(&mut discm)?;
        if discm != SET_CREATOR_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discriminator mismatch. Expected {:?}, got {:?}",
                    SET_CREATOR_IX_DISCM, discm
                ),
            ));
        }
        Ok(Self(SetCreatorIxArgs::deserialize(&mut reader)?))
    }
}

pub fn set_creator_ix_with_program_id(
    program_id: Pubkey,
    keys: SetCreatorKeys,
    args: SetCreatorIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; SET_CREATOR_IX_ACCOUNTS_LEN] = keys.into();
    let data: SetCreatorIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}

pub fn set_creator_ix(keys: SetCreatorKeys, args: SetCreatorIxArgs) -> std::io::Result<Instruction> {
    set_creator_ix_with_program_id(crate::ID, keys, args)
}

pub fn set_creator_invoke_with_program_id(
    program_id: Pubkey,
    accounts: SetCreatorAccounts<'_, '_>,
    args: SetCreatorIxArgs,
) -> ProgramResult {
    let ix = set_creator_ix_with_program_id(program_id, accounts.into(), args)?;
    invoke_instruction(&ix, accounts)
}

pub fn set_creator_invoke(accounts: SetCreatorAccounts<'_, '_>, args: SetCreatorIxArgs) -> ProgramResult {
    set_creator_invoke_with_program_id(crate::ID, accounts, args)
}

pub fn set_creator_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: SetCreatorAccounts<'_, '_>,
    args: SetCreatorIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = set_creator_ix_with_program_id(program_id, accounts.into(), args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}

pub fn set_creator_invoke_signed(
    accounts: SetCreatorAccounts<'_, '_>,
    args: SetCreatorIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    set_creator_invoke_signed_with_program_id(crate::ID, accounts, args, seeds)
}

pub fn set_creator_verify_account_keys(
    accounts: SetCreatorAccounts<'_, '_>,
    keys: SetCreatorKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.set_creator_authority.key, keys.set_creator_authority),
        (*accounts.global.key, keys.global),
        (*accounts.mint.key, keys.mint),
        (*accounts.metadata.key, keys.metadata),
        (*accounts.bonding_curve.key, keys.bonding_curve),
        (*accounts.event_authority.key, keys.event_authority),
        (*accounts.program.key, keys.program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}

pub fn set_creator_verify_writable_privileges<'me, 'info>(
    accounts: SetCreatorAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.bonding_curve] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}

pub fn set_creator_verify_signer_privileges<'me, 'info>(
    accounts: SetCreatorAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.set_creator_authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}

pub fn set_creator_verify_account_privileges<'me, 'info>(
    accounts: SetCreatorAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    set_creator_verify_writable_privileges(accounts)?;
    set_creator_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const SET_MAYHEM_VIRTUAL_PARAMS_IX_ACCOUNTS_LEN: usize = 8;
#[derive(Copy, Clone, Debug)]
pub struct SetMayhemVirtualParamsAccounts<'me, 'info> {
    pub sol_vault_authority: &'me AccountInfo<'info>,
    pub mayhem_token_vault: &'me AccountInfo<'info>,
    pub mint: &'me AccountInfo<'info>,
    pub global: &'me AccountInfo<'info>,
    pub bonding_curve: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SetMayhemVirtualParamsKeys {
    pub sol_vault_authority: Pubkey,
    pub mayhem_token_vault: Pubkey,
    pub mint: Pubkey,
    pub global: Pubkey,
    pub bonding_curve: Pubkey,
    pub token_program: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}

impl From<SetMayhemVirtualParamsAccounts<'_, '_>> for SetMayhemVirtualParamsKeys {
    fn from(accounts: SetMayhemVirtualParamsAccounts) -> Self {
        Self {
            sol_vault_authority: *accounts.sol_vault_authority.key,
            mayhem_token_vault: *accounts.mayhem_token_vault.key,
            mint: *accounts.mint.key,
            global: *accounts.global.key,
            bonding_curve: *accounts.bonding_curve.key,
            token_program: *accounts.token_program.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}

impl From<SetMayhemVirtualParamsKeys> for [AccountMeta; SET_MAYHEM_VIRTUAL_PARAMS_IX_ACCOUNTS_LEN] {
    fn from(keys: SetMayhemVirtualParamsKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.sol_vault_authority,
                is_signer: true,
                is_writable: true
            },
            AccountMeta {
                pubkey: keys.mayhem_token_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.global,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.bonding_curve,
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

impl From<[Pubkey; SET_MAYHEM_VIRTUAL_PARAMS_IX_ACCOUNTS_LEN]> for SetMayhemVirtualParamsKeys {
    fn from(pubkeys: [Pubkey; SET_MAYHEM_VIRTUAL_PARAMS_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            sol_vault_authority: pubkeys[0],
            mayhem_token_vault: pubkeys[1],
            mint: pubkeys[2],
            global: pubkeys[3],
            bonding_curve: pubkeys[4],
            token_program: pubkeys[5],
            event_authority: pubkeys[6],
            program: pubkeys[7]
        }
    }
}

impl<'info> From<SetMayhemVirtualParamsAccounts<'_, 'info>> for [AccountInfo<'info>; SET_MAYHEM_VIRTUAL_PARAMS_IX_ACCOUNTS_LEN] {
    fn from(accounts: SetMayhemVirtualParamsAccounts<'_, 'info>) -> Self {
        [
            accounts.sol_vault_authority.clone(),
            accounts.mayhem_token_vault.clone(),
            accounts.mint.clone(),
            accounts.global.clone(),
            accounts.bonding_curve.clone(),
            accounts.token_program.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone()
        ]
    }
}

impl<'me, 'info> From<&'me [AccountInfo<'info>; SET_MAYHEM_VIRTUAL_PARAMS_IX_ACCOUNTS_LEN]> for SetMayhemVirtualParamsAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; SET_MAYHEM_VIRTUAL_PARAMS_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            sol_vault_authority: &arr[0],
            mayhem_token_vault: &arr[1],
            mint: &arr[2],
            global: &arr[3],
            bonding_curve: &arr[4],
            token_program: &arr[5],
            event_authority: &arr[6],
            program: &arr[7],
        }
    }
}
pub const SET_MAYHEM_VIRTUAL_PARAMS_IX_DISCM: [u8; 8] = [61, 169, 188, 191, 153, 149, 42, 97];

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq, Default)]
pub struct SetMayhemVirtualParamsIxArgs;

#[derive(Clone, Debug, PartialEq)]
pub struct SetMayhemVirtualParamsIxData(pub SetMayhemVirtualParamsIxArgs);

impl From<SetMayhemVirtualParamsIxArgs> for SetMayhemVirtualParamsIxData {
    fn from(args: SetMayhemVirtualParamsIxArgs) -> Self {
        Self(args)
    }
}

impl SetMayhemVirtualParamsIxData {
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&SET_MAYHEM_VIRTUAL_PARAMS_IX_DISCM)?;
        self.0.serialize(&mut writer)
    }

    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }

    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut discm = [0u8; 8];
        reader.read_exact(&mut discm)?;
        if discm != SET_MAYHEM_VIRTUAL_PARAMS_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discriminator mismatch. Expected {:?}, got {:?}",
                    SET_MAYHEM_VIRTUAL_PARAMS_IX_DISCM, discm
                ),
            ));
        }
        Ok(Self(SetMayhemVirtualParamsIxArgs::deserialize(&mut reader)?))
    }
}

pub fn set_mayhem_virtual_params_ix_with_program_id(
    program_id: Pubkey,
    keys: SetMayhemVirtualParamsKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; SET_MAYHEM_VIRTUAL_PARAMS_IX_ACCOUNTS_LEN] = keys.into();
    let data: SetMayhemVirtualParamsIxData = SetMayhemVirtualParamsIxArgs.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}

pub fn set_mayhem_virtual_params_ix(keys: SetMayhemVirtualParamsKeys) -> std::io::Result<Instruction> {
    set_mayhem_virtual_params_ix_with_program_id(crate::ID, keys)
}

pub fn set_mayhem_virtual_params_invoke_with_program_id(
    program_id: Pubkey,
    accounts: SetMayhemVirtualParamsAccounts<'_, '_>,
) -> ProgramResult {
    let ix = set_mayhem_virtual_params_ix_with_program_id(program_id, accounts.into())?;
    invoke_instruction(&ix, accounts)
}

pub fn set_mayhem_virtual_params_invoke(accounts: SetMayhemVirtualParamsAccounts<'_, '_>) -> ProgramResult {
    set_mayhem_virtual_params_invoke_with_program_id(crate::ID, accounts)
}

pub fn set_mayhem_virtual_params_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: SetMayhemVirtualParamsAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = set_mayhem_virtual_params_ix_with_program_id(program_id, accounts.into())?;
    invoke_instruction_signed(&ix, accounts, seeds)
}

pub fn set_mayhem_virtual_params_invoke_signed(
    accounts: SetMayhemVirtualParamsAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    set_mayhem_virtual_params_invoke_signed_with_program_id(crate::ID, accounts, seeds)
}

pub fn set_mayhem_virtual_params_verify_account_keys(
    accounts: SetMayhemVirtualParamsAccounts<'_, '_>,
    keys: SetMayhemVirtualParamsKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.sol_vault_authority.key, keys.sol_vault_authority),
        (*accounts.mayhem_token_vault.key, keys.mayhem_token_vault),
        (*accounts.mint.key, keys.mint),
        (*accounts.global.key, keys.global),
        (*accounts.bonding_curve.key, keys.bonding_curve),
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

pub fn set_mayhem_virtual_params_verify_writable_privileges<'me, 'info>(
    accounts: SetMayhemVirtualParamsAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.sol_vault_authority,accounts.mayhem_token_vault,accounts.bonding_curve] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}

pub fn set_mayhem_virtual_params_verify_signer_privileges<'me, 'info>(
    accounts: SetMayhemVirtualParamsAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.sol_vault_authority]{
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}

pub fn set_mayhem_virtual_params_verify_account_privileges<'me, 'info>(
    accounts: SetMayhemVirtualParamsAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    set_mayhem_virtual_params_verify_writable_privileges(accounts)?;
    set_mayhem_virtual_params_verify_signer_privileges(accounts)?;
    Ok(())
}


pub const SET_METAPLEX_CREATOR_IX_ACCOUNTS_LEN: usize = 5;
#[derive(Copy, Clone, Debug)]
pub struct SetMetaplexCreatorAccounts<'me, 'info> {
    pub mint: &'me AccountInfo<'info>,
    pub metadata: &'me AccountInfo<'info>,
    pub bonding_curve: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SetMetaplexCreatorKeys {
    pub mint: Pubkey,
    pub metadata: Pubkey,
    pub bonding_curve: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}

impl From<SetMetaplexCreatorAccounts<'_, '_>> for SetMetaplexCreatorKeys {
    fn from(accounts: SetMetaplexCreatorAccounts) -> Self {
        Self {
            mint: *accounts.mint.key,
            metadata: *accounts.metadata.key,
            bonding_curve: *accounts.bonding_curve.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}

impl From<SetMetaplexCreatorKeys> for [AccountMeta; SET_METAPLEX_CREATOR_IX_ACCOUNTS_LEN] {
    fn from(keys: SetMetaplexCreatorKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.metadata,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.bonding_curve,
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

impl From<[Pubkey; SET_METAPLEX_CREATOR_IX_ACCOUNTS_LEN]> for SetMetaplexCreatorKeys {
    fn from(pubkeys: [Pubkey; SET_METAPLEX_CREATOR_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            mint: pubkeys[0],
            metadata: pubkeys[1],
            bonding_curve: pubkeys[2],
            event_authority: pubkeys[3],
            program: pubkeys[4],
        }
    }
}

impl<'info> From<SetMetaplexCreatorAccounts<'_, 'info>> for [AccountInfo<'info>; SET_METAPLEX_CREATOR_IX_ACCOUNTS_LEN] {
    fn from(accounts: SetMetaplexCreatorAccounts<'_, 'info>) -> Self {
        [
            accounts.mint.clone(),
            accounts.metadata.clone(),
            accounts.bonding_curve.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}

impl<'me, 'info> From<&'me [AccountInfo<'info>; SET_METAPLEX_CREATOR_IX_ACCOUNTS_LEN]> for SetMetaplexCreatorAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; SET_METAPLEX_CREATOR_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            mint: &arr[0],
            metadata: &arr[1],
            bonding_curve: &arr[2],
            event_authority: &arr[3],
            program: &arr[4],
        }
    }
}

pub const SET_METAPLEX_CREATOR_IX_DISCM: [u8; 8] = [138, 96, 174, 217, 48, 85, 197, 246];

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq, Default)]
pub struct SetMetaplexCreatorIxArgs;

#[derive(Clone, Debug, PartialEq)]
pub struct SetMetaplexCreatorIxData(pub SetMetaplexCreatorIxArgs);

impl From<SetMetaplexCreatorIxArgs> for SetMetaplexCreatorIxData {
    fn from(args: SetMetaplexCreatorIxArgs) -> Self {
        Self(args)
    }
}

impl SetMetaplexCreatorIxData {
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&SET_METAPLEX_CREATOR_IX_DISCM)?;
        self.0.serialize(&mut writer)
    }

    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }

    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut discm = [0u8; 8];
        reader.read_exact(&mut discm)?;
        if discm != SET_METAPLEX_CREATOR_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discriminator mismatch. Expected {:?}, got {:?}",
                    SET_METAPLEX_CREATOR_IX_DISCM, discm
                ),
            ));
        }
        Ok(Self(SetMetaplexCreatorIxArgs::deserialize(&mut reader)?))
    }
}

pub fn set_metaplex_creator_ix_with_program_id(
    program_id: Pubkey,
    keys: SetMetaplexCreatorKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; SET_METAPLEX_CREATOR_IX_ACCOUNTS_LEN] = keys.into();
    let data: SetMetaplexCreatorIxData = SetMetaplexCreatorIxArgs.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}

pub fn set_metaplex_creator_ix(keys: SetMetaplexCreatorKeys) -> std::io::Result<Instruction> {
    set_metaplex_creator_ix_with_program_id(crate::ID, keys)
}

pub fn set_metaplex_creator_invoke_with_program_id(
    program_id: Pubkey,
    accounts: SetMetaplexCreatorAccounts<'_, '_>,
) -> ProgramResult {
    let ix = set_metaplex_creator_ix_with_program_id(program_id, accounts.into())?;
    invoke_instruction(&ix, accounts)
}

pub fn set_metaplex_creator_invoke(accounts: SetMetaplexCreatorAccounts<'_, '_>) -> ProgramResult {
    set_metaplex_creator_invoke_with_program_id(crate::ID, accounts)
}

pub fn set_metaplex_creator_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: SetMetaplexCreatorAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = set_metaplex_creator_ix_with_program_id(program_id, accounts.into())?;
    invoke_instruction_signed(&ix, accounts, seeds)
}

pub fn set_metaplex_creator_invoke_signed(
    accounts: SetMetaplexCreatorAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    set_metaplex_creator_invoke_signed_with_program_id(crate::ID, accounts, seeds)
}

pub fn set_metaplex_creator_verify_account_keys(
    accounts: SetMetaplexCreatorAccounts<'_, '_>,
    keys: SetMetaplexCreatorKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.mint.key, keys.mint),
        (*accounts.metadata.key, keys.metadata),
        (*accounts.bonding_curve.key, keys.bonding_curve),
        (*accounts.event_authority.key, keys.event_authority),
        (*accounts.program.key, keys.program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}

pub fn set_metaplex_creator_verify_writable_privileges<'me, 'info>(
    accounts: SetMetaplexCreatorAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.bonding_curve] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}

pub fn set_metaplex_creator_verify_signer_privileges<'me, 'info>(
    accounts: SetMetaplexCreatorAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    Ok(())
}

pub fn set_metaplex_creator_verify_account_privileges<'me, 'info>(
    accounts: SetMetaplexCreatorAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    set_metaplex_creator_verify_writable_privileges(accounts)?;
    set_metaplex_creator_verify_signer_privileges(accounts)?;
    Ok(())
}

pub const SET_PARAMS_IX_ACCOUNTS_LEN: usize = 4;

#[derive(Copy, Clone, Debug)]
pub struct SetParamsAccounts<'me, 'info> {
    pub global: &'me AccountInfo<'info>,
    pub authority: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SetParamsKeys {
    pub global: Pubkey,
    pub authority: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}

impl From<SetParamsAccounts<'_, '_>> for SetParamsKeys {
    fn from(accounts: SetParamsAccounts) -> Self {
        Self {
            global: *accounts.global.key,
            authority: *accounts.authority.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}

impl From<SetParamsKeys> for [AccountMeta; SET_PARAMS_IX_ACCOUNTS_LEN] {
    fn from(keys: SetParamsKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.global,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.authority,
                is_signer: true,
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

impl From<[Pubkey; SET_PARAMS_IX_ACCOUNTS_LEN]> for SetParamsKeys {
    fn from(pubkeys: [Pubkey; SET_PARAMS_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            global: pubkeys[0],
            authority: pubkeys[1],
            event_authority: pubkeys[2],
            program: pubkeys[3],
        }
    }
}

impl<'info> From<SetParamsAccounts<'_, 'info>> for [AccountInfo<'info>; SET_PARAMS_IX_ACCOUNTS_LEN] {
    fn from(accounts: SetParamsAccounts<'_, 'info>) -> Self {
        [
            accounts.global.clone(),
            accounts.authority.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}

impl<'me, 'info> From<&'me [AccountInfo<'info>; SET_PARAMS_IX_ACCOUNTS_LEN]> for SetParamsAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; SET_PARAMS_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            global: &arr[0],
            authority: &arr[1],
            event_authority: &arr[2],
            program: &arr[3],
        }
    }
}

pub const SET_PARAMS_IX_DISCM: [u8; 8] = [27, 234, 178, 52, 147, 2, 187, 141];

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SetParamsIxArgs {
    pub initial_virtual_token_reserves: u64,
    pub initial_virtual_sol_reserves: u64,
    pub initial_real_token_reserves: u64,
    pub token_total_supply: u64,
    pub fee_basis_points: u64,
    pub withdraw_authority: Pubkey,
    pub enable_migrate: bool,
    pub pool_migration_fee: u64,
    pub creator_fee_basis_points: u64,
    pub set_creator_authority: Pubkey,
}

#[derive(Clone, Debug, PartialEq)]
pub struct SetParamsIxData(pub SetParamsIxArgs);

impl From<SetParamsIxArgs> for SetParamsIxData {
    fn from(args: SetParamsIxArgs) -> Self {
        Self(args)
    }
}

impl SetParamsIxData {
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&SET_PARAMS_IX_DISCM)?;
        self.0.serialize(&mut writer)
    }

    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }

    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut discm = [0u8; 8];
        reader.read_exact(&mut discm)?;
        if discm != SET_PARAMS_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discriminator mismatch. Expected {:?}, got {:?}",
                    SET_PARAMS_IX_DISCM, discm
                ),
            ));
        }
        Ok(Self(SetParamsIxArgs::deserialize(&mut reader)?))
    }
}

pub fn set_params_ix_with_program_id(
    program_id: Pubkey,
    keys: SetParamsKeys,
    args: SetParamsIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; SET_PARAMS_IX_ACCOUNTS_LEN] = keys.into();
    let data: SetParamsIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}

pub fn set_params_ix(keys: SetParamsKeys, args: SetParamsIxArgs) -> std::io::Result<Instruction> {
    set_params_ix_with_program_id(crate::ID, keys, args)
}

pub fn set_params_invoke_with_program_id(
    program_id: Pubkey,
    accounts: SetParamsAccounts<'_, '_>,
    args: SetParamsIxArgs,
) -> ProgramResult {
    let ix = set_params_ix_with_program_id(program_id, accounts.into(), args)?;
    invoke_instruction(&ix, accounts)
}

pub fn set_params_invoke(accounts: SetParamsAccounts<'_, '_>, args: SetParamsIxArgs) -> ProgramResult {
    set_params_invoke_with_program_id(crate::ID, accounts, args)
}

pub fn set_params_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: SetParamsAccounts<'_, '_>,
    args: SetParamsIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = set_params_ix_with_program_id(program_id, accounts.into(), args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}

pub fn set_params_invoke_signed(
    accounts: SetParamsAccounts<'_, '_>,
    args: SetParamsIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    set_params_invoke_signed_with_program_id(crate::ID, accounts, args, seeds)
}

pub fn set_params_verify_account_keys(
    accounts: SetParamsAccounts<'_, '_>,
    keys: SetParamsKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.global.key, keys.global),
        (*accounts.authority.key, keys.authority),
        (*accounts.event_authority.key, keys.event_authority),
        (*accounts.program.key, keys.program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}

pub fn set_params_verify_writable_privileges<'me, 'info>(
    accounts: SetParamsAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.global, accounts.authority] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}

pub fn set_params_verify_signer_privileges<'me, 'info>(
    accounts: SetParamsAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}

pub fn set_params_verify_account_privileges<'me, 'info>(
    accounts: SetParamsAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    set_params_verify_writable_privileges(accounts)?;
    set_params_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const SET_RESERVED_FEE_RECIPIENTS_IX_ACCOUNTS_LEN: usize = 4;
#[derive(Copy, Clone, Debug)]
pub struct SetReservedFeeRecipientsAccounts<'me, 'info> {
    pub global: &'me AccountInfo<'info>,
    pub authority: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SetReservedFeeRecipientsKeys {
    pub global: Pubkey,
    pub authority: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}

impl From<SetReservedFeeRecipientsAccounts<'_, '_>> for SetReservedFeeRecipientsKeys {
    fn from(accounts: SetReservedFeeRecipientsAccounts) -> Self {
        Self {
            global: *accounts.global.key,
            authority: *accounts.authority.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}

impl From<SetReservedFeeRecipientsKeys> for [AccountMeta; SET_RESERVED_FEE_RECIPIENTS_IX_ACCOUNTS_LEN] {
    fn from(keys: SetReservedFeeRecipientsKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.global,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.authority,
                is_signer: true,
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

impl From<[Pubkey; SET_RESERVED_FEE_RECIPIENTS_IX_ACCOUNTS_LEN]> for SetReservedFeeRecipientsKeys {
    fn from(pubkeys: [Pubkey; SET_RESERVED_FEE_RECIPIENTS_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            global: pubkeys[0],
            authority: pubkeys[1],
            event_authority: pubkeys[2],
            program: pubkeys[3],
        }
    }
}

impl<'info> From<SetReservedFeeRecipientsAccounts<'_, 'info>> for [AccountInfo<'info>; SET_RESERVED_FEE_RECIPIENTS_IX_ACCOUNTS_LEN] {
    fn from(accounts: SetReservedFeeRecipientsAccounts<'_, 'info>) -> Self {
        [
            accounts.global.clone(),
            accounts.authority.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}

impl<'me, 'info> From<&'me [AccountInfo<'info>; SET_RESERVED_FEE_RECIPIENTS_IX_ACCOUNTS_LEN]> for SetReservedFeeRecipientsAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; SET_RESERVED_FEE_RECIPIENTS_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            global: &arr[0],
            authority: &arr[1],
            event_authority: &arr[2],
            program: &arr[3],
        }
    }
}

pub const SET_RESERVED_FEE_RECIPIENTS_IX_DISCM: [u8; 8] = [111, 172, 162, 232, 114, 89, 213, 142];

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SetReservedFeeRecipientsIxArgs {
    pub whitelist_pda: Pubkey,
}

#[derive(Clone, Debug, PartialEq)]
pub struct SetReservedFeeRecipientsIxData(pub SetReservedFeeRecipientsIxArgs);

impl From<SetReservedFeeRecipientsIxArgs> for SetReservedFeeRecipientsIxData {
    fn from(args: SetReservedFeeRecipientsIxArgs) -> Self {
        Self(args)
    }
}

impl SetReservedFeeRecipientsIxData {
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&SET_RESERVED_FEE_RECIPIENTS_IX_DISCM)?;
        self.0.serialize(&mut writer)
    }

    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }

    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut discm = [0u8; 8];
        reader.read_exact(&mut discm)?;
        if discm != SET_RESERVED_FEE_RECIPIENTS_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discriminator mismatch. Expected {:?}, got {:?}",
                    SET_RESERVED_FEE_RECIPIENTS_IX_DISCM, discm
                ),
            ));
        }
        Ok(Self(SetReservedFeeRecipientsIxArgs::deserialize(&mut reader)?))
    }
}

pub fn set_reserved_fee_recipients_ix_with_program_id(
    program_id: Pubkey,
    keys: SetReservedFeeRecipientsKeys,
    args: SetReservedFeeRecipientsIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; SET_PARAMS_IX_ACCOUNTS_LEN] = keys.into();
    let data: SetReservedFeeRecipientsIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}

pub fn set_reserved_fee_recipients_ix(keys: SetReservedFeeRecipientsKeys, args: SetReservedFeeRecipientsIxArgs) -> std::io::Result<Instruction> {
    set_reserved_fee_recipients_ix_with_program_id(crate::ID, keys, args)
}

pub fn set_reserved_fee_recipients_invoke_with_program_id(
    program_id: Pubkey,
    accounts: SetReservedFeeRecipientsAccounts<'_, '_>,
    args: SetReservedFeeRecipientsIxArgs,
) -> ProgramResult {
    let ix = set_reserved_fee_recipients_ix_with_program_id(program_id, accounts.into(), args)?;
    invoke_instruction(&ix, accounts)
}

pub fn set_reserved_fee_recipients_invoke(accounts: SetReservedFeeRecipientsAccounts<'_, '_>, args: SetReservedFeeRecipientsIxArgs) -> ProgramResult {
    set_reserved_fee_recipients_invoke_with_program_id(crate::ID, accounts, args)
}

pub fn set_reserved_fee_recipients_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: SetReservedFeeRecipientsAccounts<'_, '_>,
    args: SetReservedFeeRecipientsIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = set_reserved_fee_recipients_ix_with_program_id(program_id, accounts.into(), args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}

pub fn set_reserved_fee_recipients_invoke_signed(
    accounts: SetReservedFeeRecipientsAccounts<'_, '_>,
    args: SetReservedFeeRecipientsIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    set_reserved_fee_recipients_invoke_signed_with_program_id(crate::ID, accounts, args, seeds)
}

pub fn set_reserved_fee_recipients_verify_account_keys(
    accounts: SetReservedFeeRecipientsAccounts<'_, '_>,
    keys: SetReservedFeeRecipientsKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.global.key, keys.global),
        (*accounts.authority.key, keys.authority),
        (*accounts.event_authority.key, keys.event_authority),
        (*accounts.program.key, keys.program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}

pub fn set_reserved_fee_recipients_verify_writable_privileges<'me, 'info>(
    accounts: SetReservedFeeRecipientsAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.global, accounts.authority] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}

pub fn set_reserved_fee_recipients_verify_signer_privileges<'me, 'info>(
    accounts: SetReservedFeeRecipientsAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}

pub fn set_reserved_fee_recipients_verify_account_privileges<'me, 'info>(
    accounts: SetReservedFeeRecipientsAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    set_reserved_fee_recipients_verify_writable_privileges(accounts)?;
    set_reserved_fee_recipients_verify_signer_privileges(accounts)?;
    Ok(())
}



pub const SYNC_USER_VOLUME_ACCUMULATOR_IX_ACCOUNTS_LEN: usize = 5;

#[derive(Copy, Clone, Debug)]
pub struct SyncUserVolumeAccumulatorAccounts<'me, 'info> {
    pub user: &'me AccountInfo<'info>,
    pub global_volume_accumulator: &'me AccountInfo<'info>,
    pub user_volume_accumulator: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SyncUserVolumeAccumulatorKeys {
    pub user: Pubkey,
    pub global_volume_accumulator: Pubkey,
    pub user_volume_accumulator: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}

impl From<SyncUserVolumeAccumulatorAccounts<'_, '_>> for SyncUserVolumeAccumulatorKeys {
    fn from(accounts: SyncUserVolumeAccumulatorAccounts) -> Self {
        Self {
            user: *accounts.user.key,
            global_volume_accumulator: *accounts.global_volume_accumulator.key,
            user_volume_accumulator: *accounts.user_volume_accumulator.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}

impl From<SyncUserVolumeAccumulatorKeys> for [AccountMeta; SYNC_USER_VOLUME_ACCUMULATOR_IX_ACCOUNTS_LEN] {
    fn from(keys: SyncUserVolumeAccumulatorKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.user,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.global_volume_accumulator,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.user_volume_accumulator,
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

impl From<[Pubkey; SYNC_USER_VOLUME_ACCUMULATOR_IX_ACCOUNTS_LEN]> for SyncUserVolumeAccumulatorKeys {
    fn from(pubkeys: [Pubkey; SYNC_USER_VOLUME_ACCUMULATOR_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            user: pubkeys[0],
            global_volume_accumulator: pubkeys[1],
            user_volume_accumulator: pubkeys[2],
            event_authority: pubkeys[3],
            program: pubkeys[4],
        }
    }
}

impl<'info> From<SyncUserVolumeAccumulatorAccounts<'_, 'info>> for [AccountInfo<'info>; SYNC_USER_VOLUME_ACCUMULATOR_IX_ACCOUNTS_LEN] {
    fn from(accounts: SyncUserVolumeAccumulatorAccounts<'_, 'info>) -> Self {
        [
            accounts.user.clone(),
            accounts.global_volume_accumulator.clone(),
            accounts.user_volume_accumulator.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}

impl<'me, 'info> From<&'me [AccountInfo<'info>; SYNC_USER_VOLUME_ACCUMULATOR_IX_ACCOUNTS_LEN]> for SyncUserVolumeAccumulatorAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; SYNC_USER_VOLUME_ACCUMULATOR_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            user: &arr[0],
            global_volume_accumulator: &arr[1],
            user_volume_accumulator: &arr[2],
            event_authority: &arr[3],
            program: &arr[4],
        }
    }
}

pub const SYNC_USER_VOLUME_ACCUMULATOR_IX_DISCM: [u8; 8] = [86, 31, 192, 87, 163, 87, 79, 238];

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SyncUserVolumeAccumulatorIxArgs {}

#[derive(Clone, Debug, PartialEq)]
pub struct SyncUserVolumeAccumulatorIxData(pub SyncUserVolumeAccumulatorIxArgs);

impl From<SyncUserVolumeAccumulatorIxArgs> for SyncUserVolumeAccumulatorIxData {
    fn from(args: SyncUserVolumeAccumulatorIxArgs) -> Self {
        Self(args)
    }
}

impl SyncUserVolumeAccumulatorIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != SYNC_USER_VOLUME_ACCUMULATOR_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    SYNC_USER_VOLUME_ACCUMULATOR_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(SyncUserVolumeAccumulatorIxArgs::deserialize(&mut reader)?))
    }

    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&SYNC_USER_VOLUME_ACCUMULATOR_IX_DISCM)?;
        self.0.serialize(&mut writer)
    }

    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}

pub fn sync_user_volume_accumulator_ix_with_program_id(
    program_id: Pubkey,
    keys: SyncUserVolumeAccumulatorKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; SYNC_USER_VOLUME_ACCUMULATOR_IX_ACCOUNTS_LEN] = keys.into();
    let data: SyncUserVolumeAccumulatorIxData = SyncUserVolumeAccumulatorIxArgs {}.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}

pub fn sync_user_volume_accumulator_ix(
    keys: SyncUserVolumeAccumulatorKeys,
) -> std::io::Result<Instruction> {
    sync_user_volume_accumulator_ix_with_program_id(crate::ID, keys)
}

pub fn sync_user_volume_accumulator_invoke_with_program_id(
    program_id: Pubkey,
    accounts: SyncUserVolumeAccumulatorAccounts<'_, '_>,
) -> ProgramResult {
    let keys: SyncUserVolumeAccumulatorKeys = accounts.into();
    let ix = sync_user_volume_accumulator_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}

pub fn sync_user_volume_accumulator_invoke(
    accounts: SyncUserVolumeAccumulatorAccounts<'_, '_>,
) -> ProgramResult {
    sync_user_volume_accumulator_invoke_with_program_id(crate::ID, accounts)
}

pub fn sync_user_volume_accumulator_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: SyncUserVolumeAccumulatorAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: SyncUserVolumeAccumulatorKeys = accounts.into();
    let ix = sync_user_volume_accumulator_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}

pub fn sync_user_volume_accumulator_invoke_signed(
    accounts: SyncUserVolumeAccumulatorAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    sync_user_volume_accumulator_invoke_signed_with_program_id(crate::ID, accounts, seeds)
}

pub fn sync_user_volume_accumulator_verify_account_keys(
    accounts: SyncUserVolumeAccumulatorAccounts<'_, '_>,
    keys: SyncUserVolumeAccumulatorKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.user.key, keys.user),
        (*accounts.global_volume_accumulator.key, keys.global_volume_accumulator),
        (*accounts.user_volume_accumulator.key, keys.user_volume_accumulator),
        (*accounts.event_authority.key, keys.event_authority),
        (*accounts.program.key, keys.program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}

pub fn sync_user_volume_accumulator_verify_writable_privileges<'me, 'info>(
    accounts: SyncUserVolumeAccumulatorAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.user_volume_accumulator] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}

pub fn sync_user_volume_accumulator_verify_signer_privileges<'me, 'info>(
    accounts: SyncUserVolumeAccumulatorAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    Ok(())
}

pub fn sync_user_volume_accumulator_verify_account_privileges<'me, 'info>(
    accounts: SyncUserVolumeAccumulatorAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    sync_user_volume_accumulator_verify_writable_privileges(accounts)?;
    sync_user_volume_accumulator_verify_signer_privileges(accounts)?;
    Ok(())
}

pub const TOGGLE_CREATEV2_IX_ACCOUNTS_LEN: usize = 4;
#[derive(Copy, Clone, Debug)]
pub struct ToggleCreateV2Accounts<'me, 'info> {
    pub global:  &'me AccountInfo<'info>,
    pub authority:  &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct ToggleCreateV2Keys {
    pub global: Pubkey,
    pub authority: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}

impl From<ToggleCreateV2Accounts<'_, '_>> for ToggleCreateV2Keys {
    fn from(accounts: ToggleCreateV2Accounts) -> Self {
        Self {
            global: *accounts.global.key,
            authority: *accounts.authority.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}

impl From<ToggleCreateV2Keys> for [AccountMeta; TOGGLE_CREATEV2_IX_ACCOUNTS_LEN] {
    fn from(keys: ToggleCreateV2Keys) -> Self {
        [
            AccountMeta {
                pubkey: keys.global,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.authority,
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

impl From<[Pubkey; TOGGLE_CREATEV2_IX_ACCOUNTS_LEN]> for ToggleCreateV2Keys {
    fn from(pubkeys: [Pubkey; TOGGLE_CREATEV2_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            global: pubkeys[0],
            authority: pubkeys[1],
            event_authority: pubkeys[2],
            program: pubkeys[3],
        }
    }
}

impl<'info> From<ToggleCreateV2Accounts<'_, 'info>> for [AccountInfo<'info>; TOGGLE_CREATEV2_IX_ACCOUNTS_LEN] {
    fn from(accounts: ToggleCreateV2Accounts<'_, 'info>) -> Self {
        [
            accounts.global.clone(),
            accounts.authority.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}

impl<'me, 'info> From<&'me [AccountInfo<'info>; TOGGLE_CREATEV2_IX_ACCOUNTS_LEN]> for ToggleCreateV2Accounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; TOGGLE_CREATEV2_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            global: &arr[0],
            authority: &arr[1],
            event_authority: &arr[2],
            program: &arr[3],
        }
    }
}
pub const TOGGLE_CREATEV2_IX_DISCM: [u8; 8] = [28, 255, 230, 240, 172, 107, 203, 171];

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ToggleCreateV2IxArgs {
    enabled: bool,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ToggleCreateV2IxData(pub ToggleCreateV2IxArgs);

impl From<ToggleCreateV2IxArgs> for ToggleCreateV2IxData {
    fn from(args: ToggleCreateV2IxArgs) -> Self {
        Self(args)
    }
}

impl ToggleCreateV2IxData {
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&TOGGLE_CREATEV2_IX_DISCM)?;
        self.0.serialize(&mut writer)
    }

    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }

    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut discm = [0u8; 8];
        reader.read_exact(&mut discm)?;
        if discm != TOGGLE_CREATEV2_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discriminator mismatch. Expected {:?}, got {:?}",
                    TOGGLE_CREATEV2_IX_DISCM, discm
                ),
            ));
        }
        Ok(Self(ToggleCreateV2IxArgs::deserialize(&mut reader)?))
    }
}
pub fn toggle_createv2_ix_with_program_id(
    program_id: Pubkey,
    keys: ToggleCreateV2Keys,
    args: ToggleCreateV2IxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; TOGGLE_CREATEV2_IX_ACCOUNTS_LEN] = keys.into();
    let data: ToggleCreateV2IxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}

pub fn toggle_createv2_ix(keys: ToggleCreateV2Keys, args: ToggleCreateV2IxArgs) -> std::io::Result<Instruction> {
    toggle_createv2_ix_with_program_id(crate::ID, keys, args)
}

pub fn toggle_createv2_invoke_with_program_id(
    program_id: Pubkey,
    accounts: ToggleCreateV2Accounts<'_, '_>,
    args: ToggleCreateV2IxArgs,
) -> ProgramResult {
    let keys: ToggleCreateV2Keys = accounts.into();
    let ix = toggle_createv2_ix_with_program_id(program_id,keys, args)?;
    invoke_instruction(&ix, accounts)
}

pub fn toggle_createv2_invoke(accounts: ToggleCreateV2Accounts<'_, '_>, args: ToggleCreateV2IxArgs) -> ProgramResult {
    toggle_createv2_invoke_with_program_id(crate::ID, accounts,args)
}

pub fn toggle_createv2_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: ToggleCreateV2Accounts<'_, '_>,
    args: ToggleCreateV2IxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys : ToggleCreateV2Keys = accounts.into();
    let ix = toggle_createv2_ix_with_program_id(program_id, keys,args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}

pub fn toggle_createv2_invoke_signed(
    accounts: ToggleCreateV2Accounts<'_, '_>,
    args: ToggleCreateV2IxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    toggle_createv2_invoke_signed_with_program_id(crate::ID, accounts,args, seeds)
}

pub fn toggle_createv2_verify_account_keys(
    accounts: ToggleCreateV2Accounts<'_, '_>,
    keys: ToggleCreateV2Keys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.global.key, keys.global),
        (*accounts.authority.key, keys.authority),
        (*accounts.event_authority.key, keys.event_authority),
        (*accounts.program.key, keys.program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}

pub fn toggle_createv2_verify_writable_privileges<'me, 'info>(
    accounts: ToggleCreateV2Accounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.global] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}

pub fn toggle_createv2_verify_signer_privileges<'me, 'info>(
    accounts: ToggleCreateV2Accounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}

pub fn toggle_createv2_verify_account_privileges<'me, 'info>(
    accounts: ToggleCreateV2Accounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    toggle_createv2_verify_writable_privileges(accounts)?;
    toggle_createv2_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const TOGGLE_MAYHEM_MODE_IX_ACCOUNTS_LEN: usize = 4;
#[derive(Copy, Clone, Debug)]
pub struct ToggleMayhemModeAccounts<'me, 'info> {
    pub global:  &'me AccountInfo<'info>,
    pub authority:  &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct ToggleMayhemModeKeys {
    pub global: Pubkey,
    pub authority: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}

impl From<ToggleMayhemModeAccounts<'_, '_>> for ToggleMayhemModeKeys {
    fn from(accounts: ToggleMayhemModeAccounts) -> Self {
        Self {
            global: *accounts.global.key,
            authority: *accounts.authority.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}

impl From<ToggleMayhemModeKeys> for [AccountMeta; TOGGLE_MAYHEM_MODE_IX_ACCOUNTS_LEN] {
    fn from(keys: ToggleMayhemModeKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.global,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.authority,
                is_signer: true,
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

impl From<[Pubkey; TOGGLE_MAYHEM_MODE_IX_ACCOUNTS_LEN]> for ToggleMayhemModeKeys {
    fn from(pubkeys: [Pubkey; TOGGLE_MAYHEM_MODE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            global: pubkeys[0],
            authority: pubkeys[1],
            event_authority: pubkeys[2],
            program: pubkeys[3],
        }
    }
}

impl<'info> From<ToggleMayhemModeAccounts<'_, 'info>> for [AccountInfo<'info>; TOGGLE_MAYHEM_MODE_IX_ACCOUNTS_LEN] {
    fn from(accounts: ToggleMayhemModeAccounts<'_, 'info>) -> Self {
        [
            accounts.global.clone(),
            accounts.authority.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}

impl<'me, 'info> From<&'me [AccountInfo<'info>; TOGGLE_MAYHEM_MODE_IX_ACCOUNTS_LEN]> for ToggleMayhemModeAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; TOGGLE_MAYHEM_MODE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            global: &arr[0],
            authority: &arr[1],
            event_authority: &arr[2],
            program: &arr[3],
        }
    }
}
pub const TOGGLE_MAYHEM_MODE_IX_DISCM: [u8; 8] = [1, 9, 111, 208, 100, 31, 255, 163];

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct  ToggleMayhemModeIxArgs {
    enabled: bool,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ToggleMayhemModeIxData(pub ToggleMayhemModeIxArgs);

impl From<ToggleMayhemModeIxArgs> for ToggleMayhemModeIxData {
    fn from(args: ToggleMayhemModeIxArgs) -> Self {
        Self(args)
    }
}

impl ToggleMayhemModeIxData {
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&TOGGLE_MAYHEM_MODE_IX_DISCM)?;
        self.0.serialize(&mut writer)
    }

    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }

    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut discm = [0u8; 8];
        reader.read_exact(&mut discm)?;
        if discm != TOGGLE_MAYHEM_MODE_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discriminator mismatch. Expected {:?}, got {:?}",
                    TOGGLE_MAYHEM_MODE_IX_DISCM, discm
                ),
            ));
        }
        Ok(Self(ToggleMayhemModeIxArgs::deserialize(&mut reader)?))
    }
}

pub fn toggle_mayhem_mode_ix_with_program_id(
    program_id: Pubkey,
    keys: ToggleMayhemModeKeys,
    args: ToggleMayhemModeIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; TOGGLE_MAYHEM_MODE_IX_ACCOUNTS_LEN] = keys.into();
    let data: ToggleMayhemModeIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}

pub fn toggle_mayhem_mode_ix(keys: ToggleMayhemModeKeys, args: ToggleMayhemModeIxArgs) -> std::io::Result<Instruction> {
    toggle_mayhem_mode_ix_with_program_id(crate::ID, keys, args)
}

pub fn toggle_mayhem_mode_invoke_with_program_id(
    program_id: Pubkey,
    accounts: ToggleMayhemModeAccounts<'_, '_>,
    args: ToggleMayhemModeIxArgs,
) -> ProgramResult {
    let keys: ToggleMayhemModeKeys = accounts.into();
    let ix = toggle_mayhem_mode_ix_with_program_id(program_id,keys, args)?;
    invoke_instruction(&ix, accounts)
}

pub fn toggle_mayhem_mode_invoke(accounts: ToggleMayhemModeAccounts<'_, '_>, args: ToggleMayhemModeIxArgs) -> ProgramResult {
    toggle_mayhem_mode_invoke_with_program_id(crate::ID, accounts,args)
}

pub fn toggle_mayhem_mode_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: ToggleMayhemModeAccounts<'_, '_>,
    args: ToggleMayhemModeIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys : ToggleMayhemModeKeys = accounts.into();
    let ix = toggle_mayhem_mode_ix_with_program_id(program_id, keys,args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}

pub fn toggle_mayhem_mode_invoke_signed(
    accounts: ToggleMayhemModeAccounts<'_, '_>,
    args: ToggleMayhemModeIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    toggle_mayhem_mode_invoke_signed_with_program_id(crate::ID, accounts,args, seeds)
}

pub fn toggle_mayhem_mode_verify_account_keys(
    accounts: ToggleMayhemModeAccounts<'_, '_>,
    keys: ToggleMayhemModeKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.global.key, keys.global),
        (*accounts.authority.key, keys.authority),
        (*accounts.event_authority.key, keys.event_authority),
        (*accounts.program.key, keys.program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}

pub fn toggle_mayhem_mode_verify_writable_privileges<'me, 'info>(
    accounts: ToggleMayhemModeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.global] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}

pub fn toggle_mayhem_mode_verify_signer_privileges<'me, 'info>(
    accounts: ToggleMayhemModeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}

pub fn toggle_mayhem_mode_verify_account_privileges<'me, 'info>(
    accounts: ToggleMayhemModeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    toggle_mayhem_mode_verify_writable_privileges(accounts)?;
    toggle_mayhem_mode_verify_signer_privileges(accounts)?;
    Ok(())
}

pub const UPDATE_GLOBAL_AUTHORITY_IX_ACCOUNTS_LEN: usize = 5;

#[derive(Copy, Clone, Debug)]
pub struct UpdateGlobalAuthorityAccounts<'me, 'info> {
    pub global: &'me AccountInfo<'info>,
    pub authority: &'me AccountInfo<'info>,
    pub new_authority: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct UpdateGlobalAuthorityKeys {
    pub global: Pubkey,
    pub authority: Pubkey,
    pub new_authority: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}

impl From<UpdateGlobalAuthorityAccounts<'_, '_>> for UpdateGlobalAuthorityKeys {
    fn from(accounts: UpdateGlobalAuthorityAccounts) -> Self {
        Self {
            global: *accounts.global.key,
            authority: *accounts.authority.key,
            new_authority: *accounts.new_authority.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}

impl From<UpdateGlobalAuthorityKeys> for [AccountMeta; UPDATE_GLOBAL_AUTHORITY_IX_ACCOUNTS_LEN] {
    fn from(keys: UpdateGlobalAuthorityKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.global,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.authority,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.new_authority,
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

impl From<[Pubkey; UPDATE_GLOBAL_AUTHORITY_IX_ACCOUNTS_LEN]> for UpdateGlobalAuthorityKeys {
    fn from(pubkeys: [Pubkey; UPDATE_GLOBAL_AUTHORITY_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            global: pubkeys[0],
            authority: pubkeys[1],
            new_authority: pubkeys[2],
            event_authority: pubkeys[3],
            program: pubkeys[4],
        }
    }
}

impl<'info> From<UpdateGlobalAuthorityAccounts<'_, 'info>> for [AccountInfo<'info>; UPDATE_GLOBAL_AUTHORITY_IX_ACCOUNTS_LEN] {
    fn from(accounts: UpdateGlobalAuthorityAccounts<'_, 'info>) -> Self {
        [
            accounts.global.clone(),
            accounts.authority.clone(),
            accounts.new_authority.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}

impl<'me, 'info> From<&'me [AccountInfo<'info>; UPDATE_GLOBAL_AUTHORITY_IX_ACCOUNTS_LEN]> for UpdateGlobalAuthorityAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; UPDATE_GLOBAL_AUTHORITY_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            global: &arr[0],
            authority: &arr[1],
            new_authority: &arr[2],
            event_authority: &arr[3],
            program: &arr[4],
        }
    }
}

pub const UPDATE_GLOBAL_AUTHORITY_IX_DISCM: [u8; 8] = [227, 181, 74, 196, 208, 21, 97, 213];

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq, Default)]
pub struct UpdateGlobalAuthorityIxArgs;

#[derive(Clone, Debug, PartialEq)]
pub struct UpdateGlobalAuthorityIxData(pub UpdateGlobalAuthorityIxArgs);

impl From<UpdateGlobalAuthorityIxArgs> for UpdateGlobalAuthorityIxData {
    fn from(args: UpdateGlobalAuthorityIxArgs) -> Self {
        Self(args)
    }
}

impl UpdateGlobalAuthorityIxData {
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&UPDATE_GLOBAL_AUTHORITY_IX_DISCM)?;
        self.0.serialize(&mut writer)
    }

    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }

    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut discm = [0u8; 8];
        reader.read_exact(&mut discm)?;
        if discm != UPDATE_GLOBAL_AUTHORITY_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discriminator mismatch. Expected {:?}, got {:?}",
                    UPDATE_GLOBAL_AUTHORITY_IX_DISCM, discm
                ),
            ));
        }
        Ok(Self(UpdateGlobalAuthorityIxArgs::deserialize(&mut reader)?))
    }
}

pub fn update_global_authority_ix_with_program_id(
    program_id: Pubkey,
    keys: UpdateGlobalAuthorityKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; UPDATE_GLOBAL_AUTHORITY_IX_ACCOUNTS_LEN] = keys.into();
    let data: UpdateGlobalAuthorityIxData = UpdateGlobalAuthorityIxArgs.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}

pub fn update_global_authority_ix(keys: UpdateGlobalAuthorityKeys) -> std::io::Result<Instruction> {
    update_global_authority_ix_with_program_id(crate::ID, keys)
}

pub fn update_global_authority_invoke_with_program_id(
    program_id: Pubkey,
    accounts: UpdateGlobalAuthorityAccounts<'_, '_>,
) -> ProgramResult {
    let ix = update_global_authority_ix_with_program_id(program_id, accounts.into())?;
    invoke_instruction(&ix, accounts)
}

pub fn update_global_authority_invoke(accounts: UpdateGlobalAuthorityAccounts<'_, '_>) -> ProgramResult {
    update_global_authority_invoke_with_program_id(crate::ID, accounts)
}

pub fn update_global_authority_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: UpdateGlobalAuthorityAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = update_global_authority_ix_with_program_id(program_id, accounts.into())?;
    invoke_instruction_signed(&ix, accounts, seeds)
}

pub fn update_global_authority_invoke_signed(
    accounts: UpdateGlobalAuthorityAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    update_global_authority_invoke_signed_with_program_id(crate::ID, accounts, seeds)
}

pub fn update_global_authority_verify_account_keys(
    accounts: UpdateGlobalAuthorityAccounts<'_, '_>,
    keys: UpdateGlobalAuthorityKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.global.key, keys.global),
        (*accounts.authority.key, keys.authority),
        (*accounts.new_authority.key, keys.new_authority),
        (*accounts.event_authority.key, keys.event_authority),
        (*accounts.program.key, keys.program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}

pub fn update_global_authority_verify_writable_privileges<'me, 'info>(
    accounts: UpdateGlobalAuthorityAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.global] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}

pub fn update_global_authority_verify_signer_privileges<'me, 'info>(
    accounts: UpdateGlobalAuthorityAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}

pub fn update_global_authority_verify_account_privileges<'me, 'info>(
    accounts: UpdateGlobalAuthorityAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    update_global_authority_verify_writable_privileges(accounts)?;
    update_global_authority_verify_signer_privileges(accounts)?;
    Ok(())
}