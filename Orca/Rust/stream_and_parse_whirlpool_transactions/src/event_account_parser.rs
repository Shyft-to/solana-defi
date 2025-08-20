use base64::engine::general_purpose;
use base64::Engine;
use serde::Serialize; 
use whirlpool_interface::events::{
    LiquidityDecreasedEvent, LiquidityDecreasedEventEvent, LIQUIDITY_DECREASED_EVENT_DISCM,
    LiquidityIncreasedEvent, LiquidityIncreasedEventEvent, LIQUIDITY_INCREASED_EVENT_DISCM,
    PoolInitializedEvent, PoolInitializedEventEvent, POOL_INITIALIZED_EVENT_DISCM,
    TradedEvent, TradedEventEvent, TRADED_EVENT_DISCM,
 };

#[derive(Debug, Clone, Serialize,PartialEq)]
pub enum DecodedEvent {
    LiquidityDecreasedEvent(LiquidityDecreasedEvent),
    LiquidityIncreasedEvent(LiquidityIncreasedEvent),
    PoolInitializedEvent(PoolInitializedEvent),
    TradedEvent(TradedEvent),
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
    LIQUIDITY_DECREASED_EVENT_DISCM => {
        let data = LiquidityDecreasedEventEvent::deserialize(&mut &buf[..]).map_err(|e| AccountEventError {
            message: format!("Failed to deserialize CreateEvent: {}", e),
        })?;
        Ok(DecodedEvent::LiquidityDecreasedEvent(data.0))
    }
    LIQUIDITY_INCREASED_EVENT_DISCM => {
        let data = LiquidityIncreasedEventEvent::deserialize(&mut &buf[..]).map_err(|e| AccountEventError {
            message: format!("Failed to deserialize TradeEvent: {}", e),
        })?;
        Ok(DecodedEvent::LiquidityIncreasedEvent(data.0))
    }
    POOL_INITIALIZED_EVENT_DISCM => {
        let data = PoolInitializedEventEvent::deserialize(&mut &buf[..]).map_err(|e| AccountEventError {
            message: format!("Failed to deserialize CompleteEvent: {}", e),
        })?;
        Ok(DecodedEvent::PoolInitializedEvent(data.0))
    }
    TRADED_EVENT_DISCM => {
        let data = TradedEventEvent::deserialize(&mut &buf[..]).map_err(|e| AccountEventError {
            message: format!("Failed to deserialize SetParamsEvent: {}", e),
        })?;
        Ok(DecodedEvent::TradedEvent(data.0))
    }
    _ => Err(AccountEventError {
        message: "Account discriminator not found.".to_string(),
    }),
    }
}