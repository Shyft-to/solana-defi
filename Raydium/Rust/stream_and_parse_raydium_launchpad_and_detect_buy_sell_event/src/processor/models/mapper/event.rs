use base64::engine::general_purpose;
use base64::Engine;
use serde::Serialize; 
use solana_program::pubkey::Pubkey;
use solana_transaction_status::InnerInstructions;

use raydium_launchpad_interface::events::{
    ClaimVestedEvent, ClaimVestedEventEvent, CLAIM_VESTED_EVENT_DISCM,
    CreateVestingEvent, CreateVestingEventEvent, CREATE_VESTING_EVENT_DISCM,
    PoolCreateEvent, PoolCreateEventEvent, POOL_CREATE_EVENT_DISCM,
    TradeEvent, TradeEventEvent, TRADE_EVENT_DISCM,
};

#[derive(Debug, Clone, Serialize, PartialEq)]
pub enum DecodedEvent {
    ClaimVestedEvent(ClaimVestedEvent),
    CreateVestingEvent(CreateVestingEvent),
    PoolCreateEvent(PoolCreateEvent),
    TradeEvent(TradeEvent),
}

#[derive(Debug)]
pub struct AccountEventError {
    pub message: String,
}

pub fn convert_to_discm(base64_string: &str) -> Result<Vec<u8>, base64::DecodeError> {
    general_purpose::STANDARD.decode(base64_string)
}

pub fn extract_inner_data(inner_instructions: &[InnerInstructions]) -> Vec<Vec<u8>> {
    let mut all_data: Vec<Vec<u8>> = Vec::new();

    for inner in inner_instructions {
        for inner_inst in &inner.instructions {
            let data = &inner_inst.instruction.data;
            all_data.push(data.clone());
        }
    }
    all_data
}


pub fn decode_event_data(buf: &[u8]) -> Result<DecodedEvent, AccountEventError> {
    if buf.len() < 8 {
        return Err(AccountEventError {
            message: "Buffer too short to contain a valid discriminator.".to_string(),
        });
    }

    let mut buf = buf; 
    let discm: [u8; 8] = buf[..8].try_into().unwrap();

    match discm {
        CLAIM_VESTED_EVENT_DISCM => {
            let data = ClaimVestedEventEvent::deserialize(&mut buf)
                .map_err(|e| AccountEventError {
                    message: format!("Failed to deserialize ClaimVestedEvent: {}", e),
                })?;
            Ok(DecodedEvent::ClaimVestedEvent(data.0))
        }
        CREATE_VESTING_EVENT_DISCM => {
            let data = CreateVestingEventEvent::deserialize(&mut buf)
                .map_err(|e| AccountEventError {
                    message: format!("Failed to deserialize CreateVestingEvent: {}", e),
                })?;
            Ok(DecodedEvent::CreateVestingEvent(data.0))
        }
        POOL_CREATE_EVENT_DISCM => {
            let data = PoolCreateEventEvent::deserialize(&mut buf)
                .map_err(|e| AccountEventError {
                    message: format!("Failed to deserialize PoolCreateEvent: {}", e),
                })?;
            Ok(DecodedEvent::PoolCreateEvent(data.0))
        }
        TRADE_EVENT_DISCM => {
            let data = TradeEventEvent::deserialize(&mut buf)
                .map_err(|e| AccountEventError {
                    message: format!("Failed to deserialize TradeEvent: {}", e),
                })?;
            Ok(DecodedEvent::TradeEvent(data.0))
        }
        _ => Err(AccountEventError {
            message: "Unknown discriminator.".to_string(),
        }),
    }
}
