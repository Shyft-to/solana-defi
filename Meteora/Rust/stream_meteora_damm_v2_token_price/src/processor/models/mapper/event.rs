
use base64::engine::general_purpose;
use base64::Engine;
use serde::Serialize; 
use solana_program::pubkey::Pubkey;
use solana_transaction_status::InnerInstructions;

use meteora_damm_interface::events::{
    EvtAddLiquidity, EvtAddLiquidityEvent, EVT_ADD_LIQUIDITY_DISCM,
    EvtClaimPartnerFee, EvtClaimPartnerFeeEvent, EVT_CLAIM_PARTNER_FEE_DISCM,
    EvtClaimPositionFee, EvtClaimPositionFeeEvent, EVT_CLAIM_POSITION_FEE_DISCM,
    EvtClaimProtocolFee, EvtClaimProtocolFeeEvent, EVT_CLAIM_PROTOCOL_FEE_DISCM,
    EvtClaimReward, EvtClaimRewardEvent, EVT_CLAIM_REWARD_DISCM,
    EvtCloseClaimFeeOperator, EvtCloseClaimFeeOperatorEvent, EVT_CLOSE_CLAIM_FEE_OPERATOR_DISCM,
    EvtCloseConfig, EvtCloseConfigEvent, EVT_CLOSE_CONFIG_DISCM,
    EvtClosePosition, EvtClosePositionEvent, EVT_CLOSE_POSITION_DISCM,
    EvtCreateClaimFeeOperator, EvtCreateClaimFeeOperatorEvent, EVT_CREATE_CLAIM_FEE_OPERATOR_DISCM,
    EvtCreateConfig, EvtCreateConfigEvent, EVT_CREATE_CONFIG_DISCM,
    EvtCreateDynamicConfig, EvtCreateDynamicConfigEvent, EVT_CREATE_DYNAMIC_CONFIG_DISCM,
    EvtCreatePosition, EvtCreatePositionEvent, EVT_CREATE_POSITION_DISCM,
    EvtCreateTokenBadge, EvtCreateTokenBadgeEvent, EVT_CREATE_TOKEN_BADGE_DISCM,
    EvtFundReward, EvtFundRewardEvent, EVT_FUND_REWARD_DISCM,
    EvtInitializePool, EvtInitializePoolEvent, EVT_INITIALIZE_POOL_DISCM,
    EvtInitializeReward, EvtInitializeRewardEvent, EVT_INITIALIZE_REWARD_DISCM,
    EvtLockPosition, EvtLockPositionEvent, EVT_LOCK_POSITION_DISCM,
    EvtPermanentLockPosition, EvtPermanentLockPositionEvent, EVT_PERMANENT_LOCK_POSITION_DISCM,
    EvtRemoveLiquidity, EvtRemoveLiquidityEvent, EVT_REMOVE_LIQUIDITY_DISCM,
    EvtSetPoolStatus, EvtSetPoolStatusEvent, EVT_SET_POOL_STATUS_DISCM,
    EvtSwap, EvtSwapEvent, EVT_SWAP_DISCM,
    EvtUpdateRewardDuration, EvtUpdateRewardDurationEvent, EVT_UPDATE_REWARD_DURATION_DISCM,
    EvtUpdateRewardFunder, EvtUpdateRewardFunderEvent, EVT_UPDATE_REWARD_FUNDER_DISCM,
    EvtWithdrawIneligibleReward, EvtWithdrawIneligibleRewardEvent, EVT_WITHDRAW_INELIGIBLE_REWARD_DISCM,
};


#[derive(Debug, Clone, Serialize, PartialEq)]
pub enum DecodedEvent {
    EvtAddLiquidity(EvtAddLiquidity),
    EvtClaimPartnerFee(EvtClaimPartnerFee),
    EvtClaimPositionFee(EvtClaimPositionFee),
    EvtClaimProtocolFee(EvtClaimProtocolFee),
    EvtClaimReward(EvtClaimReward),
    EvtCloseClaimFeeOperator(EvtCloseClaimFeeOperator),
    EvtCloseConfig(EvtCloseConfig),
    EvtClosePosition(EvtClosePosition),
    EvtCreateClaimFeeOperator(EvtCreateClaimFeeOperator),
    EvtCreateConfig(EvtCreateConfig),
    EvtCreateDynamicConfig(EvtCreateDynamicConfig),
    EvtCreatePosition(EvtCreatePosition),
    EvtCreateTokenBadge(EvtCreateTokenBadge),
    EvtFundReward(EvtFundReward),
    EvtInitializePool(EvtInitializePool),
    EvtInitializeReward(EvtInitializeReward),
    EvtLockPosition(EvtLockPosition),
    EvtPermanentLockPosition(EvtPermanentLockPosition),
    EvtRemoveLiquidity(EvtRemoveLiquidity),
    EvtSetPoolStatus(EvtSetPoolStatus),
    Swap(EvtSwap),    
    EvtUpdateRewardDuration(EvtUpdateRewardDuration),
    EvtUpdateRewardFunder(EvtUpdateRewardFunder),
    EvtWithdrawIneligibleReward(EvtWithdrawIneligibleReward),
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
        EVT_ADD_LIQUIDITY_DISCM => {
            let data = EvtAddLiquidityEvent::deserialize(&mut buf)
                .map_err(|e| AccountEventError {
                    message: format!("Failed to deserialize EvtAddLiquidityEvent: {}", e),
                })?;
            Ok(DecodedEvent::EvtAddLiquidity(data.0))
        }
        EVT_CLAIM_PARTNER_FEE_DISCM => {
            let data = EvtClaimPartnerFeeEvent::deserialize(&mut buf)
                .map_err(|e| AccountEventError {
                    message: format!("Failed to deserialize EvtClaimPartnerFeeEvent: {}", e),
                })?;
            Ok(DecodedEvent::EvtClaimPartnerFee(data.0))
        }
        EVT_CLAIM_POSITION_FEE_DISCM => {
            let data = EvtClaimPositionFeeEvent::deserialize(&mut buf)
                .map_err(|e| AccountEventError {
                    message: format!("Failed to deserialize EvtClaimPositionFeeEvent: {}", e),
                })?;
            Ok(DecodedEvent::EvtClaimPositionFee(data.0))
        }
        EVT_CLAIM_PROTOCOL_FEE_DISCM => {
            let data = EvtClaimProtocolFeeEvent::deserialize(&mut buf)
                .map_err(|e| AccountEventError {
                    message: format!("Failed to deserialize EvtClaimProtocolFeeEvent: {}", e),
                })?;
            Ok(DecodedEvent::EvtClaimProtocolFee(data.0))
        }
        EVT_CLAIM_REWARD_DISCM => {
            let data = EvtClaimRewardEvent::deserialize(&mut buf)
                .map_err(|e| AccountEventError {
                    message: format!("Failed to deserialize EvtClaimRewardEvent: {}", e),
                })?;
            Ok(DecodedEvent::EvtClaimReward(data.0))
        }
        EVT_CLOSE_CLAIM_FEE_OPERATOR_DISCM => {
            let data = EvtCloseClaimFeeOperatorEvent::deserialize(&mut buf)
                .map_err(|e| AccountEventError {
                    message: format!("Failed to deserialize EvtCloseClaimFeeOperatorEvent: {}", e),
                })?;
            Ok(DecodedEvent::EvtCloseClaimFeeOperator(data.0))
        }
        EVT_CLOSE_CONFIG_DISCM => {
            let data = EvtCloseConfigEvent::deserialize(&mut buf)
                .map_err(|e| AccountEventError {
                    message: format!("Failed to deserialize EvtCloseConfigEvent: {}", e),
                })?;
            Ok(DecodedEvent::EvtCloseConfig(data.0))
        }
        EVT_CLOSE_POSITION_DISCM => {
            let data = EvtClosePositionEvent::deserialize(&mut buf)
                .map_err(|e| AccountEventError {
                    message: format!("Failed to deserialize EvtClosePositionEvent: {}", e),
                })?;
            Ok(DecodedEvent::EvtClosePosition(data.0))
        }
        EVT_CREATE_CLAIM_FEE_OPERATOR_DISCM => {
            let data = EvtCreateClaimFeeOperatorEvent::deserialize(&mut buf)
                .map_err(|e| AccountEventError {
                    message: format!("Failed to deserialize EvtCreateClaimFeeOperatorEvent: {}", e),
                })?;
            Ok(DecodedEvent::EvtCreateClaimFeeOperator(data.0))
        }
        EVT_CREATE_CONFIG_DISCM => {
            let data = EvtCreateConfigEvent::deserialize(&mut buf)
                .map_err(|e| AccountEventError {
                    message: format!("Failed to deserialize EvtCreateConfigEvent: {}", e),
                })?;
            Ok(DecodedEvent::EvtCreateConfig(data.0))
        }
        EVT_CREATE_DYNAMIC_CONFIG_DISCM => {
            let data = EvtCreateDynamicConfigEvent::deserialize(&mut buf)
                .map_err(|e| AccountEventError {
                    message: format!("Failed to deserialize EvtCreateDynamicConfigEvent: {}", e),
                })?;
            Ok(DecodedEvent::EvtCreateDynamicConfig(data.0))
        }
        EVT_CREATE_POSITION_DISCM => {
            let data = EvtCreatePositionEvent::deserialize(&mut buf)
                .map_err(|e| AccountEventError {
                    message: format!("Failed to deserialize EvtCreatePositionEvent: {}", e),
                })?;
            Ok(DecodedEvent::EvtCreatePosition(data.0))
        }
        EVT_CREATE_TOKEN_BADGE_DISCM => {
            let data = EvtCreateTokenBadgeEvent::deserialize(&mut buf)
                .map_err(|e| AccountEventError {
                    message: format!("Failed to deserialize EvtCreateTokenBadgeEvent: {}", e),
                })?;
            Ok(DecodedEvent::EvtCreateTokenBadge(data.0))
        }
        EVT_FUND_REWARD_DISCM => {
            let data = EvtFundRewardEvent::deserialize(&mut buf)
                .map_err(|e| AccountEventError {
                    message: format!("Failed to deserialize EvtFundRewardEvent: {}", e),
                })?;
            Ok(DecodedEvent::EvtFundReward(data.0))
        }
        EVT_INITIALIZE_POOL_DISCM => {
            let data = EvtInitializePoolEvent::deserialize(&mut buf)
                .map_err(|e| AccountEventError {
                    message: format!("Failed to deserialize EvtInitializePoolEvent: {}", e),
                })?;
            Ok(DecodedEvent::EvtInitializePool(data.0))
        }
        EVT_INITIALIZE_REWARD_DISCM => {
            let data = EvtInitializeRewardEvent::deserialize(&mut buf)
                .map_err(|e| AccountEventError {
                    message: format!("Failed to deserialize EvtInitializeRewardEvent: {}", e),
                })?;
            Ok(DecodedEvent::EvtInitializeReward(data.0))
        }
        EVT_LOCK_POSITION_DISCM => {
            let data = EvtLockPositionEvent::deserialize(&mut buf)
                .map_err(|e| AccountEventError {
                    message: format!("Failed to deserialize EvtLockPositionEvent: {}", e),
                })?;
            Ok(DecodedEvent::EvtLockPosition(data.0))
        }
        EVT_PERMANENT_LOCK_POSITION_DISCM => {
            let data = EvtPermanentLockPositionEvent::deserialize(&mut buf)
                .map_err(|e| AccountEventError {
                    message: format!("Failed to deserialize EvtPermanentLockPositionEvent: {}", e),
                })?;
            Ok(DecodedEvent::EvtPermanentLockPosition(data.0))
        }
        EVT_REMOVE_LIQUIDITY_DISCM => {
            let data = EvtRemoveLiquidityEvent::deserialize(&mut buf)
                .map_err(|e| AccountEventError {
                    message: format!("Failed to deserialize EvtRemoveLiquidityEvent: {}", e),
                })?;
            Ok(DecodedEvent::EvtRemoveLiquidity(data.0))
        }
        EVT_SET_POOL_STATUS_DISCM => {
            let data = EvtSetPoolStatusEvent::deserialize(&mut buf)
                .map_err(|e| AccountEventError {
                    message: format!("Failed to deserialize EvtSetPoolStatusEvent: {}", e),
                })?;
            Ok(DecodedEvent::EvtSetPoolStatus(data.0))
        }
        EVT_SWAP_DISCM => {
            let data = EvtSwapEvent::deserialize(&mut buf)
                .map_err(|e| AccountEventError {
                    message: format!("Failed to deserialize EvtSwapEvent: {}", e),
                })?;
            Ok(DecodedEvent::Swap(data.0))
        }
        EVT_UPDATE_REWARD_DURATION_DISCM => {
            let data = EvtUpdateRewardDurationEvent::deserialize(&mut buf)
                .map_err(|e| AccountEventError {
                    message: format!("Failed to deserialize EvtUpdateRewardDurationEvent: {}", e),
                })?;
            Ok(DecodedEvent::EvtUpdateRewardDuration(data.0))
        }
        EVT_UPDATE_REWARD_FUNDER_DISCM => {
            let data = EvtUpdateRewardFunderEvent::deserialize(&mut buf)
                .map_err(|e| AccountEventError {
                    message: format!("Failed to deserialize EvtUpdateRewardFunderEvent: {}", e),
                })?;
            Ok(DecodedEvent::EvtUpdateRewardFunder(data.0))
        }
        EVT_WITHDRAW_INELIGIBLE_REWARD_DISCM => {
            let data = EvtWithdrawIneligibleRewardEvent::deserialize(&mut buf)
                .map_err(|e| AccountEventError {
                    message: format!("Failed to deserialize EvtWithdrawIneligibleRewardEvent: {}", e),
                })?;
            Ok(DecodedEvent::EvtWithdrawIneligibleReward(data.0))
        }
        _ => Err(AccountEventError {
            message: "Unknown discriminator.".to_string(),
        }),
    }
}
