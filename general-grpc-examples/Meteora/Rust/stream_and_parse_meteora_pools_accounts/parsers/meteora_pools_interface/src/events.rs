use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::pubkey::Pubkey;
use crate::*;
pub const ADD_LIQUIDITY_EVENT_DISCM: [u8; 8] = [31, 94, 125, 90, 227, 52, 61, 186];
#[derive(Clone, Debug, PartialEq, BorshDeserialize, BorshSerialize)]
pub struct AddLiquidity {
    lp_mint_amount: u64,
    token_a_amount: u64,
    token_b_amount: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct AddLiquidityEvent(pub AddLiquidity);
impl BorshSerialize for AddLiquidityEvent {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        ADD_LIQUIDITY_EVENT_DISCM.serialize(writer)?;
        self.0.serialize(writer)
    }
}
impl AddLiquidityEvent {
    pub fn deserialize(buf: &mut &[u8]) -> std::io::Result<Self> {
        let maybe_discm = <[u8; 8]>::deserialize(buf)?;
        if maybe_discm != ADD_LIQUIDITY_EVENT_DISCM {
            return Err(
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!(
                        "discm does not match. Expected: {:?}. Received: {:?}",
                        ADD_LIQUIDITY_EVENT_DISCM, maybe_discm
                    ),
                ),
            );
        }
        Ok(Self(AddLiquidity::deserialize(buf)?))
    }
}
pub const REMOVE_LIQUIDITY_EVENT_DISCM: [u8; 8] = [116, 244, 97, 232, 103, 31, 152, 58];
#[derive(Clone, Debug, PartialEq, BorshDeserialize, BorshSerialize)]
pub struct RemoveLiquidity {
    lp_unmint_amount: u64,
    token_a_out_amount: u64,
    token_b_out_amount: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct RemoveLiquidityEvent(pub RemoveLiquidity);
impl BorshSerialize for RemoveLiquidityEvent {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        REMOVE_LIQUIDITY_EVENT_DISCM.serialize(writer)?;
        self.0.serialize(writer)
    }
}
impl RemoveLiquidityEvent {
    pub fn deserialize(buf: &mut &[u8]) -> std::io::Result<Self> {
        let maybe_discm = <[u8; 8]>::deserialize(buf)?;
        if maybe_discm != REMOVE_LIQUIDITY_EVENT_DISCM {
            return Err(
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!(
                        "discm does not match. Expected: {:?}. Received: {:?}",
                        REMOVE_LIQUIDITY_EVENT_DISCM, maybe_discm
                    ),
                ),
            );
        }
        Ok(Self(RemoveLiquidity::deserialize(buf)?))
    }
}
pub const BOOTSTRAP_LIQUIDITY_EVENT_DISCM: [u8; 8] = [
    121,
    127,
    38,
    136,
    92,
    55,
    14,
    247,
];
#[derive(Clone, Debug, PartialEq, BorshDeserialize, BorshSerialize)]
pub struct BootstrapLiquidity {
    lp_mint_amount: u64,
    token_a_amount: u64,
    token_b_amount: u64,
    pool: Pubkey,
}
#[derive(Clone, Debug, PartialEq)]
pub struct BootstrapLiquidityEvent(pub BootstrapLiquidity);
impl BorshSerialize for BootstrapLiquidityEvent {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        BOOTSTRAP_LIQUIDITY_EVENT_DISCM.serialize(writer)?;
        self.0.serialize(writer)
    }
}
impl BootstrapLiquidityEvent {
    pub fn deserialize(buf: &mut &[u8]) -> std::io::Result<Self> {
        let maybe_discm = <[u8; 8]>::deserialize(buf)?;
        if maybe_discm != BOOTSTRAP_LIQUIDITY_EVENT_DISCM {
            return Err(
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!(
                        "discm does not match. Expected: {:?}. Received: {:?}",
                        BOOTSTRAP_LIQUIDITY_EVENT_DISCM, maybe_discm
                    ),
                ),
            );
        }
        Ok(Self(BootstrapLiquidity::deserialize(buf)?))
    }
}
pub const SWAP_EVENT_DISCM: [u8; 8] = [81, 108, 227, 190, 205, 208, 10, 196];
#[derive(Clone, Debug, PartialEq, BorshDeserialize, BorshSerialize)]
pub struct Swap {
    in_amount: u64,
    out_amount: u64,
    trade_fee: u64,
    protocol_fee: u64,
    host_fee: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct SwapEvent(pub Swap);
impl BorshSerialize for SwapEvent {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        SWAP_EVENT_DISCM.serialize(writer)?;
        self.0.serialize(writer)
    }
}
impl SwapEvent {
    pub fn deserialize(buf: &mut &[u8]) -> std::io::Result<Self> {
        let maybe_discm = <[u8; 8]>::deserialize(buf)?;
        if maybe_discm != SWAP_EVENT_DISCM {
            return Err(
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!(
                        "discm does not match. Expected: {:?}. Received: {:?}",
                        SWAP_EVENT_DISCM, maybe_discm
                    ),
                ),
            );
        }
        Ok(Self(Swap::deserialize(buf)?))
    }
}
pub const SET_POOL_FEES_EVENT_DISCM: [u8; 8] = [245, 26, 198, 164, 88, 18, 75, 9];
#[derive(Clone, Debug, PartialEq, BorshDeserialize, BorshSerialize)]
pub struct SetPoolFees {
    trade_fee_numerator: u64,
    trade_fee_denominator: u64,
    protocol_trade_fee_numerator: u64,
    protocol_trade_fee_denominator: u64,
    pool: Pubkey,
}
#[derive(Clone, Debug, PartialEq)]
pub struct SetPoolFeesEvent(pub SetPoolFees);
impl BorshSerialize for SetPoolFeesEvent {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        SET_POOL_FEES_EVENT_DISCM.serialize(writer)?;
        self.0.serialize(writer)
    }
}
impl SetPoolFeesEvent {
    pub fn deserialize(buf: &mut &[u8]) -> std::io::Result<Self> {
        let maybe_discm = <[u8; 8]>::deserialize(buf)?;
        if maybe_discm != SET_POOL_FEES_EVENT_DISCM {
            return Err(
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!(
                        "discm does not match. Expected: {:?}. Received: {:?}",
                        SET_POOL_FEES_EVENT_DISCM, maybe_discm
                    ),
                ),
            );
        }
        Ok(Self(SetPoolFees::deserialize(buf)?))
    }
}
pub const POOL_INFO_EVENT_DISCM: [u8; 8] = [207, 20, 87, 97, 251, 212, 234, 45];
#[derive(Clone, Debug, PartialEq, BorshDeserialize, BorshSerialize)]
pub struct PoolInfo {
    token_a_amount: u64,
    token_b_amount: u64,
    virtual_price: f64,
    current_timestamp: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct PoolInfoEvent(pub PoolInfo);
impl BorshSerialize for PoolInfoEvent {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        POOL_INFO_EVENT_DISCM.serialize(writer)?;
        self.0.serialize(writer)
    }
}
impl PoolInfoEvent {
    pub fn deserialize(buf: &mut &[u8]) -> std::io::Result<Self> {
        let maybe_discm = <[u8; 8]>::deserialize(buf)?;
        if maybe_discm != POOL_INFO_EVENT_DISCM {
            return Err(
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!(
                        "discm does not match. Expected: {:?}. Received: {:?}",
                        POOL_INFO_EVENT_DISCM, maybe_discm
                    ),
                ),
            );
        }
        Ok(Self(PoolInfo::deserialize(buf)?))
    }
}
pub const TRANSFER_ADMIN_EVENT_DISCM: [u8; 8] = [228, 169, 131, 244, 61, 56, 65, 254];
#[derive(Clone, Debug, PartialEq, BorshDeserialize, BorshSerialize)]
pub struct TransferAdmin {
    admin: Pubkey,
    new_admin: Pubkey,
    pool: Pubkey,
}
#[derive(Clone, Debug, PartialEq)]
pub struct TransferAdminEvent(pub TransferAdmin);
impl BorshSerialize for TransferAdminEvent {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        TRANSFER_ADMIN_EVENT_DISCM.serialize(writer)?;
        self.0.serialize(writer)
    }
}
impl TransferAdminEvent {
    pub fn deserialize(buf: &mut &[u8]) -> std::io::Result<Self> {
        let maybe_discm = <[u8; 8]>::deserialize(buf)?;
        if maybe_discm != TRANSFER_ADMIN_EVENT_DISCM {
            return Err(
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!(
                        "discm does not match. Expected: {:?}. Received: {:?}",
                        TRANSFER_ADMIN_EVENT_DISCM, maybe_discm
                    ),
                ),
            );
        }
        Ok(Self(TransferAdmin::deserialize(buf)?))
    }
}
pub const OVERRIDE_CURVE_PARAM_EVENT_DISCM: [u8; 8] = [
    247,
    20,
    165,
    248,
    75,
    5,
    54,
    246,
];
#[derive(Clone, Debug, PartialEq, BorshDeserialize, BorshSerialize)]
pub struct OverrideCurveParam {
    new_amp: u64,
    updated_timestamp: u64,
    pool: Pubkey,
}
#[derive(Clone, Debug, PartialEq)]
pub struct OverrideCurveParamEvent(pub OverrideCurveParam);
impl BorshSerialize for OverrideCurveParamEvent {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        OVERRIDE_CURVE_PARAM_EVENT_DISCM.serialize(writer)?;
        self.0.serialize(writer)
    }
}
impl OverrideCurveParamEvent {
    pub fn deserialize(buf: &mut &[u8]) -> std::io::Result<Self> {
        let maybe_discm = <[u8; 8]>::deserialize(buf)?;
        if maybe_discm != OVERRIDE_CURVE_PARAM_EVENT_DISCM {
            return Err(
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!(
                        "discm does not match. Expected: {:?}. Received: {:?}",
                        OVERRIDE_CURVE_PARAM_EVENT_DISCM, maybe_discm
                    ),
                ),
            );
        }
        Ok(Self(OverrideCurveParam::deserialize(buf)?))
    }
}
pub const POOL_CREATED_EVENT_DISCM: [u8; 8] = [202, 44, 41, 88, 104, 220, 157, 82];
#[derive(Clone, Debug, PartialEq, BorshDeserialize, BorshSerialize)]
pub struct PoolCreated {
    lp_mint: Pubkey,
    token_a_mint: Pubkey,
    token_b_mint: Pubkey,
    pool_type: PoolType,
    pool: Pubkey,
}
#[derive(Clone, Debug, PartialEq)]
pub struct PoolCreatedEvent(pub PoolCreated);
impl BorshSerialize for PoolCreatedEvent {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        POOL_CREATED_EVENT_DISCM.serialize(writer)?;
        self.0.serialize(writer)
    }
}
impl PoolCreatedEvent {
    pub fn deserialize(buf: &mut &[u8]) -> std::io::Result<Self> {
        let maybe_discm = <[u8; 8]>::deserialize(buf)?;
        if maybe_discm != POOL_CREATED_EVENT_DISCM {
            return Err(
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!(
                        "discm does not match. Expected: {:?}. Received: {:?}",
                        POOL_CREATED_EVENT_DISCM, maybe_discm
                    ),
                ),
            );
        }
        Ok(Self(PoolCreated::deserialize(buf)?))
    }
}
pub const POOL_ENABLED_EVENT_DISCM: [u8; 8] = [2, 151, 18, 83, 204, 134, 92, 191];
#[derive(Clone, Debug, PartialEq, BorshDeserialize, BorshSerialize)]
pub struct PoolEnabled {
    pool: Pubkey,
    enabled: bool,
}
#[derive(Clone, Debug, PartialEq)]
pub struct PoolEnabledEvent(pub PoolEnabled);
impl BorshSerialize for PoolEnabledEvent {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        POOL_ENABLED_EVENT_DISCM.serialize(writer)?;
        self.0.serialize(writer)
    }
}
impl PoolEnabledEvent {
    pub fn deserialize(buf: &mut &[u8]) -> std::io::Result<Self> {
        let maybe_discm = <[u8; 8]>::deserialize(buf)?;
        if maybe_discm != POOL_ENABLED_EVENT_DISCM {
            return Err(
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!(
                        "discm does not match. Expected: {:?}. Received: {:?}",
                        POOL_ENABLED_EVENT_DISCM, maybe_discm
                    ),
                ),
            );
        }
        Ok(Self(PoolEnabled::deserialize(buf)?))
    }
}
pub const MIGRATE_FEE_ACCOUNT_EVENT_DISCM: [u8; 8] = [
    223,
    234,
    232,
    26,
    252,
    105,
    180,
    125,
];
#[derive(Clone, Debug, PartialEq, BorshDeserialize, BorshSerialize)]
pub struct MigrateFeeAccount {
    pool: Pubkey,
    new_admin_token_a_fee: Pubkey,
    new_admin_token_b_fee: Pubkey,
    token_a_amount: u64,
    token_b_amount: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct MigrateFeeAccountEvent(pub MigrateFeeAccount);
impl BorshSerialize for MigrateFeeAccountEvent {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        MIGRATE_FEE_ACCOUNT_EVENT_DISCM.serialize(writer)?;
        self.0.serialize(writer)
    }
}
impl MigrateFeeAccountEvent {
    pub fn deserialize(buf: &mut &[u8]) -> std::io::Result<Self> {
        let maybe_discm = <[u8; 8]>::deserialize(buf)?;
        if maybe_discm != MIGRATE_FEE_ACCOUNT_EVENT_DISCM {
            return Err(
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!(
                        "discm does not match. Expected: {:?}. Received: {:?}",
                        MIGRATE_FEE_ACCOUNT_EVENT_DISCM, maybe_discm
                    ),
                ),
            );
        }
        Ok(Self(MigrateFeeAccount::deserialize(buf)?))
    }
}
pub const CREATE_LOCK_ESCROW_EVENT_DISCM: [u8; 8] = [74, 94, 106, 141, 49, 17, 98, 109];
#[derive(Clone, Debug, PartialEq, BorshDeserialize, BorshSerialize)]
pub struct CreateLockEscrow {
    pool: Pubkey,
    owner: Pubkey,
}
#[derive(Clone, Debug, PartialEq)]
pub struct CreateLockEscrowEvent(pub CreateLockEscrow);
impl BorshSerialize for CreateLockEscrowEvent {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        CREATE_LOCK_ESCROW_EVENT_DISCM.serialize(writer)?;
        self.0.serialize(writer)
    }
}
impl CreateLockEscrowEvent {
    pub fn deserialize(buf: &mut &[u8]) -> std::io::Result<Self> {
        let maybe_discm = <[u8; 8]>::deserialize(buf)?;
        if maybe_discm != CREATE_LOCK_ESCROW_EVENT_DISCM {
            return Err(
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!(
                        "discm does not match. Expected: {:?}. Received: {:?}",
                        CREATE_LOCK_ESCROW_EVENT_DISCM, maybe_discm
                    ),
                ),
            );
        }
        Ok(Self(CreateLockEscrow::deserialize(buf)?))
    }
}
pub const LOCK_EVENT_DISCM: [u8; 8] = [220, 183, 67, 215, 153, 207, 56, 234];
#[derive(Clone, Debug, PartialEq, BorshDeserialize, BorshSerialize)]
pub struct Lock {
    pool: Pubkey,
    owner: Pubkey,
    amount: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct LockEvent(pub Lock);
impl BorshSerialize for LockEvent {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        LOCK_EVENT_DISCM.serialize(writer)?;
        self.0.serialize(writer)
    }
}
impl LockEvent {
    pub fn deserialize(buf: &mut &[u8]) -> std::io::Result<Self> {
        let maybe_discm = <[u8; 8]>::deserialize(buf)?;
        if maybe_discm != LOCK_EVENT_DISCM {
            return Err(
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!(
                        "discm does not match. Expected: {:?}. Received: {:?}",
                        LOCK_EVENT_DISCM, maybe_discm
                    ),
                ),
            );
        }
        Ok(Self(Lock::deserialize(buf)?))
    }
}
pub const CLAIM_FEE_EVENT_DISCM: [u8; 8] = [75, 122, 154, 48, 140, 74, 123, 163];
#[derive(Clone, Debug, PartialEq, BorshDeserialize, BorshSerialize)]
pub struct ClaimFee {
    pool: Pubkey,
    owner: Pubkey,
    amount: u64,
    a_fee: u64,
    b_fee: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct ClaimFeeEvent(pub ClaimFee);
impl BorshSerialize for ClaimFeeEvent {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        CLAIM_FEE_EVENT_DISCM.serialize(writer)?;
        self.0.serialize(writer)
    }
}
impl ClaimFeeEvent {
    pub fn deserialize(buf: &mut &[u8]) -> std::io::Result<Self> {
        let maybe_discm = <[u8; 8]>::deserialize(buf)?;
        if maybe_discm != CLAIM_FEE_EVENT_DISCM {
            return Err(
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!(
                        "discm does not match. Expected: {:?}. Received: {:?}",
                        CLAIM_FEE_EVENT_DISCM, maybe_discm
                    ),
                ),
            );
        }
        Ok(Self(ClaimFee::deserialize(buf)?))
    }
}
pub const CREATE_CONFIG_EVENT_DISCM: [u8; 8] = [199, 152, 10, 19, 39, 39, 157, 104];
#[derive(Clone, Debug, PartialEq, BorshDeserialize, BorshSerialize)]
pub struct CreateConfig {
    trade_fee_numerator: u64,
    protocol_trade_fee_numerator: u64,
    config: Pubkey,
}
#[derive(Clone, Debug, PartialEq)]
pub struct CreateConfigEvent(pub CreateConfig);
impl BorshSerialize for CreateConfigEvent {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        CREATE_CONFIG_EVENT_DISCM.serialize(writer)?;
        self.0.serialize(writer)
    }
}
impl CreateConfigEvent {
    pub fn deserialize(buf: &mut &[u8]) -> std::io::Result<Self> {
        let maybe_discm = <[u8; 8]>::deserialize(buf)?;
        if maybe_discm != CREATE_CONFIG_EVENT_DISCM {
            return Err(
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!(
                        "discm does not match. Expected: {:?}. Received: {:?}",
                        CREATE_CONFIG_EVENT_DISCM, maybe_discm
                    ),
                ),
            );
        }
        Ok(Self(CreateConfig::deserialize(buf)?))
    }
}
pub const CLOSE_CONFIG_EVENT_DISCM: [u8; 8] = [249, 181, 108, 89, 4, 150, 90, 174];
#[derive(Clone, Debug, PartialEq, BorshDeserialize, BorshSerialize)]
pub struct CloseConfig {
    config: Pubkey,
}
#[derive(Clone, Debug, PartialEq)]
pub struct CloseConfigEvent(pub CloseConfig);
impl BorshSerialize for CloseConfigEvent {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        CLOSE_CONFIG_EVENT_DISCM.serialize(writer)?;
        self.0.serialize(writer)
    }
}
impl CloseConfigEvent {
    pub fn deserialize(buf: &mut &[u8]) -> std::io::Result<Self> {
        let maybe_discm = <[u8; 8]>::deserialize(buf)?;
        if maybe_discm != CLOSE_CONFIG_EVENT_DISCM {
            return Err(
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!(
                        "discm does not match. Expected: {:?}. Received: {:?}",
                        CLOSE_CONFIG_EVENT_DISCM, maybe_discm
                    ),
                ),
            );
        }
        Ok(Self(CloseConfig::deserialize(buf)?))
    }
}
pub const WITHDRAW_PROTOCOL_FEES_EVENT_DISCM: [u8; 8] = [
    30,
    240,
    207,
    196,
    139,
    239,
    79,
    28,
];
#[derive(Clone, Debug, PartialEq, BorshDeserialize, BorshSerialize)]
pub struct WithdrawProtocolFees {
    pool: Pubkey,
    protocol_a_fee: u64,
    protocol_b_fee: u64,
    protocol_a_fee_owner: Pubkey,
    protocol_b_fee_owner: Pubkey,
}
#[derive(Clone, Debug, PartialEq)]
pub struct WithdrawProtocolFeesEvent(pub WithdrawProtocolFees);
impl BorshSerialize for WithdrawProtocolFeesEvent {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        WITHDRAW_PROTOCOL_FEES_EVENT_DISCM.serialize(writer)?;
        self.0.serialize(writer)
    }
}
impl WithdrawProtocolFeesEvent {
    pub fn deserialize(buf: &mut &[u8]) -> std::io::Result<Self> {
        let maybe_discm = <[u8; 8]>::deserialize(buf)?;
        if maybe_discm != WITHDRAW_PROTOCOL_FEES_EVENT_DISCM {
            return Err(
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!(
                        "discm does not match. Expected: {:?}. Received: {:?}",
                        WITHDRAW_PROTOCOL_FEES_EVENT_DISCM, maybe_discm
                    ),
                ),
            );
        }
        Ok(Self(WithdrawProtocolFees::deserialize(buf)?))
    }
}
pub const PARTNER_CLAIM_FEES_EVENT_DISCM: [u8; 8] = [
    135,
    131,
    10,
    94,
    119,
    209,
    202,
    48,
];
#[derive(Clone, Debug, PartialEq, BorshDeserialize, BorshSerialize)]
pub struct PartnerClaimFees {
    pool: Pubkey,
    fee_a: u64,
    fee_b: u64,
    partner: Pubkey,
}
#[derive(Clone, Debug, PartialEq)]
pub struct PartnerClaimFeesEvent(pub PartnerClaimFees);
impl BorshSerialize for PartnerClaimFeesEvent {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        PARTNER_CLAIM_FEES_EVENT_DISCM.serialize(writer)?;
        self.0.serialize(writer)
    }
}
impl PartnerClaimFeesEvent {
    pub fn deserialize(buf: &mut &[u8]) -> std::io::Result<Self> {
        let maybe_discm = <[u8; 8]>::deserialize(buf)?;
        if maybe_discm != PARTNER_CLAIM_FEES_EVENT_DISCM {
            return Err(
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!(
                        "discm does not match. Expected: {:?}. Received: {:?}",
                        PARTNER_CLAIM_FEES_EVENT_DISCM, maybe_discm
                    ),
                ),
            );
        }
        Ok(Self(PartnerClaimFees::deserialize(buf)?))
    }
}
