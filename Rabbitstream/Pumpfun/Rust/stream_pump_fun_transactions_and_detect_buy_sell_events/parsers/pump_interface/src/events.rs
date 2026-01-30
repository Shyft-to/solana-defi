use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::pubkey::Pubkey;


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
    pub track_volume: bool,
    pub total_unclaimed_tokens: u64,
    pub total_claimed_tokens: u64,
    pub current_sol_volume: u64,
    pub last_update_timestamp: i64,
    pub ix_name: String,
    pub mayhem_mode: bool,
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
    pub admin_set_creator_authority: Pubkey,
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

pub const INIT_USER_VOLUME_ACCUMULATOR_EVENT_DISCM: [u8; 8] = [134, 36, 13, 72, 232, 101, 130, 216];
#[derive(Clone, Debug, PartialEq, BorshDeserialize, BorshSerialize, serde::Serialize)]
pub struct InitUserVolumeAccumulatorEvent {
    pub payer: Pubkey,
    pub user: Pubkey,
    pub timestamp: i64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct InitUserVolumeAccumulatorEventEvent(pub InitUserVolumeAccumulatorEvent);
impl BorshSerialize for InitUserVolumeAccumulatorEventEvent {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        INIT_USER_VOLUME_ACCUMULATOR_EVENT_DISCM.serialize(writer)?;
        self.0.serialize(writer)
    }
}
impl InitUserVolumeAccumulatorEventEvent {
    pub fn deserialize(buf: &mut &[u8]) -> std::io::Result<Self> {
        let maybe_discm = <[u8; 8]>::deserialize(buf)?;
        if maybe_discm != INIT_USER_VOLUME_ACCUMULATOR_EVENT_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    INIT_USER_VOLUME_ACCUMULATOR_EVENT_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(InitUserVolumeAccumulatorEvent::deserialize(buf)?))
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

pub const ADMIN_SET_CREATOR_EVENT_DISCM: [u8; 8] = [64, 69, 192, 104, 29, 30, 25, 107];
#[derive(Clone, Debug, PartialEq, BorshDeserialize, BorshSerialize, serde::Serialize)]
pub struct AdminSetCreatorEvent {
    pub timestamp: i64,
    pub admin_set_creator_authority: Pubkey,
    pub mint: Pubkey,
    pub bonding_curve: Pubkey,
    pub old_creator: Pubkey,
    pub new_creator: Pubkey,
}
#[derive(Clone, Debug, PartialEq)]
pub struct AdminSetCreatorEventEvent(pub AdminSetCreatorEvent);
impl BorshSerialize for AdminSetCreatorEventEvent {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        ADMIN_SET_CREATOR_EVENT_DISCM.serialize(writer)?;
        self.0.serialize(writer)
    }
}
impl AdminSetCreatorEventEvent {
    pub fn deserialize(buf: &mut &[u8]) -> std::io::Result<Self> {
        let maybe_discm = <[u8; 8]>::deserialize(buf)?;
        if maybe_discm != ADMIN_SET_CREATOR_EVENT_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    ADMIN_SET_CREATOR_EVENT_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(AdminSetCreatorEvent::deserialize(buf)?))
    }
}

pub const ADMIN_SET_IDL_AUTHORITY_EVENT_DISCM: [u8; 8] = [245, 59, 70, 34, 75, 185, 109, 92];
#[derive(Clone, Debug, PartialEq, BorshDeserialize, BorshSerialize, serde::Serialize)]
pub struct AdminSetIdlAuthorityEvent {
    pub idl_authority: Pubkey,
}
#[derive(Clone, Debug, PartialEq)]
pub struct AdminSetIdlAuthorityEventEvent(pub AdminSetIdlAuthorityEvent);
impl BorshSerialize for AdminSetIdlAuthorityEventEvent {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        ADMIN_SET_IDL_AUTHORITY_EVENT_DISCM.serialize(writer)?;
        self.0.serialize(writer)
    }
}
impl AdminSetIdlAuthorityEventEvent {
    pub fn deserialize(buf: &mut &[u8]) -> std::io::Result<Self> {
        let maybe_discm = <[u8; 8]>::deserialize(buf)?;
        if maybe_discm != ADMIN_SET_IDL_AUTHORITY_EVENT_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    ADMIN_SET_IDL_AUTHORITY_EVENT_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(AdminSetIdlAuthorityEvent::deserialize(buf)?))
    }
}

pub const ADMIN_UPDATE_TOKEN_INCENTIVES_EVENT_DISCM: [u8; 8] = [147, 250, 108, 120, 247, 29, 67, 222];
#[derive(Clone, Debug, PartialEq, BorshDeserialize, BorshSerialize, serde::Serialize)]
pub struct AdminUpdateTokenIncentivesEvent {
    pub start_time: i64,
    pub end_time: i64,
    pub day_number: u64,
    pub token_supply_per_day: u64,
    pub mint: Pubkey,
    pub seconds_in_a_day: i64,
    pub timestamp: i64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct AdminUpdateTokenIncentivesEventEvent(pub AdminUpdateTokenIncentivesEvent);
impl BorshSerialize for AdminUpdateTokenIncentivesEventEvent {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        ADMIN_UPDATE_TOKEN_INCENTIVES_EVENT_DISCM.serialize(writer)?;
        self.0.serialize(writer)
    }
}
impl AdminUpdateTokenIncentivesEventEvent {
    pub fn deserialize(buf: &mut &[u8]) -> std::io::Result<Self> {
        let maybe_discm = <[u8; 8]>::deserialize(buf)?;
        if maybe_discm != ADMIN_UPDATE_TOKEN_INCENTIVES_EVENT_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    ADMIN_UPDATE_TOKEN_INCENTIVES_EVENT_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(AdminUpdateTokenIncentivesEvent::deserialize(buf)?))
    }
}

pub const CLAIM_TOKEN_INCENTIVES_EVENT_DISCM: [u8; 8] = [79, 172, 246, 49, 205, 91, 206, 232];
#[derive(Clone, Debug, PartialEq, BorshDeserialize, BorshSerialize, serde::Serialize)]
pub struct ClaimTokenIncentivesEvent {
    pub user: Pubkey,
    pub mint: Pubkey,
    pub amount: u64,
    pub timestamp: i64,
    pub total_claimed_tokens: u64,
    pub current_sol_volume: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct ClaimTokenIncentivesEventEvent(pub ClaimTokenIncentivesEvent);
impl BorshSerialize for ClaimTokenIncentivesEventEvent {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        CLAIM_TOKEN_INCENTIVES_EVENT_DISCM.serialize(writer)?;
        self.0.serialize(writer)
    }
}
impl ClaimTokenIncentivesEventEvent {
    pub fn deserialize(buf: &mut &[u8]) -> std::io::Result<Self> {
        let maybe_discm = <[u8; 8]>::deserialize(buf)?;
        if maybe_discm != CLAIM_TOKEN_INCENTIVES_EVENT_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    CLAIM_TOKEN_INCENTIVES_EVENT_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(ClaimTokenIncentivesEvent::deserialize(buf)?))
    }
}

pub const CLOSE_USER_VOLUME_ACCUMULATOR_EVENT_DISCM: [u8; 8] = [146, 159, 189, 172, 146, 88, 56, 244];
#[derive(Clone, Debug, PartialEq, BorshDeserialize, BorshSerialize, serde::Serialize)]
pub struct CloseUserVolumeAccumulatorEvent {
    pub user: Pubkey,
    pub timestamp: i64,
    pub total_unclaimed_tokens: u64,
    pub total_claimed_tokens: u64,
    pub current_sol_volume: u64,
    pub last_update_timestamp: i64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct CloseUserVolumeAccumulatorEventEvent(pub CloseUserVolumeAccumulatorEvent);
impl BorshSerialize for CloseUserVolumeAccumulatorEventEvent {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        CLOSE_USER_VOLUME_ACCUMULATOR_EVENT_DISCM.serialize(writer)?;
        self.0.serialize(writer)
    }
}
impl CloseUserVolumeAccumulatorEventEvent {
    pub fn deserialize(buf: &mut &[u8]) -> std::io::Result<Self> {
        let maybe_discm = <[u8; 8]>::deserialize(buf)?;
        if maybe_discm != CLOSE_USER_VOLUME_ACCUMULATOR_EVENT_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    CLOSE_USER_VOLUME_ACCUMULATOR_EVENT_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(CloseUserVolumeAccumulatorEvent::deserialize(buf)?))
    }
}

pub const DISTRIBUTE_CREATOR_FEES_EVENT_DISCM: [u8; 8] = [165, 55, 129, 112, 4, 179, 202, 40];
#[derive(Clone, Debug, PartialEq, BorshDeserialize, BorshSerialize, serde::Serialize)]
pub struct DistributeCreatorFeesEvent {
    pub timestamp: i64,
    pub mint: Pubkey,
    pub bonding_curve: Pubkey,
    pub sharing_config: Pubkey,
    pub admin: Pubkey,
    pub distributed: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct DistributeCreatorFeesEventEvent(pub DistributeCreatorFeesEvent);
impl BorshSerialize for DistributeCreatorFeesEventEvent {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        DISTRIBUTE_CREATOR_FEES_EVENT_DISCM.serialize(writer)?;
        self.0.serialize(writer)
    }
}
impl DistributeCreatorFeesEventEvent {
    pub fn deserialize(buf: &mut &[u8]) -> std::io::Result<Self> {
        let maybe_discm = <[u8; 8]>::deserialize(buf)?;
        if maybe_discm != DISTRIBUTE_CREATOR_FEES_EVENT_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    DISTRIBUTE_CREATOR_FEES_EVENT_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(DistributeCreatorFeesEvent::deserialize(buf)?))
    }
}

pub const MIGRATE_BONDING_CURVE_CREATOR_EVENT_DISCM: [u8; 8] = [155, 167, 104, 220, 213, 108, 243, 3];
#[derive(Clone, Debug, PartialEq, BorshDeserialize, BorshSerialize, serde::Serialize)]
pub struct MigrateBondingCurveCreatorEvent {
    pub timestamp: i64,
    pub mint: Pubkey,
    pub bonding_curve: Pubkey,
    pub sharing_config: Pubkey,
    pub old_creator: Pubkey,
    pub new_creator: Pubkey,
}
#[derive(Clone, Debug, PartialEq)]
pub struct MigrateBondingCurveCreatorEventEvent(pub MigrateBondingCurveCreatorEvent);
impl BorshSerialize for MigrateBondingCurveCreatorEventEvent {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        MIGRATE_BONDING_CURVE_CREATOR_EVENT_DISCM.serialize(writer)?;
        self.0.serialize(writer)
    }
}
impl MigrateBondingCurveCreatorEventEvent {
    pub fn deserialize(buf: &mut &[u8]) -> std::io::Result<Self> {
        let maybe_discm = <[u8; 8]>::deserialize(buf)?;
        if maybe_discm != MIGRATE_BONDING_CURVE_CREATOR_EVENT_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    MIGRATE_BONDING_CURVE_CREATOR_EVENT_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(MigrateBondingCurveCreatorEvent::deserialize(buf)?))
    }
}

pub const MINIMUM_DISTRIBUTABLE_FEE_EVENT_DISCM: [u8; 8] = [168, 216, 132, 239, 235, 182, 49, 52];
#[derive(Clone, Debug, PartialEq, BorshDeserialize, BorshSerialize, serde::Serialize)]
pub struct MinimumDistributableFeeEvent {
    pub minimum_required: u64,
    pub distributable_fees: u64,
    pub can_distribute: bool,
}
#[derive(Clone, Debug, PartialEq)]
pub struct MinimumDistributableFeeEventEvent(pub MinimumDistributableFeeEvent);
impl BorshSerialize for MinimumDistributableFeeEventEvent {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        MINIMUM_DISTRIBUTABLE_FEE_EVENT_DISCM.serialize(writer)?;
        self.0.serialize(writer)
    }
}
impl MinimumDistributableFeeEventEvent {
    pub fn deserialize(buf: &mut &[u8]) -> std::io::Result<Self> {
        let maybe_discm = <[u8; 8]>::deserialize(buf)?;
        if maybe_discm != MINIMUM_DISTRIBUTABLE_FEE_EVENT_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    MINIMUM_DISTRIBUTABLE_FEE_EVENT_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(MinimumDistributableFeeEvent::deserialize(buf)?))
    }
}

pub const RESERVED_FEE_RECIPIENTS_EVENT_DISCM: [u8; 8] = [43, 188, 250, 18, 221, 75, 187, 95];
#[derive(Clone, Debug, PartialEq, BorshDeserialize, BorshSerialize, serde::Serialize)]
pub struct ReservedFeeRecipientsEvent {
    pub timestamp: i64,
    pub reserved_fee_recipient: Pubkey,
    pub reserved_fee_recipients: [Pubkey; 7],
}
#[derive(Clone, Debug, PartialEq)]
pub struct ReservedFeeRecipientsEventEvent(pub ReservedFeeRecipientsEvent);
impl BorshSerialize for ReservedFeeRecipientsEventEvent {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        RESERVED_FEE_RECIPIENTS_EVENT_DISCM.serialize(writer)?;
        self.0.serialize(writer)
    }
}
impl ReservedFeeRecipientsEventEvent {
    pub fn deserialize(buf: &mut &[u8]) -> std::io::Result<Self> {
        let maybe_discm = <[u8; 8]>::deserialize(buf)?;
        if maybe_discm != RESERVED_FEE_RECIPIENTS_EVENT_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    RESERVED_FEE_RECIPIENTS_EVENT_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(ReservedFeeRecipientsEvent::deserialize(buf)?))
    }
}

pub const SYNC_USER_VOLUME_ACCUMULATOR_EVENT_DISCM: [u8; 8] = [197, 122, 167, 124, 116, 81, 91, 255];
#[derive(Clone, Debug, PartialEq, BorshDeserialize, BorshSerialize, serde::Serialize)]
pub struct SyncUserVolumeAccumulatorEvent {
    pub user: Pubkey,
    pub total_claimed_tokens_before: u64,
    pub total_claimed_tokens_after: u64,
    pub timestamp: i64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct SyncUserVolumeAccumulatorEventEvent(pub SyncUserVolumeAccumulatorEvent);
impl BorshSerialize for SyncUserVolumeAccumulatorEventEvent {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        SYNC_USER_VOLUME_ACCUMULATOR_EVENT_DISCM.serialize(writer)?;
        self.0.serialize(writer)
    }
}
impl SyncUserVolumeAccumulatorEventEvent {
    pub fn deserialize(buf: &mut &[u8]) -> std::io::Result<Self> {
        let maybe_discm = <[u8; 8]>::deserialize(buf)?;
        if maybe_discm != SYNC_USER_VOLUME_ACCUMULATOR_EVENT_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    SYNC_USER_VOLUME_ACCUMULATOR_EVENT_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(SyncUserVolumeAccumulatorEvent::deserialize(buf)?))
    }
}
pub const UPDATE_MAYHEM_VIRTUAL_PARAMS_EVENT_DISCM: [u8; 8] = [117, 123, 228, 182, 161, 168, 220, 214];
#[derive(Clone, Debug, PartialEq, BorshDeserialize, BorshSerialize, serde::Serialize)]
pub struct UpdateMayhemVirtualParamsEvent {
    pub timestamp: i64,
    pub mint: Pubkey,
    pub virtual_token_reserves: u64,
    pub virtual_sol_reserves: u64,
    pub new_virtual_token_reserves: u64,
    pub new_virtual_sol_reserves: u64,
    pub real_token_reserves: u64,
    pub real_sol_reserves: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct UpdateMayhemVirtualParamsEventEvent(pub UpdateMayhemVirtualParamsEvent);
impl BorshSerialize for UpdateMayhemVirtualParamsEventEvent {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        UPDATE_MAYHEM_VIRTUAL_PARAMS_EVENT_DISCM.serialize(writer)?;
        self.0.serialize(writer)
    }
}
impl UpdateMayhemVirtualParamsEventEvent {
    pub fn deserialize(buf: &mut &[u8]) -> std::io::Result<Self> {
        let maybe_discm = <[u8; 8]>::deserialize(buf)?;
        if maybe_discm != UPDATE_MAYHEM_VIRTUAL_PARAMS_EVENT_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    UPDATE_MAYHEM_VIRTUAL_PARAMS_EVENT_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(UpdateMayhemVirtualParamsEvent::deserialize(buf)?))
    }
}