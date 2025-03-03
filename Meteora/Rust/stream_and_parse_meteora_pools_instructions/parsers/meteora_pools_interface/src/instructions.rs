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
use std::io::Read;
use inflector::Inflector;
use strum_macros::{Display, EnumString};

#[derive(Clone, Debug, PartialEq, EnumString, Display)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum MeteoraPoolsProgramIx {
    InitializePermissionedPool(InitializePermissionedPoolIxArgs),
    InitializePermissionlessPool(InitializePermissionlessPoolIxArgs),
    InitializePermissionlessPoolWithFeeTier(
        InitializePermissionlessPoolWithFeeTierIxArgs,
    ),
    EnableOrDisablePool(EnableOrDisablePoolIxArgs),
    Swap(SwapIxArgs),
    RemoveLiquiditySingleSide(RemoveLiquiditySingleSideIxArgs),
    AddImbalanceLiquidity(AddImbalanceLiquidityIxArgs),
    RemoveBalanceLiquidity(RemoveBalanceLiquidityIxArgs),
    AddBalanceLiquidity(AddBalanceLiquidityIxArgs),
    SetPoolFees(SetPoolFeesIxArgs),
    OverrideCurveParam(OverrideCurveParamIxArgs),
    GetPoolInfo,
    BootstrapLiquidity(BootstrapLiquidityIxArgs),
    CreateMintMetadata,
    CreateLockEscrow,
    Lock(LockIxArgs),
    ClaimFee(ClaimFeeIxArgs),
    CreateConfig(CreateConfigIxArgs),
    CloseConfig,
    InitializePermissionlessConstantProductPoolWithConfig(
        InitializePermissionlessConstantProductPoolWithConfigIxArgs,
    ),
    InitializePermissionlessConstantProductPoolWithConfig2(
        InitializePermissionlessConstantProductPoolWithConfig2IxArgs,
    ),
    InitializeCustomizablePermissionlessConstantProductPool(
        InitializeCustomizablePermissionlessConstantProductPoolIxArgs,
    ),
    UpdateActivationPoint(UpdateActivationPointIxArgs),
    WithdrawProtocolFees,
    SetWhitelistedVault(SetWhitelistedVaultIxArgs),
    PartnerClaimFee(PartnerClaimFeeIxArgs),
}
impl MeteoraPoolsProgramIx {
    pub fn name(&self) -> String {
        // Use the ToString derived method to get the enum variant name
        self.to_string().to_camel_case()
    }
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        match maybe_discm {
            INITIALIZE_PERMISSIONED_POOL_IX_DISCM => {
                Ok(
                    Self::InitializePermissionedPool(
                        InitializePermissionedPoolIxArgs::deserialize(&mut reader)?,
                    ),
                )
            }
            INITIALIZE_PERMISSIONLESS_POOL_IX_DISCM => {
                Ok(
                    Self::InitializePermissionlessPool(
                        InitializePermissionlessPoolIxArgs::deserialize(&mut reader)?,
                    ),
                )
            }
            INITIALIZE_PERMISSIONLESS_POOL_WITH_FEE_TIER_IX_DISCM => {
                Ok(
                    Self::InitializePermissionlessPoolWithFeeTier(
                        InitializePermissionlessPoolWithFeeTierIxArgs::deserialize(
                            &mut reader,
                        )?,
                    ),
                )
            }
            ENABLE_OR_DISABLE_POOL_IX_DISCM => {
                Ok(
                    Self::EnableOrDisablePool(
                        EnableOrDisablePoolIxArgs::deserialize(&mut reader)?,
                    ),
                )
            }
            SWAP_IX_DISCM => Ok(Self::Swap(SwapIxArgs::deserialize(&mut reader)?)),
            REMOVE_LIQUIDITY_SINGLE_SIDE_IX_DISCM => {
                Ok(
                    Self::RemoveLiquiditySingleSide(
                        RemoveLiquiditySingleSideIxArgs::deserialize(&mut reader)?,
                    ),
                )
            }
            ADD_IMBALANCE_LIQUIDITY_IX_DISCM => {
                Ok(
                    Self::AddImbalanceLiquidity(
                        AddImbalanceLiquidityIxArgs::deserialize(&mut reader)?,
                    ),
                )
            }
            REMOVE_BALANCE_LIQUIDITY_IX_DISCM => {
                Ok(
                    Self::RemoveBalanceLiquidity(
                        RemoveBalanceLiquidityIxArgs::deserialize(&mut reader)?,
                    ),
                )
            }
            ADD_BALANCE_LIQUIDITY_IX_DISCM => {
                Ok(
                    Self::AddBalanceLiquidity(
                        AddBalanceLiquidityIxArgs::deserialize(&mut reader)?,
                    ),
                )
            }
            SET_POOL_FEES_IX_DISCM => {
                Ok(Self::SetPoolFees(SetPoolFeesIxArgs::deserialize(&mut reader)?))
            }
            OVERRIDE_CURVE_PARAM_IX_DISCM => {
                Ok(
                    Self::OverrideCurveParam(
                        OverrideCurveParamIxArgs::deserialize(&mut reader)?,
                    ),
                )
            }
            GET_POOL_INFO_IX_DISCM => Ok(Self::GetPoolInfo),
            BOOTSTRAP_LIQUIDITY_IX_DISCM => {
                Ok(
                    Self::BootstrapLiquidity(
                        BootstrapLiquidityIxArgs::deserialize(&mut reader)?,
                    ),
                )
            }
            CREATE_MINT_METADATA_IX_DISCM => Ok(Self::CreateMintMetadata),
            CREATE_LOCK_ESCROW_IX_DISCM => Ok(Self::CreateLockEscrow),
            LOCK_IX_DISCM => Ok(Self::Lock(LockIxArgs::deserialize(&mut reader)?)),
            CLAIM_FEE_IX_DISCM => {
                Ok(Self::ClaimFee(ClaimFeeIxArgs::deserialize(&mut reader)?))
            }
            CREATE_CONFIG_IX_DISCM => {
                Ok(Self::CreateConfig(CreateConfigIxArgs::deserialize(&mut reader)?))
            }
            CLOSE_CONFIG_IX_DISCM => Ok(Self::CloseConfig),
            INITIALIZE_PERMISSIONLESS_CONSTANT_PRODUCT_POOL_WITH_CONFIG_IX_DISCM => {
                Ok(
                    Self::InitializePermissionlessConstantProductPoolWithConfig(
                        InitializePermissionlessConstantProductPoolWithConfigIxArgs::deserialize(
                            &mut reader,
                        )?,
                    ),
                )
            }
            INITIALIZE_PERMISSIONLESS_CONSTANT_PRODUCT_POOL_WITH_CONFIG2_IX_DISCM => {
                Ok(
                    Self::InitializePermissionlessConstantProductPoolWithConfig2(
                        InitializePermissionlessConstantProductPoolWithConfig2IxArgs::deserialize(
                            &mut reader,
                        )?,
                    ),
                )
            }
            INITIALIZE_CUSTOMIZABLE_PERMISSIONLESS_CONSTANT_PRODUCT_POOL_IX_DISCM => {
                Ok(
                    Self::InitializeCustomizablePermissionlessConstantProductPool(
                        InitializeCustomizablePermissionlessConstantProductPoolIxArgs::deserialize(
                            &mut reader,
                        )?,
                    ),
                )
            }
            UPDATE_ACTIVATION_POINT_IX_DISCM => {
                Ok(
                    Self::UpdateActivationPoint(
                        UpdateActivationPointIxArgs::deserialize(&mut reader)?,
                    ),
                )
            }
            WITHDRAW_PROTOCOL_FEES_IX_DISCM => Ok(Self::WithdrawProtocolFees),
            SET_WHITELISTED_VAULT_IX_DISCM => {
                Ok(
                    Self::SetWhitelistedVault(
                        SetWhitelistedVaultIxArgs::deserialize(&mut reader)?,
                    ),
                )
            }
            PARTNER_CLAIM_FEE_IX_DISCM => {
                Ok(
                    Self::PartnerClaimFee(
                        PartnerClaimFeeIxArgs::deserialize(&mut reader)?,
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
            Self::InitializePermissionedPool(args) => {
                writer.write_all(&INITIALIZE_PERMISSIONED_POOL_IX_DISCM)?;
                args.serialize(&mut writer)
            }
            Self::InitializePermissionlessPool(args) => {
                writer.write_all(&INITIALIZE_PERMISSIONLESS_POOL_IX_DISCM)?;
                args.serialize(&mut writer)
            }
            Self::InitializePermissionlessPoolWithFeeTier(args) => {
                writer
                    .write_all(&INITIALIZE_PERMISSIONLESS_POOL_WITH_FEE_TIER_IX_DISCM)?;
                args.serialize(&mut writer)
            }
            Self::EnableOrDisablePool(args) => {
                writer.write_all(&ENABLE_OR_DISABLE_POOL_IX_DISCM)?;
                args.serialize(&mut writer)
            }
            Self::Swap(args) => {
                writer.write_all(&SWAP_IX_DISCM)?;
                args.serialize(&mut writer)
            }
            Self::RemoveLiquiditySingleSide(args) => {
                writer.write_all(&REMOVE_LIQUIDITY_SINGLE_SIDE_IX_DISCM)?;
                args.serialize(&mut writer)
            }
            Self::AddImbalanceLiquidity(args) => {
                writer.write_all(&ADD_IMBALANCE_LIQUIDITY_IX_DISCM)?;
                args.serialize(&mut writer)
            }
            Self::RemoveBalanceLiquidity(args) => {
                writer.write_all(&REMOVE_BALANCE_LIQUIDITY_IX_DISCM)?;
                args.serialize(&mut writer)
            }
            Self::AddBalanceLiquidity(args) => {
                writer.write_all(&ADD_BALANCE_LIQUIDITY_IX_DISCM)?;
                args.serialize(&mut writer)
            }
            Self::SetPoolFees(args) => {
                writer.write_all(&SET_POOL_FEES_IX_DISCM)?;
                args.serialize(&mut writer)
            }
            Self::OverrideCurveParam(args) => {
                writer.write_all(&OVERRIDE_CURVE_PARAM_IX_DISCM)?;
                args.serialize(&mut writer)
            }
            Self::GetPoolInfo => writer.write_all(&GET_POOL_INFO_IX_DISCM),
            Self::BootstrapLiquidity(args) => {
                writer.write_all(&BOOTSTRAP_LIQUIDITY_IX_DISCM)?;
                args.serialize(&mut writer)
            }
            Self::CreateMintMetadata => writer.write_all(&CREATE_MINT_METADATA_IX_DISCM),
            Self::CreateLockEscrow => writer.write_all(&CREATE_LOCK_ESCROW_IX_DISCM),
            Self::Lock(args) => {
                writer.write_all(&LOCK_IX_DISCM)?;
                args.serialize(&mut writer)
            }
            Self::ClaimFee(args) => {
                writer.write_all(&CLAIM_FEE_IX_DISCM)?;
                args.serialize(&mut writer)
            }
            Self::CreateConfig(args) => {
                writer.write_all(&CREATE_CONFIG_IX_DISCM)?;
                args.serialize(&mut writer)
            }
            Self::CloseConfig => writer.write_all(&CLOSE_CONFIG_IX_DISCM),
            Self::InitializePermissionlessConstantProductPoolWithConfig(args) => {
                writer
                    .write_all(
                        &INITIALIZE_PERMISSIONLESS_CONSTANT_PRODUCT_POOL_WITH_CONFIG_IX_DISCM,
                    )?;
                args.serialize(&mut writer)
            }
            Self::InitializePermissionlessConstantProductPoolWithConfig2(args) => {
                writer
                    .write_all(
                        &INITIALIZE_PERMISSIONLESS_CONSTANT_PRODUCT_POOL_WITH_CONFIG2_IX_DISCM,
                    )?;
                args.serialize(&mut writer)
            }
            Self::InitializeCustomizablePermissionlessConstantProductPool(args) => {
                writer
                    .write_all(
                        &INITIALIZE_CUSTOMIZABLE_PERMISSIONLESS_CONSTANT_PRODUCT_POOL_IX_DISCM,
                    )?;
                args.serialize(&mut writer)
            }
            Self::UpdateActivationPoint(args) => {
                writer.write_all(&UPDATE_ACTIVATION_POINT_IX_DISCM)?;
                args.serialize(&mut writer)
            }
            Self::WithdrawProtocolFees => {
                writer.write_all(&WITHDRAW_PROTOCOL_FEES_IX_DISCM)
            }
            Self::SetWhitelistedVault(args) => {
                writer.write_all(&SET_WHITELISTED_VAULT_IX_DISCM)?;
                args.serialize(&mut writer)
            }
            Self::PartnerClaimFee(args) => {
                writer.write_all(&PARTNER_CLAIM_FEE_IX_DISCM)?;
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
pub const INITIALIZE_PERMISSIONED_POOL_IX_ACCOUNTS_LEN: usize = 24;
#[derive(Copy, Clone, Debug)]
pub struct InitializePermissionedPoolAccounts<'me, 'info> {
    pub pool: &'me AccountInfo<'info>,
    pub lp_mint: &'me AccountInfo<'info>,
    pub token_a_mint: &'me AccountInfo<'info>,
    pub token_b_mint: &'me AccountInfo<'info>,
    pub a_vault: &'me AccountInfo<'info>,
    pub b_vault: &'me AccountInfo<'info>,
    pub a_vault_lp_mint: &'me AccountInfo<'info>,
    pub b_vault_lp_mint: &'me AccountInfo<'info>,
    pub a_vault_lp: &'me AccountInfo<'info>,
    pub b_vault_lp: &'me AccountInfo<'info>,
    pub admin_token_a: &'me AccountInfo<'info>,
    pub admin_token_b: &'me AccountInfo<'info>,
    pub admin_pool_lp: &'me AccountInfo<'info>,
    pub protocol_token_a_fee: &'me AccountInfo<'info>,
    pub protocol_token_b_fee: &'me AccountInfo<'info>,
    pub admin: &'me AccountInfo<'info>,
    pub fee_owner: &'me AccountInfo<'info>,
    pub rent: &'me AccountInfo<'info>,
    pub mint_metadata: &'me AccountInfo<'info>,
    pub metadata_program: &'me AccountInfo<'info>,
    pub vault_program: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub associated_token_program: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct InitializePermissionedPoolKeys {
    pub pool: Pubkey,
    pub lp_mint: Pubkey,
    pub token_a_mint: Pubkey,
    pub token_b_mint: Pubkey,
    pub a_vault: Pubkey,
    pub b_vault: Pubkey,
    pub a_vault_lp_mint: Pubkey,
    pub b_vault_lp_mint: Pubkey,
    pub a_vault_lp: Pubkey,
    pub b_vault_lp: Pubkey,
    pub admin_token_a: Pubkey,
    pub admin_token_b: Pubkey,
    pub admin_pool_lp: Pubkey,
    pub protocol_token_a_fee: Pubkey,
    pub protocol_token_b_fee: Pubkey,
    pub admin: Pubkey,
    pub fee_owner: Pubkey,
    pub rent: Pubkey,
    pub mint_metadata: Pubkey,
    pub metadata_program: Pubkey,
    pub vault_program: Pubkey,
    pub token_program: Pubkey,
    pub associated_token_program: Pubkey,
    pub system_program: Pubkey,
}
impl From<InitializePermissionedPoolAccounts<'_, '_>>
for InitializePermissionedPoolKeys {
    fn from(accounts: InitializePermissionedPoolAccounts) -> Self {
        Self {
            pool: *accounts.pool.key,
            lp_mint: *accounts.lp_mint.key,
            token_a_mint: *accounts.token_a_mint.key,
            token_b_mint: *accounts.token_b_mint.key,
            a_vault: *accounts.a_vault.key,
            b_vault: *accounts.b_vault.key,
            a_vault_lp_mint: *accounts.a_vault_lp_mint.key,
            b_vault_lp_mint: *accounts.b_vault_lp_mint.key,
            a_vault_lp: *accounts.a_vault_lp.key,
            b_vault_lp: *accounts.b_vault_lp.key,
            admin_token_a: *accounts.admin_token_a.key,
            admin_token_b: *accounts.admin_token_b.key,
            admin_pool_lp: *accounts.admin_pool_lp.key,
            protocol_token_a_fee: *accounts.protocol_token_a_fee.key,
            protocol_token_b_fee: *accounts.protocol_token_b_fee.key,
            admin: *accounts.admin.key,
            fee_owner: *accounts.fee_owner.key,
            rent: *accounts.rent.key,
            mint_metadata: *accounts.mint_metadata.key,
            metadata_program: *accounts.metadata_program.key,
            vault_program: *accounts.vault_program.key,
            token_program: *accounts.token_program.key,
            associated_token_program: *accounts.associated_token_program.key,
            system_program: *accounts.system_program.key,
        }
    }
}
impl From<InitializePermissionedPoolKeys>
for [AccountMeta; INITIALIZE_PERMISSIONED_POOL_IX_ACCOUNTS_LEN] {
    fn from(keys: InitializePermissionedPoolKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.pool,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.lp_mint,
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
                pubkey: keys.admin_token_a,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.admin_token_b,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.admin_pool_lp,
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
                pubkey: keys.admin,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.fee_owner,
                is_signer: false,
                is_writable: false,
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
impl From<[Pubkey; INITIALIZE_PERMISSIONED_POOL_IX_ACCOUNTS_LEN]>
for InitializePermissionedPoolKeys {
    fn from(pubkeys: [Pubkey; INITIALIZE_PERMISSIONED_POOL_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            pool: pubkeys[0],
            lp_mint: pubkeys[1],
            token_a_mint: pubkeys[2],
            token_b_mint: pubkeys[3],
            a_vault: pubkeys[4],
            b_vault: pubkeys[5],
            a_vault_lp_mint: pubkeys[6],
            b_vault_lp_mint: pubkeys[7],
            a_vault_lp: pubkeys[8],
            b_vault_lp: pubkeys[9],
            admin_token_a: pubkeys[10],
            admin_token_b: pubkeys[11],
            admin_pool_lp: pubkeys[12],
            protocol_token_a_fee: pubkeys[13],
            protocol_token_b_fee: pubkeys[14],
            admin: pubkeys[15],
            fee_owner: pubkeys[16],
            rent: pubkeys[17],
            mint_metadata: pubkeys[18],
            metadata_program: pubkeys[19],
            vault_program: pubkeys[20],
            token_program: pubkeys[21],
            associated_token_program: pubkeys[22],
            system_program: pubkeys[23],
        }
    }
}
impl<'info> From<InitializePermissionedPoolAccounts<'_, 'info>>
for [AccountInfo<'info>; INITIALIZE_PERMISSIONED_POOL_IX_ACCOUNTS_LEN] {
    fn from(accounts: InitializePermissionedPoolAccounts<'_, 'info>) -> Self {
        [
            accounts.pool.clone(),
            accounts.lp_mint.clone(),
            accounts.token_a_mint.clone(),
            accounts.token_b_mint.clone(),
            accounts.a_vault.clone(),
            accounts.b_vault.clone(),
            accounts.a_vault_lp_mint.clone(),
            accounts.b_vault_lp_mint.clone(),
            accounts.a_vault_lp.clone(),
            accounts.b_vault_lp.clone(),
            accounts.admin_token_a.clone(),
            accounts.admin_token_b.clone(),
            accounts.admin_pool_lp.clone(),
            accounts.protocol_token_a_fee.clone(),
            accounts.protocol_token_b_fee.clone(),
            accounts.admin.clone(),
            accounts.fee_owner.clone(),
            accounts.rent.clone(),
            accounts.mint_metadata.clone(),
            accounts.metadata_program.clone(),
            accounts.vault_program.clone(),
            accounts.token_program.clone(),
            accounts.associated_token_program.clone(),
            accounts.system_program.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; INITIALIZE_PERMISSIONED_POOL_IX_ACCOUNTS_LEN]>
for InitializePermissionedPoolAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; INITIALIZE_PERMISSIONED_POOL_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            pool: &arr[0],
            lp_mint: &arr[1],
            token_a_mint: &arr[2],
            token_b_mint: &arr[3],
            a_vault: &arr[4],
            b_vault: &arr[5],
            a_vault_lp_mint: &arr[6],
            b_vault_lp_mint: &arr[7],
            a_vault_lp: &arr[8],
            b_vault_lp: &arr[9],
            admin_token_a: &arr[10],
            admin_token_b: &arr[11],
            admin_pool_lp: &arr[12],
            protocol_token_a_fee: &arr[13],
            protocol_token_b_fee: &arr[14],
            admin: &arr[15],
            fee_owner: &arr[16],
            rent: &arr[17],
            mint_metadata: &arr[18],
            metadata_program: &arr[19],
            vault_program: &arr[20],
            token_program: &arr[21],
            associated_token_program: &arr[22],
            system_program: &arr[23],
        }
    }
}
pub const INITIALIZE_PERMISSIONED_POOL_IX_DISCM: [u8; 8] = [
    77,
    85,
    178,
    157,
    50,
    48,
    212,
    126,
];
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InitializePermissionedPoolIxArgs {
    pub curve_type: CurveType,
}
#[derive(Clone, Debug, PartialEq)]
pub struct InitializePermissionedPoolIxData(pub InitializePermissionedPoolIxArgs);
impl From<InitializePermissionedPoolIxArgs> for InitializePermissionedPoolIxData {
    fn from(args: InitializePermissionedPoolIxArgs) -> Self {
        Self(args)
    }
}
impl InitializePermissionedPoolIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != INITIALIZE_PERMISSIONED_POOL_IX_DISCM {
            return Err(
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!(
                        "discm does not match. Expected: {:?}. Received: {:?}",
                        INITIALIZE_PERMISSIONED_POOL_IX_DISCM, maybe_discm
                    ),
                ),
            );
        }
        Ok(Self(InitializePermissionedPoolIxArgs::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&INITIALIZE_PERMISSIONED_POOL_IX_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn initialize_permissioned_pool_ix_with_program_id(
    program_id: Pubkey,
    keys: InitializePermissionedPoolKeys,
    args: InitializePermissionedPoolIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; INITIALIZE_PERMISSIONED_POOL_IX_ACCOUNTS_LEN] = keys.into();
    let data: InitializePermissionedPoolIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn initialize_permissioned_pool_ix(
    keys: InitializePermissionedPoolKeys,
    args: InitializePermissionedPoolIxArgs,
) -> std::io::Result<Instruction> {
    initialize_permissioned_pool_ix_with_program_id(crate::ID, keys, args)
}
pub fn initialize_permissioned_pool_invoke_with_program_id(
    program_id: Pubkey,
    accounts: InitializePermissionedPoolAccounts<'_, '_>,
    args: InitializePermissionedPoolIxArgs,
) -> ProgramResult {
    let keys: InitializePermissionedPoolKeys = accounts.into();
    let ix = initialize_permissioned_pool_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn initialize_permissioned_pool_invoke(
    accounts: InitializePermissionedPoolAccounts<'_, '_>,
    args: InitializePermissionedPoolIxArgs,
) -> ProgramResult {
    initialize_permissioned_pool_invoke_with_program_id(crate::ID, accounts, args)
}
pub fn initialize_permissioned_pool_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: InitializePermissionedPoolAccounts<'_, '_>,
    args: InitializePermissionedPoolIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: InitializePermissionedPoolKeys = accounts.into();
    let ix = initialize_permissioned_pool_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn initialize_permissioned_pool_invoke_signed(
    accounts: InitializePermissionedPoolAccounts<'_, '_>,
    args: InitializePermissionedPoolIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    initialize_permissioned_pool_invoke_signed_with_program_id(
        crate::ID,
        accounts,
        args,
        seeds,
    )
}
pub fn initialize_permissioned_pool_verify_account_keys(
    accounts: InitializePermissionedPoolAccounts<'_, '_>,
    keys: InitializePermissionedPoolKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.pool.key, keys.pool),
        (*accounts.lp_mint.key, keys.lp_mint),
        (*accounts.token_a_mint.key, keys.token_a_mint),
        (*accounts.token_b_mint.key, keys.token_b_mint),
        (*accounts.a_vault.key, keys.a_vault),
        (*accounts.b_vault.key, keys.b_vault),
        (*accounts.a_vault_lp_mint.key, keys.a_vault_lp_mint),
        (*accounts.b_vault_lp_mint.key, keys.b_vault_lp_mint),
        (*accounts.a_vault_lp.key, keys.a_vault_lp),
        (*accounts.b_vault_lp.key, keys.b_vault_lp),
        (*accounts.admin_token_a.key, keys.admin_token_a),
        (*accounts.admin_token_b.key, keys.admin_token_b),
        (*accounts.admin_pool_lp.key, keys.admin_pool_lp),
        (*accounts.protocol_token_a_fee.key, keys.protocol_token_a_fee),
        (*accounts.protocol_token_b_fee.key, keys.protocol_token_b_fee),
        (*accounts.admin.key, keys.admin),
        (*accounts.fee_owner.key, keys.fee_owner),
        (*accounts.rent.key, keys.rent),
        (*accounts.mint_metadata.key, keys.mint_metadata),
        (*accounts.metadata_program.key, keys.metadata_program),
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
pub fn initialize_permissioned_pool_verify_writable_privileges<'me, 'info>(
    accounts: InitializePermissionedPoolAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.pool,
        accounts.lp_mint,
        accounts.a_vault,
        accounts.b_vault,
        accounts.a_vault_lp_mint,
        accounts.b_vault_lp_mint,
        accounts.a_vault_lp,
        accounts.b_vault_lp,
        accounts.admin_token_a,
        accounts.admin_token_b,
        accounts.admin_pool_lp,
        accounts.protocol_token_a_fee,
        accounts.protocol_token_b_fee,
        accounts.admin,
        accounts.mint_metadata,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn initialize_permissioned_pool_verify_signer_privileges<'me, 'info>(
    accounts: InitializePermissionedPoolAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.pool, accounts.admin] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn initialize_permissioned_pool_verify_account_privileges<'me, 'info>(
    accounts: InitializePermissionedPoolAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    initialize_permissioned_pool_verify_writable_privileges(accounts)?;
    initialize_permissioned_pool_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const INITIALIZE_PERMISSIONLESS_POOL_IX_ACCOUNTS_LEN: usize = 26;
#[derive(Copy, Clone, Debug)]
pub struct InitializePermissionlessPoolAccounts<'me, 'info> {
    pub pool: &'me AccountInfo<'info>,
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
    pub payer_token_a: &'me AccountInfo<'info>,
    pub payer_token_b: &'me AccountInfo<'info>,
    pub payer_pool_lp: &'me AccountInfo<'info>,
    pub protocol_token_a_fee: &'me AccountInfo<'info>,
    pub protocol_token_b_fee: &'me AccountInfo<'info>,
    pub payer: &'me AccountInfo<'info>,
    pub fee_owner: &'me AccountInfo<'info>,
    pub rent: &'me AccountInfo<'info>,
    pub mint_metadata: &'me AccountInfo<'info>,
    pub metadata_program: &'me AccountInfo<'info>,
    pub vault_program: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub associated_token_program: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct InitializePermissionlessPoolKeys {
    pub pool: Pubkey,
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
    pub payer_token_a: Pubkey,
    pub payer_token_b: Pubkey,
    pub payer_pool_lp: Pubkey,
    pub protocol_token_a_fee: Pubkey,
    pub protocol_token_b_fee: Pubkey,
    pub payer: Pubkey,
    pub fee_owner: Pubkey,
    pub rent: Pubkey,
    pub mint_metadata: Pubkey,
    pub metadata_program: Pubkey,
    pub vault_program: Pubkey,
    pub token_program: Pubkey,
    pub associated_token_program: Pubkey,
    pub system_program: Pubkey,
}
impl From<InitializePermissionlessPoolAccounts<'_, '_>>
for InitializePermissionlessPoolKeys {
    fn from(accounts: InitializePermissionlessPoolAccounts) -> Self {
        Self {
            pool: *accounts.pool.key,
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
            payer_token_a: *accounts.payer_token_a.key,
            payer_token_b: *accounts.payer_token_b.key,
            payer_pool_lp: *accounts.payer_pool_lp.key,
            protocol_token_a_fee: *accounts.protocol_token_a_fee.key,
            protocol_token_b_fee: *accounts.protocol_token_b_fee.key,
            payer: *accounts.payer.key,
            fee_owner: *accounts.fee_owner.key,
            rent: *accounts.rent.key,
            mint_metadata: *accounts.mint_metadata.key,
            metadata_program: *accounts.metadata_program.key,
            vault_program: *accounts.vault_program.key,
            token_program: *accounts.token_program.key,
            associated_token_program: *accounts.associated_token_program.key,
            system_program: *accounts.system_program.key,
        }
    }
}
impl From<InitializePermissionlessPoolKeys>
for [AccountMeta; INITIALIZE_PERMISSIONLESS_POOL_IX_ACCOUNTS_LEN] {
    fn from(keys: InitializePermissionlessPoolKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.pool,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.lp_mint,
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
                pubkey: keys.payer_pool_lp,
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
                pubkey: keys.fee_owner,
                is_signer: false,
                is_writable: false,
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
impl From<[Pubkey; INITIALIZE_PERMISSIONLESS_POOL_IX_ACCOUNTS_LEN]>
for InitializePermissionlessPoolKeys {
    fn from(pubkeys: [Pubkey; INITIALIZE_PERMISSIONLESS_POOL_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            pool: pubkeys[0],
            lp_mint: pubkeys[1],
            token_a_mint: pubkeys[2],
            token_b_mint: pubkeys[3],
            a_vault: pubkeys[4],
            b_vault: pubkeys[5],
            a_token_vault: pubkeys[6],
            b_token_vault: pubkeys[7],
            a_vault_lp_mint: pubkeys[8],
            b_vault_lp_mint: pubkeys[9],
            a_vault_lp: pubkeys[10],
            b_vault_lp: pubkeys[11],
            payer_token_a: pubkeys[12],
            payer_token_b: pubkeys[13],
            payer_pool_lp: pubkeys[14],
            protocol_token_a_fee: pubkeys[15],
            protocol_token_b_fee: pubkeys[16],
            payer: pubkeys[17],
            fee_owner: pubkeys[18],
            rent: pubkeys[19],
            mint_metadata: pubkeys[20],
            metadata_program: pubkeys[21],
            vault_program: pubkeys[22],
            token_program: pubkeys[23],
            associated_token_program: pubkeys[24],
            system_program: pubkeys[25],
        }
    }
}
impl<'info> From<InitializePermissionlessPoolAccounts<'_, 'info>>
for [AccountInfo<'info>; INITIALIZE_PERMISSIONLESS_POOL_IX_ACCOUNTS_LEN] {
    fn from(accounts: InitializePermissionlessPoolAccounts<'_, 'info>) -> Self {
        [
            accounts.pool.clone(),
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
            accounts.payer_token_a.clone(),
            accounts.payer_token_b.clone(),
            accounts.payer_pool_lp.clone(),
            accounts.protocol_token_a_fee.clone(),
            accounts.protocol_token_b_fee.clone(),
            accounts.payer.clone(),
            accounts.fee_owner.clone(),
            accounts.rent.clone(),
            accounts.mint_metadata.clone(),
            accounts.metadata_program.clone(),
            accounts.vault_program.clone(),
            accounts.token_program.clone(),
            accounts.associated_token_program.clone(),
            accounts.system_program.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; INITIALIZE_PERMISSIONLESS_POOL_IX_ACCOUNTS_LEN]>
for InitializePermissionlessPoolAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; INITIALIZE_PERMISSIONLESS_POOL_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            pool: &arr[0],
            lp_mint: &arr[1],
            token_a_mint: &arr[2],
            token_b_mint: &arr[3],
            a_vault: &arr[4],
            b_vault: &arr[5],
            a_token_vault: &arr[6],
            b_token_vault: &arr[7],
            a_vault_lp_mint: &arr[8],
            b_vault_lp_mint: &arr[9],
            a_vault_lp: &arr[10],
            b_vault_lp: &arr[11],
            payer_token_a: &arr[12],
            payer_token_b: &arr[13],
            payer_pool_lp: &arr[14],
            protocol_token_a_fee: &arr[15],
            protocol_token_b_fee: &arr[16],
            payer: &arr[17],
            fee_owner: &arr[18],
            rent: &arr[19],
            mint_metadata: &arr[20],
            metadata_program: &arr[21],
            vault_program: &arr[22],
            token_program: &arr[23],
            associated_token_program: &arr[24],
            system_program: &arr[25],
        }
    }
}
pub const INITIALIZE_PERMISSIONLESS_POOL_IX_DISCM: [u8; 8] = [
    118,
    173,
    41,
    157,
    173,
    72,
    97,
    103,
];
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InitializePermissionlessPoolIxArgs {
    pub curve_type: CurveType,
    pub token_a_amount: u64,
    pub token_b_amount: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct InitializePermissionlessPoolIxData(pub InitializePermissionlessPoolIxArgs);
impl From<InitializePermissionlessPoolIxArgs> for InitializePermissionlessPoolIxData {
    fn from(args: InitializePermissionlessPoolIxArgs) -> Self {
        Self(args)
    }
}
impl InitializePermissionlessPoolIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != INITIALIZE_PERMISSIONLESS_POOL_IX_DISCM {
            return Err(
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!(
                        "discm does not match. Expected: {:?}. Received: {:?}",
                        INITIALIZE_PERMISSIONLESS_POOL_IX_DISCM, maybe_discm
                    ),
                ),
            );
        }
        Ok(Self(InitializePermissionlessPoolIxArgs::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&INITIALIZE_PERMISSIONLESS_POOL_IX_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn initialize_permissionless_pool_ix_with_program_id(
    program_id: Pubkey,
    keys: InitializePermissionlessPoolKeys,
    args: InitializePermissionlessPoolIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; INITIALIZE_PERMISSIONLESS_POOL_IX_ACCOUNTS_LEN] = keys
        .into();
    let data: InitializePermissionlessPoolIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn initialize_permissionless_pool_ix(
    keys: InitializePermissionlessPoolKeys,
    args: InitializePermissionlessPoolIxArgs,
) -> std::io::Result<Instruction> {
    initialize_permissionless_pool_ix_with_program_id(crate::ID, keys, args)
}
pub fn initialize_permissionless_pool_invoke_with_program_id(
    program_id: Pubkey,
    accounts: InitializePermissionlessPoolAccounts<'_, '_>,
    args: InitializePermissionlessPoolIxArgs,
) -> ProgramResult {
    let keys: InitializePermissionlessPoolKeys = accounts.into();
    let ix = initialize_permissionless_pool_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn initialize_permissionless_pool_invoke(
    accounts: InitializePermissionlessPoolAccounts<'_, '_>,
    args: InitializePermissionlessPoolIxArgs,
) -> ProgramResult {
    initialize_permissionless_pool_invoke_with_program_id(crate::ID, accounts, args)
}
pub fn initialize_permissionless_pool_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: InitializePermissionlessPoolAccounts<'_, '_>,
    args: InitializePermissionlessPoolIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: InitializePermissionlessPoolKeys = accounts.into();
    let ix = initialize_permissionless_pool_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn initialize_permissionless_pool_invoke_signed(
    accounts: InitializePermissionlessPoolAccounts<'_, '_>,
    args: InitializePermissionlessPoolIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    initialize_permissionless_pool_invoke_signed_with_program_id(
        crate::ID,
        accounts,
        args,
        seeds,
    )
}
pub fn initialize_permissionless_pool_verify_account_keys(
    accounts: InitializePermissionlessPoolAccounts<'_, '_>,
    keys: InitializePermissionlessPoolKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.pool.key, keys.pool),
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
        (*accounts.payer_token_a.key, keys.payer_token_a),
        (*accounts.payer_token_b.key, keys.payer_token_b),
        (*accounts.payer_pool_lp.key, keys.payer_pool_lp),
        (*accounts.protocol_token_a_fee.key, keys.protocol_token_a_fee),
        (*accounts.protocol_token_b_fee.key, keys.protocol_token_b_fee),
        (*accounts.payer.key, keys.payer),
        (*accounts.fee_owner.key, keys.fee_owner),
        (*accounts.rent.key, keys.rent),
        (*accounts.mint_metadata.key, keys.mint_metadata),
        (*accounts.metadata_program.key, keys.metadata_program),
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
pub fn initialize_permissionless_pool_verify_writable_privileges<'me, 'info>(
    accounts: InitializePermissionlessPoolAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.pool,
        accounts.lp_mint,
        accounts.a_vault,
        accounts.b_vault,
        accounts.a_token_vault,
        accounts.b_token_vault,
        accounts.a_vault_lp_mint,
        accounts.b_vault_lp_mint,
        accounts.a_vault_lp,
        accounts.b_vault_lp,
        accounts.payer_token_a,
        accounts.payer_token_b,
        accounts.payer_pool_lp,
        accounts.protocol_token_a_fee,
        accounts.protocol_token_b_fee,
        accounts.payer,
        accounts.mint_metadata,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn initialize_permissionless_pool_verify_signer_privileges<'me, 'info>(
    accounts: InitializePermissionlessPoolAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.payer] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn initialize_permissionless_pool_verify_account_privileges<'me, 'info>(
    accounts: InitializePermissionlessPoolAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    initialize_permissionless_pool_verify_writable_privileges(accounts)?;
    initialize_permissionless_pool_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const INITIALIZE_PERMISSIONLESS_POOL_WITH_FEE_TIER_IX_ACCOUNTS_LEN: usize = 26;
#[derive(Copy, Clone, Debug)]
pub struct InitializePermissionlessPoolWithFeeTierAccounts<'me, 'info> {
    pub pool: &'me AccountInfo<'info>,
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
    pub payer_token_a: &'me AccountInfo<'info>,
    pub payer_token_b: &'me AccountInfo<'info>,
    pub payer_pool_lp: &'me AccountInfo<'info>,
    pub protocol_token_a_fee: &'me AccountInfo<'info>,
    pub protocol_token_b_fee: &'me AccountInfo<'info>,
    pub payer: &'me AccountInfo<'info>,
    pub fee_owner: &'me AccountInfo<'info>,
    pub rent: &'me AccountInfo<'info>,
    pub mint_metadata: &'me AccountInfo<'info>,
    pub metadata_program: &'me AccountInfo<'info>,
    pub vault_program: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub associated_token_program: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct InitializePermissionlessPoolWithFeeTierKeys {
    pub pool: Pubkey,
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
    pub payer_token_a: Pubkey,
    pub payer_token_b: Pubkey,
    pub payer_pool_lp: Pubkey,
    pub protocol_token_a_fee: Pubkey,
    pub protocol_token_b_fee: Pubkey,
    pub payer: Pubkey,
    pub fee_owner: Pubkey,
    pub rent: Pubkey,
    pub mint_metadata: Pubkey,
    pub metadata_program: Pubkey,
    pub vault_program: Pubkey,
    pub token_program: Pubkey,
    pub associated_token_program: Pubkey,
    pub system_program: Pubkey,
}
impl From<InitializePermissionlessPoolWithFeeTierAccounts<'_, '_>>
for InitializePermissionlessPoolWithFeeTierKeys {
    fn from(accounts: InitializePermissionlessPoolWithFeeTierAccounts) -> Self {
        Self {
            pool: *accounts.pool.key,
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
            payer_token_a: *accounts.payer_token_a.key,
            payer_token_b: *accounts.payer_token_b.key,
            payer_pool_lp: *accounts.payer_pool_lp.key,
            protocol_token_a_fee: *accounts.protocol_token_a_fee.key,
            protocol_token_b_fee: *accounts.protocol_token_b_fee.key,
            payer: *accounts.payer.key,
            fee_owner: *accounts.fee_owner.key,
            rent: *accounts.rent.key,
            mint_metadata: *accounts.mint_metadata.key,
            metadata_program: *accounts.metadata_program.key,
            vault_program: *accounts.vault_program.key,
            token_program: *accounts.token_program.key,
            associated_token_program: *accounts.associated_token_program.key,
            system_program: *accounts.system_program.key,
        }
    }
}
impl From<InitializePermissionlessPoolWithFeeTierKeys>
for [AccountMeta; INITIALIZE_PERMISSIONLESS_POOL_WITH_FEE_TIER_IX_ACCOUNTS_LEN] {
    fn from(keys: InitializePermissionlessPoolWithFeeTierKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.pool,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.lp_mint,
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
                pubkey: keys.payer_pool_lp,
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
                pubkey: keys.fee_owner,
                is_signer: false,
                is_writable: false,
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
impl From<[Pubkey; INITIALIZE_PERMISSIONLESS_POOL_WITH_FEE_TIER_IX_ACCOUNTS_LEN]>
for InitializePermissionlessPoolWithFeeTierKeys {
    fn from(
        pubkeys: [Pubkey; INITIALIZE_PERMISSIONLESS_POOL_WITH_FEE_TIER_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            pool: pubkeys[0],
            lp_mint: pubkeys[1],
            token_a_mint: pubkeys[2],
            token_b_mint: pubkeys[3],
            a_vault: pubkeys[4],
            b_vault: pubkeys[5],
            a_token_vault: pubkeys[6],
            b_token_vault: pubkeys[7],
            a_vault_lp_mint: pubkeys[8],
            b_vault_lp_mint: pubkeys[9],
            a_vault_lp: pubkeys[10],
            b_vault_lp: pubkeys[11],
            payer_token_a: pubkeys[12],
            payer_token_b: pubkeys[13],
            payer_pool_lp: pubkeys[14],
            protocol_token_a_fee: pubkeys[15],
            protocol_token_b_fee: pubkeys[16],
            payer: pubkeys[17],
            fee_owner: pubkeys[18],
            rent: pubkeys[19],
            mint_metadata: pubkeys[20],
            metadata_program: pubkeys[21],
            vault_program: pubkeys[22],
            token_program: pubkeys[23],
            associated_token_program: pubkeys[24],
            system_program: pubkeys[25],
        }
    }
}
impl<'info> From<InitializePermissionlessPoolWithFeeTierAccounts<'_, 'info>>
for [AccountInfo<'info>; INITIALIZE_PERMISSIONLESS_POOL_WITH_FEE_TIER_IX_ACCOUNTS_LEN] {
    fn from(
        accounts: InitializePermissionlessPoolWithFeeTierAccounts<'_, 'info>,
    ) -> Self {
        [
            accounts.pool.clone(),
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
            accounts.payer_token_a.clone(),
            accounts.payer_token_b.clone(),
            accounts.payer_pool_lp.clone(),
            accounts.protocol_token_a_fee.clone(),
            accounts.protocol_token_b_fee.clone(),
            accounts.payer.clone(),
            accounts.fee_owner.clone(),
            accounts.rent.clone(),
            accounts.mint_metadata.clone(),
            accounts.metadata_program.clone(),
            accounts.vault_program.clone(),
            accounts.token_program.clone(),
            accounts.associated_token_program.clone(),
            accounts.system_program.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<
    &'me [AccountInfo<
        'info,
    >; INITIALIZE_PERMISSIONLESS_POOL_WITH_FEE_TIER_IX_ACCOUNTS_LEN],
> for InitializePermissionlessPoolWithFeeTierAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<
            'info,
        >; INITIALIZE_PERMISSIONLESS_POOL_WITH_FEE_TIER_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            pool: &arr[0],
            lp_mint: &arr[1],
            token_a_mint: &arr[2],
            token_b_mint: &arr[3],
            a_vault: &arr[4],
            b_vault: &arr[5],
            a_token_vault: &arr[6],
            b_token_vault: &arr[7],
            a_vault_lp_mint: &arr[8],
            b_vault_lp_mint: &arr[9],
            a_vault_lp: &arr[10],
            b_vault_lp: &arr[11],
            payer_token_a: &arr[12],
            payer_token_b: &arr[13],
            payer_pool_lp: &arr[14],
            protocol_token_a_fee: &arr[15],
            protocol_token_b_fee: &arr[16],
            payer: &arr[17],
            fee_owner: &arr[18],
            rent: &arr[19],
            mint_metadata: &arr[20],
            metadata_program: &arr[21],
            vault_program: &arr[22],
            token_program: &arr[23],
            associated_token_program: &arr[24],
            system_program: &arr[25],
        }
    }
}
pub const INITIALIZE_PERMISSIONLESS_POOL_WITH_FEE_TIER_IX_DISCM: [u8; 8] = [
    6,
    135,
    68,
    147,
    229,
    82,
    169,
    113,
];
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InitializePermissionlessPoolWithFeeTierIxArgs {
    pub curve_type: CurveType,
    pub trade_fee_bps: u64,
    pub token_a_amount: u64,
    pub token_b_amount: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct InitializePermissionlessPoolWithFeeTierIxData(
    pub InitializePermissionlessPoolWithFeeTierIxArgs,
);
impl From<InitializePermissionlessPoolWithFeeTierIxArgs>
for InitializePermissionlessPoolWithFeeTierIxData {
    fn from(args: InitializePermissionlessPoolWithFeeTierIxArgs) -> Self {
        Self(args)
    }
}
impl InitializePermissionlessPoolWithFeeTierIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != INITIALIZE_PERMISSIONLESS_POOL_WITH_FEE_TIER_IX_DISCM {
            return Err(
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!(
                        "discm does not match. Expected: {:?}. Received: {:?}",
                        INITIALIZE_PERMISSIONLESS_POOL_WITH_FEE_TIER_IX_DISCM,
                        maybe_discm
                    ),
                ),
            );
        }
        Ok(
            Self(
                InitializePermissionlessPoolWithFeeTierIxArgs::deserialize(&mut reader)?,
            ),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&INITIALIZE_PERMISSIONLESS_POOL_WITH_FEE_TIER_IX_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn initialize_permissionless_pool_with_fee_tier_ix_with_program_id(
    program_id: Pubkey,
    keys: InitializePermissionlessPoolWithFeeTierKeys,
    args: InitializePermissionlessPoolWithFeeTierIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; INITIALIZE_PERMISSIONLESS_POOL_WITH_FEE_TIER_IX_ACCOUNTS_LEN] = keys
        .into();
    let data: InitializePermissionlessPoolWithFeeTierIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn initialize_permissionless_pool_with_fee_tier_ix(
    keys: InitializePermissionlessPoolWithFeeTierKeys,
    args: InitializePermissionlessPoolWithFeeTierIxArgs,
) -> std::io::Result<Instruction> {
    initialize_permissionless_pool_with_fee_tier_ix_with_program_id(
        crate::ID,
        keys,
        args,
    )
}
pub fn initialize_permissionless_pool_with_fee_tier_invoke_with_program_id(
    program_id: Pubkey,
    accounts: InitializePermissionlessPoolWithFeeTierAccounts<'_, '_>,
    args: InitializePermissionlessPoolWithFeeTierIxArgs,
) -> ProgramResult {
    let keys: InitializePermissionlessPoolWithFeeTierKeys = accounts.into();
    let ix = initialize_permissionless_pool_with_fee_tier_ix_with_program_id(
        program_id,
        keys,
        args,
    )?;
    invoke_instruction(&ix, accounts)
}
pub fn initialize_permissionless_pool_with_fee_tier_invoke(
    accounts: InitializePermissionlessPoolWithFeeTierAccounts<'_, '_>,
    args: InitializePermissionlessPoolWithFeeTierIxArgs,
) -> ProgramResult {
    initialize_permissionless_pool_with_fee_tier_invoke_with_program_id(
        crate::ID,
        accounts,
        args,
    )
}
pub fn initialize_permissionless_pool_with_fee_tier_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: InitializePermissionlessPoolWithFeeTierAccounts<'_, '_>,
    args: InitializePermissionlessPoolWithFeeTierIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: InitializePermissionlessPoolWithFeeTierKeys = accounts.into();
    let ix = initialize_permissionless_pool_with_fee_tier_ix_with_program_id(
        program_id,
        keys,
        args,
    )?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn initialize_permissionless_pool_with_fee_tier_invoke_signed(
    accounts: InitializePermissionlessPoolWithFeeTierAccounts<'_, '_>,
    args: InitializePermissionlessPoolWithFeeTierIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    initialize_permissionless_pool_with_fee_tier_invoke_signed_with_program_id(
        crate::ID,
        accounts,
        args,
        seeds,
    )
}
pub fn initialize_permissionless_pool_with_fee_tier_verify_account_keys(
    accounts: InitializePermissionlessPoolWithFeeTierAccounts<'_, '_>,
    keys: InitializePermissionlessPoolWithFeeTierKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.pool.key, keys.pool),
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
        (*accounts.payer_token_a.key, keys.payer_token_a),
        (*accounts.payer_token_b.key, keys.payer_token_b),
        (*accounts.payer_pool_lp.key, keys.payer_pool_lp),
        (*accounts.protocol_token_a_fee.key, keys.protocol_token_a_fee),
        (*accounts.protocol_token_b_fee.key, keys.protocol_token_b_fee),
        (*accounts.payer.key, keys.payer),
        (*accounts.fee_owner.key, keys.fee_owner),
        (*accounts.rent.key, keys.rent),
        (*accounts.mint_metadata.key, keys.mint_metadata),
        (*accounts.metadata_program.key, keys.metadata_program),
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
pub fn initialize_permissionless_pool_with_fee_tier_verify_writable_privileges<
    'me,
    'info,
>(
    accounts: InitializePermissionlessPoolWithFeeTierAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.pool,
        accounts.lp_mint,
        accounts.a_vault,
        accounts.b_vault,
        accounts.a_token_vault,
        accounts.b_token_vault,
        accounts.a_vault_lp_mint,
        accounts.b_vault_lp_mint,
        accounts.a_vault_lp,
        accounts.b_vault_lp,
        accounts.payer_token_a,
        accounts.payer_token_b,
        accounts.payer_pool_lp,
        accounts.protocol_token_a_fee,
        accounts.protocol_token_b_fee,
        accounts.payer,
        accounts.mint_metadata,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn initialize_permissionless_pool_with_fee_tier_verify_signer_privileges<'me, 'info>(
    accounts: InitializePermissionlessPoolWithFeeTierAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.payer] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn initialize_permissionless_pool_with_fee_tier_verify_account_privileges<
    'me,
    'info,
>(
    accounts: InitializePermissionlessPoolWithFeeTierAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    initialize_permissionless_pool_with_fee_tier_verify_writable_privileges(accounts)?;
    initialize_permissionless_pool_with_fee_tier_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const ENABLE_OR_DISABLE_POOL_IX_ACCOUNTS_LEN: usize = 2;
#[derive(Copy, Clone, Debug)]
pub struct EnableOrDisablePoolAccounts<'me, 'info> {
    pub pool: &'me AccountInfo<'info>,
    pub admin: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct EnableOrDisablePoolKeys {
    pub pool: Pubkey,
    pub admin: Pubkey,
}
impl From<EnableOrDisablePoolAccounts<'_, '_>> for EnableOrDisablePoolKeys {
    fn from(accounts: EnableOrDisablePoolAccounts) -> Self {
        Self {
            pool: *accounts.pool.key,
            admin: *accounts.admin.key,
        }
    }
}
impl From<EnableOrDisablePoolKeys>
for [AccountMeta; ENABLE_OR_DISABLE_POOL_IX_ACCOUNTS_LEN] {
    fn from(keys: EnableOrDisablePoolKeys) -> Self {
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
        ]
    }
}
impl From<[Pubkey; ENABLE_OR_DISABLE_POOL_IX_ACCOUNTS_LEN]> for EnableOrDisablePoolKeys {
    fn from(pubkeys: [Pubkey; ENABLE_OR_DISABLE_POOL_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            pool: pubkeys[0],
            admin: pubkeys[1],
        }
    }
}
impl<'info> From<EnableOrDisablePoolAccounts<'_, 'info>>
for [AccountInfo<'info>; ENABLE_OR_DISABLE_POOL_IX_ACCOUNTS_LEN] {
    fn from(accounts: EnableOrDisablePoolAccounts<'_, 'info>) -> Self {
        [accounts.pool.clone(), accounts.admin.clone()]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; ENABLE_OR_DISABLE_POOL_IX_ACCOUNTS_LEN]>
for EnableOrDisablePoolAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; ENABLE_OR_DISABLE_POOL_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            pool: &arr[0],
            admin: &arr[1],
        }
    }
}
pub const ENABLE_OR_DISABLE_POOL_IX_DISCM: [u8; 8] = [
    128,
    6,
    228,
    131,
    55,
    161,
    52,
    169,
];
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct EnableOrDisablePoolIxArgs {
    pub enable: bool,
}
#[derive(Clone, Debug, PartialEq)]
pub struct EnableOrDisablePoolIxData(pub EnableOrDisablePoolIxArgs);
impl From<EnableOrDisablePoolIxArgs> for EnableOrDisablePoolIxData {
    fn from(args: EnableOrDisablePoolIxArgs) -> Self {
        Self(args)
    }
}
impl EnableOrDisablePoolIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != ENABLE_OR_DISABLE_POOL_IX_DISCM {
            return Err(
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!(
                        "discm does not match. Expected: {:?}. Received: {:?}",
                        ENABLE_OR_DISABLE_POOL_IX_DISCM, maybe_discm
                    ),
                ),
            );
        }
        Ok(Self(EnableOrDisablePoolIxArgs::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&ENABLE_OR_DISABLE_POOL_IX_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn enable_or_disable_pool_ix_with_program_id(
    program_id: Pubkey,
    keys: EnableOrDisablePoolKeys,
    args: EnableOrDisablePoolIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; ENABLE_OR_DISABLE_POOL_IX_ACCOUNTS_LEN] = keys.into();
    let data: EnableOrDisablePoolIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn enable_or_disable_pool_ix(
    keys: EnableOrDisablePoolKeys,
    args: EnableOrDisablePoolIxArgs,
) -> std::io::Result<Instruction> {
    enable_or_disable_pool_ix_with_program_id(crate::ID, keys, args)
}
pub fn enable_or_disable_pool_invoke_with_program_id(
    program_id: Pubkey,
    accounts: EnableOrDisablePoolAccounts<'_, '_>,
    args: EnableOrDisablePoolIxArgs,
) -> ProgramResult {
    let keys: EnableOrDisablePoolKeys = accounts.into();
    let ix = enable_or_disable_pool_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn enable_or_disable_pool_invoke(
    accounts: EnableOrDisablePoolAccounts<'_, '_>,
    args: EnableOrDisablePoolIxArgs,
) -> ProgramResult {
    enable_or_disable_pool_invoke_with_program_id(crate::ID, accounts, args)
}
pub fn enable_or_disable_pool_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: EnableOrDisablePoolAccounts<'_, '_>,
    args: EnableOrDisablePoolIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: EnableOrDisablePoolKeys = accounts.into();
    let ix = enable_or_disable_pool_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn enable_or_disable_pool_invoke_signed(
    accounts: EnableOrDisablePoolAccounts<'_, '_>,
    args: EnableOrDisablePoolIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    enable_or_disable_pool_invoke_signed_with_program_id(
        crate::ID,
        accounts,
        args,
        seeds,
    )
}
pub fn enable_or_disable_pool_verify_account_keys(
    accounts: EnableOrDisablePoolAccounts<'_, '_>,
    keys: EnableOrDisablePoolKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.pool.key, keys.pool),
        (*accounts.admin.key, keys.admin),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn enable_or_disable_pool_verify_writable_privileges<'me, 'info>(
    accounts: EnableOrDisablePoolAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.pool] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn enable_or_disable_pool_verify_signer_privileges<'me, 'info>(
    accounts: EnableOrDisablePoolAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.admin] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn enable_or_disable_pool_verify_account_privileges<'me, 'info>(
    accounts: EnableOrDisablePoolAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    enable_or_disable_pool_verify_writable_privileges(accounts)?;
    enable_or_disable_pool_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const SWAP_IX_ACCOUNTS_LEN: usize = 15;
#[derive(Copy, Clone, Debug)]
pub struct SwapAccounts<'me, 'info> {
    pub pool: &'me AccountInfo<'info>,
    pub user_source_token: &'me AccountInfo<'info>,
    pub user_destination_token: &'me AccountInfo<'info>,
    pub a_vault: &'me AccountInfo<'info>,
    pub b_vault: &'me AccountInfo<'info>,
    pub a_token_vault: &'me AccountInfo<'info>,
    pub b_token_vault: &'me AccountInfo<'info>,
    pub a_vault_lp_mint: &'me AccountInfo<'info>,
    pub b_vault_lp_mint: &'me AccountInfo<'info>,
    pub a_vault_lp: &'me AccountInfo<'info>,
    pub b_vault_lp: &'me AccountInfo<'info>,
    pub protocol_token_fee: &'me AccountInfo<'info>,
    pub user: &'me AccountInfo<'info>,
    pub vault_program: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SwapKeys {
    pub pool: Pubkey,
    pub user_source_token: Pubkey,
    pub user_destination_token: Pubkey,
    pub a_vault: Pubkey,
    pub b_vault: Pubkey,
    pub a_token_vault: Pubkey,
    pub b_token_vault: Pubkey,
    pub a_vault_lp_mint: Pubkey,
    pub b_vault_lp_mint: Pubkey,
    pub a_vault_lp: Pubkey,
    pub b_vault_lp: Pubkey,
    pub protocol_token_fee: Pubkey,
    pub user: Pubkey,
    pub vault_program: Pubkey,
    pub token_program: Pubkey,
}
impl From<SwapAccounts<'_, '_>> for SwapKeys {
    fn from(accounts: SwapAccounts) -> Self {
        Self {
            pool: *accounts.pool.key,
            user_source_token: *accounts.user_source_token.key,
            user_destination_token: *accounts.user_destination_token.key,
            a_vault: *accounts.a_vault.key,
            b_vault: *accounts.b_vault.key,
            a_token_vault: *accounts.a_token_vault.key,
            b_token_vault: *accounts.b_token_vault.key,
            a_vault_lp_mint: *accounts.a_vault_lp_mint.key,
            b_vault_lp_mint: *accounts.b_vault_lp_mint.key,
            a_vault_lp: *accounts.a_vault_lp.key,
            b_vault_lp: *accounts.b_vault_lp.key,
            protocol_token_fee: *accounts.protocol_token_fee.key,
            user: *accounts.user.key,
            vault_program: *accounts.vault_program.key,
            token_program: *accounts.token_program.key,
        }
    }
}
impl From<SwapKeys> for [AccountMeta; SWAP_IX_ACCOUNTS_LEN] {
    fn from(keys: SwapKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.pool,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_source_token,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_destination_token,
                is_signer: false,
                is_writable: true,
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
                pubkey: keys.protocol_token_fee,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user,
                is_signer: true,
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
        ]
    }
}
impl From<[Pubkey; SWAP_IX_ACCOUNTS_LEN]> for SwapKeys {
    fn from(pubkeys: [Pubkey; SWAP_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            pool: pubkeys[0],
            user_source_token: pubkeys[1],
            user_destination_token: pubkeys[2],
            a_vault: pubkeys[3],
            b_vault: pubkeys[4],
            a_token_vault: pubkeys[5],
            b_token_vault: pubkeys[6],
            a_vault_lp_mint: pubkeys[7],
            b_vault_lp_mint: pubkeys[8],
            a_vault_lp: pubkeys[9],
            b_vault_lp: pubkeys[10],
            protocol_token_fee: pubkeys[11],
            user: pubkeys[12],
            vault_program: pubkeys[13],
            token_program: pubkeys[14],
        }
    }
}
impl<'info> From<SwapAccounts<'_, 'info>>
for [AccountInfo<'info>; SWAP_IX_ACCOUNTS_LEN] {
    fn from(accounts: SwapAccounts<'_, 'info>) -> Self {
        [
            accounts.pool.clone(),
            accounts.user_source_token.clone(),
            accounts.user_destination_token.clone(),
            accounts.a_vault.clone(),
            accounts.b_vault.clone(),
            accounts.a_token_vault.clone(),
            accounts.b_token_vault.clone(),
            accounts.a_vault_lp_mint.clone(),
            accounts.b_vault_lp_mint.clone(),
            accounts.a_vault_lp.clone(),
            accounts.b_vault_lp.clone(),
            accounts.protocol_token_fee.clone(),
            accounts.user.clone(),
            accounts.vault_program.clone(),
            accounts.token_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; SWAP_IX_ACCOUNTS_LEN]>
for SwapAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; SWAP_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            pool: &arr[0],
            user_source_token: &arr[1],
            user_destination_token: &arr[2],
            a_vault: &arr[3],
            b_vault: &arr[4],
            a_token_vault: &arr[5],
            b_token_vault: &arr[6],
            a_vault_lp_mint: &arr[7],
            b_vault_lp_mint: &arr[8],
            a_vault_lp: &arr[9],
            b_vault_lp: &arr[10],
            protocol_token_fee: &arr[11],
            user: &arr[12],
            vault_program: &arr[13],
            token_program: &arr[14],
        }
    }
}
pub const SWAP_IX_DISCM: [u8; 8] = [248, 198, 158, 145, 225, 117, 135, 200];
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SwapIxArgs {
    pub in_amount: u64,
    pub minimum_out_amount: u64,
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
        (*accounts.pool.key, keys.pool),
        (*accounts.user_source_token.key, keys.user_source_token),
        (*accounts.user_destination_token.key, keys.user_destination_token),
        (*accounts.a_vault.key, keys.a_vault),
        (*accounts.b_vault.key, keys.b_vault),
        (*accounts.a_token_vault.key, keys.a_token_vault),
        (*accounts.b_token_vault.key, keys.b_token_vault),
        (*accounts.a_vault_lp_mint.key, keys.a_vault_lp_mint),
        (*accounts.b_vault_lp_mint.key, keys.b_vault_lp_mint),
        (*accounts.a_vault_lp.key, keys.a_vault_lp),
        (*accounts.b_vault_lp.key, keys.b_vault_lp),
        (*accounts.protocol_token_fee.key, keys.protocol_token_fee),
        (*accounts.user.key, keys.user),
        (*accounts.vault_program.key, keys.vault_program),
        (*accounts.token_program.key, keys.token_program),
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
        accounts.user_source_token,
        accounts.user_destination_token,
        accounts.a_vault,
        accounts.b_vault,
        accounts.a_token_vault,
        accounts.b_token_vault,
        accounts.a_vault_lp_mint,
        accounts.b_vault_lp_mint,
        accounts.a_vault_lp,
        accounts.b_vault_lp,
        accounts.protocol_token_fee,
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
pub const REMOVE_LIQUIDITY_SINGLE_SIDE_IX_ACCOUNTS_LEN: usize = 15;
#[derive(Copy, Clone, Debug)]
pub struct RemoveLiquiditySingleSideAccounts<'me, 'info> {
    pub pool: &'me AccountInfo<'info>,
    pub lp_mint: &'me AccountInfo<'info>,
    pub user_pool_lp: &'me AccountInfo<'info>,
    pub a_vault_lp: &'me AccountInfo<'info>,
    pub b_vault_lp: &'me AccountInfo<'info>,
    pub a_vault: &'me AccountInfo<'info>,
    pub b_vault: &'me AccountInfo<'info>,
    pub a_vault_lp_mint: &'me AccountInfo<'info>,
    pub b_vault_lp_mint: &'me AccountInfo<'info>,
    pub a_token_vault: &'me AccountInfo<'info>,
    pub b_token_vault: &'me AccountInfo<'info>,
    pub user_destination_token: &'me AccountInfo<'info>,
    pub user: &'me AccountInfo<'info>,
    pub vault_program: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct RemoveLiquiditySingleSideKeys {
    pub pool: Pubkey,
    pub lp_mint: Pubkey,
    pub user_pool_lp: Pubkey,
    pub a_vault_lp: Pubkey,
    pub b_vault_lp: Pubkey,
    pub a_vault: Pubkey,
    pub b_vault: Pubkey,
    pub a_vault_lp_mint: Pubkey,
    pub b_vault_lp_mint: Pubkey,
    pub a_token_vault: Pubkey,
    pub b_token_vault: Pubkey,
    pub user_destination_token: Pubkey,
    pub user: Pubkey,
    pub vault_program: Pubkey,
    pub token_program: Pubkey,
}
impl From<RemoveLiquiditySingleSideAccounts<'_, '_>> for RemoveLiquiditySingleSideKeys {
    fn from(accounts: RemoveLiquiditySingleSideAccounts) -> Self {
        Self {
            pool: *accounts.pool.key,
            lp_mint: *accounts.lp_mint.key,
            user_pool_lp: *accounts.user_pool_lp.key,
            a_vault_lp: *accounts.a_vault_lp.key,
            b_vault_lp: *accounts.b_vault_lp.key,
            a_vault: *accounts.a_vault.key,
            b_vault: *accounts.b_vault.key,
            a_vault_lp_mint: *accounts.a_vault_lp_mint.key,
            b_vault_lp_mint: *accounts.b_vault_lp_mint.key,
            a_token_vault: *accounts.a_token_vault.key,
            b_token_vault: *accounts.b_token_vault.key,
            user_destination_token: *accounts.user_destination_token.key,
            user: *accounts.user.key,
            vault_program: *accounts.vault_program.key,
            token_program: *accounts.token_program.key,
        }
    }
}
impl From<RemoveLiquiditySingleSideKeys>
for [AccountMeta; REMOVE_LIQUIDITY_SINGLE_SIDE_IX_ACCOUNTS_LEN] {
    fn from(keys: RemoveLiquiditySingleSideKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.pool,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.lp_mint,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_pool_lp,
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
                pubkey: keys.user_destination_token,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user,
                is_signer: true,
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
        ]
    }
}
impl From<[Pubkey; REMOVE_LIQUIDITY_SINGLE_SIDE_IX_ACCOUNTS_LEN]>
for RemoveLiquiditySingleSideKeys {
    fn from(pubkeys: [Pubkey; REMOVE_LIQUIDITY_SINGLE_SIDE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            pool: pubkeys[0],
            lp_mint: pubkeys[1],
            user_pool_lp: pubkeys[2],
            a_vault_lp: pubkeys[3],
            b_vault_lp: pubkeys[4],
            a_vault: pubkeys[5],
            b_vault: pubkeys[6],
            a_vault_lp_mint: pubkeys[7],
            b_vault_lp_mint: pubkeys[8],
            a_token_vault: pubkeys[9],
            b_token_vault: pubkeys[10],
            user_destination_token: pubkeys[11],
            user: pubkeys[12],
            vault_program: pubkeys[13],
            token_program: pubkeys[14],
        }
    }
}
impl<'info> From<RemoveLiquiditySingleSideAccounts<'_, 'info>>
for [AccountInfo<'info>; REMOVE_LIQUIDITY_SINGLE_SIDE_IX_ACCOUNTS_LEN] {
    fn from(accounts: RemoveLiquiditySingleSideAccounts<'_, 'info>) -> Self {
        [
            accounts.pool.clone(),
            accounts.lp_mint.clone(),
            accounts.user_pool_lp.clone(),
            accounts.a_vault_lp.clone(),
            accounts.b_vault_lp.clone(),
            accounts.a_vault.clone(),
            accounts.b_vault.clone(),
            accounts.a_vault_lp_mint.clone(),
            accounts.b_vault_lp_mint.clone(),
            accounts.a_token_vault.clone(),
            accounts.b_token_vault.clone(),
            accounts.user_destination_token.clone(),
            accounts.user.clone(),
            accounts.vault_program.clone(),
            accounts.token_program.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; REMOVE_LIQUIDITY_SINGLE_SIDE_IX_ACCOUNTS_LEN]>
for RemoveLiquiditySingleSideAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; REMOVE_LIQUIDITY_SINGLE_SIDE_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            pool: &arr[0],
            lp_mint: &arr[1],
            user_pool_lp: &arr[2],
            a_vault_lp: &arr[3],
            b_vault_lp: &arr[4],
            a_vault: &arr[5],
            b_vault: &arr[6],
            a_vault_lp_mint: &arr[7],
            b_vault_lp_mint: &arr[8],
            a_token_vault: &arr[9],
            b_token_vault: &arr[10],
            user_destination_token: &arr[11],
            user: &arr[12],
            vault_program: &arr[13],
            token_program: &arr[14],
        }
    }
}
pub const REMOVE_LIQUIDITY_SINGLE_SIDE_IX_DISCM: [u8; 8] = [
    84,
    84,
    177,
    66,
    254,
    185,
    10,
    251,
];
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RemoveLiquiditySingleSideIxArgs {
    pub pool_token_amount: u64,
    pub minimum_out_amount: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct RemoveLiquiditySingleSideIxData(pub RemoveLiquiditySingleSideIxArgs);
impl From<RemoveLiquiditySingleSideIxArgs> for RemoveLiquiditySingleSideIxData {
    fn from(args: RemoveLiquiditySingleSideIxArgs) -> Self {
        Self(args)
    }
}
impl RemoveLiquiditySingleSideIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != REMOVE_LIQUIDITY_SINGLE_SIDE_IX_DISCM {
            return Err(
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!(
                        "discm does not match. Expected: {:?}. Received: {:?}",
                        REMOVE_LIQUIDITY_SINGLE_SIDE_IX_DISCM, maybe_discm
                    ),
                ),
            );
        }
        Ok(Self(RemoveLiquiditySingleSideIxArgs::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&REMOVE_LIQUIDITY_SINGLE_SIDE_IX_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn remove_liquidity_single_side_ix_with_program_id(
    program_id: Pubkey,
    keys: RemoveLiquiditySingleSideKeys,
    args: RemoveLiquiditySingleSideIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; REMOVE_LIQUIDITY_SINGLE_SIDE_IX_ACCOUNTS_LEN] = keys.into();
    let data: RemoveLiquiditySingleSideIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn remove_liquidity_single_side_ix(
    keys: RemoveLiquiditySingleSideKeys,
    args: RemoveLiquiditySingleSideIxArgs,
) -> std::io::Result<Instruction> {
    remove_liquidity_single_side_ix_with_program_id(crate::ID, keys, args)
}
pub fn remove_liquidity_single_side_invoke_with_program_id(
    program_id: Pubkey,
    accounts: RemoveLiquiditySingleSideAccounts<'_, '_>,
    args: RemoveLiquiditySingleSideIxArgs,
) -> ProgramResult {
    let keys: RemoveLiquiditySingleSideKeys = accounts.into();
    let ix = remove_liquidity_single_side_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn remove_liquidity_single_side_invoke(
    accounts: RemoveLiquiditySingleSideAccounts<'_, '_>,
    args: RemoveLiquiditySingleSideIxArgs,
) -> ProgramResult {
    remove_liquidity_single_side_invoke_with_program_id(crate::ID, accounts, args)
}
pub fn remove_liquidity_single_side_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: RemoveLiquiditySingleSideAccounts<'_, '_>,
    args: RemoveLiquiditySingleSideIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: RemoveLiquiditySingleSideKeys = accounts.into();
    let ix = remove_liquidity_single_side_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn remove_liquidity_single_side_invoke_signed(
    accounts: RemoveLiquiditySingleSideAccounts<'_, '_>,
    args: RemoveLiquiditySingleSideIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    remove_liquidity_single_side_invoke_signed_with_program_id(
        crate::ID,
        accounts,
        args,
        seeds,
    )
}
pub fn remove_liquidity_single_side_verify_account_keys(
    accounts: RemoveLiquiditySingleSideAccounts<'_, '_>,
    keys: RemoveLiquiditySingleSideKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.pool.key, keys.pool),
        (*accounts.lp_mint.key, keys.lp_mint),
        (*accounts.user_pool_lp.key, keys.user_pool_lp),
        (*accounts.a_vault_lp.key, keys.a_vault_lp),
        (*accounts.b_vault_lp.key, keys.b_vault_lp),
        (*accounts.a_vault.key, keys.a_vault),
        (*accounts.b_vault.key, keys.b_vault),
        (*accounts.a_vault_lp_mint.key, keys.a_vault_lp_mint),
        (*accounts.b_vault_lp_mint.key, keys.b_vault_lp_mint),
        (*accounts.a_token_vault.key, keys.a_token_vault),
        (*accounts.b_token_vault.key, keys.b_token_vault),
        (*accounts.user_destination_token.key, keys.user_destination_token),
        (*accounts.user.key, keys.user),
        (*accounts.vault_program.key, keys.vault_program),
        (*accounts.token_program.key, keys.token_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn remove_liquidity_single_side_verify_writable_privileges<'me, 'info>(
    accounts: RemoveLiquiditySingleSideAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.pool,
        accounts.lp_mint,
        accounts.user_pool_lp,
        accounts.a_vault_lp,
        accounts.b_vault_lp,
        accounts.a_vault,
        accounts.b_vault,
        accounts.a_vault_lp_mint,
        accounts.b_vault_lp_mint,
        accounts.a_token_vault,
        accounts.b_token_vault,
        accounts.user_destination_token,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn remove_liquidity_single_side_verify_signer_privileges<'me, 'info>(
    accounts: RemoveLiquiditySingleSideAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.user] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn remove_liquidity_single_side_verify_account_privileges<'me, 'info>(
    accounts: RemoveLiquiditySingleSideAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    remove_liquidity_single_side_verify_writable_privileges(accounts)?;
    remove_liquidity_single_side_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const ADD_IMBALANCE_LIQUIDITY_IX_ACCOUNTS_LEN: usize = 16;
#[derive(Copy, Clone, Debug)]
pub struct AddImbalanceLiquidityAccounts<'me, 'info> {
    pub pool: &'me AccountInfo<'info>,
    pub lp_mint: &'me AccountInfo<'info>,
    pub user_pool_lp: &'me AccountInfo<'info>,
    pub a_vault_lp: &'me AccountInfo<'info>,
    pub b_vault_lp: &'me AccountInfo<'info>,
    pub a_vault: &'me AccountInfo<'info>,
    pub b_vault: &'me AccountInfo<'info>,
    pub a_vault_lp_mint: &'me AccountInfo<'info>,
    pub b_vault_lp_mint: &'me AccountInfo<'info>,
    pub a_token_vault: &'me AccountInfo<'info>,
    pub b_token_vault: &'me AccountInfo<'info>,
    pub user_a_token: &'me AccountInfo<'info>,
    pub user_b_token: &'me AccountInfo<'info>,
    pub user: &'me AccountInfo<'info>,
    pub vault_program: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct AddImbalanceLiquidityKeys {
    pub pool: Pubkey,
    pub lp_mint: Pubkey,
    pub user_pool_lp: Pubkey,
    pub a_vault_lp: Pubkey,
    pub b_vault_lp: Pubkey,
    pub a_vault: Pubkey,
    pub b_vault: Pubkey,
    pub a_vault_lp_mint: Pubkey,
    pub b_vault_lp_mint: Pubkey,
    pub a_token_vault: Pubkey,
    pub b_token_vault: Pubkey,
    pub user_a_token: Pubkey,
    pub user_b_token: Pubkey,
    pub user: Pubkey,
    pub vault_program: Pubkey,
    pub token_program: Pubkey,
}
impl From<AddImbalanceLiquidityAccounts<'_, '_>> for AddImbalanceLiquidityKeys {
    fn from(accounts: AddImbalanceLiquidityAccounts) -> Self {
        Self {
            pool: *accounts.pool.key,
            lp_mint: *accounts.lp_mint.key,
            user_pool_lp: *accounts.user_pool_lp.key,
            a_vault_lp: *accounts.a_vault_lp.key,
            b_vault_lp: *accounts.b_vault_lp.key,
            a_vault: *accounts.a_vault.key,
            b_vault: *accounts.b_vault.key,
            a_vault_lp_mint: *accounts.a_vault_lp_mint.key,
            b_vault_lp_mint: *accounts.b_vault_lp_mint.key,
            a_token_vault: *accounts.a_token_vault.key,
            b_token_vault: *accounts.b_token_vault.key,
            user_a_token: *accounts.user_a_token.key,
            user_b_token: *accounts.user_b_token.key,
            user: *accounts.user.key,
            vault_program: *accounts.vault_program.key,
            token_program: *accounts.token_program.key,
        }
    }
}
impl From<AddImbalanceLiquidityKeys>
for [AccountMeta; ADD_IMBALANCE_LIQUIDITY_IX_ACCOUNTS_LEN] {
    fn from(keys: AddImbalanceLiquidityKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.pool,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.lp_mint,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_pool_lp,
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
                pubkey: keys.user_a_token,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_b_token,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user,
                is_signer: true,
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
        ]
    }
}
impl From<[Pubkey; ADD_IMBALANCE_LIQUIDITY_IX_ACCOUNTS_LEN]>
for AddImbalanceLiquidityKeys {
    fn from(pubkeys: [Pubkey; ADD_IMBALANCE_LIQUIDITY_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            pool: pubkeys[0],
            lp_mint: pubkeys[1],
            user_pool_lp: pubkeys[2],
            a_vault_lp: pubkeys[3],
            b_vault_lp: pubkeys[4],
            a_vault: pubkeys[5],
            b_vault: pubkeys[6],
            a_vault_lp_mint: pubkeys[7],
            b_vault_lp_mint: pubkeys[8],
            a_token_vault: pubkeys[9],
            b_token_vault: pubkeys[10],
            user_a_token: pubkeys[11],
            user_b_token: pubkeys[12],
            user: pubkeys[13],
            vault_program: pubkeys[14],
            token_program: pubkeys[15],
        }
    }
}
impl<'info> From<AddImbalanceLiquidityAccounts<'_, 'info>>
for [AccountInfo<'info>; ADD_IMBALANCE_LIQUIDITY_IX_ACCOUNTS_LEN] {
    fn from(accounts: AddImbalanceLiquidityAccounts<'_, 'info>) -> Self {
        [
            accounts.pool.clone(),
            accounts.lp_mint.clone(),
            accounts.user_pool_lp.clone(),
            accounts.a_vault_lp.clone(),
            accounts.b_vault_lp.clone(),
            accounts.a_vault.clone(),
            accounts.b_vault.clone(),
            accounts.a_vault_lp_mint.clone(),
            accounts.b_vault_lp_mint.clone(),
            accounts.a_token_vault.clone(),
            accounts.b_token_vault.clone(),
            accounts.user_a_token.clone(),
            accounts.user_b_token.clone(),
            accounts.user.clone(),
            accounts.vault_program.clone(),
            accounts.token_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; ADD_IMBALANCE_LIQUIDITY_IX_ACCOUNTS_LEN]>
for AddImbalanceLiquidityAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; ADD_IMBALANCE_LIQUIDITY_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            pool: &arr[0],
            lp_mint: &arr[1],
            user_pool_lp: &arr[2],
            a_vault_lp: &arr[3],
            b_vault_lp: &arr[4],
            a_vault: &arr[5],
            b_vault: &arr[6],
            a_vault_lp_mint: &arr[7],
            b_vault_lp_mint: &arr[8],
            a_token_vault: &arr[9],
            b_token_vault: &arr[10],
            user_a_token: &arr[11],
            user_b_token: &arr[12],
            user: &arr[13],
            vault_program: &arr[14],
            token_program: &arr[15],
        }
    }
}
pub const ADD_IMBALANCE_LIQUIDITY_IX_DISCM: [u8; 8] = [
    79,
    35,
    122,
    84,
    173,
    15,
    93,
    191,
];
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct AddImbalanceLiquidityIxArgs {
    pub minimum_pool_token_amount: u64,
    pub token_a_amount: u64,
    pub token_b_amount: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct AddImbalanceLiquidityIxData(pub AddImbalanceLiquidityIxArgs);
impl From<AddImbalanceLiquidityIxArgs> for AddImbalanceLiquidityIxData {
    fn from(args: AddImbalanceLiquidityIxArgs) -> Self {
        Self(args)
    }
}
impl AddImbalanceLiquidityIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != ADD_IMBALANCE_LIQUIDITY_IX_DISCM {
            return Err(
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!(
                        "discm does not match. Expected: {:?}. Received: {:?}",
                        ADD_IMBALANCE_LIQUIDITY_IX_DISCM, maybe_discm
                    ),
                ),
            );
        }
        Ok(Self(AddImbalanceLiquidityIxArgs::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&ADD_IMBALANCE_LIQUIDITY_IX_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn add_imbalance_liquidity_ix_with_program_id(
    program_id: Pubkey,
    keys: AddImbalanceLiquidityKeys,
    args: AddImbalanceLiquidityIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; ADD_IMBALANCE_LIQUIDITY_IX_ACCOUNTS_LEN] = keys.into();
    let data: AddImbalanceLiquidityIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn add_imbalance_liquidity_ix(
    keys: AddImbalanceLiquidityKeys,
    args: AddImbalanceLiquidityIxArgs,
) -> std::io::Result<Instruction> {
    add_imbalance_liquidity_ix_with_program_id(crate::ID, keys, args)
}
pub fn add_imbalance_liquidity_invoke_with_program_id(
    program_id: Pubkey,
    accounts: AddImbalanceLiquidityAccounts<'_, '_>,
    args: AddImbalanceLiquidityIxArgs,
) -> ProgramResult {
    let keys: AddImbalanceLiquidityKeys = accounts.into();
    let ix = add_imbalance_liquidity_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn add_imbalance_liquidity_invoke(
    accounts: AddImbalanceLiquidityAccounts<'_, '_>,
    args: AddImbalanceLiquidityIxArgs,
) -> ProgramResult {
    add_imbalance_liquidity_invoke_with_program_id(crate::ID, accounts, args)
}
pub fn add_imbalance_liquidity_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: AddImbalanceLiquidityAccounts<'_, '_>,
    args: AddImbalanceLiquidityIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: AddImbalanceLiquidityKeys = accounts.into();
    let ix = add_imbalance_liquidity_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn add_imbalance_liquidity_invoke_signed(
    accounts: AddImbalanceLiquidityAccounts<'_, '_>,
    args: AddImbalanceLiquidityIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    add_imbalance_liquidity_invoke_signed_with_program_id(
        crate::ID,
        accounts,
        args,
        seeds,
    )
}
pub fn add_imbalance_liquidity_verify_account_keys(
    accounts: AddImbalanceLiquidityAccounts<'_, '_>,
    keys: AddImbalanceLiquidityKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.pool.key, keys.pool),
        (*accounts.lp_mint.key, keys.lp_mint),
        (*accounts.user_pool_lp.key, keys.user_pool_lp),
        (*accounts.a_vault_lp.key, keys.a_vault_lp),
        (*accounts.b_vault_lp.key, keys.b_vault_lp),
        (*accounts.a_vault.key, keys.a_vault),
        (*accounts.b_vault.key, keys.b_vault),
        (*accounts.a_vault_lp_mint.key, keys.a_vault_lp_mint),
        (*accounts.b_vault_lp_mint.key, keys.b_vault_lp_mint),
        (*accounts.a_token_vault.key, keys.a_token_vault),
        (*accounts.b_token_vault.key, keys.b_token_vault),
        (*accounts.user_a_token.key, keys.user_a_token),
        (*accounts.user_b_token.key, keys.user_b_token),
        (*accounts.user.key, keys.user),
        (*accounts.vault_program.key, keys.vault_program),
        (*accounts.token_program.key, keys.token_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn add_imbalance_liquidity_verify_writable_privileges<'me, 'info>(
    accounts: AddImbalanceLiquidityAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.pool,
        accounts.lp_mint,
        accounts.user_pool_lp,
        accounts.a_vault_lp,
        accounts.b_vault_lp,
        accounts.a_vault,
        accounts.b_vault,
        accounts.a_vault_lp_mint,
        accounts.b_vault_lp_mint,
        accounts.a_token_vault,
        accounts.b_token_vault,
        accounts.user_a_token,
        accounts.user_b_token,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn add_imbalance_liquidity_verify_signer_privileges<'me, 'info>(
    accounts: AddImbalanceLiquidityAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.user] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn add_imbalance_liquidity_verify_account_privileges<'me, 'info>(
    accounts: AddImbalanceLiquidityAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    add_imbalance_liquidity_verify_writable_privileges(accounts)?;
    add_imbalance_liquidity_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const REMOVE_BALANCE_LIQUIDITY_IX_ACCOUNTS_LEN: usize = 16;
#[derive(Copy, Clone, Debug)]
pub struct RemoveBalanceLiquidityAccounts<'me, 'info> {
    pub pool: &'me AccountInfo<'info>,
    pub lp_mint: &'me AccountInfo<'info>,
    pub user_pool_lp: &'me AccountInfo<'info>,
    pub a_vault_lp: &'me AccountInfo<'info>,
    pub b_vault_lp: &'me AccountInfo<'info>,
    pub a_vault: &'me AccountInfo<'info>,
    pub b_vault: &'me AccountInfo<'info>,
    pub a_vault_lp_mint: &'me AccountInfo<'info>,
    pub b_vault_lp_mint: &'me AccountInfo<'info>,
    pub a_token_vault: &'me AccountInfo<'info>,
    pub b_token_vault: &'me AccountInfo<'info>,
    pub user_a_token: &'me AccountInfo<'info>,
    pub user_b_token: &'me AccountInfo<'info>,
    pub user: &'me AccountInfo<'info>,
    pub vault_program: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct RemoveBalanceLiquidityKeys {
    pub pool: Pubkey,
    pub lp_mint: Pubkey,
    pub user_pool_lp: Pubkey,
    pub a_vault_lp: Pubkey,
    pub b_vault_lp: Pubkey,
    pub a_vault: Pubkey,
    pub b_vault: Pubkey,
    pub a_vault_lp_mint: Pubkey,
    pub b_vault_lp_mint: Pubkey,
    pub a_token_vault: Pubkey,
    pub b_token_vault: Pubkey,
    pub user_a_token: Pubkey,
    pub user_b_token: Pubkey,
    pub user: Pubkey,
    pub vault_program: Pubkey,
    pub token_program: Pubkey,
}
impl From<RemoveBalanceLiquidityAccounts<'_, '_>> for RemoveBalanceLiquidityKeys {
    fn from(accounts: RemoveBalanceLiquidityAccounts) -> Self {
        Self {
            pool: *accounts.pool.key,
            lp_mint: *accounts.lp_mint.key,
            user_pool_lp: *accounts.user_pool_lp.key,
            a_vault_lp: *accounts.a_vault_lp.key,
            b_vault_lp: *accounts.b_vault_lp.key,
            a_vault: *accounts.a_vault.key,
            b_vault: *accounts.b_vault.key,
            a_vault_lp_mint: *accounts.a_vault_lp_mint.key,
            b_vault_lp_mint: *accounts.b_vault_lp_mint.key,
            a_token_vault: *accounts.a_token_vault.key,
            b_token_vault: *accounts.b_token_vault.key,
            user_a_token: *accounts.user_a_token.key,
            user_b_token: *accounts.user_b_token.key,
            user: *accounts.user.key,
            vault_program: *accounts.vault_program.key,
            token_program: *accounts.token_program.key,
        }
    }
}
impl From<RemoveBalanceLiquidityKeys>
for [AccountMeta; REMOVE_BALANCE_LIQUIDITY_IX_ACCOUNTS_LEN] {
    fn from(keys: RemoveBalanceLiquidityKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.pool,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.lp_mint,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_pool_lp,
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
                pubkey: keys.user_a_token,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_b_token,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user,
                is_signer: true,
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
        ]
    }
}
impl From<[Pubkey; REMOVE_BALANCE_LIQUIDITY_IX_ACCOUNTS_LEN]>
for RemoveBalanceLiquidityKeys {
    fn from(pubkeys: [Pubkey; REMOVE_BALANCE_LIQUIDITY_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            pool: pubkeys[0],
            lp_mint: pubkeys[1],
            user_pool_lp: pubkeys[2],
            a_vault_lp: pubkeys[3],
            b_vault_lp: pubkeys[4],
            a_vault: pubkeys[5],
            b_vault: pubkeys[6],
            a_vault_lp_mint: pubkeys[7],
            b_vault_lp_mint: pubkeys[8],
            a_token_vault: pubkeys[9],
            b_token_vault: pubkeys[10],
            user_a_token: pubkeys[11],
            user_b_token: pubkeys[12],
            user: pubkeys[13],
            vault_program: pubkeys[14],
            token_program: pubkeys[15],
        }
    }
}
impl<'info> From<RemoveBalanceLiquidityAccounts<'_, 'info>>
for [AccountInfo<'info>; REMOVE_BALANCE_LIQUIDITY_IX_ACCOUNTS_LEN] {
    fn from(accounts: RemoveBalanceLiquidityAccounts<'_, 'info>) -> Self {
        [
            accounts.pool.clone(),
            accounts.lp_mint.clone(),
            accounts.user_pool_lp.clone(),
            accounts.a_vault_lp.clone(),
            accounts.b_vault_lp.clone(),
            accounts.a_vault.clone(),
            accounts.b_vault.clone(),
            accounts.a_vault_lp_mint.clone(),
            accounts.b_vault_lp_mint.clone(),
            accounts.a_token_vault.clone(),
            accounts.b_token_vault.clone(),
            accounts.user_a_token.clone(),
            accounts.user_b_token.clone(),
            accounts.user.clone(),
            accounts.vault_program.clone(),
            accounts.token_program.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; REMOVE_BALANCE_LIQUIDITY_IX_ACCOUNTS_LEN]>
for RemoveBalanceLiquidityAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; REMOVE_BALANCE_LIQUIDITY_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            pool: &arr[0],
            lp_mint: &arr[1],
            user_pool_lp: &arr[2],
            a_vault_lp: &arr[3],
            b_vault_lp: &arr[4],
            a_vault: &arr[5],
            b_vault: &arr[6],
            a_vault_lp_mint: &arr[7],
            b_vault_lp_mint: &arr[8],
            a_token_vault: &arr[9],
            b_token_vault: &arr[10],
            user_a_token: &arr[11],
            user_b_token: &arr[12],
            user: &arr[13],
            vault_program: &arr[14],
            token_program: &arr[15],
        }
    }
}
pub const REMOVE_BALANCE_LIQUIDITY_IX_DISCM: [u8; 8] = [
    133,
    109,
    44,
    179,
    56,
    238,
    114,
    33,
];
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RemoveBalanceLiquidityIxArgs {
    pub pool_token_amount: u64,
    pub minimum_a_token_out: u64,
    pub minimum_b_token_out: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct RemoveBalanceLiquidityIxData(pub RemoveBalanceLiquidityIxArgs);
impl From<RemoveBalanceLiquidityIxArgs> for RemoveBalanceLiquidityIxData {
    fn from(args: RemoveBalanceLiquidityIxArgs) -> Self {
        Self(args)
    }
}
impl RemoveBalanceLiquidityIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != REMOVE_BALANCE_LIQUIDITY_IX_DISCM {
            return Err(
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!(
                        "discm does not match. Expected: {:?}. Received: {:?}",
                        REMOVE_BALANCE_LIQUIDITY_IX_DISCM, maybe_discm
                    ),
                ),
            );
        }
        Ok(Self(RemoveBalanceLiquidityIxArgs::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&REMOVE_BALANCE_LIQUIDITY_IX_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn remove_balance_liquidity_ix_with_program_id(
    program_id: Pubkey,
    keys: RemoveBalanceLiquidityKeys,
    args: RemoveBalanceLiquidityIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; REMOVE_BALANCE_LIQUIDITY_IX_ACCOUNTS_LEN] = keys.into();
    let data: RemoveBalanceLiquidityIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn remove_balance_liquidity_ix(
    keys: RemoveBalanceLiquidityKeys,
    args: RemoveBalanceLiquidityIxArgs,
) -> std::io::Result<Instruction> {
    remove_balance_liquidity_ix_with_program_id(crate::ID, keys, args)
}
pub fn remove_balance_liquidity_invoke_with_program_id(
    program_id: Pubkey,
    accounts: RemoveBalanceLiquidityAccounts<'_, '_>,
    args: RemoveBalanceLiquidityIxArgs,
) -> ProgramResult {
    let keys: RemoveBalanceLiquidityKeys = accounts.into();
    let ix = remove_balance_liquidity_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn remove_balance_liquidity_invoke(
    accounts: RemoveBalanceLiquidityAccounts<'_, '_>,
    args: RemoveBalanceLiquidityIxArgs,
) -> ProgramResult {
    remove_balance_liquidity_invoke_with_program_id(crate::ID, accounts, args)
}
pub fn remove_balance_liquidity_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: RemoveBalanceLiquidityAccounts<'_, '_>,
    args: RemoveBalanceLiquidityIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: RemoveBalanceLiquidityKeys = accounts.into();
    let ix = remove_balance_liquidity_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn remove_balance_liquidity_invoke_signed(
    accounts: RemoveBalanceLiquidityAccounts<'_, '_>,
    args: RemoveBalanceLiquidityIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    remove_balance_liquidity_invoke_signed_with_program_id(
        crate::ID,
        accounts,
        args,
        seeds,
    )
}
pub fn remove_balance_liquidity_verify_account_keys(
    accounts: RemoveBalanceLiquidityAccounts<'_, '_>,
    keys: RemoveBalanceLiquidityKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.pool.key, keys.pool),
        (*accounts.lp_mint.key, keys.lp_mint),
        (*accounts.user_pool_lp.key, keys.user_pool_lp),
        (*accounts.a_vault_lp.key, keys.a_vault_lp),
        (*accounts.b_vault_lp.key, keys.b_vault_lp),
        (*accounts.a_vault.key, keys.a_vault),
        (*accounts.b_vault.key, keys.b_vault),
        (*accounts.a_vault_lp_mint.key, keys.a_vault_lp_mint),
        (*accounts.b_vault_lp_mint.key, keys.b_vault_lp_mint),
        (*accounts.a_token_vault.key, keys.a_token_vault),
        (*accounts.b_token_vault.key, keys.b_token_vault),
        (*accounts.user_a_token.key, keys.user_a_token),
        (*accounts.user_b_token.key, keys.user_b_token),
        (*accounts.user.key, keys.user),
        (*accounts.vault_program.key, keys.vault_program),
        (*accounts.token_program.key, keys.token_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn remove_balance_liquidity_verify_writable_privileges<'me, 'info>(
    accounts: RemoveBalanceLiquidityAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.pool,
        accounts.lp_mint,
        accounts.user_pool_lp,
        accounts.a_vault_lp,
        accounts.b_vault_lp,
        accounts.a_vault,
        accounts.b_vault,
        accounts.a_vault_lp_mint,
        accounts.b_vault_lp_mint,
        accounts.a_token_vault,
        accounts.b_token_vault,
        accounts.user_a_token,
        accounts.user_b_token,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn remove_balance_liquidity_verify_signer_privileges<'me, 'info>(
    accounts: RemoveBalanceLiquidityAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.user] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn remove_balance_liquidity_verify_account_privileges<'me, 'info>(
    accounts: RemoveBalanceLiquidityAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    remove_balance_liquidity_verify_writable_privileges(accounts)?;
    remove_balance_liquidity_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const ADD_BALANCE_LIQUIDITY_IX_ACCOUNTS_LEN: usize = 16;
#[derive(Copy, Clone, Debug)]
pub struct AddBalanceLiquidityAccounts<'me, 'info> {
    pub pool: &'me AccountInfo<'info>,
    pub lp_mint: &'me AccountInfo<'info>,
    pub user_pool_lp: &'me AccountInfo<'info>,
    pub a_vault_lp: &'me AccountInfo<'info>,
    pub b_vault_lp: &'me AccountInfo<'info>,
    pub a_vault: &'me AccountInfo<'info>,
    pub b_vault: &'me AccountInfo<'info>,
    pub a_vault_lp_mint: &'me AccountInfo<'info>,
    pub b_vault_lp_mint: &'me AccountInfo<'info>,
    pub a_token_vault: &'me AccountInfo<'info>,
    pub b_token_vault: &'me AccountInfo<'info>,
    pub user_a_token: &'me AccountInfo<'info>,
    pub user_b_token: &'me AccountInfo<'info>,
    pub user: &'me AccountInfo<'info>,
    pub vault_program: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct AddBalanceLiquidityKeys {
    pub pool: Pubkey,
    pub lp_mint: Pubkey,
    pub user_pool_lp: Pubkey,
    pub a_vault_lp: Pubkey,
    pub b_vault_lp: Pubkey,
    pub a_vault: Pubkey,
    pub b_vault: Pubkey,
    pub a_vault_lp_mint: Pubkey,
    pub b_vault_lp_mint: Pubkey,
    pub a_token_vault: Pubkey,
    pub b_token_vault: Pubkey,
    pub user_a_token: Pubkey,
    pub user_b_token: Pubkey,
    pub user: Pubkey,
    pub vault_program: Pubkey,
    pub token_program: Pubkey,
}
impl From<AddBalanceLiquidityAccounts<'_, '_>> for AddBalanceLiquidityKeys {
    fn from(accounts: AddBalanceLiquidityAccounts) -> Self {
        Self {
            pool: *accounts.pool.key,
            lp_mint: *accounts.lp_mint.key,
            user_pool_lp: *accounts.user_pool_lp.key,
            a_vault_lp: *accounts.a_vault_lp.key,
            b_vault_lp: *accounts.b_vault_lp.key,
            a_vault: *accounts.a_vault.key,
            b_vault: *accounts.b_vault.key,
            a_vault_lp_mint: *accounts.a_vault_lp_mint.key,
            b_vault_lp_mint: *accounts.b_vault_lp_mint.key,
            a_token_vault: *accounts.a_token_vault.key,
            b_token_vault: *accounts.b_token_vault.key,
            user_a_token: *accounts.user_a_token.key,
            user_b_token: *accounts.user_b_token.key,
            user: *accounts.user.key,
            vault_program: *accounts.vault_program.key,
            token_program: *accounts.token_program.key,
        }
    }
}
impl From<AddBalanceLiquidityKeys>
for [AccountMeta; ADD_BALANCE_LIQUIDITY_IX_ACCOUNTS_LEN] {
    fn from(keys: AddBalanceLiquidityKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.pool,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.lp_mint,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_pool_lp,
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
                pubkey: keys.user_a_token,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_b_token,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user,
                is_signer: true,
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
        ]
    }
}
impl From<[Pubkey; ADD_BALANCE_LIQUIDITY_IX_ACCOUNTS_LEN]> for AddBalanceLiquidityKeys {
    fn from(pubkeys: [Pubkey; ADD_BALANCE_LIQUIDITY_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            pool: pubkeys[0],
            lp_mint: pubkeys[1],
            user_pool_lp: pubkeys[2],
            a_vault_lp: pubkeys[3],
            b_vault_lp: pubkeys[4],
            a_vault: pubkeys[5],
            b_vault: pubkeys[6],
            a_vault_lp_mint: pubkeys[7],
            b_vault_lp_mint: pubkeys[8],
            a_token_vault: pubkeys[9],
            b_token_vault: pubkeys[10],
            user_a_token: pubkeys[11],
            user_b_token: pubkeys[12],
            user: pubkeys[13],
            vault_program: pubkeys[14],
            token_program: pubkeys[15],
        }
    }
}
impl<'info> From<AddBalanceLiquidityAccounts<'_, 'info>>
for [AccountInfo<'info>; ADD_BALANCE_LIQUIDITY_IX_ACCOUNTS_LEN] {
    fn from(accounts: AddBalanceLiquidityAccounts<'_, 'info>) -> Self {
        [
            accounts.pool.clone(),
            accounts.lp_mint.clone(),
            accounts.user_pool_lp.clone(),
            accounts.a_vault_lp.clone(),
            accounts.b_vault_lp.clone(),
            accounts.a_vault.clone(),
            accounts.b_vault.clone(),
            accounts.a_vault_lp_mint.clone(),
            accounts.b_vault_lp_mint.clone(),
            accounts.a_token_vault.clone(),
            accounts.b_token_vault.clone(),
            accounts.user_a_token.clone(),
            accounts.user_b_token.clone(),
            accounts.user.clone(),
            accounts.vault_program.clone(),
            accounts.token_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; ADD_BALANCE_LIQUIDITY_IX_ACCOUNTS_LEN]>
for AddBalanceLiquidityAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; ADD_BALANCE_LIQUIDITY_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            pool: &arr[0],
            lp_mint: &arr[1],
            user_pool_lp: &arr[2],
            a_vault_lp: &arr[3],
            b_vault_lp: &arr[4],
            a_vault: &arr[5],
            b_vault: &arr[6],
            a_vault_lp_mint: &arr[7],
            b_vault_lp_mint: &arr[8],
            a_token_vault: &arr[9],
            b_token_vault: &arr[10],
            user_a_token: &arr[11],
            user_b_token: &arr[12],
            user: &arr[13],
            vault_program: &arr[14],
            token_program: &arr[15],
        }
    }
}
pub const ADD_BALANCE_LIQUIDITY_IX_DISCM: [u8; 8] = [
    168,
    227,
    50,
    62,
    189,
    171,
    84,
    176,
];
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct AddBalanceLiquidityIxArgs {
    pub pool_token_amount: u64,
    pub maximum_token_a_amount: u64,
    pub maximum_token_b_amount: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct AddBalanceLiquidityIxData(pub AddBalanceLiquidityIxArgs);
impl From<AddBalanceLiquidityIxArgs> for AddBalanceLiquidityIxData {
    fn from(args: AddBalanceLiquidityIxArgs) -> Self {
        Self(args)
    }
}
impl AddBalanceLiquidityIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != ADD_BALANCE_LIQUIDITY_IX_DISCM {
            return Err(
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!(
                        "discm does not match. Expected: {:?}. Received: {:?}",
                        ADD_BALANCE_LIQUIDITY_IX_DISCM, maybe_discm
                    ),
                ),
            );
        }
        Ok(Self(AddBalanceLiquidityIxArgs::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&ADD_BALANCE_LIQUIDITY_IX_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn add_balance_liquidity_ix_with_program_id(
    program_id: Pubkey,
    keys: AddBalanceLiquidityKeys,
    args: AddBalanceLiquidityIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; ADD_BALANCE_LIQUIDITY_IX_ACCOUNTS_LEN] = keys.into();
    let data: AddBalanceLiquidityIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn add_balance_liquidity_ix(
    keys: AddBalanceLiquidityKeys,
    args: AddBalanceLiquidityIxArgs,
) -> std::io::Result<Instruction> {
    add_balance_liquidity_ix_with_program_id(crate::ID, keys, args)
}
pub fn add_balance_liquidity_invoke_with_program_id(
    program_id: Pubkey,
    accounts: AddBalanceLiquidityAccounts<'_, '_>,
    args: AddBalanceLiquidityIxArgs,
) -> ProgramResult {
    let keys: AddBalanceLiquidityKeys = accounts.into();
    let ix = add_balance_liquidity_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn add_balance_liquidity_invoke(
    accounts: AddBalanceLiquidityAccounts<'_, '_>,
    args: AddBalanceLiquidityIxArgs,
) -> ProgramResult {
    add_balance_liquidity_invoke_with_program_id(crate::ID, accounts, args)
}
pub fn add_balance_liquidity_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: AddBalanceLiquidityAccounts<'_, '_>,
    args: AddBalanceLiquidityIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: AddBalanceLiquidityKeys = accounts.into();
    let ix = add_balance_liquidity_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn add_balance_liquidity_invoke_signed(
    accounts: AddBalanceLiquidityAccounts<'_, '_>,
    args: AddBalanceLiquidityIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    add_balance_liquidity_invoke_signed_with_program_id(crate::ID, accounts, args, seeds)
}
pub fn add_balance_liquidity_verify_account_keys(
    accounts: AddBalanceLiquidityAccounts<'_, '_>,
    keys: AddBalanceLiquidityKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.pool.key, keys.pool),
        (*accounts.lp_mint.key, keys.lp_mint),
        (*accounts.user_pool_lp.key, keys.user_pool_lp),
        (*accounts.a_vault_lp.key, keys.a_vault_lp),
        (*accounts.b_vault_lp.key, keys.b_vault_lp),
        (*accounts.a_vault.key, keys.a_vault),
        (*accounts.b_vault.key, keys.b_vault),
        (*accounts.a_vault_lp_mint.key, keys.a_vault_lp_mint),
        (*accounts.b_vault_lp_mint.key, keys.b_vault_lp_mint),
        (*accounts.a_token_vault.key, keys.a_token_vault),
        (*accounts.b_token_vault.key, keys.b_token_vault),
        (*accounts.user_a_token.key, keys.user_a_token),
        (*accounts.user_b_token.key, keys.user_b_token),
        (*accounts.user.key, keys.user),
        (*accounts.vault_program.key, keys.vault_program),
        (*accounts.token_program.key, keys.token_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn add_balance_liquidity_verify_writable_privileges<'me, 'info>(
    accounts: AddBalanceLiquidityAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.pool,
        accounts.lp_mint,
        accounts.user_pool_lp,
        accounts.a_vault_lp,
        accounts.b_vault_lp,
        accounts.a_vault,
        accounts.b_vault,
        accounts.a_vault_lp_mint,
        accounts.b_vault_lp_mint,
        accounts.a_token_vault,
        accounts.b_token_vault,
        accounts.user_a_token,
        accounts.user_b_token,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn add_balance_liquidity_verify_signer_privileges<'me, 'info>(
    accounts: AddBalanceLiquidityAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.user] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn add_balance_liquidity_verify_account_privileges<'me, 'info>(
    accounts: AddBalanceLiquidityAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    add_balance_liquidity_verify_writable_privileges(accounts)?;
    add_balance_liquidity_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const SET_POOL_FEES_IX_ACCOUNTS_LEN: usize = 2;
#[derive(Copy, Clone, Debug)]
pub struct SetPoolFeesAccounts<'me, 'info> {
    pub pool: &'me AccountInfo<'info>,
    pub fee_operator: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SetPoolFeesKeys {
    pub pool: Pubkey,
    pub fee_operator: Pubkey,
}
impl From<SetPoolFeesAccounts<'_, '_>> for SetPoolFeesKeys {
    fn from(accounts: SetPoolFeesAccounts) -> Self {
        Self {
            pool: *accounts.pool.key,
            fee_operator: *accounts.fee_operator.key,
        }
    }
}
impl From<SetPoolFeesKeys> for [AccountMeta; SET_POOL_FEES_IX_ACCOUNTS_LEN] {
    fn from(keys: SetPoolFeesKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.pool,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.fee_operator,
                is_signer: true,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; SET_POOL_FEES_IX_ACCOUNTS_LEN]> for SetPoolFeesKeys {
    fn from(pubkeys: [Pubkey; SET_POOL_FEES_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            pool: pubkeys[0],
            fee_operator: pubkeys[1],
        }
    }
}
impl<'info> From<SetPoolFeesAccounts<'_, 'info>>
for [AccountInfo<'info>; SET_POOL_FEES_IX_ACCOUNTS_LEN] {
    fn from(accounts: SetPoolFeesAccounts<'_, 'info>) -> Self {
        [accounts.pool.clone(), accounts.fee_operator.clone()]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; SET_POOL_FEES_IX_ACCOUNTS_LEN]>
for SetPoolFeesAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; SET_POOL_FEES_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            pool: &arr[0],
            fee_operator: &arr[1],
        }
    }
}
pub const SET_POOL_FEES_IX_DISCM: [u8; 8] = [102, 44, 158, 54, 205, 37, 126, 78];
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SetPoolFeesIxArgs {
    pub fees: PoolFees,
    pub new_partner_fee_numerator: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct SetPoolFeesIxData(pub SetPoolFeesIxArgs);
impl From<SetPoolFeesIxArgs> for SetPoolFeesIxData {
    fn from(args: SetPoolFeesIxArgs) -> Self {
        Self(args)
    }
}
impl SetPoolFeesIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != SET_POOL_FEES_IX_DISCM {
            return Err(
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!(
                        "discm does not match. Expected: {:?}. Received: {:?}",
                        SET_POOL_FEES_IX_DISCM, maybe_discm
                    ),
                ),
            );
        }
        Ok(Self(SetPoolFeesIxArgs::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&SET_POOL_FEES_IX_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn set_pool_fees_ix_with_program_id(
    program_id: Pubkey,
    keys: SetPoolFeesKeys,
    args: SetPoolFeesIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; SET_POOL_FEES_IX_ACCOUNTS_LEN] = keys.into();
    let data: SetPoolFeesIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn set_pool_fees_ix(
    keys: SetPoolFeesKeys,
    args: SetPoolFeesIxArgs,
) -> std::io::Result<Instruction> {
    set_pool_fees_ix_with_program_id(crate::ID, keys, args)
}
pub fn set_pool_fees_invoke_with_program_id(
    program_id: Pubkey,
    accounts: SetPoolFeesAccounts<'_, '_>,
    args: SetPoolFeesIxArgs,
) -> ProgramResult {
    let keys: SetPoolFeesKeys = accounts.into();
    let ix = set_pool_fees_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn set_pool_fees_invoke(
    accounts: SetPoolFeesAccounts<'_, '_>,
    args: SetPoolFeesIxArgs,
) -> ProgramResult {
    set_pool_fees_invoke_with_program_id(crate::ID, accounts, args)
}
pub fn set_pool_fees_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: SetPoolFeesAccounts<'_, '_>,
    args: SetPoolFeesIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: SetPoolFeesKeys = accounts.into();
    let ix = set_pool_fees_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn set_pool_fees_invoke_signed(
    accounts: SetPoolFeesAccounts<'_, '_>,
    args: SetPoolFeesIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    set_pool_fees_invoke_signed_with_program_id(crate::ID, accounts, args, seeds)
}
pub fn set_pool_fees_verify_account_keys(
    accounts: SetPoolFeesAccounts<'_, '_>,
    keys: SetPoolFeesKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.pool.key, keys.pool),
        (*accounts.fee_operator.key, keys.fee_operator),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn set_pool_fees_verify_writable_privileges<'me, 'info>(
    accounts: SetPoolFeesAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.pool] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn set_pool_fees_verify_signer_privileges<'me, 'info>(
    accounts: SetPoolFeesAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.fee_operator] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn set_pool_fees_verify_account_privileges<'me, 'info>(
    accounts: SetPoolFeesAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    set_pool_fees_verify_writable_privileges(accounts)?;
    set_pool_fees_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const OVERRIDE_CURVE_PARAM_IX_ACCOUNTS_LEN: usize = 2;
#[derive(Copy, Clone, Debug)]
pub struct OverrideCurveParamAccounts<'me, 'info> {
    pub pool: &'me AccountInfo<'info>,
    pub admin: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct OverrideCurveParamKeys {
    pub pool: Pubkey,
    pub admin: Pubkey,
}
impl From<OverrideCurveParamAccounts<'_, '_>> for OverrideCurveParamKeys {
    fn from(accounts: OverrideCurveParamAccounts) -> Self {
        Self {
            pool: *accounts.pool.key,
            admin: *accounts.admin.key,
        }
    }
}
impl From<OverrideCurveParamKeys>
for [AccountMeta; OVERRIDE_CURVE_PARAM_IX_ACCOUNTS_LEN] {
    fn from(keys: OverrideCurveParamKeys) -> Self {
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
        ]
    }
}
impl From<[Pubkey; OVERRIDE_CURVE_PARAM_IX_ACCOUNTS_LEN]> for OverrideCurveParamKeys {
    fn from(pubkeys: [Pubkey; OVERRIDE_CURVE_PARAM_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            pool: pubkeys[0],
            admin: pubkeys[1],
        }
    }
}
impl<'info> From<OverrideCurveParamAccounts<'_, 'info>>
for [AccountInfo<'info>; OVERRIDE_CURVE_PARAM_IX_ACCOUNTS_LEN] {
    fn from(accounts: OverrideCurveParamAccounts<'_, 'info>) -> Self {
        [accounts.pool.clone(), accounts.admin.clone()]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; OVERRIDE_CURVE_PARAM_IX_ACCOUNTS_LEN]>
for OverrideCurveParamAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; OVERRIDE_CURVE_PARAM_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            pool: &arr[0],
            admin: &arr[1],
        }
    }
}
pub const OVERRIDE_CURVE_PARAM_IX_DISCM: [u8; 8] = [98, 86, 204, 51, 94, 71, 69, 187];
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct OverrideCurveParamIxArgs {
    pub curve_type: CurveType,
}
#[derive(Clone, Debug, PartialEq)]
pub struct OverrideCurveParamIxData(pub OverrideCurveParamIxArgs);
impl From<OverrideCurveParamIxArgs> for OverrideCurveParamIxData {
    fn from(args: OverrideCurveParamIxArgs) -> Self {
        Self(args)
    }
}
impl OverrideCurveParamIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != OVERRIDE_CURVE_PARAM_IX_DISCM {
            return Err(
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!(
                        "discm does not match. Expected: {:?}. Received: {:?}",
                        OVERRIDE_CURVE_PARAM_IX_DISCM, maybe_discm
                    ),
                ),
            );
        }
        Ok(Self(OverrideCurveParamIxArgs::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&OVERRIDE_CURVE_PARAM_IX_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn override_curve_param_ix_with_program_id(
    program_id: Pubkey,
    keys: OverrideCurveParamKeys,
    args: OverrideCurveParamIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; OVERRIDE_CURVE_PARAM_IX_ACCOUNTS_LEN] = keys.into();
    let data: OverrideCurveParamIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn override_curve_param_ix(
    keys: OverrideCurveParamKeys,
    args: OverrideCurveParamIxArgs,
) -> std::io::Result<Instruction> {
    override_curve_param_ix_with_program_id(crate::ID, keys, args)
}
pub fn override_curve_param_invoke_with_program_id(
    program_id: Pubkey,
    accounts: OverrideCurveParamAccounts<'_, '_>,
    args: OverrideCurveParamIxArgs,
) -> ProgramResult {
    let keys: OverrideCurveParamKeys = accounts.into();
    let ix = override_curve_param_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn override_curve_param_invoke(
    accounts: OverrideCurveParamAccounts<'_, '_>,
    args: OverrideCurveParamIxArgs,
) -> ProgramResult {
    override_curve_param_invoke_with_program_id(crate::ID, accounts, args)
}
pub fn override_curve_param_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: OverrideCurveParamAccounts<'_, '_>,
    args: OverrideCurveParamIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: OverrideCurveParamKeys = accounts.into();
    let ix = override_curve_param_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn override_curve_param_invoke_signed(
    accounts: OverrideCurveParamAccounts<'_, '_>,
    args: OverrideCurveParamIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    override_curve_param_invoke_signed_with_program_id(crate::ID, accounts, args, seeds)
}
pub fn override_curve_param_verify_account_keys(
    accounts: OverrideCurveParamAccounts<'_, '_>,
    keys: OverrideCurveParamKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.pool.key, keys.pool),
        (*accounts.admin.key, keys.admin),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn override_curve_param_verify_writable_privileges<'me, 'info>(
    accounts: OverrideCurveParamAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.pool] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn override_curve_param_verify_signer_privileges<'me, 'info>(
    accounts: OverrideCurveParamAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.admin] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn override_curve_param_verify_account_privileges<'me, 'info>(
    accounts: OverrideCurveParamAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    override_curve_param_verify_writable_privileges(accounts)?;
    override_curve_param_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const GET_POOL_INFO_IX_ACCOUNTS_LEN: usize = 8;
#[derive(Copy, Clone, Debug)]
pub struct GetPoolInfoAccounts<'me, 'info> {
    pub pool: &'me AccountInfo<'info>,
    pub lp_mint: &'me AccountInfo<'info>,
    pub a_vault_lp: &'me AccountInfo<'info>,
    pub b_vault_lp: &'me AccountInfo<'info>,
    pub a_vault: &'me AccountInfo<'info>,
    pub b_vault: &'me AccountInfo<'info>,
    pub a_vault_lp_mint: &'me AccountInfo<'info>,
    pub b_vault_lp_mint: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GetPoolInfoKeys {
    pub pool: Pubkey,
    pub lp_mint: Pubkey,
    pub a_vault_lp: Pubkey,
    pub b_vault_lp: Pubkey,
    pub a_vault: Pubkey,
    pub b_vault: Pubkey,
    pub a_vault_lp_mint: Pubkey,
    pub b_vault_lp_mint: Pubkey,
}
impl From<GetPoolInfoAccounts<'_, '_>> for GetPoolInfoKeys {
    fn from(accounts: GetPoolInfoAccounts) -> Self {
        Self {
            pool: *accounts.pool.key,
            lp_mint: *accounts.lp_mint.key,
            a_vault_lp: *accounts.a_vault_lp.key,
            b_vault_lp: *accounts.b_vault_lp.key,
            a_vault: *accounts.a_vault.key,
            b_vault: *accounts.b_vault.key,
            a_vault_lp_mint: *accounts.a_vault_lp_mint.key,
            b_vault_lp_mint: *accounts.b_vault_lp_mint.key,
        }
    }
}
impl From<GetPoolInfoKeys> for [AccountMeta; GET_POOL_INFO_IX_ACCOUNTS_LEN] {
    fn from(keys: GetPoolInfoKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.pool,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.lp_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.a_vault_lp,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.b_vault_lp,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.a_vault,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.b_vault,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.a_vault_lp_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.b_vault_lp_mint,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; GET_POOL_INFO_IX_ACCOUNTS_LEN]> for GetPoolInfoKeys {
    fn from(pubkeys: [Pubkey; GET_POOL_INFO_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            pool: pubkeys[0],
            lp_mint: pubkeys[1],
            a_vault_lp: pubkeys[2],
            b_vault_lp: pubkeys[3],
            a_vault: pubkeys[4],
            b_vault: pubkeys[5],
            a_vault_lp_mint: pubkeys[6],
            b_vault_lp_mint: pubkeys[7],
        }
    }
}
impl<'info> From<GetPoolInfoAccounts<'_, 'info>>
for [AccountInfo<'info>; GET_POOL_INFO_IX_ACCOUNTS_LEN] {
    fn from(accounts: GetPoolInfoAccounts<'_, 'info>) -> Self {
        [
            accounts.pool.clone(),
            accounts.lp_mint.clone(),
            accounts.a_vault_lp.clone(),
            accounts.b_vault_lp.clone(),
            accounts.a_vault.clone(),
            accounts.b_vault.clone(),
            accounts.a_vault_lp_mint.clone(),
            accounts.b_vault_lp_mint.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; GET_POOL_INFO_IX_ACCOUNTS_LEN]>
for GetPoolInfoAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; GET_POOL_INFO_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            pool: &arr[0],
            lp_mint: &arr[1],
            a_vault_lp: &arr[2],
            b_vault_lp: &arr[3],
            a_vault: &arr[4],
            b_vault: &arr[5],
            a_vault_lp_mint: &arr[6],
            b_vault_lp_mint: &arr[7],
        }
    }
}
pub const GET_POOL_INFO_IX_DISCM: [u8; 8] = [9, 48, 220, 101, 22, 240, 78, 200];
#[derive(Clone, Debug, PartialEq)]
pub struct GetPoolInfoIxData;
impl GetPoolInfoIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != GET_POOL_INFO_IX_DISCM {
            return Err(
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!(
                        "discm does not match. Expected: {:?}. Received: {:?}",
                        GET_POOL_INFO_IX_DISCM, maybe_discm
                    ),
                ),
            );
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&GET_POOL_INFO_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn get_pool_info_ix_with_program_id(
    program_id: Pubkey,
    keys: GetPoolInfoKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; GET_POOL_INFO_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: GetPoolInfoIxData.try_to_vec()?,
    })
}
pub fn get_pool_info_ix(keys: GetPoolInfoKeys) -> std::io::Result<Instruction> {
    get_pool_info_ix_with_program_id(crate::ID, keys)
}
pub fn get_pool_info_invoke_with_program_id(
    program_id: Pubkey,
    accounts: GetPoolInfoAccounts<'_, '_>,
) -> ProgramResult {
    let keys: GetPoolInfoKeys = accounts.into();
    let ix = get_pool_info_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn get_pool_info_invoke(accounts: GetPoolInfoAccounts<'_, '_>) -> ProgramResult {
    get_pool_info_invoke_with_program_id(crate::ID, accounts)
}
pub fn get_pool_info_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: GetPoolInfoAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: GetPoolInfoKeys = accounts.into();
    let ix = get_pool_info_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn get_pool_info_invoke_signed(
    accounts: GetPoolInfoAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    get_pool_info_invoke_signed_with_program_id(crate::ID, accounts, seeds)
}
pub fn get_pool_info_verify_account_keys(
    accounts: GetPoolInfoAccounts<'_, '_>,
    keys: GetPoolInfoKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.pool.key, keys.pool),
        (*accounts.lp_mint.key, keys.lp_mint),
        (*accounts.a_vault_lp.key, keys.a_vault_lp),
        (*accounts.b_vault_lp.key, keys.b_vault_lp),
        (*accounts.a_vault.key, keys.a_vault),
        (*accounts.b_vault.key, keys.b_vault),
        (*accounts.a_vault_lp_mint.key, keys.a_vault_lp_mint),
        (*accounts.b_vault_lp_mint.key, keys.b_vault_lp_mint),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub const BOOTSTRAP_LIQUIDITY_IX_ACCOUNTS_LEN: usize = 16;
#[derive(Copy, Clone, Debug)]
pub struct BootstrapLiquidityAccounts<'me, 'info> {
    pub pool: &'me AccountInfo<'info>,
    pub lp_mint: &'me AccountInfo<'info>,
    pub user_pool_lp: &'me AccountInfo<'info>,
    pub a_vault_lp: &'me AccountInfo<'info>,
    pub b_vault_lp: &'me AccountInfo<'info>,
    pub a_vault: &'me AccountInfo<'info>,
    pub b_vault: &'me AccountInfo<'info>,
    pub a_vault_lp_mint: &'me AccountInfo<'info>,
    pub b_vault_lp_mint: &'me AccountInfo<'info>,
    pub a_token_vault: &'me AccountInfo<'info>,
    pub b_token_vault: &'me AccountInfo<'info>,
    pub user_a_token: &'me AccountInfo<'info>,
    pub user_b_token: &'me AccountInfo<'info>,
    pub user: &'me AccountInfo<'info>,
    pub vault_program: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct BootstrapLiquidityKeys {
    pub pool: Pubkey,
    pub lp_mint: Pubkey,
    pub user_pool_lp: Pubkey,
    pub a_vault_lp: Pubkey,
    pub b_vault_lp: Pubkey,
    pub a_vault: Pubkey,
    pub b_vault: Pubkey,
    pub a_vault_lp_mint: Pubkey,
    pub b_vault_lp_mint: Pubkey,
    pub a_token_vault: Pubkey,
    pub b_token_vault: Pubkey,
    pub user_a_token: Pubkey,
    pub user_b_token: Pubkey,
    pub user: Pubkey,
    pub vault_program: Pubkey,
    pub token_program: Pubkey,
}
impl From<BootstrapLiquidityAccounts<'_, '_>> for BootstrapLiquidityKeys {
    fn from(accounts: BootstrapLiquidityAccounts) -> Self {
        Self {
            pool: *accounts.pool.key,
            lp_mint: *accounts.lp_mint.key,
            user_pool_lp: *accounts.user_pool_lp.key,
            a_vault_lp: *accounts.a_vault_lp.key,
            b_vault_lp: *accounts.b_vault_lp.key,
            a_vault: *accounts.a_vault.key,
            b_vault: *accounts.b_vault.key,
            a_vault_lp_mint: *accounts.a_vault_lp_mint.key,
            b_vault_lp_mint: *accounts.b_vault_lp_mint.key,
            a_token_vault: *accounts.a_token_vault.key,
            b_token_vault: *accounts.b_token_vault.key,
            user_a_token: *accounts.user_a_token.key,
            user_b_token: *accounts.user_b_token.key,
            user: *accounts.user.key,
            vault_program: *accounts.vault_program.key,
            token_program: *accounts.token_program.key,
        }
    }
}
impl From<BootstrapLiquidityKeys>
for [AccountMeta; BOOTSTRAP_LIQUIDITY_IX_ACCOUNTS_LEN] {
    fn from(keys: BootstrapLiquidityKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.pool,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.lp_mint,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_pool_lp,
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
                pubkey: keys.user_a_token,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_b_token,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user,
                is_signer: true,
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
        ]
    }
}
impl From<[Pubkey; BOOTSTRAP_LIQUIDITY_IX_ACCOUNTS_LEN]> for BootstrapLiquidityKeys {
    fn from(pubkeys: [Pubkey; BOOTSTRAP_LIQUIDITY_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            pool: pubkeys[0],
            lp_mint: pubkeys[1],
            user_pool_lp: pubkeys[2],
            a_vault_lp: pubkeys[3],
            b_vault_lp: pubkeys[4],
            a_vault: pubkeys[5],
            b_vault: pubkeys[6],
            a_vault_lp_mint: pubkeys[7],
            b_vault_lp_mint: pubkeys[8],
            a_token_vault: pubkeys[9],
            b_token_vault: pubkeys[10],
            user_a_token: pubkeys[11],
            user_b_token: pubkeys[12],
            user: pubkeys[13],
            vault_program: pubkeys[14],
            token_program: pubkeys[15],
        }
    }
}
impl<'info> From<BootstrapLiquidityAccounts<'_, 'info>>
for [AccountInfo<'info>; BOOTSTRAP_LIQUIDITY_IX_ACCOUNTS_LEN] {
    fn from(accounts: BootstrapLiquidityAccounts<'_, 'info>) -> Self {
        [
            accounts.pool.clone(),
            accounts.lp_mint.clone(),
            accounts.user_pool_lp.clone(),
            accounts.a_vault_lp.clone(),
            accounts.b_vault_lp.clone(),
            accounts.a_vault.clone(),
            accounts.b_vault.clone(),
            accounts.a_vault_lp_mint.clone(),
            accounts.b_vault_lp_mint.clone(),
            accounts.a_token_vault.clone(),
            accounts.b_token_vault.clone(),
            accounts.user_a_token.clone(),
            accounts.user_b_token.clone(),
            accounts.user.clone(),
            accounts.vault_program.clone(),
            accounts.token_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; BOOTSTRAP_LIQUIDITY_IX_ACCOUNTS_LEN]>
for BootstrapLiquidityAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; BOOTSTRAP_LIQUIDITY_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            pool: &arr[0],
            lp_mint: &arr[1],
            user_pool_lp: &arr[2],
            a_vault_lp: &arr[3],
            b_vault_lp: &arr[4],
            a_vault: &arr[5],
            b_vault: &arr[6],
            a_vault_lp_mint: &arr[7],
            b_vault_lp_mint: &arr[8],
            a_token_vault: &arr[9],
            b_token_vault: &arr[10],
            user_a_token: &arr[11],
            user_b_token: &arr[12],
            user: &arr[13],
            vault_program: &arr[14],
            token_program: &arr[15],
        }
    }
}
pub const BOOTSTRAP_LIQUIDITY_IX_DISCM: [u8; 8] = [4, 228, 215, 71, 225, 253, 119, 206];
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct BootstrapLiquidityIxArgs {
    pub token_a_amount: u64,
    pub token_b_amount: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct BootstrapLiquidityIxData(pub BootstrapLiquidityIxArgs);
impl From<BootstrapLiquidityIxArgs> for BootstrapLiquidityIxData {
    fn from(args: BootstrapLiquidityIxArgs) -> Self {
        Self(args)
    }
}
impl BootstrapLiquidityIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != BOOTSTRAP_LIQUIDITY_IX_DISCM {
            return Err(
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!(
                        "discm does not match. Expected: {:?}. Received: {:?}",
                        BOOTSTRAP_LIQUIDITY_IX_DISCM, maybe_discm
                    ),
                ),
            );
        }
        Ok(Self(BootstrapLiquidityIxArgs::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&BOOTSTRAP_LIQUIDITY_IX_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn bootstrap_liquidity_ix_with_program_id(
    program_id: Pubkey,
    keys: BootstrapLiquidityKeys,
    args: BootstrapLiquidityIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; BOOTSTRAP_LIQUIDITY_IX_ACCOUNTS_LEN] = keys.into();
    let data: BootstrapLiquidityIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn bootstrap_liquidity_ix(
    keys: BootstrapLiquidityKeys,
    args: BootstrapLiquidityIxArgs,
) -> std::io::Result<Instruction> {
    bootstrap_liquidity_ix_with_program_id(crate::ID, keys, args)
}
pub fn bootstrap_liquidity_invoke_with_program_id(
    program_id: Pubkey,
    accounts: BootstrapLiquidityAccounts<'_, '_>,
    args: BootstrapLiquidityIxArgs,
) -> ProgramResult {
    let keys: BootstrapLiquidityKeys = accounts.into();
    let ix = bootstrap_liquidity_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn bootstrap_liquidity_invoke(
    accounts: BootstrapLiquidityAccounts<'_, '_>,
    args: BootstrapLiquidityIxArgs,
) -> ProgramResult {
    bootstrap_liquidity_invoke_with_program_id(crate::ID, accounts, args)
}
pub fn bootstrap_liquidity_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: BootstrapLiquidityAccounts<'_, '_>,
    args: BootstrapLiquidityIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: BootstrapLiquidityKeys = accounts.into();
    let ix = bootstrap_liquidity_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn bootstrap_liquidity_invoke_signed(
    accounts: BootstrapLiquidityAccounts<'_, '_>,
    args: BootstrapLiquidityIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    bootstrap_liquidity_invoke_signed_with_program_id(crate::ID, accounts, args, seeds)
}
pub fn bootstrap_liquidity_verify_account_keys(
    accounts: BootstrapLiquidityAccounts<'_, '_>,
    keys: BootstrapLiquidityKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.pool.key, keys.pool),
        (*accounts.lp_mint.key, keys.lp_mint),
        (*accounts.user_pool_lp.key, keys.user_pool_lp),
        (*accounts.a_vault_lp.key, keys.a_vault_lp),
        (*accounts.b_vault_lp.key, keys.b_vault_lp),
        (*accounts.a_vault.key, keys.a_vault),
        (*accounts.b_vault.key, keys.b_vault),
        (*accounts.a_vault_lp_mint.key, keys.a_vault_lp_mint),
        (*accounts.b_vault_lp_mint.key, keys.b_vault_lp_mint),
        (*accounts.a_token_vault.key, keys.a_token_vault),
        (*accounts.b_token_vault.key, keys.b_token_vault),
        (*accounts.user_a_token.key, keys.user_a_token),
        (*accounts.user_b_token.key, keys.user_b_token),
        (*accounts.user.key, keys.user),
        (*accounts.vault_program.key, keys.vault_program),
        (*accounts.token_program.key, keys.token_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn bootstrap_liquidity_verify_writable_privileges<'me, 'info>(
    accounts: BootstrapLiquidityAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.pool,
        accounts.lp_mint,
        accounts.user_pool_lp,
        accounts.a_vault_lp,
        accounts.b_vault_lp,
        accounts.a_vault,
        accounts.b_vault,
        accounts.a_vault_lp_mint,
        accounts.b_vault_lp_mint,
        accounts.a_token_vault,
        accounts.b_token_vault,
        accounts.user_a_token,
        accounts.user_b_token,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn bootstrap_liquidity_verify_signer_privileges<'me, 'info>(
    accounts: BootstrapLiquidityAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.user] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn bootstrap_liquidity_verify_account_privileges<'me, 'info>(
    accounts: BootstrapLiquidityAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    bootstrap_liquidity_verify_writable_privileges(accounts)?;
    bootstrap_liquidity_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const CREATE_MINT_METADATA_IX_ACCOUNTS_LEN: usize = 7;
#[derive(Copy, Clone, Debug)]
pub struct CreateMintMetadataAccounts<'me, 'info> {
    pub pool: &'me AccountInfo<'info>,
    pub lp_mint: &'me AccountInfo<'info>,
    pub a_vault_lp: &'me AccountInfo<'info>,
    pub mint_metadata: &'me AccountInfo<'info>,
    pub metadata_program: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub payer: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CreateMintMetadataKeys {
    pub pool: Pubkey,
    pub lp_mint: Pubkey,
    pub a_vault_lp: Pubkey,
    pub mint_metadata: Pubkey,
    pub metadata_program: Pubkey,
    pub system_program: Pubkey,
    pub payer: Pubkey,
}
impl From<CreateMintMetadataAccounts<'_, '_>> for CreateMintMetadataKeys {
    fn from(accounts: CreateMintMetadataAccounts) -> Self {
        Self {
            pool: *accounts.pool.key,
            lp_mint: *accounts.lp_mint.key,
            a_vault_lp: *accounts.a_vault_lp.key,
            mint_metadata: *accounts.mint_metadata.key,
            metadata_program: *accounts.metadata_program.key,
            system_program: *accounts.system_program.key,
            payer: *accounts.payer.key,
        }
    }
}
impl From<CreateMintMetadataKeys>
for [AccountMeta; CREATE_MINT_METADATA_IX_ACCOUNTS_LEN] {
    fn from(keys: CreateMintMetadataKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.pool,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.lp_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.a_vault_lp,
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
                pubkey: keys.system_program,
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
impl From<[Pubkey; CREATE_MINT_METADATA_IX_ACCOUNTS_LEN]> for CreateMintMetadataKeys {
    fn from(pubkeys: [Pubkey; CREATE_MINT_METADATA_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            pool: pubkeys[0],
            lp_mint: pubkeys[1],
            a_vault_lp: pubkeys[2],
            mint_metadata: pubkeys[3],
            metadata_program: pubkeys[4],
            system_program: pubkeys[5],
            payer: pubkeys[6],
        }
    }
}
impl<'info> From<CreateMintMetadataAccounts<'_, 'info>>
for [AccountInfo<'info>; CREATE_MINT_METADATA_IX_ACCOUNTS_LEN] {
    fn from(accounts: CreateMintMetadataAccounts<'_, 'info>) -> Self {
        [
            accounts.pool.clone(),
            accounts.lp_mint.clone(),
            accounts.a_vault_lp.clone(),
            accounts.mint_metadata.clone(),
            accounts.metadata_program.clone(),
            accounts.system_program.clone(),
            accounts.payer.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; CREATE_MINT_METADATA_IX_ACCOUNTS_LEN]>
for CreateMintMetadataAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; CREATE_MINT_METADATA_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            pool: &arr[0],
            lp_mint: &arr[1],
            a_vault_lp: &arr[2],
            mint_metadata: &arr[3],
            metadata_program: &arr[4],
            system_program: &arr[5],
            payer: &arr[6],
        }
    }
}
pub const CREATE_MINT_METADATA_IX_DISCM: [u8; 8] = [13, 70, 168, 41, 250, 100, 148, 90];
#[derive(Clone, Debug, PartialEq)]
pub struct CreateMintMetadataIxData;
impl CreateMintMetadataIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != CREATE_MINT_METADATA_IX_DISCM {
            return Err(
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!(
                        "discm does not match. Expected: {:?}. Received: {:?}",
                        CREATE_MINT_METADATA_IX_DISCM, maybe_discm
                    ),
                ),
            );
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&CREATE_MINT_METADATA_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn create_mint_metadata_ix_with_program_id(
    program_id: Pubkey,
    keys: CreateMintMetadataKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; CREATE_MINT_METADATA_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: CreateMintMetadataIxData.try_to_vec()?,
    })
}
pub fn create_mint_metadata_ix(
    keys: CreateMintMetadataKeys,
) -> std::io::Result<Instruction> {
    create_mint_metadata_ix_with_program_id(crate::ID, keys)
}
pub fn create_mint_metadata_invoke_with_program_id(
    program_id: Pubkey,
    accounts: CreateMintMetadataAccounts<'_, '_>,
) -> ProgramResult {
    let keys: CreateMintMetadataKeys = accounts.into();
    let ix = create_mint_metadata_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn create_mint_metadata_invoke(
    accounts: CreateMintMetadataAccounts<'_, '_>,
) -> ProgramResult {
    create_mint_metadata_invoke_with_program_id(crate::ID, accounts)
}
pub fn create_mint_metadata_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: CreateMintMetadataAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: CreateMintMetadataKeys = accounts.into();
    let ix = create_mint_metadata_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn create_mint_metadata_invoke_signed(
    accounts: CreateMintMetadataAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    create_mint_metadata_invoke_signed_with_program_id(crate::ID, accounts, seeds)
}
pub fn create_mint_metadata_verify_account_keys(
    accounts: CreateMintMetadataAccounts<'_, '_>,
    keys: CreateMintMetadataKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.pool.key, keys.pool),
        (*accounts.lp_mint.key, keys.lp_mint),
        (*accounts.a_vault_lp.key, keys.a_vault_lp),
        (*accounts.mint_metadata.key, keys.mint_metadata),
        (*accounts.metadata_program.key, keys.metadata_program),
        (*accounts.system_program.key, keys.system_program),
        (*accounts.payer.key, keys.payer),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn create_mint_metadata_verify_writable_privileges<'me, 'info>(
    accounts: CreateMintMetadataAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.mint_metadata, accounts.payer] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn create_mint_metadata_verify_signer_privileges<'me, 'info>(
    accounts: CreateMintMetadataAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.payer] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn create_mint_metadata_verify_account_privileges<'me, 'info>(
    accounts: CreateMintMetadataAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    create_mint_metadata_verify_writable_privileges(accounts)?;
    create_mint_metadata_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const CREATE_LOCK_ESCROW_IX_ACCOUNTS_LEN: usize = 6;
#[derive(Copy, Clone, Debug)]
pub struct CreateLockEscrowAccounts<'me, 'info> {
    pub pool: &'me AccountInfo<'info>,
    pub lock_escrow: &'me AccountInfo<'info>,
    pub owner: &'me AccountInfo<'info>,
    pub lp_mint: &'me AccountInfo<'info>,
    pub payer: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CreateLockEscrowKeys {
    pub pool: Pubkey,
    pub lock_escrow: Pubkey,
    pub owner: Pubkey,
    pub lp_mint: Pubkey,
    pub payer: Pubkey,
    pub system_program: Pubkey,
}
impl From<CreateLockEscrowAccounts<'_, '_>> for CreateLockEscrowKeys {
    fn from(accounts: CreateLockEscrowAccounts) -> Self {
        Self {
            pool: *accounts.pool.key,
            lock_escrow: *accounts.lock_escrow.key,
            owner: *accounts.owner.key,
            lp_mint: *accounts.lp_mint.key,
            payer: *accounts.payer.key,
            system_program: *accounts.system_program.key,
        }
    }
}
impl From<CreateLockEscrowKeys> for [AccountMeta; CREATE_LOCK_ESCROW_IX_ACCOUNTS_LEN] {
    fn from(keys: CreateLockEscrowKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.pool,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.lock_escrow,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.owner,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.lp_mint,
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
        ]
    }
}
impl From<[Pubkey; CREATE_LOCK_ESCROW_IX_ACCOUNTS_LEN]> for CreateLockEscrowKeys {
    fn from(pubkeys: [Pubkey; CREATE_LOCK_ESCROW_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            pool: pubkeys[0],
            lock_escrow: pubkeys[1],
            owner: pubkeys[2],
            lp_mint: pubkeys[3],
            payer: pubkeys[4],
            system_program: pubkeys[5],
        }
    }
}
impl<'info> From<CreateLockEscrowAccounts<'_, 'info>>
for [AccountInfo<'info>; CREATE_LOCK_ESCROW_IX_ACCOUNTS_LEN] {
    fn from(accounts: CreateLockEscrowAccounts<'_, 'info>) -> Self {
        [
            accounts.pool.clone(),
            accounts.lock_escrow.clone(),
            accounts.owner.clone(),
            accounts.lp_mint.clone(),
            accounts.payer.clone(),
            accounts.system_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; CREATE_LOCK_ESCROW_IX_ACCOUNTS_LEN]>
for CreateLockEscrowAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; CREATE_LOCK_ESCROW_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            pool: &arr[0],
            lock_escrow: &arr[1],
            owner: &arr[2],
            lp_mint: &arr[3],
            payer: &arr[4],
            system_program: &arr[5],
        }
    }
}
pub const CREATE_LOCK_ESCROW_IX_DISCM: [u8; 8] = [54, 87, 165, 19, 69, 227, 218, 224];
#[derive(Clone, Debug, PartialEq)]
pub struct CreateLockEscrowIxData;
impl CreateLockEscrowIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != CREATE_LOCK_ESCROW_IX_DISCM {
            return Err(
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!(
                        "discm does not match. Expected: {:?}. Received: {:?}",
                        CREATE_LOCK_ESCROW_IX_DISCM, maybe_discm
                    ),
                ),
            );
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&CREATE_LOCK_ESCROW_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn create_lock_escrow_ix_with_program_id(
    program_id: Pubkey,
    keys: CreateLockEscrowKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; CREATE_LOCK_ESCROW_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: CreateLockEscrowIxData.try_to_vec()?,
    })
}
pub fn create_lock_escrow_ix(
    keys: CreateLockEscrowKeys,
) -> std::io::Result<Instruction> {
    create_lock_escrow_ix_with_program_id(crate::ID, keys)
}
pub fn create_lock_escrow_invoke_with_program_id(
    program_id: Pubkey,
    accounts: CreateLockEscrowAccounts<'_, '_>,
) -> ProgramResult {
    let keys: CreateLockEscrowKeys = accounts.into();
    let ix = create_lock_escrow_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn create_lock_escrow_invoke(
    accounts: CreateLockEscrowAccounts<'_, '_>,
) -> ProgramResult {
    create_lock_escrow_invoke_with_program_id(crate::ID, accounts)
}
pub fn create_lock_escrow_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: CreateLockEscrowAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: CreateLockEscrowKeys = accounts.into();
    let ix = create_lock_escrow_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn create_lock_escrow_invoke_signed(
    accounts: CreateLockEscrowAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    create_lock_escrow_invoke_signed_with_program_id(crate::ID, accounts, seeds)
}
pub fn create_lock_escrow_verify_account_keys(
    accounts: CreateLockEscrowAccounts<'_, '_>,
    keys: CreateLockEscrowKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.pool.key, keys.pool),
        (*accounts.lock_escrow.key, keys.lock_escrow),
        (*accounts.owner.key, keys.owner),
        (*accounts.lp_mint.key, keys.lp_mint),
        (*accounts.payer.key, keys.payer),
        (*accounts.system_program.key, keys.system_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn create_lock_escrow_verify_writable_privileges<'me, 'info>(
    accounts: CreateLockEscrowAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.lock_escrow, accounts.payer] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn create_lock_escrow_verify_signer_privileges<'me, 'info>(
    accounts: CreateLockEscrowAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.payer] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn create_lock_escrow_verify_account_privileges<'me, 'info>(
    accounts: CreateLockEscrowAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    create_lock_escrow_verify_writable_privileges(accounts)?;
    create_lock_escrow_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const LOCK_IX_ACCOUNTS_LEN: usize = 13;
#[derive(Copy, Clone, Debug)]
pub struct LockAccounts<'me, 'info> {
    pub pool: &'me AccountInfo<'info>,
    pub lp_mint: &'me AccountInfo<'info>,
    pub lock_escrow: &'me AccountInfo<'info>,
    pub owner: &'me AccountInfo<'info>,
    pub source_tokens: &'me AccountInfo<'info>,
    pub escrow_vault: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub a_vault: &'me AccountInfo<'info>,
    pub b_vault: &'me AccountInfo<'info>,
    pub a_vault_lp: &'me AccountInfo<'info>,
    pub b_vault_lp: &'me AccountInfo<'info>,
    pub a_vault_lp_mint: &'me AccountInfo<'info>,
    pub b_vault_lp_mint: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LockKeys {
    pub pool: Pubkey,
    pub lp_mint: Pubkey,
    pub lock_escrow: Pubkey,
    pub owner: Pubkey,
    pub source_tokens: Pubkey,
    pub escrow_vault: Pubkey,
    pub token_program: Pubkey,
    pub a_vault: Pubkey,
    pub b_vault: Pubkey,
    pub a_vault_lp: Pubkey,
    pub b_vault_lp: Pubkey,
    pub a_vault_lp_mint: Pubkey,
    pub b_vault_lp_mint: Pubkey,
}
impl From<LockAccounts<'_, '_>> for LockKeys {
    fn from(accounts: LockAccounts) -> Self {
        Self {
            pool: *accounts.pool.key,
            lp_mint: *accounts.lp_mint.key,
            lock_escrow: *accounts.lock_escrow.key,
            owner: *accounts.owner.key,
            source_tokens: *accounts.source_tokens.key,
            escrow_vault: *accounts.escrow_vault.key,
            token_program: *accounts.token_program.key,
            a_vault: *accounts.a_vault.key,
            b_vault: *accounts.b_vault.key,
            a_vault_lp: *accounts.a_vault_lp.key,
            b_vault_lp: *accounts.b_vault_lp.key,
            a_vault_lp_mint: *accounts.a_vault_lp_mint.key,
            b_vault_lp_mint: *accounts.b_vault_lp_mint.key,
        }
    }
}
impl From<LockKeys> for [AccountMeta; LOCK_IX_ACCOUNTS_LEN] {
    fn from(keys: LockKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.pool,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.lp_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.lock_escrow,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.owner,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.source_tokens,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.escrow_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.a_vault,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.b_vault,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.a_vault_lp,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.b_vault_lp,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.a_vault_lp_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.b_vault_lp_mint,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; LOCK_IX_ACCOUNTS_LEN]> for LockKeys {
    fn from(pubkeys: [Pubkey; LOCK_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            pool: pubkeys[0],
            lp_mint: pubkeys[1],
            lock_escrow: pubkeys[2],
            owner: pubkeys[3],
            source_tokens: pubkeys[4],
            escrow_vault: pubkeys[5],
            token_program: pubkeys[6],
            a_vault: pubkeys[7],
            b_vault: pubkeys[8],
            a_vault_lp: pubkeys[9],
            b_vault_lp: pubkeys[10],
            a_vault_lp_mint: pubkeys[11],
            b_vault_lp_mint: pubkeys[12],
        }
    }
}
impl<'info> From<LockAccounts<'_, 'info>>
for [AccountInfo<'info>; LOCK_IX_ACCOUNTS_LEN] {
    fn from(accounts: LockAccounts<'_, 'info>) -> Self {
        [
            accounts.pool.clone(),
            accounts.lp_mint.clone(),
            accounts.lock_escrow.clone(),
            accounts.owner.clone(),
            accounts.source_tokens.clone(),
            accounts.escrow_vault.clone(),
            accounts.token_program.clone(),
            accounts.a_vault.clone(),
            accounts.b_vault.clone(),
            accounts.a_vault_lp.clone(),
            accounts.b_vault_lp.clone(),
            accounts.a_vault_lp_mint.clone(),
            accounts.b_vault_lp_mint.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; LOCK_IX_ACCOUNTS_LEN]>
for LockAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; LOCK_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            pool: &arr[0],
            lp_mint: &arr[1],
            lock_escrow: &arr[2],
            owner: &arr[3],
            source_tokens: &arr[4],
            escrow_vault: &arr[5],
            token_program: &arr[6],
            a_vault: &arr[7],
            b_vault: &arr[8],
            a_vault_lp: &arr[9],
            b_vault_lp: &arr[10],
            a_vault_lp_mint: &arr[11],
            b_vault_lp_mint: &arr[12],
        }
    }
}
pub const LOCK_IX_DISCM: [u8; 8] = [21, 19, 208, 43, 237, 62, 255, 87];
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct LockIxArgs {
    pub max_amount: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct LockIxData(pub LockIxArgs);
impl From<LockIxArgs> for LockIxData {
    fn from(args: LockIxArgs) -> Self {
        Self(args)
    }
}
impl LockIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != LOCK_IX_DISCM {
            return Err(
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!(
                        "discm does not match. Expected: {:?}. Received: {:?}",
                        LOCK_IX_DISCM, maybe_discm
                    ),
                ),
            );
        }
        Ok(Self(LockIxArgs::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&LOCK_IX_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn lock_ix_with_program_id(
    program_id: Pubkey,
    keys: LockKeys,
    args: LockIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; LOCK_IX_ACCOUNTS_LEN] = keys.into();
    let data: LockIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn lock_ix(keys: LockKeys, args: LockIxArgs) -> std::io::Result<Instruction> {
    lock_ix_with_program_id(crate::ID, keys, args)
}
pub fn lock_invoke_with_program_id(
    program_id: Pubkey,
    accounts: LockAccounts<'_, '_>,
    args: LockIxArgs,
) -> ProgramResult {
    let keys: LockKeys = accounts.into();
    let ix = lock_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn lock_invoke(accounts: LockAccounts<'_, '_>, args: LockIxArgs) -> ProgramResult {
    lock_invoke_with_program_id(crate::ID, accounts, args)
}
pub fn lock_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: LockAccounts<'_, '_>,
    args: LockIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: LockKeys = accounts.into();
    let ix = lock_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn lock_invoke_signed(
    accounts: LockAccounts<'_, '_>,
    args: LockIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    lock_invoke_signed_with_program_id(crate::ID, accounts, args, seeds)
}
pub fn lock_verify_account_keys(
    accounts: LockAccounts<'_, '_>,
    keys: LockKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.pool.key, keys.pool),
        (*accounts.lp_mint.key, keys.lp_mint),
        (*accounts.lock_escrow.key, keys.lock_escrow),
        (*accounts.owner.key, keys.owner),
        (*accounts.source_tokens.key, keys.source_tokens),
        (*accounts.escrow_vault.key, keys.escrow_vault),
        (*accounts.token_program.key, keys.token_program),
        (*accounts.a_vault.key, keys.a_vault),
        (*accounts.b_vault.key, keys.b_vault),
        (*accounts.a_vault_lp.key, keys.a_vault_lp),
        (*accounts.b_vault_lp.key, keys.b_vault_lp),
        (*accounts.a_vault_lp_mint.key, keys.a_vault_lp_mint),
        (*accounts.b_vault_lp_mint.key, keys.b_vault_lp_mint),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn lock_verify_writable_privileges<'me, 'info>(
    accounts: LockAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.pool,
        accounts.lock_escrow,
        accounts.owner,
        accounts.source_tokens,
        accounts.escrow_vault,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn lock_verify_signer_privileges<'me, 'info>(
    accounts: LockAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.owner] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn lock_verify_account_privileges<'me, 'info>(
    accounts: LockAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    lock_verify_writable_privileges(accounts)?;
    lock_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const CLAIM_FEE_IX_ACCOUNTS_LEN: usize = 18;
#[derive(Copy, Clone, Debug)]
pub struct ClaimFeeAccounts<'me, 'info> {
    pub pool: &'me AccountInfo<'info>,
    pub lp_mint: &'me AccountInfo<'info>,
    pub lock_escrow: &'me AccountInfo<'info>,
    pub owner: &'me AccountInfo<'info>,
    pub source_tokens: &'me AccountInfo<'info>,
    pub escrow_vault: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub a_token_vault: &'me AccountInfo<'info>,
    pub b_token_vault: &'me AccountInfo<'info>,
    pub a_vault: &'me AccountInfo<'info>,
    pub b_vault: &'me AccountInfo<'info>,
    pub a_vault_lp: &'me AccountInfo<'info>,
    pub b_vault_lp: &'me AccountInfo<'info>,
    pub a_vault_lp_mint: &'me AccountInfo<'info>,
    pub b_vault_lp_mint: &'me AccountInfo<'info>,
    pub user_a_token: &'me AccountInfo<'info>,
    pub user_b_token: &'me AccountInfo<'info>,
    pub vault_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct ClaimFeeKeys {
    pub pool: Pubkey,
    pub lp_mint: Pubkey,
    pub lock_escrow: Pubkey,
    pub owner: Pubkey,
    pub source_tokens: Pubkey,
    pub escrow_vault: Pubkey,
    pub token_program: Pubkey,
    pub a_token_vault: Pubkey,
    pub b_token_vault: Pubkey,
    pub a_vault: Pubkey,
    pub b_vault: Pubkey,
    pub a_vault_lp: Pubkey,
    pub b_vault_lp: Pubkey,
    pub a_vault_lp_mint: Pubkey,
    pub b_vault_lp_mint: Pubkey,
    pub user_a_token: Pubkey,
    pub user_b_token: Pubkey,
    pub vault_program: Pubkey,
}
impl From<ClaimFeeAccounts<'_, '_>> for ClaimFeeKeys {
    fn from(accounts: ClaimFeeAccounts) -> Self {
        Self {
            pool: *accounts.pool.key,
            lp_mint: *accounts.lp_mint.key,
            lock_escrow: *accounts.lock_escrow.key,
            owner: *accounts.owner.key,
            source_tokens: *accounts.source_tokens.key,
            escrow_vault: *accounts.escrow_vault.key,
            token_program: *accounts.token_program.key,
            a_token_vault: *accounts.a_token_vault.key,
            b_token_vault: *accounts.b_token_vault.key,
            a_vault: *accounts.a_vault.key,
            b_vault: *accounts.b_vault.key,
            a_vault_lp: *accounts.a_vault_lp.key,
            b_vault_lp: *accounts.b_vault_lp.key,
            a_vault_lp_mint: *accounts.a_vault_lp_mint.key,
            b_vault_lp_mint: *accounts.b_vault_lp_mint.key,
            user_a_token: *accounts.user_a_token.key,
            user_b_token: *accounts.user_b_token.key,
            vault_program: *accounts.vault_program.key,
        }
    }
}
impl From<ClaimFeeKeys> for [AccountMeta; CLAIM_FEE_IX_ACCOUNTS_LEN] {
    fn from(keys: ClaimFeeKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.pool,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.lp_mint,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.lock_escrow,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.owner,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.source_tokens,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.escrow_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_program,
                is_signer: false,
                is_writable: false,
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
                pubkey: keys.user_a_token,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_b_token,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.vault_program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; CLAIM_FEE_IX_ACCOUNTS_LEN]> for ClaimFeeKeys {
    fn from(pubkeys: [Pubkey; CLAIM_FEE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            pool: pubkeys[0],
            lp_mint: pubkeys[1],
            lock_escrow: pubkeys[2],
            owner: pubkeys[3],
            source_tokens: pubkeys[4],
            escrow_vault: pubkeys[5],
            token_program: pubkeys[6],
            a_token_vault: pubkeys[7],
            b_token_vault: pubkeys[8],
            a_vault: pubkeys[9],
            b_vault: pubkeys[10],
            a_vault_lp: pubkeys[11],
            b_vault_lp: pubkeys[12],
            a_vault_lp_mint: pubkeys[13],
            b_vault_lp_mint: pubkeys[14],
            user_a_token: pubkeys[15],
            user_b_token: pubkeys[16],
            vault_program: pubkeys[17],
        }
    }
}
impl<'info> From<ClaimFeeAccounts<'_, 'info>>
for [AccountInfo<'info>; CLAIM_FEE_IX_ACCOUNTS_LEN] {
    fn from(accounts: ClaimFeeAccounts<'_, 'info>) -> Self {
        [
            accounts.pool.clone(),
            accounts.lp_mint.clone(),
            accounts.lock_escrow.clone(),
            accounts.owner.clone(),
            accounts.source_tokens.clone(),
            accounts.escrow_vault.clone(),
            accounts.token_program.clone(),
            accounts.a_token_vault.clone(),
            accounts.b_token_vault.clone(),
            accounts.a_vault.clone(),
            accounts.b_vault.clone(),
            accounts.a_vault_lp.clone(),
            accounts.b_vault_lp.clone(),
            accounts.a_vault_lp_mint.clone(),
            accounts.b_vault_lp_mint.clone(),
            accounts.user_a_token.clone(),
            accounts.user_b_token.clone(),
            accounts.vault_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; CLAIM_FEE_IX_ACCOUNTS_LEN]>
for ClaimFeeAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; CLAIM_FEE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            pool: &arr[0],
            lp_mint: &arr[1],
            lock_escrow: &arr[2],
            owner: &arr[3],
            source_tokens: &arr[4],
            escrow_vault: &arr[5],
            token_program: &arr[6],
            a_token_vault: &arr[7],
            b_token_vault: &arr[8],
            a_vault: &arr[9],
            b_vault: &arr[10],
            a_vault_lp: &arr[11],
            b_vault_lp: &arr[12],
            a_vault_lp_mint: &arr[13],
            b_vault_lp_mint: &arr[14],
            user_a_token: &arr[15],
            user_b_token: &arr[16],
            vault_program: &arr[17],
        }
    }
}
pub const CLAIM_FEE_IX_DISCM: [u8; 8] = [169, 32, 79, 137, 136, 232, 70, 137];
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ClaimFeeIxArgs {
    pub max_amount: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct ClaimFeeIxData(pub ClaimFeeIxArgs);
impl From<ClaimFeeIxArgs> for ClaimFeeIxData {
    fn from(args: ClaimFeeIxArgs) -> Self {
        Self(args)
    }
}
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
        Ok(Self(ClaimFeeIxArgs::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&CLAIM_FEE_IX_DISCM)?;
        self.0.serialize(&mut writer)
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
    args: ClaimFeeIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; CLAIM_FEE_IX_ACCOUNTS_LEN] = keys.into();
    let data: ClaimFeeIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn claim_fee_ix(
    keys: ClaimFeeKeys,
    args: ClaimFeeIxArgs,
) -> std::io::Result<Instruction> {
    claim_fee_ix_with_program_id(crate::ID, keys, args)
}
pub fn claim_fee_invoke_with_program_id(
    program_id: Pubkey,
    accounts: ClaimFeeAccounts<'_, '_>,
    args: ClaimFeeIxArgs,
) -> ProgramResult {
    let keys: ClaimFeeKeys = accounts.into();
    let ix = claim_fee_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn claim_fee_invoke(
    accounts: ClaimFeeAccounts<'_, '_>,
    args: ClaimFeeIxArgs,
) -> ProgramResult {
    claim_fee_invoke_with_program_id(crate::ID, accounts, args)
}
pub fn claim_fee_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: ClaimFeeAccounts<'_, '_>,
    args: ClaimFeeIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: ClaimFeeKeys = accounts.into();
    let ix = claim_fee_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn claim_fee_invoke_signed(
    accounts: ClaimFeeAccounts<'_, '_>,
    args: ClaimFeeIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    claim_fee_invoke_signed_with_program_id(crate::ID, accounts, args, seeds)
}
pub fn claim_fee_verify_account_keys(
    accounts: ClaimFeeAccounts<'_, '_>,
    keys: ClaimFeeKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.pool.key, keys.pool),
        (*accounts.lp_mint.key, keys.lp_mint),
        (*accounts.lock_escrow.key, keys.lock_escrow),
        (*accounts.owner.key, keys.owner),
        (*accounts.source_tokens.key, keys.source_tokens),
        (*accounts.escrow_vault.key, keys.escrow_vault),
        (*accounts.token_program.key, keys.token_program),
        (*accounts.a_token_vault.key, keys.a_token_vault),
        (*accounts.b_token_vault.key, keys.b_token_vault),
        (*accounts.a_vault.key, keys.a_vault),
        (*accounts.b_vault.key, keys.b_vault),
        (*accounts.a_vault_lp.key, keys.a_vault_lp),
        (*accounts.b_vault_lp.key, keys.b_vault_lp),
        (*accounts.a_vault_lp_mint.key, keys.a_vault_lp_mint),
        (*accounts.b_vault_lp_mint.key, keys.b_vault_lp_mint),
        (*accounts.user_a_token.key, keys.user_a_token),
        (*accounts.user_b_token.key, keys.user_b_token),
        (*accounts.vault_program.key, keys.vault_program),
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
        accounts.pool,
        accounts.lp_mint,
        accounts.lock_escrow,
        accounts.owner,
        accounts.source_tokens,
        accounts.escrow_vault,
        accounts.a_token_vault,
        accounts.b_token_vault,
        accounts.a_vault,
        accounts.b_vault,
        accounts.a_vault_lp,
        accounts.b_vault_lp,
        accounts.a_vault_lp_mint,
        accounts.b_vault_lp_mint,
        accounts.user_a_token,
        accounts.user_b_token,
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
    for should_be_signer in [accounts.owner] {
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
pub const CREATE_CONFIG_IX_ACCOUNTS_LEN: usize = 3;
#[derive(Copy, Clone, Debug)]
pub struct CreateConfigAccounts<'me, 'info> {
    pub config: &'me AccountInfo<'info>,
    pub admin: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CreateConfigKeys {
    pub config: Pubkey,
    pub admin: Pubkey,
    pub system_program: Pubkey,
}
impl From<CreateConfigAccounts<'_, '_>> for CreateConfigKeys {
    fn from(accounts: CreateConfigAccounts) -> Self {
        Self {
            config: *accounts.config.key,
            admin: *accounts.admin.key,
            system_program: *accounts.system_program.key,
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
        ]
    }
}
impl From<[Pubkey; CREATE_CONFIG_IX_ACCOUNTS_LEN]> for CreateConfigKeys {
    fn from(pubkeys: [Pubkey; CREATE_CONFIG_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            config: pubkeys[0],
            admin: pubkeys[1],
            system_program: pubkeys[2],
        }
    }
}
impl<'info> From<CreateConfigAccounts<'_, 'info>>
for [AccountInfo<'info>; CREATE_CONFIG_IX_ACCOUNTS_LEN] {
    fn from(accounts: CreateConfigAccounts<'_, 'info>) -> Self {
        [
            accounts.config.clone(),
            accounts.admin.clone(),
            accounts.system_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; CREATE_CONFIG_IX_ACCOUNTS_LEN]>
for CreateConfigAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; CREATE_CONFIG_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            config: &arr[0],
            admin: &arr[1],
            system_program: &arr[2],
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
    for should_be_writable in [accounts.config, accounts.admin] {
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
pub const CLOSE_CONFIG_IX_ACCOUNTS_LEN: usize = 3;
#[derive(Copy, Clone, Debug)]
pub struct CloseConfigAccounts<'me, 'info> {
    pub config: &'me AccountInfo<'info>,
    pub admin: &'me AccountInfo<'info>,
    pub rent_receiver: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CloseConfigKeys {
    pub config: Pubkey,
    pub admin: Pubkey,
    pub rent_receiver: Pubkey,
}
impl From<CloseConfigAccounts<'_, '_>> for CloseConfigKeys {
    fn from(accounts: CloseConfigAccounts) -> Self {
        Self {
            config: *accounts.config.key,
            admin: *accounts.admin.key,
            rent_receiver: *accounts.rent_receiver.key,
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
        ]
    }
}
impl From<[Pubkey; CLOSE_CONFIG_IX_ACCOUNTS_LEN]> for CloseConfigKeys {
    fn from(pubkeys: [Pubkey; CLOSE_CONFIG_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            config: pubkeys[0],
            admin: pubkeys[1],
            rent_receiver: pubkeys[2],
        }
    }
}
impl<'info> From<CloseConfigAccounts<'_, 'info>>
for [AccountInfo<'info>; CLOSE_CONFIG_IX_ACCOUNTS_LEN] {
    fn from(accounts: CloseConfigAccounts<'_, 'info>) -> Self {
        [accounts.config.clone(), accounts.admin.clone(), accounts.rent_receiver.clone()]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; CLOSE_CONFIG_IX_ACCOUNTS_LEN]>
for CloseConfigAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; CLOSE_CONFIG_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            config: &arr[0],
            admin: &arr[1],
            rent_receiver: &arr[2],
        }
    }
}
pub const CLOSE_CONFIG_IX_DISCM: [u8; 8] = [145, 9, 72, 157, 95, 125, 61, 85];
#[derive(Clone, Debug, PartialEq)]
pub struct CloseConfigIxData;
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
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&CLOSE_CONFIG_IX_DISCM)
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
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; CLOSE_CONFIG_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: CloseConfigIxData.try_to_vec()?,
    })
}
pub fn close_config_ix(keys: CloseConfigKeys) -> std::io::Result<Instruction> {
    close_config_ix_with_program_id(crate::ID, keys)
}
pub fn close_config_invoke_with_program_id(
    program_id: Pubkey,
    accounts: CloseConfigAccounts<'_, '_>,
) -> ProgramResult {
    let keys: CloseConfigKeys = accounts.into();
    let ix = close_config_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn close_config_invoke(accounts: CloseConfigAccounts<'_, '_>) -> ProgramResult {
    close_config_invoke_with_program_id(crate::ID, accounts)
}
pub fn close_config_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: CloseConfigAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: CloseConfigKeys = accounts.into();
    let ix = close_config_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn close_config_invoke_signed(
    accounts: CloseConfigAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    close_config_invoke_signed_with_program_id(crate::ID, accounts, seeds)
}
pub fn close_config_verify_account_keys(
    accounts: CloseConfigAccounts<'_, '_>,
    keys: CloseConfigKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.config.key, keys.config),
        (*accounts.admin.key, keys.admin),
        (*accounts.rent_receiver.key, keys.rent_receiver),
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
    for should_be_writable in [accounts.config, accounts.admin, accounts.rent_receiver] {
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
pub const INITIALIZE_PERMISSIONLESS_CONSTANT_PRODUCT_POOL_WITH_CONFIG_IX_ACCOUNTS_LEN: usize = 26;
#[derive(Copy, Clone, Debug)]
pub struct InitializePermissionlessConstantProductPoolWithConfigAccounts<'me, 'info> {
    pub pool: &'me AccountInfo<'info>,
    pub config: &'me AccountInfo<'info>,
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
    pub payer_token_a: &'me AccountInfo<'info>,
    pub payer_token_b: &'me AccountInfo<'info>,
    pub payer_pool_lp: &'me AccountInfo<'info>,
    pub protocol_token_a_fee: &'me AccountInfo<'info>,
    pub protocol_token_b_fee: &'me AccountInfo<'info>,
    pub payer: &'me AccountInfo<'info>,
    pub rent: &'me AccountInfo<'info>,
    pub mint_metadata: &'me AccountInfo<'info>,
    pub metadata_program: &'me AccountInfo<'info>,
    pub vault_program: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub associated_token_program: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct InitializePermissionlessConstantProductPoolWithConfigKeys {
    pub pool: Pubkey,
    pub config: Pubkey,
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
    pub payer_token_a: Pubkey,
    pub payer_token_b: Pubkey,
    pub payer_pool_lp: Pubkey,
    pub protocol_token_a_fee: Pubkey,
    pub protocol_token_b_fee: Pubkey,
    pub payer: Pubkey,
    pub rent: Pubkey,
    pub mint_metadata: Pubkey,
    pub metadata_program: Pubkey,
    pub vault_program: Pubkey,
    pub token_program: Pubkey,
    pub associated_token_program: Pubkey,
    pub system_program: Pubkey,
}
impl From<InitializePermissionlessConstantProductPoolWithConfigAccounts<'_, '_>>
for InitializePermissionlessConstantProductPoolWithConfigKeys {
    fn from(
        accounts: InitializePermissionlessConstantProductPoolWithConfigAccounts,
    ) -> Self {
        Self {
            pool: *accounts.pool.key,
            config: *accounts.config.key,
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
            payer_token_a: *accounts.payer_token_a.key,
            payer_token_b: *accounts.payer_token_b.key,
            payer_pool_lp: *accounts.payer_pool_lp.key,
            protocol_token_a_fee: *accounts.protocol_token_a_fee.key,
            protocol_token_b_fee: *accounts.protocol_token_b_fee.key,
            payer: *accounts.payer.key,
            rent: *accounts.rent.key,
            mint_metadata: *accounts.mint_metadata.key,
            metadata_program: *accounts.metadata_program.key,
            vault_program: *accounts.vault_program.key,
            token_program: *accounts.token_program.key,
            associated_token_program: *accounts.associated_token_program.key,
            system_program: *accounts.system_program.key,
        }
    }
}
impl From<InitializePermissionlessConstantProductPoolWithConfigKeys>
for [AccountMeta; INITIALIZE_PERMISSIONLESS_CONSTANT_PRODUCT_POOL_WITH_CONFIG_IX_ACCOUNTS_LEN] {
    fn from(keys: InitializePermissionlessConstantProductPoolWithConfigKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.pool,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.config,
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
                is_writable: false,
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
                pubkey: keys.payer_pool_lp,
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
impl From<
    [Pubkey; INITIALIZE_PERMISSIONLESS_CONSTANT_PRODUCT_POOL_WITH_CONFIG_IX_ACCOUNTS_LEN],
> for InitializePermissionlessConstantProductPoolWithConfigKeys {
    fn from(
        pubkeys: [Pubkey; INITIALIZE_PERMISSIONLESS_CONSTANT_PRODUCT_POOL_WITH_CONFIG_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            pool: pubkeys[0],
            config: pubkeys[1],
            lp_mint: pubkeys[2],
            token_a_mint: pubkeys[3],
            token_b_mint: pubkeys[4],
            a_vault: pubkeys[5],
            b_vault: pubkeys[6],
            a_token_vault: pubkeys[7],
            b_token_vault: pubkeys[8],
            a_vault_lp_mint: pubkeys[9],
            b_vault_lp_mint: pubkeys[10],
            a_vault_lp: pubkeys[11],
            b_vault_lp: pubkeys[12],
            payer_token_a: pubkeys[13],
            payer_token_b: pubkeys[14],
            payer_pool_lp: pubkeys[15],
            protocol_token_a_fee: pubkeys[16],
            protocol_token_b_fee: pubkeys[17],
            payer: pubkeys[18],
            rent: pubkeys[19],
            mint_metadata: pubkeys[20],
            metadata_program: pubkeys[21],
            vault_program: pubkeys[22],
            token_program: pubkeys[23],
            associated_token_program: pubkeys[24],
            system_program: pubkeys[25],
        }
    }
}
impl<
    'info,
> From<InitializePermissionlessConstantProductPoolWithConfigAccounts<'_, 'info>>
for [AccountInfo<
    'info,
>; INITIALIZE_PERMISSIONLESS_CONSTANT_PRODUCT_POOL_WITH_CONFIG_IX_ACCOUNTS_LEN] {
    fn from(
        accounts: InitializePermissionlessConstantProductPoolWithConfigAccounts<
            '_,
            'info,
        >,
    ) -> Self {
        [
            accounts.pool.clone(),
            accounts.config.clone(),
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
            accounts.payer_token_a.clone(),
            accounts.payer_token_b.clone(),
            accounts.payer_pool_lp.clone(),
            accounts.protocol_token_a_fee.clone(),
            accounts.protocol_token_b_fee.clone(),
            accounts.payer.clone(),
            accounts.rent.clone(),
            accounts.mint_metadata.clone(),
            accounts.metadata_program.clone(),
            accounts.vault_program.clone(),
            accounts.token_program.clone(),
            accounts.associated_token_program.clone(),
            accounts.system_program.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<
    &'me [AccountInfo<
        'info,
    >; INITIALIZE_PERMISSIONLESS_CONSTANT_PRODUCT_POOL_WITH_CONFIG_IX_ACCOUNTS_LEN],
> for InitializePermissionlessConstantProductPoolWithConfigAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<
            'info,
        >; INITIALIZE_PERMISSIONLESS_CONSTANT_PRODUCT_POOL_WITH_CONFIG_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            pool: &arr[0],
            config: &arr[1],
            lp_mint: &arr[2],
            token_a_mint: &arr[3],
            token_b_mint: &arr[4],
            a_vault: &arr[5],
            b_vault: &arr[6],
            a_token_vault: &arr[7],
            b_token_vault: &arr[8],
            a_vault_lp_mint: &arr[9],
            b_vault_lp_mint: &arr[10],
            a_vault_lp: &arr[11],
            b_vault_lp: &arr[12],
            payer_token_a: &arr[13],
            payer_token_b: &arr[14],
            payer_pool_lp: &arr[15],
            protocol_token_a_fee: &arr[16],
            protocol_token_b_fee: &arr[17],
            payer: &arr[18],
            rent: &arr[19],
            mint_metadata: &arr[20],
            metadata_program: &arr[21],
            vault_program: &arr[22],
            token_program: &arr[23],
            associated_token_program: &arr[24],
            system_program: &arr[25],
        }
    }
}
pub const INITIALIZE_PERMISSIONLESS_CONSTANT_PRODUCT_POOL_WITH_CONFIG_IX_DISCM: [u8; 8] = [
    7,
    166,
    138,
    171,
    206,
    171,
    236,
    244,
];
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InitializePermissionlessConstantProductPoolWithConfigIxArgs {
    pub token_a_amount: u64,
    pub token_b_amount: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct InitializePermissionlessConstantProductPoolWithConfigIxData(
    pub InitializePermissionlessConstantProductPoolWithConfigIxArgs,
);
impl From<InitializePermissionlessConstantProductPoolWithConfigIxArgs>
for InitializePermissionlessConstantProductPoolWithConfigIxData {
    fn from(args: InitializePermissionlessConstantProductPoolWithConfigIxArgs) -> Self {
        Self(args)
    }
}
impl InitializePermissionlessConstantProductPoolWithConfigIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm
            != INITIALIZE_PERMISSIONLESS_CONSTANT_PRODUCT_POOL_WITH_CONFIG_IX_DISCM
        {
            return Err(
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!(
                        "discm does not match. Expected: {:?}. Received: {:?}",
                        INITIALIZE_PERMISSIONLESS_CONSTANT_PRODUCT_POOL_WITH_CONFIG_IX_DISCM,
                        maybe_discm
                    ),
                ),
            );
        }
        Ok(
            Self(
                InitializePermissionlessConstantProductPoolWithConfigIxArgs::deserialize(
                    &mut reader,
                )?,
            ),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer
            .write_all(
                &INITIALIZE_PERMISSIONLESS_CONSTANT_PRODUCT_POOL_WITH_CONFIG_IX_DISCM,
            )?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn initialize_permissionless_constant_product_pool_with_config_ix_with_program_id(
    program_id: Pubkey,
    keys: InitializePermissionlessConstantProductPoolWithConfigKeys,
    args: InitializePermissionlessConstantProductPoolWithConfigIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; INITIALIZE_PERMISSIONLESS_CONSTANT_PRODUCT_POOL_WITH_CONFIG_IX_ACCOUNTS_LEN] = keys
        .into();
    let data: InitializePermissionlessConstantProductPoolWithConfigIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn initialize_permissionless_constant_product_pool_with_config_ix(
    keys: InitializePermissionlessConstantProductPoolWithConfigKeys,
    args: InitializePermissionlessConstantProductPoolWithConfigIxArgs,
) -> std::io::Result<Instruction> {
    initialize_permissionless_constant_product_pool_with_config_ix_with_program_id(
        crate::ID,
        keys,
        args,
    )
}
pub fn initialize_permissionless_constant_product_pool_with_config_invoke_with_program_id(
    program_id: Pubkey,
    accounts: InitializePermissionlessConstantProductPoolWithConfigAccounts<'_, '_>,
    args: InitializePermissionlessConstantProductPoolWithConfigIxArgs,
) -> ProgramResult {
    let keys: InitializePermissionlessConstantProductPoolWithConfigKeys = accounts
        .into();
    let ix = initialize_permissionless_constant_product_pool_with_config_ix_with_program_id(
        program_id,
        keys,
        args,
    )?;
    invoke_instruction(&ix, accounts)
}
pub fn initialize_permissionless_constant_product_pool_with_config_invoke(
    accounts: InitializePermissionlessConstantProductPoolWithConfigAccounts<'_, '_>,
    args: InitializePermissionlessConstantProductPoolWithConfigIxArgs,
) -> ProgramResult {
    initialize_permissionless_constant_product_pool_with_config_invoke_with_program_id(
        crate::ID,
        accounts,
        args,
    )
}
pub fn initialize_permissionless_constant_product_pool_with_config_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: InitializePermissionlessConstantProductPoolWithConfigAccounts<'_, '_>,
    args: InitializePermissionlessConstantProductPoolWithConfigIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: InitializePermissionlessConstantProductPoolWithConfigKeys = accounts
        .into();
    let ix = initialize_permissionless_constant_product_pool_with_config_ix_with_program_id(
        program_id,
        keys,
        args,
    )?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn initialize_permissionless_constant_product_pool_with_config_invoke_signed(
    accounts: InitializePermissionlessConstantProductPoolWithConfigAccounts<'_, '_>,
    args: InitializePermissionlessConstantProductPoolWithConfigIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    initialize_permissionless_constant_product_pool_with_config_invoke_signed_with_program_id(
        crate::ID,
        accounts,
        args,
        seeds,
    )
}
pub fn initialize_permissionless_constant_product_pool_with_config_verify_account_keys(
    accounts: InitializePermissionlessConstantProductPoolWithConfigAccounts<'_, '_>,
    keys: InitializePermissionlessConstantProductPoolWithConfigKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.pool.key, keys.pool),
        (*accounts.config.key, keys.config),
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
        (*accounts.payer_token_a.key, keys.payer_token_a),
        (*accounts.payer_token_b.key, keys.payer_token_b),
        (*accounts.payer_pool_lp.key, keys.payer_pool_lp),
        (*accounts.protocol_token_a_fee.key, keys.protocol_token_a_fee),
        (*accounts.protocol_token_b_fee.key, keys.protocol_token_b_fee),
        (*accounts.payer.key, keys.payer),
        (*accounts.rent.key, keys.rent),
        (*accounts.mint_metadata.key, keys.mint_metadata),
        (*accounts.metadata_program.key, keys.metadata_program),
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
pub fn initialize_permissionless_constant_product_pool_with_config_verify_writable_privileges<
    'me,
    'info,
>(
    accounts: InitializePermissionlessConstantProductPoolWithConfigAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.pool,
        accounts.lp_mint,
        accounts.a_vault,
        accounts.b_vault,
        accounts.a_token_vault,
        accounts.b_token_vault,
        accounts.a_vault_lp_mint,
        accounts.b_vault_lp_mint,
        accounts.a_vault_lp,
        accounts.b_vault_lp,
        accounts.payer_token_a,
        accounts.payer_token_b,
        accounts.payer_pool_lp,
        accounts.protocol_token_a_fee,
        accounts.protocol_token_b_fee,
        accounts.payer,
        accounts.mint_metadata,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn initialize_permissionless_constant_product_pool_with_config_verify_signer_privileges<
    'me,
    'info,
>(
    accounts: InitializePermissionlessConstantProductPoolWithConfigAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.payer] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn initialize_permissionless_constant_product_pool_with_config_verify_account_privileges<
    'me,
    'info,
>(
    accounts: InitializePermissionlessConstantProductPoolWithConfigAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    initialize_permissionless_constant_product_pool_with_config_verify_writable_privileges(
        accounts,
    )?;
    initialize_permissionless_constant_product_pool_with_config_verify_signer_privileges(
        accounts,
    )?;
    Ok(())
}
pub const INITIALIZE_PERMISSIONLESS_CONSTANT_PRODUCT_POOL_WITH_CONFIG2_IX_ACCOUNTS_LEN: usize = 26;
#[derive(Copy, Clone, Debug)]
pub struct InitializePermissionlessConstantProductPoolWithConfig2Accounts<'me, 'info> {
    pub pool: &'me AccountInfo<'info>,
    pub config: &'me AccountInfo<'info>,
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
    pub payer_token_a: &'me AccountInfo<'info>,
    pub payer_token_b: &'me AccountInfo<'info>,
    pub payer_pool_lp: &'me AccountInfo<'info>,
    pub protocol_token_a_fee: &'me AccountInfo<'info>,
    pub protocol_token_b_fee: &'me AccountInfo<'info>,
    pub payer: &'me AccountInfo<'info>,
    pub rent: &'me AccountInfo<'info>,
    pub mint_metadata: &'me AccountInfo<'info>,
    pub metadata_program: &'me AccountInfo<'info>,
    pub vault_program: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub associated_token_program: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct InitializePermissionlessConstantProductPoolWithConfig2Keys {
    pub pool: Pubkey,
    pub config: Pubkey,
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
    pub payer_token_a: Pubkey,
    pub payer_token_b: Pubkey,
    pub payer_pool_lp: Pubkey,
    pub protocol_token_a_fee: Pubkey,
    pub protocol_token_b_fee: Pubkey,
    pub payer: Pubkey,
    pub rent: Pubkey,
    pub mint_metadata: Pubkey,
    pub metadata_program: Pubkey,
    pub vault_program: Pubkey,
    pub token_program: Pubkey,
    pub associated_token_program: Pubkey,
    pub system_program: Pubkey,
}
impl From<InitializePermissionlessConstantProductPoolWithConfig2Accounts<'_, '_>>
for InitializePermissionlessConstantProductPoolWithConfig2Keys {
    fn from(
        accounts: InitializePermissionlessConstantProductPoolWithConfig2Accounts,
    ) -> Self {
        Self {
            pool: *accounts.pool.key,
            config: *accounts.config.key,
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
            payer_token_a: *accounts.payer_token_a.key,
            payer_token_b: *accounts.payer_token_b.key,
            payer_pool_lp: *accounts.payer_pool_lp.key,
            protocol_token_a_fee: *accounts.protocol_token_a_fee.key,
            protocol_token_b_fee: *accounts.protocol_token_b_fee.key,
            payer: *accounts.payer.key,
            rent: *accounts.rent.key,
            mint_metadata: *accounts.mint_metadata.key,
            metadata_program: *accounts.metadata_program.key,
            vault_program: *accounts.vault_program.key,
            token_program: *accounts.token_program.key,
            associated_token_program: *accounts.associated_token_program.key,
            system_program: *accounts.system_program.key,
        }
    }
}
impl From<InitializePermissionlessConstantProductPoolWithConfig2Keys>
for [AccountMeta; INITIALIZE_PERMISSIONLESS_CONSTANT_PRODUCT_POOL_WITH_CONFIG2_IX_ACCOUNTS_LEN] {
    fn from(keys: InitializePermissionlessConstantProductPoolWithConfig2Keys) -> Self {
        [
            AccountMeta {
                pubkey: keys.pool,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.config,
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
                is_writable: false,
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
                pubkey: keys.payer_pool_lp,
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
impl From<
    [Pubkey; INITIALIZE_PERMISSIONLESS_CONSTANT_PRODUCT_POOL_WITH_CONFIG2_IX_ACCOUNTS_LEN],
> for InitializePermissionlessConstantProductPoolWithConfig2Keys {
    fn from(
        pubkeys: [Pubkey; INITIALIZE_PERMISSIONLESS_CONSTANT_PRODUCT_POOL_WITH_CONFIG2_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            pool: pubkeys[0],
            config: pubkeys[1],
            lp_mint: pubkeys[2],
            token_a_mint: pubkeys[3],
            token_b_mint: pubkeys[4],
            a_vault: pubkeys[5],
            b_vault: pubkeys[6],
            a_token_vault: pubkeys[7],
            b_token_vault: pubkeys[8],
            a_vault_lp_mint: pubkeys[9],
            b_vault_lp_mint: pubkeys[10],
            a_vault_lp: pubkeys[11],
            b_vault_lp: pubkeys[12],
            payer_token_a: pubkeys[13],
            payer_token_b: pubkeys[14],
            payer_pool_lp: pubkeys[15],
            protocol_token_a_fee: pubkeys[16],
            protocol_token_b_fee: pubkeys[17],
            payer: pubkeys[18],
            rent: pubkeys[19],
            mint_metadata: pubkeys[20],
            metadata_program: pubkeys[21],
            vault_program: pubkeys[22],
            token_program: pubkeys[23],
            associated_token_program: pubkeys[24],
            system_program: pubkeys[25],
        }
    }
}
impl<
    'info,
> From<InitializePermissionlessConstantProductPoolWithConfig2Accounts<'_, 'info>>
for [AccountInfo<
    'info,
>; INITIALIZE_PERMISSIONLESS_CONSTANT_PRODUCT_POOL_WITH_CONFIG2_IX_ACCOUNTS_LEN] {
    fn from(
        accounts: InitializePermissionlessConstantProductPoolWithConfig2Accounts<
            '_,
            'info,
        >,
    ) -> Self {
        [
            accounts.pool.clone(),
            accounts.config.clone(),
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
            accounts.payer_token_a.clone(),
            accounts.payer_token_b.clone(),
            accounts.payer_pool_lp.clone(),
            accounts.protocol_token_a_fee.clone(),
            accounts.protocol_token_b_fee.clone(),
            accounts.payer.clone(),
            accounts.rent.clone(),
            accounts.mint_metadata.clone(),
            accounts.metadata_program.clone(),
            accounts.vault_program.clone(),
            accounts.token_program.clone(),
            accounts.associated_token_program.clone(),
            accounts.system_program.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<
    &'me [AccountInfo<
        'info,
    >; INITIALIZE_PERMISSIONLESS_CONSTANT_PRODUCT_POOL_WITH_CONFIG2_IX_ACCOUNTS_LEN],
> for InitializePermissionlessConstantProductPoolWithConfig2Accounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<
            'info,
        >; INITIALIZE_PERMISSIONLESS_CONSTANT_PRODUCT_POOL_WITH_CONFIG2_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            pool: &arr[0],
            config: &arr[1],
            lp_mint: &arr[2],
            token_a_mint: &arr[3],
            token_b_mint: &arr[4],
            a_vault: &arr[5],
            b_vault: &arr[6],
            a_token_vault: &arr[7],
            b_token_vault: &arr[8],
            a_vault_lp_mint: &arr[9],
            b_vault_lp_mint: &arr[10],
            a_vault_lp: &arr[11],
            b_vault_lp: &arr[12],
            payer_token_a: &arr[13],
            payer_token_b: &arr[14],
            payer_pool_lp: &arr[15],
            protocol_token_a_fee: &arr[16],
            protocol_token_b_fee: &arr[17],
            payer: &arr[18],
            rent: &arr[19],
            mint_metadata: &arr[20],
            metadata_program: &arr[21],
            vault_program: &arr[22],
            token_program: &arr[23],
            associated_token_program: &arr[24],
            system_program: &arr[25],
        }
    }
}
pub const INITIALIZE_PERMISSIONLESS_CONSTANT_PRODUCT_POOL_WITH_CONFIG2_IX_DISCM: [u8; 8] = [
    48,
    149,
    220,
    130,
    61,
    11,
    9,
    178,
];
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InitializePermissionlessConstantProductPoolWithConfig2IxArgs {
    pub token_a_amount: u64,
    pub token_b_amount: u64,
    pub activation_point: Option<u64>,
}
#[derive(Clone, Debug, PartialEq)]
pub struct InitializePermissionlessConstantProductPoolWithConfig2IxData(
    pub InitializePermissionlessConstantProductPoolWithConfig2IxArgs,
);
impl From<InitializePermissionlessConstantProductPoolWithConfig2IxArgs>
for InitializePermissionlessConstantProductPoolWithConfig2IxData {
    fn from(args: InitializePermissionlessConstantProductPoolWithConfig2IxArgs) -> Self {
        Self(args)
    }
}
impl InitializePermissionlessConstantProductPoolWithConfig2IxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm
            != INITIALIZE_PERMISSIONLESS_CONSTANT_PRODUCT_POOL_WITH_CONFIG2_IX_DISCM
        {
            return Err(
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!(
                        "discm does not match. Expected: {:?}. Received: {:?}",
                        INITIALIZE_PERMISSIONLESS_CONSTANT_PRODUCT_POOL_WITH_CONFIG2_IX_DISCM,
                        maybe_discm
                    ),
                ),
            );
        }
        Ok(
            Self(
                InitializePermissionlessConstantProductPoolWithConfig2IxArgs::deserialize(
                    &mut reader,
                )?,
            ),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer
            .write_all(
                &INITIALIZE_PERMISSIONLESS_CONSTANT_PRODUCT_POOL_WITH_CONFIG2_IX_DISCM,
            )?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn initialize_permissionless_constant_product_pool_with_config2_ix_with_program_id(
    program_id: Pubkey,
    keys: InitializePermissionlessConstantProductPoolWithConfig2Keys,
    args: InitializePermissionlessConstantProductPoolWithConfig2IxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; INITIALIZE_PERMISSIONLESS_CONSTANT_PRODUCT_POOL_WITH_CONFIG2_IX_ACCOUNTS_LEN] = keys
        .into();
    let data: InitializePermissionlessConstantProductPoolWithConfig2IxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn initialize_permissionless_constant_product_pool_with_config2_ix(
    keys: InitializePermissionlessConstantProductPoolWithConfig2Keys,
    args: InitializePermissionlessConstantProductPoolWithConfig2IxArgs,
) -> std::io::Result<Instruction> {
    initialize_permissionless_constant_product_pool_with_config2_ix_with_program_id(
        crate::ID,
        keys,
        args,
    )
}
pub fn initialize_permissionless_constant_product_pool_with_config2_invoke_with_program_id(
    program_id: Pubkey,
    accounts: InitializePermissionlessConstantProductPoolWithConfig2Accounts<'_, '_>,
    args: InitializePermissionlessConstantProductPoolWithConfig2IxArgs,
) -> ProgramResult {
    let keys: InitializePermissionlessConstantProductPoolWithConfig2Keys = accounts
        .into();
    let ix = initialize_permissionless_constant_product_pool_with_config2_ix_with_program_id(
        program_id,
        keys,
        args,
    )?;
    invoke_instruction(&ix, accounts)
}
pub fn initialize_permissionless_constant_product_pool_with_config2_invoke(
    accounts: InitializePermissionlessConstantProductPoolWithConfig2Accounts<'_, '_>,
    args: InitializePermissionlessConstantProductPoolWithConfig2IxArgs,
) -> ProgramResult {
    initialize_permissionless_constant_product_pool_with_config2_invoke_with_program_id(
        crate::ID,
        accounts,
        args,
    )
}
pub fn initialize_permissionless_constant_product_pool_with_config2_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: InitializePermissionlessConstantProductPoolWithConfig2Accounts<'_, '_>,
    args: InitializePermissionlessConstantProductPoolWithConfig2IxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: InitializePermissionlessConstantProductPoolWithConfig2Keys = accounts
        .into();
    let ix = initialize_permissionless_constant_product_pool_with_config2_ix_with_program_id(
        program_id,
        keys,
        args,
    )?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn initialize_permissionless_constant_product_pool_with_config2_invoke_signed(
    accounts: InitializePermissionlessConstantProductPoolWithConfig2Accounts<'_, '_>,
    args: InitializePermissionlessConstantProductPoolWithConfig2IxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    initialize_permissionless_constant_product_pool_with_config2_invoke_signed_with_program_id(
        crate::ID,
        accounts,
        args,
        seeds,
    )
}
pub fn initialize_permissionless_constant_product_pool_with_config2_verify_account_keys(
    accounts: InitializePermissionlessConstantProductPoolWithConfig2Accounts<'_, '_>,
    keys: InitializePermissionlessConstantProductPoolWithConfig2Keys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.pool.key, keys.pool),
        (*accounts.config.key, keys.config),
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
        (*accounts.payer_token_a.key, keys.payer_token_a),
        (*accounts.payer_token_b.key, keys.payer_token_b),
        (*accounts.payer_pool_lp.key, keys.payer_pool_lp),
        (*accounts.protocol_token_a_fee.key, keys.protocol_token_a_fee),
        (*accounts.protocol_token_b_fee.key, keys.protocol_token_b_fee),
        (*accounts.payer.key, keys.payer),
        (*accounts.rent.key, keys.rent),
        (*accounts.mint_metadata.key, keys.mint_metadata),
        (*accounts.metadata_program.key, keys.metadata_program),
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
pub fn initialize_permissionless_constant_product_pool_with_config2_verify_writable_privileges<
    'me,
    'info,
>(
    accounts: InitializePermissionlessConstantProductPoolWithConfig2Accounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.pool,
        accounts.lp_mint,
        accounts.a_vault,
        accounts.b_vault,
        accounts.a_token_vault,
        accounts.b_token_vault,
        accounts.a_vault_lp_mint,
        accounts.b_vault_lp_mint,
        accounts.a_vault_lp,
        accounts.b_vault_lp,
        accounts.payer_token_a,
        accounts.payer_token_b,
        accounts.payer_pool_lp,
        accounts.protocol_token_a_fee,
        accounts.protocol_token_b_fee,
        accounts.payer,
        accounts.mint_metadata,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn initialize_permissionless_constant_product_pool_with_config2_verify_signer_privileges<
    'me,
    'info,
>(
    accounts: InitializePermissionlessConstantProductPoolWithConfig2Accounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.payer] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn initialize_permissionless_constant_product_pool_with_config2_verify_account_privileges<
    'me,
    'info,
>(
    accounts: InitializePermissionlessConstantProductPoolWithConfig2Accounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    initialize_permissionless_constant_product_pool_with_config2_verify_writable_privileges(
        accounts,
    )?;
    initialize_permissionless_constant_product_pool_with_config2_verify_signer_privileges(
        accounts,
    )?;
    Ok(())
}
pub const INITIALIZE_CUSTOMIZABLE_PERMISSIONLESS_CONSTANT_PRODUCT_POOL_IX_ACCOUNTS_LEN: usize = 25;
#[derive(Copy, Clone, Debug)]
pub struct InitializeCustomizablePermissionlessConstantProductPoolAccounts<'me, 'info> {
    pub pool: &'me AccountInfo<'info>,
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
    pub payer_token_a: &'me AccountInfo<'info>,
    pub payer_token_b: &'me AccountInfo<'info>,
    pub payer_pool_lp: &'me AccountInfo<'info>,
    pub protocol_token_a_fee: &'me AccountInfo<'info>,
    pub protocol_token_b_fee: &'me AccountInfo<'info>,
    pub payer: &'me AccountInfo<'info>,
    pub rent: &'me AccountInfo<'info>,
    pub mint_metadata: &'me AccountInfo<'info>,
    pub metadata_program: &'me AccountInfo<'info>,
    pub vault_program: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub associated_token_program: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct InitializeCustomizablePermissionlessConstantProductPoolKeys {
    pub pool: Pubkey,
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
    pub payer_token_a: Pubkey,
    pub payer_token_b: Pubkey,
    pub payer_pool_lp: Pubkey,
    pub protocol_token_a_fee: Pubkey,
    pub protocol_token_b_fee: Pubkey,
    pub payer: Pubkey,
    pub rent: Pubkey,
    pub mint_metadata: Pubkey,
    pub metadata_program: Pubkey,
    pub vault_program: Pubkey,
    pub token_program: Pubkey,
    pub associated_token_program: Pubkey,
    pub system_program: Pubkey,
}
impl From<InitializeCustomizablePermissionlessConstantProductPoolAccounts<'_, '_>>
for InitializeCustomizablePermissionlessConstantProductPoolKeys {
    fn from(
        accounts: InitializeCustomizablePermissionlessConstantProductPoolAccounts,
    ) -> Self {
        Self {
            pool: *accounts.pool.key,
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
            payer_token_a: *accounts.payer_token_a.key,
            payer_token_b: *accounts.payer_token_b.key,
            payer_pool_lp: *accounts.payer_pool_lp.key,
            protocol_token_a_fee: *accounts.protocol_token_a_fee.key,
            protocol_token_b_fee: *accounts.protocol_token_b_fee.key,
            payer: *accounts.payer.key,
            rent: *accounts.rent.key,
            mint_metadata: *accounts.mint_metadata.key,
            metadata_program: *accounts.metadata_program.key,
            vault_program: *accounts.vault_program.key,
            token_program: *accounts.token_program.key,
            associated_token_program: *accounts.associated_token_program.key,
            system_program: *accounts.system_program.key,
        }
    }
}
impl From<InitializeCustomizablePermissionlessConstantProductPoolKeys>
for [AccountMeta; INITIALIZE_CUSTOMIZABLE_PERMISSIONLESS_CONSTANT_PRODUCT_POOL_IX_ACCOUNTS_LEN] {
    fn from(keys: InitializeCustomizablePermissionlessConstantProductPoolKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.pool,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.lp_mint,
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
                pubkey: keys.payer_pool_lp,
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
impl From<
    [Pubkey; INITIALIZE_CUSTOMIZABLE_PERMISSIONLESS_CONSTANT_PRODUCT_POOL_IX_ACCOUNTS_LEN],
> for InitializeCustomizablePermissionlessConstantProductPoolKeys {
    fn from(
        pubkeys: [Pubkey; INITIALIZE_CUSTOMIZABLE_PERMISSIONLESS_CONSTANT_PRODUCT_POOL_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            pool: pubkeys[0],
            lp_mint: pubkeys[1],
            token_a_mint: pubkeys[2],
            token_b_mint: pubkeys[3],
            a_vault: pubkeys[4],
            b_vault: pubkeys[5],
            a_token_vault: pubkeys[6],
            b_token_vault: pubkeys[7],
            a_vault_lp_mint: pubkeys[8],
            b_vault_lp_mint: pubkeys[9],
            a_vault_lp: pubkeys[10],
            b_vault_lp: pubkeys[11],
            payer_token_a: pubkeys[12],
            payer_token_b: pubkeys[13],
            payer_pool_lp: pubkeys[14],
            protocol_token_a_fee: pubkeys[15],
            protocol_token_b_fee: pubkeys[16],
            payer: pubkeys[17],
            rent: pubkeys[18],
            mint_metadata: pubkeys[19],
            metadata_program: pubkeys[20],
            vault_program: pubkeys[21],
            token_program: pubkeys[22],
            associated_token_program: pubkeys[23],
            system_program: pubkeys[24],
        }
    }
}
impl<
    'info,
> From<InitializeCustomizablePermissionlessConstantProductPoolAccounts<'_, 'info>>
for [AccountInfo<
    'info,
>; INITIALIZE_CUSTOMIZABLE_PERMISSIONLESS_CONSTANT_PRODUCT_POOL_IX_ACCOUNTS_LEN] {
    fn from(
        accounts: InitializeCustomizablePermissionlessConstantProductPoolAccounts<
            '_,
            'info,
        >,
    ) -> Self {
        [
            accounts.pool.clone(),
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
            accounts.payer_token_a.clone(),
            accounts.payer_token_b.clone(),
            accounts.payer_pool_lp.clone(),
            accounts.protocol_token_a_fee.clone(),
            accounts.protocol_token_b_fee.clone(),
            accounts.payer.clone(),
            accounts.rent.clone(),
            accounts.mint_metadata.clone(),
            accounts.metadata_program.clone(),
            accounts.vault_program.clone(),
            accounts.token_program.clone(),
            accounts.associated_token_program.clone(),
            accounts.system_program.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<
    &'me [AccountInfo<
        'info,
    >; INITIALIZE_CUSTOMIZABLE_PERMISSIONLESS_CONSTANT_PRODUCT_POOL_IX_ACCOUNTS_LEN],
> for InitializeCustomizablePermissionlessConstantProductPoolAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<
            'info,
        >; INITIALIZE_CUSTOMIZABLE_PERMISSIONLESS_CONSTANT_PRODUCT_POOL_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            pool: &arr[0],
            lp_mint: &arr[1],
            token_a_mint: &arr[2],
            token_b_mint: &arr[3],
            a_vault: &arr[4],
            b_vault: &arr[5],
            a_token_vault: &arr[6],
            b_token_vault: &arr[7],
            a_vault_lp_mint: &arr[8],
            b_vault_lp_mint: &arr[9],
            a_vault_lp: &arr[10],
            b_vault_lp: &arr[11],
            payer_token_a: &arr[12],
            payer_token_b: &arr[13],
            payer_pool_lp: &arr[14],
            protocol_token_a_fee: &arr[15],
            protocol_token_b_fee: &arr[16],
            payer: &arr[17],
            rent: &arr[18],
            mint_metadata: &arr[19],
            metadata_program: &arr[20],
            vault_program: &arr[21],
            token_program: &arr[22],
            associated_token_program: &arr[23],
            system_program: &arr[24],
        }
    }
}
pub const INITIALIZE_CUSTOMIZABLE_PERMISSIONLESS_CONSTANT_PRODUCT_POOL_IX_DISCM: [u8; 8] = [
    145,
    24,
    172,
    194,
    219,
    125,
    3,
    190,
];
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InitializeCustomizablePermissionlessConstantProductPoolIxArgs {
    pub token_a_amount: u64,
    pub token_b_amount: u64,
    pub params: CustomizableParams,
}
#[derive(Clone, Debug, PartialEq)]
pub struct InitializeCustomizablePermissionlessConstantProductPoolIxData(
    pub InitializeCustomizablePermissionlessConstantProductPoolIxArgs,
);
impl From<InitializeCustomizablePermissionlessConstantProductPoolIxArgs>
for InitializeCustomizablePermissionlessConstantProductPoolIxData {
    fn from(
        args: InitializeCustomizablePermissionlessConstantProductPoolIxArgs,
    ) -> Self {
        Self(args)
    }
}
impl InitializeCustomizablePermissionlessConstantProductPoolIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm
            != INITIALIZE_CUSTOMIZABLE_PERMISSIONLESS_CONSTANT_PRODUCT_POOL_IX_DISCM
        {
            return Err(
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!(
                        "discm does not match. Expected: {:?}. Received: {:?}",
                        INITIALIZE_CUSTOMIZABLE_PERMISSIONLESS_CONSTANT_PRODUCT_POOL_IX_DISCM,
                        maybe_discm
                    ),
                ),
            );
        }
        Ok(
            Self(
                InitializeCustomizablePermissionlessConstantProductPoolIxArgs::deserialize(
                    &mut reader,
                )?,
            ),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer
            .write_all(
                &INITIALIZE_CUSTOMIZABLE_PERMISSIONLESS_CONSTANT_PRODUCT_POOL_IX_DISCM,
            )?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn initialize_customizable_permissionless_constant_product_pool_ix_with_program_id(
    program_id: Pubkey,
    keys: InitializeCustomizablePermissionlessConstantProductPoolKeys,
    args: InitializeCustomizablePermissionlessConstantProductPoolIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; INITIALIZE_CUSTOMIZABLE_PERMISSIONLESS_CONSTANT_PRODUCT_POOL_IX_ACCOUNTS_LEN] = keys
        .into();
    let data: InitializeCustomizablePermissionlessConstantProductPoolIxData = args
        .into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn initialize_customizable_permissionless_constant_product_pool_ix(
    keys: InitializeCustomizablePermissionlessConstantProductPoolKeys,
    args: InitializeCustomizablePermissionlessConstantProductPoolIxArgs,
) -> std::io::Result<Instruction> {
    initialize_customizable_permissionless_constant_product_pool_ix_with_program_id(
        crate::ID,
        keys,
        args,
    )
}
pub fn initialize_customizable_permissionless_constant_product_pool_invoke_with_program_id(
    program_id: Pubkey,
    accounts: InitializeCustomizablePermissionlessConstantProductPoolAccounts<'_, '_>,
    args: InitializeCustomizablePermissionlessConstantProductPoolIxArgs,
) -> ProgramResult {
    let keys: InitializeCustomizablePermissionlessConstantProductPoolKeys = accounts
        .into();
    let ix = initialize_customizable_permissionless_constant_product_pool_ix_with_program_id(
        program_id,
        keys,
        args,
    )?;
    invoke_instruction(&ix, accounts)
}
pub fn initialize_customizable_permissionless_constant_product_pool_invoke(
    accounts: InitializeCustomizablePermissionlessConstantProductPoolAccounts<'_, '_>,
    args: InitializeCustomizablePermissionlessConstantProductPoolIxArgs,
) -> ProgramResult {
    initialize_customizable_permissionless_constant_product_pool_invoke_with_program_id(
        crate::ID,
        accounts,
        args,
    )
}
pub fn initialize_customizable_permissionless_constant_product_pool_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: InitializeCustomizablePermissionlessConstantProductPoolAccounts<'_, '_>,
    args: InitializeCustomizablePermissionlessConstantProductPoolIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: InitializeCustomizablePermissionlessConstantProductPoolKeys = accounts
        .into();
    let ix = initialize_customizable_permissionless_constant_product_pool_ix_with_program_id(
        program_id,
        keys,
        args,
    )?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn initialize_customizable_permissionless_constant_product_pool_invoke_signed(
    accounts: InitializeCustomizablePermissionlessConstantProductPoolAccounts<'_, '_>,
    args: InitializeCustomizablePermissionlessConstantProductPoolIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    initialize_customizable_permissionless_constant_product_pool_invoke_signed_with_program_id(
        crate::ID,
        accounts,
        args,
        seeds,
    )
}
pub fn initialize_customizable_permissionless_constant_product_pool_verify_account_keys(
    accounts: InitializeCustomizablePermissionlessConstantProductPoolAccounts<'_, '_>,
    keys: InitializeCustomizablePermissionlessConstantProductPoolKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.pool.key, keys.pool),
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
        (*accounts.payer_token_a.key, keys.payer_token_a),
        (*accounts.payer_token_b.key, keys.payer_token_b),
        (*accounts.payer_pool_lp.key, keys.payer_pool_lp),
        (*accounts.protocol_token_a_fee.key, keys.protocol_token_a_fee),
        (*accounts.protocol_token_b_fee.key, keys.protocol_token_b_fee),
        (*accounts.payer.key, keys.payer),
        (*accounts.rent.key, keys.rent),
        (*accounts.mint_metadata.key, keys.mint_metadata),
        (*accounts.metadata_program.key, keys.metadata_program),
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
pub fn initialize_customizable_permissionless_constant_product_pool_verify_writable_privileges<
    'me,
    'info,
>(
    accounts: InitializeCustomizablePermissionlessConstantProductPoolAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.pool,
        accounts.lp_mint,
        accounts.a_vault,
        accounts.b_vault,
        accounts.a_token_vault,
        accounts.b_token_vault,
        accounts.a_vault_lp_mint,
        accounts.b_vault_lp_mint,
        accounts.a_vault_lp,
        accounts.b_vault_lp,
        accounts.payer_token_a,
        accounts.payer_token_b,
        accounts.payer_pool_lp,
        accounts.protocol_token_a_fee,
        accounts.protocol_token_b_fee,
        accounts.payer,
        accounts.mint_metadata,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn initialize_customizable_permissionless_constant_product_pool_verify_signer_privileges<
    'me,
    'info,
>(
    accounts: InitializeCustomizablePermissionlessConstantProductPoolAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.payer] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn initialize_customizable_permissionless_constant_product_pool_verify_account_privileges<
    'me,
    'info,
>(
    accounts: InitializeCustomizablePermissionlessConstantProductPoolAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    initialize_customizable_permissionless_constant_product_pool_verify_writable_privileges(
        accounts,
    )?;
    initialize_customizable_permissionless_constant_product_pool_verify_signer_privileges(
        accounts,
    )?;
    Ok(())
}
pub const UPDATE_ACTIVATION_POINT_IX_ACCOUNTS_LEN: usize = 2;
#[derive(Copy, Clone, Debug)]
pub struct UpdateActivationPointAccounts<'me, 'info> {
    pub pool: &'me AccountInfo<'info>,
    pub admin: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct UpdateActivationPointKeys {
    pub pool: Pubkey,
    pub admin: Pubkey,
}
impl From<UpdateActivationPointAccounts<'_, '_>> for UpdateActivationPointKeys {
    fn from(accounts: UpdateActivationPointAccounts) -> Self {
        Self {
            pool: *accounts.pool.key,
            admin: *accounts.admin.key,
        }
    }
}
impl From<UpdateActivationPointKeys>
for [AccountMeta; UPDATE_ACTIVATION_POINT_IX_ACCOUNTS_LEN] {
    fn from(keys: UpdateActivationPointKeys) -> Self {
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
        ]
    }
}
impl From<[Pubkey; UPDATE_ACTIVATION_POINT_IX_ACCOUNTS_LEN]>
for UpdateActivationPointKeys {
    fn from(pubkeys: [Pubkey; UPDATE_ACTIVATION_POINT_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            pool: pubkeys[0],
            admin: pubkeys[1],
        }
    }
}
impl<'info> From<UpdateActivationPointAccounts<'_, 'info>>
for [AccountInfo<'info>; UPDATE_ACTIVATION_POINT_IX_ACCOUNTS_LEN] {
    fn from(accounts: UpdateActivationPointAccounts<'_, 'info>) -> Self {
        [accounts.pool.clone(), accounts.admin.clone()]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; UPDATE_ACTIVATION_POINT_IX_ACCOUNTS_LEN]>
for UpdateActivationPointAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; UPDATE_ACTIVATION_POINT_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            pool: &arr[0],
            admin: &arr[1],
        }
    }
}
pub const UPDATE_ACTIVATION_POINT_IX_DISCM: [u8; 8] = [
    150,
    62,
    125,
    219,
    171,
    220,
    26,
    237,
];
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct UpdateActivationPointIxArgs {
    pub new_activation_point: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct UpdateActivationPointIxData(pub UpdateActivationPointIxArgs);
impl From<UpdateActivationPointIxArgs> for UpdateActivationPointIxData {
    fn from(args: UpdateActivationPointIxArgs) -> Self {
        Self(args)
    }
}
impl UpdateActivationPointIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != UPDATE_ACTIVATION_POINT_IX_DISCM {
            return Err(
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!(
                        "discm does not match. Expected: {:?}. Received: {:?}",
                        UPDATE_ACTIVATION_POINT_IX_DISCM, maybe_discm
                    ),
                ),
            );
        }
        Ok(Self(UpdateActivationPointIxArgs::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&UPDATE_ACTIVATION_POINT_IX_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn update_activation_point_ix_with_program_id(
    program_id: Pubkey,
    keys: UpdateActivationPointKeys,
    args: UpdateActivationPointIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; UPDATE_ACTIVATION_POINT_IX_ACCOUNTS_LEN] = keys.into();
    let data: UpdateActivationPointIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn update_activation_point_ix(
    keys: UpdateActivationPointKeys,
    args: UpdateActivationPointIxArgs,
) -> std::io::Result<Instruction> {
    update_activation_point_ix_with_program_id(crate::ID, keys, args)
}
pub fn update_activation_point_invoke_with_program_id(
    program_id: Pubkey,
    accounts: UpdateActivationPointAccounts<'_, '_>,
    args: UpdateActivationPointIxArgs,
) -> ProgramResult {
    let keys: UpdateActivationPointKeys = accounts.into();
    let ix = update_activation_point_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn update_activation_point_invoke(
    accounts: UpdateActivationPointAccounts<'_, '_>,
    args: UpdateActivationPointIxArgs,
) -> ProgramResult {
    update_activation_point_invoke_with_program_id(crate::ID, accounts, args)
}
pub fn update_activation_point_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: UpdateActivationPointAccounts<'_, '_>,
    args: UpdateActivationPointIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: UpdateActivationPointKeys = accounts.into();
    let ix = update_activation_point_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn update_activation_point_invoke_signed(
    accounts: UpdateActivationPointAccounts<'_, '_>,
    args: UpdateActivationPointIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    update_activation_point_invoke_signed_with_program_id(
        crate::ID,
        accounts,
        args,
        seeds,
    )
}
pub fn update_activation_point_verify_account_keys(
    accounts: UpdateActivationPointAccounts<'_, '_>,
    keys: UpdateActivationPointKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.pool.key, keys.pool),
        (*accounts.admin.key, keys.admin),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn update_activation_point_verify_writable_privileges<'me, 'info>(
    accounts: UpdateActivationPointAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.pool] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn update_activation_point_verify_signer_privileges<'me, 'info>(
    accounts: UpdateActivationPointAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.admin] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn update_activation_point_verify_account_privileges<'me, 'info>(
    accounts: UpdateActivationPointAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    update_activation_point_verify_writable_privileges(accounts)?;
    update_activation_point_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const WITHDRAW_PROTOCOL_FEES_IX_ACCOUNTS_LEN: usize = 7;
#[derive(Copy, Clone, Debug)]
pub struct WithdrawProtocolFeesAccounts<'me, 'info> {
    pub pool: &'me AccountInfo<'info>,
    pub a_vault_lp: &'me AccountInfo<'info>,
    pub protocol_token_a_fee: &'me AccountInfo<'info>,
    pub protocol_token_b_fee: &'me AccountInfo<'info>,
    pub treasury_token_a: &'me AccountInfo<'info>,
    pub treasury_token_b: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct WithdrawProtocolFeesKeys {
    pub pool: Pubkey,
    pub a_vault_lp: Pubkey,
    pub protocol_token_a_fee: Pubkey,
    pub protocol_token_b_fee: Pubkey,
    pub treasury_token_a: Pubkey,
    pub treasury_token_b: Pubkey,
    pub token_program: Pubkey,
}
impl From<WithdrawProtocolFeesAccounts<'_, '_>> for WithdrawProtocolFeesKeys {
    fn from(accounts: WithdrawProtocolFeesAccounts) -> Self {
        Self {
            pool: *accounts.pool.key,
            a_vault_lp: *accounts.a_vault_lp.key,
            protocol_token_a_fee: *accounts.protocol_token_a_fee.key,
            protocol_token_b_fee: *accounts.protocol_token_b_fee.key,
            treasury_token_a: *accounts.treasury_token_a.key,
            treasury_token_b: *accounts.treasury_token_b.key,
            token_program: *accounts.token_program.key,
        }
    }
}
impl From<WithdrawProtocolFeesKeys>
for [AccountMeta; WITHDRAW_PROTOCOL_FEES_IX_ACCOUNTS_LEN] {
    fn from(keys: WithdrawProtocolFeesKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.pool,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.a_vault_lp,
                is_signer: false,
                is_writable: false,
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
                pubkey: keys.treasury_token_a,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.treasury_token_b,
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
impl From<[Pubkey; WITHDRAW_PROTOCOL_FEES_IX_ACCOUNTS_LEN]>
for WithdrawProtocolFeesKeys {
    fn from(pubkeys: [Pubkey; WITHDRAW_PROTOCOL_FEES_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            pool: pubkeys[0],
            a_vault_lp: pubkeys[1],
            protocol_token_a_fee: pubkeys[2],
            protocol_token_b_fee: pubkeys[3],
            treasury_token_a: pubkeys[4],
            treasury_token_b: pubkeys[5],
            token_program: pubkeys[6],
        }
    }
}
impl<'info> From<WithdrawProtocolFeesAccounts<'_, 'info>>
for [AccountInfo<'info>; WITHDRAW_PROTOCOL_FEES_IX_ACCOUNTS_LEN] {
    fn from(accounts: WithdrawProtocolFeesAccounts<'_, 'info>) -> Self {
        [
            accounts.pool.clone(),
            accounts.a_vault_lp.clone(),
            accounts.protocol_token_a_fee.clone(),
            accounts.protocol_token_b_fee.clone(),
            accounts.treasury_token_a.clone(),
            accounts.treasury_token_b.clone(),
            accounts.token_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; WITHDRAW_PROTOCOL_FEES_IX_ACCOUNTS_LEN]>
for WithdrawProtocolFeesAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; WITHDRAW_PROTOCOL_FEES_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            pool: &arr[0],
            a_vault_lp: &arr[1],
            protocol_token_a_fee: &arr[2],
            protocol_token_b_fee: &arr[3],
            treasury_token_a: &arr[4],
            treasury_token_b: &arr[5],
            token_program: &arr[6],
        }
    }
}
pub const WITHDRAW_PROTOCOL_FEES_IX_DISCM: [u8; 8] = [11, 68, 165, 98, 18, 208, 134, 73];
#[derive(Clone, Debug, PartialEq)]
pub struct WithdrawProtocolFeesIxData;
impl WithdrawProtocolFeesIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != WITHDRAW_PROTOCOL_FEES_IX_DISCM {
            return Err(
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!(
                        "discm does not match. Expected: {:?}. Received: {:?}",
                        WITHDRAW_PROTOCOL_FEES_IX_DISCM, maybe_discm
                    ),
                ),
            );
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&WITHDRAW_PROTOCOL_FEES_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn withdraw_protocol_fees_ix_with_program_id(
    program_id: Pubkey,
    keys: WithdrawProtocolFeesKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; WITHDRAW_PROTOCOL_FEES_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: WithdrawProtocolFeesIxData.try_to_vec()?,
    })
}
pub fn withdraw_protocol_fees_ix(
    keys: WithdrawProtocolFeesKeys,
) -> std::io::Result<Instruction> {
    withdraw_protocol_fees_ix_with_program_id(crate::ID, keys)
}
pub fn withdraw_protocol_fees_invoke_with_program_id(
    program_id: Pubkey,
    accounts: WithdrawProtocolFeesAccounts<'_, '_>,
) -> ProgramResult {
    let keys: WithdrawProtocolFeesKeys = accounts.into();
    let ix = withdraw_protocol_fees_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn withdraw_protocol_fees_invoke(
    accounts: WithdrawProtocolFeesAccounts<'_, '_>,
) -> ProgramResult {
    withdraw_protocol_fees_invoke_with_program_id(crate::ID, accounts)
}
pub fn withdraw_protocol_fees_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: WithdrawProtocolFeesAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: WithdrawProtocolFeesKeys = accounts.into();
    let ix = withdraw_protocol_fees_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn withdraw_protocol_fees_invoke_signed(
    accounts: WithdrawProtocolFeesAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    withdraw_protocol_fees_invoke_signed_with_program_id(crate::ID, accounts, seeds)
}
pub fn withdraw_protocol_fees_verify_account_keys(
    accounts: WithdrawProtocolFeesAccounts<'_, '_>,
    keys: WithdrawProtocolFeesKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.pool.key, keys.pool),
        (*accounts.a_vault_lp.key, keys.a_vault_lp),
        (*accounts.protocol_token_a_fee.key, keys.protocol_token_a_fee),
        (*accounts.protocol_token_b_fee.key, keys.protocol_token_b_fee),
        (*accounts.treasury_token_a.key, keys.treasury_token_a),
        (*accounts.treasury_token_b.key, keys.treasury_token_b),
        (*accounts.token_program.key, keys.token_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn withdraw_protocol_fees_verify_writable_privileges<'me, 'info>(
    accounts: WithdrawProtocolFeesAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.protocol_token_a_fee,
        accounts.protocol_token_b_fee,
        accounts.treasury_token_a,
        accounts.treasury_token_b,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn withdraw_protocol_fees_verify_account_privileges<'me, 'info>(
    accounts: WithdrawProtocolFeesAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    withdraw_protocol_fees_verify_writable_privileges(accounts)?;
    Ok(())
}
pub const SET_WHITELISTED_VAULT_IX_ACCOUNTS_LEN: usize = 2;
#[derive(Copy, Clone, Debug)]
pub struct SetWhitelistedVaultAccounts<'me, 'info> {
    pub pool: &'me AccountInfo<'info>,
    pub admin: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SetWhitelistedVaultKeys {
    pub pool: Pubkey,
    pub admin: Pubkey,
}
impl From<SetWhitelistedVaultAccounts<'_, '_>> for SetWhitelistedVaultKeys {
    fn from(accounts: SetWhitelistedVaultAccounts) -> Self {
        Self {
            pool: *accounts.pool.key,
            admin: *accounts.admin.key,
        }
    }
}
impl From<SetWhitelistedVaultKeys>
for [AccountMeta; SET_WHITELISTED_VAULT_IX_ACCOUNTS_LEN] {
    fn from(keys: SetWhitelistedVaultKeys) -> Self {
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
        ]
    }
}
impl From<[Pubkey; SET_WHITELISTED_VAULT_IX_ACCOUNTS_LEN]> for SetWhitelistedVaultKeys {
    fn from(pubkeys: [Pubkey; SET_WHITELISTED_VAULT_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            pool: pubkeys[0],
            admin: pubkeys[1],
        }
    }
}
impl<'info> From<SetWhitelistedVaultAccounts<'_, 'info>>
for [AccountInfo<'info>; SET_WHITELISTED_VAULT_IX_ACCOUNTS_LEN] {
    fn from(accounts: SetWhitelistedVaultAccounts<'_, 'info>) -> Self {
        [accounts.pool.clone(), accounts.admin.clone()]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; SET_WHITELISTED_VAULT_IX_ACCOUNTS_LEN]>
for SetWhitelistedVaultAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; SET_WHITELISTED_VAULT_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            pool: &arr[0],
            admin: &arr[1],
        }
    }
}
pub const SET_WHITELISTED_VAULT_IX_DISCM: [u8; 8] = [12, 148, 94, 42, 55, 57, 83, 247];
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SetWhitelistedVaultIxArgs {
    pub whitelisted_vault: Pubkey,
}
#[derive(Clone, Debug, PartialEq)]
pub struct SetWhitelistedVaultIxData(pub SetWhitelistedVaultIxArgs);
impl From<SetWhitelistedVaultIxArgs> for SetWhitelistedVaultIxData {
    fn from(args: SetWhitelistedVaultIxArgs) -> Self {
        Self(args)
    }
}
impl SetWhitelistedVaultIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != SET_WHITELISTED_VAULT_IX_DISCM {
            return Err(
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!(
                        "discm does not match. Expected: {:?}. Received: {:?}",
                        SET_WHITELISTED_VAULT_IX_DISCM, maybe_discm
                    ),
                ),
            );
        }
        Ok(Self(SetWhitelistedVaultIxArgs::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&SET_WHITELISTED_VAULT_IX_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn set_whitelisted_vault_ix_with_program_id(
    program_id: Pubkey,
    keys: SetWhitelistedVaultKeys,
    args: SetWhitelistedVaultIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; SET_WHITELISTED_VAULT_IX_ACCOUNTS_LEN] = keys.into();
    let data: SetWhitelistedVaultIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn set_whitelisted_vault_ix(
    keys: SetWhitelistedVaultKeys,
    args: SetWhitelistedVaultIxArgs,
) -> std::io::Result<Instruction> {
    set_whitelisted_vault_ix_with_program_id(crate::ID, keys, args)
}
pub fn set_whitelisted_vault_invoke_with_program_id(
    program_id: Pubkey,
    accounts: SetWhitelistedVaultAccounts<'_, '_>,
    args: SetWhitelistedVaultIxArgs,
) -> ProgramResult {
    let keys: SetWhitelistedVaultKeys = accounts.into();
    let ix = set_whitelisted_vault_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn set_whitelisted_vault_invoke(
    accounts: SetWhitelistedVaultAccounts<'_, '_>,
    args: SetWhitelistedVaultIxArgs,
) -> ProgramResult {
    set_whitelisted_vault_invoke_with_program_id(crate::ID, accounts, args)
}
pub fn set_whitelisted_vault_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: SetWhitelistedVaultAccounts<'_, '_>,
    args: SetWhitelistedVaultIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: SetWhitelistedVaultKeys = accounts.into();
    let ix = set_whitelisted_vault_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn set_whitelisted_vault_invoke_signed(
    accounts: SetWhitelistedVaultAccounts<'_, '_>,
    args: SetWhitelistedVaultIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    set_whitelisted_vault_invoke_signed_with_program_id(crate::ID, accounts, args, seeds)
}
pub fn set_whitelisted_vault_verify_account_keys(
    accounts: SetWhitelistedVaultAccounts<'_, '_>,
    keys: SetWhitelistedVaultKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.pool.key, keys.pool),
        (*accounts.admin.key, keys.admin),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn set_whitelisted_vault_verify_writable_privileges<'me, 'info>(
    accounts: SetWhitelistedVaultAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.pool] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn set_whitelisted_vault_verify_signer_privileges<'me, 'info>(
    accounts: SetWhitelistedVaultAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.admin] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn set_whitelisted_vault_verify_account_privileges<'me, 'info>(
    accounts: SetWhitelistedVaultAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    set_whitelisted_vault_verify_writable_privileges(accounts)?;
    set_whitelisted_vault_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const PARTNER_CLAIM_FEE_IX_ACCOUNTS_LEN: usize = 8;
#[derive(Copy, Clone, Debug)]
pub struct PartnerClaimFeeAccounts<'me, 'info> {
    pub pool: &'me AccountInfo<'info>,
    pub a_vault_lp: &'me AccountInfo<'info>,
    pub protocol_token_a_fee: &'me AccountInfo<'info>,
    pub protocol_token_b_fee: &'me AccountInfo<'info>,
    pub partner_token_a: &'me AccountInfo<'info>,
    pub partner_token_b: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub partner_authority: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct PartnerClaimFeeKeys {
    pub pool: Pubkey,
    pub a_vault_lp: Pubkey,
    pub protocol_token_a_fee: Pubkey,
    pub protocol_token_b_fee: Pubkey,
    pub partner_token_a: Pubkey,
    pub partner_token_b: Pubkey,
    pub token_program: Pubkey,
    pub partner_authority: Pubkey,
}
impl From<PartnerClaimFeeAccounts<'_, '_>> for PartnerClaimFeeKeys {
    fn from(accounts: PartnerClaimFeeAccounts) -> Self {
        Self {
            pool: *accounts.pool.key,
            a_vault_lp: *accounts.a_vault_lp.key,
            protocol_token_a_fee: *accounts.protocol_token_a_fee.key,
            protocol_token_b_fee: *accounts.protocol_token_b_fee.key,
            partner_token_a: *accounts.partner_token_a.key,
            partner_token_b: *accounts.partner_token_b.key,
            token_program: *accounts.token_program.key,
            partner_authority: *accounts.partner_authority.key,
        }
    }
}
impl From<PartnerClaimFeeKeys> for [AccountMeta; PARTNER_CLAIM_FEE_IX_ACCOUNTS_LEN] {
    fn from(keys: PartnerClaimFeeKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.pool,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.a_vault_lp,
                is_signer: false,
                is_writable: false,
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
                pubkey: keys.partner_token_a,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.partner_token_b,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.partner_authority,
                is_signer: true,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; PARTNER_CLAIM_FEE_IX_ACCOUNTS_LEN]> for PartnerClaimFeeKeys {
    fn from(pubkeys: [Pubkey; PARTNER_CLAIM_FEE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            pool: pubkeys[0],
            a_vault_lp: pubkeys[1],
            protocol_token_a_fee: pubkeys[2],
            protocol_token_b_fee: pubkeys[3],
            partner_token_a: pubkeys[4],
            partner_token_b: pubkeys[5],
            token_program: pubkeys[6],
            partner_authority: pubkeys[7],
        }
    }
}
impl<'info> From<PartnerClaimFeeAccounts<'_, 'info>>
for [AccountInfo<'info>; PARTNER_CLAIM_FEE_IX_ACCOUNTS_LEN] {
    fn from(accounts: PartnerClaimFeeAccounts<'_, 'info>) -> Self {
        [
            accounts.pool.clone(),
            accounts.a_vault_lp.clone(),
            accounts.protocol_token_a_fee.clone(),
            accounts.protocol_token_b_fee.clone(),
            accounts.partner_token_a.clone(),
            accounts.partner_token_b.clone(),
            accounts.token_program.clone(),
            accounts.partner_authority.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; PARTNER_CLAIM_FEE_IX_ACCOUNTS_LEN]>
for PartnerClaimFeeAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; PARTNER_CLAIM_FEE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            pool: &arr[0],
            a_vault_lp: &arr[1],
            protocol_token_a_fee: &arr[2],
            protocol_token_b_fee: &arr[3],
            partner_token_a: &arr[4],
            partner_token_b: &arr[5],
            token_program: &arr[6],
            partner_authority: &arr[7],
        }
    }
}
pub const PARTNER_CLAIM_FEE_IX_DISCM: [u8; 8] = [57, 53, 176, 30, 123, 70, 52, 64];
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct PartnerClaimFeeIxArgs {
    pub max_amount_a: u64,
    pub max_amount_b: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct PartnerClaimFeeIxData(pub PartnerClaimFeeIxArgs);
impl From<PartnerClaimFeeIxArgs> for PartnerClaimFeeIxData {
    fn from(args: PartnerClaimFeeIxArgs) -> Self {
        Self(args)
    }
}
impl PartnerClaimFeeIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != PARTNER_CLAIM_FEE_IX_DISCM {
            return Err(
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!(
                        "discm does not match. Expected: {:?}. Received: {:?}",
                        PARTNER_CLAIM_FEE_IX_DISCM, maybe_discm
                    ),
                ),
            );
        }
        Ok(Self(PartnerClaimFeeIxArgs::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&PARTNER_CLAIM_FEE_IX_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn partner_claim_fee_ix_with_program_id(
    program_id: Pubkey,
    keys: PartnerClaimFeeKeys,
    args: PartnerClaimFeeIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; PARTNER_CLAIM_FEE_IX_ACCOUNTS_LEN] = keys.into();
    let data: PartnerClaimFeeIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn partner_claim_fee_ix(
    keys: PartnerClaimFeeKeys,
    args: PartnerClaimFeeIxArgs,
) -> std::io::Result<Instruction> {
    partner_claim_fee_ix_with_program_id(crate::ID, keys, args)
}
pub fn partner_claim_fee_invoke_with_program_id(
    program_id: Pubkey,
    accounts: PartnerClaimFeeAccounts<'_, '_>,
    args: PartnerClaimFeeIxArgs,
) -> ProgramResult {
    let keys: PartnerClaimFeeKeys = accounts.into();
    let ix = partner_claim_fee_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn partner_claim_fee_invoke(
    accounts: PartnerClaimFeeAccounts<'_, '_>,
    args: PartnerClaimFeeIxArgs,
) -> ProgramResult {
    partner_claim_fee_invoke_with_program_id(crate::ID, accounts, args)
}
pub fn partner_claim_fee_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: PartnerClaimFeeAccounts<'_, '_>,
    args: PartnerClaimFeeIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: PartnerClaimFeeKeys = accounts.into();
    let ix = partner_claim_fee_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn partner_claim_fee_invoke_signed(
    accounts: PartnerClaimFeeAccounts<'_, '_>,
    args: PartnerClaimFeeIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    partner_claim_fee_invoke_signed_with_program_id(crate::ID, accounts, args, seeds)
}
pub fn partner_claim_fee_verify_account_keys(
    accounts: PartnerClaimFeeAccounts<'_, '_>,
    keys: PartnerClaimFeeKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.pool.key, keys.pool),
        (*accounts.a_vault_lp.key, keys.a_vault_lp),
        (*accounts.protocol_token_a_fee.key, keys.protocol_token_a_fee),
        (*accounts.protocol_token_b_fee.key, keys.protocol_token_b_fee),
        (*accounts.partner_token_a.key, keys.partner_token_a),
        (*accounts.partner_token_b.key, keys.partner_token_b),
        (*accounts.token_program.key, keys.token_program),
        (*accounts.partner_authority.key, keys.partner_authority),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn partner_claim_fee_verify_writable_privileges<'me, 'info>(
    accounts: PartnerClaimFeeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.pool,
        accounts.protocol_token_a_fee,
        accounts.protocol_token_b_fee,
        accounts.partner_token_a,
        accounts.partner_token_b,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn partner_claim_fee_verify_signer_privileges<'me, 'info>(
    accounts: PartnerClaimFeeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.partner_authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn partner_claim_fee_verify_account_privileges<'me, 'info>(
    accounts: PartnerClaimFeeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    partner_claim_fee_verify_writable_privileges(accounts)?;
    partner_claim_fee_verify_signer_privileges(accounts)?;
    Ok(())
}
