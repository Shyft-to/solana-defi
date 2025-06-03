#[cfg(feature = "serde")]
// use crate::serializer::{deserialize_u128_as_string, serialize_u128_as_string};
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
// use std::fmt;
use strum_macros::{Display, EnumString};



#[derive(Clone, Debug, PartialEq, EnumString, Display)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum PumpProgramIx {
    Buy(BuyIxArgs),
    CollectCreatorFee,
    Create(CreateIxArgs),
    ExtendAccount,
    Initialize,
    Migrate,
    Sell(SellIxArgs),
    SetCreator(SetCreatorIxArgs),
    SetMetaPlexCreator,
    SetParams(SetParamsIxArgs),
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
            BUY_IX_DISCM => Ok(Self::Buy(BuyIxArgs::deserialize(&mut reader)?)),
            COLLECT_CREATOR_FEE_IX_DISCM => Ok(Self::CollectCreatorFee),
            CREATE_IX_DISCM => Ok(Self::Create(CreateIxArgs::deserialize(&mut reader)?)),
            EXTEND_ACCOUNT_IX_DISCM => Ok(Self::ExtendAccount),
            INITIALIZE_IX_DISCM => Ok(Self::Initialize),
            MIGRATE_IX_DISCM => Ok(Self::Migrate),
            SELL_IX_DISCM => Ok(Self::Sell(SellIxArgs::deserialize(&mut reader)?)),
            SET_CREATOR_IX_DISCM => Ok(Self::SetCreator(SetCreatorIxArgs::deserialize(&mut reader)?)),
            SET_METAPLEX_CREATOR_IX_DISCM => Ok(Self::SetMetaPlexCreator),
            SET_PARAMS_IX_DISCM => {
                Ok(Self::SetParams(SetParamsIxArgs::deserialize(&mut reader)?))
            }
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
             Self::Buy(args) => {
                writer.write_all(&BUY_IX_DISCM)?;
                args.serialize(&mut writer)
            }
            Self::CollectCreatorFee => writer.write_all(&COLLECT_CREATOR_FEE_IX_DISCM),
            Self::Create(args) => {
                writer.write_all(&CREATE_IX_DISCM)?;
                args.serialize(&mut writer)
            }
            Self::ExtendAccount => writer.write_all(&EXTEND_ACCOUNT_IX_DISCM),
            Self::Initialize => writer.write_all(&INITIALIZE_IX_DISCM),
            Self::Migrate =>  writer.write_all(&MIGRATE_IX_DISCM),

             Self::Sell(args) => {
                writer.write_all(&SELL_IX_DISCM)?;
                args.serialize(&mut writer)
            }
            Self::SetCreator(args) => {
                writer.write_all(&SET_CREATOR_IX_DISCM)?;
                args.serialize(&mut writer)
            }
            Self::SetMetaPlexCreator => writer.write_all(&SET_METAPLEX_CREATOR_IX_DISCM),

            Self::SetParams(args) => {
                writer.write_all(&SET_PARAMS_IX_DISCM)?;
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

pub const BUY_IX_ACCOUNTS_LEN: usize = 12;
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
    pub rent: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
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
    pub rent: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
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
            rent: *accounts.rent.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
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
            rent: pubkeys[9],
            event_authority: pubkeys[10],
            program: pubkeys[11],
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
            accounts.rent.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
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
            rent: &arr[9],
            event_authority: &arr[10],
            program: &arr[11],
        }
    }
}
pub const BUY_IX_DISCM: [u8; 8] = [102, 6, 61, 18, 1, 218, 235, 234];
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct BuyIxArgs {
    pub amount: u64,
    pub max_sol_cost: u64,
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
pub fn buy_verify_writable_privileges<'me, 'info>(
    accounts: BuyAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.fee_recipient,
        accounts.bonding_curve,
        accounts.associated_bonding_curve,
        accounts.associated_user,
        accounts.user,
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

pub const SELL_IX_ACCOUNTS_LEN: usize = 12;

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