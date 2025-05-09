use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::pubkey::Pubkey;

// ConfigChangeEvent
pub const CONFIG_CHANGE_EVENT_DISCM: [u8; 8] = [121, 163, 205, 201, 57, 218, 117, 60];

#[derive(Clone, Debug, PartialEq, BorshDeserialize, BorshSerialize)]
pub struct ConfigChangeEvent {
    pub index: u16,
    pub owner: Pubkey,
    pub protocol_fee_rate: u32,
    pub trade_fee_rate: u32,
    pub tick_spacing: u16,
    pub fund_fee_rate: u32,
    pub fund_owner: Pubkey,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ConfigChangeEventEvent(pub ConfigChangeEvent);

impl BorshSerialize for ConfigChangeEventEvent {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        CONFIG_CHANGE_EVENT_DISCM.serialize(writer)?;
        self.0.serialize(writer)
    }
}

impl ConfigChangeEventEvent {
    pub fn deserialize(buf: &mut &[u8]) -> std::io::Result<Self> {
        let maybe_discm = <[u8; 8]>::deserialize(buf)?;
        if maybe_discm != CONFIG_CHANGE_EVENT_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    CONFIG_CHANGE_EVENT_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(ConfigChangeEvent::deserialize(buf)?))
    }
}

// CreatePersonalPositionEvent
pub const CREATE_PERSONAL_POSITION_EVENT_DISCM: [u8; 8] =[121, 163, 205, 201, 57, 218, 117, 60];

#[derive(Clone, Debug, PartialEq, BorshDeserialize, BorshSerialize)]
pub struct CreatePersonalPositionEvent {
    pub pool_state: Pubkey,
    pub minter: Pubkey,
    pub nft_owner: Pubkey,
    pub tick_lower_index: i32,
    pub tick_upper_index: i32,
    pub liquidity: u128,
    pub deposit_amount0: u64,
    pub deposit_amount1: u64,
    pub deposit_amount0_transfer_fee: u64,
    pub deposit_amount1_transfer_fee: u64,
}

#[derive(Clone, Debug, PartialEq)]
pub struct CreatePersonalPositionEventEvent(pub CreatePersonalPositionEvent);

impl BorshSerialize for CreatePersonalPositionEventEvent {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        CREATE_PERSONAL_POSITION_EVENT_DISCM.serialize(writer)?;
        self.0.serialize(writer)
    }
}

impl CreatePersonalPositionEventEvent {
    pub fn deserialize(buf: &mut &[u8]) -> std::io::Result<Self> {
        let maybe_discm = <[u8; 8]>::deserialize(buf)?;
        if maybe_discm != CREATE_PERSONAL_POSITION_EVENT_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    CREATE_PERSONAL_POSITION_EVENT_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(CreatePersonalPositionEvent::deserialize(buf)?))
    }
}

// IncreaseLiquidityEvent
pub const INCREASE_LIQUIDITY_EVENT_DISCM: [u8; 8] = [121, 163, 205, 201, 57, 218, 117, 60];

#[derive(Clone, Debug, PartialEq, BorshDeserialize, BorshSerialize)]
pub struct IncreaseLiquidityEvent {
    pub position_nft_mint: Pubkey,
    pub liquidity: u128,
    pub amount0: u64,
    pub amount1: u64,
    pub amount0_transfer_fee: u64,
    pub amount1_transfer_fee: u64,
}

#[derive(Clone, Debug, PartialEq)]
pub struct IncreaseLiquidityEventEvent(pub IncreaseLiquidityEvent);

impl BorshSerialize for IncreaseLiquidityEventEvent {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        INCREASE_LIQUIDITY_EVENT_DISCM.serialize(writer)?;
        self.0.serialize(writer)
    }
}

impl IncreaseLiquidityEventEvent {
    pub fn deserialize(buf: &mut &[u8]) -> std::io::Result<Self> {
        let maybe_discm = <[u8; 8]>::deserialize(buf)?;
        if maybe_discm != INCREASE_LIQUIDITY_EVENT_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    INCREASE_LIQUIDITY_EVENT_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(IncreaseLiquidityEvent::deserialize(buf)?))
    }
}

// DecreaseLiquidityEvent
pub const DECREASE_LIQUIDITY_EVENT_DISCM: [u8; 8] = [121, 163, 205, 201, 57, 218, 117, 60];

#[derive(Clone, Debug, PartialEq, BorshDeserialize, BorshSerialize)]
pub struct DecreaseLiquidityEvent {
    pub position_nft_mint: Pubkey,
    pub liquidity: u128,
    pub decrease_amount0: u64,
    pub decrease_amount1: u64,
    pub fee_amount0: u64,
    pub fee_amount1: u64,
    pub reward_amounts: [u64; 3],
    pub transfer_fee0: u64,
    pub transfer_fee1: u64,
}

#[derive(Clone, Debug, PartialEq)]
pub struct DecreaseLiquidityEventEvent(pub DecreaseLiquidityEvent);

impl BorshSerialize for DecreaseLiquidityEventEvent {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        DECREASE_LIQUIDITY_EVENT_DISCM.serialize(writer)?;
        self.0.serialize(writer)
    }
}

impl DecreaseLiquidityEventEvent {
    pub fn deserialize(buf: &mut &[u8]) -> std::io::Result<Self> {
        let maybe_discm = <[u8; 8]>::deserialize(buf)?;
        if maybe_discm != DECREASE_LIQUIDITY_EVENT_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    DECREASE_LIQUIDITY_EVENT_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(DecreaseLiquidityEvent::deserialize(buf)?))
    }
}

// LiquidityCalculateEvent
pub const LIQUIDITY_CALCULATE_EVENT_DISCM: [u8; 8] = [121, 163, 205, 201, 57, 218, 117, 60];

#[derive(Clone, Debug, PartialEq, BorshDeserialize, BorshSerialize)]
pub struct LiquidityCalculateEvent {
    pub pool_liquidity: u128,
    pub pool_sqrt_price_x64: u128,
    pub pool_tick: i32,
    pub calc_amount0: u64,
    pub calc_amount1: u64,
    pub trade_fee_owed0: u64,
    pub trade_fee_owed1: u64,
    pub transfer_fee0: u64,
    pub transfer_fee1: u64,
}

#[derive(Clone, Debug, PartialEq)]
pub struct LiquidityCalculateEventEvent(pub LiquidityCalculateEvent);

impl BorshSerialize for LiquidityCalculateEventEvent {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        LIQUIDITY_CALCULATE_EVENT_DISCM.serialize(writer)?;
        self.0.serialize(writer)
    }
}

impl LiquidityCalculateEventEvent {
    pub fn deserialize(buf: &mut &[u8]) -> std::io::Result<Self> {
        let maybe_discm = <[u8; 8]>::deserialize(buf)?;
        if maybe_discm != LIQUIDITY_CALCULATE_EVENT_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    LIQUIDITY_CALCULATE_EVENT_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(LiquidityCalculateEvent::deserialize(buf)?))
    }
}

// CollectPersonalFeeEvent
pub const COLLECT_PERSONAL_FEE_EVENT_DISCM: [u8; 8] = [121, 163, 205, 201, 57, 218, 117, 60];

#[derive(Clone, Debug, PartialEq, BorshDeserialize, BorshSerialize)]
pub struct CollectPersonalFeeEvent {
    pub position_nft_mint: Pubkey,
    pub recipient_token_account0: Pubkey,
    pub recipient_token_account1: Pubkey,
    pub amount0: u64,
    pub amount1: u64,
}

#[derive(Clone, Debug, PartialEq)]
pub struct CollectPersonalFeeEventEvent(pub CollectPersonalFeeEvent);

impl BorshSerialize for CollectPersonalFeeEventEvent {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        COLLECT_PERSONAL_FEE_EVENT_DISCM.serialize(writer)?;
        self.0.serialize(writer)
    }
}

impl CollectPersonalFeeEventEvent {
    pub fn deserialize(buf: &mut &[u8]) -> std::io::Result<Self> {
        let maybe_discm = <[u8; 8]>::deserialize(buf)?;
        if maybe_discm != COLLECT_PERSONAL_FEE_EVENT_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    COLLECT_PERSONAL_FEE_EVENT_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(CollectPersonalFeeEvent::deserialize(buf)?))
    }
}

// UpdateRewardInfosEvent
pub const UPDATE_REWARD_INFOS_EVENT_DISCM: [u8; 8] = [121, 163, 205, 201, 57, 218, 117, 60];
#[derive(Clone, Debug, PartialEq, BorshDeserialize, BorshSerialize)]
pub struct UpdateRewardInfosEvent {
    pub reward_growth_global_x64: [u128; 3],
}

#[derive(Clone, Debug, PartialEq)]
pub struct UpdateRewardInfosEventEvent(pub UpdateRewardInfosEvent);

impl BorshSerialize for UpdateRewardInfosEventEvent {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        UPDATE_REWARD_INFOS_EVENT_DISCM.serialize(writer)?;
        self.0.serialize(writer)
    }
}

impl UpdateRewardInfosEventEvent {
    pub fn deserialize(buf: &mut &[u8]) -> std::io::Result<Self> {
        let maybe_discm = <[u8; 8]>::deserialize(buf)?;
        if maybe_discm != UPDATE_REWARD_INFOS_EVENT_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    UPDATE_REWARD_INFOS_EVENT_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(UpdateRewardInfosEvent::deserialize(buf)?))
    }
}

// PoolCreatedEvent
pub const POOL_CREATED_EVENT_DISCM: [u8; 8] = [121, 163, 205, 201, 57, 218, 117, 60];

#[derive(Clone, Debug, PartialEq, BorshDeserialize, BorshSerialize)]
pub struct PoolCreatedEvent {
    pub token_mint0: Pubkey,
    pub token_mint1: Pubkey,
    pub tick_spacing: u16,
    pub pool_state: Pubkey,
    pub sqrt_price_x64: u128,
    pub tick: i32,
    pub token_vault0: Pubkey,
    pub token_vault1: Pubkey,
}

#[derive(Clone, Debug, PartialEq)]
pub struct PoolCreatedEventEvent(pub PoolCreatedEvent);

impl BorshSerialize for PoolCreatedEventEvent {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        POOL_CREATED_EVENT_DISCM.serialize(writer)?;
        self.0.serialize(writer)
    }
}

impl PoolCreatedEventEvent {
    pub fn deserialize(buf: &mut &[u8]) -> std::io::Result<Self> {
        let maybe_discm = <[u8; 8]>::deserialize(buf)?;
        if maybe_discm != POOL_CREATED_EVENT_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    POOL_CREATED_EVENT_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(PoolCreatedEvent::deserialize(buf)?))
    }
}

// CollectProtocolFeeEvent
pub const COLLECT_PROTOCOL_FEE_EVENT_DISCM: [u8; 8] = [121, 163, 205, 201, 57, 218, 117, 60];

#[derive(Clone, Debug, PartialEq, BorshDeserialize, BorshSerialize)]
pub struct CollectProtocolFeeEvent {
    pub pool_state: Pubkey,
    pub recipient_token_account0: Pubkey,
    pub recipient_token_account1: Pubkey,
    pub amount0: u64,
    pub amount1: u64,
}

#[derive(Clone, Debug, PartialEq)]
pub struct CollectProtocolFeeEventEvent(pub CollectProtocolFeeEvent);

impl BorshSerialize for CollectProtocolFeeEventEvent {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        COLLECT_PROTOCOL_FEE_EVENT_DISCM.serialize(writer)?;
        self.0.serialize(writer)
    }
}

impl CollectProtocolFeeEventEvent {
    pub fn deserialize(buf: &mut &[u8]) -> std::io::Result<Self> {
        let maybe_discm = <[u8; 8]>::deserialize(buf)?;
        if maybe_discm != COLLECT_PROTOCOL_FEE_EVENT_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    COLLECT_PROTOCOL_FEE_EVENT_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(CollectProtocolFeeEvent::deserialize(buf)?))
    }
}

// SwapEvent (already provided in the example, included for completeness)
pub const SWAP_EVENT_DISCM: [u8; 8] = [64, 198, 205, 232, 38, 8, 113, 226];

#[derive(Clone, Debug, PartialEq, BorshDeserialize, BorshSerialize)]
pub struct SwapEvent {
    pub pool_id: Pubkey,
    pub input_vault_before: u64,
    pub output_vault_before: u64,
    pub input_amount: u64,
    pub output_amount: u64,
    pub input_transfer_fee: u64,
    pub output_transfer_fee: u64,
    pub base_input: bool,
}

#[derive(Clone, Debug, PartialEq)]
pub struct SwapEventEvent(pub SwapEvent);

impl BorshSerialize for SwapEventEvent {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        SWAP_EVENT_DISCM.serialize(writer)?;
        self.0.serialize(writer)
    }
}

impl SwapEventEvent {
    pub fn deserialize(buf: &mut &[u8]) -> std::io::Result<Self> {
        let maybe_discm = <[u8; 8]>::deserialize(buf)?;
        if maybe_discm != SWAP_EVENT_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    SWAP_EVENT_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(SwapEvent::deserialize(buf)?))
    }
}

// LiquidityChangeEvent
pub const LIQUIDITY_CHANGE_EVENT_DISCM: [u8; 8] = [121, 163, 205, 201, 57, 218, 117, 60];

#[derive(Clone, Debug, PartialEq, BorshDeserialize, BorshSerialize)]
pub struct LiquidityChangeEvent {
    pub pool_state: Pubkey,
    pub tick: i32,
    pub tick_lower: i32,
    pub tick_upper: i32,
    pub liquidity_before: u128,
    pub liquidity_after: u128,
}

#[derive(Clone, Debug, PartialEq)]
pub struct LiquidityChangeEventEvent(pub LiquidityChangeEvent);

impl BorshSerialize for LiquidityChangeEventEvent {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        LIQUIDITY_CHANGE_EVENT_DISCM.serialize(writer)?;
        self.0.serialize(writer)
    }
}

impl LiquidityChangeEventEvent {
    pub fn deserialize(buf: &mut &[u8]) -> std::io::Result<Self> {
        let maybe_discm = <[u8; 8]>::deserialize(buf)?;
        if maybe_discm != LIQUIDITY_CHANGE_EVENT_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    LIQUIDITY_CHANGE_EVENT_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(LiquidityChangeEvent::deserialize(buf)?))
    }
}