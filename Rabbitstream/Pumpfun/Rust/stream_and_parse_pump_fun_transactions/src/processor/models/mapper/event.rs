use base64::engine::general_purpose;
use base64::Engine;
use serde::Serialize; 
use pump_interface::events::{
    CreateEvent, CreateEventEvent, CREATE_EVENT_EVENT_DISCM,
    TradeEvent, TradeEventEvent, TRADE_EVENT_EVENT_DISCM,
    CompleteEvent, CompleteEventEvent, COMPLETE_EVENT_EVENT_DISCM,
    SetParamsEvent, SetParamsEventEvent, SET_PARAMS_EVENT_EVENT_DISCM,
    CollectCreatorFeeEvent, CollectCreatorFeeEventEvent, COLLECT_CREATOR_FEE_EVENT_DISCM,
    CompletePumpAmmMigrationEvent, CompletePumpAmmMigrationEventEvent, COMPLETE_PUMP_AMM_MIGRATION_EVENT_DISCM,
    ExtendAccountEvent, ExtendAccountEventEvent, EXTEND_ACCOUNT_EVENT_DISCM,
    SetCreatorEvent, SetCreatorEventEvent, SET_CREATOR_EVENT_DISCM,
    SetMetaplexCreatorEvent, SetMetaplexCreatorEventEvent, SET_METAPLEX_CREATOR_EVENT_DISCM,
    UpdateGlobalAuthorityEvent, UpdateGlobalAuthorityEventEvent, UPDATE_GLOBAL_AUTHORITY_EVENT_DISCM,
    InitUserVolumeAccumulatorEvent,InitUserVolumeAccumulatorEventEvent,INIT_USER_VOLUME_ACCUMULATOR_EVENT_DISCM,
 };

#[derive(Debug, Clone, Serialize,PartialEq)]
pub enum DecodedEvent {
    CreateEvent(CreateEvent),
    TradeEvent(TradeEvent),
    CompleteEvent(CompleteEvent),
    SetParamsEvent(SetParamsEvent),
    CollectCreatorFeeEvent(CollectCreatorFeeEvent),
    CompletePumpAmmMigrationEvent(CompletePumpAmmMigrationEvent),
    ExtendAccountEvent(ExtendAccountEvent),
    SetCreatorEvent(SetCreatorEvent),
    SetMetaplexCreatorEvent(SetMetaplexCreatorEvent),
    UpdateGlobalAuthorityEvent(UpdateGlobalAuthorityEvent),
    InitUserVolumeAccumulatorEvent(InitUserVolumeAccumulatorEvent),
}

#[derive(Debug)]
pub struct AccountEventError {
    pub message: String,
}

pub fn convert_to_discm(base64_string: &str) -> Result<Vec<u8>, base64::DecodeError> {
    general_purpose::STANDARD.decode(base64_string)
}

pub fn extract_log_message(logs: &[String]) -> Option<String> {
    logs.iter()
        .find_map(|message| {
            if message.starts_with("Program data: ") {
                let encoded = message.trim_start_matches("Program data: ").trim();
                Some(encoded.to_string())
            } else {
                None
            }
        })
}
pub fn decode_event_data(buf: &[u8]) -> Result<DecodedEvent, AccountEventError> {
    if buf.len() < 8 {
        return Err(AccountEventError {
            message: "Buffer too short to contain a valid discriminator.".to_string(),
        });
    }

    let discriminator: [u8; 8] = buf[..8].try_into().expect("Failed to extract first 8 bytes");

  match discriminator {    
    CREATE_EVENT_EVENT_DISCM => {
        let data = CreateEventEvent::deserialize(&mut &buf[..]).map_err(|e| AccountEventError {
            message: format!("Failed to deserialize CreateEvent: {}", e),
        })?;
        Ok(DecodedEvent::CreateEvent(data.0))
    }
    TRADE_EVENT_EVENT_DISCM => {
        let data = TradeEventEvent::deserialize(&mut &buf[..]).map_err(|e| AccountEventError {
            message: format!("Failed to deserialize TradeEvent: {}", e),
        })?;
        Ok(DecodedEvent::TradeEvent(data.0))
    }
    COMPLETE_EVENT_EVENT_DISCM => {
        let data = CompleteEventEvent::deserialize(&mut &buf[..]).map_err(|e| AccountEventError {
            message: format!("Failed to deserialize CompleteEvent: {}", e),
        })?;
        Ok(DecodedEvent::CompleteEvent(data.0))
    }
    SET_PARAMS_EVENT_EVENT_DISCM => {
        let data = SetParamsEventEvent::deserialize(&mut &buf[..]).map_err(|e| AccountEventError {
            message: format!("Failed to deserialize SetParamsEvent: {}", e),
        })?;
        Ok(DecodedEvent::SetParamsEvent(data.0))
    }
    COLLECT_CREATOR_FEE_EVENT_DISCM => {
        let data = CollectCreatorFeeEventEvent::deserialize(&mut &buf[..]).map_err(|e| AccountEventError {
            message: format!("Failed to deserialize CollectCreatorFeeEvent: {}", e),
        })?;
        Ok(DecodedEvent::CollectCreatorFeeEvent(data.0))
    }
    COMPLETE_PUMP_AMM_MIGRATION_EVENT_DISCM => {
        let data = CompletePumpAmmMigrationEventEvent::deserialize(&mut &buf[..]).map_err(|e| AccountEventError {
            message: format!("Failed to deserialize CompletePumpAmmMigrationEvent: {}", e),
        })?;
        Ok(DecodedEvent::CompletePumpAmmMigrationEvent(data.0))
    }
    EXTEND_ACCOUNT_EVENT_DISCM => {
        let data = ExtendAccountEventEvent::deserialize(&mut &buf[..]).map_err(|e| AccountEventError {
            message: format!("Failed to deserialize ExtendAccountEvent: {}", e),
        })?;
        Ok(DecodedEvent::ExtendAccountEvent(data.0))
    }
    SET_CREATOR_EVENT_DISCM => {
        let data = SetCreatorEventEvent::deserialize(&mut &buf[..]).map_err(|e| AccountEventError {
            message: format!("Failed to deserialize SetCreatorEvent: {}", e),
        })?;
        Ok(DecodedEvent::SetCreatorEvent(data.0))
    }
    INIT_USER_VOLUME_ACCUMULATOR_EVENT_DISCM => {
        let data = InitUserVolumeAccumulatorEventEvent::deserialize(&mut &buf[..]).map_err(|e| AccountEventError {
            message: format!("Failed to deserialize InitUserVolumeAccumulatorEvent: {}", e),
        })?;
        Ok(DecodedEvent::InitUserVolumeAccumulatorEvent(data.0))
    }
    SET_METAPLEX_CREATOR_EVENT_DISCM => {
        let data = SetMetaplexCreatorEventEvent::deserialize(&mut &buf[..]).map_err(|e| AccountEventError {
            message: format!("Failed to deserialize SetMetaplexCreatorEvent: {}", e),
        })?;
        Ok(DecodedEvent::SetMetaplexCreatorEvent(data.0))
    }
    UPDATE_GLOBAL_AUTHORITY_EVENT_DISCM => {
        let data = UpdateGlobalAuthorityEventEvent::deserialize(&mut &buf[..]).map_err(|e| AccountEventError {
            message: format!("Failed to deserialize UpdateGlobalAuthorityEvent: {}", e),
        })?;
        Ok(DecodedEvent::UpdateGlobalAuthorityEvent(data.0))
    }
    _ => Err(AccountEventError {
        message: "Account discriminator not found.".to_string(),
    }),
    }
}