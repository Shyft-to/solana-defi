use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::pubkey::Pubkey;
use std::io;
pub const BUY_EVENT_EVENT_DISCM: [u8; 8] = [103, 244, 82, 31, 44, 245, 119, 119];

#[derive(Clone, Debug, PartialEq, BorshDeserialize, BorshSerialize)]
pub struct BuyEvent {
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
pub const CREATE_CONFIG_EVENT_EVENT_DISCM: [u8; 8] = [107, 52, 89, 129, 55, 226, 81, 22];

#[derive(Clone, Debug, PartialEq, BorshDeserialize, BorshSerialize)]
pub struct CreateConfigEvent {
    pub admin: Pubkey,
    pub config: u64, // You can replace this with the actual config fields
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

#[derive(Clone, Debug, PartialEq, BorshDeserialize, BorshSerialize)]
pub struct CreatePoolEvent {
    pub creator: Pubkey,
    pub base_token: Pubkey,
    pub quote_token: Pubkey,
    pub pool_id: Pubkey,
    pub initial_base_amount: u64,
    pub initial_quote_amount: u64,
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
pub const DEPOSIT_EVENT_EVENT_DISCM: [u8; 8] = [120, 248, 61, 83, 31, 142, 107, 144];

#[derive(Clone, Debug, PartialEq, BorshDeserialize, BorshSerialize)]
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
pub const DISABLE_EVENT_EVENT_DISCM: [u8; 8] = [107, 253, 193, 76, 228, 202, 27, 104];

#[derive(Clone, Debug, PartialEq, BorshDeserialize, BorshSerialize)]
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
pub const EXTEND_ACCOUNT_EVENT_EVENT_DISCM: [u8; 8] = [97, 97, 215, 144, 93, 146, 22, 124];

#[derive(Clone, Debug, PartialEq, BorshDeserialize, BorshSerialize)]
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
pub const SELL_EVENT_EVENT_DISCM: [u8; 8] = [62, 47, 55, 10, 165, 3, 220, 42];

#[derive(Clone, Debug, PartialEq, BorshDeserialize, BorshSerialize)]
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
pub const UPDATE_ADMIN_EVENT_EVENT_DISCM: [u8; 8] = [225, 152, 171, 87, 246, 63, 66, 234];

#[derive(Clone, Debug, PartialEq, BorshDeserialize, BorshSerialize)]
pub struct UpdateAdminEvent {
    pub new_admin: Pubkey,
    pub old_admin: Pubkey,
    pub timestamp: i64,
}

#[derive(Clone, Debug, PartialEq)]
pub struct UpdateAdminEventEvent(pub UpdateAdminEvent);

impl BorshSerialize for UpdateAdminEventEvent {
    fn serialize<W: io::Write>(&self, writer: &mut W) -> io::Result<()> {
        UPDATE_ADMIN_EVENT_EVENT_DISCM.serialize(writer)?;
        self.0.serialize(writer)
    }
}

impl UpdateAdminEventEvent {
    pub fn deserialize(buf: &mut &[u8]) -> io::Result<Self> {
        let maybe_discm = <[u8; 8]>::deserialize(buf)?;
        if maybe_discm != UPDATE_ADMIN_EVENT_EVENT_DISCM {
            return Err(io::Error::new(
                io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    UPDATE_ADMIN_EVENT_EVENT_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(UpdateAdminEvent::deserialize(buf)?))
    }
}
pub const UPDATE_FEE_CONFIG_EVENT_EVENT_DISCM: [u8; 8] = [90, 23, 65, 35, 62, 244, 188, 208];

#[derive(Clone, Debug, PartialEq, BorshDeserialize, BorshSerialize)]
pub struct UpdateFeeConfigEvent {
    pub timestamp: i64,
    pub new_fee: u64,
    pub old_fee: u64,
    pub admin: Pubkey,
}

#[derive(Clone, Debug, PartialEq)]
pub struct UpdateFeeConfigEventEvent(pub UpdateFeeConfigEvent);

impl BorshSerialize for UpdateFeeConfigEventEvent {
    fn serialize<W: io::Write>(&self, writer: &mut W) -> io::Result<()> {
        UPDATE_FEE_CONFIG_EVENT_EVENT_DISCM.serialize(writer)?;
        self.0.serialize(writer)
    }
}

impl UpdateFeeConfigEventEvent {
    pub fn deserialize(buf: &mut &[u8]) -> io::Result<Self> {
        let maybe_discm = <[u8; 8]>::deserialize(buf)?;
        if maybe_discm != UPDATE_FEE_CONFIG_EVENT_EVENT_DISCM {
            return Err(io::Error::new(
                io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    UPDATE_FEE_CONFIG_EVENT_EVENT_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(UpdateFeeConfigEvent::deserialize(buf)?))
    }
}


pub const WITHDRAW_EVENT_EVENT_DISCM: [u8; 8] = [22, 9, 133, 26, 160, 44, 71, 192];

#[derive(Clone, Debug, PartialEq, BorshDeserialize, BorshSerialize)]
pub struct WithdrawEvent {
    pub pool_id: Pubkey,
    pub lp_amount_burned: u64,
    pub token0_amount: u64,
    pub token1_amount: u64,
    pub token0_transfer_fee: u64,
    pub token1_transfer_fee: u64,
}

#[derive(Clone, Debug, PartialEq)]
pub struct WithdrawEventEvent(pub WithdrawEvent);

impl BorshSerialize for WithdrawEventEvent {
    fn serialize<W: io::Write>(&self, writer: &mut W) -> io::Result<()> {
        WITHDRAW_EVENT_EVENT_DISCM.serialize(writer)?;
        self.0.serialize(writer)
    }
}

impl WithdrawEventEvent {
    pub fn deserialize(buf: &mut &[u8]) -> io::Result<Self> {
        let maybe_discm = <[u8; 8]>::deserialize(buf)?;
        if maybe_discm != WITHDRAW_EVENT_EVENT_DISCM {
            return Err(io::Error::new(
                io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    WITHDRAW_EVENT_EVENT_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(WithdrawEvent::deserialize(buf)?))
    }
}
