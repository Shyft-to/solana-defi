use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::pubkey::Pubkey;
pub const COMPOSITION_FEE_EVENT_DISCM: [u8; 8] = [128, 151, 123, 106, 17, 102, 113, 142];
#[derive(Clone, Debug, PartialEq, BorshDeserialize, BorshSerialize)]
pub struct CompositionFee {
    from: Pubkey,
    bin_id: i16,
    token_x_fee_amount: u64,
    token_y_fee_amount: u64,
    protocol_token_x_fee_amount: u64,
    protocol_token_y_fee_amount: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct CompositionFeeEvent(pub CompositionFee);
impl BorshSerialize for CompositionFeeEvent {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        COMPOSITION_FEE_EVENT_DISCM.serialize(writer)?;
        self.0.serialize(writer)
    }
}
impl CompositionFeeEvent {
    pub fn deserialize(buf: &mut &[u8]) -> std::io::Result<Self> {
        let maybe_discm = <[u8; 8]>::deserialize(buf)?;
        if maybe_discm != COMPOSITION_FEE_EVENT_DISCM {
            return Err(
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!(
                        "discm does not match. Expected: {:?}. Received: {:?}",
                        COMPOSITION_FEE_EVENT_DISCM, maybe_discm
                    ),
                ),
            );
        }
        Ok(Self(CompositionFee::deserialize(buf)?))
    }
}
pub const ADD_LIQUIDITY_EVENT_DISCM: [u8; 8] = [31, 94, 125, 90, 227, 52, 61, 186];
#[derive(Clone, Debug, PartialEq, BorshDeserialize, BorshSerialize)]
pub struct AddLiquidity {
    lb_pair: Pubkey,
    from: Pubkey,
    position: Pubkey,
    amounts: [u64; 2],
    active_bin_id: i32,
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
    lb_pair: Pubkey,
    from: Pubkey,
    position: Pubkey,
    amounts: [u64; 2],
    active_bin_id: i32,
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
pub const SWAP_EVENT_DISCM: [u8; 8] = [81, 108, 227, 190, 205, 208, 10, 196];
#[derive(Clone, Debug, PartialEq, BorshDeserialize, BorshSerialize)]
pub struct Swap {
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
pub const CLAIM_REWARD_EVENT_DISCM: [u8; 8] = [148, 116, 134, 204, 22, 171, 85, 95];
#[derive(Clone, Debug, PartialEq, BorshDeserialize, BorshSerialize)]
pub struct ClaimReward {
    lb_pair: Pubkey,
    position: Pubkey,
    owner: Pubkey,
    reward_index: u64,
    total_reward: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct ClaimRewardEvent(pub ClaimReward);
impl BorshSerialize for ClaimRewardEvent {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        CLAIM_REWARD_EVENT_DISCM.serialize(writer)?;
        self.0.serialize(writer)
    }
}
impl ClaimRewardEvent {
    pub fn deserialize(buf: &mut &[u8]) -> std::io::Result<Self> {
        let maybe_discm = <[u8; 8]>::deserialize(buf)?;
        if maybe_discm != CLAIM_REWARD_EVENT_DISCM {
            return Err(
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!(
                        "discm does not match. Expected: {:?}. Received: {:?}",
                        CLAIM_REWARD_EVENT_DISCM, maybe_discm
                    ),
                ),
            );
        }
        Ok(Self(ClaimReward::deserialize(buf)?))
    }
}
pub const FUND_REWARD_EVENT_DISCM: [u8; 8] = [246, 228, 58, 130, 145, 170, 79, 204];
#[derive(Clone, Debug, PartialEq, BorshDeserialize, BorshSerialize)]
pub struct FundReward {
    lb_pair: Pubkey,
    funder: Pubkey,
    reward_index: u64,
    amount: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct FundRewardEvent(pub FundReward);
impl BorshSerialize for FundRewardEvent {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        FUND_REWARD_EVENT_DISCM.serialize(writer)?;
        self.0.serialize(writer)
    }
}
impl FundRewardEvent {
    pub fn deserialize(buf: &mut &[u8]) -> std::io::Result<Self> {
        let maybe_discm = <[u8; 8]>::deserialize(buf)?;
        if maybe_discm != FUND_REWARD_EVENT_DISCM {
            return Err(
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!(
                        "discm does not match. Expected: {:?}. Received: {:?}",
                        FUND_REWARD_EVENT_DISCM, maybe_discm
                    ),
                ),
            );
        }
        Ok(Self(FundReward::deserialize(buf)?))
    }
}
pub const INITIALIZE_REWARD_EVENT_DISCM: [u8; 8] = [211, 153, 88, 62, 149, 60, 177, 70];
#[derive(Clone, Debug, PartialEq, BorshDeserialize, BorshSerialize)]
pub struct InitializeReward {
    lb_pair: Pubkey,
    reward_mint: Pubkey,
    funder: Pubkey,
    reward_index: u64,
    reward_duration: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct InitializeRewardEvent(pub InitializeReward);
impl BorshSerialize for InitializeRewardEvent {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        INITIALIZE_REWARD_EVENT_DISCM.serialize(writer)?;
        self.0.serialize(writer)
    }
}
impl InitializeRewardEvent {
    pub fn deserialize(buf: &mut &[u8]) -> std::io::Result<Self> {
        let maybe_discm = <[u8; 8]>::deserialize(buf)?;
        if maybe_discm != INITIALIZE_REWARD_EVENT_DISCM {
            return Err(
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!(
                        "discm does not match. Expected: {:?}. Received: {:?}",
                        INITIALIZE_REWARD_EVENT_DISCM, maybe_discm
                    ),
                ),
            );
        }
        Ok(Self(InitializeReward::deserialize(buf)?))
    }
}
pub const UPDATE_REWARD_DURATION_EVENT_DISCM: [u8; 8] = [
    223,
    245,
    224,
    153,
    49,
    29,
    163,
    172,
];
#[derive(Clone, Debug, PartialEq, BorshDeserialize, BorshSerialize)]
pub struct UpdateRewardDuration {
    lb_pair: Pubkey,
    reward_index: u64,
    old_reward_duration: u64,
    new_reward_duration: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct UpdateRewardDurationEvent(pub UpdateRewardDuration);
impl BorshSerialize for UpdateRewardDurationEvent {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        UPDATE_REWARD_DURATION_EVENT_DISCM.serialize(writer)?;
        self.0.serialize(writer)
    }
}
impl UpdateRewardDurationEvent {
    pub fn deserialize(buf: &mut &[u8]) -> std::io::Result<Self> {
        let maybe_discm = <[u8; 8]>::deserialize(buf)?;
        if maybe_discm != UPDATE_REWARD_DURATION_EVENT_DISCM {
            return Err(
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!(
                        "discm does not match. Expected: {:?}. Received: {:?}",
                        UPDATE_REWARD_DURATION_EVENT_DISCM, maybe_discm
                    ),
                ),
            );
        }
        Ok(Self(UpdateRewardDuration::deserialize(buf)?))
    }
}
pub const UPDATE_REWARD_FUNDER_EVENT_DISCM: [u8; 8] = [
    224,
    178,
    174,
    74,
    252,
    165,
    85,
    180,
];
#[derive(Clone, Debug, PartialEq, BorshDeserialize, BorshSerialize)]
pub struct UpdateRewardFunder {
    lb_pair: Pubkey,
    reward_index: u64,
    old_funder: Pubkey,
    new_funder: Pubkey,
}
#[derive(Clone, Debug, PartialEq)]
pub struct UpdateRewardFunderEvent(pub UpdateRewardFunder);
impl BorshSerialize for UpdateRewardFunderEvent {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        UPDATE_REWARD_FUNDER_EVENT_DISCM.serialize(writer)?;
        self.0.serialize(writer)
    }
}
impl UpdateRewardFunderEvent {
    pub fn deserialize(buf: &mut &[u8]) -> std::io::Result<Self> {
        let maybe_discm = <[u8; 8]>::deserialize(buf)?;
        if maybe_discm != UPDATE_REWARD_FUNDER_EVENT_DISCM {
            return Err(
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!(
                        "discm does not match. Expected: {:?}. Received: {:?}",
                        UPDATE_REWARD_FUNDER_EVENT_DISCM, maybe_discm
                    ),
                ),
            );
        }
        Ok(Self(UpdateRewardFunder::deserialize(buf)?))
    }
}
pub const POSITION_CLOSE_EVENT_DISCM: [u8; 8] = [255, 196, 16, 107, 28, 202, 53, 128];
#[derive(Clone, Debug, PartialEq, BorshDeserialize, BorshSerialize)]
pub struct PositionClose {
    position: Pubkey,
    owner: Pubkey,
}
#[derive(Clone, Debug, PartialEq)]
pub struct PositionCloseEvent(pub PositionClose);
impl BorshSerialize for PositionCloseEvent {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        POSITION_CLOSE_EVENT_DISCM.serialize(writer)?;
        self.0.serialize(writer)
    }
}
impl PositionCloseEvent {
    pub fn deserialize(buf: &mut &[u8]) -> std::io::Result<Self> {
        let maybe_discm = <[u8; 8]>::deserialize(buf)?;
        if maybe_discm != POSITION_CLOSE_EVENT_DISCM {
            return Err(
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!(
                        "discm does not match. Expected: {:?}. Received: {:?}",
                        POSITION_CLOSE_EVENT_DISCM, maybe_discm
                    ),
                ),
            );
        }
        Ok(Self(PositionClose::deserialize(buf)?))
    }
}
pub const CLAIM_FEE_EVENT_DISCM: [u8; 8] = [75, 122, 154, 48, 140, 74, 123, 163];
#[derive(Clone, Debug, PartialEq, BorshDeserialize, BorshSerialize)]
pub struct ClaimFee {
    lb_pair: Pubkey,
    position: Pubkey,
    owner: Pubkey,
    fee_x: u64,
    fee_y: u64,
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
pub const LB_PAIR_CREATE_EVENT_DISCM: [u8; 8] = [185, 74, 252, 125, 27, 215, 188, 111];
#[derive(Clone, Debug, PartialEq, BorshDeserialize, BorshSerialize)]
pub struct LbPairCreate {
    lb_pair: Pubkey,
    bin_step: u16,
    token_x: Pubkey,
    token_y: Pubkey,
}
#[derive(Clone, Debug, PartialEq)]
pub struct LbPairCreateEvent(pub LbPairCreate);
impl BorshSerialize for LbPairCreateEvent {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        LB_PAIR_CREATE_EVENT_DISCM.serialize(writer)?;
        self.0.serialize(writer)
    }
}
impl LbPairCreateEvent {
    pub fn deserialize(buf: &mut &[u8]) -> std::io::Result<Self> {
        let maybe_discm = <[u8; 8]>::deserialize(buf)?;
        if maybe_discm != LB_PAIR_CREATE_EVENT_DISCM {
            return Err(
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!(
                        "discm does not match. Expected: {:?}. Received: {:?}",
                        LB_PAIR_CREATE_EVENT_DISCM, maybe_discm
                    ),
                ),
            );
        }
        Ok(Self(LbPairCreate::deserialize(buf)?))
    }
}
pub const POSITION_CREATE_EVENT_DISCM: [u8; 8] = [144, 142, 252, 84, 157, 53, 37, 121];
#[derive(Clone, Debug, PartialEq, BorshDeserialize, BorshSerialize)]
pub struct PositionCreate {
    lb_pair: Pubkey,
    position: Pubkey,
    owner: Pubkey,
}
#[derive(Clone, Debug, PartialEq)]
pub struct PositionCreateEvent(pub PositionCreate);
impl BorshSerialize for PositionCreateEvent {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        POSITION_CREATE_EVENT_DISCM.serialize(writer)?;
        self.0.serialize(writer)
    }
}
impl PositionCreateEvent {
    pub fn deserialize(buf: &mut &[u8]) -> std::io::Result<Self> {
        let maybe_discm = <[u8; 8]>::deserialize(buf)?;
        if maybe_discm != POSITION_CREATE_EVENT_DISCM {
            return Err(
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!(
                        "discm does not match. Expected: {:?}. Received: {:?}",
                        POSITION_CREATE_EVENT_DISCM, maybe_discm
                    ),
                ),
            );
        }
        Ok(Self(PositionCreate::deserialize(buf)?))
    }
}
pub const FEE_PARAMETER_UPDATE_EVENT_DISCM: [u8; 8] = [
    48,
    76,
    241,
    117,
    144,
    215,
    242,
    44,
];
#[derive(Clone, Debug, PartialEq, BorshDeserialize, BorshSerialize)]
pub struct FeeParameterUpdate {
    lb_pair: Pubkey,
    protocol_share: u16,
    base_factor: u16,
}
#[derive(Clone, Debug, PartialEq)]
pub struct FeeParameterUpdateEvent(pub FeeParameterUpdate);
impl BorshSerialize for FeeParameterUpdateEvent {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        FEE_PARAMETER_UPDATE_EVENT_DISCM.serialize(writer)?;
        self.0.serialize(writer)
    }
}
impl FeeParameterUpdateEvent {
    pub fn deserialize(buf: &mut &[u8]) -> std::io::Result<Self> {
        let maybe_discm = <[u8; 8]>::deserialize(buf)?;
        if maybe_discm != FEE_PARAMETER_UPDATE_EVENT_DISCM {
            return Err(
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!(
                        "discm does not match. Expected: {:?}. Received: {:?}",
                        FEE_PARAMETER_UPDATE_EVENT_DISCM, maybe_discm
                    ),
                ),
            );
        }
        Ok(Self(FeeParameterUpdate::deserialize(buf)?))
    }
}
pub const INCREASE_OBSERVATION_EVENT_DISCM: [u8; 8] = [
    99,
    249,
    17,
    121,
    166,
    156,
    207,
    215,
];
#[derive(Clone, Debug, PartialEq, BorshDeserialize, BorshSerialize)]
pub struct IncreaseObservation {
    oracle: Pubkey,
    new_observation_length: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct IncreaseObservationEvent(pub IncreaseObservation);
impl BorshSerialize for IncreaseObservationEvent {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        INCREASE_OBSERVATION_EVENT_DISCM.serialize(writer)?;
        self.0.serialize(writer)
    }
}
impl IncreaseObservationEvent {
    pub fn deserialize(buf: &mut &[u8]) -> std::io::Result<Self> {
        let maybe_discm = <[u8; 8]>::deserialize(buf)?;
        if maybe_discm != INCREASE_OBSERVATION_EVENT_DISCM {
            return Err(
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!(
                        "discm does not match. Expected: {:?}. Received: {:?}",
                        INCREASE_OBSERVATION_EVENT_DISCM, maybe_discm
                    ),
                ),
            );
        }
        Ok(Self(IncreaseObservation::deserialize(buf)?))
    }
}
pub const WITHDRAW_INELIGIBLE_REWARD_EVENT_DISCM: [u8; 8] = [
    231,
    189,
    65,
    149,
    102,
    215,
    154,
    244,
];
#[derive(Clone, Debug, PartialEq, BorshDeserialize, BorshSerialize)]
pub struct WithdrawIneligibleReward {
    lb_pair: Pubkey,
    reward_mint: Pubkey,
    amount: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct WithdrawIneligibleRewardEvent(pub WithdrawIneligibleReward);
impl BorshSerialize for WithdrawIneligibleRewardEvent {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        WITHDRAW_INELIGIBLE_REWARD_EVENT_DISCM.serialize(writer)?;
        self.0.serialize(writer)
    }
}
impl WithdrawIneligibleRewardEvent {
    pub fn deserialize(buf: &mut &[u8]) -> std::io::Result<Self> {
        let maybe_discm = <[u8; 8]>::deserialize(buf)?;
        if maybe_discm != WITHDRAW_INELIGIBLE_REWARD_EVENT_DISCM {
            return Err(
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!(
                        "discm does not match. Expected: {:?}. Received: {:?}",
                        WITHDRAW_INELIGIBLE_REWARD_EVENT_DISCM, maybe_discm
                    ),
                ),
            );
        }
        Ok(Self(WithdrawIneligibleReward::deserialize(buf)?))
    }
}
pub const UPDATE_POSITION_OPERATOR_EVENT_DISCM: [u8; 8] = [
    39,
    115,
    48,
    204,
    246,
    47,
    66,
    57,
];
#[derive(Clone, Debug, PartialEq, BorshDeserialize, BorshSerialize)]
pub struct UpdatePositionOperator {
    position: Pubkey,
    old_operator: Pubkey,
    new_operator: Pubkey,
}
#[derive(Clone, Debug, PartialEq)]
pub struct UpdatePositionOperatorEvent(pub UpdatePositionOperator);
impl BorshSerialize for UpdatePositionOperatorEvent {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        UPDATE_POSITION_OPERATOR_EVENT_DISCM.serialize(writer)?;
        self.0.serialize(writer)
    }
}
impl UpdatePositionOperatorEvent {
    pub fn deserialize(buf: &mut &[u8]) -> std::io::Result<Self> {
        let maybe_discm = <[u8; 8]>::deserialize(buf)?;
        if maybe_discm != UPDATE_POSITION_OPERATOR_EVENT_DISCM {
            return Err(
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!(
                        "discm does not match. Expected: {:?}. Received: {:?}",
                        UPDATE_POSITION_OPERATOR_EVENT_DISCM, maybe_discm
                    ),
                ),
            );
        }
        Ok(Self(UpdatePositionOperator::deserialize(buf)?))
    }
}
pub const UPDATE_POSITION_LOCK_RELEASE_POINT_EVENT_DISCM: [u8; 8] = [
    133,
    214,
    66,
    224,
    64,
    12,
    7,
    191,
];
#[derive(Clone, Debug, PartialEq, BorshDeserialize, BorshSerialize)]
pub struct UpdatePositionLockReleasePoint {
    position: Pubkey,
    current_point: u64,
    new_lock_release_point: u64,
    old_lock_release_point: u64,
    sender: Pubkey,
}
#[derive(Clone, Debug, PartialEq)]
pub struct UpdatePositionLockReleasePointEvent(pub UpdatePositionLockReleasePoint);
impl BorshSerialize for UpdatePositionLockReleasePointEvent {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        UPDATE_POSITION_LOCK_RELEASE_POINT_EVENT_DISCM.serialize(writer)?;
        self.0.serialize(writer)
    }
}
impl UpdatePositionLockReleasePointEvent {
    pub fn deserialize(buf: &mut &[u8]) -> std::io::Result<Self> {
        let maybe_discm = <[u8; 8]>::deserialize(buf)?;
        if maybe_discm != UPDATE_POSITION_LOCK_RELEASE_POINT_EVENT_DISCM {
            return Err(
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!(
                        "discm does not match. Expected: {:?}. Received: {:?}",
                        UPDATE_POSITION_LOCK_RELEASE_POINT_EVENT_DISCM, maybe_discm
                    ),
                ),
            );
        }
        Ok(Self(UpdatePositionLockReleasePoint::deserialize(buf)?))
    }
}
pub const GO_TO_A_BIN_EVENT_DISCM: [u8; 8] = [59, 138, 76, 68, 138, 131, 176, 67];
#[derive(Clone, Debug, PartialEq, BorshDeserialize, BorshSerialize)]
pub struct GoToABin {
    lb_pair: Pubkey,
    from_bin_id: i32,
    to_bin_id: i32,
}
#[derive(Clone, Debug, PartialEq)]
pub struct GoToABinEvent(pub GoToABin);
impl BorshSerialize for GoToABinEvent {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        GO_TO_A_BIN_EVENT_DISCM.serialize(writer)?;
        self.0.serialize(writer)
    }
}
impl GoToABinEvent {
    pub fn deserialize(buf: &mut &[u8]) -> std::io::Result<Self> {
        let maybe_discm = <[u8; 8]>::deserialize(buf)?;
        if maybe_discm != GO_TO_A_BIN_EVENT_DISCM {
            return Err(
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!(
                        "discm does not match. Expected: {:?}. Received: {:?}",
                        GO_TO_A_BIN_EVENT_DISCM, maybe_discm
                    ),
                ),
            );
        }
        Ok(Self(GoToABin::deserialize(buf)?))
    }
}
