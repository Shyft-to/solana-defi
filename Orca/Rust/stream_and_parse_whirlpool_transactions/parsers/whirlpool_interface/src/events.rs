use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::pubkey::Pubkey;
use std::io;

// LiquidityDecreased event
pub const LIQUIDITY_DECREASED_EVENT_DISCM: [u8; 8] = [166, 1, 36, 71, 112, 202, 181, 171];

#[derive(Clone, Debug, PartialEq, BorshDeserialize, BorshSerialize, serde::Serialize)]
pub struct LiquidityDecreasedEvent {
    pub whirlpool: Pubkey,
    pub position: Pubkey,
    pub tick_lower_index: i32,
    pub tick_upper_index: i32,
    pub liquidity: u128,
    pub token_a_amount: u64,
    pub token_b_amount: u64,
    pub token_a_transfer_fee: u64,
    pub token_b_transfer_fee: u64,
}

#[derive(Clone, Debug, PartialEq)]
pub struct LiquidityDecreasedEventEvent(pub LiquidityDecreasedEvent);

impl BorshSerialize for LiquidityDecreasedEventEvent {
    fn serialize<W: io::Write>(&self, writer: &mut W) -> io::Result<()> {
        LIQUIDITY_DECREASED_EVENT_DISCM.serialize(writer)?;
        self.0.serialize(writer)
    }
}

impl LiquidityDecreasedEventEvent {
    pub fn deserialize(buf: &mut &[u8]) -> io::Result<Self> {
        let maybe_discm = <[u8; 8]>::deserialize(buf)?;
        if maybe_discm != LIQUIDITY_DECREASED_EVENT_DISCM {
            return Err(io::Error::new(
                io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    LIQUIDITY_DECREASED_EVENT_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(LiquidityDecreasedEvent::deserialize(buf)?))
    }
}

// LiquidityIncreased event
pub const LIQUIDITY_INCREASED_EVENT_DISCM: [u8; 8] = [30, 7, 144, 181, 102, 254, 155, 161];

#[derive(Clone, Debug, PartialEq, BorshDeserialize, BorshSerialize, serde::Serialize)]
pub struct LiquidityIncreasedEvent {
    pub whirlpool: Pubkey,
    pub position: Pubkey,
    pub tick_lower_index: i32,
    pub tick_upper_index: i32,
    pub liquidity: u128,
    pub token_a_amount: u64,
    pub token_b_amount: u64,
    pub token_a_transfer_fee: u64,
    pub token_b_transfer_fee: u64,
}

#[derive(Clone, Debug, PartialEq)]
pub struct LiquidityIncreasedEventEvent(pub LiquidityIncreasedEvent);

impl BorshSerialize for LiquidityIncreasedEventEvent {
    fn serialize<W: io::Write>(&self, writer: &mut W) -> io::Result<()> {
        LIQUIDITY_INCREASED_EVENT_DISCM.serialize(writer)?;
        self.0.serialize(writer)
    }
}

impl LiquidityIncreasedEventEvent {
    pub fn deserialize(buf: &mut &[u8]) -> io::Result<Self> {
        let maybe_discm = <[u8; 8]>::deserialize(buf)?;
        if maybe_discm != LIQUIDITY_INCREASED_EVENT_DISCM {
            return Err(io::Error::new(
                io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    LIQUIDITY_INCREASED_EVENT_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(LiquidityIncreasedEvent::deserialize(buf)?))
    }
}

// PoolInitialized event
pub const POOL_INITIALIZED_EVENT_DISCM: [u8; 8] = [100, 118, 173, 87, 12, 198, 254, 229];

#[derive(Clone, Debug, PartialEq, BorshDeserialize, BorshSerialize, serde::Serialize)]
pub struct PoolInitializedEvent {
    pub whirlpool: Pubkey,
    pub whirlpools_config: Pubkey,
    pub token_mint_a: Pubkey,
    pub token_mint_b: Pubkey,
    pub tick_spacing: u16,
    pub token_program_a: Pubkey,
    pub token_program_b: Pubkey,
    pub decimals_a: u8,
    pub decimals_b: u8,
    pub initial_sqrt_price: u128,
}

#[derive(Clone, Debug, PartialEq)]
pub struct PoolInitializedEventEvent(pub PoolInitializedEvent);

impl BorshSerialize for PoolInitializedEventEvent {
    fn serialize<W: io::Write>(&self, writer: &mut W) -> io::Result<()> {
        POOL_INITIALIZED_EVENT_DISCM.serialize(writer)?;
        self.0.serialize(writer)
    }
}

impl PoolInitializedEventEvent {
    pub fn deserialize(buf: &mut &[u8]) -> io::Result<Self> {
        let maybe_discm = <[u8; 8]>::deserialize(buf)?;
        if maybe_discm != POOL_INITIALIZED_EVENT_DISCM {
            return Err(io::Error::new(
                io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    POOL_INITIALIZED_EVENT_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(PoolInitializedEvent::deserialize(buf)?))
    }
}

pub const TRADED_EVENT_DISCM: [u8; 8] = [225, 202, 73, 175, 147, 43, 160, 150];

#[derive(Clone, Debug, PartialEq, BorshDeserialize, BorshSerialize, serde::Serialize)]
pub struct TradedEvent {
    pub whirlpool: Pubkey,
    pub a_to_b: bool,
    pub pre_sqrt_price: u128,
    pub post_sqrt_price: u128,
    pub input_amount: u64,
    pub output_amount: u64,
    pub input_transfer_fee: u64,
    pub output_transfer_fee: u64,
    pub lp_fee: u64,
    pub protocol_fee: u64,
}

#[derive(Clone, Debug, PartialEq)]
pub struct TradedEventEvent(pub TradedEvent);

impl BorshSerialize for TradedEventEvent {
    fn serialize<W: io::Write>(&self, writer: &mut W) -> io::Result<()> {
        TRADED_EVENT_DISCM.serialize(writer)?;
        self.0.serialize(writer)
    }
}

impl TradedEventEvent {
    pub fn deserialize(buf: &mut &[u8]) -> io::Result<Self> {
        let maybe_discm = <[u8; 8]>::deserialize(buf)?;
        if maybe_discm != TRADED_EVENT_DISCM {
            return Err(io::Error::new(
                io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    TRADED_EVENT_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(TradedEvent::deserialize(buf)?))
    }
}