use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    decode_error::DecodeError, msg, program_error::{PrintProgramError, ProgramError}, pubkey::Pubkey
};
use thiserror::Error;

pub const EVT_ADD_LIQUIDITY_DISCM: [u8; 8] = [175, 242, 8, 157, 30, 247, 185, 169];
#[derive(Clone, Debug, BorshDeserialize, BorshSerialize, PartialEq,serde::Serialize, serde::Deserialize)]
pub struct EvtAddLiquidity {
    lb_pair: Pubkey,
    from: Pubkey,
    position: Pubkey,
    amounts: [u64; 2],
    active_bin_id: i32,
}
#[derive(Clone, Debug, PartialEq)]
pub struct EvtAddLiquidityEvent(pub EvtAddLiquidity);
impl BorshSerialize for EvtAddLiquidityEvent {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        EVT_ADD_LIQUIDITY_DISCM.serialize(writer)?;
        self.0.serialize(writer)
    }
}
impl EvtAddLiquidityEvent {
    pub fn deserialize(buf: &mut &[u8]) -> std::io::Result<Self> {
        let maybe_discm = <[u8; 8]>::deserialize(buf)?;
        if maybe_discm != EVT_ADD_LIQUIDITY_DISCM {
            return Err(
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!(
                        "discm does not match. Expected: {:?}. Received: {:?}",
                        EVT_ADD_LIQUIDITY_DISCM, maybe_discm
                    ),
                ),
            );
        }
        Ok(Self(EvtAddLiquidity::deserialize(buf)?))
    }
}

pub const EVT_CLAIM_PARTNER_FEE_DISCM: [u8; 8] = [118, 99, 77, 10, 226, 1, 1, 87];
#[derive(Clone, Debug, BorshDeserialize, BorshSerialize, PartialEq,serde::Serialize, serde::Deserialize)]
pub struct EvtClaimPartnerFee {
    // Placeholder fields; actual fields depend on program requirements
    lb_pair: Pubkey,
    partner: Pubkey,
    fee_amount: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct EvtClaimPartnerFeeEvent(pub EvtClaimPartnerFee);
impl BorshSerialize for EvtClaimPartnerFeeEvent {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        EVT_CLAIM_PARTNER_FEE_DISCM.serialize(writer)?;
        self.0.serialize(writer)
    }
}
impl EvtClaimPartnerFeeEvent {
    pub fn deserialize(buf: &mut &[u8]) -> std::io::Result<Self> {
        let maybe_discm = <[u8; 8]>::deserialize(buf)?;
        if maybe_discm != EVT_CLAIM_PARTNER_FEE_DISCM {
            return Err(
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!(
                        "discm does not match. Expected: {:?}. Received: {:?}",
                        EVT_CLAIM_PARTNER_FEE_DISCM, maybe_discm
                    ),
                ),
            );
        }
        Ok(Self(EvtClaimPartnerFee::deserialize(buf)?))
    }
}

pub const EVT_CLAIM_POSITION_FEE_DISCM: [u8; 8] = [198, 182, 183, 52, 97, 12, 49, 56];
#[derive(Clone, Debug, BorshDeserialize, BorshSerialize, PartialEq,serde::Serialize, serde::Deserialize)]
pub struct EvtClaimPositionFee {
    lb_pair: Pubkey,
    position: Pubkey,
    owner: Pubkey,
    fee_x: u64,
    fee_y: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct EvtClaimPositionFeeEvent(pub EvtClaimPositionFee);
impl BorshSerialize for EvtClaimPositionFeeEvent {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        EVT_CLAIM_POSITION_FEE_DISCM.serialize(writer)?;
        self.0.serialize(writer)
    }
}
impl EvtClaimPositionFeeEvent {
    pub fn deserialize(buf: &mut &[u8]) -> std::io::Result<Self> {
        let maybe_discm = <[u8; 8]>::deserialize(buf)?;
        if maybe_discm != EVT_CLAIM_POSITION_FEE_DISCM {
            return Err(
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!(
                        "discm does not match. Expected: {:?}. Received: {:?}",
                        EVT_CLAIM_POSITION_FEE_DISCM, maybe_discm
                    ),
                ),
            );
        }
        Ok(Self(EvtClaimPositionFee::deserialize(buf)?))
    }
}

pub const EVT_CLAIM_PROTOCOL_FEE_DISCM: [u8; 8] = [186, 244, 75, 251, 188, 13, 25, 33];
#[derive(Clone, Debug, BorshDeserialize, BorshSerialize, PartialEq,serde::Serialize, serde::Deserialize)]
pub struct EvtClaimProtocolFee {
    // Placeholder fields; actual fields depend on program requirements
    lb_pair: Pubkey,
    fee_amount: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct EvtClaimProtocolFeeEvent(pub EvtClaimProtocolFee);
impl BorshSerialize for EvtClaimProtocolFeeEvent {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        EVT_CLAIM_PROTOCOL_FEE_DISCM.serialize(writer)?;
        self.0.serialize(writer)
    }
}
impl EvtClaimProtocolFeeEvent {
    pub fn deserialize(buf: &mut &[u8]) -> std::io::Result<Self> {
        let maybe_discm = <[u8; 8]>::deserialize(buf)?;
        if maybe_discm != EVT_CLAIM_PROTOCOL_FEE_DISCM {
            return Err(
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!(
                        "discm does not match. Expected: {:?}. Received: {:?}",
                        EVT_CLAIM_PROTOCOL_FEE_DISCM, maybe_discm
                    ),
                ),
            );
        }
        Ok(Self(EvtClaimProtocolFee::deserialize(buf)?))
    }
}

pub const EVT_CLAIM_REWARD_DISCM: [u8; 8] = [218, 86, 147, 200, 235, 188, 215, 231];
#[derive(Clone, Debug, BorshDeserialize, BorshSerialize, PartialEq,serde::Serialize, serde::Deserialize)]
pub struct EvtClaimReward {
    lb_pair: Pubkey,
    position: Pubkey,
    owner: Pubkey,
    reward_index: u64,
    total_reward: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct EvtClaimRewardEvent(pub EvtClaimReward);
impl BorshSerialize for EvtClaimRewardEvent {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        EVT_CLAIM_REWARD_DISCM.serialize(writer)?;
        self.0.serialize(writer)
    }
}
impl EvtClaimRewardEvent {
    pub fn deserialize(buf: &mut &[u8]) -> std::io::Result<Self> {
        let maybe_discm = <[u8; 8]>::deserialize(buf)?;
        if maybe_discm != EVT_CLAIM_REWARD_DISCM {
            return Err(
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!(
                        "discm does not match. Expected: {:?}. Received: {:?}",
                        EVT_CLAIM_REWARD_DISCM, maybe_discm
                    ),
                ),
            );
        }
        Ok(Self(EvtClaimReward::deserialize(buf)?))
    }
}

pub const EVT_CLOSE_CLAIM_FEE_OPERATOR_DISCM: [u8; 8] = [111, 39, 37, 55, 110, 216, 194, 23];
#[derive(Clone, Debug, BorshDeserialize, BorshSerialize, PartialEq,serde::Serialize, serde::Deserialize)]
pub struct EvtCloseClaimFeeOperator {
    // Placeholder fields; actual fields depend on program requirements
    operator: Pubkey,
}
#[derive(Clone, Debug, PartialEq)]
pub struct EvtCloseClaimFeeOperatorEvent(pub EvtCloseClaimFeeOperator);
impl BorshSerialize for EvtCloseClaimFeeOperatorEvent {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        EVT_CLOSE_CLAIM_FEE_OPERATOR_DISCM.serialize(writer)?;
        self.0.serialize(writer)
    }
}
impl EvtCloseClaimFeeOperatorEvent {
    pub fn deserialize(buf: &mut &[u8]) -> std::io::Result<Self> {
        let maybe_discm = <[u8; 8]>::deserialize(buf)?;
        if maybe_discm != EVT_CLOSE_CLAIM_FEE_OPERATOR_DISCM {
            return Err(
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!(
                        "discm does not match. Expected: {:?}. Received: {:?}",
                        EVT_CLOSE_CLAIM_FEE_OPERATOR_DISCM, maybe_discm
                    ),
                ),
            );
        }
        Ok(Self(EvtCloseClaimFeeOperator::deserialize(buf)?))
    }
}

pub const EVT_CLOSE_CONFIG_DISCM: [u8; 8] = [36, 30, 239, 45, 58, 132, 14, 5];
#[derive(Clone, Debug, BorshDeserialize, BorshSerialize, PartialEq,serde::Serialize, serde::Deserialize)]
pub struct EvtCloseConfig {
    // Placeholder fields; actual fields depend on program requirements
    config: Pubkey,
}
#[derive(Clone, Debug, PartialEq)]
pub struct EvtCloseConfigEvent(pub EvtCloseConfig);
impl BorshSerialize for EvtCloseConfigEvent {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        EVT_CLOSE_CONFIG_DISCM.serialize(writer)?;
        self.0.serialize(writer)
    }
}
impl EvtCloseConfigEvent {
    pub fn deserialize(buf: &mut &[u8]) -> std::io::Result<Self> {
        let maybe_discm = <[u8; 8]>::deserialize(buf)?;
        if maybe_discm != EVT_CLOSE_CONFIG_DISCM {
            return Err(
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!(
                        "discm does not match. Expected: {:?}. Received: {:?}",
                        EVT_CLOSE_CONFIG_DISCM, maybe_discm
                    ),
                ),
            );
        }
        Ok(Self(EvtCloseConfig::deserialize(buf)?))
    }
}

pub const EVT_CLOSE_POSITION_DISCM: [u8; 8] = [20, 145, 144, 68, 143, 142, 214, 178];
#[derive(Clone, Debug, BorshDeserialize, BorshSerialize, PartialEq,serde::Serialize, serde::Deserialize)]
pub struct EvtClosePosition {
    position: Pubkey,
    owner: Pubkey,
}
#[derive(Clone, Debug, PartialEq)]
pub struct EvtClosePositionEvent(pub EvtClosePosition);
impl BorshSerialize for EvtClosePositionEvent {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        EVT_CLOSE_POSITION_DISCM.serialize(writer)?;
        self.0.serialize(writer)
    }
}
impl EvtClosePositionEvent {
    pub fn deserialize(buf: &mut &[u8]) -> std::io::Result<Self> {
        let maybe_discm = <[u8; 8]>::deserialize(buf)?;
        if maybe_discm != EVT_CLOSE_POSITION_DISCM {
            return Err(
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!(
                        "discm does not match. Expected: {:?}. Received: {:?}",
                        EVT_CLOSE_POSITION_DISCM, maybe_discm
                    ),
                ),
            );
        }
        Ok(Self(EvtClosePosition::deserialize(buf)?))
    }
}

pub const EVT_CREATE_CLAIM_FEE_OPERATOR_DISCM: [u8; 8] = [21, 6, 153, 120, 68, 116, 28, 177];
#[derive(Clone, Debug, BorshDeserialize, BorshSerialize, PartialEq,serde::Serialize, serde::Deserialize)]
pub struct EvtCreateClaimFeeOperator {
    // Placeholder fields; actual fields depend on program requirements
    operator: Pubkey,
}
#[derive(Clone, Debug, PartialEq)]
pub struct EvtCreateClaimFeeOperatorEvent(pub EvtCreateClaimFeeOperator);
impl BorshSerialize for EvtCreateClaimFeeOperatorEvent {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        EVT_CREATE_CLAIM_FEE_OPERATOR_DISCM.serialize(writer)?;
        self.0.serialize(writer)
    }
}
impl EvtCreateClaimFeeOperatorEvent {
    pub fn deserialize(buf: &mut &[u8]) -> std::io::Result<Self> {
        let maybe_discm = <[u8; 8]>::deserialize(buf)?;
        if maybe_discm != EVT_CREATE_CLAIM_FEE_OPERATOR_DISCM {
            return Err(
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!(
                        "discm does not match. Expected: {:?}. Received: {:?}",
                        EVT_CREATE_CLAIM_FEE_OPERATOR_DISCM, maybe_discm
                    ),
                ),
            );
        }
        Ok(Self(EvtCreateClaimFeeOperator::deserialize(buf)?))
    }
}

pub const EVT_CREATE_CONFIG_DISCM: [u8; 8] = [131, 207, 180, 174, 180, 73, 165, 54];
#[derive(Clone, Debug, BorshDeserialize, BorshSerialize, PartialEq,serde::Serialize, serde::Deserialize)]
pub struct EvtCreateConfig {
    // Placeholder fields; actual fields depend on program requirements
    config: Pubkey,
}
#[derive(Clone, Debug, PartialEq)]
pub struct EvtCreateConfigEvent(pub EvtCreateConfig);
impl BorshSerialize for EvtCreateConfigEvent {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        EVT_CREATE_CONFIG_DISCM.serialize(writer)?;
        self.0.serialize(writer)
    }
}
impl EvtCreateConfigEvent {
    pub fn deserialize(buf: &mut &[u8]) -> std::io::Result<Self> {
        let maybe_discm = <[u8; 8]>::deserialize(buf)?;
        if maybe_discm != EVT_CREATE_CONFIG_DISCM {
            return Err(
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!(
                        "discm does not match. Expected: {:?}. Received: {:?}",
                        EVT_CREATE_CONFIG_DISCM, maybe_discm
                    ),
                ),
            );
        }
        Ok(Self(EvtCreateConfig::deserialize(buf)?))
    }
}

pub const EVT_CREATE_DYNAMIC_CONFIG_DISCM: [u8; 8] = [231, 197, 13, 164, 248, 213, 133, 152];
#[derive(Clone, Debug, BorshDeserialize, BorshSerialize, PartialEq,serde::Serialize, serde::Deserialize)]
pub struct EvtCreateDynamicConfig {
    // Placeholder fields; actual fields depend on program requirements
    config: Pubkey,
}
#[derive(Clone, Debug, PartialEq)]
pub struct EvtCreateDynamicConfigEvent(pub EvtCreateDynamicConfig);
impl BorshSerialize for EvtCreateDynamicConfigEvent {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        EVT_CREATE_DYNAMIC_CONFIG_DISCM.serialize(writer)?;
        self.0.serialize(writer)
    }
}
impl EvtCreateDynamicConfigEvent {
    pub fn deserialize(buf: &mut &[u8]) -> std::io::Result<Self> {
        let maybe_discm = <[u8; 8]>::deserialize(buf)?;
        if maybe_discm != EVT_CREATE_DYNAMIC_CONFIG_DISCM {
            return Err(
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!(
                        "discm does not match. Expected: {:?}. Received: {:?}",
                        EVT_CREATE_DYNAMIC_CONFIG_DISCM, maybe_discm
                    ),
                ),
            );
        }
        Ok(Self(EvtCreateDynamicConfig::deserialize(buf)?))
    }
}

pub const EVT_CREATE_POSITION_DISCM: [u8; 8] = [156, 15, 119, 198, 29, 181, 221, 55];
#[derive(Clone, Debug, BorshDeserialize, BorshSerialize, PartialEq,serde::Serialize, serde::Deserialize)]
pub struct EvtCreatePosition {
    lb_pair: Pubkey,
    position: Pubkey,
    owner: Pubkey,
}
#[derive(Clone, Debug, PartialEq)]
pub struct EvtCreatePositionEvent(pub EvtCreatePosition);
impl BorshSerialize for EvtCreatePositionEvent {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        EVT_CREATE_POSITION_DISCM.serialize(writer)?;
        self.0.serialize(writer)
    }
}
impl EvtCreatePositionEvent {
    pub fn deserialize(buf: &mut &[u8]) -> std::io::Result<Self> {
        let maybe_discm = <[u8; 8]>::deserialize(buf)?;
        if maybe_discm != EVT_CREATE_POSITION_DISCM {
            return Err(
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!(
                        "discm does not match. Expected: {:?}. Received: {:?}",
                        EVT_CREATE_POSITION_DISCM, maybe_discm
                    ),
                ),
            );
        }
        Ok(Self(EvtCreatePosition::deserialize(buf)?))
    }
}

pub const EVT_CREATE_TOKEN_BADGE_DISCM: [u8; 8] = [141, 120, 134, 116, 34, 28, 114, 160];
#[derive(Clone, Debug, BorshDeserialize, BorshSerialize, PartialEq,serde::Serialize, serde::Deserialize)]
pub struct EvtCreateTokenBadge {
    // Placeholder fields; actual fields depend on program requirements
    token_mint: Pubkey,
}
#[derive(Clone, Debug, PartialEq)]
pub struct EvtCreateTokenBadgeEvent(pub EvtCreateTokenBadge);
impl BorshSerialize for EvtCreateTokenBadgeEvent {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        EVT_CREATE_TOKEN_BADGE_DISCM.serialize(writer)?;
        self.0.serialize(writer)
    }
}
impl EvtCreateTokenBadgeEvent {
    pub fn deserialize(buf: &mut &[u8]) -> std::io::Result<Self> {
        let maybe_discm = <[u8; 8]>::deserialize(buf)?;
        if maybe_discm != EVT_CREATE_TOKEN_BADGE_DISCM {
            return Err(
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!(
                        "discm does not match. Expected: {:?}. Received: {:?}",
                        EVT_CREATE_TOKEN_BADGE_DISCM, maybe_discm
                    ),
                ),
            );
        }
        Ok(Self(EvtCreateTokenBadge::deserialize(buf)?))
    }
}

pub const EVT_FUND_REWARD_DISCM: [u8; 8] = [104, 233, 237, 122, 199, 191, 121, 85];
#[derive(Clone, Debug, BorshDeserialize, BorshSerialize, PartialEq,serde::Serialize, serde::Deserialize)]
pub struct EvtFundReward {
    lb_pair: Pubkey,
    funder: Pubkey,
    reward_index: u64,
    amount: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct EvtFundRewardEvent(pub EvtFundReward);
impl BorshSerialize for EvtFundRewardEvent {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        EVT_FUND_REWARD_DISCM.serialize(writer)?;
        self.0.serialize(writer)
    }
}
impl EvtFundRewardEvent {
    pub fn deserialize(buf: &mut &[u8]) -> std::io::Result<Self> {
        let maybe_discm = <[u8; 8]>::deserialize(buf)?;
        if maybe_discm != EVT_FUND_REWARD_DISCM {
            return Err(
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!(
                        "discm does not match. Expected: {:?}. Received: {:?}",
                        EVT_FUND_REWARD_DISCM, maybe_discm
                    ),
                ),
            );
        }
        Ok(Self(EvtFundReward::deserialize(buf)?))
    }
}

pub const EVT_INITIALIZE_POOL_DISCM: [u8; 8] = [228, 50, 246, 85, 203, 66, 134, 37];
#[derive(Clone, Debug, BorshDeserialize, BorshSerialize, PartialEq,serde::Serialize, serde::Deserialize)]
pub struct EvtInitializePool {
    lb_pair: Pubkey,
    bin_step: u16,
    token_x: Pubkey,
    token_y: Pubkey,
}
#[derive(Clone, Debug, PartialEq)]
pub struct EvtInitializePoolEvent(pub EvtInitializePool);
impl BorshSerialize for EvtInitializePoolEvent {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        EVT_INITIALIZE_POOL_DISCM.serialize(writer)?;
        self.0.serialize(writer)
    }
}
impl EvtInitializePoolEvent {
    pub fn deserialize(buf: &mut &[u8]) -> std::io::Result<Self> {
        let maybe_discm = <[u8; 8]>::deserialize(buf)?;
        if maybe_discm != EVT_INITIALIZE_POOL_DISCM {
            return Err(
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!(
                        "discm does not match. Expected: {:?}. Received: {:?}",
                        EVT_INITIALIZE_POOL_DISCM, maybe_discm
                    ),
                ),
            );
        }
        Ok(Self(EvtInitializePool::deserialize(buf)?))
    }
}

pub const EVT_INITIALIZE_REWARD_DISCM: [u8; 8] = [129, 91, 188, 3, 246, 52, 185, 249];
#[derive(Clone, Debug, BorshDeserialize, BorshSerialize, PartialEq,serde::Serialize, serde::Deserialize)]
pub struct EvtInitializeReward {
    lb_pair: Pubkey,
    reward_mint: Pubkey,
    funder: Pubkey,
    reward_index: u64,
    reward_duration: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct EvtInitializeRewardEvent(pub EvtInitializeReward);
impl BorshSerialize for EvtInitializeRewardEvent {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        EVT_INITIALIZE_REWARD_DISCM.serialize(writer)?;
        self.0.serialize(writer)
    }
}
impl EvtInitializeRewardEvent {
    pub fn deserialize(buf: &mut &[u8]) -> std::io::Result<Self> {
        let maybe_discm = <[u8; 8]>::deserialize(buf)?;
        if maybe_discm != EVT_INITIALIZE_REWARD_DISCM {
            return Err(
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!(
                        "discm does not match. Expected: {:?}. Received: {:?}",
                        EVT_INITIALIZE_REWARD_DISCM, maybe_discm
                    ),
                ),
            );
        }
        Ok(Self(EvtInitializeReward::deserialize(buf)?))
    }
}

pub const EVT_LOCK_POSITION_DISCM: [u8; 8] = [168, 63, 108, 83, 219, 82, 2, 200];
#[derive(Clone, Debug, BorshDeserialize, BorshSerialize, PartialEq,serde::Serialize, serde::Deserialize)]
pub struct EvtLockPosition {
    // Placeholder fields; actual fields depend on program requirements
    position: Pubkey,
    lock_duration: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct EvtLockPositionEvent(pub EvtLockPosition);
impl BorshSerialize for EvtLockPositionEvent {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        EVT_LOCK_POSITION_DISCM.serialize(writer)?;
        self.0.serialize(writer)
    }
}
impl EvtLockPositionEvent {
    pub fn deserialize(buf: &mut &[u8]) -> std::io::Result<Self> {
        let maybe_discm = <[u8; 8]>::deserialize(buf)?;
        if maybe_discm != EVT_LOCK_POSITION_DISCM {
            return Err(
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!(
                        "discm does not match. Expected: {:?}. Received: {:?}",
                        EVT_LOCK_POSITION_DISCM, maybe_discm
                    ),
                ),
            );
        }
        Ok(Self(EvtLockPosition::deserialize(buf)?))
    }
}

pub const EVT_PERMANENT_LOCK_POSITION_DISCM: [u8; 8] = [145, 143, 162, 218, 218, 80, 67, 11];
#[derive(Clone, Debug, BorshDeserialize, BorshSerialize, PartialEq,serde::Serialize, serde::Deserialize)]
pub struct EvtPermanentLockPosition {
    // Placeholder fields; actual fields depend on program requirements
    position: Pubkey,
}
#[derive(Clone, Debug, PartialEq)]
pub struct EvtPermanentLockPositionEvent(pub EvtPermanentLockPosition);
impl BorshSerialize for EvtPermanentLockPositionEvent {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        EVT_PERMANENT_LOCK_POSITION_DISCM.serialize(writer)?;
        self.0.serialize(writer)
    }
}
impl EvtPermanentLockPositionEvent {
    pub fn deserialize(buf: &mut &[u8]) -> std::io::Result<Self> {
        let maybe_discm = <[u8; 8]>::deserialize(buf)?;
        if maybe_discm != EVT_PERMANENT_LOCK_POSITION_DISCM {
            return Err(
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!(
                        "discm does not match. Expected: {:?}. Received: {:?}",
                        EVT_PERMANENT_LOCK_POSITION_DISCM, maybe_discm
                    ),
                ),
            );
        }
        Ok(Self(EvtPermanentLockPosition::deserialize(buf)?))
    }
}

pub const EVT_REMOVE_LIQUIDITY_DISCM: [u8; 8] = [87, 46, 88, 98, 175, 96, 34, 91];
#[derive(Clone, Debug, BorshDeserialize, BorshSerialize, PartialEq,serde::Serialize, serde::Deserialize)]
pub struct EvtRemoveLiquidity {
    lb_pair: Pubkey,
    from: Pubkey,
    position: Pubkey,
    amounts: [u64; 2],
    active_bin_id: i32,
}
#[derive(Clone, Debug, PartialEq)]
pub struct EvtRemoveLiquidityEvent(pub EvtRemoveLiquidity);
impl BorshSerialize for EvtRemoveLiquidityEvent {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        EVT_REMOVE_LIQUIDITY_DISCM.serialize(writer)?;
        self.0.serialize(writer)
    }
}
impl EvtRemoveLiquidityEvent {
    pub fn deserialize(buf: &mut &[u8]) -> std::io::Result<Self> {
        let maybe_discm = <[u8; 8]>::deserialize(buf)?;
        if maybe_discm != EVT_REMOVE_LIQUIDITY_DISCM {
            return Err(
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!(
                        "discm does not match. Expected: {:?}. Received: {:?}",
                        EVT_REMOVE_LIQUIDITY_DISCM, maybe_discm
                    ),
                ),
            );
        }
        Ok(Self(EvtRemoveLiquidity::deserialize(buf)?))
    }
}

pub const EVT_SET_POOL_STATUS_DISCM: [u8; 8] = [100, 213, 74, 3, 95, 91, 228, 146];
#[derive(Clone, Debug, BorshDeserialize, BorshSerialize, PartialEq,serde::Serialize, serde::Deserialize)]
pub struct EvtSetPoolStatus {
    // Placeholder fields; actual fields depend on program requirements
    lb_pair: Pubkey,
    status: u8,
}
#[derive(Clone, Debug, PartialEq)]
pub struct EvtSetPoolStatusEvent(pub EvtSetPoolStatus);
impl BorshSerialize for EvtSetPoolStatusEvent {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        EVT_SET_POOL_STATUS_DISCM.serialize(writer)?;
        self.0.serialize(writer)
    }
}
impl EvtSetPoolStatusEvent {
    pub fn deserialize(buf: &mut &[u8]) -> std::io::Result<Self> {
        let maybe_discm = <[u8; 8]>::deserialize(buf)?;
        if maybe_discm != EVT_SET_POOL_STATUS_DISCM {
            return Err(
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!(
                        "discm does not match. Expected: {:?}. Received: {:?}",
                        EVT_SET_POOL_STATUS_DISCM, maybe_discm
                    ),
                ),
            );
        }
        Ok(Self(EvtSetPoolStatus::deserialize(buf)?))
    }
}

pub const EVT_SWAP_DISCM: [u8; 8] = [27, 60, 21, 213, 138, 170, 187, 147];
#[derive(Clone, Debug, BorshDeserialize, BorshSerialize, PartialEq,serde::Serialize, serde::Deserialize)]
pub struct EvtSwap {
    lb_pair: Pubkey,
    from: Pubkey,
    start_bin_id: i32,
    end_bin_id: i32,
    amount_in: u64,
    amount_out: u64,
    swap_for_y: bool,
    fee: u64,
    protocol_fee: u64,
    fee_bps: u128,
    host_fee: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct EvtSwapEvent(pub EvtSwap);
impl BorshSerialize for EvtSwapEvent {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        EVT_SWAP_DISCM.serialize(writer)?;
        self.0.serialize(writer)
    }
}
impl EvtSwapEvent {
    pub fn deserialize(buf: &mut &[u8]) -> std::io::Result<Self> {
        let maybe_discm = <[u8; 8]>::deserialize(buf)?;
        if maybe_discm != EVT_SWAP_DISCM {
            return Err(
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!(
                        "discm does not match. Expected: {:?}. Received: {:?}",
                        EVT_SWAP_DISCM, maybe_discm
                    ),
                ),
            );
        }
        Ok(Self(EvtSwap::deserialize(buf)?))
    }
}

pub const EVT_UPDATE_REWARD_DURATION_DISCM: [u8; 8] = [149, 135, 65, 231, 129, 153, 65, 57];
#[derive(Clone, Debug, BorshDeserialize, BorshSerialize, PartialEq,serde::Serialize, serde::Deserialize)]
pub struct EvtUpdateRewardDuration {
    lb_pair: Pubkey,
    reward_index: u64,
    old_reward_duration: u64,
    new_reward_duration: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct EvtUpdateRewardDurationEvent(pub EvtUpdateRewardDuration);
impl BorshSerialize for EvtUpdateRewardDurationEvent {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        EVT_UPDATE_REWARD_DURATION_DISCM.serialize(writer)?;
        self.0.serialize(writer)
    }
}
impl EvtUpdateRewardDurationEvent {
    pub fn deserialize(buf: &mut &[u8]) -> std::io::Result<Self> {
        let maybe_discm = <[u8; 8]>::deserialize(buf)?;
        if maybe_discm != EVT_UPDATE_REWARD_DURATION_DISCM {
            return Err(
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!(
                        "discm does not match. Expected: {:?}. Received: {:?}",
                        EVT_UPDATE_REWARD_DURATION_DISCM, maybe_discm
                    ),
                ),
            );
        }
        Ok(Self(EvtUpdateRewardDuration::deserialize(buf)?))
    }
}

pub const EVT_UPDATE_REWARD_FUNDER_DISCM: [u8; 8] = [76, 154, 208, 13, 40, 115, 246, 146];
#[derive(Clone, Debug, BorshDeserialize, BorshSerialize, PartialEq,serde::Serialize, serde::Deserialize)]
pub struct EvtUpdateRewardFunder {
    lb_pair: Pubkey,
    reward_index: u64,
    old_funder: Pubkey,
    new_funder: Pubkey,
}
#[derive(Clone, Debug, PartialEq)]
pub struct EvtUpdateRewardFunderEvent(pub EvtUpdateRewardFunder);
impl BorshSerialize for EvtUpdateRewardFunderEvent {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        EVT_UPDATE_REWARD_FUNDER_DISCM.serialize(writer)?;
        self.0.serialize(writer)
    }
}
impl EvtUpdateRewardFunderEvent {
    pub fn deserialize(buf: &mut &[u8]) -> std::io::Result<Self> {
        let maybe_discm = <[u8; 8]>::deserialize(buf)?;
        if maybe_discm != EVT_UPDATE_REWARD_FUNDER_DISCM {
            return Err(
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!(
                        "discm does not match. Expected: {:?}. Received: {:?}",
                        EVT_UPDATE_REWARD_FUNDER_DISCM, maybe_discm
                    ),
                ),
            );
        }
        Ok(Self(EvtUpdateRewardFunder::deserialize(buf)?))
    }
}

pub const EVT_WITHDRAW_INELIGIBLE_REWARD_DISCM: [u8; 8] = [248, 215, 184, 78, 31, 180, 179, 168];
#[derive(Clone, Debug, BorshDeserialize, BorshSerialize, PartialEq,serde::Serialize, serde::Deserialize)]
pub struct EvtWithdrawIneligibleReward {
    lb_pair: Pubkey,
    reward_mint: Pubkey,
    amount: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct EvtWithdrawIneligibleRewardEvent(pub EvtWithdrawIneligibleReward);
impl BorshSerialize for EvtWithdrawIneligibleRewardEvent {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        EVT_WITHDRAW_INELIGIBLE_REWARD_DISCM.serialize(writer)?;
        self.0.serialize(writer)
    }
}
impl EvtWithdrawIneligibleRewardEvent {
    pub fn deserialize(buf: &mut &[u8]) -> std::io::Result<Self> {
        let maybe_discm = <[u8; 8]>::deserialize(buf)?;
        if maybe_discm != EVT_WITHDRAW_INELIGIBLE_REWARD_DISCM {
            return Err(
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!(
                        "discm does not match. Expected: {:?}. Received: {:?}",
                        EVT_WITHDRAW_INELIGIBLE_REWARD_DISCM, maybe_discm
                    ),
                ),
            );
        }
        Ok(Self(EvtWithdrawIneligibleReward::deserialize(buf)?))
    }
}