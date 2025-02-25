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
pub enum RaydiumAmmProgramIx {
    Initialize(InitializeIxArgs),
    Initialize2(Initialize2IxArgs),
    MonitorStep(MonitorStepIxArgs),
    Deposit(DepositIxArgs),
    Withdraw(WithdrawIxArgs),
    MigrateToOpenBook,
    SetParams(SetParamsIxArgs),
    WithdrawPnl,
    WithdrawSrm(WithdrawSrmIxArgs),
    SwapBaseIn(SwapBaseInIxArgs),
    PreInitialize(PreInitializeIxArgs),
    SwapBaseOut(SwapBaseOutIxArgs),
    SimulateInfo(SimulateInfoIxArgs),
    AdminCancelOrders(AdminCancelOrdersIxArgs),
    CreateConfigAccount,
    UpdateConfigAccount(UpdateConfigAccountIxArgs),
}
impl RaydiumAmmProgramIx {
    pub fn name(&self) -> String {
        // Use the ToString derived method to get the enum variant name
        self.to_string().to_camel_case()
    }
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm_buf = [0u8; 1];
        reader.read_exact(&mut maybe_discm_buf)?;
        let maybe_discm = maybe_discm_buf[0];
        match maybe_discm {
            INITIALIZE_IX_DISCM => {
                Ok(Self::Initialize(InitializeIxArgs::deserialize(&mut reader)?))
            }
            INITIALIZE2_IX_DISCM => {
                Ok(Self::Initialize2(Initialize2IxArgs::deserialize(&mut reader)?))
            }
            MONITOR_STEP_IX_DISCM => {
                Ok(Self::MonitorStep(MonitorStepIxArgs::deserialize(&mut reader)?))
            }
            DEPOSIT_IX_DISCM => {
                Ok(Self::Deposit(DepositIxArgs::deserialize(&mut reader)?))
            }
            WITHDRAW_IX_DISCM => {
                Ok(Self::Withdraw(WithdrawIxArgs::deserialize(&mut reader)?))
            }
            MIGRATE_TO_OPEN_BOOK_IX_DISCM => Ok(Self::MigrateToOpenBook),
            SET_PARAMS_IX_DISCM => {
                Ok(Self::SetParams(SetParamsIxArgs::deserialize(&mut reader)?))
            }
            WITHDRAW_PNL_IX_DISCM => Ok(Self::WithdrawPnl),
            WITHDRAW_SRM_IX_DISCM => {
                Ok(Self::WithdrawSrm(WithdrawSrmIxArgs::deserialize(&mut reader)?))
            }
            SWAP_BASE_IN_IX_DISCM => {
                Ok(Self::SwapBaseIn(SwapBaseInIxArgs::deserialize(&mut reader)?))
            }
            PRE_INITIALIZE_IX_DISCM => {
                Ok(Self::PreInitialize(PreInitializeIxArgs::deserialize(&mut reader)?))
            }
            SWAP_BASE_OUT_IX_DISCM => {
                Ok(Self::SwapBaseOut(SwapBaseOutIxArgs::deserialize(&mut reader)?))
            }
            SIMULATE_INFO_IX_DISCM => {
                Ok(Self::SimulateInfo(SimulateInfoIxArgs::deserialize(&mut reader)?))
            }
            ADMIN_CANCEL_ORDERS_IX_DISCM => {
                Ok(
                    Self::AdminCancelOrders(
                        AdminCancelOrdersIxArgs::deserialize(&mut reader)?,
                    ),
                )
            }
            CREATE_CONFIG_ACCOUNT_IX_DISCM => Ok(Self::CreateConfigAccount),
            UPDATE_CONFIG_ACCOUNT_IX_DISCM => {
                Ok(
                    Self::UpdateConfigAccount(
                        UpdateConfigAccountIxArgs::deserialize(&mut reader)?,
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
            Self::Initialize(args) => {
                writer.write_all(&[INITIALIZE_IX_DISCM])?;
                args.serialize(&mut writer)
            }
            Self::Initialize2(args) => {
                writer.write_all(&[INITIALIZE2_IX_DISCM])?;
                args.serialize(&mut writer)
            }
            Self::MonitorStep(args) => {
                writer.write_all(&[MONITOR_STEP_IX_DISCM])?;
                args.serialize(&mut writer)
            }
            Self::Deposit(args) => {
                writer.write_all(&[DEPOSIT_IX_DISCM])?;
                args.serialize(&mut writer)
            }
            Self::Withdraw(args) => {
                writer.write_all(&[WITHDRAW_IX_DISCM])?;
                args.serialize(&mut writer)
            }
            Self::MigrateToOpenBook => writer.write_all(&[MIGRATE_TO_OPEN_BOOK_IX_DISCM]),
            Self::SetParams(args) => {
                writer.write_all(&[SET_PARAMS_IX_DISCM])?;
                args.serialize(&mut writer)
            }
            Self::WithdrawPnl => writer.write_all(&[WITHDRAW_PNL_IX_DISCM]),
            Self::WithdrawSrm(args) => {
                writer.write_all(&[WITHDRAW_SRM_IX_DISCM])?;
                args.serialize(&mut writer)
            }
            Self::SwapBaseIn(args) => {
                writer.write_all(&[SWAP_BASE_IN_IX_DISCM])?;
                args.serialize(&mut writer)
            }
            Self::PreInitialize(args) => {
                writer.write_all(&[PRE_INITIALIZE_IX_DISCM])?;
                args.serialize(&mut writer)
            }
            Self::SwapBaseOut(args) => {
                writer.write_all(&[SWAP_BASE_OUT_IX_DISCM])?;
                args.serialize(&mut writer)
            }
            Self::SimulateInfo(args) => {
                writer.write_all(&[SIMULATE_INFO_IX_DISCM])?;
                args.serialize(&mut writer)
            }
            Self::AdminCancelOrders(args) => {
                writer.write_all(&[ADMIN_CANCEL_ORDERS_IX_DISCM])?;
                args.serialize(&mut writer)
            }
            Self::CreateConfigAccount => {
                writer.write_all(&[CREATE_CONFIG_ACCOUNT_IX_DISCM])
            }
            Self::UpdateConfigAccount(args) => {
                writer.write_all(&[UPDATE_CONFIG_ACCOUNT_IX_DISCM])?;
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
pub const INITIALIZE_IX_ACCOUNTS_LEN: usize = 18;
#[derive(Copy, Clone, Debug)]
pub struct InitializeAccounts<'me, 'info> {
    pub token_program: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub rent: &'me AccountInfo<'info>,
    pub amm: &'me AccountInfo<'info>,
    pub amm_authority: &'me AccountInfo<'info>,
    pub amm_open_orders: &'me AccountInfo<'info>,
    pub lp_mint_address: &'me AccountInfo<'info>,
    pub coin_mint_address: &'me AccountInfo<'info>,
    pub pc_mint_address: &'me AccountInfo<'info>,
    pub pool_coin_token_account: &'me AccountInfo<'info>,
    pub pool_pc_token_account: &'me AccountInfo<'info>,
    pub pool_withdraw_queue: &'me AccountInfo<'info>,
    pub pool_target_orders_account: &'me AccountInfo<'info>,
    pub user_lp_token_account: &'me AccountInfo<'info>,
    pub pool_temp_lp_token_account: &'me AccountInfo<'info>,
    pub serum_program: &'me AccountInfo<'info>,
    pub serum_market: &'me AccountInfo<'info>,
    pub user_wallet: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug)]
pub struct InitializeKeys {
    pub token_program: Pubkey,
    pub system_program: Pubkey,
    pub rent: Pubkey,
    pub amm: Pubkey,
    pub amm_authority: Pubkey,
    pub amm_open_orders: Pubkey,
    pub lp_mint_address: Pubkey,
    pub coin_mint_address: Pubkey,
    pub pc_mint_address: Pubkey,
    pub pool_coin_token_account: Pubkey,
    pub pool_pc_token_account: Pubkey,
    pub pool_withdraw_queue: Pubkey,
    pub pool_target_orders_account: Pubkey,
    pub user_lp_token_account: Pubkey,
    pub pool_temp_lp_token_account: Pubkey,
    pub serum_program: Pubkey,
    pub serum_market: Pubkey,
    pub user_wallet: Pubkey,
}
impl From<InitializeAccounts<'_, '_>> for InitializeKeys {
    fn from(accounts: InitializeAccounts) -> Self {
        Self {
            token_program: *accounts.token_program.key,
            system_program: *accounts.system_program.key,
            rent: *accounts.rent.key,
            amm: *accounts.amm.key,
            amm_authority: *accounts.amm_authority.key,
            amm_open_orders: *accounts.amm_open_orders.key,
            lp_mint_address: *accounts.lp_mint_address.key,
            coin_mint_address: *accounts.coin_mint_address.key,
            pc_mint_address: *accounts.pc_mint_address.key,
            pool_coin_token_account: *accounts.pool_coin_token_account.key,
            pool_pc_token_account: *accounts.pool_pc_token_account.key,
            pool_withdraw_queue: *accounts.pool_withdraw_queue.key,
            pool_target_orders_account: *accounts.pool_target_orders_account.key,
            user_lp_token_account: *accounts.user_lp_token_account.key,
            pool_temp_lp_token_account: *accounts.pool_temp_lp_token_account.key,
            serum_program: *accounts.serum_program.key,
            serum_market: *accounts.serum_market.key,
            user_wallet: *accounts.user_wallet.key,
        }
    }
}
impl From<InitializeKeys> for [AccountMeta; INITIALIZE_IX_ACCOUNTS_LEN] {
    fn from(keys: InitializeKeys) -> Self {
        [
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
                pubkey: keys.amm,
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
                pubkey: keys.lp_mint_address,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.coin_mint_address,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pc_mint_address,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pool_coin_token_account,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pool_pc_token_account,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pool_withdraw_queue,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.pool_target_orders_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_lp_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.pool_temp_lp_token_account,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.serum_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.serum_market,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.user_wallet,
                is_signer: true,
                is_writable: true,
            },
        ]
    }
}
impl From<[Pubkey; INITIALIZE_IX_ACCOUNTS_LEN]> for InitializeKeys {
    fn from(pubkeys: [Pubkey; INITIALIZE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            token_program: pubkeys[0],
            system_program: pubkeys[1],
            rent: pubkeys[2],
            amm: pubkeys[3],
            amm_authority: pubkeys[4],
            amm_open_orders: pubkeys[5],
            lp_mint_address: pubkeys[6],
            coin_mint_address: pubkeys[7],
            pc_mint_address: pubkeys[8],
            pool_coin_token_account: pubkeys[9],
            pool_pc_token_account: pubkeys[10],
            pool_withdraw_queue: pubkeys[11],
            pool_target_orders_account: pubkeys[12],
            user_lp_token_account: pubkeys[13],
            pool_temp_lp_token_account: pubkeys[14],
            serum_program: pubkeys[15],
            serum_market: pubkeys[16],
            user_wallet: pubkeys[17],
        }
    }
}
impl<'info> From<InitializeAccounts<'_, 'info>>
for [AccountInfo<'info>; INITIALIZE_IX_ACCOUNTS_LEN] {
    fn from(accounts: InitializeAccounts<'_, 'info>) -> Self {
        [
            accounts.token_program.clone(),
            accounts.system_program.clone(),
            accounts.rent.clone(),
            accounts.amm.clone(),
            accounts.amm_authority.clone(),
            accounts.amm_open_orders.clone(),
            accounts.lp_mint_address.clone(),
            accounts.coin_mint_address.clone(),
            accounts.pc_mint_address.clone(),
            accounts.pool_coin_token_account.clone(),
            accounts.pool_pc_token_account.clone(),
            accounts.pool_withdraw_queue.clone(),
            accounts.pool_target_orders_account.clone(),
            accounts.user_lp_token_account.clone(),
            accounts.pool_temp_lp_token_account.clone(),
            accounts.serum_program.clone(),
            accounts.serum_market.clone(),
            accounts.user_wallet.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; INITIALIZE_IX_ACCOUNTS_LEN]>
for InitializeAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; INITIALIZE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            token_program: &arr[0],
            system_program: &arr[1],
            rent: &arr[2],
            amm: &arr[3],
            amm_authority: &arr[4],
            amm_open_orders: &arr[5],
            lp_mint_address: &arr[6],
            coin_mint_address: &arr[7],
            pc_mint_address: &arr[8],
            pool_coin_token_account: &arr[9],
            pool_pc_token_account: &arr[10],
            pool_withdraw_queue: &arr[11],
            pool_target_orders_account: &arr[12],
            user_lp_token_account: &arr[13],
            pool_temp_lp_token_account: &arr[14],
            serum_program: &arr[15],
            serum_market: &arr[16],
            user_wallet: &arr[17],
        }
    }
}
pub const INITIALIZE_IX_DISCM: u8 = 0u8;
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InitializeIxArgs {
    pub nonce: u8,
    pub open_time: u64,
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
        let mut maybe_discm_buf = [0u8; 1];
        reader.read_exact(&mut maybe_discm_buf)?;
        let maybe_discm = maybe_discm_buf[0];
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
        Ok(Self(InitializeIxArgs::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&[INITIALIZE_IX_DISCM])?;
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
) -> std::io::Result<Instruction> {
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
) -> std::io::Result<Instruction> {
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
        (accounts.token_program.key, &keys.token_program),
        (accounts.system_program.key, &keys.system_program),
        (accounts.rent.key, &keys.rent),
        (accounts.amm.key, &keys.amm),
        (accounts.amm_authority.key, &keys.amm_authority),
        (accounts.amm_open_orders.key, &keys.amm_open_orders),
        (accounts.lp_mint_address.key, &keys.lp_mint_address),
        (accounts.coin_mint_address.key, &keys.coin_mint_address),
        (accounts.pc_mint_address.key, &keys.pc_mint_address),
        (accounts.pool_coin_token_account.key, &keys.pool_coin_token_account),
        (accounts.pool_pc_token_account.key, &keys.pool_pc_token_account),
        (accounts.pool_withdraw_queue.key, &keys.pool_withdraw_queue),
        (accounts.pool_target_orders_account.key, &keys.pool_target_orders_account),
        (accounts.user_lp_token_account.key, &keys.user_lp_token_account),
        (accounts.pool_temp_lp_token_account.key, &keys.pool_temp_lp_token_account),
        (accounts.serum_program.key, &keys.serum_program),
        (accounts.serum_market.key, &keys.serum_market),
        (accounts.user_wallet.key, &keys.user_wallet),
    ] {
        if actual != expected {
            return Err((*actual, *expected));
        }
    }
    Ok(())
}
pub fn initialize_verify_writable_privileges<'me, 'info>(
    accounts: InitializeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.amm,
        accounts.amm_open_orders,
        accounts.lp_mint_address,
        accounts.pool_withdraw_queue,
        accounts.pool_target_orders_account,
        accounts.user_lp_token_account,
        accounts.user_wallet,
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
    for should_be_signer in [accounts.user_wallet] {
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
pub const INITIALIZE2_IX_ACCOUNTS_LEN: usize = 21;
#[derive(Copy, Clone, Debug)]
pub struct Initialize2Accounts<'me, 'info> {
    pub token_program: &'me AccountInfo<'info>,
    pub spl_associated_token_account: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub rent: &'me AccountInfo<'info>,
    pub amm: &'me AccountInfo<'info>,
    pub amm_authority: &'me AccountInfo<'info>,
    pub amm_open_orders: &'me AccountInfo<'info>,
    pub lp_mint: &'me AccountInfo<'info>,
    pub coin_mint: &'me AccountInfo<'info>,
    pub pc_mint: &'me AccountInfo<'info>,
    pub pool_coin_token_account: &'me AccountInfo<'info>,
    pub pool_pc_token_account: &'me AccountInfo<'info>,
    pub pool_withdraw_queue: &'me AccountInfo<'info>,
    pub amm_target_orders: &'me AccountInfo<'info>,
    pub pool_temp_lp: &'me AccountInfo<'info>,
    pub serum_program: &'me AccountInfo<'info>,
    pub serum_market: &'me AccountInfo<'info>,
    pub user_wallet: &'me AccountInfo<'info>,
    pub user_token_coin: &'me AccountInfo<'info>,
    pub user_token_pc: &'me AccountInfo<'info>,
    pub user_lp_token_account: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug)]
pub struct Initialize2Keys {
    pub token_program: Pubkey,
    pub spl_associated_token_account: Pubkey,
    pub system_program: Pubkey,
    pub rent: Pubkey,
    pub amm: Pubkey,
    pub amm_authority: Pubkey,
    pub amm_open_orders: Pubkey,
    pub lp_mint: Pubkey,
    pub coin_mint: Pubkey,
    pub pc_mint: Pubkey,
    pub pool_coin_token_account: Pubkey,
    pub pool_pc_token_account: Pubkey,
    pub pool_withdraw_queue: Pubkey,
    pub amm_target_orders: Pubkey,
    pub pool_temp_lp: Pubkey,
    pub serum_program: Pubkey,
    pub serum_market: Pubkey,
    pub user_wallet: Pubkey,
    pub user_token_coin: Pubkey,
    pub user_token_pc: Pubkey,
    pub user_lp_token_account: Pubkey,
}
impl From<Initialize2Accounts<'_, '_>> for Initialize2Keys {
    fn from(accounts: Initialize2Accounts) -> Self {
        Self {
            token_program: *accounts.token_program.key,
            spl_associated_token_account: *accounts.spl_associated_token_account.key,
            system_program: *accounts.system_program.key,
            rent: *accounts.rent.key,
            amm: *accounts.amm.key,
            amm_authority: *accounts.amm_authority.key,
            amm_open_orders: *accounts.amm_open_orders.key,
            lp_mint: *accounts.lp_mint.key,
            coin_mint: *accounts.coin_mint.key,
            pc_mint: *accounts.pc_mint.key,
            pool_coin_token_account: *accounts.pool_coin_token_account.key,
            pool_pc_token_account: *accounts.pool_pc_token_account.key,
            pool_withdraw_queue: *accounts.pool_withdraw_queue.key,
            amm_target_orders: *accounts.amm_target_orders.key,
            pool_temp_lp: *accounts.pool_temp_lp.key,
            serum_program: *accounts.serum_program.key,
            serum_market: *accounts.serum_market.key,
            user_wallet: *accounts.user_wallet.key,
            user_token_coin: *accounts.user_token_coin.key,
            user_token_pc: *accounts.user_token_pc.key,
            user_lp_token_account: *accounts.user_lp_token_account.key,
        }
    }
}
impl From<Initialize2Keys> for [AccountMeta; INITIALIZE2_IX_ACCOUNTS_LEN] {
    fn from(keys: Initialize2Keys) -> Self {
        [
            AccountMeta {
                pubkey: keys.token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.spl_associated_token_account,
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
                pubkey: keys.amm,
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
                pubkey: keys.lp_mint,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.coin_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pc_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pool_coin_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.pool_pc_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.pool_withdraw_queue,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.amm_target_orders,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.pool_temp_lp,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.serum_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.serum_market,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.user_wallet,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_token_coin,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_token_pc,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_lp_token_account,
                is_signer: false,
                is_writable: true,
            },
        ]
    }
}
impl From<[Pubkey; INITIALIZE2_IX_ACCOUNTS_LEN]> for Initialize2Keys {
    fn from(pubkeys: [Pubkey; INITIALIZE2_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            token_program: pubkeys[0],
            spl_associated_token_account: pubkeys[1],
            system_program: pubkeys[2],
            rent: pubkeys[3],
            amm: pubkeys[4],
            amm_authority: pubkeys[5],
            amm_open_orders: pubkeys[6],
            lp_mint: pubkeys[7],
            coin_mint: pubkeys[8],
            pc_mint: pubkeys[9],
            pool_coin_token_account: pubkeys[10],
            pool_pc_token_account: pubkeys[11],
            pool_withdraw_queue: pubkeys[12],
            amm_target_orders: pubkeys[13],
            pool_temp_lp: pubkeys[14],
            serum_program: pubkeys[15],
            serum_market: pubkeys[16],
            user_wallet: pubkeys[17],
            user_token_coin: pubkeys[18],
            user_token_pc: pubkeys[19],
            user_lp_token_account: pubkeys[20],
        }
    }
}
impl<'info> From<Initialize2Accounts<'_, 'info>>
for [AccountInfo<'info>; INITIALIZE2_IX_ACCOUNTS_LEN] {
    fn from(accounts: Initialize2Accounts<'_, 'info>) -> Self {
        [
            accounts.token_program.clone(),
            accounts.spl_associated_token_account.clone(),
            accounts.system_program.clone(),
            accounts.rent.clone(),
            accounts.amm.clone(),
            accounts.amm_authority.clone(),
            accounts.amm_open_orders.clone(),
            accounts.lp_mint.clone(),
            accounts.coin_mint.clone(),
            accounts.pc_mint.clone(),
            accounts.pool_coin_token_account.clone(),
            accounts.pool_pc_token_account.clone(),
            accounts.pool_withdraw_queue.clone(),
            accounts.amm_target_orders.clone(),
            accounts.pool_temp_lp.clone(),
            accounts.serum_program.clone(),
            accounts.serum_market.clone(),
            accounts.user_wallet.clone(),
            accounts.user_token_coin.clone(),
            accounts.user_token_pc.clone(),
            accounts.user_lp_token_account.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; INITIALIZE2_IX_ACCOUNTS_LEN]>
for Initialize2Accounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; INITIALIZE2_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            token_program: &arr[0],
            spl_associated_token_account: &arr[1],
            system_program: &arr[2],
            rent: &arr[3],
            amm: &arr[4],
            amm_authority: &arr[5],
            amm_open_orders: &arr[6],
            lp_mint: &arr[7],
            coin_mint: &arr[8],
            pc_mint: &arr[9],
            pool_coin_token_account: &arr[10],
            pool_pc_token_account: &arr[11],
            pool_withdraw_queue: &arr[12],
            amm_target_orders: &arr[13],
            pool_temp_lp: &arr[14],
            serum_program: &arr[15],
            serum_market: &arr[16],
            user_wallet: &arr[17],
            user_token_coin: &arr[18],
            user_token_pc: &arr[19],
            user_lp_token_account: &arr[20],
        }
    }
}
pub const INITIALIZE2_IX_DISCM: u8 = 1u8;
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Initialize2IxArgs {
    pub nonce: u8,
    pub open_time: u64,
    pub init_pc_amount: u64,
    pub init_coin_amount: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct Initialize2IxData(pub Initialize2IxArgs);
impl From<Initialize2IxArgs> for Initialize2IxData {
    fn from(args: Initialize2IxArgs) -> Self {
        Self(args)
    }
}
impl Initialize2IxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm_buf = [0u8; 1];
        reader.read_exact(&mut maybe_discm_buf)?;
        let maybe_discm = maybe_discm_buf[0];
        if maybe_discm != INITIALIZE2_IX_DISCM {
            return Err(
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!(
                        "discm does not match. Expected: {:?}. Received: {:?}",
                        INITIALIZE2_IX_DISCM, maybe_discm
                    ),
                ),
            );
        }
        Ok(Self(Initialize2IxArgs::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&[INITIALIZE2_IX_DISCM])?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn initialize2_ix_with_program_id(
    program_id: Pubkey,
    keys: Initialize2Keys,
    args: Initialize2IxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; INITIALIZE2_IX_ACCOUNTS_LEN] = keys.into();
    let data: Initialize2IxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn initialize2_ix(
    keys: Initialize2Keys,
    args: Initialize2IxArgs,
) -> std::io::Result<Instruction> {
    initialize2_ix_with_program_id(crate::ID, keys, args)
}
pub fn initialize2_invoke_with_program_id(
    program_id: Pubkey,
    accounts: Initialize2Accounts<'_, '_>,
    args: Initialize2IxArgs,
) -> ProgramResult {
    let keys: Initialize2Keys = accounts.into();
    let ix = initialize2_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn initialize2_invoke(
    accounts: Initialize2Accounts<'_, '_>,
    args: Initialize2IxArgs,
) -> ProgramResult {
    initialize2_invoke_with_program_id(crate::ID, accounts, args)
}
pub fn initialize2_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: Initialize2Accounts<'_, '_>,
    args: Initialize2IxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: Initialize2Keys = accounts.into();
    let ix = initialize2_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn initialize2_invoke_signed(
    accounts: Initialize2Accounts<'_, '_>,
    args: Initialize2IxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    initialize2_invoke_signed_with_program_id(crate::ID, accounts, args, seeds)
}
pub fn initialize2_verify_account_keys(
    accounts: Initialize2Accounts<'_, '_>,
    keys: Initialize2Keys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (accounts.token_program.key, &keys.token_program),
        (accounts.spl_associated_token_account.key, &keys.spl_associated_token_account),
        (accounts.system_program.key, &keys.system_program),
        (accounts.rent.key, &keys.rent),
        (accounts.amm.key, &keys.amm),
        (accounts.amm_authority.key, &keys.amm_authority),
        (accounts.amm_open_orders.key, &keys.amm_open_orders),
        (accounts.lp_mint.key, &keys.lp_mint),
        (accounts.coin_mint.key, &keys.coin_mint),
        (accounts.pc_mint.key, &keys.pc_mint),
        (accounts.pool_coin_token_account.key, &keys.pool_coin_token_account),
        (accounts.pool_pc_token_account.key, &keys.pool_pc_token_account),
        (accounts.pool_withdraw_queue.key, &keys.pool_withdraw_queue),
        (accounts.amm_target_orders.key, &keys.amm_target_orders),
        (accounts.pool_temp_lp.key, &keys.pool_temp_lp),
        (accounts.serum_program.key, &keys.serum_program),
        (accounts.serum_market.key, &keys.serum_market),
        (accounts.user_wallet.key, &keys.user_wallet),
        (accounts.user_token_coin.key, &keys.user_token_coin),
        (accounts.user_token_pc.key, &keys.user_token_pc),
        (accounts.user_lp_token_account.key, &keys.user_lp_token_account),
    ] {
        if actual != expected {
            return Err((*actual, *expected));
        }
    }
    Ok(())
}
pub fn initialize2_verify_writable_privileges<'me, 'info>(
    accounts: Initialize2Accounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.amm,
        accounts.amm_open_orders,
        accounts.lp_mint,
        accounts.pool_coin_token_account,
        accounts.pool_pc_token_account,
        accounts.pool_withdraw_queue,
        accounts.amm_target_orders,
        accounts.pool_temp_lp,
        accounts.user_wallet,
        accounts.user_token_coin,
        accounts.user_token_pc,
        accounts.user_lp_token_account,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn initialize2_verify_signer_privileges<'me, 'info>(
    accounts: Initialize2Accounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.user_wallet] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn initialize2_verify_account_privileges<'me, 'info>(
    accounts: Initialize2Accounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    initialize2_verify_writable_privileges(accounts)?;
    initialize2_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const MONITOR_STEP_IX_ACCOUNTS_LEN: usize = 21;
#[derive(Copy, Clone, Debug)]
pub struct MonitorStepAccounts<'me, 'info> {
    pub token_program: &'me AccountInfo<'info>,
    pub rent: &'me AccountInfo<'info>,
    pub clock: &'me AccountInfo<'info>,
    pub amm: &'me AccountInfo<'info>,
    pub amm_authority: &'me AccountInfo<'info>,
    pub amm_open_orders: &'me AccountInfo<'info>,
    pub amm_target_orders: &'me AccountInfo<'info>,
    pub pool_coin_token_account: &'me AccountInfo<'info>,
    pub pool_pc_token_account: &'me AccountInfo<'info>,
    pub pool_withdraw_queue: &'me AccountInfo<'info>,
    pub serum_program: &'me AccountInfo<'info>,
    pub serum_market: &'me AccountInfo<'info>,
    pub serum_coin_vault_account: &'me AccountInfo<'info>,
    pub serum_pc_vault_account: &'me AccountInfo<'info>,
    pub serum_vault_signer: &'me AccountInfo<'info>,
    pub serum_req_q: &'me AccountInfo<'info>,
    pub serum_event_q: &'me AccountInfo<'info>,
    pub serum_bids: &'me AccountInfo<'info>,
    pub serum_asks: &'me AccountInfo<'info>,
    pub serum_fee_discount: &'me AccountInfo<'info>,
    pub referrer_pc_account: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug)]
pub struct MonitorStepKeys {
    pub token_program: Pubkey,
    pub rent: Pubkey,
    pub clock: Pubkey,
    pub amm: Pubkey,
    pub amm_authority: Pubkey,
    pub amm_open_orders: Pubkey,
    pub amm_target_orders: Pubkey,
    pub pool_coin_token_account: Pubkey,
    pub pool_pc_token_account: Pubkey,
    pub pool_withdraw_queue: Pubkey,
    pub serum_program: Pubkey,
    pub serum_market: Pubkey,
    pub serum_coin_vault_account: Pubkey,
    pub serum_pc_vault_account: Pubkey,
    pub serum_vault_signer: Pubkey,
    pub serum_req_q: Pubkey,
    pub serum_event_q: Pubkey,
    pub serum_bids: Pubkey,
    pub serum_asks: Pubkey,
    pub serum_fee_discount: Pubkey,
    pub referrer_pc_account: Pubkey,
}
impl From<MonitorStepAccounts<'_, '_>> for MonitorStepKeys {
    fn from(accounts: MonitorStepAccounts) -> Self {
        Self {
            token_program: *accounts.token_program.key,
            rent: *accounts.rent.key,
            clock: *accounts.clock.key,
            amm: *accounts.amm.key,
            amm_authority: *accounts.amm_authority.key,
            amm_open_orders: *accounts.amm_open_orders.key,
            amm_target_orders: *accounts.amm_target_orders.key,
            pool_coin_token_account: *accounts.pool_coin_token_account.key,
            pool_pc_token_account: *accounts.pool_pc_token_account.key,
            pool_withdraw_queue: *accounts.pool_withdraw_queue.key,
            serum_program: *accounts.serum_program.key,
            serum_market: *accounts.serum_market.key,
            serum_coin_vault_account: *accounts.serum_coin_vault_account.key,
            serum_pc_vault_account: *accounts.serum_pc_vault_account.key,
            serum_vault_signer: *accounts.serum_vault_signer.key,
            serum_req_q: *accounts.serum_req_q.key,
            serum_event_q: *accounts.serum_event_q.key,
            serum_bids: *accounts.serum_bids.key,
            serum_asks: *accounts.serum_asks.key,
            serum_fee_discount: *accounts.serum_fee_discount.key,
            referrer_pc_account: *accounts.referrer_pc_account.key,
        }
    }
}
impl From<MonitorStepKeys> for [AccountMeta; MONITOR_STEP_IX_ACCOUNTS_LEN] {
    fn from(keys: MonitorStepKeys) -> Self {
        [
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
                pubkey: keys.clock,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.amm,
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
                pubkey: keys.amm_target_orders,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.pool_coin_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.pool_pc_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.pool_withdraw_queue,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.serum_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.serum_market,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.serum_coin_vault_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.serum_pc_vault_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.serum_vault_signer,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.serum_req_q,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.serum_event_q,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.serum_bids,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.serum_asks,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.serum_fee_discount,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.referrer_pc_account,
                is_signer: false,
                is_writable: true,
            },
        ]
    }
}
impl From<[Pubkey; MONITOR_STEP_IX_ACCOUNTS_LEN]> for MonitorStepKeys {
    fn from(pubkeys: [Pubkey; MONITOR_STEP_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            token_program: pubkeys[0],
            rent: pubkeys[1],
            clock: pubkeys[2],
            amm: pubkeys[3],
            amm_authority: pubkeys[4],
            amm_open_orders: pubkeys[5],
            amm_target_orders: pubkeys[6],
            pool_coin_token_account: pubkeys[7],
            pool_pc_token_account: pubkeys[8],
            pool_withdraw_queue: pubkeys[9],
            serum_program: pubkeys[10],
            serum_market: pubkeys[11],
            serum_coin_vault_account: pubkeys[12],
            serum_pc_vault_account: pubkeys[13],
            serum_vault_signer: pubkeys[14],
            serum_req_q: pubkeys[15],
            serum_event_q: pubkeys[16],
            serum_bids: pubkeys[17],
            serum_asks: pubkeys[18],
            serum_fee_discount: pubkeys[19],
            referrer_pc_account: pubkeys[20],
        }
    }
}
impl<'info> From<MonitorStepAccounts<'_, 'info>>
for [AccountInfo<'info>; MONITOR_STEP_IX_ACCOUNTS_LEN] {
    fn from(accounts: MonitorStepAccounts<'_, 'info>) -> Self {
        [
            accounts.token_program.clone(),
            accounts.rent.clone(),
            accounts.clock.clone(),
            accounts.amm.clone(),
            accounts.amm_authority.clone(),
            accounts.amm_open_orders.clone(),
            accounts.amm_target_orders.clone(),
            accounts.pool_coin_token_account.clone(),
            accounts.pool_pc_token_account.clone(),
            accounts.pool_withdraw_queue.clone(),
            accounts.serum_program.clone(),
            accounts.serum_market.clone(),
            accounts.serum_coin_vault_account.clone(),
            accounts.serum_pc_vault_account.clone(),
            accounts.serum_vault_signer.clone(),
            accounts.serum_req_q.clone(),
            accounts.serum_event_q.clone(),
            accounts.serum_bids.clone(),
            accounts.serum_asks.clone(),
            accounts.serum_fee_discount.clone(),
            accounts.referrer_pc_account.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; MONITOR_STEP_IX_ACCOUNTS_LEN]>
for MonitorStepAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; MONITOR_STEP_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            token_program: &arr[0],
            rent: &arr[1],
            clock: &arr[2],
            amm: &arr[3],
            amm_authority: &arr[4],
            amm_open_orders: &arr[5],
            amm_target_orders: &arr[6],
            pool_coin_token_account: &arr[7],
            pool_pc_token_account: &arr[8],
            pool_withdraw_queue: &arr[9],
            serum_program: &arr[10],
            serum_market: &arr[11],
            serum_coin_vault_account: &arr[12],
            serum_pc_vault_account: &arr[13],
            serum_vault_signer: &arr[14],
            serum_req_q: &arr[15],
            serum_event_q: &arr[16],
            serum_bids: &arr[17],
            serum_asks: &arr[18],
            serum_fee_discount: &arr[19],
            referrer_pc_account: &arr[20],
        }
    }
}
pub const MONITOR_STEP_IX_DISCM: u8 = 2u8;
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct MonitorStepIxArgs {
    pub plan_order_limit: u16,
    pub place_order_limit: u16,
    pub cancel_order_limit: u16,
}
#[derive(Clone, Debug, PartialEq)]
pub struct MonitorStepIxData(pub MonitorStepIxArgs);
impl From<MonitorStepIxArgs> for MonitorStepIxData {
    fn from(args: MonitorStepIxArgs) -> Self {
        Self(args)
    }
}
impl MonitorStepIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm_buf = [0u8; 1];
        reader.read_exact(&mut maybe_discm_buf)?;
        let maybe_discm = maybe_discm_buf[0];
        if maybe_discm != MONITOR_STEP_IX_DISCM {
            return Err(
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!(
                        "discm does not match. Expected: {:?}. Received: {:?}",
                        MONITOR_STEP_IX_DISCM, maybe_discm
                    ),
                ),
            );
        }
        Ok(Self(MonitorStepIxArgs::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&[MONITOR_STEP_IX_DISCM])?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn monitor_step_ix_with_program_id(
    program_id: Pubkey,
    keys: MonitorStepKeys,
    args: MonitorStepIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; MONITOR_STEP_IX_ACCOUNTS_LEN] = keys.into();
    let data: MonitorStepIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn monitor_step_ix(
    keys: MonitorStepKeys,
    args: MonitorStepIxArgs,
) -> std::io::Result<Instruction> {
    monitor_step_ix_with_program_id(crate::ID, keys, args)
}
pub fn monitor_step_invoke_with_program_id(
    program_id: Pubkey,
    accounts: MonitorStepAccounts<'_, '_>,
    args: MonitorStepIxArgs,
) -> ProgramResult {
    let keys: MonitorStepKeys = accounts.into();
    let ix = monitor_step_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn monitor_step_invoke(
    accounts: MonitorStepAccounts<'_, '_>,
    args: MonitorStepIxArgs,
) -> ProgramResult {
    monitor_step_invoke_with_program_id(crate::ID, accounts, args)
}
pub fn monitor_step_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: MonitorStepAccounts<'_, '_>,
    args: MonitorStepIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: MonitorStepKeys = accounts.into();
    let ix = monitor_step_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn monitor_step_invoke_signed(
    accounts: MonitorStepAccounts<'_, '_>,
    args: MonitorStepIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    monitor_step_invoke_signed_with_program_id(crate::ID, accounts, args, seeds)
}
pub fn monitor_step_verify_account_keys(
    accounts: MonitorStepAccounts<'_, '_>,
    keys: MonitorStepKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (accounts.token_program.key, &keys.token_program),
        (accounts.rent.key, &keys.rent),
        (accounts.clock.key, &keys.clock),
        (accounts.amm.key, &keys.amm),
        (accounts.amm_authority.key, &keys.amm_authority),
        (accounts.amm_open_orders.key, &keys.amm_open_orders),
        (accounts.amm_target_orders.key, &keys.amm_target_orders),
        (accounts.pool_coin_token_account.key, &keys.pool_coin_token_account),
        (accounts.pool_pc_token_account.key, &keys.pool_pc_token_account),
        (accounts.pool_withdraw_queue.key, &keys.pool_withdraw_queue),
        (accounts.serum_program.key, &keys.serum_program),
        (accounts.serum_market.key, &keys.serum_market),
        (accounts.serum_coin_vault_account.key, &keys.serum_coin_vault_account),
        (accounts.serum_pc_vault_account.key, &keys.serum_pc_vault_account),
        (accounts.serum_vault_signer.key, &keys.serum_vault_signer),
        (accounts.serum_req_q.key, &keys.serum_req_q),
        (accounts.serum_event_q.key, &keys.serum_event_q),
        (accounts.serum_bids.key, &keys.serum_bids),
        (accounts.serum_asks.key, &keys.serum_asks),
        (accounts.serum_fee_discount.key, &keys.serum_fee_discount),
        (accounts.referrer_pc_account.key, &keys.referrer_pc_account),
    ] {
        if actual != expected {
            return Err((*actual, *expected));
        }
    }
    Ok(())
}
pub fn monitor_step_verify_writable_privileges<'me, 'info>(
    accounts: MonitorStepAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.amm,
        accounts.amm_open_orders,
        accounts.amm_target_orders,
        accounts.pool_coin_token_account,
        accounts.pool_pc_token_account,
        accounts.pool_withdraw_queue,
        accounts.serum_market,
        accounts.serum_coin_vault_account,
        accounts.serum_pc_vault_account,
        accounts.serum_req_q,
        accounts.serum_event_q,
        accounts.serum_bids,
        accounts.serum_asks,
        accounts.serum_fee_discount,
        accounts.referrer_pc_account,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn monitor_step_verify_account_privileges<'me, 'info>(
    accounts: MonitorStepAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    monitor_step_verify_writable_privileges(accounts)?;
    Ok(())
}
pub const DEPOSIT_IX_ACCOUNTS_LEN: usize = 14;
#[derive(Copy, Clone, Debug)]
pub struct DepositAccounts<'me, 'info> {
    pub token_program: &'me AccountInfo<'info>,
    pub amm: &'me AccountInfo<'info>,
    pub amm_authority: &'me AccountInfo<'info>,
    pub amm_open_orders: &'me AccountInfo<'info>,
    pub amm_target_orders: &'me AccountInfo<'info>,
    pub lp_mint_address: &'me AccountInfo<'info>,
    pub pool_coin_token_account: &'me AccountInfo<'info>,
    pub pool_pc_token_account: &'me AccountInfo<'info>,
    pub serum_market: &'me AccountInfo<'info>,
    pub user_coin_token_account: &'me AccountInfo<'info>,
    pub user_pc_token_account: &'me AccountInfo<'info>,
    pub user_lp_token_account: &'me AccountInfo<'info>,
    pub user_owner: &'me AccountInfo<'info>,
    pub serum_event_queue: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug)]
pub struct DepositKeys {
    pub token_program: Pubkey,
    pub amm: Pubkey,
    pub amm_authority: Pubkey,
    pub amm_open_orders: Pubkey,
    pub amm_target_orders: Pubkey,
    pub lp_mint_address: Pubkey,
    pub pool_coin_token_account: Pubkey,
    pub pool_pc_token_account: Pubkey,
    pub serum_market: Pubkey,
    pub user_coin_token_account: Pubkey,
    pub user_pc_token_account: Pubkey,
    pub user_lp_token_account: Pubkey,
    pub user_owner: Pubkey,
    pub serum_event_queue: Pubkey,
}
impl From<DepositAccounts<'_, '_>> for DepositKeys {
    fn from(accounts: DepositAccounts) -> Self {
        Self {
            token_program: *accounts.token_program.key,
            amm: *accounts.amm.key,
            amm_authority: *accounts.amm_authority.key,
            amm_open_orders: *accounts.amm_open_orders.key,
            amm_target_orders: *accounts.amm_target_orders.key,
            lp_mint_address: *accounts.lp_mint_address.key,
            pool_coin_token_account: *accounts.pool_coin_token_account.key,
            pool_pc_token_account: *accounts.pool_pc_token_account.key,
            serum_market: *accounts.serum_market.key,
            user_coin_token_account: *accounts.user_coin_token_account.key,
            user_pc_token_account: *accounts.user_pc_token_account.key,
            user_lp_token_account: *accounts.user_lp_token_account.key,
            user_owner: *accounts.user_owner.key,
            serum_event_queue: *accounts.serum_event_queue.key,
        }
    }
}
impl From<DepositKeys> for [AccountMeta; DEPOSIT_IX_ACCOUNTS_LEN] {
    fn from(keys: DepositKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.amm,
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
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.amm_target_orders,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.lp_mint_address,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.pool_coin_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.pool_pc_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.serum_market,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.user_coin_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_pc_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_lp_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_owner,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.serum_event_queue,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; DEPOSIT_IX_ACCOUNTS_LEN]> for DepositKeys {
    fn from(pubkeys: [Pubkey; DEPOSIT_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            token_program: pubkeys[0],
            amm: pubkeys[1],
            amm_authority: pubkeys[2],
            amm_open_orders: pubkeys[3],
            amm_target_orders: pubkeys[4],
            lp_mint_address: pubkeys[5],
            pool_coin_token_account: pubkeys[6],
            pool_pc_token_account: pubkeys[7],
            serum_market: pubkeys[8],
            user_coin_token_account: pubkeys[9],
            user_pc_token_account: pubkeys[10],
            user_lp_token_account: pubkeys[11],
            user_owner: pubkeys[12],
            serum_event_queue: pubkeys[13],
        }
    }
}
impl<'info> From<DepositAccounts<'_, 'info>>
for [AccountInfo<'info>; DEPOSIT_IX_ACCOUNTS_LEN] {
    fn from(accounts: DepositAccounts<'_, 'info>) -> Self {
        [
            accounts.token_program.clone(),
            accounts.amm.clone(),
            accounts.amm_authority.clone(),
            accounts.amm_open_orders.clone(),
            accounts.amm_target_orders.clone(),
            accounts.lp_mint_address.clone(),
            accounts.pool_coin_token_account.clone(),
            accounts.pool_pc_token_account.clone(),
            accounts.serum_market.clone(),
            accounts.user_coin_token_account.clone(),
            accounts.user_pc_token_account.clone(),
            accounts.user_lp_token_account.clone(),
            accounts.user_owner.clone(),
            accounts.serum_event_queue.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; DEPOSIT_IX_ACCOUNTS_LEN]>
for DepositAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; DEPOSIT_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            token_program: &arr[0],
            amm: &arr[1],
            amm_authority: &arr[2],
            amm_open_orders: &arr[3],
            amm_target_orders: &arr[4],
            lp_mint_address: &arr[5],
            pool_coin_token_account: &arr[6],
            pool_pc_token_account: &arr[7],
            serum_market: &arr[8],
            user_coin_token_account: &arr[9],
            user_pc_token_account: &arr[10],
            user_lp_token_account: &arr[11],
            user_owner: &arr[12],
            serum_event_queue: &arr[13],
        }
    }
}
pub const DEPOSIT_IX_DISCM: u8 = 3u8;
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct DepositIxArgs {
    pub max_coin_amount: u64,
    pub max_pc_amount: u64,
    pub base_side: u64,
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
        let mut maybe_discm_buf = [0u8; 1];
        reader.read_exact(&mut maybe_discm_buf)?;
        let maybe_discm = maybe_discm_buf[0];
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
        writer.write_all(&[DEPOSIT_IX_DISCM])?;
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
        (accounts.token_program.key, &keys.token_program),
        (accounts.amm.key, &keys.amm),
        (accounts.amm_authority.key, &keys.amm_authority),
        (accounts.amm_open_orders.key, &keys.amm_open_orders),
        (accounts.amm_target_orders.key, &keys.amm_target_orders),
        (accounts.lp_mint_address.key, &keys.lp_mint_address),
        (accounts.pool_coin_token_account.key, &keys.pool_coin_token_account),
        (accounts.pool_pc_token_account.key, &keys.pool_pc_token_account),
        (accounts.serum_market.key, &keys.serum_market),
        (accounts.user_coin_token_account.key, &keys.user_coin_token_account),
        (accounts.user_pc_token_account.key, &keys.user_pc_token_account),
        (accounts.user_lp_token_account.key, &keys.user_lp_token_account),
        (accounts.user_owner.key, &keys.user_owner),
        (accounts.serum_event_queue.key, &keys.serum_event_queue),
    ] {
        if actual != expected {
            return Err((*actual, *expected));
        }
    }
    Ok(())
}
pub fn deposit_verify_writable_privileges<'me, 'info>(
    accounts: DepositAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.amm,
        accounts.amm_target_orders,
        accounts.lp_mint_address,
        accounts.pool_coin_token_account,
        accounts.pool_pc_token_account,
        accounts.user_coin_token_account,
        accounts.user_pc_token_account,
        accounts.user_lp_token_account,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn deposit_verify_signer_privileges<'me, 'info>(
    accounts: DepositAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.user_owner] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn deposit_verify_account_privileges<'me, 'info>(
    accounts: DepositAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    deposit_verify_writable_privileges(accounts)?;
    deposit_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const WITHDRAW_IX_ACCOUNTS_LEN: usize = 22;
#[derive(Copy, Clone, Debug)]
pub struct WithdrawAccounts<'me, 'info> {
    pub token_program: &'me AccountInfo<'info>,
    pub amm: &'me AccountInfo<'info>,
    pub amm_authority: &'me AccountInfo<'info>,
    pub amm_open_orders: &'me AccountInfo<'info>,
    pub amm_target_orders: &'me AccountInfo<'info>,
    pub lp_mint_address: &'me AccountInfo<'info>,
    pub pool_coin_token_account: &'me AccountInfo<'info>,
    pub pool_pc_token_account: &'me AccountInfo<'info>,
    pub pool_withdraw_queue: &'me AccountInfo<'info>,
    pub pool_temp_lp_token_account: &'me AccountInfo<'info>,
    pub serum_program: &'me AccountInfo<'info>,
    pub serum_market: &'me AccountInfo<'info>,
    pub serum_coin_vault_account: &'me AccountInfo<'info>,
    pub serum_pc_vault_account: &'me AccountInfo<'info>,
    pub serum_vault_signer: &'me AccountInfo<'info>,
    pub user_lp_token_account: &'me AccountInfo<'info>,
    pub user_coin_token_account: &'me AccountInfo<'info>,
    pub user_pc_token_account: &'me AccountInfo<'info>,
    pub user_owner: &'me AccountInfo<'info>,
    pub serum_event_q: &'me AccountInfo<'info>,
    pub serum_bids: &'me AccountInfo<'info>,
    pub serum_asks: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug)]
pub struct WithdrawKeys {
    pub token_program: Pubkey,
    pub amm: Pubkey,
    pub amm_authority: Pubkey,
    pub amm_open_orders: Pubkey,
    pub amm_target_orders: Pubkey,
    pub lp_mint_address: Pubkey,
    pub pool_coin_token_account: Pubkey,
    pub pool_pc_token_account: Pubkey,
    pub pool_withdraw_queue: Pubkey,
    pub pool_temp_lp_token_account: Pubkey,
    pub serum_program: Pubkey,
    pub serum_market: Pubkey,
    pub serum_coin_vault_account: Pubkey,
    pub serum_pc_vault_account: Pubkey,
    pub serum_vault_signer: Pubkey,
    pub user_lp_token_account: Pubkey,
    pub user_coin_token_account: Pubkey,
    pub user_pc_token_account: Pubkey,
    pub user_owner: Pubkey,
    pub serum_event_q: Pubkey,
    pub serum_bids: Pubkey,
    pub serum_asks: Pubkey,
}
impl From<WithdrawAccounts<'_, '_>> for WithdrawKeys {
    fn from(accounts: WithdrawAccounts) -> Self {
        Self {
            token_program: *accounts.token_program.key,
            amm: *accounts.amm.key,
            amm_authority: *accounts.amm_authority.key,
            amm_open_orders: *accounts.amm_open_orders.key,
            amm_target_orders: *accounts.amm_target_orders.key,
            lp_mint_address: *accounts.lp_mint_address.key,
            pool_coin_token_account: *accounts.pool_coin_token_account.key,
            pool_pc_token_account: *accounts.pool_pc_token_account.key,
            pool_withdraw_queue: *accounts.pool_withdraw_queue.key,
            pool_temp_lp_token_account: *accounts.pool_temp_lp_token_account.key,
            serum_program: *accounts.serum_program.key,
            serum_market: *accounts.serum_market.key,
            serum_coin_vault_account: *accounts.serum_coin_vault_account.key,
            serum_pc_vault_account: *accounts.serum_pc_vault_account.key,
            serum_vault_signer: *accounts.serum_vault_signer.key,
            user_lp_token_account: *accounts.user_lp_token_account.key,
            user_coin_token_account: *accounts.user_coin_token_account.key,
            user_pc_token_account: *accounts.user_pc_token_account.key,
            user_owner: *accounts.user_owner.key,
            serum_event_q: *accounts.serum_event_q.key,
            serum_bids: *accounts.serum_bids.key,
            serum_asks: *accounts.serum_asks.key,
        }
    }
}
impl From<WithdrawKeys> for [AccountMeta; WITHDRAW_IX_ACCOUNTS_LEN] {
    fn from(keys: WithdrawKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.amm,
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
                pubkey: keys.amm_target_orders,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.lp_mint_address,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.pool_coin_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.pool_pc_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.pool_withdraw_queue,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.pool_temp_lp_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.serum_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.serum_market,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.serum_coin_vault_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.serum_pc_vault_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.serum_vault_signer,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.user_lp_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_coin_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_pc_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_owner,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.serum_event_q,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.serum_bids,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.serum_asks,
                is_signer: false,
                is_writable: true,
            },
        ]
    }
}
impl From<[Pubkey; WITHDRAW_IX_ACCOUNTS_LEN]> for WithdrawKeys {
    fn from(pubkeys: [Pubkey; WITHDRAW_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            token_program: pubkeys[0],
            amm: pubkeys[1],
            amm_authority: pubkeys[2],
            amm_open_orders: pubkeys[3],
            amm_target_orders: pubkeys[4],
            lp_mint_address: pubkeys[5],
            pool_coin_token_account: pubkeys[6],
            pool_pc_token_account: pubkeys[7],
            pool_withdraw_queue: pubkeys[8],
            pool_temp_lp_token_account: pubkeys[9],
            serum_program: pubkeys[10],
            serum_market: pubkeys[11],
            serum_coin_vault_account: pubkeys[12],
            serum_pc_vault_account: pubkeys[13],
            serum_vault_signer: pubkeys[14],
            user_lp_token_account: pubkeys[15],
            user_coin_token_account: pubkeys[16],
            user_pc_token_account: pubkeys[17],
            user_owner: pubkeys[18],
            serum_event_q: pubkeys[19],
            serum_bids: pubkeys[20],
            serum_asks: pubkeys[21],
        }
    }
}
impl<'info> From<WithdrawAccounts<'_, 'info>>
for [AccountInfo<'info>; WITHDRAW_IX_ACCOUNTS_LEN] {
    fn from(accounts: WithdrawAccounts<'_, 'info>) -> Self {
        [
            accounts.token_program.clone(),
            accounts.amm.clone(),
            accounts.amm_authority.clone(),
            accounts.amm_open_orders.clone(),
            accounts.amm_target_orders.clone(),
            accounts.lp_mint_address.clone(),
            accounts.pool_coin_token_account.clone(),
            accounts.pool_pc_token_account.clone(),
            accounts.pool_withdraw_queue.clone(),
            accounts.pool_temp_lp_token_account.clone(),
            accounts.serum_program.clone(),
            accounts.serum_market.clone(),
            accounts.serum_coin_vault_account.clone(),
            accounts.serum_pc_vault_account.clone(),
            accounts.serum_vault_signer.clone(),
            accounts.user_lp_token_account.clone(),
            accounts.user_coin_token_account.clone(),
            accounts.user_pc_token_account.clone(),
            accounts.user_owner.clone(),
            accounts.serum_event_q.clone(),
            accounts.serum_bids.clone(),
            accounts.serum_asks.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; WITHDRAW_IX_ACCOUNTS_LEN]>
for WithdrawAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; WITHDRAW_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            token_program: &arr[0],
            amm: &arr[1],
            amm_authority: &arr[2],
            amm_open_orders: &arr[3],
            amm_target_orders: &arr[4],
            lp_mint_address: &arr[5],
            pool_coin_token_account: &arr[6],
            pool_pc_token_account: &arr[7],
            pool_withdraw_queue: &arr[8],
            pool_temp_lp_token_account: &arr[9],
            serum_program: &arr[10],
            serum_market: &arr[11],
            serum_coin_vault_account: &arr[12],
            serum_pc_vault_account: &arr[13],
            serum_vault_signer: &arr[14],
            user_lp_token_account: &arr[15],
            user_coin_token_account: &arr[16],
            user_pc_token_account: &arr[17],
            user_owner: &arr[18],
            serum_event_q: &arr[19],
            serum_bids: &arr[20],
            serum_asks: &arr[21],
        }
    }
}
pub const WITHDRAW_IX_DISCM: u8 = 4u8;
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct WithdrawIxArgs {
    pub amount: u64,
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
        let mut maybe_discm_buf = [0u8; 1];
        reader.read_exact(&mut maybe_discm_buf)?;
        let maybe_discm = maybe_discm_buf[0];
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
        writer.write_all(&[WITHDRAW_IX_DISCM])?;
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
    let metas: [AccountMeta; WITHDRAW_IX_ACCOUNTS_LEN] = keys.into();
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
        (accounts.token_program.key, &keys.token_program),
        (accounts.amm.key, &keys.amm),
        (accounts.amm_authority.key, &keys.amm_authority),
        (accounts.amm_open_orders.key, &keys.amm_open_orders),
        (accounts.amm_target_orders.key, &keys.amm_target_orders),
        (accounts.lp_mint_address.key, &keys.lp_mint_address),
        (accounts.pool_coin_token_account.key, &keys.pool_coin_token_account),
        (accounts.pool_pc_token_account.key, &keys.pool_pc_token_account),
        (accounts.pool_withdraw_queue.key, &keys.pool_withdraw_queue),
        (accounts.pool_temp_lp_token_account.key, &keys.pool_temp_lp_token_account),
        (accounts.serum_program.key, &keys.serum_program),
        (accounts.serum_market.key, &keys.serum_market),
        (accounts.serum_coin_vault_account.key, &keys.serum_coin_vault_account),
        (accounts.serum_pc_vault_account.key, &keys.serum_pc_vault_account),
        (accounts.serum_vault_signer.key, &keys.serum_vault_signer),
        (accounts.user_lp_token_account.key, &keys.user_lp_token_account),
        (accounts.user_coin_token_account.key, &keys.user_coin_token_account),
        (accounts.user_pc_token_account.key, &keys.user_pc_token_account),
        (accounts.user_owner.key, &keys.user_owner),
        (accounts.serum_event_q.key, &keys.serum_event_q),
        (accounts.serum_bids.key, &keys.serum_bids),
        (accounts.serum_asks.key, &keys.serum_asks),
    ] {
        if actual != expected {
            return Err((*actual, *expected));
        }
    }
    Ok(())
}
pub fn withdraw_verify_writable_privileges<'me, 'info>(
    accounts: WithdrawAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.amm,
        accounts.amm_open_orders,
        accounts.amm_target_orders,
        accounts.lp_mint_address,
        accounts.pool_coin_token_account,
        accounts.pool_pc_token_account,
        accounts.pool_withdraw_queue,
        accounts.pool_temp_lp_token_account,
        accounts.serum_market,
        accounts.serum_coin_vault_account,
        accounts.serum_pc_vault_account,
        accounts.user_lp_token_account,
        accounts.user_coin_token_account,
        accounts.user_pc_token_account,
        accounts.serum_event_q,
        accounts.serum_bids,
        accounts.serum_asks,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn withdraw_verify_signer_privileges<'me, 'info>(
    accounts: WithdrawAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.user_owner] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn withdraw_verify_account_privileges<'me, 'info>(
    accounts: WithdrawAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    withdraw_verify_writable_privileges(accounts)?;
    withdraw_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const MIGRATE_TO_OPEN_BOOK_IX_ACCOUNTS_LEN: usize = 21;
#[derive(Copy, Clone, Debug)]
pub struct MigrateToOpenBookAccounts<'me, 'info> {
    pub token_program: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub rent: &'me AccountInfo<'info>,
    pub amm: &'me AccountInfo<'info>,
    pub amm_authority: &'me AccountInfo<'info>,
    pub amm_open_orders: &'me AccountInfo<'info>,
    pub amm_token_coin: &'me AccountInfo<'info>,
    pub amm_token_pc: &'me AccountInfo<'info>,
    pub amm_target_orders: &'me AccountInfo<'info>,
    pub serum_program: &'me AccountInfo<'info>,
    pub serum_market: &'me AccountInfo<'info>,
    pub serum_bids: &'me AccountInfo<'info>,
    pub serum_asks: &'me AccountInfo<'info>,
    pub serum_event_queue: &'me AccountInfo<'info>,
    pub serum_coin_vault: &'me AccountInfo<'info>,
    pub serum_pc_vault: &'me AccountInfo<'info>,
    pub serum_vault_signer: &'me AccountInfo<'info>,
    pub new_amm_open_orders: &'me AccountInfo<'info>,
    pub new_serum_program: &'me AccountInfo<'info>,
    pub new_serum_market: &'me AccountInfo<'info>,
    pub admin: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug)]
pub struct MigrateToOpenBookKeys {
    pub token_program: Pubkey,
    pub system_program: Pubkey,
    pub rent: Pubkey,
    pub amm: Pubkey,
    pub amm_authority: Pubkey,
    pub amm_open_orders: Pubkey,
    pub amm_token_coin: Pubkey,
    pub amm_token_pc: Pubkey,
    pub amm_target_orders: Pubkey,
    pub serum_program: Pubkey,
    pub serum_market: Pubkey,
    pub serum_bids: Pubkey,
    pub serum_asks: Pubkey,
    pub serum_event_queue: Pubkey,
    pub serum_coin_vault: Pubkey,
    pub serum_pc_vault: Pubkey,
    pub serum_vault_signer: Pubkey,
    pub new_amm_open_orders: Pubkey,
    pub new_serum_program: Pubkey,
    pub new_serum_market: Pubkey,
    pub admin: Pubkey,
}
impl From<MigrateToOpenBookAccounts<'_, '_>> for MigrateToOpenBookKeys {
    fn from(accounts: MigrateToOpenBookAccounts) -> Self {
        Self {
            token_program: *accounts.token_program.key,
            system_program: *accounts.system_program.key,
            rent: *accounts.rent.key,
            amm: *accounts.amm.key,
            amm_authority: *accounts.amm_authority.key,
            amm_open_orders: *accounts.amm_open_orders.key,
            amm_token_coin: *accounts.amm_token_coin.key,
            amm_token_pc: *accounts.amm_token_pc.key,
            amm_target_orders: *accounts.amm_target_orders.key,
            serum_program: *accounts.serum_program.key,
            serum_market: *accounts.serum_market.key,
            serum_bids: *accounts.serum_bids.key,
            serum_asks: *accounts.serum_asks.key,
            serum_event_queue: *accounts.serum_event_queue.key,
            serum_coin_vault: *accounts.serum_coin_vault.key,
            serum_pc_vault: *accounts.serum_pc_vault.key,
            serum_vault_signer: *accounts.serum_vault_signer.key,
            new_amm_open_orders: *accounts.new_amm_open_orders.key,
            new_serum_program: *accounts.new_serum_program.key,
            new_serum_market: *accounts.new_serum_market.key,
            admin: *accounts.admin.key,
        }
    }
}
impl From<MigrateToOpenBookKeys>
for [AccountMeta; MIGRATE_TO_OPEN_BOOK_IX_ACCOUNTS_LEN] {
    fn from(keys: MigrateToOpenBookKeys) -> Self {
        [
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
                pubkey: keys.amm,
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
                pubkey: keys.amm_token_coin,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.amm_token_pc,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.amm_target_orders,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.serum_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.serum_market,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.serum_bids,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.serum_asks,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.serum_event_queue,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.serum_coin_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.serum_pc_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.serum_vault_signer,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.new_amm_open_orders,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.new_serum_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.new_serum_market,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.admin,
                is_signer: true,
                is_writable: true,
            },
        ]
    }
}
impl From<[Pubkey; MIGRATE_TO_OPEN_BOOK_IX_ACCOUNTS_LEN]> for MigrateToOpenBookKeys {
    fn from(pubkeys: [Pubkey; MIGRATE_TO_OPEN_BOOK_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            token_program: pubkeys[0],
            system_program: pubkeys[1],
            rent: pubkeys[2],
            amm: pubkeys[3],
            amm_authority: pubkeys[4],
            amm_open_orders: pubkeys[5],
            amm_token_coin: pubkeys[6],
            amm_token_pc: pubkeys[7],
            amm_target_orders: pubkeys[8],
            serum_program: pubkeys[9],
            serum_market: pubkeys[10],
            serum_bids: pubkeys[11],
            serum_asks: pubkeys[12],
            serum_event_queue: pubkeys[13],
            serum_coin_vault: pubkeys[14],
            serum_pc_vault: pubkeys[15],
            serum_vault_signer: pubkeys[16],
            new_amm_open_orders: pubkeys[17],
            new_serum_program: pubkeys[18],
            new_serum_market: pubkeys[19],
            admin: pubkeys[20],
        }
    }
}
impl<'info> From<MigrateToOpenBookAccounts<'_, 'info>>
for [AccountInfo<'info>; MIGRATE_TO_OPEN_BOOK_IX_ACCOUNTS_LEN] {
    fn from(accounts: MigrateToOpenBookAccounts<'_, 'info>) -> Self {
        [
            accounts.token_program.clone(),
            accounts.system_program.clone(),
            accounts.rent.clone(),
            accounts.amm.clone(),
            accounts.amm_authority.clone(),
            accounts.amm_open_orders.clone(),
            accounts.amm_token_coin.clone(),
            accounts.amm_token_pc.clone(),
            accounts.amm_target_orders.clone(),
            accounts.serum_program.clone(),
            accounts.serum_market.clone(),
            accounts.serum_bids.clone(),
            accounts.serum_asks.clone(),
            accounts.serum_event_queue.clone(),
            accounts.serum_coin_vault.clone(),
            accounts.serum_pc_vault.clone(),
            accounts.serum_vault_signer.clone(),
            accounts.new_amm_open_orders.clone(),
            accounts.new_serum_program.clone(),
            accounts.new_serum_market.clone(),
            accounts.admin.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; MIGRATE_TO_OPEN_BOOK_IX_ACCOUNTS_LEN]>
for MigrateToOpenBookAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; MIGRATE_TO_OPEN_BOOK_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            token_program: &arr[0],
            system_program: &arr[1],
            rent: &arr[2],
            amm: &arr[3],
            amm_authority: &arr[4],
            amm_open_orders: &arr[5],
            amm_token_coin: &arr[6],
            amm_token_pc: &arr[7],
            amm_target_orders: &arr[8],
            serum_program: &arr[9],
            serum_market: &arr[10],
            serum_bids: &arr[11],
            serum_asks: &arr[12],
            serum_event_queue: &arr[13],
            serum_coin_vault: &arr[14],
            serum_pc_vault: &arr[15],
            serum_vault_signer: &arr[16],
            new_amm_open_orders: &arr[17],
            new_serum_program: &arr[18],
            new_serum_market: &arr[19],
            admin: &arr[20],
        }
    }
}
pub const MIGRATE_TO_OPEN_BOOK_IX_DISCM: u8 = 5u8;
#[derive(Clone, Debug, PartialEq)]
pub struct MigrateToOpenBookIxData;
impl MigrateToOpenBookIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm_buf = [0u8; 1];
        reader.read_exact(&mut maybe_discm_buf)?;
        let maybe_discm = maybe_discm_buf[0];
        if maybe_discm != MIGRATE_TO_OPEN_BOOK_IX_DISCM {
            return Err(
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!(
                        "discm does not match. Expected: {:?}. Received: {:?}",
                        MIGRATE_TO_OPEN_BOOK_IX_DISCM, maybe_discm
                    ),
                ),
            );
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&[MIGRATE_TO_OPEN_BOOK_IX_DISCM])
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn migrate_to_open_book_ix_with_program_id(
    program_id: Pubkey,
    keys: MigrateToOpenBookKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; MIGRATE_TO_OPEN_BOOK_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: MigrateToOpenBookIxData.try_to_vec()?,
    })
}
pub fn migrate_to_open_book_ix(
    keys: MigrateToOpenBookKeys,
) -> std::io::Result<Instruction> {
    migrate_to_open_book_ix_with_program_id(crate::ID, keys)
}
pub fn migrate_to_open_book_invoke_with_program_id(
    program_id: Pubkey,
    accounts: MigrateToOpenBookAccounts<'_, '_>,
) -> ProgramResult {
    let keys: MigrateToOpenBookKeys = accounts.into();
    let ix = migrate_to_open_book_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn migrate_to_open_book_invoke(
    accounts: MigrateToOpenBookAccounts<'_, '_>,
) -> ProgramResult {
    migrate_to_open_book_invoke_with_program_id(crate::ID, accounts)
}
pub fn migrate_to_open_book_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: MigrateToOpenBookAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: MigrateToOpenBookKeys = accounts.into();
    let ix = migrate_to_open_book_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn migrate_to_open_book_invoke_signed(
    accounts: MigrateToOpenBookAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    migrate_to_open_book_invoke_signed_with_program_id(crate::ID, accounts, seeds)
}
pub fn migrate_to_open_book_verify_account_keys(
    accounts: MigrateToOpenBookAccounts<'_, '_>,
    keys: MigrateToOpenBookKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (accounts.token_program.key, &keys.token_program),
        (accounts.system_program.key, &keys.system_program),
        (accounts.rent.key, &keys.rent),
        (accounts.amm.key, &keys.amm),
        (accounts.amm_authority.key, &keys.amm_authority),
        (accounts.amm_open_orders.key, &keys.amm_open_orders),
        (accounts.amm_token_coin.key, &keys.amm_token_coin),
        (accounts.amm_token_pc.key, &keys.amm_token_pc),
        (accounts.amm_target_orders.key, &keys.amm_target_orders),
        (accounts.serum_program.key, &keys.serum_program),
        (accounts.serum_market.key, &keys.serum_market),
        (accounts.serum_bids.key, &keys.serum_bids),
        (accounts.serum_asks.key, &keys.serum_asks),
        (accounts.serum_event_queue.key, &keys.serum_event_queue),
        (accounts.serum_coin_vault.key, &keys.serum_coin_vault),
        (accounts.serum_pc_vault.key, &keys.serum_pc_vault),
        (accounts.serum_vault_signer.key, &keys.serum_vault_signer),
        (accounts.new_amm_open_orders.key, &keys.new_amm_open_orders),
        (accounts.new_serum_program.key, &keys.new_serum_program),
        (accounts.new_serum_market.key, &keys.new_serum_market),
        (accounts.admin.key, &keys.admin),
    ] {
        if actual != expected {
            return Err((*actual, *expected));
        }
    }
    Ok(())
}
pub fn migrate_to_open_book_verify_writable_privileges<'me, 'info>(
    accounts: MigrateToOpenBookAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.amm,
        accounts.amm_open_orders,
        accounts.amm_token_coin,
        accounts.amm_token_pc,
        accounts.amm_target_orders,
        accounts.serum_market,
        accounts.serum_bids,
        accounts.serum_asks,
        accounts.serum_event_queue,
        accounts.serum_coin_vault,
        accounts.serum_pc_vault,
        accounts.new_amm_open_orders,
        accounts.admin,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn migrate_to_open_book_verify_signer_privileges<'me, 'info>(
    accounts: MigrateToOpenBookAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.admin] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn migrate_to_open_book_verify_account_privileges<'me, 'info>(
    accounts: MigrateToOpenBookAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    migrate_to_open_book_verify_writable_privileges(accounts)?;
    migrate_to_open_book_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const SET_PARAMS_IX_ACCOUNTS_LEN: usize = 17;
#[derive(Copy, Clone, Debug)]
pub struct SetParamsAccounts<'me, 'info> {
    pub token_program: &'me AccountInfo<'info>,
    pub amm: &'me AccountInfo<'info>,
    pub amm_authority: &'me AccountInfo<'info>,
    pub amm_open_orders: &'me AccountInfo<'info>,
    pub amm_target_orders: &'me AccountInfo<'info>,
    pub amm_coin_vault: &'me AccountInfo<'info>,
    pub amm_pc_vault: &'me AccountInfo<'info>,
    pub serum_program: &'me AccountInfo<'info>,
    pub serum_market: &'me AccountInfo<'info>,
    pub serum_coin_vault: &'me AccountInfo<'info>,
    pub serum_pc_vault: &'me AccountInfo<'info>,
    pub serum_vault_signer: &'me AccountInfo<'info>,
    pub serum_event_queue: &'me AccountInfo<'info>,
    pub serum_bids: &'me AccountInfo<'info>,
    pub serum_asks: &'me AccountInfo<'info>,
    pub amm_admin_account: &'me AccountInfo<'info>,
    pub new_amm_open_orders_account: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug)]
pub struct SetParamsKeys {
    pub token_program: Pubkey,
    pub amm: Pubkey,
    pub amm_authority: Pubkey,
    pub amm_open_orders: Pubkey,
    pub amm_target_orders: Pubkey,
    pub amm_coin_vault: Pubkey,
    pub amm_pc_vault: Pubkey,
    pub serum_program: Pubkey,
    pub serum_market: Pubkey,
    pub serum_coin_vault: Pubkey,
    pub serum_pc_vault: Pubkey,
    pub serum_vault_signer: Pubkey,
    pub serum_event_queue: Pubkey,
    pub serum_bids: Pubkey,
    pub serum_asks: Pubkey,
    pub amm_admin_account: Pubkey,
    pub new_amm_open_orders_account: Pubkey,
}
impl From<SetParamsAccounts<'_, '_>> for SetParamsKeys {
    fn from(accounts: SetParamsAccounts) -> Self {
        Self {
            token_program: *accounts.token_program.key,
            amm: *accounts.amm.key,
            amm_authority: *accounts.amm_authority.key,
            amm_open_orders: *accounts.amm_open_orders.key,
            amm_target_orders: *accounts.amm_target_orders.key,
            amm_coin_vault: *accounts.amm_coin_vault.key,
            amm_pc_vault: *accounts.amm_pc_vault.key,
            serum_program: *accounts.serum_program.key,
            serum_market: *accounts.serum_market.key,
            serum_coin_vault: *accounts.serum_coin_vault.key,
            serum_pc_vault: *accounts.serum_pc_vault.key,
            serum_vault_signer: *accounts.serum_vault_signer.key,
            serum_event_queue: *accounts.serum_event_queue.key,
            serum_bids: *accounts.serum_bids.key,
            serum_asks: *accounts.serum_asks.key,
            amm_admin_account: *accounts.amm_admin_account.key,
            new_amm_open_orders_account: *accounts.new_amm_open_orders_account.key,
        }
    }
}
impl From<SetParamsKeys> for [AccountMeta; SET_PARAMS_IX_ACCOUNTS_LEN] {
    fn from(keys: SetParamsKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.amm,
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
                pubkey: keys.amm_target_orders,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.amm_coin_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.amm_pc_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.serum_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.serum_market,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.serum_coin_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.serum_pc_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.serum_vault_signer,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.serum_event_queue,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.serum_bids,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.serum_asks,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.amm_admin_account,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.new_amm_open_orders_account,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; SET_PARAMS_IX_ACCOUNTS_LEN]> for SetParamsKeys {
    fn from(pubkeys: [Pubkey; SET_PARAMS_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            token_program: pubkeys[0],
            amm: pubkeys[1],
            amm_authority: pubkeys[2],
            amm_open_orders: pubkeys[3],
            amm_target_orders: pubkeys[4],
            amm_coin_vault: pubkeys[5],
            amm_pc_vault: pubkeys[6],
            serum_program: pubkeys[7],
            serum_market: pubkeys[8],
            serum_coin_vault: pubkeys[9],
            serum_pc_vault: pubkeys[10],
            serum_vault_signer: pubkeys[11],
            serum_event_queue: pubkeys[12],
            serum_bids: pubkeys[13],
            serum_asks: pubkeys[14],
            amm_admin_account: pubkeys[15],
            new_amm_open_orders_account: pubkeys[16],
        }
    }
}
impl<'info> From<SetParamsAccounts<'_, 'info>>
for [AccountInfo<'info>; SET_PARAMS_IX_ACCOUNTS_LEN] {
    fn from(accounts: SetParamsAccounts<'_, 'info>) -> Self {
        [
            accounts.token_program.clone(),
            accounts.amm.clone(),
            accounts.amm_authority.clone(),
            accounts.amm_open_orders.clone(),
            accounts.amm_target_orders.clone(),
            accounts.amm_coin_vault.clone(),
            accounts.amm_pc_vault.clone(),
            accounts.serum_program.clone(),
            accounts.serum_market.clone(),
            accounts.serum_coin_vault.clone(),
            accounts.serum_pc_vault.clone(),
            accounts.serum_vault_signer.clone(),
            accounts.serum_event_queue.clone(),
            accounts.serum_bids.clone(),
            accounts.serum_asks.clone(),
            accounts.amm_admin_account.clone(),
            accounts.new_amm_open_orders_account.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; SET_PARAMS_IX_ACCOUNTS_LEN]>
for SetParamsAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; SET_PARAMS_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            token_program: &arr[0],
            amm: &arr[1],
            amm_authority: &arr[2],
            amm_open_orders: &arr[3],
            amm_target_orders: &arr[4],
            amm_coin_vault: &arr[5],
            amm_pc_vault: &arr[6],
            serum_program: &arr[7],
            serum_market: &arr[8],
            serum_coin_vault: &arr[9],
            serum_pc_vault: &arr[10],
            serum_vault_signer: &arr[11],
            serum_event_queue: &arr[12],
            serum_bids: &arr[13],
            serum_asks: &arr[14],
            amm_admin_account: &arr[15],
            new_amm_open_orders_account: &arr[16],
        }
    }
}
pub const SET_PARAMS_IX_DISCM: u8 = 6u8;
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SetParamsIxArgs {
    pub param: u8,
    pub value: Option<u64>,
    pub new_pubkey: Option<Pubkey>,
    pub fees: Option<Fees>,
    pub last_order_distance: Option<LastOrderDistance>,
    pub need_take_amounts: Option<NeedTake>,
}
#[derive(Clone, Debug, PartialEq)]
pub struct SetParamsIxData(pub SetParamsIxArgs);
impl From<SetParamsIxArgs> for SetParamsIxData {
    fn from(args: SetParamsIxArgs) -> Self {
        Self(args)
    }
}
impl SetParamsIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm_buf = [0u8; 1];
        reader.read_exact(&mut maybe_discm_buf)?;
        let maybe_discm = maybe_discm_buf[0];
        if maybe_discm != SET_PARAMS_IX_DISCM {
            return Err(
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!(
                        "discm does not match. Expected: {:?}. Received: {:?}",
                        SET_PARAMS_IX_DISCM, maybe_discm
                    ),
                ),
            );
        }
        Ok(Self(SetParamsIxArgs::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&[SET_PARAMS_IX_DISCM])?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
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
pub fn set_params_ix(
    keys: SetParamsKeys,
    args: SetParamsIxArgs,
) -> std::io::Result<Instruction> {
    set_params_ix_with_program_id(crate::ID, keys, args)
}
pub fn set_params_invoke_with_program_id(
    program_id: Pubkey,
    accounts: SetParamsAccounts<'_, '_>,
    args: SetParamsIxArgs,
) -> ProgramResult {
    let keys: SetParamsKeys = accounts.into();
    let ix = set_params_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn set_params_invoke(
    accounts: SetParamsAccounts<'_, '_>,
    args: SetParamsIxArgs,
) -> ProgramResult {
    set_params_invoke_with_program_id(crate::ID, accounts, args)
}
pub fn set_params_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: SetParamsAccounts<'_, '_>,
    args: SetParamsIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: SetParamsKeys = accounts.into();
    let ix = set_params_ix_with_program_id(program_id, keys, args)?;
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
        (accounts.token_program.key, &keys.token_program),
        (accounts.amm.key, &keys.amm),
        (accounts.amm_authority.key, &keys.amm_authority),
        (accounts.amm_open_orders.key, &keys.amm_open_orders),
        (accounts.amm_target_orders.key, &keys.amm_target_orders),
        (accounts.amm_coin_vault.key, &keys.amm_coin_vault),
        (accounts.amm_pc_vault.key, &keys.amm_pc_vault),
        (accounts.serum_program.key, &keys.serum_program),
        (accounts.serum_market.key, &keys.serum_market),
        (accounts.serum_coin_vault.key, &keys.serum_coin_vault),
        (accounts.serum_pc_vault.key, &keys.serum_pc_vault),
        (accounts.serum_vault_signer.key, &keys.serum_vault_signer),
        (accounts.serum_event_queue.key, &keys.serum_event_queue),
        (accounts.serum_bids.key, &keys.serum_bids),
        (accounts.serum_asks.key, &keys.serum_asks),
        (accounts.amm_admin_account.key, &keys.amm_admin_account),
        (accounts.new_amm_open_orders_account.key, &keys.new_amm_open_orders_account),
    ] {
        if actual != expected {
            return Err((*actual, *expected));
        }
    }
    Ok(())
}
pub fn set_params_verify_writable_privileges<'me, 'info>(
    accounts: SetParamsAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.amm,
        accounts.amm_open_orders,
        accounts.amm_target_orders,
        accounts.amm_coin_vault,
        accounts.amm_pc_vault,
        accounts.serum_market,
        accounts.serum_coin_vault,
        accounts.serum_pc_vault,
        accounts.serum_event_queue,
        accounts.serum_bids,
        accounts.serum_asks,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn set_params_verify_signer_privileges<'me, 'info>(
    accounts: SetParamsAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.amm_admin_account] {
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
pub const WITHDRAW_PNL_IX_ACCOUNTS_LEN: usize = 18;
#[derive(Copy, Clone, Debug)]
pub struct WithdrawPnlAccounts<'me, 'info> {
    pub token_program: &'me AccountInfo<'info>,
    pub amm: &'me AccountInfo<'info>,
    pub amm_config: &'me AccountInfo<'info>,
    pub amm_authority: &'me AccountInfo<'info>,
    pub amm_open_orders: &'me AccountInfo<'info>,
    pub pool_coin_token_account: &'me AccountInfo<'info>,
    pub pool_pc_token_account: &'me AccountInfo<'info>,
    pub coin_pnl_token_account: &'me AccountInfo<'info>,
    pub pc_pnl_token_account: &'me AccountInfo<'info>,
    pub pnl_owner_account: &'me AccountInfo<'info>,
    pub amm_target_orders: &'me AccountInfo<'info>,
    pub serum_program: &'me AccountInfo<'info>,
    pub serum_market: &'me AccountInfo<'info>,
    pub serum_event_queue: &'me AccountInfo<'info>,
    pub serum_coin_vault_account: &'me AccountInfo<'info>,
    pub serum_pc_vault_account: &'me AccountInfo<'info>,
    pub serum_vault_signer: &'me AccountInfo<'info>,
    pub referrer_pc_account: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug)]
pub struct WithdrawPnlKeys {
    pub token_program: Pubkey,
    pub amm: Pubkey,
    pub amm_config: Pubkey,
    pub amm_authority: Pubkey,
    pub amm_open_orders: Pubkey,
    pub pool_coin_token_account: Pubkey,
    pub pool_pc_token_account: Pubkey,
    pub coin_pnl_token_account: Pubkey,
    pub pc_pnl_token_account: Pubkey,
    pub pnl_owner_account: Pubkey,
    pub amm_target_orders: Pubkey,
    pub serum_program: Pubkey,
    pub serum_market: Pubkey,
    pub serum_event_queue: Pubkey,
    pub serum_coin_vault_account: Pubkey,
    pub serum_pc_vault_account: Pubkey,
    pub serum_vault_signer: Pubkey,
    pub referrer_pc_account: Pubkey,
}
impl From<WithdrawPnlAccounts<'_, '_>> for WithdrawPnlKeys {
    fn from(accounts: WithdrawPnlAccounts) -> Self {
        Self {
            token_program: *accounts.token_program.key,
            amm: *accounts.amm.key,
            amm_config: *accounts.amm_config.key,
            amm_authority: *accounts.amm_authority.key,
            amm_open_orders: *accounts.amm_open_orders.key,
            pool_coin_token_account: *accounts.pool_coin_token_account.key,
            pool_pc_token_account: *accounts.pool_pc_token_account.key,
            coin_pnl_token_account: *accounts.coin_pnl_token_account.key,
            pc_pnl_token_account: *accounts.pc_pnl_token_account.key,
            pnl_owner_account: *accounts.pnl_owner_account.key,
            amm_target_orders: *accounts.amm_target_orders.key,
            serum_program: *accounts.serum_program.key,
            serum_market: *accounts.serum_market.key,
            serum_event_queue: *accounts.serum_event_queue.key,
            serum_coin_vault_account: *accounts.serum_coin_vault_account.key,
            serum_pc_vault_account: *accounts.serum_pc_vault_account.key,
            serum_vault_signer: *accounts.serum_vault_signer.key,
            referrer_pc_account: *accounts.referrer_pc_account.key,
        }
    }
}
impl From<WithdrawPnlKeys> for [AccountMeta; WITHDRAW_PNL_IX_ACCOUNTS_LEN] {
    fn from(keys: WithdrawPnlKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.amm,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.amm_config,
                is_signer: false,
                is_writable: false,
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
                pubkey: keys.pool_coin_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.pool_pc_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.coin_pnl_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.pc_pnl_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.pnl_owner_account,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.amm_target_orders,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.serum_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.serum_market,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.serum_event_queue,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.serum_coin_vault_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.serum_pc_vault_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.serum_vault_signer,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.referrer_pc_account,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; WITHDRAW_PNL_IX_ACCOUNTS_LEN]> for WithdrawPnlKeys {
    fn from(pubkeys: [Pubkey; WITHDRAW_PNL_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            token_program: pubkeys[0],
            amm: pubkeys[1],
            amm_config: pubkeys[2],
            amm_authority: pubkeys[3],
            amm_open_orders: pubkeys[4],
            pool_coin_token_account: pubkeys[5],
            pool_pc_token_account: pubkeys[6],
            coin_pnl_token_account: pubkeys[7],
            pc_pnl_token_account: pubkeys[8],
            pnl_owner_account: pubkeys[9],
            amm_target_orders: pubkeys[10],
            serum_program: pubkeys[11],
            serum_market: pubkeys[12],
            serum_event_queue: pubkeys[13],
            serum_coin_vault_account: pubkeys[14],
            serum_pc_vault_account: pubkeys[15],
            serum_vault_signer: pubkeys[16],
            referrer_pc_account: pubkeys[17],
        }
    }
}
impl<'info> From<WithdrawPnlAccounts<'_, 'info>>
for [AccountInfo<'info>; WITHDRAW_PNL_IX_ACCOUNTS_LEN] {
    fn from(accounts: WithdrawPnlAccounts<'_, 'info>) -> Self {
        [
            accounts.token_program.clone(),
            accounts.amm.clone(),
            accounts.amm_config.clone(),
            accounts.amm_authority.clone(),
            accounts.amm_open_orders.clone(),
            accounts.pool_coin_token_account.clone(),
            accounts.pool_pc_token_account.clone(),
            accounts.coin_pnl_token_account.clone(),
            accounts.pc_pnl_token_account.clone(),
            accounts.pnl_owner_account.clone(),
            accounts.amm_target_orders.clone(),
            accounts.serum_program.clone(),
            accounts.serum_market.clone(),
            accounts.serum_event_queue.clone(),
            accounts.serum_coin_vault_account.clone(),
            accounts.serum_pc_vault_account.clone(),
            accounts.serum_vault_signer.clone(),
            accounts.referrer_pc_account.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; WITHDRAW_PNL_IX_ACCOUNTS_LEN]>
for WithdrawPnlAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; WITHDRAW_PNL_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            token_program: &arr[0],
            amm: &arr[1],
            amm_config: &arr[2],
            amm_authority: &arr[3],
            amm_open_orders: &arr[4],
            pool_coin_token_account: &arr[5],
            pool_pc_token_account: &arr[6],
            coin_pnl_token_account: &arr[7],
            pc_pnl_token_account: &arr[8],
            pnl_owner_account: &arr[9],
            amm_target_orders: &arr[10],
            serum_program: &arr[11],
            serum_market: &arr[12],
            serum_event_queue: &arr[13],
            serum_coin_vault_account: &arr[14],
            serum_pc_vault_account: &arr[15],
            serum_vault_signer: &arr[16],
            referrer_pc_account: &arr[17],
        }
    }
}
pub const WITHDRAW_PNL_IX_DISCM: u8 = 7u8;
#[derive(Clone, Debug, PartialEq)]
pub struct WithdrawPnlIxData;
impl WithdrawPnlIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm_buf = [0u8; 1];
        reader.read_exact(&mut maybe_discm_buf)?;
        let maybe_discm = maybe_discm_buf[0];
        if maybe_discm != WITHDRAW_PNL_IX_DISCM {
            return Err(
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!(
                        "discm does not match. Expected: {:?}. Received: {:?}",
                        WITHDRAW_PNL_IX_DISCM, maybe_discm
                    ),
                ),
            );
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&[WITHDRAW_PNL_IX_DISCM])
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn withdraw_pnl_ix_with_program_id(
    program_id: Pubkey,
    keys: WithdrawPnlKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; WITHDRAW_PNL_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: WithdrawPnlIxData.try_to_vec()?,
    })
}
pub fn withdraw_pnl_ix(keys: WithdrawPnlKeys) -> std::io::Result<Instruction> {
    withdraw_pnl_ix_with_program_id(crate::ID, keys)
}
pub fn withdraw_pnl_invoke_with_program_id(
    program_id: Pubkey,
    accounts: WithdrawPnlAccounts<'_, '_>,
) -> ProgramResult {
    let keys: WithdrawPnlKeys = accounts.into();
    let ix = withdraw_pnl_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn withdraw_pnl_invoke(accounts: WithdrawPnlAccounts<'_, '_>) -> ProgramResult {
    withdraw_pnl_invoke_with_program_id(crate::ID, accounts)
}
pub fn withdraw_pnl_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: WithdrawPnlAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: WithdrawPnlKeys = accounts.into();
    let ix = withdraw_pnl_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn withdraw_pnl_invoke_signed(
    accounts: WithdrawPnlAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    withdraw_pnl_invoke_signed_with_program_id(crate::ID, accounts, seeds)
}
pub fn withdraw_pnl_verify_account_keys(
    accounts: WithdrawPnlAccounts<'_, '_>,
    keys: WithdrawPnlKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (accounts.token_program.key, &keys.token_program),
        (accounts.amm.key, &keys.amm),
        (accounts.amm_config.key, &keys.amm_config),
        (accounts.amm_authority.key, &keys.amm_authority),
        (accounts.amm_open_orders.key, &keys.amm_open_orders),
        (accounts.pool_coin_token_account.key, &keys.pool_coin_token_account),
        (accounts.pool_pc_token_account.key, &keys.pool_pc_token_account),
        (accounts.coin_pnl_token_account.key, &keys.coin_pnl_token_account),
        (accounts.pc_pnl_token_account.key, &keys.pc_pnl_token_account),
        (accounts.pnl_owner_account.key, &keys.pnl_owner_account),
        (accounts.amm_target_orders.key, &keys.amm_target_orders),
        (accounts.serum_program.key, &keys.serum_program),
        (accounts.serum_market.key, &keys.serum_market),
        (accounts.serum_event_queue.key, &keys.serum_event_queue),
        (accounts.serum_coin_vault_account.key, &keys.serum_coin_vault_account),
        (accounts.serum_pc_vault_account.key, &keys.serum_pc_vault_account),
        (accounts.serum_vault_signer.key, &keys.serum_vault_signer),
        (accounts.referrer_pc_account.key, &keys.referrer_pc_account),
    ] {
        if actual != expected {
            return Err((*actual, *expected));
        }
    }
    Ok(())
}
pub fn withdraw_pnl_verify_writable_privileges<'me, 'info>(
    accounts: WithdrawPnlAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.amm,
        accounts.amm_open_orders,
        accounts.pool_coin_token_account,
        accounts.pool_pc_token_account,
        accounts.coin_pnl_token_account,
        accounts.pc_pnl_token_account,
        accounts.amm_target_orders,
        accounts.serum_market,
        accounts.serum_coin_vault_account,
        accounts.serum_pc_vault_account,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn withdraw_pnl_verify_signer_privileges<'me, 'info>(
    accounts: WithdrawPnlAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.pnl_owner_account] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn withdraw_pnl_verify_account_privileges<'me, 'info>(
    accounts: WithdrawPnlAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    withdraw_pnl_verify_writable_privileges(accounts)?;
    withdraw_pnl_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const WITHDRAW_SRM_IX_ACCOUNTS_LEN: usize = 6;
#[derive(Copy, Clone, Debug)]
pub struct WithdrawSrmAccounts<'me, 'info> {
    pub token_program: &'me AccountInfo<'info>,
    pub amm: &'me AccountInfo<'info>,
    pub amm_owner_account: &'me AccountInfo<'info>,
    pub amm_authority: &'me AccountInfo<'info>,
    pub srm_token: &'me AccountInfo<'info>,
    pub dest_srm_token: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug)]
pub struct WithdrawSrmKeys {
    pub token_program: Pubkey,
    pub amm: Pubkey,
    pub amm_owner_account: Pubkey,
    pub amm_authority: Pubkey,
    pub srm_token: Pubkey,
    pub dest_srm_token: Pubkey,
}
impl From<WithdrawSrmAccounts<'_, '_>> for WithdrawSrmKeys {
    fn from(accounts: WithdrawSrmAccounts) -> Self {
        Self {
            token_program: *accounts.token_program.key,
            amm: *accounts.amm.key,
            amm_owner_account: *accounts.amm_owner_account.key,
            amm_authority: *accounts.amm_authority.key,
            srm_token: *accounts.srm_token.key,
            dest_srm_token: *accounts.dest_srm_token.key,
        }
    }
}
impl From<WithdrawSrmKeys> for [AccountMeta; WITHDRAW_SRM_IX_ACCOUNTS_LEN] {
    fn from(keys: WithdrawSrmKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.amm,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.amm_owner_account,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.amm_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.srm_token,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.dest_srm_token,
                is_signer: false,
                is_writable: true,
            },
        ]
    }
}
impl From<[Pubkey; WITHDRAW_SRM_IX_ACCOUNTS_LEN]> for WithdrawSrmKeys {
    fn from(pubkeys: [Pubkey; WITHDRAW_SRM_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            token_program: pubkeys[0],
            amm: pubkeys[1],
            amm_owner_account: pubkeys[2],
            amm_authority: pubkeys[3],
            srm_token: pubkeys[4],
            dest_srm_token: pubkeys[5],
        }
    }
}
impl<'info> From<WithdrawSrmAccounts<'_, 'info>>
for [AccountInfo<'info>; WITHDRAW_SRM_IX_ACCOUNTS_LEN] {
    fn from(accounts: WithdrawSrmAccounts<'_, 'info>) -> Self {
        [
            accounts.token_program.clone(),
            accounts.amm.clone(),
            accounts.amm_owner_account.clone(),
            accounts.amm_authority.clone(),
            accounts.srm_token.clone(),
            accounts.dest_srm_token.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; WITHDRAW_SRM_IX_ACCOUNTS_LEN]>
for WithdrawSrmAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; WITHDRAW_SRM_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            token_program: &arr[0],
            amm: &arr[1],
            amm_owner_account: &arr[2],
            amm_authority: &arr[3],
            srm_token: &arr[4],
            dest_srm_token: &arr[5],
        }
    }
}
pub const WITHDRAW_SRM_IX_DISCM: u8 = 8u8;
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct WithdrawSrmIxArgs {
    pub amount: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct WithdrawSrmIxData(pub WithdrawSrmIxArgs);
impl From<WithdrawSrmIxArgs> for WithdrawSrmIxData {
    fn from(args: WithdrawSrmIxArgs) -> Self {
        Self(args)
    }
}
impl WithdrawSrmIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm_buf = [0u8; 1];
        reader.read_exact(&mut maybe_discm_buf)?;
        let maybe_discm = maybe_discm_buf[0];
        if maybe_discm != WITHDRAW_SRM_IX_DISCM {
            return Err(
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!(
                        "discm does not match. Expected: {:?}. Received: {:?}",
                        WITHDRAW_SRM_IX_DISCM, maybe_discm
                    ),
                ),
            );
        }
        Ok(Self(WithdrawSrmIxArgs::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&[WITHDRAW_SRM_IX_DISCM])?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn withdraw_srm_ix_with_program_id(
    program_id: Pubkey,
    keys: WithdrawSrmKeys,
    args: WithdrawSrmIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; WITHDRAW_SRM_IX_ACCOUNTS_LEN] = keys.into();
    let data: WithdrawSrmIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn withdraw_srm_ix(
    keys: WithdrawSrmKeys,
    args: WithdrawSrmIxArgs,
) -> std::io::Result<Instruction> {
    withdraw_srm_ix_with_program_id(crate::ID, keys, args)
}
pub fn withdraw_srm_invoke_with_program_id(
    program_id: Pubkey,
    accounts: WithdrawSrmAccounts<'_, '_>,
    args: WithdrawSrmIxArgs,
) -> ProgramResult {
    let keys: WithdrawSrmKeys = accounts.into();
    let ix = withdraw_srm_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn withdraw_srm_invoke(
    accounts: WithdrawSrmAccounts<'_, '_>,
    args: WithdrawSrmIxArgs,
) -> ProgramResult {
    withdraw_srm_invoke_with_program_id(crate::ID, accounts, args)
}
pub fn withdraw_srm_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: WithdrawSrmAccounts<'_, '_>,
    args: WithdrawSrmIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: WithdrawSrmKeys = accounts.into();
    let ix = withdraw_srm_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn withdraw_srm_invoke_signed(
    accounts: WithdrawSrmAccounts<'_, '_>,
    args: WithdrawSrmIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    withdraw_srm_invoke_signed_with_program_id(crate::ID, accounts, args, seeds)
}
pub fn withdraw_srm_verify_account_keys(
    accounts: WithdrawSrmAccounts<'_, '_>,
    keys: WithdrawSrmKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (accounts.token_program.key, &keys.token_program),
        (accounts.amm.key, &keys.amm),
        (accounts.amm_owner_account.key, &keys.amm_owner_account),
        (accounts.amm_authority.key, &keys.amm_authority),
        (accounts.srm_token.key, &keys.srm_token),
        (accounts.dest_srm_token.key, &keys.dest_srm_token),
    ] {
        if actual != expected {
            return Err((*actual, *expected));
        }
    }
    Ok(())
}
pub fn withdraw_srm_verify_writable_privileges<'me, 'info>(
    accounts: WithdrawSrmAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.srm_token, accounts.dest_srm_token] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn withdraw_srm_verify_signer_privileges<'me, 'info>(
    accounts: WithdrawSrmAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.amm_owner_account] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn withdraw_srm_verify_account_privileges<'me, 'info>(
    accounts: WithdrawSrmAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    withdraw_srm_verify_writable_privileges(accounts)?;
    withdraw_srm_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const SWAP_BASE_IN_IX_ACCOUNTS_LEN: usize = 18;
#[derive(Copy, Clone, Debug)]
pub struct SwapBaseInAccounts<'me, 'info> {
    pub token_program: &'me AccountInfo<'info>,
    pub amm: &'me AccountInfo<'info>,
    pub amm_authority: &'me AccountInfo<'info>,
    pub amm_open_orders: &'me AccountInfo<'info>,
    pub amm_target_orders: &'me AccountInfo<'info>,
    pub pool_coin_token_account: &'me AccountInfo<'info>,
    pub pool_pc_token_account: &'me AccountInfo<'info>,
    pub serum_program: &'me AccountInfo<'info>,
    pub serum_market: &'me AccountInfo<'info>,
    pub serum_bids: &'me AccountInfo<'info>,
    pub serum_asks: &'me AccountInfo<'info>,
    pub serum_event_queue: &'me AccountInfo<'info>,
    pub serum_coin_vault_account: &'me AccountInfo<'info>,
    pub serum_pc_vault_account: &'me AccountInfo<'info>,
    pub serum_vault_signer: &'me AccountInfo<'info>,
    pub user_source_token_account: &'me AccountInfo<'info>,
    pub user_destination_token_account: &'me AccountInfo<'info>,
    pub user_source_owner: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug)]
pub struct SwapBaseInKeys {
    pub token_program: Pubkey,
    pub amm: Pubkey,
    pub amm_authority: Pubkey,
    pub amm_open_orders: Pubkey,
    pub amm_target_orders: Pubkey,
    pub pool_coin_token_account: Pubkey,
    pub pool_pc_token_account: Pubkey,
    pub serum_program: Pubkey,
    pub serum_market: Pubkey,
    pub serum_bids: Pubkey,
    pub serum_asks: Pubkey,
    pub serum_event_queue: Pubkey,
    pub serum_coin_vault_account: Pubkey,
    pub serum_pc_vault_account: Pubkey,
    pub serum_vault_signer: Pubkey,
    pub user_source_token_account: Pubkey,
    pub user_destination_token_account: Pubkey,
    pub user_source_owner: Pubkey,
}
impl From<SwapBaseInAccounts<'_, '_>> for SwapBaseInKeys {
    fn from(accounts: SwapBaseInAccounts) -> Self {
        Self {
            token_program: *accounts.token_program.key,
            amm: *accounts.amm.key,
            amm_authority: *accounts.amm_authority.key,
            amm_open_orders: *accounts.amm_open_orders.key,
            amm_target_orders: *accounts.amm_target_orders.key,
            pool_coin_token_account: *accounts.pool_coin_token_account.key,
            pool_pc_token_account: *accounts.pool_pc_token_account.key,
            serum_program: *accounts.serum_program.key,
            serum_market: *accounts.serum_market.key,
            serum_bids: *accounts.serum_bids.key,
            serum_asks: *accounts.serum_asks.key,
            serum_event_queue: *accounts.serum_event_queue.key,
            serum_coin_vault_account: *accounts.serum_coin_vault_account.key,
            serum_pc_vault_account: *accounts.serum_pc_vault_account.key,
            serum_vault_signer: *accounts.serum_vault_signer.key,
            user_source_token_account: *accounts.user_source_token_account.key,
            user_destination_token_account: *accounts.user_destination_token_account.key,
            user_source_owner: *accounts.user_source_owner.key,
        }
    }
}
impl From<SwapBaseInKeys> for [AccountMeta; SWAP_BASE_IN_IX_ACCOUNTS_LEN] {
    fn from(keys: SwapBaseInKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.amm,
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
                pubkey: keys.amm_target_orders,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.pool_coin_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.pool_pc_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.serum_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.serum_market,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.serum_bids,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.serum_asks,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.serum_event_queue,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.serum_coin_vault_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.serum_pc_vault_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.serum_vault_signer,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.user_source_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_destination_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_source_owner,
                is_signer: true,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; SWAP_BASE_IN_IX_ACCOUNTS_LEN]> for SwapBaseInKeys {
    fn from(pubkeys: [Pubkey; SWAP_BASE_IN_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            token_program: pubkeys[0],
            amm: pubkeys[1],
            amm_authority: pubkeys[2],
            amm_open_orders: pubkeys[3],
            amm_target_orders: pubkeys[4],
            pool_coin_token_account: pubkeys[5],
            pool_pc_token_account: pubkeys[6],
            serum_program: pubkeys[7],
            serum_market: pubkeys[8],
            serum_bids: pubkeys[9],
            serum_asks: pubkeys[10],
            serum_event_queue: pubkeys[11],
            serum_coin_vault_account: pubkeys[12],
            serum_pc_vault_account: pubkeys[13],
            serum_vault_signer: pubkeys[14],
            user_source_token_account: pubkeys[15],
            user_destination_token_account: pubkeys[16],
            user_source_owner: pubkeys[17],
        }
    }
}
impl<'info> From<SwapBaseInAccounts<'_, 'info>>
for [AccountInfo<'info>; SWAP_BASE_IN_IX_ACCOUNTS_LEN] {
    fn from(accounts: SwapBaseInAccounts<'_, 'info>) -> Self {
        [
            accounts.token_program.clone(),
            accounts.amm.clone(),
            accounts.amm_authority.clone(),
            accounts.amm_open_orders.clone(),
            accounts.amm_target_orders.clone(),
            accounts.pool_coin_token_account.clone(),
            accounts.pool_pc_token_account.clone(),
            accounts.serum_program.clone(),
            accounts.serum_market.clone(),
            accounts.serum_bids.clone(),
            accounts.serum_asks.clone(),
            accounts.serum_event_queue.clone(),
            accounts.serum_coin_vault_account.clone(),
            accounts.serum_pc_vault_account.clone(),
            accounts.serum_vault_signer.clone(),
            accounts.user_source_token_account.clone(),
            accounts.user_destination_token_account.clone(),
            accounts.user_source_owner.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; SWAP_BASE_IN_IX_ACCOUNTS_LEN]>
for SwapBaseInAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; SWAP_BASE_IN_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            token_program: &arr[0],
            amm: &arr[1],
            amm_authority: &arr[2],
            amm_open_orders: &arr[3],
            amm_target_orders: &arr[4],
            pool_coin_token_account: &arr[5],
            pool_pc_token_account: &arr[6],
            serum_program: &arr[7],
            serum_market: &arr[8],
            serum_bids: &arr[9],
            serum_asks: &arr[10],
            serum_event_queue: &arr[11],
            serum_coin_vault_account: &arr[12],
            serum_pc_vault_account: &arr[13],
            serum_vault_signer: &arr[14],
            user_source_token_account: &arr[15],
            user_destination_token_account: &arr[16],
            user_source_owner: &arr[17],
        }
    }
}
pub const SWAP_BASE_IN_IX_DISCM: u8 = 9u8;
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SwapBaseInIxArgs {
    pub amount_in: u64,
    pub minimum_amount_out: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct SwapBaseInIxData(pub SwapBaseInIxArgs);
impl From<SwapBaseInIxArgs> for SwapBaseInIxData {
    fn from(args: SwapBaseInIxArgs) -> Self {
        Self(args)
    }
}
impl SwapBaseInIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm_buf = [0u8; 1];
        reader.read_exact(&mut maybe_discm_buf)?;
        let maybe_discm = maybe_discm_buf[0];
        if maybe_discm != SWAP_BASE_IN_IX_DISCM {
            return Err(
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!(
                        "discm does not match. Expected: {:?}. Received: {:?}",
                        SWAP_BASE_IN_IX_DISCM, maybe_discm
                    ),
                ),
            );
        }
        Ok(Self(SwapBaseInIxArgs::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&[SWAP_BASE_IN_IX_DISCM])?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn swap_base_in_ix_with_program_id(
    program_id: Pubkey,
    keys: SwapBaseInKeys,
    args: SwapBaseInIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; SWAP_BASE_IN_IX_ACCOUNTS_LEN] = keys.into();
    let data: SwapBaseInIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn swap_base_in_ix(
    keys: SwapBaseInKeys,
    args: SwapBaseInIxArgs,
) -> std::io::Result<Instruction> {
    swap_base_in_ix_with_program_id(crate::ID, keys, args)
}
pub fn swap_base_in_invoke_with_program_id(
    program_id: Pubkey,
    accounts: SwapBaseInAccounts<'_, '_>,
    args: SwapBaseInIxArgs,
) -> ProgramResult {
    let keys: SwapBaseInKeys = accounts.into();
    let ix = swap_base_in_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn swap_base_in_invoke(
    accounts: SwapBaseInAccounts<'_, '_>,
    args: SwapBaseInIxArgs,
) -> ProgramResult {
    swap_base_in_invoke_with_program_id(crate::ID, accounts, args)
}
pub fn swap_base_in_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: SwapBaseInAccounts<'_, '_>,
    args: SwapBaseInIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: SwapBaseInKeys = accounts.into();
    let ix = swap_base_in_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn swap_base_in_invoke_signed(
    accounts: SwapBaseInAccounts<'_, '_>,
    args: SwapBaseInIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    swap_base_in_invoke_signed_with_program_id(crate::ID, accounts, args, seeds)
}
pub fn swap_base_in_verify_account_keys(
    accounts: SwapBaseInAccounts<'_, '_>,
    keys: SwapBaseInKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (accounts.token_program.key, &keys.token_program),
        (accounts.amm.key, &keys.amm),
        (accounts.amm_authority.key, &keys.amm_authority),
        (accounts.amm_open_orders.key, &keys.amm_open_orders),
        (accounts.amm_target_orders.key, &keys.amm_target_orders),
        (accounts.pool_coin_token_account.key, &keys.pool_coin_token_account),
        (accounts.pool_pc_token_account.key, &keys.pool_pc_token_account),
        (accounts.serum_program.key, &keys.serum_program),
        (accounts.serum_market.key, &keys.serum_market),
        (accounts.serum_bids.key, &keys.serum_bids),
        (accounts.serum_asks.key, &keys.serum_asks),
        (accounts.serum_event_queue.key, &keys.serum_event_queue),
        (accounts.serum_coin_vault_account.key, &keys.serum_coin_vault_account),
        (accounts.serum_pc_vault_account.key, &keys.serum_pc_vault_account),
        (accounts.serum_vault_signer.key, &keys.serum_vault_signer),
        (accounts.user_source_token_account.key, &keys.user_source_token_account),
        (
            accounts.user_destination_token_account.key,
            &keys.user_destination_token_account,
        ),
        (accounts.user_source_owner.key, &keys.user_source_owner),
    ] {
        if actual != expected {
            return Err((*actual, *expected));
        }
    }
    Ok(())
}
pub fn swap_base_in_verify_writable_privileges<'me, 'info>(
    accounts: SwapBaseInAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.amm,
        accounts.amm_open_orders,
        accounts.amm_target_orders,
        accounts.pool_coin_token_account,
        accounts.pool_pc_token_account,
        accounts.serum_market,
        accounts.serum_bids,
        accounts.serum_asks,
        accounts.serum_event_queue,
        accounts.serum_coin_vault_account,
        accounts.serum_pc_vault_account,
        accounts.user_source_token_account,
        accounts.user_destination_token_account,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn swap_base_in_verify_signer_privileges<'me, 'info>(
    accounts: SwapBaseInAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.user_source_owner] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn swap_base_in_verify_account_privileges<'me, 'info>(
    accounts: SwapBaseInAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    swap_base_in_verify_writable_privileges(accounts)?;
    swap_base_in_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const PRE_INITIALIZE_IX_ACCOUNTS_LEN: usize = 14;
#[derive(Copy, Clone, Debug)]
pub struct PreInitializeAccounts<'me, 'info> {
    pub token_program: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub rent: &'me AccountInfo<'info>,
    pub amm_target_orders: &'me AccountInfo<'info>,
    pub pool_withdraw_queue: &'me AccountInfo<'info>,
    pub amm_authority: &'me AccountInfo<'info>,
    pub lp_mint_address: &'me AccountInfo<'info>,
    pub coin_mint_address: &'me AccountInfo<'info>,
    pub pc_mint_address: &'me AccountInfo<'info>,
    pub pool_coin_token_account: &'me AccountInfo<'info>,
    pub pool_pc_token_account: &'me AccountInfo<'info>,
    pub pool_temp_lp_token_account: &'me AccountInfo<'info>,
    pub serum_market: &'me AccountInfo<'info>,
    pub user_wallet: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug)]
pub struct PreInitializeKeys {
    pub token_program: Pubkey,
    pub system_program: Pubkey,
    pub rent: Pubkey,
    pub amm_target_orders: Pubkey,
    pub pool_withdraw_queue: Pubkey,
    pub amm_authority: Pubkey,
    pub lp_mint_address: Pubkey,
    pub coin_mint_address: Pubkey,
    pub pc_mint_address: Pubkey,
    pub pool_coin_token_account: Pubkey,
    pub pool_pc_token_account: Pubkey,
    pub pool_temp_lp_token_account: Pubkey,
    pub serum_market: Pubkey,
    pub user_wallet: Pubkey,
}
impl From<PreInitializeAccounts<'_, '_>> for PreInitializeKeys {
    fn from(accounts: PreInitializeAccounts) -> Self {
        Self {
            token_program: *accounts.token_program.key,
            system_program: *accounts.system_program.key,
            rent: *accounts.rent.key,
            amm_target_orders: *accounts.amm_target_orders.key,
            pool_withdraw_queue: *accounts.pool_withdraw_queue.key,
            amm_authority: *accounts.amm_authority.key,
            lp_mint_address: *accounts.lp_mint_address.key,
            coin_mint_address: *accounts.coin_mint_address.key,
            pc_mint_address: *accounts.pc_mint_address.key,
            pool_coin_token_account: *accounts.pool_coin_token_account.key,
            pool_pc_token_account: *accounts.pool_pc_token_account.key,
            pool_temp_lp_token_account: *accounts.pool_temp_lp_token_account.key,
            serum_market: *accounts.serum_market.key,
            user_wallet: *accounts.user_wallet.key,
        }
    }
}
impl From<PreInitializeKeys> for [AccountMeta; PRE_INITIALIZE_IX_ACCOUNTS_LEN] {
    fn from(keys: PreInitializeKeys) -> Self {
        [
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
                pubkey: keys.amm_target_orders,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.pool_withdraw_queue,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.amm_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.lp_mint_address,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.coin_mint_address,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pc_mint_address,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pool_coin_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.pool_pc_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.pool_temp_lp_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.serum_market,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.user_wallet,
                is_signer: true,
                is_writable: true,
            },
        ]
    }
}
impl From<[Pubkey; PRE_INITIALIZE_IX_ACCOUNTS_LEN]> for PreInitializeKeys {
    fn from(pubkeys: [Pubkey; PRE_INITIALIZE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            token_program: pubkeys[0],
            system_program: pubkeys[1],
            rent: pubkeys[2],
            amm_target_orders: pubkeys[3],
            pool_withdraw_queue: pubkeys[4],
            amm_authority: pubkeys[5],
            lp_mint_address: pubkeys[6],
            coin_mint_address: pubkeys[7],
            pc_mint_address: pubkeys[8],
            pool_coin_token_account: pubkeys[9],
            pool_pc_token_account: pubkeys[10],
            pool_temp_lp_token_account: pubkeys[11],
            serum_market: pubkeys[12],
            user_wallet: pubkeys[13],
        }
    }
}
impl<'info> From<PreInitializeAccounts<'_, 'info>>
for [AccountInfo<'info>; PRE_INITIALIZE_IX_ACCOUNTS_LEN] {
    fn from(accounts: PreInitializeAccounts<'_, 'info>) -> Self {
        [
            accounts.token_program.clone(),
            accounts.system_program.clone(),
            accounts.rent.clone(),
            accounts.amm_target_orders.clone(),
            accounts.pool_withdraw_queue.clone(),
            accounts.amm_authority.clone(),
            accounts.lp_mint_address.clone(),
            accounts.coin_mint_address.clone(),
            accounts.pc_mint_address.clone(),
            accounts.pool_coin_token_account.clone(),
            accounts.pool_pc_token_account.clone(),
            accounts.pool_temp_lp_token_account.clone(),
            accounts.serum_market.clone(),
            accounts.user_wallet.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; PRE_INITIALIZE_IX_ACCOUNTS_LEN]>
for PreInitializeAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; PRE_INITIALIZE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            token_program: &arr[0],
            system_program: &arr[1],
            rent: &arr[2],
            amm_target_orders: &arr[3],
            pool_withdraw_queue: &arr[4],
            amm_authority: &arr[5],
            lp_mint_address: &arr[6],
            coin_mint_address: &arr[7],
            pc_mint_address: &arr[8],
            pool_coin_token_account: &arr[9],
            pool_pc_token_account: &arr[10],
            pool_temp_lp_token_account: &arr[11],
            serum_market: &arr[12],
            user_wallet: &arr[13],
        }
    }
}
pub const PRE_INITIALIZE_IX_DISCM: u8 = 10u8;
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct PreInitializeIxArgs {
    pub nonce: u8,
}
#[derive(Clone, Debug, PartialEq)]
pub struct PreInitializeIxData(pub PreInitializeIxArgs);
impl From<PreInitializeIxArgs> for PreInitializeIxData {
    fn from(args: PreInitializeIxArgs) -> Self {
        Self(args)
    }
}
impl PreInitializeIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm_buf = [0u8; 1];
        reader.read_exact(&mut maybe_discm_buf)?;
        let maybe_discm = maybe_discm_buf[0];
        if maybe_discm != PRE_INITIALIZE_IX_DISCM {
            return Err(
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!(
                        "discm does not match. Expected: {:?}. Received: {:?}",
                        PRE_INITIALIZE_IX_DISCM, maybe_discm
                    ),
                ),
            );
        }
        Ok(Self(PreInitializeIxArgs::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&[PRE_INITIALIZE_IX_DISCM])?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn pre_initialize_ix_with_program_id(
    program_id: Pubkey,
    keys: PreInitializeKeys,
    args: PreInitializeIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; PRE_INITIALIZE_IX_ACCOUNTS_LEN] = keys.into();
    let data: PreInitializeIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn pre_initialize_ix(
    keys: PreInitializeKeys,
    args: PreInitializeIxArgs,
) -> std::io::Result<Instruction> {
    pre_initialize_ix_with_program_id(crate::ID, keys, args)
}
pub fn pre_initialize_invoke_with_program_id(
    program_id: Pubkey,
    accounts: PreInitializeAccounts<'_, '_>,
    args: PreInitializeIxArgs,
) -> ProgramResult {
    let keys: PreInitializeKeys = accounts.into();
    let ix = pre_initialize_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn pre_initialize_invoke(
    accounts: PreInitializeAccounts<'_, '_>,
    args: PreInitializeIxArgs,
) -> ProgramResult {
    pre_initialize_invoke_with_program_id(crate::ID, accounts, args)
}
pub fn pre_initialize_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: PreInitializeAccounts<'_, '_>,
    args: PreInitializeIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: PreInitializeKeys = accounts.into();
    let ix = pre_initialize_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn pre_initialize_invoke_signed(
    accounts: PreInitializeAccounts<'_, '_>,
    args: PreInitializeIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    pre_initialize_invoke_signed_with_program_id(crate::ID, accounts, args, seeds)
}
pub fn pre_initialize_verify_account_keys(
    accounts: PreInitializeAccounts<'_, '_>,
    keys: PreInitializeKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (accounts.token_program.key, &keys.token_program),
        (accounts.system_program.key, &keys.system_program),
        (accounts.rent.key, &keys.rent),
        (accounts.amm_target_orders.key, &keys.amm_target_orders),
        (accounts.pool_withdraw_queue.key, &keys.pool_withdraw_queue),
        (accounts.amm_authority.key, &keys.amm_authority),
        (accounts.lp_mint_address.key, &keys.lp_mint_address),
        (accounts.coin_mint_address.key, &keys.coin_mint_address),
        (accounts.pc_mint_address.key, &keys.pc_mint_address),
        (accounts.pool_coin_token_account.key, &keys.pool_coin_token_account),
        (accounts.pool_pc_token_account.key, &keys.pool_pc_token_account),
        (accounts.pool_temp_lp_token_account.key, &keys.pool_temp_lp_token_account),
        (accounts.serum_market.key, &keys.serum_market),
        (accounts.user_wallet.key, &keys.user_wallet),
    ] {
        if actual != expected {
            return Err((*actual, *expected));
        }
    }
    Ok(())
}
pub fn pre_initialize_verify_writable_privileges<'me, 'info>(
    accounts: PreInitializeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.amm_target_orders,
        accounts.pool_withdraw_queue,
        accounts.lp_mint_address,
        accounts.pool_coin_token_account,
        accounts.pool_pc_token_account,
        accounts.pool_temp_lp_token_account,
        accounts.user_wallet,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn pre_initialize_verify_signer_privileges<'me, 'info>(
    accounts: PreInitializeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.user_wallet] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn pre_initialize_verify_account_privileges<'me, 'info>(
    accounts: PreInitializeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    pre_initialize_verify_writable_privileges(accounts)?;
    pre_initialize_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const SWAP_BASE_OUT_IX_ACCOUNTS_LEN: usize = 18;
#[derive(Copy, Clone, Debug)]
pub struct SwapBaseOutAccounts<'me, 'info> {
    pub token_program: &'me AccountInfo<'info>,
    pub amm: &'me AccountInfo<'info>,
    pub amm_authority: &'me AccountInfo<'info>,
    pub amm_open_orders: &'me AccountInfo<'info>,
    pub amm_target_orders: &'me AccountInfo<'info>,
    pub pool_coin_token_account: &'me AccountInfo<'info>,
    pub pool_pc_token_account: &'me AccountInfo<'info>,
    pub serum_program: &'me AccountInfo<'info>,
    pub serum_market: &'me AccountInfo<'info>,
    pub serum_bids: &'me AccountInfo<'info>,
    pub serum_asks: &'me AccountInfo<'info>,
    pub serum_event_queue: &'me AccountInfo<'info>,
    pub serum_coin_vault_account: &'me AccountInfo<'info>,
    pub serum_pc_vault_account: &'me AccountInfo<'info>,
    pub serum_vault_signer: &'me AccountInfo<'info>,
    pub user_source_token_account: &'me AccountInfo<'info>,
    pub user_destination_token_account: &'me AccountInfo<'info>,
    pub user_source_owner: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug)]
pub struct SwapBaseOutKeys {
    pub token_program: Pubkey,
    pub amm: Pubkey,
    pub amm_authority: Pubkey,
    pub amm_open_orders: Pubkey,
    pub amm_target_orders: Pubkey,
    pub pool_coin_token_account: Pubkey,
    pub pool_pc_token_account: Pubkey,
    pub serum_program: Pubkey,
    pub serum_market: Pubkey,
    pub serum_bids: Pubkey,
    pub serum_asks: Pubkey,
    pub serum_event_queue: Pubkey,
    pub serum_coin_vault_account: Pubkey,
    pub serum_pc_vault_account: Pubkey,
    pub serum_vault_signer: Pubkey,
    pub user_source_token_account: Pubkey,
    pub user_destination_token_account: Pubkey,
    pub user_source_owner: Pubkey,
}
impl From<SwapBaseOutAccounts<'_, '_>> for SwapBaseOutKeys {
    fn from(accounts: SwapBaseOutAccounts) -> Self {
        Self {
            token_program: *accounts.token_program.key,
            amm: *accounts.amm.key,
            amm_authority: *accounts.amm_authority.key,
            amm_open_orders: *accounts.amm_open_orders.key,
            amm_target_orders: *accounts.amm_target_orders.key,
            pool_coin_token_account: *accounts.pool_coin_token_account.key,
            pool_pc_token_account: *accounts.pool_pc_token_account.key,
            serum_program: *accounts.serum_program.key,
            serum_market: *accounts.serum_market.key,
            serum_bids: *accounts.serum_bids.key,
            serum_asks: *accounts.serum_asks.key,
            serum_event_queue: *accounts.serum_event_queue.key,
            serum_coin_vault_account: *accounts.serum_coin_vault_account.key,
            serum_pc_vault_account: *accounts.serum_pc_vault_account.key,
            serum_vault_signer: *accounts.serum_vault_signer.key,
            user_source_token_account: *accounts.user_source_token_account.key,
            user_destination_token_account: *accounts.user_destination_token_account.key,
            user_source_owner: *accounts.user_source_owner.key,
        }
    }
}
impl From<SwapBaseOutKeys> for [AccountMeta; SWAP_BASE_OUT_IX_ACCOUNTS_LEN] {
    fn from(keys: SwapBaseOutKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.amm,
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
                pubkey: keys.amm_target_orders,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.pool_coin_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.pool_pc_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.serum_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.serum_market,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.serum_bids,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.serum_asks,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.serum_event_queue,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.serum_coin_vault_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.serum_pc_vault_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.serum_vault_signer,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.user_source_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_destination_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_source_owner,
                is_signer: true,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; SWAP_BASE_OUT_IX_ACCOUNTS_LEN]> for SwapBaseOutKeys {
    fn from(pubkeys: [Pubkey; SWAP_BASE_OUT_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            token_program: pubkeys[0],
            amm: pubkeys[1],
            amm_authority: pubkeys[2],
            amm_open_orders: pubkeys[3],
            amm_target_orders: pubkeys[4],
            pool_coin_token_account: pubkeys[5],
            pool_pc_token_account: pubkeys[6],
            serum_program: pubkeys[7],
            serum_market: pubkeys[8],
            serum_bids: pubkeys[9],
            serum_asks: pubkeys[10],
            serum_event_queue: pubkeys[11],
            serum_coin_vault_account: pubkeys[12],
            serum_pc_vault_account: pubkeys[13],
            serum_vault_signer: pubkeys[14],
            user_source_token_account: pubkeys[15],
            user_destination_token_account: pubkeys[16],
            user_source_owner: pubkeys[17],
        }
    }
}
impl<'info> From<SwapBaseOutAccounts<'_, 'info>>
for [AccountInfo<'info>; SWAP_BASE_OUT_IX_ACCOUNTS_LEN] {
    fn from(accounts: SwapBaseOutAccounts<'_, 'info>) -> Self {
        [
            accounts.token_program.clone(),
            accounts.amm.clone(),
            accounts.amm_authority.clone(),
            accounts.amm_open_orders.clone(),
            accounts.amm_target_orders.clone(),
            accounts.pool_coin_token_account.clone(),
            accounts.pool_pc_token_account.clone(),
            accounts.serum_program.clone(),
            accounts.serum_market.clone(),
            accounts.serum_bids.clone(),
            accounts.serum_asks.clone(),
            accounts.serum_event_queue.clone(),
            accounts.serum_coin_vault_account.clone(),
            accounts.serum_pc_vault_account.clone(),
            accounts.serum_vault_signer.clone(),
            accounts.user_source_token_account.clone(),
            accounts.user_destination_token_account.clone(),
            accounts.user_source_owner.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; SWAP_BASE_OUT_IX_ACCOUNTS_LEN]>
for SwapBaseOutAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; SWAP_BASE_OUT_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            token_program: &arr[0],
            amm: &arr[1],
            amm_authority: &arr[2],
            amm_open_orders: &arr[3],
            amm_target_orders: &arr[4],
            pool_coin_token_account: &arr[5],
            pool_pc_token_account: &arr[6],
            serum_program: &arr[7],
            serum_market: &arr[8],
            serum_bids: &arr[9],
            serum_asks: &arr[10],
            serum_event_queue: &arr[11],
            serum_coin_vault_account: &arr[12],
            serum_pc_vault_account: &arr[13],
            serum_vault_signer: &arr[14],
            user_source_token_account: &arr[15],
            user_destination_token_account: &arr[16],
            user_source_owner: &arr[17],
        }
    }
}
pub const SWAP_BASE_OUT_IX_DISCM: u8 = 11u8;
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq,Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SwapBaseOutIxArgs {
    pub max_amount_in: u64,
    pub amount_out: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct SwapBaseOutIxData(pub SwapBaseOutIxArgs);
impl From<SwapBaseOutIxArgs> for SwapBaseOutIxData {
    fn from(args: SwapBaseOutIxArgs) -> Self {
        Self(args)
    }
}
impl SwapBaseOutIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm_buf = [0u8; 1];
        reader.read_exact(&mut maybe_discm_buf)?;
        let maybe_discm = maybe_discm_buf[0];
        if maybe_discm != SWAP_BASE_OUT_IX_DISCM {
            return Err(
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!(
                        "discm does not match. Expected: {:?}. Received: {:?}",
                        SWAP_BASE_OUT_IX_DISCM, maybe_discm
                    ),
                ),
            );
        }
        Ok(Self(SwapBaseOutIxArgs::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&[SWAP_BASE_OUT_IX_DISCM])?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn swap_base_out_ix_with_program_id(
    program_id: Pubkey,
    keys: SwapBaseOutKeys,
    args: SwapBaseOutIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; SWAP_BASE_OUT_IX_ACCOUNTS_LEN] = keys.into();
    let data: SwapBaseOutIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn swap_base_out_ix(
    keys: SwapBaseOutKeys,
    args: SwapBaseOutIxArgs,
) -> std::io::Result<Instruction> {
    swap_base_out_ix_with_program_id(crate::ID, keys, args)
}
pub fn swap_base_out_invoke_with_program_id(
    program_id: Pubkey,
    accounts: SwapBaseOutAccounts<'_, '_>,
    args: SwapBaseOutIxArgs,
) -> ProgramResult {
    let keys: SwapBaseOutKeys = accounts.into();
    let ix = swap_base_out_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn swap_base_out_invoke(
    accounts: SwapBaseOutAccounts<'_, '_>,
    args: SwapBaseOutIxArgs,
) -> ProgramResult {
    swap_base_out_invoke_with_program_id(crate::ID, accounts, args)
}
pub fn swap_base_out_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: SwapBaseOutAccounts<'_, '_>,
    args: SwapBaseOutIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: SwapBaseOutKeys = accounts.into();
    let ix = swap_base_out_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn swap_base_out_invoke_signed(
    accounts: SwapBaseOutAccounts<'_, '_>,
    args: SwapBaseOutIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    swap_base_out_invoke_signed_with_program_id(crate::ID, accounts, args, seeds)
}
pub fn swap_base_out_verify_account_keys(
    accounts: SwapBaseOutAccounts<'_, '_>,
    keys: SwapBaseOutKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (accounts.token_program.key, &keys.token_program),
        (accounts.amm.key, &keys.amm),
        (accounts.amm_authority.key, &keys.amm_authority),
        (accounts.amm_open_orders.key, &keys.amm_open_orders),
        (accounts.amm_target_orders.key, &keys.amm_target_orders),
        (accounts.pool_coin_token_account.key, &keys.pool_coin_token_account),
        (accounts.pool_pc_token_account.key, &keys.pool_pc_token_account),
        (accounts.serum_program.key, &keys.serum_program),
        (accounts.serum_market.key, &keys.serum_market),
        (accounts.serum_bids.key, &keys.serum_bids),
        (accounts.serum_asks.key, &keys.serum_asks),
        (accounts.serum_event_queue.key, &keys.serum_event_queue),
        (accounts.serum_coin_vault_account.key, &keys.serum_coin_vault_account),
        (accounts.serum_pc_vault_account.key, &keys.serum_pc_vault_account),
        (accounts.serum_vault_signer.key, &keys.serum_vault_signer),
        (accounts.user_source_token_account.key, &keys.user_source_token_account),
        (
            accounts.user_destination_token_account.key,
            &keys.user_destination_token_account,
        ),
        (accounts.user_source_owner.key, &keys.user_source_owner),
    ] {
        if actual != expected {
            return Err((*actual, *expected));
        }
    }
    Ok(())
}
pub fn swap_base_out_verify_writable_privileges<'me, 'info>(
    accounts: SwapBaseOutAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.amm,
        accounts.amm_open_orders,
        accounts.amm_target_orders,
        accounts.pool_coin_token_account,
        accounts.pool_pc_token_account,
        accounts.serum_market,
        accounts.serum_bids,
        accounts.serum_asks,
        accounts.serum_event_queue,
        accounts.serum_coin_vault_account,
        accounts.serum_pc_vault_account,
        accounts.user_source_token_account,
        accounts.user_destination_token_account,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn swap_base_out_verify_signer_privileges<'me, 'info>(
    accounts: SwapBaseOutAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.user_source_owner] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn swap_base_out_verify_account_privileges<'me, 'info>(
    accounts: SwapBaseOutAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    swap_base_out_verify_writable_privileges(accounts)?;
    swap_base_out_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const SIMULATE_INFO_IX_ACCOUNTS_LEN: usize = 8;
#[derive(Copy, Clone, Debug)]
pub struct SimulateInfoAccounts<'me, 'info> {
    pub amm: &'me AccountInfo<'info>,
    pub amm_authority: &'me AccountInfo<'info>,
    pub amm_open_orders: &'me AccountInfo<'info>,
    pub pool_coin_token_account: &'me AccountInfo<'info>,
    pub pool_pc_token_account: &'me AccountInfo<'info>,
    pub lp_mint_address: &'me AccountInfo<'info>,
    pub serum_market: &'me AccountInfo<'info>,
    pub serum_event_queue: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug)]
pub struct SimulateInfoKeys {
    pub amm: Pubkey,
    pub amm_authority: Pubkey,
    pub amm_open_orders: Pubkey,
    pub pool_coin_token_account: Pubkey,
    pub pool_pc_token_account: Pubkey,
    pub lp_mint_address: Pubkey,
    pub serum_market: Pubkey,
    pub serum_event_queue: Pubkey,
}
impl From<SimulateInfoAccounts<'_, '_>> for SimulateInfoKeys {
    fn from(accounts: SimulateInfoAccounts) -> Self {
        Self {
            amm: *accounts.amm.key,
            amm_authority: *accounts.amm_authority.key,
            amm_open_orders: *accounts.amm_open_orders.key,
            pool_coin_token_account: *accounts.pool_coin_token_account.key,
            pool_pc_token_account: *accounts.pool_pc_token_account.key,
            lp_mint_address: *accounts.lp_mint_address.key,
            serum_market: *accounts.serum_market.key,
            serum_event_queue: *accounts.serum_event_queue.key,
        }
    }
}
impl From<SimulateInfoKeys> for [AccountMeta; SIMULATE_INFO_IX_ACCOUNTS_LEN] {
    fn from(keys: SimulateInfoKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.amm,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.amm_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.amm_open_orders,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pool_coin_token_account,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pool_pc_token_account,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.lp_mint_address,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.serum_market,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.serum_event_queue,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; SIMULATE_INFO_IX_ACCOUNTS_LEN]> for SimulateInfoKeys {
    fn from(pubkeys: [Pubkey; SIMULATE_INFO_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            amm: pubkeys[0],
            amm_authority: pubkeys[1],
            amm_open_orders: pubkeys[2],
            pool_coin_token_account: pubkeys[3],
            pool_pc_token_account: pubkeys[4],
            lp_mint_address: pubkeys[5],
            serum_market: pubkeys[6],
            serum_event_queue: pubkeys[7],
        }
    }
}
impl<'info> From<SimulateInfoAccounts<'_, 'info>>
for [AccountInfo<'info>; SIMULATE_INFO_IX_ACCOUNTS_LEN] {
    fn from(accounts: SimulateInfoAccounts<'_, 'info>) -> Self {
        [
            accounts.amm.clone(),
            accounts.amm_authority.clone(),
            accounts.amm_open_orders.clone(),
            accounts.pool_coin_token_account.clone(),
            accounts.pool_pc_token_account.clone(),
            accounts.lp_mint_address.clone(),
            accounts.serum_market.clone(),
            accounts.serum_event_queue.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; SIMULATE_INFO_IX_ACCOUNTS_LEN]>
for SimulateInfoAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; SIMULATE_INFO_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            amm: &arr[0],
            amm_authority: &arr[1],
            amm_open_orders: &arr[2],
            pool_coin_token_account: &arr[3],
            pool_pc_token_account: &arr[4],
            lp_mint_address: &arr[5],
            serum_market: &arr[6],
            serum_event_queue: &arr[7],
        }
    }
}
pub const SIMULATE_INFO_IX_DISCM: u8 = 12u8;
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq,Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SimulateInfoIxArgs {
    pub param: u8,
    pub swap_base_in_value: Option<SwapInstructionBaseIn>,
    pub swap_base_out_value: Option<SwapInstructionBaseOut>,
}
#[derive(Clone, Debug, PartialEq)]
pub struct SimulateInfoIxData(pub SimulateInfoIxArgs);
impl From<SimulateInfoIxArgs> for SimulateInfoIxData {
    fn from(args: SimulateInfoIxArgs) -> Self {
        Self(args)
    }
}
impl SimulateInfoIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm_buf = [0u8; 1];
        reader.read_exact(&mut maybe_discm_buf)?;
        let maybe_discm = maybe_discm_buf[0];
        if maybe_discm != SIMULATE_INFO_IX_DISCM {
            return Err(
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!(
                        "discm does not match. Expected: {:?}. Received: {:?}",
                        SIMULATE_INFO_IX_DISCM, maybe_discm
                    ),
                ),
            );
        }
        Ok(Self(SimulateInfoIxArgs::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&[SIMULATE_INFO_IX_DISCM])?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn simulate_info_ix_with_program_id(
    program_id: Pubkey,
    keys: SimulateInfoKeys,
    args: SimulateInfoIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; SIMULATE_INFO_IX_ACCOUNTS_LEN] = keys.into();
    let data: SimulateInfoIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn simulate_info_ix(
    keys: SimulateInfoKeys,
    args: SimulateInfoIxArgs,
) -> std::io::Result<Instruction> {
    simulate_info_ix_with_program_id(crate::ID, keys, args)
}
pub fn simulate_info_invoke_with_program_id(
    program_id: Pubkey,
    accounts: SimulateInfoAccounts<'_, '_>,
    args: SimulateInfoIxArgs,
) -> ProgramResult {
    let keys: SimulateInfoKeys = accounts.into();
    let ix = simulate_info_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn simulate_info_invoke(
    accounts: SimulateInfoAccounts<'_, '_>,
    args: SimulateInfoIxArgs,
) -> ProgramResult {
    simulate_info_invoke_with_program_id(crate::ID, accounts, args)
}
pub fn simulate_info_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: SimulateInfoAccounts<'_, '_>,
    args: SimulateInfoIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: SimulateInfoKeys = accounts.into();
    let ix = simulate_info_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn simulate_info_invoke_signed(
    accounts: SimulateInfoAccounts<'_, '_>,
    args: SimulateInfoIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    simulate_info_invoke_signed_with_program_id(crate::ID, accounts, args, seeds)
}
pub fn simulate_info_verify_account_keys(
    accounts: SimulateInfoAccounts<'_, '_>,
    keys: SimulateInfoKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (accounts.amm.key, &keys.amm),
        (accounts.amm_authority.key, &keys.amm_authority),
        (accounts.amm_open_orders.key, &keys.amm_open_orders),
        (accounts.pool_coin_token_account.key, &keys.pool_coin_token_account),
        (accounts.pool_pc_token_account.key, &keys.pool_pc_token_account),
        (accounts.lp_mint_address.key, &keys.lp_mint_address),
        (accounts.serum_market.key, &keys.serum_market),
        (accounts.serum_event_queue.key, &keys.serum_event_queue),
    ] {
        if actual != expected {
            return Err((*actual, *expected));
        }
    }
    Ok(())
}
pub const ADMIN_CANCEL_ORDERS_IX_ACCOUNTS_LEN: usize = 17;
#[derive(Copy, Clone, Debug)]
pub struct AdminCancelOrdersAccounts<'me, 'info> {
    pub token_program: &'me AccountInfo<'info>,
    pub amm: &'me AccountInfo<'info>,
    pub amm_authority: &'me AccountInfo<'info>,
    pub amm_open_orders: &'me AccountInfo<'info>,
    pub amm_target_orders: &'me AccountInfo<'info>,
    pub pool_coin_token_account: &'me AccountInfo<'info>,
    pub pool_pc_token_account: &'me AccountInfo<'info>,
    pub amm_owner_account: &'me AccountInfo<'info>,
    pub amm_config: &'me AccountInfo<'info>,
    pub serum_program: &'me AccountInfo<'info>,
    pub serum_market: &'me AccountInfo<'info>,
    pub serum_coin_vault_account: &'me AccountInfo<'info>,
    pub serum_pc_vault_account: &'me AccountInfo<'info>,
    pub serum_vault_signer: &'me AccountInfo<'info>,
    pub serum_event_q: &'me AccountInfo<'info>,
    pub serum_bids: &'me AccountInfo<'info>,
    pub serum_asks: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug)]
pub struct AdminCancelOrdersKeys {
    pub token_program: Pubkey,
    pub amm: Pubkey,
    pub amm_authority: Pubkey,
    pub amm_open_orders: Pubkey,
    pub amm_target_orders: Pubkey,
    pub pool_coin_token_account: Pubkey,
    pub pool_pc_token_account: Pubkey,
    pub amm_owner_account: Pubkey,
    pub amm_config: Pubkey,
    pub serum_program: Pubkey,
    pub serum_market: Pubkey,
    pub serum_coin_vault_account: Pubkey,
    pub serum_pc_vault_account: Pubkey,
    pub serum_vault_signer: Pubkey,
    pub serum_event_q: Pubkey,
    pub serum_bids: Pubkey,
    pub serum_asks: Pubkey,
}
impl From<AdminCancelOrdersAccounts<'_, '_>> for AdminCancelOrdersKeys {
    fn from(accounts: AdminCancelOrdersAccounts) -> Self {
        Self {
            token_program: *accounts.token_program.key,
            amm: *accounts.amm.key,
            amm_authority: *accounts.amm_authority.key,
            amm_open_orders: *accounts.amm_open_orders.key,
            amm_target_orders: *accounts.amm_target_orders.key,
            pool_coin_token_account: *accounts.pool_coin_token_account.key,
            pool_pc_token_account: *accounts.pool_pc_token_account.key,
            amm_owner_account: *accounts.amm_owner_account.key,
            amm_config: *accounts.amm_config.key,
            serum_program: *accounts.serum_program.key,
            serum_market: *accounts.serum_market.key,
            serum_coin_vault_account: *accounts.serum_coin_vault_account.key,
            serum_pc_vault_account: *accounts.serum_pc_vault_account.key,
            serum_vault_signer: *accounts.serum_vault_signer.key,
            serum_event_q: *accounts.serum_event_q.key,
            serum_bids: *accounts.serum_bids.key,
            serum_asks: *accounts.serum_asks.key,
        }
    }
}
impl From<AdminCancelOrdersKeys> for [AccountMeta; ADMIN_CANCEL_ORDERS_IX_ACCOUNTS_LEN] {
    fn from(keys: AdminCancelOrdersKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.amm,
                is_signer: false,
                is_writable: false,
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
                pubkey: keys.amm_target_orders,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.pool_coin_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.pool_pc_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.amm_owner_account,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.amm_config,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.serum_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.serum_market,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.serum_coin_vault_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.serum_pc_vault_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.serum_vault_signer,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.serum_event_q,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.serum_bids,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.serum_asks,
                is_signer: false,
                is_writable: true,
            },
        ]
    }
}
impl From<[Pubkey; ADMIN_CANCEL_ORDERS_IX_ACCOUNTS_LEN]> for AdminCancelOrdersKeys {
    fn from(pubkeys: [Pubkey; ADMIN_CANCEL_ORDERS_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            token_program: pubkeys[0],
            amm: pubkeys[1],
            amm_authority: pubkeys[2],
            amm_open_orders: pubkeys[3],
            amm_target_orders: pubkeys[4],
            pool_coin_token_account: pubkeys[5],
            pool_pc_token_account: pubkeys[6],
            amm_owner_account: pubkeys[7],
            amm_config: pubkeys[8],
            serum_program: pubkeys[9],
            serum_market: pubkeys[10],
            serum_coin_vault_account: pubkeys[11],
            serum_pc_vault_account: pubkeys[12],
            serum_vault_signer: pubkeys[13],
            serum_event_q: pubkeys[14],
            serum_bids: pubkeys[15],
            serum_asks: pubkeys[16],
        }
    }
}
impl<'info> From<AdminCancelOrdersAccounts<'_, 'info>>
for [AccountInfo<'info>; ADMIN_CANCEL_ORDERS_IX_ACCOUNTS_LEN] {
    fn from(accounts: AdminCancelOrdersAccounts<'_, 'info>) -> Self {
        [
            accounts.token_program.clone(),
            accounts.amm.clone(),
            accounts.amm_authority.clone(),
            accounts.amm_open_orders.clone(),
            accounts.amm_target_orders.clone(),
            accounts.pool_coin_token_account.clone(),
            accounts.pool_pc_token_account.clone(),
            accounts.amm_owner_account.clone(),
            accounts.amm_config.clone(),
            accounts.serum_program.clone(),
            accounts.serum_market.clone(),
            accounts.serum_coin_vault_account.clone(),
            accounts.serum_pc_vault_account.clone(),
            accounts.serum_vault_signer.clone(),
            accounts.serum_event_q.clone(),
            accounts.serum_bids.clone(),
            accounts.serum_asks.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; ADMIN_CANCEL_ORDERS_IX_ACCOUNTS_LEN]>
for AdminCancelOrdersAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; ADMIN_CANCEL_ORDERS_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            token_program: &arr[0],
            amm: &arr[1],
            amm_authority: &arr[2],
            amm_open_orders: &arr[3],
            amm_target_orders: &arr[4],
            pool_coin_token_account: &arr[5],
            pool_pc_token_account: &arr[6],
            amm_owner_account: &arr[7],
            amm_config: &arr[8],
            serum_program: &arr[9],
            serum_market: &arr[10],
            serum_coin_vault_account: &arr[11],
            serum_pc_vault_account: &arr[12],
            serum_vault_signer: &arr[13],
            serum_event_q: &arr[14],
            serum_bids: &arr[15],
            serum_asks: &arr[16],
        }
    }
}
pub const ADMIN_CANCEL_ORDERS_IX_DISCM: u8 = 13u8;
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct AdminCancelOrdersIxArgs {
    pub limit: u16,
}
#[derive(Clone, Debug, PartialEq)]
pub struct AdminCancelOrdersIxData(pub AdminCancelOrdersIxArgs);
impl From<AdminCancelOrdersIxArgs> for AdminCancelOrdersIxData {
    fn from(args: AdminCancelOrdersIxArgs) -> Self {
        Self(args)
    }
}
impl AdminCancelOrdersIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm_buf = [0u8; 1];
        reader.read_exact(&mut maybe_discm_buf)?;
        let maybe_discm = maybe_discm_buf[0];
        if maybe_discm != ADMIN_CANCEL_ORDERS_IX_DISCM {
            return Err(
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!(
                        "discm does not match. Expected: {:?}. Received: {:?}",
                        ADMIN_CANCEL_ORDERS_IX_DISCM, maybe_discm
                    ),
                ),
            );
        }
        Ok(Self(AdminCancelOrdersIxArgs::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&[ADMIN_CANCEL_ORDERS_IX_DISCM])?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn admin_cancel_orders_ix_with_program_id(
    program_id: Pubkey,
    keys: AdminCancelOrdersKeys,
    args: AdminCancelOrdersIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; ADMIN_CANCEL_ORDERS_IX_ACCOUNTS_LEN] = keys.into();
    let data: AdminCancelOrdersIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn admin_cancel_orders_ix(
    keys: AdminCancelOrdersKeys,
    args: AdminCancelOrdersIxArgs,
) -> std::io::Result<Instruction> {
    admin_cancel_orders_ix_with_program_id(crate::ID, keys, args)
}
pub fn admin_cancel_orders_invoke_with_program_id(
    program_id: Pubkey,
    accounts: AdminCancelOrdersAccounts<'_, '_>,
    args: AdminCancelOrdersIxArgs,
) -> ProgramResult {
    let keys: AdminCancelOrdersKeys = accounts.into();
    let ix = admin_cancel_orders_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn admin_cancel_orders_invoke(
    accounts: AdminCancelOrdersAccounts<'_, '_>,
    args: AdminCancelOrdersIxArgs,
) -> ProgramResult {
    admin_cancel_orders_invoke_with_program_id(crate::ID, accounts, args)
}
pub fn admin_cancel_orders_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: AdminCancelOrdersAccounts<'_, '_>,
    args: AdminCancelOrdersIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: AdminCancelOrdersKeys = accounts.into();
    let ix = admin_cancel_orders_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn admin_cancel_orders_invoke_signed(
    accounts: AdminCancelOrdersAccounts<'_, '_>,
    args: AdminCancelOrdersIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    admin_cancel_orders_invoke_signed_with_program_id(crate::ID, accounts, args, seeds)
}
pub fn admin_cancel_orders_verify_account_keys(
    accounts: AdminCancelOrdersAccounts<'_, '_>,
    keys: AdminCancelOrdersKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (accounts.token_program.key, &keys.token_program),
        (accounts.amm.key, &keys.amm),
        (accounts.amm_authority.key, &keys.amm_authority),
        (accounts.amm_open_orders.key, &keys.amm_open_orders),
        (accounts.amm_target_orders.key, &keys.amm_target_orders),
        (accounts.pool_coin_token_account.key, &keys.pool_coin_token_account),
        (accounts.pool_pc_token_account.key, &keys.pool_pc_token_account),
        (accounts.amm_owner_account.key, &keys.amm_owner_account),
        (accounts.amm_config.key, &keys.amm_config),
        (accounts.serum_program.key, &keys.serum_program),
        (accounts.serum_market.key, &keys.serum_market),
        (accounts.serum_coin_vault_account.key, &keys.serum_coin_vault_account),
        (accounts.serum_pc_vault_account.key, &keys.serum_pc_vault_account),
        (accounts.serum_vault_signer.key, &keys.serum_vault_signer),
        (accounts.serum_event_q.key, &keys.serum_event_q),
        (accounts.serum_bids.key, &keys.serum_bids),
        (accounts.serum_asks.key, &keys.serum_asks),
    ] {
        if actual != expected {
            return Err((*actual, *expected));
        }
    }
    Ok(())
}
pub fn admin_cancel_orders_verify_writable_privileges<'me, 'info>(
    accounts: AdminCancelOrdersAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.amm_open_orders,
        accounts.amm_target_orders,
        accounts.pool_coin_token_account,
        accounts.pool_pc_token_account,
        accounts.amm_config,
        accounts.serum_market,
        accounts.serum_coin_vault_account,
        accounts.serum_pc_vault_account,
        accounts.serum_event_q,
        accounts.serum_bids,
        accounts.serum_asks,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn admin_cancel_orders_verify_signer_privileges<'me, 'info>(
    accounts: AdminCancelOrdersAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.amm_owner_account] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn admin_cancel_orders_verify_account_privileges<'me, 'info>(
    accounts: AdminCancelOrdersAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    admin_cancel_orders_verify_writable_privileges(accounts)?;
    admin_cancel_orders_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const CREATE_CONFIG_ACCOUNT_IX_ACCOUNTS_LEN: usize = 5;
#[derive(Copy, Clone, Debug)]
pub struct CreateConfigAccountAccounts<'me, 'info> {
    pub admin: &'me AccountInfo<'info>,
    pub amm_config: &'me AccountInfo<'info>,
    pub owner: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub rent: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug)]
pub struct CreateConfigAccountKeys {
    pub admin: Pubkey,
    pub amm_config: Pubkey,
    pub owner: Pubkey,
    pub system_program: Pubkey,
    pub rent: Pubkey,
}
impl From<CreateConfigAccountAccounts<'_, '_>> for CreateConfigAccountKeys {
    fn from(accounts: CreateConfigAccountAccounts) -> Self {
        Self {
            admin: *accounts.admin.key,
            amm_config: *accounts.amm_config.key,
            owner: *accounts.owner.key,
            system_program: *accounts.system_program.key,
            rent: *accounts.rent.key,
        }
    }
}
impl From<CreateConfigAccountKeys>
for [AccountMeta; CREATE_CONFIG_ACCOUNT_IX_ACCOUNTS_LEN] {
    fn from(keys: CreateConfigAccountKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.admin,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.amm_config,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.owner,
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
impl From<[Pubkey; CREATE_CONFIG_ACCOUNT_IX_ACCOUNTS_LEN]> for CreateConfigAccountKeys {
    fn from(pubkeys: [Pubkey; CREATE_CONFIG_ACCOUNT_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            admin: pubkeys[0],
            amm_config: pubkeys[1],
            owner: pubkeys[2],
            system_program: pubkeys[3],
            rent: pubkeys[4],
        }
    }
}
impl<'info> From<CreateConfigAccountAccounts<'_, 'info>>
for [AccountInfo<'info>; CREATE_CONFIG_ACCOUNT_IX_ACCOUNTS_LEN] {
    fn from(accounts: CreateConfigAccountAccounts<'_, 'info>) -> Self {
        [
            accounts.admin.clone(),
            accounts.amm_config.clone(),
            accounts.owner.clone(),
            accounts.system_program.clone(),
            accounts.rent.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; CREATE_CONFIG_ACCOUNT_IX_ACCOUNTS_LEN]>
for CreateConfigAccountAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; CREATE_CONFIG_ACCOUNT_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            admin: &arr[0],
            amm_config: &arr[1],
            owner: &arr[2],
            system_program: &arr[3],
            rent: &arr[4],
        }
    }
}
pub const CREATE_CONFIG_ACCOUNT_IX_DISCM: u8 = 14u8;
#[derive(Clone, Debug, PartialEq)]
pub struct CreateConfigAccountIxData;
impl CreateConfigAccountIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm_buf = [0u8; 1];
        reader.read_exact(&mut maybe_discm_buf)?;
        let maybe_discm = maybe_discm_buf[0];
        if maybe_discm != CREATE_CONFIG_ACCOUNT_IX_DISCM {
            return Err(
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!(
                        "discm does not match. Expected: {:?}. Received: {:?}",
                        CREATE_CONFIG_ACCOUNT_IX_DISCM, maybe_discm
                    ),
                ),
            );
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&[CREATE_CONFIG_ACCOUNT_IX_DISCM])
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn create_config_account_ix_with_program_id(
    program_id: Pubkey,
    keys: CreateConfigAccountKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; CREATE_CONFIG_ACCOUNT_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: CreateConfigAccountIxData.try_to_vec()?,
    })
}
pub fn create_config_account_ix(
    keys: CreateConfigAccountKeys,
) -> std::io::Result<Instruction> {
    create_config_account_ix_with_program_id(crate::ID, keys)
}
pub fn create_config_account_invoke_with_program_id(
    program_id: Pubkey,
    accounts: CreateConfigAccountAccounts<'_, '_>,
) -> ProgramResult {
    let keys: CreateConfigAccountKeys = accounts.into();
    let ix = create_config_account_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn create_config_account_invoke(
    accounts: CreateConfigAccountAccounts<'_, '_>,
) -> ProgramResult {
    create_config_account_invoke_with_program_id(crate::ID, accounts)
}
pub fn create_config_account_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: CreateConfigAccountAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: CreateConfigAccountKeys = accounts.into();
    let ix = create_config_account_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn create_config_account_invoke_signed(
    accounts: CreateConfigAccountAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    create_config_account_invoke_signed_with_program_id(crate::ID, accounts, seeds)
}
pub fn create_config_account_verify_account_keys(
    accounts: CreateConfigAccountAccounts<'_, '_>,
    keys: CreateConfigAccountKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (accounts.admin.key, &keys.admin),
        (accounts.amm_config.key, &keys.amm_config),
        (accounts.owner.key, &keys.owner),
        (accounts.system_program.key, &keys.system_program),
        (accounts.rent.key, &keys.rent),
    ] {
        if actual != expected {
            return Err((*actual, *expected));
        }
    }
    Ok(())
}
pub fn create_config_account_verify_writable_privileges<'me, 'info>(
    accounts: CreateConfigAccountAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.admin, accounts.amm_config] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn create_config_account_verify_signer_privileges<'me, 'info>(
    accounts: CreateConfigAccountAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.admin] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn create_config_account_verify_account_privileges<'me, 'info>(
    accounts: CreateConfigAccountAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    create_config_account_verify_writable_privileges(accounts)?;
    create_config_account_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const UPDATE_CONFIG_ACCOUNT_IX_ACCOUNTS_LEN: usize = 2;
#[derive(Copy, Clone, Debug)]
pub struct UpdateConfigAccountAccounts<'me, 'info> {
    pub admin: &'me AccountInfo<'info>,
    pub amm_config: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug)]
pub struct UpdateConfigAccountKeys {
    pub admin: Pubkey,
    pub amm_config: Pubkey,
}
impl From<UpdateConfigAccountAccounts<'_, '_>> for UpdateConfigAccountKeys {
    fn from(accounts: UpdateConfigAccountAccounts) -> Self {
        Self {
            admin: *accounts.admin.key,
            amm_config: *accounts.amm_config.key,
        }
    }
}
impl From<UpdateConfigAccountKeys>
for [AccountMeta; UPDATE_CONFIG_ACCOUNT_IX_ACCOUNTS_LEN] {
    fn from(keys: UpdateConfigAccountKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.admin,
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
impl From<[Pubkey; UPDATE_CONFIG_ACCOUNT_IX_ACCOUNTS_LEN]> for UpdateConfigAccountKeys {
    fn from(pubkeys: [Pubkey; UPDATE_CONFIG_ACCOUNT_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            admin: pubkeys[0],
            amm_config: pubkeys[1],
        }
    }
}
impl<'info> From<UpdateConfigAccountAccounts<'_, 'info>>
for [AccountInfo<'info>; UPDATE_CONFIG_ACCOUNT_IX_ACCOUNTS_LEN] {
    fn from(accounts: UpdateConfigAccountAccounts<'_, 'info>) -> Self {
        [accounts.admin.clone(), accounts.amm_config.clone()]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; UPDATE_CONFIG_ACCOUNT_IX_ACCOUNTS_LEN]>
for UpdateConfigAccountAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; UPDATE_CONFIG_ACCOUNT_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            admin: &arr[0],
            amm_config: &arr[1],
        }
    }
}
pub const UPDATE_CONFIG_ACCOUNT_IX_DISCM: u8 = 15u8;
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq,Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct UpdateConfigAccountIxArgs {
    pub param: u8,
    pub owner: Pubkey,
}
#[derive(Clone, Debug, PartialEq)]
pub struct UpdateConfigAccountIxData(pub UpdateConfigAccountIxArgs);
impl From<UpdateConfigAccountIxArgs> for UpdateConfigAccountIxData {
    fn from(args: UpdateConfigAccountIxArgs) -> Self {
        Self(args)
    }
}
impl UpdateConfigAccountIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm_buf = [0u8; 1];
        reader.read_exact(&mut maybe_discm_buf)?;
        let maybe_discm = maybe_discm_buf[0];
        if maybe_discm != UPDATE_CONFIG_ACCOUNT_IX_DISCM {
            return Err(
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!(
                        "discm does not match. Expected: {:?}. Received: {:?}",
                        UPDATE_CONFIG_ACCOUNT_IX_DISCM, maybe_discm
                    ),
                ),
            );
        }
        Ok(Self(UpdateConfigAccountIxArgs::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&[UPDATE_CONFIG_ACCOUNT_IX_DISCM])?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn update_config_account_ix_with_program_id(
    program_id: Pubkey,
    keys: UpdateConfigAccountKeys,
    args: UpdateConfigAccountIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; UPDATE_CONFIG_ACCOUNT_IX_ACCOUNTS_LEN] = keys.into();
    let data: UpdateConfigAccountIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn update_config_account_ix(
    keys: UpdateConfigAccountKeys,
    args: UpdateConfigAccountIxArgs,
) -> std::io::Result<Instruction> {
    update_config_account_ix_with_program_id(crate::ID, keys, args)
}
pub fn update_config_account_invoke_with_program_id(
    program_id: Pubkey,
    accounts: UpdateConfigAccountAccounts<'_, '_>,
    args: UpdateConfigAccountIxArgs,
) -> ProgramResult {
    let keys: UpdateConfigAccountKeys = accounts.into();
    let ix = update_config_account_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn update_config_account_invoke(
    accounts: UpdateConfigAccountAccounts<'_, '_>,
    args: UpdateConfigAccountIxArgs,
) -> ProgramResult {
    update_config_account_invoke_with_program_id(crate::ID, accounts, args)
}
pub fn update_config_account_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: UpdateConfigAccountAccounts<'_, '_>,
    args: UpdateConfigAccountIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: UpdateConfigAccountKeys = accounts.into();
    let ix = update_config_account_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn update_config_account_invoke_signed(
    accounts: UpdateConfigAccountAccounts<'_, '_>,
    args: UpdateConfigAccountIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    update_config_account_invoke_signed_with_program_id(crate::ID, accounts, args, seeds)
}
pub fn update_config_account_verify_account_keys(
    accounts: UpdateConfigAccountAccounts<'_, '_>,
    keys: UpdateConfigAccountKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (accounts.admin.key, &keys.admin),
        (accounts.amm_config.key, &keys.amm_config),
    ] {
        if actual != expected {
            return Err((*actual, *expected));
        }
    }
    Ok(())
}
pub fn update_config_account_verify_writable_privileges<'me, 'info>(
    accounts: UpdateConfigAccountAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.amm_config] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn update_config_account_verify_signer_privileges<'me, 'info>(
    accounts: UpdateConfigAccountAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.admin] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn update_config_account_verify_account_privileges<'me, 'info>(
    accounts: UpdateConfigAccountAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    update_config_account_verify_writable_privileges(accounts)?;
    update_config_account_verify_signer_privileges(accounts)?;
    Ok(())
}
