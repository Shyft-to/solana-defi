use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::pubkey::Pubkey;
use std::fmt;

// Discriminators for events
pub const CLAIM_VESTED_EVENT_DISCM: [u8; 8] = [21, 194, 114, 87, 120, 211, 226, 32];
pub const CREATE_VESTING_EVENT_DISCM: [u8; 8] = [150, 152, 11, 179, 52, 210, 191, 125];
pub const POOL_CREATE_EVENT_DISCM: [u8; 8] = [151, 215, 226, 9, 118, 161, 115, 174];
pub const TRADE_EVENT_DISCM: [u8; 8] = [228, 69, 165, 46, 81, 203, 154, 29];


// ClaimVestedEvent
#[derive(Clone, Debug, BorshDeserialize, BorshSerialize, PartialEq,serde::Serialize, serde::Deserialize)]
pub struct ClaimVestedEvent {
    pub pool_state: Pubkey,
    pub beneficiary: Pubkey,
    pub claim_amount: u64,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ClaimVestedEventEvent(pub ClaimVestedEvent);

impl BorshSerialize for ClaimVestedEventEvent {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        CLAIM_VESTED_EVENT_DISCM.serialize(writer)?;
        self.0.serialize(writer)
    }
}

impl ClaimVestedEventEvent {
    pub fn deserialize(buf: &mut &[u8]) -> std::io::Result<Self> {
        let maybe_discm = <[u8; 8]>::deserialize(buf)?;
        if maybe_discm != CLAIM_VESTED_EVENT_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    CLAIM_VESTED_EVENT_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(ClaimVestedEvent::deserialize(buf)?))
    }
}

// CreateVestingEvent
#[derive(Clone, Debug, BorshDeserialize, BorshSerialize, PartialEq,serde::Serialize, serde::Deserialize)]
pub struct CreateVestingEvent {
    pub pool_state: Pubkey,
    pub beneficiary: Pubkey,
    pub share_amount: u64,
}

#[derive(Clone, Debug, PartialEq)]
pub struct CreateVestingEventEvent(pub CreateVestingEvent);

impl BorshSerialize for CreateVestingEventEvent {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        CREATE_VESTING_EVENT_DISCM.serialize(writer)?;
        self.0.serialize(writer)
    }
}

impl CreateVestingEventEvent {
    pub fn deserialize(buf: &mut &[u8]) -> std::io::Result<Self> {
        let maybe_discm = <[u8; 8]>::deserialize(buf)?;
        if maybe_discm != CREATE_VESTING_EVENT_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    CREATE_VESTING_EVENT_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(CreateVestingEvent::deserialize(buf)?))
    }
}

// Supporting structs and enums for PoolCreateEvent
#[derive(Clone, Debug, BorshDeserialize, BorshSerialize, PartialEq,serde::Serialize, serde::Deserialize)]
pub struct MintParams {
    pub decimals: u8,
    pub name: String,
    pub symbol: String,
    pub uri: String,
}

#[derive(Clone, Debug, BorshDeserialize, BorshSerialize, PartialEq,serde::Serialize, serde::Deserialize)]
pub struct ConstantCurve {
    pub supply: u64,
    pub total_base_sell: u64,
    pub total_quote_fund_raising: u64,
    pub migrate_type: u8,
}

#[derive(Clone, Debug, BorshDeserialize, BorshSerialize, PartialEq,serde::Serialize, serde::Deserialize)]
pub struct FixedCurve {
    pub supply: u64,
    pub total_quote_fund_raising: u64,
    pub migrate_type: u8,
}

#[derive(Clone, Debug, BorshDeserialize, BorshSerialize, PartialEq,serde::Serialize, serde::Deserialize)]
pub struct LinearCurve {
    pub supply: u64,
    pub total_quote_fund_raising: u64,
    pub migrate_type: u8,
}

#[derive(Clone, Debug, BorshDeserialize, BorshSerialize, PartialEq,serde::Serialize, serde::Deserialize)]
pub enum CurveParams {
    Constant { data: ConstantCurve },
    Fixed { data: FixedCurve },
    Linear { data: LinearCurve },
}

#[derive(Clone, Debug, BorshDeserialize, BorshSerialize, PartialEq,serde::Serialize, serde::Deserialize)]
pub struct VestingParams {
    pub total_locked_amount: u64,
    pub cliff_period: u64,
    pub unlock_period: u64,
}

// PoolCreateEvent
#[derive(Clone, Debug, BorshDeserialize, BorshSerialize, PartialEq,serde::Serialize, serde::Deserialize)]
pub struct PoolCreateEvent {
    pub pool_state: Pubkey,
    pub creator: Pubkey,
    pub config: Pubkey,
    pub base_mint_param: MintParams,
    pub curve_param: CurveParams,
    pub vesting_param: VestingParams,
}

#[derive(Clone, Debug, PartialEq)]
pub struct PoolCreateEventEvent(pub PoolCreateEvent);

impl BorshSerialize for PoolCreateEventEvent {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        POOL_CREATE_EVENT_DISCM.serialize(writer)?;
        self.0.serialize(writer)
    }
}

impl PoolCreateEventEvent {
    pub fn deserialize(buf: &mut &[u8]) -> std::io::Result<Self> {
        let maybe_discm = <[u8; 8]>::deserialize(buf)?;
        if maybe_discm != POOL_CREATE_EVENT_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    POOL_CREATE_EVENT_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(PoolCreateEvent::deserialize(buf)?))
    }
}

// Supporting enums for TradeEvent
#[derive(Clone, Debug, BorshDeserialize, BorshSerialize, PartialEq,serde::Serialize, serde::Deserialize)]
pub enum PoolStatus {
    Fund,
    Migrate,
    Trade,
}

#[derive(Clone, Debug, BorshDeserialize, BorshSerialize, PartialEq,serde::Serialize, serde::Deserialize)]
pub enum TradeDirection {
    Buy,
    Sell,
}
impl fmt::Display for TradeDirection {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TradeDirection::Buy => write!(f, "Buy"),
            TradeDirection::Sell => write!(f, "Sell"),
        }
    }
}

#[derive(Clone, Debug, BorshDeserialize, BorshSerialize, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct TradeEvent {
    pub pool_state: Pubkey,
    pub total_base_sell: u64,
    pub virtual_base: u64,
    pub virtual_quote: u64,
    pub real_base_before: u64,
    pub real_quote_before: u64,
    pub real_base_after: u64,
    pub real_quote_after: u64,
    pub amount_in: u64,
    pub amount_out: u64,
    pub protocol_fee: u64,
    pub platform_fee: u64,
    pub share_fee: u64,
    pub trade_direction: TradeDirection,
    pub pool_status: PoolStatus,
}



#[derive(Clone, Debug, PartialEq)]
pub struct TradeEventEvent(pub TradeEvent);

impl BorshSerialize for TradeEventEvent {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        TRADE_EVENT_DISCM.serialize(writer)?;
        self.0.serialize(writer)
    }
}
impl TradeEventEvent {
    pub fn deserialize(buf: &mut &[u8]) -> std::io::Result<Self> {
        let maybe_discm = <[u8; 8]>::deserialize(buf)?;
        if maybe_discm != TRADE_EVENT_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    TRADE_EVENT_DISCM, maybe_discm
                ),
            ));
        }

        // consume the extra 8 bytes wrapper (quick/hacky fix)
        if buf.len() < 8 {
            return Err(std::io::Error::new(std::io::ErrorKind::UnexpectedEof, "missing extra header"));
        }
        let _extra: [u8; 8] = <[u8; 8]>::deserialize(buf)?;

        // now deserialize the real struct
        Ok(Self(TradeEvent::deserialize(buf)?))
    }
}
