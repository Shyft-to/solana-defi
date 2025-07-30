use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::pubkey::Pubkey;
//use serde::Serialize;


pub const CREATE_EVENT_EVENT_DISCM: [u8; 8] = [27, 114, 169, 77, 222, 235, 99, 118];
#[derive(Clone, Debug, PartialEq, BorshDeserialize, BorshSerialize, serde::Serialize)]
pub struct CreateEvent {
    pub name: String,
    pub symbol: String,
    pub uri: String,
    pub mint: Pubkey,
    pub bonding_curve: Pubkey,
    pub user: Pubkey,
    pub creator: Pubkey,
    pub timestamp: i64,
    pub virtual_token_reserves: u64,
    pub virtual_sol_reserves: u64,
    pub real_token_reserves: u64,
    pub token_total_supply: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct CreateEventEvent(pub CreateEvent);
impl BorshSerialize for CreateEventEvent {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        CREATE_EVENT_EVENT_DISCM.serialize(writer)?;
        self.0.serialize(writer)
    }
}
impl CreateEventEvent {
    pub fn deserialize(buf: &mut &[u8]) -> std::io::Result<Self> {
        let maybe_discm = <[u8; 8]>::deserialize(buf)?;
        if maybe_discm != CREATE_EVENT_EVENT_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    CREATE_EVENT_EVENT_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(CreateEvent::deserialize(buf)?))
    }
}

pub const TRADE_EVENT_EVENT_DISCM: [u8; 8] = [189, 219, 127, 211, 78, 230, 97, 238];
#[derive(Clone, Debug, PartialEq, BorshDeserialize, BorshSerialize, serde::Serialize)]
pub struct TradeEvent {
    pub mint: Pubkey,
    pub sol_amount: u64,
    pub token_amount: u64,
    pub is_buy: bool,
    pub user: Pubkey,
    pub timestamp: i64,
    pub virtual_sol_reserves: u64,
    pub virtual_token_reserves: u64,
    pub real_sol_reserves: u64,
    pub real_token_reserves: u64,
    pub fee_recipient: Pubkey,
    pub fee_basis_points: u64,
    pub fee: u64,
    pub creator: Pubkey,
    pub creator_fee_basis_points: u64,
    pub creator_fee: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct TradeEventEvent(pub TradeEvent);
impl BorshSerialize for TradeEventEvent {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        TRADE_EVENT_EVENT_DISCM.serialize(writer)?;
        self.0.serialize(writer)
    }
}
impl TradeEventEvent {
    pub fn deserialize(buf: &mut &[u8]) -> std::io::Result<Self> {
        let maybe_discm = <[u8; 8]>::deserialize(buf)?;
        if maybe_discm != TRADE_EVENT_EVENT_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    TRADE_EVENT_EVENT_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(TradeEvent::deserialize(buf)?))
    }
}

pub const COMPLETE_EVENT_EVENT_DISCM: [u8; 8] = [95, 114, 97, 156, 212, 46, 152, 8];
#[derive(Clone, Debug, PartialEq, BorshDeserialize, BorshSerialize, serde::Serialize)]
pub struct CompleteEvent {
    pub user: Pubkey,
    pub mint: Pubkey,
    pub bonding_curve: Pubkey,
    pub timestamp: i64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct CompleteEventEvent(pub CompleteEvent);
impl BorshSerialize for CompleteEventEvent {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        COMPLETE_EVENT_EVENT_DISCM.serialize(writer)?;
        self.0.serialize(writer)
    }
}
impl CompleteEventEvent {
    pub fn deserialize(buf: &mut &[u8]) -> std::io::Result<Self> {
        let maybe_discm = <[u8; 8]>::deserialize(buf)?;
        if maybe_discm != COMPLETE_EVENT_EVENT_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    COMPLETE_EVENT_EVENT_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(CompleteEvent::deserialize(buf)?))
    }
}

pub const SET_PARAMS_EVENT_EVENT_DISCM: [u8; 8] = [223, 195, 159, 246, 62, 48, 143, 131];
#[derive(Clone, Debug, PartialEq, BorshDeserialize, BorshSerialize, serde::Serialize)]
pub struct SetParamsEvent {
    pub fee_recipient: Pubkey,
    pub initial_virtual_token_reserves: u64,
    pub initial_virtual_sol_reserves: u64,
    pub initial_real_token_reserves: u64,
    pub final_real_sol_reserves: u64,
    pub token_total_supply: u64,
    pub fee_basis_points: u64,
    pub withdraw_authority: Pubkey,
    pub enable_migrate: bool,
    pub pool_migration_fee: u64,
    pub creator_fee_basis_points: u64,
    pub fee_recipients: [Pubkey; 8],
    pub timestamp: i64,
    pub set_creator_authority: Pubkey,
}
#[derive(Clone, Debug, PartialEq)]
pub struct SetParamsEventEvent(pub SetParamsEvent);
impl BorshSerialize for SetParamsEventEvent {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        SET_PARAMS_EVENT_EVENT_DISCM.serialize(writer)?;
        self.0.serialize(writer)
    }
}
impl SetParamsEventEvent {
    pub fn deserialize(buf: &mut &[u8]) -> std::io::Result<Self> {
        let maybe_discm = <[u8; 8]>::deserialize(buf)?;
        if maybe_discm != SET_PARAMS_EVENT_EVENT_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    SET_PARAMS_EVENT_EVENT_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(SetParamsEvent::deserialize(buf)?))
    }
}

pub const COLLECT_CREATOR_FEE_EVENT_DISCM: [u8; 8] = [122, 2, 127, 1, 14, 191, 12, 175];
#[derive(Clone, Debug, PartialEq, BorshDeserialize, BorshSerialize, serde::Serialize)]
pub struct CollectCreatorFeeEvent {
    pub timestamp: i64,
    pub creator: Pubkey,
    pub creator_fee: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct CollectCreatorFeeEventEvent(pub CollectCreatorFeeEvent);
impl BorshSerialize for CollectCreatorFeeEventEvent {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        COLLECT_CREATOR_FEE_EVENT_DISCM.serialize(writer)?;
        self.0.serialize(writer)
    }
}
impl CollectCreatorFeeEventEvent {
    pub fn deserialize(buf: &mut &[u8]) -> std::io::Result<Self> {
        let maybe_discm = <[u8; 8]>::deserialize(buf)?;
        if maybe_discm != COLLECT_CREATOR_FEE_EVENT_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    COLLECT_CREATOR_FEE_EVENT_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(CollectCreatorFeeEvent::deserialize(buf)?))
    }
}

pub const COMPLETE_PUMP_AMM_MIGRATION_EVENT_DISCM: [u8; 8] = [189, 233, 93, 185, 92, 148, 234, 148];
#[derive(Clone, Debug, PartialEq, BorshDeserialize, BorshSerialize, serde::Serialize)]
pub struct CompletePumpAmmMigrationEvent {
    pub user: Pubkey,
    pub mint: Pubkey,
    pub mint_amount: u64,
    pub sol_amount: u64,
    pub pool_migration_fee: u64,
    pub bonding_curve: Pubkey,
    pub timestamp: i64,
    pub pool: Pubkey,
}
#[derive(Clone, Debug, PartialEq)]
pub struct CompletePumpAmmMigrationEventEvent(pub CompletePumpAmmMigrationEvent);
impl BorshSerialize for CompletePumpAmmMigrationEventEvent {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        COMPLETE_PUMP_AMM_MIGRATION_EVENT_DISCM.serialize(writer)?;
        self.0.serialize(writer)
    }
}
impl CompletePumpAmmMigrationEventEvent {
    pub fn deserialize(buf: &mut &[u8]) -> std::io::Result<Self> {
        let maybe_discm = <[u8; 8]>::deserialize(buf)?;
        if maybe_discm != COMPLETE_PUMP_AMM_MIGRATION_EVENT_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    COMPLETE_PUMP_AMM_MIGRATION_EVENT_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(CompletePumpAmmMigrationEvent::deserialize(buf)?))
    }
}

pub const EXTEND_ACCOUNT_EVENT_DISCM: [u8; 8] = [97, 97, 215, 144, 93, 146, 22, 124];
#[derive(Clone, Debug, PartialEq, BorshDeserialize, BorshSerialize, serde::Serialize)]
pub struct ExtendAccountEvent {
    pub account: Pubkey,
    pub user: Pubkey,
    pub current_size: u64,
    pub new_size: u64,
    pub timestamp: i64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct ExtendAccountEventEvent(pub ExtendAccountEvent);
impl BorshSerialize for ExtendAccountEventEvent {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        EXTEND_ACCOUNT_EVENT_DISCM.serialize(writer)?;
        self.0.serialize(writer)
    }
}
impl ExtendAccountEventEvent {
    pub fn deserialize(buf: &mut &[u8]) -> std::io::Result<Self> {
        let maybe_discm = <[u8; 8]>::deserialize(buf)?;
        if maybe_discm != EXTEND_ACCOUNT_EVENT_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    EXTEND_ACCOUNT_EVENT_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(ExtendAccountEvent::deserialize(buf)?))
    }
}

pub const SET_CREATOR_EVENT_DISCM: [u8; 8] = [237, 52, 123, 37, 245, 251, 72, 210];
#[derive(Clone, Debug, PartialEq, BorshDeserialize, BorshSerialize, serde::Serialize)]
pub struct SetCreatorEvent {
    pub timestamp: i64,
    pub mint: Pubkey,
    pub bonding_curve: Pubkey,
    pub creator: Pubkey,
}
#[derive(Clone, Debug, PartialEq)]
pub struct SetCreatorEventEvent(pub SetCreatorEvent);
impl BorshSerialize for SetCreatorEventEvent {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        SET_CREATOR_EVENT_DISCM.serialize(writer)?;
        self.0.serialize(writer)
    }
}
impl SetCreatorEventEvent {
    pub fn deserialize(buf: &mut &[u8]) -> std::io::Result<Self> {
        let maybe_discm = <[u8; 8]>::deserialize(buf)?;
        if maybe_discm != SET_CREATOR_EVENT_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    SET_CREATOR_EVENT_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(SetCreatorEvent::deserialize(buf)?))
    }
}

pub const SET_METAPLEX_CREATOR_EVENT_DISCM: [u8; 8] = [142, 203, 6, 32, 127, 105, 191, 162];
#[derive(Clone, Debug, PartialEq, BorshDeserialize, BorshSerialize, serde::Serialize)]
pub struct SetMetaplexCreatorEvent {
    pub timestamp: i64,
    pub mint: Pubkey,
    pub bonding_curve: Pubkey,
    pub metadata: Pubkey,
    pub creator: Pubkey,
}
#[derive(Clone, Debug, PartialEq)]
pub struct SetMetaplexCreatorEventEvent(pub SetMetaplexCreatorEvent);
impl BorshSerialize for SetMetaplexCreatorEventEvent {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        SET_METAPLEX_CREATOR_EVENT_DISCM.serialize(writer)?;
        self.0.serialize(writer)
    }
}
impl SetMetaplexCreatorEventEvent {
    pub fn deserialize(buf: &mut &[u8]) -> std::io::Result<Self> {
        let maybe_discm = <[u8; 8]>::deserialize(buf)?;
        if maybe_discm != SET_METAPLEX_CREATOR_EVENT_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    SET_METAPLEX_CREATOR_EVENT_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(SetMetaplexCreatorEvent::deserialize(buf)?))
    }
}

pub const UPDATE_GLOBAL_AUTHORITY_EVENT_DISCM: [u8; 8] = [182, 195, 137, 42, 35, 206, 207, 247];
#[derive(Clone, Debug, PartialEq, BorshDeserialize, BorshSerialize, serde::Serialize)]
pub struct UpdateGlobalAuthorityEvent {
    pub global: Pubkey,
    pub authority: Pubkey,
    pub new_authority: Pubkey,
    pub timestamp: i64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct UpdateGlobalAuthorityEventEvent(pub UpdateGlobalAuthorityEvent);
impl BorshSerialize for UpdateGlobalAuthorityEventEvent {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        UPDATE_GLOBAL_AUTHORITY_EVENT_DISCM.serialize(writer)?;
        self.0.serialize(writer)
    }
}
impl UpdateGlobalAuthorityEventEvent {
    pub fn deserialize(buf: &mut &[u8]) -> std::io::Result<Self> {
        let maybe_discm = <[u8; 8]>::deserialize(buf)?;
        if maybe_discm != UPDATE_GLOBAL_AUTHORITY_EVENT_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    UPDATE_GLOBAL_AUTHORITY_EVENT_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(UpdateGlobalAuthorityEvent::deserialize(buf)?))
    }
}