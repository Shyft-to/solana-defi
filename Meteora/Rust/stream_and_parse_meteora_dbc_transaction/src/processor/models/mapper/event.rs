use base64::engine::general_purpose;
use base64::Engine;
use serde::Serialize;
use solana_transaction_status::InnerInstructions;

use meteora_dbc_interface::events::{
    EvtClaimCreatorTradingFee, EvtClaimCreatorTradingFeeEvent, EVT_CLAIM_CREATOR_TRADING_FEE_DISCM,
    EvtClaimProtocolFee, EvtClaimProtocolFeeEvent, EVT_CLAIM_PROTOCOL_FEE_DISCM,
    EvtClaimTradingFee, EvtClaimTradingFeeEvent, EVT_CLAIM_TRADING_FEE_DISCM,
    EvtCloseClaimFeeOperator, EvtCloseClaimFeeOperatorEvent, EVT_CLOSE_CLAIM_FEE_OPERATOR_DISCM,
    EvtCreateClaimFeeOperator, EvtCreateClaimFeeOperatorEvent, EVT_CREATE_CLAIM_FEE_OPERATOR_DISCM,
    EvtCreateConfig, EvtCreateConfigEvent, EVT_CREATE_CONFIG_DISCM,
    EvtCreateConfigV2, EvtCreateConfigV2Event, EVT_CREATE_CONFIG_V2_DISCM,
    EvtCreateDammV2MigrationMetadata, EvtCreateDammV2MigrationMetadataEvent, EVT_CREATE_DAMM_V2_MIGRATION_METADATA_DISCM,
    EvtCreateMeteoraMigrationMetadata, EvtCreateMeteoraMigrationMetadataEvent, EVT_CREATE_METEORA_MIGRATION_METADATA_DISCM,
    EvtCreatorWithdrawSurplus, EvtCreatorWithdrawSurplusEvent, EVT_CREATOR_WITHDRAW_SURPLUS_DISCM,
    EvtCurveComplete, EvtCurveCompleteEvent, EVT_CURVE_COMPLETE_DISCM,
    EvtInitializePool, EvtInitializePoolEvent, EVT_INITIALIZE_POOL_DISCM,
    EvtPartnerMetadata, EvtPartnerMetadataEvent, EVT_PARTNER_METADATA_DISCM,
    EvtPartnerWithdrawMigrationFee, EvtPartnerWithdrawMigrationFeeEvent, EVT_PARTNER_WITHDRAW_MIGRATION_FEE_DISCM,
    EvtPartnerWithdrawSurplus, EvtPartnerWithdrawSurplusEvent, EVT_PARTNER_WITHDRAW_SURPLUS_DISCM,
    EvtProtocolWithdrawSurplus, EvtProtocolWithdrawSurplusEvent, EVT_PROTOCOL_WITHDRAW_SURPLUS_DISCM,
    EvtSwap, EvtSwapEvent, EVT_SWAP_DISCM,
    EvtSwap2, EvtSwap2Event, EVT_SWAP2_DISCM,
    EvtUpdatePoolCreator, EvtUpdatePoolCreatorEvent, EVT_UPDATE_POOL_CREATOR_DISCM,
    EvtVirtualPoolMetadata, EvtVirtualPoolMetadataEvent, EVT_VIRTUAL_POOL_METADATA_DISCM,
    EvtWithdrawLeftover, EvtWithdrawLeftoverEvent, EVT_WITHDRAW_LEFTOVER_DISCM,
    EvtWithdrawMigrationFee, EvtWithdrawMigrationFeeEvent, EVT_WITHDRAW_MIGRATION_FEE_DISCM,
};

#[derive(Debug, Clone, Serialize, PartialEq)]
pub enum DecodedEvent {
    ClaimCreatorTradingFee(EvtClaimCreatorTradingFee),
    ClaimProtocolFee(EvtClaimProtocolFee),
    ClaimTradingFee(EvtClaimTradingFee),
    CloseClaimFeeOperator(EvtCloseClaimFeeOperator),
    CreateClaimFeeOperator(EvtCreateClaimFeeOperator),
    CreateConfig(EvtCreateConfig),
    CreateConfigV2(EvtCreateConfigV2),
    CreateDammV2MigrationMetadata(EvtCreateDammV2MigrationMetadata),
    CreateMeteoraMigrationMetadata(EvtCreateMeteoraMigrationMetadata),
    CreatorWithdrawSurplus(EvtCreatorWithdrawSurplus),
    CurveComplete(EvtCurveComplete),
    InitializePool(EvtInitializePool),
    PartnerMetadata(EvtPartnerMetadata),
    PartnerWithdrawMigrationFee(EvtPartnerWithdrawMigrationFee),
    PartnerWithdrawSurplus(EvtPartnerWithdrawSurplus),
    ProtocolWithdrawSurplus(EvtProtocolWithdrawSurplus),
    Swap(EvtSwap),
    Swap2(EvtSwap2),
    UpdatePoolCreator(EvtUpdatePoolCreator),
    VirtualPoolMetadata(EvtVirtualPoolMetadata),
    WithdrawLeftover(EvtWithdrawLeftover),
    WithdrawMigrationFee(EvtWithdrawMigrationFee),
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
        EVT_CLAIM_CREATOR_TRADING_FEE_DISCM => {
            let data = EvtClaimCreatorTradingFeeEvent::deserialize(&mut buf).map_err(|e| AccountEventError {
                message: format!("Failed to deserialize ClaimCreatorTradingFeeEvent: {}", e),
            })?;
            Ok(DecodedEvent::ClaimCreatorTradingFee(data.0))
        }
        EVT_CLAIM_PROTOCOL_FEE_DISCM => {
            let data = EvtClaimProtocolFeeEvent::deserialize(&mut buf).map_err(|e| AccountEventError {
                message: format!("Failed to deserialize ClaimProtocolFeeEvent: {}", e),
            })?;
            Ok(DecodedEvent::ClaimProtocolFee(data.0))
        }
        EVT_CLAIM_TRADING_FEE_DISCM => {
            let data = EvtClaimTradingFeeEvent::deserialize(&mut buf).map_err(|e| AccountEventError {
                message: format!("Failed to deserialize ClaimTradingFeeEvent: {}", e),
            })?;
            Ok(DecodedEvent::ClaimTradingFee(data.0))
        }
        EVT_CLOSE_CLAIM_FEE_OPERATOR_DISCM => {
            let data = EvtCloseClaimFeeOperatorEvent::deserialize(&mut buf).map_err(|e| AccountEventError {
                message: format!("Failed to deserialize CloseClaimFeeOperatorEvent: {}", e),
            })?;
            Ok(DecodedEvent::CloseClaimFeeOperator(data.0))
        }
        EVT_CREATE_CLAIM_FEE_OPERATOR_DISCM => {
            let data = EvtCreateClaimFeeOperatorEvent::deserialize(&mut buf).map_err(|e| AccountEventError {
                message: format!("Failed to deserialize CreateClaimFeeOperatorEvent: {}", e),
            })?;
            Ok(DecodedEvent::CreateClaimFeeOperator(data.0))
        }
        EVT_CREATE_CONFIG_DISCM => {
            let data = EvtCreateConfigEvent::deserialize(&mut buf).map_err(|e| AccountEventError {
                message: format!("Failed to deserialize CreateConfigEvent: {}", e),
            })?;
            Ok(DecodedEvent::CreateConfig(data.0))
        }
        EVT_CREATE_CONFIG_V2_DISCM => {
            let data = EvtCreateConfigV2Event::deserialize(&mut buf).map_err(|e| AccountEventError {
                message: format!("Failed to deserialize CreateConfigV2Event: {}", e),
            })?;
            Ok(DecodedEvent::CreateConfigV2(data.0))
        }
        EVT_CREATE_DAMM_V2_MIGRATION_METADATA_DISCM => {
            let data = EvtCreateDammV2MigrationMetadataEvent::deserialize(&mut buf).map_err(|e| AccountEventError {
                message: format!("Failed to deserialize CreateDammV2MigrationMetadataEvent: {}", e),
            })?;
            Ok(DecodedEvent::CreateDammV2MigrationMetadata(data.0))
        }
        EVT_CREATE_METEORA_MIGRATION_METADATA_DISCM => {
            let data = EvtCreateMeteoraMigrationMetadataEvent::deserialize(&mut buf).map_err(|e| AccountEventError {
                message: format!("Failed to deserialize CreateMeteoraMigrationMetadataEvent: {}", e),
            })?;
            Ok(DecodedEvent::CreateMeteoraMigrationMetadata(data.0))
        }
        EVT_CREATOR_WITHDRAW_SURPLUS_DISCM => {
            let data = EvtCreatorWithdrawSurplusEvent::deserialize(&mut buf).map_err(|e| AccountEventError {
                message: format!("Failed to deserialize CreatorWithdrawSurplusEvent: {}", e),
            })?;
            Ok(DecodedEvent::CreatorWithdrawSurplus(data.0))
        }
        EVT_CURVE_COMPLETE_DISCM => {
            let data = EvtCurveCompleteEvent::deserialize(&mut buf).map_err(|e| AccountEventError {
                message: format!("Failed to deserialize CurveCompleteEvent: {}", e),
            })?;
            Ok(DecodedEvent::CurveComplete(data.0))
        }
        EVT_INITIALIZE_POOL_DISCM => {
            let data = EvtInitializePoolEvent::deserialize(&mut buf).map_err(|e| AccountEventError {
                message: format!("Failed to deserialize InitializePoolEvent: {}", e),
            })?;
            Ok(DecodedEvent::InitializePool(data.0))
        }
        EVT_PARTNER_METADATA_DISCM => {
            let data = EvtPartnerMetadataEvent::deserialize(&mut buf).map_err(|e| AccountEventError {
                message: format!("Failed to deserialize PartnerMetadataEvent: {}", e),
            })?;
            Ok(DecodedEvent::PartnerMetadata(data.0))
        }
        EVT_PARTNER_WITHDRAW_MIGRATION_FEE_DISCM => {
            let data = EvtPartnerWithdrawMigrationFeeEvent::deserialize(&mut buf).map_err(|e| AccountEventError {
                message: format!("Failed to deserialize PartnerWithdrawMigrationFeeEvent: {}", e),
            })?;
            Ok(DecodedEvent::PartnerWithdrawMigrationFee(data.0))
        }
        EVT_PARTNER_WITHDRAW_SURPLUS_DISCM => {
            let data = EvtPartnerWithdrawSurplusEvent::deserialize(&mut buf).map_err(|e| AccountEventError {
                message: format!("Failed to deserialize PartnerWithdrawSurplusEvent: {}", e),
            })?;
            Ok(DecodedEvent::PartnerWithdrawSurplus(data.0))
        }
        EVT_PROTOCOL_WITHDRAW_SURPLUS_DISCM => {
            let data = EvtProtocolWithdrawSurplusEvent::deserialize(&mut buf).map_err(|e| AccountEventError {
                message: format!("Failed to deserialize ProtocolWithdrawSurplusEvent: {}", e),
            })?;
            Ok(DecodedEvent::ProtocolWithdrawSurplus(data.0))
        }
        EVT_SWAP_DISCM => {
            let data = EvtSwapEvent::deserialize(&mut buf)
            .map_err(|e| AccountEventError {
                message: format!("Failed to deserialize SwapEvent: {}", e),
            })?;
            Ok(DecodedEvent::Swap(data.0))
        }
        EVT_SWAP2_DISCM => {
            let data = EvtSwap2Event::deserialize(&mut buf).map_err(|e| AccountEventError {
                message: format!("Failed to deserialize Swap2Event: {}", e),
            })?;
            Ok(DecodedEvent::Swap2(data.0))
        }
        EVT_UPDATE_POOL_CREATOR_DISCM => {
            let data = EvtUpdatePoolCreatorEvent::deserialize(&mut buf).map_err(|e| AccountEventError {
                message: format!("Failed to deserialize UpdatePoolCreatorEvent: {}", e),
            })?;
            Ok(DecodedEvent::UpdatePoolCreator(data.0))
        }
        EVT_VIRTUAL_POOL_METADATA_DISCM => {
            let data = EvtVirtualPoolMetadataEvent::deserialize(&mut buf).map_err(|e| AccountEventError {
                message: format!("Failed to deserialize VirtualPoolMetadataEvent: {}", e),
            })?;
            Ok(DecodedEvent::VirtualPoolMetadata(data.0))
        }
        EVT_WITHDRAW_LEFTOVER_DISCM => {
            let data = EvtWithdrawLeftoverEvent::deserialize(&mut buf).map_err(|e| AccountEventError {
                message: format!("Failed to deserialize WithdrawLeftoverEvent: {}", e),
            })?;
            Ok(DecodedEvent::WithdrawLeftover(data.0))
        }
        EVT_WITHDRAW_MIGRATION_FEE_DISCM => {
            let data = EvtWithdrawMigrationFeeEvent::deserialize(&mut buf).map_err(|e| AccountEventError {
                message: format!("Failed to deserialize WithdrawMigrationFeeEvent: {}", e),
            })?;
            Ok(DecodedEvent::WithdrawMigrationFee(data.0))
        }
        _ => Err(AccountEventError {
            message: "Account discriminator not found.".to_string(),
        }),
    }
}