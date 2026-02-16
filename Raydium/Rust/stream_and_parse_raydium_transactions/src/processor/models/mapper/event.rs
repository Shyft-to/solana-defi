use base64::engine::general_purpose;
use base64::Engine;
use serde::Serialize;

use raydium_amm_interface::events::{
    SwapBaseInLog, SwapBaseInLogEvent,
    SwapBaseOutLog, SwapBaseOutLogEvent,
};

#[derive(Debug, Clone, Serialize, PartialEq)]
pub enum DecodedEvent {
    SwapBaseInLog(SwapBaseInLog),
    SwapBaseOutLog(SwapBaseOutLog),
}

#[derive(Debug)]
pub struct AccountEventError {
    pub message: String,
}

pub fn convert_to_discm(base64_string: &str) -> Result<Vec<u8>, base64::DecodeError> {
    general_purpose::STANDARD.decode(base64_string)
}

pub fn extract_all_log_messages(logs: &[String]) -> Vec<String> {
    logs.iter()
        .filter_map(|message| {
            message
                .strip_prefix("Program log: ray_log: ")
                .map(|s| s.trim().to_string())
        })
        .collect()
}
pub fn decode_event_data(buf: &[u8]) -> Result<DecodedEvent, AccountEventError> {
    if buf.is_empty() {
        return Err(AccountEventError {
            message: "Empty buffer".to_string(),
        });
    }

    if buf[0] == 3 {
        if let Ok(data) = SwapBaseInLogEvent::deserialize(&mut &buf[..]) {
            Ok(DecodedEvent::SwapBaseInLog(data.0))
        } else {
            Err(AccountEventError {
                message: "Failed to decode as SwapBaseInLog".to_string(),
            })
        }
    } else {
        if let Ok(data) = SwapBaseOutLogEvent::deserialize(&mut &buf[..]) {
            Ok(DecodedEvent::SwapBaseOutLog(data.0))
        } else {
            Err(AccountEventError {
                message: "Failed to decode as SwapBaseOutLog".to_string(),
            })
        }
    }
}
