use base64::engine::general_purpose;
use base64::Engine;
use serde::Serialize; 
use pumpfun_amm_interface::events::{
    BuyEvent, BuyEventEvent, BUY_EVENT_EVENT_DISCM,
    CreateConfigEvent, CreateConfigEventEvent, CREATE_CONFIG_EVENT_EVENT_DISCM,
    PumpAmmCreatePoolEvent, PumpAmmCreatePoolEventEvent, CREATE_POOL_EVENT_EVENT_DISCM,
    DepositEvent, DepositEventEvent, DEPOSIT_EVENT_EVENT_DISCM,
    DisableEvent, DisableEventEvent, DISABLE_EVENT_EVENT_DISCM,
    ExtendAccountEvent, ExtendAccountEventEvent, EXTEND_ACCOUNT_EVENT_EVENT_DISCM,
    SellEvent, SellEventEvent, SELL_EVENT_EVENT_DISCM,
    UpdateAdminEvent, UpdateAdminEventEvent, UPDATE_ADMIN_EVENT_EVENT_DISCM,
    UpdateFeeConfigEvent, UpdateFeeConfigEventEvent, UPDATE_FEE_CONFIG_EVENT_EVENT_DISCM,
    WithdrawEvent, WithdrawEventEvent, WITHDRAW_EVENT_EVENT_DISCM,
 };

#[derive(Debug, Clone, Serialize,PartialEq)]
pub enum DecodedEvent {
    BuyEvent(BuyEvent),
    CreateConfigEvent(CreateConfigEvent),
    PumpAmmCreatePoolEvent(PumpAmmCreatePoolEvent),
    DepositEvent(DepositEvent),
    DisableEvent(DisableEvent),
    ExtendAccountEvent(ExtendAccountEvent),
    SellEvent(SellEvent),
    UpdateAdminEvent(UpdateAdminEvent),
    UpdateFeeConfigEvent(UpdateFeeConfigEvent),
    WithdrawEvent(WithdrawEvent),
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
    BUY_EVENT_EVENT_DISCM => {
        let data = BuyEventEvent::deserialize(&mut &buf[..]).map_err(|e| AccountEventError {
            message: format!("Failed to deserialize CreateEvent: {}", e),
        })?;
        Ok(DecodedEvent::BuyEvent(data.0))
    }
    CREATE_CONFIG_EVENT_EVENT_DISCM => {
        let data = CreateConfigEventEvent::deserialize(&mut &buf[..]).map_err(|e| AccountEventError {
            message: format!("Failed to deserialize TradeEvent: {}", e),
        })?;
        Ok(DecodedEvent::CreateConfigEvent(data.0))
    }
    CREATE_POOL_EVENT_EVENT_DISCM => {
        let data = PumpAmmCreatePoolEventEvent::deserialize(&mut &buf[..]).map_err(|e| AccountEventError {
            message: format!("Failed to deserialize CompleteEvent: {}", e),
        })?;
        Ok(DecodedEvent::PumpAmmCreatePoolEvent(data.0))
    }
    DEPOSIT_EVENT_EVENT_DISCM => {
        let data = DepositEventEvent::deserialize(&mut &buf[..]).map_err(|e| AccountEventError {
            message: format!("Failed to deserialize SetParamsEvent: {}", e),
        })?;
        Ok(DecodedEvent::DepositEvent(data.0))
    }
    DISABLE_EVENT_EVENT_DISCM => {
        let data = DisableEventEvent::deserialize(&mut &buf[..]).map_err(|e| AccountEventError {
            message: format!("Failed to deserialize CollectCreatorFeeEvent: {}", e),
        })?;
        Ok(DecodedEvent::DisableEvent(data.0))
    }
    EXTEND_ACCOUNT_EVENT_EVENT_DISCM => {
        let data = ExtendAccountEventEvent::deserialize(&mut &buf[..]).map_err(|e| AccountEventError {
            message: format!("Failed to deserialize CompletePumpAmmMigrationEvent: {}", e),
        })?;
        Ok(DecodedEvent::ExtendAccountEvent(data.0))
    }
    SELL_EVENT_EVENT_DISCM => {
        let data = SellEventEvent::deserialize(&mut &buf[..]).map_err(|e| AccountEventError {
            message: format!("Failed to deserialize SellEvent: {}", e),
        })?;
        Ok(DecodedEvent::SellEvent(data.0))
    }
    UPDATE_ADMIN_EVENT_EVENT_DISCM => {
        let data = UpdateAdminEventEvent::deserialize(&mut &buf[..]).map_err(|e| AccountEventError {
            message: format!("Failed to deserialize SetCreatorEvent: {}", e),
        })?;
        Ok(DecodedEvent::UpdateAdminEvent(data.0))
    }
    UPDATE_FEE_CONFIG_EVENT_EVENT_DISCM => {
        let data = UpdateFeeConfigEventEvent::deserialize(&mut &buf[..]).map_err(|e| AccountEventError {
            message: format!("Failed to deserialize SetMetaplexCreatorEvent: {}", e),
        })?;
        Ok(DecodedEvent::UpdateFeeConfigEvent(data.0))
    }
    WITHDRAW_EVENT_EVENT_DISCM => {
        let data = WithdrawEventEvent::deserialize(&mut &buf[..]).map_err(|e| AccountEventError {
            message: format!("Failed to deserialize UpdateGlobalAuthorityEvent: {}", e),
        })?;
        Ok(DecodedEvent::WithdrawEvent(data.0))
    }
    _ => Err(AccountEventError {
        message: "Account discriminator not found.".to_string(),
    }),
    }
}