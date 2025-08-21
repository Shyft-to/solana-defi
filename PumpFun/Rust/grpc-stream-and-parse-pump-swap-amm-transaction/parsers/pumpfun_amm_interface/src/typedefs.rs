use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::pubkey::Pubkey;
use std::io;

pub const BUY_EVENT_EVENT_DISCM: [u8; 8] = [103, 244, 82, 31, 44, 245, 119, 119];
#[derive(Clone, Debug, BorshDeserialize, BorshSerialize, PartialEq)]
pub struct BuyEvent {
    pub timestamp: i64,
    pub base_amount_out: u64,
    pub max_quote_amount_in: u64,
    pub user_base_token_reserves: u64,
    pub user_quote_token_reserves: u64,
    pub pool_base_token_reserves: u64,
    pub pool_quote_token_reserves: u64,
    pub quote_amount_in: u64,
    pub lp_fee_basis_points: u64,
    pub lp_fee: u64,
    pub protocol_fee_basis_points: u64,
    pub protocol_fee: u64,
    pub quote_amount_in_with_lp_fee: u64,
    pub user_quote_amount_in: u64,
    pub pool: Pubkey,
    pub user: Pubkey,
    pub user_base_token_account: Pubkey,
    pub user_quote_token_account: Pubkey,
    pub protocol_fee_recipient: Pubkey,
    pub protocol_fee_recipient_token_account: Pubkey,
}

#[derive(Clone, Debug, PartialEq)]
pub struct BuyEventEvent(pub BuyEvent);
impl BorshSerialize for BuyEventEvent {
    fn serialize<W: io::Write>(&self, writer: &mut W) -> io::Result<()> {
        BUY_EVENT_EVENT_DISCM.serialize(writer)?;
        self.0.serialize(writer)
    }
}

impl BuyEventEvent {
    pub fn deserialize(buf: &mut &[u8]) -> io::Result<Self> {
        let maybe_discm = <[u8; 8]>::deserialize(buf)?;
        if maybe_discm != BUY_EVENT_EVENT_DISCM {
            return Err(io::Error::new(
                io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    BUY_EVENT_EVENT_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(BuyEvent::deserialize(buf)?))
    }
}

// Similarly implement other events

pub const CREATE_CONFIG_EVENT_EVENT_DISCM: [u8; 8] = [107, 52, 89, 129, 55, 226, 81, 22];
#[derive(Clone, Debug, BorshDeserialize, BorshSerialize, PartialEq)]
pub struct CreateConfigEvent {
    pub timestamp: i64,
    pub admin: Pubkey,
    pub lp_fee_basis_points: u64,
    pub protocol_fee_basis_points: u64,
    pub protocol_fee_recipients: [Pubkey; 8],
}

#[derive(Clone, Debug, PartialEq)]
pub struct CreateConfigEventEvent(pub CreateConfigEvent);
impl BorshSerialize for CreateConfigEventEvent {
    fn serialize<W: io::Write>(&self, writer: &mut W) -> io::Result<()> {
        CREATE_CONFIG_EVENT_EVENT_DISCM.serialize(writer)?;
        self.0.serialize(writer)
    }
}

impl CreateConfigEventEvent {
    pub fn deserialize(buf: &mut &[u8]) -> io::Result<Self> {
        let maybe_discm = <[u8; 8]>::deserialize(buf)?;
        if maybe_discm != CREATE_CONFIG_EVENT_EVENT_DISCM {
            return Err(io::Error::new(
                io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    CREATE_CONFIG_EVENT_EVENT_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(CreateConfigEvent::deserialize(buf)?))
    }
}

pub const CREATE_POOL_EVENT_EVENT_DISCM: [u8; 8] = [177, 49, 12, 210, 160, 118, 167, 116];
#[derive(Clone, Debug, BorshDeserialize, BorshSerialize, PartialEq)]
pub struct CreatePoolEvent {
    pub timestamp: i64,
    pub index: u16,
    pub creator: Pubkey,
    pub base_mint: Pubkey,
    pub quote_mint: Pubkey,
    pub base_mint_decimals: u8,
    pub quote_mint_decimals: u8,
    pub base_amount_in: u64,
    pub quote_amount_in: u64,
    pub pool_base_amount: u64,
    pub pool_quote_amount: u64,
    pub minimum_liquidity: u64,
    pub initial_liquidity: u64,
    pub lp_token_amount_out: u64,
    pub pool_bump: u8,
    pub pool: Pubkey,
    pub lp_mint: Pubkey,
    pub user_base_token_account: Pubkey,
    pub user_quote_token_account: Pubkey,
}

#[derive(Clone, Debug, PartialEq)]
pub struct CreatePoolEventEvent(pub CreatePoolEvent);
impl BorshSerialize for CreatePoolEventEvent {
    fn serialize<W: io::Write>(&self, writer: &mut W) -> io::Result<()> {
        CREATE_POOL_EVENT_EVENT_DISCM.serialize(writer)?;
        self.0.serialize(writer)
    }
}

impl CreatePoolEventEvent {
    pub fn deserialize(buf: &mut &[u8]) -> io::Result<Self> {
        let maybe_discm = <[u8; 8]>::deserialize(buf)?;
        if maybe_discm != CREATE_POOL_EVENT_EVENT_DISCM {
            return Err(io::Error::new(
                io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    CREATE_POOL_EVENT_EVENT_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(CreatePoolEvent::deserialize(buf)?))
    }
}

pub const DEPOSIT_EVENT_EVENT_DISCM: [u8; 8] = [108, 141, 99, 12, 192, 39, 54, 98];
#[derive(Clone, Debug, BorshDeserialize, BorshSerialize, PartialEq)]
pub struct DepositEvent {
    pub timestamp: i64,
    pub lp_token_amount_out: u64,
    pub max_base_amount_in: u64,
    pub max_quote_amount_in: u64,
    pub user_base_token_reserves: u64,
    pub user_quote_token_reserves: u64,
    pub pool_base_token_reserves: u64,
    pub pool_quote_token_reserves: u64,
    pub base_amount_in: u64,
    pub quote_amount_in: u64,
    pub lp_mint_supply: u64,
    pub pool: Pubkey,
    pub user: Pubkey,
    pub user_base_token_account: Pubkey,
    pub user_quote_token_account: Pubkey,
    pub user_pool_token_account: Pubkey,
}

#[derive(Clone, Debug, PartialEq)]
pub struct DepositEventEvent(pub DepositEvent);
impl BorshSerialize for DepositEventEvent {
    fn serialize<W: io::Write>(&self, writer: &mut W) -> io::Result<()> {
        DEPOSIT_EVENT_EVENT_DISCM.serialize(writer)?;
        self.0.serialize(writer)
    }
}

impl DepositEventEvent {
    pub fn deserialize(buf: &mut &[u8]) -> io::Result<Self> {
        let maybe_discm = <[u8; 8]>::deserialize(buf)?;
        if maybe_discm != DEPOSIT_EVENT_EVENT_DISCM {
            return Err(io::Error::new(
                io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    DEPOSIT_EVENT_EVENT_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(DepositEvent::deserialize(buf)?))
    }
}
pub const DISABLE_EVENT_EVENT_DISCM: [u8; 8] = [94, 15, 111, 205, 22, 177, 32, 43];
#[derive(Clone, Debug, BorshDeserialize, BorshSerialize, PartialEq)]
pub struct DisableEvent {
    pub timestamp: i64,
    pub admin: Pubkey,
    pub disable_create_pool: bool,
    pub disable_deposit: bool,
    pub disable_withdraw: bool,
    pub disable_buy: bool,
    pub disable_sell: bool,
}

#[derive(Clone, Debug, PartialEq)]
pub struct DisableEventEvent(pub DisableEvent);
impl BorshSerialize for DisableEventEvent {
    fn serialize<W: io::Write>(&self, writer: &mut W) -> io::Result<()> {
        DISABLE_EVENT_EVENT_DISCM.serialize(writer)?;
        self.0.serialize(writer)
    }
}

impl DisableEventEvent {
    pub fn deserialize(buf: &mut &[u8]) -> io::Result<Self> {
        let maybe_discm = <[u8; 8]>::deserialize(buf)?;
        if maybe_discm != DISABLE_EVENT_EVENT_DISCM {
            return Err(io::Error::new(
                io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    DISABLE_EVENT_EVENT_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(DisableEvent::deserialize(buf)?))
    }
}
pub const EXTEND_ACCOUNT_EVENT_EVENT_DISCM: [u8; 8] = [129, 89, 52, 211, 75, 186, 47, 163];
#[derive(Clone, Debug, BorshDeserialize, BorshSerialize, PartialEq)]
pub struct ExtendAccountEvent {
    pub timestamp: i64,
    pub account: Pubkey,
    pub user: Pubkey,
    pub current_size: u64,
    pub new_size: u64,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ExtendAccountEventEvent(pub ExtendAccountEvent);
impl BorshSerialize for ExtendAccountEventEvent {
    fn serialize<W: io::Write>(&self, writer: &mut W) -> io::Result<()> {
        EXTEND_ACCOUNT_EVENT_EVENT_DISCM.serialize(writer)?;
        self.0.serialize(writer)
    }
}

impl ExtendAccountEventEvent {
    pub fn deserialize(buf: &mut &[u8]) -> io::Result<Self> {
        let maybe_discm = <[u8; 8]>::deserialize(buf)?;
        if maybe_discm != EXTEND_ACCOUNT_EVENT_EVENT_DISCM {
            return Err(io::Error::new(
                io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    EXTEND_ACCOUNT_EVENT_EVENT_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(ExtendAccountEvent::deserialize(buf)?))
    }
}
pub const GLOBAL_CONFIG_EVENT_DISCM: [u8; 8] = [222, 198, 121, 44, 88, 11, 234, 144];
#[derive(Clone, Debug, BorshDeserialize, BorshSerialize, PartialEq)]
pub struct GlobalConfig {
    pub admin: Pubkey,
    pub lp_fee_basis_points: u64,
    pub protocol_fee_basis_points: u64,
    pub disable_flags: u8,
    pub protocol_fee_recipients: [Pubkey; 8],
}

#[derive(Clone, Debug, PartialEq)]
pub struct GlobalConfigEvent(pub GlobalConfig);
impl BorshSerialize for GlobalConfigEvent {
    fn serialize<W: io::Write>(&self, writer: &mut W) -> io::Result<()> {
        GLOBAL_CONFIG_EVENT_DISCM.serialize(writer)?;
        self.0.serialize(writer)
    }
}

impl GlobalConfigEvent {
    pub fn deserialize(buf: &mut &[u8]) -> io::Result<Self> {
        let maybe_discm = <[u8; 8]>::deserialize(buf)?;
        if maybe_discm != GLOBAL_CONFIG_EVENT_DISCM {
            return Err(io::Error::new(
                io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    GLOBAL_CONFIG_EVENT_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(GlobalConfig::deserialize(buf)?))
    }
}
pub const POOL_EVENT_DISCM: [u8; 8] = [59, 118, 182, 16, 24, 213, 88, 149];
#[derive(Clone, Debug, BorshDeserialize, BorshSerialize, PartialEq)]
pub struct Pool {
    pub pool_bump: u8,
    pub index: u16,
    pub creator: Pubkey,
    pub base_mint: Pubkey,
    pub quote_mint: Pubkey,
    pub lp_mint: Pubkey,
    pub pool_base_token_account: Pubkey,
    pub pool_quote_token_account: Pubkey,
    pub lp_supply: u64,
}

#[derive(Clone, Debug, PartialEq)]
pub struct PoolEvent(pub Pool);
impl BorshSerialize for PoolEvent {
    fn serialize<W: io::Write>(&self, writer: &mut W) -> io::Result<()> {
        POOL_EVENT_DISCM.serialize(writer)?;
        self.0.serialize(writer)
    }
}

impl PoolEvent {
    pub fn deserialize(buf: &mut &[u8]) -> io::Result<Self> {
        let maybe_discm = <[u8; 8]>::deserialize(buf)?;
        if maybe_discm != POOL_EVENT_DISCM {
            return Err(io::Error::new(
                io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    POOL_EVENT_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(Pool::deserialize(buf)?))
    }
}
pub const SELL_EVENT_EVENT_DISCM: [u8; 8] = [215, 30, 54, 10, 90, 37, 12, 179];
#[derive(Clone, Debug, BorshDeserialize, BorshSerialize, PartialEq)]
pub struct SellEvent {
    pub timestamp: i64,
    pub base_amount_in: u64,
    pub min_quote_amount_out: u64,
    pub user_base_token_reserves: u64,
    pub user_quote_token_reserves: u64,
    pub pool_base_token_reserves: u64,
    pub pool_quote_token_reserves: u64,
    pub quote_amount_out: u64,
    pub lp_fee_basis_points: u64,
    pub lp_fee: u64,
    pub protocol_fee_basis_points: u64,
    pub protocol_fee: u64,
    pub quote_amount_out_without_lp_fee: u64,
    pub user_quote_amount_out: u64,
    pub pool: Pubkey,
    pub user: Pubkey,
    pub user_base_token_account: Pubkey,
    pub user_quote_token_account: Pubkey,
    pub protocol_fee_recipient: Pubkey,
    pub protocol_fee_recipient_token_account: Pubkey,
}

#[derive(Clone, Debug, PartialEq)]
pub struct SellEventEvent(pub SellEvent);
impl BorshSerialize for SellEventEvent {
    fn serialize<W: io::Write>(&self, writer: &mut W) -> io::Result<()> {
        SELL_EVENT_EVENT_DISCM.serialize(writer)?;
        self.0.serialize(writer)
    }
}

impl SellEventEvent {
    pub fn deserialize(buf: &mut &[u8]) -> io::Result<Self> {
        let maybe_discm = <[u8; 8]>::deserialize(buf)?;
        if maybe_discm != SELL_EVENT_EVENT_DISCM {
            return Err(io::Error::new(
                io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    SELL_EVENT_EVENT_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(SellEvent::deserialize(buf)?))
    }
}
