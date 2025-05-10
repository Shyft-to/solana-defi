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
use inflector::Inflector;
use std::io::Read;
use strum_macros::{Display, EnumString};

#[derive(Clone, Debug, PartialEq, EnumString, Display)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum PumpfunAmmProgramIx {
    Buy(BuyIxArgs),
    CreateConfig(CreateConfigIxArgs),
    CreatePool(CreatePoolIxArgs),
    Deposit(DepositIxArgs),
    Disable(DisableIxArgs),
    ExtendAccount(ExtendAccountIxArgs),
    Sell(SellIxArgs),
    UpdateAdmin(UpdateAdminIxArgs),
    UpdateFeeConfig(UpdateFeeConfigIxArgs),
    Withdraw(WithdrawIxArgs),
}
impl PumpfunAmmProgramIx {
        pub fn name(&self) -> &str {
        match self {
            Self::Buy(_) => "Buy",
            Self::CreateConfig(_) => "CreateConfig",
            Self::CreatePool(_) => "CreatePool",
            Self::Deposit(_) => "Deposit",
            Self::Disable(_) => "Disable",
            Self::ExtendAccount(_) => "ExtendAccount",
            Self::Sell(_) => "Sell",
            Self::UpdateAdmin(_) => "UpdateAdmin",
            Self::UpdateFeeConfig(_) => "UpdateFeeConfig",
            Self::Withdraw(_) => "Withdraw",
        }
    }

    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        match maybe_discm {
            BUY_IX_DISCM => {
                Ok(
                    Self::Buy(
                        BuyIxArgs::deserialize(&mut reader)?,
                    ),
                )
            }
            CREATE_CONFIG_IX_DISCM => {
                Ok(
                    Self::CreateConfig(
                        CreateConfigIxArgs::deserialize(&mut reader)?,
                    ),
                )
            }
            CREATE_POOL_IX_DISCM => {
                Ok(
                    Self::CreatePool(
                        CreatePoolIxArgs::deserialize(&mut reader)?,
                    ),
                )
            }
            DEPOSIT_IX_DISCM => {
                Ok(
                    Self::Deposit(
                        DepositIxArgs::deserialize(&mut reader)?,
                    ),
                )
            }
            DISABLE_IX_DISCM => {
                Ok(Self::Disable(DisableIxArgs::deserialize(&mut reader)?))
            }
            EXTEND_ACCOUNT_IX_DISCM => {
                Ok(Self::ExtendAccount(ExtendAccountIxArgs::deserialize(&mut reader)?))
            }
            SELL_IX_DISCM => {
                Ok(Self::Sell(SellIxArgs::deserialize(&mut reader)?))
            }
            UPDATE_ADMIN_IX_DISCM => {
                Ok(Self::UpdateAdmin(UpdateAdminIxArgs::deserialize(&mut reader)?))
            }
            UPDATE_FEE_CONFIG_IX_DISCM => {
                Ok(Self::UpdateFeeConfig(UpdateFeeConfigIxArgs::deserialize(&mut reader)?))
            }
            WITHDRAW_IX_DISCM => {
                Ok(Self::Withdraw(WithdrawIxArgs::deserialize(&mut reader)?))
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
            Self::Buy(args) => {
                writer.write_all(&BUY_IX_DISCM)?;
                args.serialize(&mut writer)
            }
            Self::CreateConfig(args) => {
                writer.write_all(&CREATE_CONFIG_IX_DISCM)?;
                args.serialize(&mut writer)
            }
            Self::CreatePool(args) => {
                writer.write_all(&CREATE_POOL_IX_DISCM)?;
                args.serialize(&mut writer)
            }
            Self::Deposit(args) => {
                writer.write_all(&DEPOSIT_IX_DISCM)?;
                args.serialize(&mut writer)
            }
            Self::Disable(args) => {
                writer.write_all(&DISABLE_IX_DISCM)?;
                args.serialize(&mut writer)
            }
            Self::ExtendAccount(args) => {
                writer.write_all(&EXTEND_ACCOUNT_IX_DISCM)?;
                args.serialize(&mut writer)
            }
            Self::Sell(args) => {
                writer.write_all(&SELL_IX_DISCM)?;
                args.serialize(&mut writer)
            }
            Self::UpdateAdmin(args) => {
                writer.write_all(&UPDATE_ADMIN_IX_DISCM)?;
                args.serialize(&mut writer)
            }
            Self::UpdateFeeConfig(args) => {
                writer.write_all(&UPDATE_FEE_CONFIG_IX_DISCM)?;
                args.serialize(&mut writer)
            }
            Self::Withdraw(args) => {
                writer.write_all(&WITHDRAW_IX_DISCM)?;
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
pub const BUY_IX_ACCOUNTS_LEN: usize = 17;
#[derive(Copy, Clone, Debug)]
pub struct BuyAccounts<'me, 'info> {
    pub pool: &'me AccountInfo<'info>,
    pub user: &'me AccountInfo<'info>,
    pub global_config: &'me AccountInfo<'info>,
    pub base_mint: &'me AccountInfo<'info>,
    pub quote_mint: &'me AccountInfo<'info>,
    pub user_base_token_account: &'me AccountInfo<'info>,
    pub user_quote_token_account: &'me AccountInfo<'info>,
    pub pool_base_token_account: &'me AccountInfo<'info>,
    pub pool_quote_token_account: &'me AccountInfo<'info>,
    pub protocol_fee_recipient: &'me AccountInfo<'info>,
    pub protocol_fee_recipient_token_account: &'me AccountInfo<'info>,
    pub base_token_program: &'me AccountInfo<'info>,
    pub quote_token_program: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub associated_token_program: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct BuyKeys {
    pub pool: Pubkey,
    pub user: Pubkey,
    pub global_config: Pubkey,
    pub base_mint: Pubkey,
    pub quote_mint: Pubkey,
    pub user_base_token_account: Pubkey,
    pub user_quote_token_account: Pubkey,
    pub pool_base_token_account: Pubkey,
    pub pool_quote_token_account: Pubkey,
    pub protocol_fee_recipient: Pubkey,
    pub protocol_fee_recipient_token_account: Pubkey,
    pub base_token_program: Pubkey,
    pub quote_token_program: Pubkey,
    pub system_program: Pubkey,
    pub associated_token_program: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}

impl From<BuyAccounts<'_, '_>> for BuyKeys {
    fn from(accounts: BuyAccounts) -> Self {
        Self {
            pool: *accounts.pool.key,
            user: *accounts.user.key,
            global_config: *accounts.global_config.key,
            base_mint: *accounts.base_mint.key,
            quote_mint: *accounts.quote_mint.key,
            user_base_token_account: *accounts.user_base_token_account.key,
            user_quote_token_account: *accounts.user_quote_token_account.key,
            pool_base_token_account: *accounts.pool_base_token_account.key,
            pool_quote_token_account: *accounts.pool_quote_token_account.key,
            protocol_fee_recipient: *accounts.protocol_fee_recipient.key,
            protocol_fee_recipient_token_account: *accounts.protocol_fee_recipient_token_account.key,
            base_token_program: *accounts.base_token_program.key,
            quote_token_program: *accounts.quote_token_program.key,
            system_program: *accounts.system_program.key,
            associated_token_program: *accounts.associated_token_program.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}

impl From<BuyKeys> for [AccountMeta; BUY_IX_ACCOUNTS_LEN] {
    fn from(keys: BuyKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.pool,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.global_config,
                is_signer: false,
                is_writable: false,
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
                pubkey: keys.user_base_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_quote_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.pool_base_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.pool_quote_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.protocol_fee_recipient,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.protocol_fee_recipient_token_account,
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
        ]
    }
}

impl From<[Pubkey; BUY_IX_ACCOUNTS_LEN]> for BuyKeys {
    fn from(pubkeys: [Pubkey; BUY_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            pool: pubkeys[0],
            user: pubkeys[1],
            global_config: pubkeys[2],
            base_mint: pubkeys[3],
            quote_mint: pubkeys[4],
            user_base_token_account: pubkeys[5],
            user_quote_token_account: pubkeys[6],
            pool_base_token_account: pubkeys[7],
            pool_quote_token_account: pubkeys[8],
            protocol_fee_recipient: pubkeys[9],
            protocol_fee_recipient_token_account: pubkeys[10],
            base_token_program: pubkeys[11],
            quote_token_program: pubkeys[12],
            system_program: pubkeys[13],
            associated_token_program: pubkeys[14],
            event_authority: pubkeys[15],
            program: pubkeys[16],
        }
    }
}

impl<'info> From<BuyAccounts<'_, 'info>> for [AccountInfo<'info>; BUY_IX_ACCOUNTS_LEN] {
    fn from(accounts: BuyAccounts<'_, 'info>) -> Self {
        [
            accounts.pool.clone(),
            accounts.user.clone(),
            accounts.global_config.clone(),
            accounts.base_mint.clone(),
            accounts.quote_mint.clone(),
            accounts.user_base_token_account.clone(),
            accounts.user_quote_token_account.clone(),
            accounts.pool_base_token_account.clone(),
            accounts.pool_quote_token_account.clone(),
            accounts.protocol_fee_recipient.clone(),
            accounts.protocol_fee_recipient_token_account.clone(),
            accounts.base_token_program.clone(),
            accounts.quote_token_program.clone(),
            accounts.system_program.clone(),
            accounts.associated_token_program.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}

impl<'me, 'info> From<&'me [AccountInfo<'info>; BUY_IX_ACCOUNTS_LEN]> for BuyAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; BUY_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            pool: &arr[0],
            user: &arr[1],
            global_config: &arr[2],
            base_mint: &arr[3],
            quote_mint: &arr[4],
            user_base_token_account: &arr[5],
            user_quote_token_account: &arr[6],
            pool_base_token_account: &arr[7],
            pool_quote_token_account: &arr[8],
            protocol_fee_recipient: &arr[9],
            protocol_fee_recipient_token_account: &arr[10],
            base_token_program: &arr[11],
            quote_token_program: &arr[12],
            system_program: &arr[13],
            associated_token_program: &arr[14],
            event_authority: &arr[15],
            program: &arr[16],
        }
    }
}

pub const BUY_IX_DISCM: [u8; 8] =  [102, 6, 61, 18, 1, 218, 235, 234];
pub const BUY_IX_DISCM_B : [u8; 8] = [228, 69, 165, 46, 81, 203, 154, 29];
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct BuyIxArgs {
    pub base_amount_out: u64,
    pub max_quote_amount_in: u64
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
pub fn buy_ix(
    keys: BuyKeys,
    args: BuyIxArgs,
) -> std::io::Result<Instruction> {
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
pub fn buy_invoke(
    accounts: BuyAccounts<'_, '_>,
    args: BuyIxArgs,
) -> ProgramResult {
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
        (*accounts.pool.key, keys.pool),
        (*accounts.user.key, keys.user),
        (*accounts.global_config.key, keys.global_config),
        (*accounts.base_mint.key, keys.base_mint),
        (*accounts.quote_mint.key, keys.quote_mint),
        (*accounts.user_base_token_account.key, keys.user_base_token_account),
        (*accounts.user_quote_token_account.key, keys.user_quote_token_account),
        (*accounts.pool_base_token_account.key, keys.pool_base_token_account),
        (*accounts.pool_quote_token_account.key, keys.pool_quote_token_account),
        (*accounts.protocol_fee_recipient.key, keys.protocol_fee_recipient),
        (*accounts.protocol_fee_recipient_token_account.key, keys.protocol_fee_recipient_token_account),
        (*accounts.base_token_program.key, keys.base_token_program),
        (*accounts.quote_token_program.key, keys.quote_token_program),
        (*accounts.system_program.key, keys.system_program),
        (*accounts.associated_token_program.key, keys.associated_token_program),
        (*accounts.event_authority.key, keys.event_authority),
        (*accounts.program.key, keys.program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}

pub fn buy_verify_is_writable_privileges<'me, 'info>(
    accounts: BuyAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_is_writable in [
        accounts.pool,
        accounts.user,
        accounts.user_base_token_account,
        accounts.user_quote_token_account,
        accounts.pool_base_token_account,
        accounts.pool_quote_token_account,
        accounts.protocol_fee_recipient_token_account,
    ] {
        if !should_be_is_writable.is_writable {
            return Err((should_be_is_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}

pub fn buy_verify_is_signer_privileges<'me, 'info>(
    accounts: BuyAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_is_signer in [accounts.user] {
        if !should_be_is_signer.is_signer {
            return Err((should_be_is_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn buy_verify_account_privileges<'me, 'info>(
    accounts: BuyAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    buy_verify_is_writable_privileges(accounts)?;
    buy_verify_is_signer_privileges(accounts)?;
    Ok(())
}
pub const CREATE_CONFIG_IX_ACCOUNTS_LEN: usize = 5;
#[derive(Copy, Clone, Debug)]
pub struct CreateConfigAccounts<'me, 'info> {
    pub admin: &'me AccountInfo<'info>,
    pub global_config: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CreateConfigKeys {
    pub admin: Pubkey,
    pub global_config: Pubkey,
    pub system_program : Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey
}
impl From<CreateConfigAccounts<'_, '_>> for CreateConfigKeys {
    fn from(accounts: CreateConfigAccounts) -> Self {
        Self {
            admin: *accounts.admin.key,
            global_config: *accounts.global_config.key,
            system_program: *accounts.system_program.key,
            event_authority: *accounts.event_authority.key, 
            program: *accounts.program.key
        }
    }
}
impl From<CreateConfigKeys> for [AccountMeta; CREATE_CONFIG_IX_ACCOUNTS_LEN] {
    fn from(keys: CreateConfigKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.admin,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.global_config,
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
impl From<[Pubkey; CREATE_CONFIG_IX_ACCOUNTS_LEN]> for CreateConfigKeys {
    fn from(pubkeys: [Pubkey; CREATE_CONFIG_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            admin: pubkeys[0],
            global_config: pubkeys[1],
            system_program: pubkeys[2],
            event_authority: pubkeys[3],
            program: pubkeys[4]
        }
    }
}
impl<'info> From<CreateConfigAccounts<'_, 'info>>
    for [AccountInfo<'info>; CREATE_CONFIG_IX_ACCOUNTS_LEN]
{
    fn from(accounts: CreateConfigAccounts<'_, 'info>) -> Self {
        [
            accounts.admin.clone(),
            accounts.global_config.clone(),
            accounts.system_program.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; CREATE_CONFIG_IX_ACCOUNTS_LEN]>
    for CreateConfigAccounts<'me, 'info>
{
    fn from(arr: &'me [AccountInfo<'info>; CREATE_CONFIG_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            admin: &arr[0],
            global_config: &arr[1],
            system_program: &arr[2],
            event_authority: &arr[3],
            program: &arr[4]
        }
    }
}
pub const CREATE_CONFIG_IX_DISCM: [u8; 8] =  [201, 207, 243, 114, 75, 111, 47, 189];
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CreateConfigIxArgs {
    pub lp_fee_basis_points: u64,
    pub protocol_fee_basis_points: u64,
    pub protocol_fee_recipients: [Pubkey; 8]
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
        (*accounts.admin.key, keys.admin),
        (*accounts.global_config.key, keys.global_config),
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
    for should_be_is_writable in [accounts.admin, accounts.global_config] {
        if !should_be_is_writable.is_writable {
            return Err((should_be_is_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn create_config_verify_is_signer_privileges<'me, 'info>(
    accounts: CreateConfigAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_is_signer in [accounts.admin] {
        if !should_be_is_signer.is_signer {
            return Err((should_be_is_signer, ProgramError::MissingRequiredSignature));
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
pub const CREATE_POOL_IX_ACCOUNTS_LEN: usize = 18;
#[derive(Copy, Clone, Debug)]
pub struct CreatePoolAccounts<'me, 'info> {
    pub pool: &'me AccountInfo<'info>,
    pub global_config: &'me AccountInfo<'info>,
    pub creator: &'me AccountInfo<'info>,
    pub base_mint: &'me AccountInfo<'info>,
    pub quote_mint: &'me AccountInfo<'info>,
    pub lp_mint: &'me AccountInfo<'info>,
    pub user_base_token_account: &'me AccountInfo<'info>,
    pub user_quote_token_account: &'me AccountInfo<'info>,
    pub user_pool_token_account: &'me AccountInfo<'info>,
    pub pool_base_token_account: &'me AccountInfo<'info>,
    pub pool_quote_token_account: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub token_2022_program: &'me AccountInfo<'info>,
    pub base_token_program: &'me AccountInfo<'info>,
    pub quote_token_program: &'me AccountInfo<'info>,
    pub associated_token_program: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CreatePoolKeys {
    pub pool: Pubkey,
    pub global_config: Pubkey,
    pub creator: Pubkey,
    pub base_mint: Pubkey,
    pub quote_mint: Pubkey,
    pub lp_mint: Pubkey,
    pub user_base_token_account: Pubkey,
    pub user_quote_token_account: Pubkey,
    pub user_pool_token_account: Pubkey,
    pub pool_base_token_account: Pubkey,
    pub pool_quote_token_account: Pubkey,
    pub system_program: Pubkey,
    pub token_2022_program: Pubkey,
    pub base_token_program: Pubkey,
    pub quote_token_program: Pubkey,
    pub associated_token_program: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}

impl From<CreatePoolAccounts<'_, '_>> for CreatePoolKeys {
    fn from(accounts: CreatePoolAccounts) -> Self {
        Self {
            pool: *accounts.pool.key,
            global_config: *accounts.global_config.key,
            creator: *accounts.creator.key,
            base_mint: *accounts.base_mint.key,
            quote_mint: *accounts.quote_mint.key,
            lp_mint: *accounts.lp_mint.key,
            user_base_token_account: *accounts.user_base_token_account.key,
            user_quote_token_account: *accounts.user_quote_token_account.key,
            user_pool_token_account: *accounts.user_pool_token_account.key,
            pool_base_token_account: *accounts.pool_base_token_account.key,
            pool_quote_token_account: *accounts.pool_quote_token_account.key,
            system_program: *accounts.system_program.key,
            token_2022_program: *accounts.token_2022_program.key,
            base_token_program: *accounts.base_token_program.key,
            quote_token_program: *accounts.quote_token_program.key,
            associated_token_program: *accounts.associated_token_program.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<CreatePoolKeys> for [AccountMeta; CREATE_POOL_IX_ACCOUNTS_LEN] {
    fn from(keys: CreatePoolKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.pool,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.global_config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.creator,
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
                pubkey: keys.lp_mint,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_base_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_quote_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_pool_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.pool_base_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.pool_quote_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.system_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_2022_program,
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
        ]
    }
}
impl From<[Pubkey; CREATE_POOL_IX_ACCOUNTS_LEN]> for CreatePoolKeys {
    fn from(pubkeys: [Pubkey; CREATE_POOL_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            pool: pubkeys[0],
            global_config: pubkeys[1],
            creator: pubkeys[2],
            base_mint: pubkeys[3],
            quote_mint: pubkeys[4],
            lp_mint: pubkeys[5],
            user_base_token_account: pubkeys[6],
            user_quote_token_account: pubkeys[7],
            user_pool_token_account: pubkeys[8],
            pool_base_token_account: pubkeys[9],
            pool_quote_token_account: pubkeys[10],
            system_program: pubkeys[11],
            token_2022_program: pubkeys[12],
            base_token_program: pubkeys[13],
            quote_token_program: pubkeys[14],
            associated_token_program: pubkeys[15],
            event_authority: pubkeys[16],
            program: pubkeys[17],
        }
    }
}
impl<'info> From<CreatePoolAccounts<'_, 'info>>
    for [AccountInfo<'info>; CREATE_POOL_IX_ACCOUNTS_LEN]
{
    fn from(accounts: CreatePoolAccounts<'_, 'info>) -> Self {
        [
            accounts.pool.clone(),
            accounts.global_config.clone(),
            accounts.creator.clone(),
            accounts.base_mint.clone(),
            accounts.quote_mint.clone(),
            accounts.lp_mint.clone(),
            accounts.user_base_token_account.clone(),
            accounts.user_quote_token_account.clone(),
            accounts.user_pool_token_account.clone(),
            accounts.pool_base_token_account.clone(),
            accounts.pool_quote_token_account.clone(),
            accounts.system_program.clone(),
            accounts.token_2022_program.clone(),
            accounts.base_token_program.clone(),
            accounts.quote_token_program.clone(),
            accounts.associated_token_program.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; CREATE_POOL_IX_ACCOUNTS_LEN]>
    for CreatePoolAccounts<'me, 'info>
{
    fn from(arr: &'me [AccountInfo<'info>; CREATE_POOL_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            pool: &arr[0],
            global_config: &arr[1],
            creator: &arr[2],
            base_mint: &arr[3],
            quote_mint: &arr[4],
            lp_mint: &arr[5],
            user_base_token_account: &arr[6],
            user_quote_token_account: &arr[7],
            user_pool_token_account: &arr[8],
            pool_base_token_account: &arr[9],
            pool_quote_token_account: &arr[10],
            system_program: &arr[11],
            token_2022_program: &arr[12],
            base_token_program: &arr[13],
            quote_token_program: &arr[14],
            associated_token_program: &arr[15],
            event_authority: &arr[16],
            program: &arr[17],
        }
    }
}
pub const CREATE_POOL_IX_DISCM: [u8; 8] = [233, 146, 209, 142, 207, 104, 64, 188];
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CreatePoolIxArgs {
    pub index: u16,
    pub base_amount_in: u64,
    pub quote_amount_in: u64,
}
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
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != CREATE_POOL_IX_DISCM {
            return Err(
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!(
                        "discm does not match. Expected: {:?}. Received: {:?}",
                        CREATE_POOL_IX_DISCM, maybe_discm
                    ),
                ),
            );
        }
        Ok(Self(CreatePoolIxArgs::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&CREATE_POOL_IX_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
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
pub fn create_pool_ix(
    keys: CreatePoolKeys,
    args: CreatePoolIxArgs,
) -> std::io::Result<Instruction> {
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
pub fn create_pool_invoke(
    accounts: CreatePoolAccounts<'_, '_>,
    args: CreatePoolIxArgs,
) -> ProgramResult {
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
        (*accounts.pool.key, keys.pool),
        (*accounts.global_config.key, keys.global_config),
        (*accounts.creator.key, keys.creator),
        (*accounts.base_mint.key, keys.base_mint),
        (*accounts.quote_mint.key, keys.quote_mint),
        (*accounts.lp_mint.key, keys.lp_mint),
        (*accounts.user_base_token_account.key, keys.user_base_token_account),
        (*accounts.user_quote_token_account.key, keys.user_quote_token_account),
        (*accounts.user_pool_token_account.key, keys.user_pool_token_account),
        (*accounts.pool_base_token_account.key, keys.pool_base_token_account),
        (*accounts.pool_quote_token_account.key, keys.pool_quote_token_account),
        (*accounts.system_program.key, keys.system_program),
        (*accounts.token_2022_program.key, keys.token_2022_program),
        (*accounts.base_token_program.key, keys.base_token_program),
        (*accounts.quote_token_program.key, keys.quote_token_program),
        (*accounts.associated_token_program.key, keys.associated_token_program),
        (*accounts.event_authority.key, keys.event_authority),
        (*accounts.program.key, keys.program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}

pub fn create_pool_verify_is_writable_privileges<'me, 'info>(
    accounts: CreatePoolAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_is_writable in [
        accounts.pool,
        accounts.creator,
        accounts.lp_mint,
        accounts.user_base_token_account,
        accounts.user_quote_token_account,
        accounts.user_pool_token_account,
        accounts.pool_base_token_account,
        accounts.pool_quote_token_account,
    ] {
        if !should_be_is_writable.is_writable {
            return Err((should_be_is_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}

pub fn create_pool_verify_is_signer_privileges<'me, 'info>(
    accounts: CreatePoolAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_is_signer in [accounts.creator] {
        if !should_be_is_signer.is_signer {
            return Err((should_be_is_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn create_pool_verify_account_privileges<'me, 'info>(
    accounts: CreatePoolAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    create_pool_verify_is_writable_privileges(accounts)?;
    create_pool_verify_is_signer_privileges(accounts)?;
    Ok(())
}
pub const DEPOSIT_IX_ACCOUNTS_LEN: usize = 15;
#[derive(Copy, Clone, Debug)]
pub struct DepositAccounts<'me, 'info> {
    pub pool: &'me AccountInfo<'info>,
    pub global_config: &'me AccountInfo<'info>,
    pub user: &'me AccountInfo<'info>,
    pub base_mint: &'me AccountInfo<'info>,
    pub quote_mint: &'me AccountInfo<'info>,
    pub lp_mint: &'me AccountInfo<'info>,
    pub user_base_token_account: &'me AccountInfo<'info>,
    pub user_quote_token_account: &'me AccountInfo<'info>,
    pub user_pool_token_account: &'me AccountInfo<'info>,
    pub pool_base_token_account: &'me AccountInfo<'info>,
    pub pool_quote_token_account: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub token_2022_program: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct DepositKeys {
    pub pool: Pubkey,
    pub global_config: Pubkey,
    pub user: Pubkey,
    pub base_mint: Pubkey,
    pub quote_mint: Pubkey,
    pub lp_mint: Pubkey,
    pub user_base_token_account: Pubkey,
    pub user_quote_token_account: Pubkey,
    pub user_pool_token_account: Pubkey,
    pub pool_base_token_account: Pubkey,
    pub pool_quote_token_account: Pubkey,
    pub token_program: Pubkey,
    pub token_2022_program: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<DepositAccounts<'_, '_>> for DepositKeys {
    fn from(accounts: DepositAccounts) -> Self {
        Self {
            pool: *accounts.pool.key,
            global_config: *accounts.global_config.key,
            user: *accounts.user.key,
            base_mint: *accounts.base_mint.key,
            quote_mint: *accounts.quote_mint.key,
            lp_mint: *accounts.lp_mint.key,
            user_base_token_account: *accounts.user_base_token_account.key,
            user_quote_token_account: *accounts.user_quote_token_account.key,
            user_pool_token_account: *accounts.user_pool_token_account.key,
            pool_base_token_account: *accounts.pool_base_token_account.key,
            pool_quote_token_account: *accounts.pool_quote_token_account.key,
            token_program: *accounts.token_program.key,
            token_2022_program: *accounts.token_2022_program.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<DepositKeys> for [AccountMeta; DEPOSIT_IX_ACCOUNTS_LEN] {
    fn from(keys: DepositKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.pool,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.global_config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.user,
                is_signer: true,
                is_writable: false,
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
                pubkey: keys.lp_mint,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_base_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_quote_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_pool_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.pool_base_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.pool_quote_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_2022_program,
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
impl From<[Pubkey; DEPOSIT_IX_ACCOUNTS_LEN]> for DepositKeys {
    fn from(pubkeys: [Pubkey; DEPOSIT_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            pool: pubkeys[0],
            global_config: pubkeys[1],
            user: pubkeys[2],
            base_mint: pubkeys[3],
            quote_mint: pubkeys[4],
            lp_mint: pubkeys[5],
            user_base_token_account: pubkeys[6],
            user_quote_token_account: pubkeys[7],
            user_pool_token_account: pubkeys[8],
            pool_base_token_account: pubkeys[9],
            pool_quote_token_account: pubkeys[10],
            token_program: pubkeys[11],
            token_2022_program: pubkeys[12],
            event_authority: pubkeys[13],
            program: pubkeys[14],
        }
    }
}
impl<'info> From<DepositAccounts<'_, 'info>>
    for [AccountInfo<'info>; DEPOSIT_IX_ACCOUNTS_LEN]
{
    fn from(accounts: DepositAccounts<'_, 'info>) -> Self {
        [
            accounts.pool.clone(),
            accounts.global_config.clone(),
            accounts.user.clone(),
            accounts.base_mint.clone(),
            accounts.quote_mint.clone(),
            accounts.lp_mint.clone(),
            accounts.user_base_token_account.clone(),
            accounts.user_quote_token_account.clone(),
            accounts.user_pool_token_account.clone(),
            accounts.pool_base_token_account.clone(),
            accounts.pool_quote_token_account.clone(),
            accounts.token_program.clone(),
            accounts.token_2022_program.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; DEPOSIT_IX_ACCOUNTS_LEN]>
    for DepositAccounts<'me, 'info>
{
    fn from(arr: &'me [AccountInfo<'info>; DEPOSIT_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            pool: &arr[0],
            global_config: &arr[1],
            user: &arr[2],
            base_mint: &arr[3],
            quote_mint: &arr[4],
            lp_mint: &arr[5],
            user_base_token_account: &arr[6],
            user_quote_token_account: &arr[7],
            user_pool_token_account: &arr[8],
            pool_base_token_account: &arr[9],
            pool_quote_token_account: &arr[10],
            token_program: &arr[11],
            token_2022_program: &arr[12],
            event_authority: &arr[13],
            program: &arr[14],
        }
    }
}

pub const DEPOSIT_IX_DISCM: [u8; 8] =[242, 35, 198, 137, 82, 225, 242, 182];
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct DepositIxArgs {
    pub lp_token_amount_out: u64,
    pub max_base_amount_in: u64,
    pub max_quote_amount_in: u64,
}

#[derive(Clone, Debug, PartialEq)]
pub struct DepositIxData(pub DepositIxArgs);
impl From<DepositIxArgs> for DepositIxData {
    fn from(args: DepositIxArgs) -> Self {
        Self(args)
    }
}
impl DepositIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != DEPOSIT_IX_DISCM {
            return Err(
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!(
                        "discm does not match. Expected: {:?}. Received: {:?}",
                        DEPOSIT_IX_DISCM, maybe_discm
                    ),
                ),
            );
        }
        Ok(Self(DepositIxArgs::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&DEPOSIT_IX_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn deposit_ix_with_program_id(
    program_id: Pubkey,
    keys: DepositKeys,
    args: DepositIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; DEPOSIT_IX_ACCOUNTS_LEN] = keys.into();
    let data: DepositIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn deposit_ix(
    keys: DepositKeys,
    args: DepositIxArgs,
) -> std::io::Result<Instruction> {
    deposit_ix_with_program_id(crate::ID, keys, args)
}
pub fn deposit_invoke_with_program_id(
    program_id: Pubkey,
    accounts: DepositAccounts<'_, '_>,
    args: DepositIxArgs,
) -> ProgramResult {
    let keys: DepositKeys = accounts.into();
    let ix = deposit_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn deposit_invoke(
    accounts: DepositAccounts<'_, '_>,
    args: DepositIxArgs,
) -> ProgramResult {
    deposit_invoke_with_program_id(crate::ID, accounts, args)
}
pub fn deposit_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: DepositAccounts<'_, '_>,
    args: DepositIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: DepositKeys = accounts.into();
    let ix = deposit_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn deposit_invoke_signed(
    accounts: DepositAccounts<'_, '_>,
    args: DepositIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    deposit_invoke_signed_with_program_id(crate::ID, accounts, args, seeds)
}
pub fn deposit_verify_account_keys(
    accounts: DepositAccounts<'_, '_>,
    keys: DepositKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.pool.key, keys.pool),
        (*accounts.global_config.key, keys.global_config),
        (*accounts.user.key, keys.user),
        (*accounts.base_mint.key, keys.base_mint),
        (*accounts.quote_mint.key, keys.quote_mint),
        (*accounts.lp_mint.key, keys.lp_mint),
        (*accounts.user_base_token_account.key, keys.user_base_token_account),
        (*accounts.user_quote_token_account.key, keys.user_quote_token_account),
        (*accounts.user_pool_token_account.key, keys.user_pool_token_account),
        (*accounts.pool_base_token_account.key, keys.pool_base_token_account),
        (*accounts.pool_quote_token_account.key, keys.pool_quote_token_account),
        (*accounts.token_program.key, keys.token_program),
        (*accounts.token_2022_program.key, keys.token_2022_program),
        (*accounts.event_authority.key, keys.event_authority),
        (*accounts.program.key, keys.program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn deposit_verify_is_writable_privileges<'me, 'info>(
    accounts: DepositAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_is_writable in [
        accounts.pool,
        accounts.lp_mint,
        accounts.user_base_token_account,
        accounts.user_quote_token_account,
        accounts.user_pool_token_account,
        accounts.pool_base_token_account,
        accounts.pool_quote_token_account,
    ] {
        if !should_be_is_writable.is_writable {
            return Err((should_be_is_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}

pub fn deposit_verify_is_signer_privileges<'me, 'info>(
    accounts: DepositAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_is_signer in [accounts.user] {
        if !should_be_is_signer.is_signer {
            return Err((should_be_is_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn deposit_verify_account_privileges<'me, 'info>(
    accounts: DepositAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    deposit_verify_is_writable_privileges(accounts)?;
    deposit_verify_is_signer_privileges(accounts)?;
    Ok(())
}
pub const DISABLE_IX_ACCOUNTS_LEN: usize = 4;
#[derive(Copy, Clone, Debug)]
pub struct DisableAccounts<'me, 'info> {
    pub admin: &'me AccountInfo<'info>,          
    pub global_config: &'me AccountInfo<'info>,   
    pub event_authority: &'me AccountInfo<'info>, 
    pub program: &'me AccountInfo<'info>,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct DisableKeys {
    pub admin: Pubkey,
    pub global_config: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey
}
impl From<DisableAccounts<'_, '_>> for DisableKeys {
    fn from(accounts: DisableAccounts) -> Self {
        Self {
            admin: *accounts.admin.key,
            global_config: *accounts.global_config.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<DisableKeys> for [AccountMeta; DISABLE_IX_ACCOUNTS_LEN] {
    fn from(keys: DisableKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.admin,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.global_config,
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
impl From<[Pubkey; DISABLE_IX_ACCOUNTS_LEN]> for DisableKeys {
    fn from(pubkeys: [Pubkey; DISABLE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            admin: pubkeys[0],
            global_config: pubkeys[1],
            event_authority: pubkeys[2],
            program: pubkeys[3],
        }
    }
}
impl<'info> From<DisableAccounts<'_, 'info>> for [AccountInfo<'info>; DISABLE_IX_ACCOUNTS_LEN] {
    fn from(accounts: DisableAccounts<'_, 'info>) -> Self {
        [
            accounts.admin.clone(),
            accounts.global_config.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; DISABLE_IX_ACCOUNTS_LEN]>
    for DisableAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; DISABLE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            admin: &arr[0],
            global_config: &arr[1],
            event_authority: &arr[2],
            program: &arr[3],
        }
    }
}
pub const DISABLE_IX_DISCM: [u8; 8] = [185, 173, 187, 90, 216, 15, 238, 233];
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct DisableIxArgs {
    pub disable_create_pool: bool,
    pub disable_deposit: bool,
    pub disable_withdraw: bool,
    pub disable_buy: bool,
    pub disable_sell: bool,
}
#[derive(Clone, Debug, PartialEq)]
pub struct DisableIxData(pub DisableIxArgs);
impl From<DisableIxArgs> for DisableIxData {
    fn from(args: DisableIxArgs) -> Self {
        Self(args)
    }
}
impl DisableIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != DISABLE_IX_DISCM {
            return Err(
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!(
                        "discm does not match. Expected: {:?}. Received: {:?}",
                        DISABLE_IX_DISCM, maybe_discm
                    ),
                ),
            );
        }
        Ok(Self(DisableIxArgs::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&DISABLE_IX_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn disable_ix_with_program_id(
    program_id: Pubkey,
    keys: DisableKeys,
    args: DisableIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; DISABLE_IX_ACCOUNTS_LEN] = keys.into();
    let data: DisableIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn disable_ix(
    keys: DisableKeys,
    args: DisableIxArgs,
) -> std::io::Result<Instruction> {
    disable_ix_with_program_id(crate::ID, keys, args)
}
pub fn disable_invoke_with_program_id(
    program_id: Pubkey,
    accounts: DisableAccounts<'_, '_>,
    args: DisableIxArgs,
) -> ProgramResult {
    let keys: DisableKeys = accounts.into();
    let ix = disable_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn disable_invoke(
    accounts: DisableAccounts<'_, '_>,
    args: DisableIxArgs,
) -> ProgramResult {
    disable_invoke_with_program_id(crate::ID, accounts, args)
}
pub fn disable_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: DisableAccounts<'_, '_>,
    args: DisableIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: DisableKeys = accounts.into();
    let ix = disable_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn disable_invoke_signed(
    accounts: DisableAccounts<'_, '_>,
    args: DisableIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    disable_invoke_signed_with_program_id(crate::ID, accounts, args, seeds)
}
pub fn disable_verify_account_keys(
    accounts: DisableAccounts<'_, '_>,
    keys: DisableKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.admin.key, keys.admin),
        (*accounts.global_config.key, keys.global_config),
        (*accounts.event_authority.key, keys.event_authority),
        (*accounts.program.key, keys.program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}

pub fn disable_verify_is_writable_privileges<'me, 'info>(
    accounts: DisableAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_is_writable in [
        accounts.global_config, 
    ] {
        if !should_be_is_writable.is_writable {
            return Err((should_be_is_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}

pub fn disable_verify_is_signer_privileges<'me, 'info>(
    accounts: DisableAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_is_signer in [accounts.admin] {
        if !should_be_is_signer.is_signer {
            return Err((should_be_is_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn disable_verify_account_privileges<'me, 'info>(
    accounts: DisableAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    disable_verify_is_writable_privileges(accounts)?;
    disable_verify_is_signer_privileges(accounts)?;
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
    pub program: Pubkey
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
            program: pubkeys[4]
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
impl<'me, 'info> From<&'me [AccountInfo<'info>; EXTEND_ACCOUNT_IX_ACCOUNTS_LEN]>
for ExtendAccountAccounts<'me, 'info> {
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
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
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
            return Err(
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!(
                        "discm does not match. Expected: {:?}. Received: {:?}",
                        EXTEND_ACCOUNT_IX_DISCM, maybe_discm
                    ),
                ),
            );
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
pub fn extend_account_ix(
    keys: ExtendAccountKeys,
    args: ExtendAccountIxArgs,
) -> std::io::Result<Instruction> {
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
pub fn extend_account_invoke(
    accounts: ExtendAccountAccounts<'_, '_>,
    args: ExtendAccountIxArgs,
) -> ProgramResult {
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
pub fn extend_account_verify_is_writable_privileges<'me, 'info>(
    accounts: ExtendAccountAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_is_writable in [accounts.account] {
        if !should_be_is_writable.is_writable {
            return Err((should_be_is_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn extend_account_verify_is_signer_privileges<'me, 'info>(
    accounts: ExtendAccountAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_is_signer in [accounts.user] {
        if !should_be_is_signer.is_signer {
            return Err((should_be_is_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn extend_account_verify_account_privileges<'me, 'info>(
    accounts: ExtendAccountAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    extend_account_verify_is_writable_privileges(accounts)?;
    extend_account_verify_is_signer_privileges(accounts)?;
    Ok(())
}
pub const SELL_IX_ACCOUNTS_LEN: usize = 17;

#[derive(Copy, Clone, Debug)]
pub struct SellAccounts<'me, 'info> {
    pub pool: &'me AccountInfo<'info>,
    pub user: &'me AccountInfo<'info>,
    pub global_config: &'me AccountInfo<'info>,
    pub base_mint: &'me AccountInfo<'info>,
    pub quote_mint: &'me AccountInfo<'info>,
    pub user_base_token_account: &'me AccountInfo<'info>,
    pub user_quote_token_account: &'me AccountInfo<'info>,
    pub pool_base_token_account: &'me AccountInfo<'info>,
    pub pool_quote_token_account: &'me AccountInfo<'info>,
    pub protocol_fee_recipient: &'me AccountInfo<'info>,
    pub protocol_fee_recipient_token_account: &'me AccountInfo<'info>,
    pub base_token_program: &'me AccountInfo<'info>,
    pub quote_token_program: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub associated_token_program: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SellKeys {
    pub pool: Pubkey,
    pub user: Pubkey,
    pub global_config: Pubkey,
    pub base_mint: Pubkey,
    pub quote_mint: Pubkey,
    pub user_base_token_account: Pubkey,
    pub user_quote_token_account: Pubkey,
    pub pool_base_token_account: Pubkey,
    pub pool_quote_token_account: Pubkey,
    pub protocol_fee_recipient: Pubkey,
    pub protocol_fee_recipient_token_account: Pubkey,
    pub base_token_program: Pubkey,
    pub quote_token_program: Pubkey,
    pub system_program: Pubkey,
    pub associated_token_program: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}

impl From<SellAccounts<'_, '_>> for SellKeys {
    fn from(accounts: SellAccounts) -> Self {
        Self {
            pool: *accounts.pool.key,
            user: *accounts.user.key,
            global_config: *accounts.global_config.key,
            base_mint: *accounts.base_mint.key,
            quote_mint: *accounts.quote_mint.key,
            user_base_token_account: *accounts.user_base_token_account.key,
            user_quote_token_account: *accounts.user_quote_token_account.key,
            pool_base_token_account: *accounts.pool_base_token_account.key,
            pool_quote_token_account: *accounts.pool_quote_token_account.key,
            protocol_fee_recipient: *accounts.protocol_fee_recipient.key,
            protocol_fee_recipient_token_account: *accounts.protocol_fee_recipient_token_account.key,
            base_token_program: *accounts.base_token_program.key,
            quote_token_program: *accounts.quote_token_program.key,
            system_program: *accounts.system_program.key,
            associated_token_program: *accounts.associated_token_program.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<SellKeys> for [AccountMeta; SELL_IX_ACCOUNTS_LEN] {
    fn from(keys: SellKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.pool,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.user,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.global_config,
                is_signer: false,
                is_writable: false,
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
                pubkey: keys.user_base_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_quote_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.pool_base_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.pool_quote_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.protocol_fee_recipient,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.protocol_fee_recipient_token_account,
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
        ]
    }
}
impl From<[Pubkey; SELL_IX_ACCOUNTS_LEN]> for SellKeys {
    fn from(pubkeys: [Pubkey; SELL_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            pool: pubkeys[0],
            user: pubkeys[1],
            global_config: pubkeys[2],
            base_mint: pubkeys[3],
            quote_mint: pubkeys[4],
            user_base_token_account: pubkeys[5],
            user_quote_token_account: pubkeys[6],
            pool_base_token_account: pubkeys[7],
            pool_quote_token_account: pubkeys[8],
            protocol_fee_recipient: pubkeys[9],
            protocol_fee_recipient_token_account: pubkeys[10],
            base_token_program: pubkeys[11],
            quote_token_program: pubkeys[12],
            system_program: pubkeys[13],
            associated_token_program: pubkeys[14],
            event_authority: pubkeys[15],
            program: pubkeys[16],
        }
    }
}
impl<'info> From<SellAccounts<'_, 'info>> for [AccountInfo<'info>; SELL_IX_ACCOUNTS_LEN] {
    fn from(accounts: SellAccounts<'_, 'info>) -> Self {
        [
            accounts.pool.clone(),
            accounts.user.clone(),
            accounts.global_config.clone(),
            accounts.base_mint.clone(),
            accounts.quote_mint.clone(),
            accounts.user_base_token_account.clone(),
            accounts.user_quote_token_account.clone(),
            accounts.pool_base_token_account.clone(),
            accounts.pool_quote_token_account.clone(),
            accounts.protocol_fee_recipient.clone(),
            accounts.protocol_fee_recipient_token_account.clone(),
            accounts.base_token_program.clone(),
            accounts.quote_token_program.clone(),
            accounts.system_program.clone(),
            accounts.associated_token_program.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; SELL_IX_ACCOUNTS_LEN]>
for SellAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; SELL_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            pool: &arr[0],
            user: &arr[1],
            global_config: &arr[2],
            base_mint: &arr[3],
            quote_mint: &arr[4],
            user_base_token_account: &arr[5],
            user_quote_token_account: &arr[6],
            pool_base_token_account: &arr[7],
            pool_quote_token_account: &arr[8],
            protocol_fee_recipient: &arr[9],
            protocol_fee_recipient_token_account: &arr[10],
            base_token_program: &arr[11],
            quote_token_program: &arr[12],
            system_program: &arr[13],
            associated_token_program: &arr[14],
            event_authority: &arr[15],
            program: &arr[16],
        }
    }
}

pub const SELL_IX_DISCM: [u8; 8] = [51, 230, 133, 164, 1, 127, 131, 173];

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SellIxArgs {
    pub base_amount_in: u64,
    pub min_quote_amount_out: u64,
}

#[derive(Clone, Debug, PartialEq)]
pub struct SellIxData(pub SellIxArgs);
impl From<SellIxArgs> for SellIxData {
    fn from(args: SellIxArgs) -> Self {
        Self(args)
    }
}
impl SellIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != SELL_IX_DISCM {
            return Err(
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!(
                        "discm does not match. Expected: {:?}. Received: {:?}",
                        SELL_IX_DISCM, maybe_discm
                    ),
                ),
            );
        }
        Ok(Self(SellIxArgs::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&SELL_IX_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
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
pub fn sell_ix(
    keys: SellKeys,
    args: SellIxArgs,
) -> std::io::Result<Instruction> {
    sell_ix_with_program_id(crate::ID, keys, args)
}
pub fn sell_invoke_with_program_id(
    program_id: Pubkey,
    accounts: SellAccounts<'_, '_>,
    args: SellIxArgs,
) -> ProgramResult {
    let keys: SellKeys = accounts.into();
    let ix = sell_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn sell_invoke(
    accounts: SellAccounts<'_, '_>,
    args: SellIxArgs,
) -> ProgramResult {
    sell_invoke_with_program_id(crate::ID, accounts, args)
}
pub fn sell_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: SellAccounts<'_, '_>,
    args: SellIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: SellKeys = accounts.into();
    let ix = sell_ix_with_program_id(program_id, keys, args)?;
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
        (*accounts.pool.key, keys.pool),
        (*accounts.user.key, keys.user),
        (*accounts.global_config.key, keys.global_config),
        (*accounts.base_mint.key, keys.base_mint),
        (*accounts.quote_mint.key, keys.quote_mint),
        (*accounts.user_base_token_account.key, keys.user_base_token_account),
        (*accounts.user_quote_token_account.key, keys.user_quote_token_account),
        (*accounts.pool_base_token_account.key, keys.pool_base_token_account),
        (*accounts.pool_quote_token_account.key, keys.pool_quote_token_account),
        (*accounts.protocol_fee_recipient.key, keys.protocol_fee_recipient),
        (*accounts.protocol_fee_recipient_token_account.key, keys.protocol_fee_recipient_token_account),
        (*accounts.base_token_program.key, keys.base_token_program),
        (*accounts.quote_token_program.key, keys.quote_token_program),
        (*accounts.system_program.key, keys.system_program),
        (*accounts.associated_token_program.key, keys.associated_token_program),
        (*accounts.event_authority.key, keys.event_authority),
        (*accounts.program.key, keys.program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn sell_verify_is_writable_privileges<'me, 'info>(
    accounts: SellAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_is_writable in [
        accounts.user,
        accounts.user_base_token_account,
        accounts.user_quote_token_account,
        accounts.pool_base_token_account,
        accounts.pool_quote_token_account,
        accounts.protocol_fee_recipient_token_account,
    ] {
        if !should_be_is_writable.is_writable {
            return Err((should_be_is_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn sell_verify_is_signer_privileges<'me, 'info>(
    accounts: SellAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_is_signer in [accounts.user] {
        if !should_be_is_signer.is_signer {
            return Err((should_be_is_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn sell_verify_account_privileges<'me, 'info>(
    accounts: SellAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    sell_verify_is_writable_privileges(accounts)?;
    sell_verify_is_signer_privileges(accounts)?;
    Ok(())
}
pub const UPDATE_ADMIN_IX_ACCOUNTS_LEN: usize = 5;

#[derive(Copy, Clone, Debug)]
pub struct UpdateAdminAccounts<'me, 'info> {
    pub admin: &'me AccountInfo<'info>,
    pub global_config: &'me AccountInfo<'info>,
    pub new_admin: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug)]
pub struct UpdateAdminKeys {
    pub admin: Pubkey,
    pub global_config: Pubkey,
    pub new_admin: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<UpdateAdminAccounts<'_, '_>> for UpdateAdminKeys {
    fn from(accounts: UpdateAdminAccounts) -> Self {
        Self {
            admin: *accounts.admin.key,
            global_config: *accounts.global_config.key,
            new_admin: *accounts.new_admin.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<UpdateAdminKeys> for [AccountMeta; UPDATE_ADMIN_IX_ACCOUNTS_LEN] {
    fn from(keys: UpdateAdminKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.admin,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.global_config,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.new_admin,
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
impl From<[Pubkey; UPDATE_ADMIN_IX_ACCOUNTS_LEN]> for UpdateAdminKeys {
    fn from(pubkeys: [Pubkey; UPDATE_ADMIN_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            admin: pubkeys[0],
            global_config: pubkeys[1],
            new_admin: pubkeys[2],
            event_authority: pubkeys[3],
            program: pubkeys[4],
        }
    }
}
impl<'info> From<UpdateAdminAccounts<'_, 'info>> for [AccountInfo<'info>; UPDATE_ADMIN_IX_ACCOUNTS_LEN] {
    fn from(accounts: UpdateAdminAccounts<'_, 'info>) -> Self {
        [
            accounts.admin.clone(),
            accounts.global_config.clone(),
            accounts.new_admin.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; UPDATE_ADMIN_IX_ACCOUNTS_LEN]>
for UpdateAdminAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; UPDATE_ADMIN_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            admin: &arr[0],
            global_config: &arr[1],
            new_admin: &arr[2],
            event_authority: &arr[3],
            program: &arr[4]
        }
    }
}
pub const UPDATE_ADMIN_IX_DISCM: [u8; 8] =  [161, 176, 40, 213, 60, 184, 179, 228];
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct UpdateAdminIxArgs;
#[derive(Clone, Debug, PartialEq)]
pub struct UpdateAdminIxData(pub UpdateAdminIxArgs);
impl From<UpdateAdminIxArgs> for UpdateAdminIxData {
    fn from(args: UpdateAdminIxArgs) -> Self {
        Self(args)
    }
}
impl UpdateAdminIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != UPDATE_ADMIN_IX_DISCM {
            return Err(
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!(
                        "discm does not match. Expected: {:?}. Received: {:?}",
                        UPDATE_ADMIN_IX_DISCM, maybe_discm
                    ),
                ),
            );
        }
        Ok(Self(UpdateAdminIxArgs::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&UPDATE_ADMIN_IX_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn update_admin_ix_with_program_id(
    program_id: Pubkey,
    keys: UpdateAdminKeys,
    args: UpdateAdminIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; UPDATE_ADMIN_IX_ACCOUNTS_LEN] = keys.into();
    let data: UpdateAdminIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn update_admin_ix(
    keys: UpdateAdminKeys,
    args: UpdateAdminIxArgs,
) -> std::io::Result<Instruction> {
    update_admin_ix_with_program_id(crate::ID, keys, args)
}
pub fn update_admin_invoke_with_program_id(
    program_id: Pubkey,
    accounts: UpdateAdminAccounts<'_, '_>,
    args: UpdateAdminIxArgs,
) -> ProgramResult {
    let keys: UpdateAdminKeys = accounts.into();
    let ix = update_admin_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn update_admin_invoke(
    accounts: UpdateAdminAccounts<'_, '_>,
    args: UpdateAdminIxArgs,
) -> ProgramResult {
    update_admin_invoke_with_program_id(crate::ID, accounts, args)
}
pub fn update_admin_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: UpdateAdminAccounts<'_, '_>,
    args: UpdateAdminIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: UpdateAdminKeys = accounts.into();
    let ix = update_admin_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn update_admin_invoke_signed(
    accounts: UpdateAdminAccounts<'_, '_>,
    args: UpdateAdminIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    update_admin_invoke_signed_with_program_id(crate::ID, accounts, args, seeds)
}
pub fn update_admin_verify_account_keys(
    accounts: UpdateAdminAccounts<'_, '_>,
    keys: UpdateAdminKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.admin.key, keys.admin),
        (*accounts.global_config.key, keys.global_config),
        (*accounts.new_admin.key, keys.new_admin),
        (*accounts.event_authority.key, keys.event_authority),
        (*accounts.program.key, keys.program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn update_admin_verify_is_writable_privileges<'me, 'info>(
    accounts: UpdateAdminAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_is_writable in [
        accounts.global_config
    ] {
        if !should_be_is_writable.is_writable {
            return Err((should_be_is_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn update_admin_verify_is_signer_privileges<'me, 'info>(
    accounts: UpdateAdminAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_is_signer in [accounts.admin] {
        if !should_be_is_signer.is_signer {
            return Err((should_be_is_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn update_admin_verify_account_privileges<'me, 'info>(
    accounts: UpdateAdminAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    update_admin_verify_is_writable_privileges(accounts)?;
    update_admin_verify_is_signer_privileges(accounts)?;
    Ok(())
}

pub const UPDATE_FEE_CONFIG_ACCOUNTS_LEN: usize = 4;
#[derive(Copy, Clone, Debug)]
pub struct UpdateFeeConfigAccounts<'me, 'info> {
    pub admin: &'me AccountInfo<'info>,
    pub global_config: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct UpdateFeeConfigKeys {
    pub admin: Pubkey,
    pub global_config: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<UpdateFeeConfigAccounts<'_, '_>> for UpdateFeeConfigKeys {
    fn from(accounts: UpdateFeeConfigAccounts) -> Self {
        Self {
            admin: *accounts.admin.key,
            global_config: *accounts.global_config.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<UpdateFeeConfigKeys> for [AccountMeta; UPDATE_FEE_CONFIG_ACCOUNTS_LEN] {
    fn from(keys: UpdateFeeConfigKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.admin,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.global_config,
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
impl From<[Pubkey; UPDATE_FEE_CONFIG_ACCOUNTS_LEN]> for UpdateFeeConfigKeys {
    fn from(pubkeys: [Pubkey; UPDATE_FEE_CONFIG_ACCOUNTS_LEN]) -> Self {
        Self {
            admin: pubkeys[0],
            global_config: pubkeys[1],
            event_authority: pubkeys[2],
            program: pubkeys[3],
        }
    }
}
impl<'info> From<UpdateFeeConfigAccounts<'_, 'info>> for [AccountInfo<'info>; UPDATE_FEE_CONFIG_ACCOUNTS_LEN] {
    fn from(accounts: UpdateFeeConfigAccounts<'_, 'info>) -> Self {
        [
            accounts.admin.clone(),
            accounts.global_config.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}

impl<'me, 'info> From<&'me [AccountInfo<'info>; UPDATE_FEE_CONFIG_ACCOUNTS_LEN]>
for UpdateFeeConfigAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; UPDATE_FEE_CONFIG_ACCOUNTS_LEN]) -> Self {
        Self {
            admin: &arr[0],
            global_config: &arr[1],
            event_authority: &arr[2],
            program: &arr[3],
        }
    }
}
pub const UPDATE_FEE_CONFIG_IX_DISCM: [u8; 8] = [143, 190, 90, 218, 196, 30, 51, 222];

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct UpdateFeeConfigIxArgs {
    pub lp_fee_basis_points: u64,
    pub protocol_fee_basis_points: u64,
    pub protocol_fee_recipients: [Pubkey; 8], 
}

#[derive(Clone, Debug, PartialEq)]
pub struct UpdateFeeConfigIxData(pub UpdateFeeConfigIxArgs);
impl From<UpdateFeeConfigIxArgs> for UpdateFeeConfigIxData {
    fn from(args: UpdateFeeConfigIxArgs) -> Self {
        Self(args)
    }
}
impl UpdateFeeConfigIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != UPDATE_FEE_CONFIG_IX_DISCM {
            return Err(
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!(
                        "discm does not match. Expected: {:?}. Received: {:?}",
                        UPDATE_FEE_CONFIG_IX_DISCM, maybe_discm
                    ),
                ),
            );
        }
        Ok(Self(UpdateFeeConfigIxArgs::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&UPDATE_FEE_CONFIG_IX_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn update_fee_config_ix_with_program_id(
    program_id: Pubkey,
    keys: UpdateFeeConfigKeys,
    args: UpdateFeeConfigIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; UPDATE_FEE_CONFIG_ACCOUNTS_LEN] = keys.into();
    let data: UpdateFeeConfigIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn update_fee_config_ix(
    keys: UpdateFeeConfigKeys,
    args: UpdateFeeConfigIxArgs,
) -> std::io::Result<Instruction> {
    update_fee_config_ix_with_program_id(crate::ID, keys, args)
}
pub fn update_fee_config_invoke_with_program_id(
    program_id: Pubkey,
    accounts: UpdateFeeConfigAccounts<'_, '_>,
    args: UpdateFeeConfigIxArgs,
) -> ProgramResult {
    let keys: UpdateFeeConfigKeys = accounts.into();
    let ix = update_fee_config_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn update_fee_config_invoke(
    accounts: UpdateFeeConfigAccounts<'_, '_>,
    args: UpdateFeeConfigIxArgs,
) -> ProgramResult {
    update_fee_config_invoke_with_program_id(crate::ID, accounts, args)
}
pub fn update_fee_config_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: UpdateFeeConfigAccounts<'_, '_>,
    args: UpdateFeeConfigIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: UpdateFeeConfigKeys = accounts.into();
    let ix = update_fee_config_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn update_fee_config_invoke_signed(
    accounts: UpdateFeeConfigAccounts<'_, '_>,
    args: UpdateFeeConfigIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    update_fee_config_invoke_signed_with_program_id(crate::ID, accounts, args, seeds)
}
pub fn update_fee_config_verify_account_keys(
    accounts: UpdateFeeConfigAccounts<'_, '_>,
    keys: UpdateFeeConfigKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.admin.key, keys.admin),
        (*accounts.global_config.key, keys.global_config),
        (*accounts.event_authority.key, keys.event_authority),
        (*accounts.program.key, keys.program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn update_fee_config_verify_is_writable_privileges<'me, 'info>(
    accounts: UpdateFeeConfigAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_is_writable in [
        accounts.global_config, 
    ] {
        if !should_be_is_writable.is_writable {
            return Err((should_be_is_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}

pub fn update_fee_config_verify_is_signer_privileges<'me, 'info>(
    accounts: UpdateFeeConfigAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_is_signer in [accounts.admin] {
        if !should_be_is_signer.is_signer {
            return Err((should_be_is_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn update_fee_config_verify_account_privileges<'me, 'info>(
    accounts: UpdateFeeConfigAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    update_fee_config_verify_is_writable_privileges(accounts)?;
    update_fee_config_verify_is_signer_privileges(accounts)?;
    Ok(())
}
pub const WITHDRAW_ACCOUNTS_LEN: usize = 15; 

#[derive(Copy, Clone, Debug)]
pub struct WithdrawAccounts<'me, 'info> {
    pub pool: &'me AccountInfo<'info>,                         
    pub global_config: &'me AccountInfo<'info>,                    
    pub user: &'me AccountInfo<'info>,                     
    pub base_mint: &'me AccountInfo<'info>,                 
    pub quote_mint: &'me AccountInfo<'info>,       
    pub lp_mint: &'me AccountInfo<'info>,     
    pub user_base_token_account: &'me AccountInfo<'info>,      
    pub user_quote_token_account: &'me AccountInfo<'info>,       
    pub user_pool_token_account: &'me AccountInfo<'info>,      
    pub pool_base_token_account: &'me AccountInfo<'info>,                 
    pub pool_quote_token_account: &'me AccountInfo<'info>,            
    pub token_program: &'me AccountInfo<'info>,              
    pub token_2022_program: &'me AccountInfo<'info>,                       
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}  
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct WithdrawKeys {
    pub pool: Pubkey,
    pub global_config: Pubkey,
    pub user: Pubkey,
    pub base_mint: Pubkey,
    pub quote_mint: Pubkey,
    pub lp_mint: Pubkey,
    pub user_base_token_account: Pubkey,
    pub user_quote_token_account: Pubkey,
    pub user_pool_token_account: Pubkey,
    pub pool_base_token_account: Pubkey,
    pub pool_quote_token_account: Pubkey,
    pub token_program: Pubkey,
    pub token_2022_program: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<WithdrawAccounts<'_, '_>> for WithdrawKeys {
    fn from(accounts: WithdrawAccounts) -> Self {
        Self {
            pool: *accounts.pool.key,
            global_config: *accounts.global_config.key,
            user: *accounts.user.key,
            base_mint: *accounts.base_mint.key,
            quote_mint: *accounts.quote_mint.key,
            lp_mint: *accounts.lp_mint.key,
            user_base_token_account: *accounts.user_base_token_account.key,
            user_quote_token_account: *accounts.user_quote_token_account.key,
            user_pool_token_account: *accounts.user_pool_token_account.key,
            pool_base_token_account: *accounts.pool_base_token_account.key,
            pool_quote_token_account: *accounts.pool_quote_token_account.key,
            token_program: *accounts.token_program.key,
            token_2022_program: *accounts.token_2022_program.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}

impl From<WithdrawKeys> for [AccountMeta; WITHDRAW_ACCOUNTS_LEN] {
    fn from(keys: WithdrawKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.pool,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.global_config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.user,
                is_signer: true,
                is_writable: false,
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
                pubkey: keys.lp_mint,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_base_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_quote_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_pool_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.pool_base_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.pool_quote_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_2022_program,
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
impl From<[Pubkey; WITHDRAW_ACCOUNTS_LEN]> for WithdrawKeys {
    fn from(pubkeys: [Pubkey; WITHDRAW_ACCOUNTS_LEN]) -> Self {
        Self {
            pool: pubkeys[0],
            global_config: pubkeys[1],
            user: pubkeys[2],
            base_mint: pubkeys[3],
            quote_mint: pubkeys[4],
            lp_mint: pubkeys[5],
            user_base_token_account: pubkeys[6],
            user_quote_token_account: pubkeys[7],
            user_pool_token_account: pubkeys[8],
            pool_base_token_account: pubkeys[9],
            pool_quote_token_account: pubkeys[10],
            token_program: pubkeys[11],
            token_2022_program: pubkeys[12],
            event_authority: pubkeys[13],
            program: pubkeys[14],
        }
    }
}

impl<'info> From<WithdrawAccounts<'_, 'info>>
    for [AccountInfo<'info>; WITHDRAW_ACCOUNTS_LEN]
{
    fn from(accounts: WithdrawAccounts<'_, 'info>) -> Self {
        [
            accounts.pool.clone(),
            accounts.global_config.clone(),
            accounts.user.clone(),
            accounts.base_mint.clone(),
            accounts.quote_mint.clone(),
            accounts.lp_mint.clone(),
            accounts.user_base_token_account.clone(),
            accounts.user_quote_token_account.clone(),
            accounts.user_pool_token_account.clone(),
            accounts.pool_base_token_account.clone(),
            accounts.pool_quote_token_account.clone(),
            accounts.token_program.clone(),
            accounts.token_2022_program.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; WITHDRAW_ACCOUNTS_LEN]>
    for WithdrawAccounts<'me, 'info>
{
    fn from(arr: &'me [AccountInfo<'info>; WITHDRAW_ACCOUNTS_LEN]) -> Self {
        Self {
            pool: &arr[0],
            global_config: &arr[1],
            user: &arr[2],
            base_mint: &arr[3],
            quote_mint: &arr[4],
            lp_mint: &arr[5],
            user_base_token_account: &arr[6],
            user_quote_token_account: &arr[7],
            user_pool_token_account: &arr[8],
            pool_base_token_account: &arr[9],
            pool_quote_token_account: &arr[10],
            token_program: &arr[11],
            token_2022_program: &arr[12],
            event_authority: &arr[13],
            program: &arr[14],
        }
    }
}
pub const WITHDRAW_IX_DISCM: [u8; 8] = [183, 18, 70, 156, 148, 109, 161, 34];
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct WithdrawIxArgs {
    pub lp_token_amount_in: u64,
    pub min_base_amount_out: u64,
    pub min_quote_amount_out: u64,
}

#[derive(Clone, Debug, PartialEq)]
pub struct WithdrawIxData(pub WithdrawIxArgs);
impl From<WithdrawIxArgs> for WithdrawIxData {
    fn from(args: WithdrawIxArgs) -> Self {
        Self(args)
    }
}
impl WithdrawIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != WITHDRAW_IX_DISCM {
            return Err(
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!(
                        "discm does not match. Expected: {:?}. Received: {:?}",
                        WITHDRAW_IX_DISCM, maybe_discm
                    ),
                ),
            );
        }
        Ok(Self(WithdrawIxArgs::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&WITHDRAW_IX_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn withdraw_ix_with_program_id(
    program_id: Pubkey,
    keys: WithdrawKeys,
    args: WithdrawIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; WITHDRAW_ACCOUNTS_LEN] = keys.into();
    let data: WithdrawIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn withdraw_ix(
    keys: WithdrawKeys,
    args: WithdrawIxArgs,
) -> std::io::Result<Instruction> {
    withdraw_ix_with_program_id(crate::ID, keys, args)
}
pub fn withdraw_invoke_with_program_id(
    program_id: Pubkey,
    accounts: WithdrawAccounts<'_, '_>,
    args: WithdrawIxArgs,
) -> ProgramResult {
    let keys: WithdrawKeys = accounts.into();
    let ix = withdraw_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn withdraw_invoke(
    accounts: WithdrawAccounts<'_, '_>,
    args: WithdrawIxArgs,
) -> ProgramResult {
    withdraw_invoke_with_program_id(crate::ID, accounts, args)
}
pub fn withdraw_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: WithdrawAccounts<'_, '_>,
    args: WithdrawIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: WithdrawKeys = accounts.into();
    let ix = withdraw_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn withdraw_invoke_signed(
    accounts: WithdrawAccounts<'_, '_>,
    args: WithdrawIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    withdraw_invoke_signed_with_program_id(crate::ID, accounts, args, seeds)
}
pub fn withdraw_verify_account_keys(
    accounts: WithdrawAccounts<'_, '_>,
    keys: WithdrawKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.pool.key, keys.pool),
        (*accounts.global_config.key, keys.global_config),
        (*accounts.user.key, keys.user),
        (*accounts.base_mint.key, keys.base_mint),
        (*accounts.quote_mint.key, keys.quote_mint),
        (*accounts.lp_mint.key, keys.lp_mint),
        (*accounts.user_base_token_account.key, keys.user_base_token_account),
        (*accounts.user_quote_token_account.key, keys.user_quote_token_account),
        (*accounts.user_pool_token_account.key, keys.user_pool_token_account),
        (*accounts.pool_base_token_account.key, keys.pool_base_token_account),
        (*accounts.pool_quote_token_account.key, keys.pool_quote_token_account),
        (*accounts.token_program.key, keys.token_program),
        (*accounts.token_2022_program.key, keys.token_2022_program),
        (*accounts.event_authority.key, keys.event_authority),
        (*accounts.program.key, keys.program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn withdraw_verify_is_writable_privileges<'me, 'info>(
    accounts: WithdrawAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_is_writable in [
        accounts.pool,
        accounts.lp_mint,
        accounts.user_base_token_account,
        accounts.user_quote_token_account,
        accounts.user_pool_token_account,
        accounts.pool_base_token_account,
        accounts.pool_quote_token_account,
    ] {
        if !should_be_is_writable.is_writable {
            return Err((should_be_is_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}

pub fn withdraw_verify_is_signer_privileges<'me, 'info>(
    accounts: WithdrawAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_is_signer in [accounts.user] {
        if !should_be_is_signer.is_signer {
            return Err((should_be_is_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn withdraw_verify_account_privileges<'me, 'info>(
    accounts: WithdrawAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    withdraw_verify_is_writable_privileges(accounts)?;
    withdraw_verify_is_signer_privileges(accounts)?;
    Ok(())
}
