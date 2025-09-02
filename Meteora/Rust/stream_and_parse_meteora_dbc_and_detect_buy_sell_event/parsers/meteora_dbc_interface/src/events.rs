use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::pubkey::Pubkey;
use crate::{
    PoolFeeParameters,LockedVestingParams,
    ConfigParameters,SwapResult2,
    SwapParameters,
    SwapResult,
    LiquidityDistributionParameters,
    SwapParameters2,
};

pub const EVT_CLAIM_CREATOR_TRADING_FEE_DISCM: [u8; 8] = [154, 228, 215, 202, 133, 155, 214, 138];

#[derive(Clone, Debug, BorshDeserialize, BorshSerialize, PartialEq,serde::Serialize, serde::Deserialize)]
pub struct EvtClaimCreatorTradingFee {
    pub pool: Pubkey,
    pub token_base_amount: u64,
    pub token_quote_amount: u64,
}

#[derive(Clone, Debug, PartialEq)]
pub struct EvtClaimCreatorTradingFeeEvent(pub EvtClaimCreatorTradingFee);
impl BorshSerialize for EvtClaimCreatorTradingFeeEvent {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        EVT_CLAIM_CREATOR_TRADING_FEE_DISCM.serialize(writer)?;
        self.0.serialize(writer)
    }
}
impl EvtClaimCreatorTradingFeeEvent {
    pub fn deserialize(buf: &mut &[u8]) -> std::io::Result<Self> {
        let maybe_discm = <[u8; 8]>::deserialize(buf)?;
        if maybe_discm != EVT_CLAIM_CREATOR_TRADING_FEE_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    EVT_CLAIM_CREATOR_TRADING_FEE_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(EvtClaimCreatorTradingFee::deserialize(buf)?))
    }
}

pub const EVT_CLAIM_PROTOCOL_FEE_DISCM: [u8; 8] = [186, 244, 75, 251, 188, 13, 25, 33];

#[derive(Clone, Debug, BorshDeserialize, BorshSerialize, PartialEq,serde::Serialize, serde::Deserialize)]
pub struct EvtClaimProtocolFee {
    pub pool: Pubkey,
    pub token_base_amount: u64,
    pub token_quote_amount: u64,
}

#[derive(Clone, Debug, PartialEq)]
pub struct EvtClaimProtocolFeeEvent(pub EvtClaimProtocolFee);

impl BorshSerialize for EvtClaimProtocolFeeEvent {
   fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        EVT_CLAIM_PROTOCOL_FEE_DISCM.serialize(writer)?;
        self.0.serialize(writer)
    }
}

impl EvtClaimProtocolFeeEvent {
    pub fn deserialize(buf: &mut &[u8]) -> std::io::Result<Self> {
        let maybe_discm = <[u8; 8]>::deserialize(buf)?;
        if maybe_discm != EVT_CLAIM_PROTOCOL_FEE_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    EVT_CLAIM_PROTOCOL_FEE_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(EvtClaimProtocolFee::deserialize(buf)?))
    }
}

pub const EVT_CLAIM_TRADING_FEE_DISCM: [u8; 8] = [26, 83, 117, 240, 92, 202, 112, 254];

#[derive(Clone, Debug, BorshDeserialize, BorshSerialize, PartialEq,serde::Serialize, serde::Deserialize)]
pub struct EvtClaimTradingFee {
    pub pool: Pubkey,
    pub token_base_amount: u64,
    pub token_quote_amount: u64,
}

#[derive(Clone, Debug, PartialEq)]
pub struct EvtClaimTradingFeeEvent(pub EvtClaimTradingFee);

impl BorshSerialize for EvtClaimTradingFeeEvent {
   fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        EVT_CLAIM_TRADING_FEE_DISCM.serialize(writer)?;
        self.0.serialize(writer)
    }
}

impl EvtClaimTradingFeeEvent {
    pub fn deserialize(buf: &mut &[u8]) -> std::io::Result<Self> {
        let maybe_discm = <[u8; 8]>::deserialize(buf)?;
        if maybe_discm != EVT_CLAIM_TRADING_FEE_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    EVT_CLAIM_TRADING_FEE_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(EvtClaimTradingFee::deserialize(buf)?))
    }
}

pub const EVT_CLOSE_CLAIM_FEE_OPERATOR_DISCM: [u8; 8] = [111, 39, 37, 55, 110, 216, 194, 23];

#[derive(Clone, Debug, BorshDeserialize, BorshSerialize, PartialEq,serde::Serialize, serde::Deserialize)]
pub struct EvtCloseClaimFeeOperator {
    pub claim_fee_operator: Pubkey,
    pub operator: Pubkey,
}

#[derive(Clone, Debug, PartialEq)]
pub struct EvtCloseClaimFeeOperatorEvent(pub EvtCloseClaimFeeOperator);

impl BorshSerialize for EvtCloseClaimFeeOperatorEvent {
   fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        EVT_CLOSE_CLAIM_FEE_OPERATOR_DISCM.serialize(writer)?;
        self.0.serialize(writer)
    }
}

impl EvtCloseClaimFeeOperatorEvent {
    pub fn deserialize(buf: &mut &[u8]) -> std::io::Result<Self> {
        let maybe_discm = <[u8; 8]>::deserialize(buf)?;
        if maybe_discm != EVT_CLOSE_CLAIM_FEE_OPERATOR_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    EVT_CLOSE_CLAIM_FEE_OPERATOR_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(EvtCloseClaimFeeOperator::deserialize(buf)?))
    }
}

pub const EVT_CREATE_CLAIM_FEE_OPERATOR_DISCM: [u8; 8] = [21, 6, 153, 120, 68, 116, 28, 177];

#[derive(Clone, Debug, BorshDeserialize, BorshSerialize, PartialEq,serde::Serialize, serde::Deserialize)]
pub struct EvtCreateClaimFeeOperator {
    pub operator: Pubkey,
}

#[derive(Clone, Debug, PartialEq)]
pub struct EvtCreateClaimFeeOperatorEvent(pub EvtCreateClaimFeeOperator);

impl BorshSerialize for EvtCreateClaimFeeOperatorEvent {
   fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        EVT_CREATE_CLAIM_FEE_OPERATOR_DISCM.serialize(writer)?;
        self.0.serialize(writer)
    }
}

impl EvtCreateClaimFeeOperatorEvent {
    pub fn deserialize(buf: &mut &[u8]) -> std::io::Result<Self> {
        let maybe_discm = <[u8; 8]>::deserialize(buf)?;
        if maybe_discm != EVT_CREATE_CLAIM_FEE_OPERATOR_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    EVT_CREATE_CLAIM_FEE_OPERATOR_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(EvtCreateClaimFeeOperator::deserialize(buf)?))
    }
}

pub const EVT_CREATE_CONFIG_DISCM: [u8; 8] = [131, 207, 180, 174, 180, 73, 165, 54];

#[derive(Clone, Debug, BorshDeserialize, BorshSerialize, PartialEq,serde::Serialize, serde::Deserialize)]
pub struct EvtCreateConfig {
    pub config: Pubkey,
    pub quote_mint: Pubkey,
    pub fee_claimer: Pubkey,
    pub owner: Pubkey,
    pub pool_fees: PoolFeeParameters,
    pub collect_fee_mode: u8,
    pub migration_option: u8,
    pub activation_type: u8,
    pub token_decimal: u8,
    pub token_type: u8,
    pub partner_locked_lp_percentage: u8,
    pub partner_lp_percentage: u8,
    pub creator_locked_lp_percentage: u8,
    pub creator_lp_percentage: u8,
    pub swap_base_amount: u64,
    pub migration_quote_threshold: u64,
    pub migration_base_amount: u64,
    pub sqrt_start_price: u128,
    pub locked_vesting: LockedVestingParams,
    pub migration_fee_option: u8,
    pub fixed_token_supply_flag: u8,
    pub pre_migration_token_supply: u64,
    pub post_migration_token_supply: u64,
    pub curve: Vec<LiquidityDistributionParameters>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct EvtCreateConfigEvent(pub EvtCreateConfig);

impl BorshSerialize for EvtCreateConfigEvent {
   fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        EVT_CREATE_CONFIG_DISCM.serialize(writer)?;
        self.0.serialize(writer)
    }
}

impl EvtCreateConfigEvent {
    pub fn deserialize(buf: &mut &[u8]) -> std::io::Result<Self> {
        let maybe_discm = <[u8; 8]>::deserialize(buf)?;
        if maybe_discm != EVT_CREATE_CONFIG_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    EVT_CREATE_CONFIG_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(EvtCreateConfig::deserialize(buf)?))
    }
}

pub const EVT_CREATE_CONFIG_V2_DISCM: [u8; 8] = [163, 74, 66, 187, 119, 195, 26, 144];

#[derive(Clone, Debug, BorshDeserialize, BorshSerialize, PartialEq,serde::Serialize, serde::Deserialize)]
pub struct EvtCreateConfigV2 {
    pub config: Pubkey,
    pub quote_mint: Pubkey,
    pub fee_claimer: Pubkey,
    pub leftover_receiver: Pubkey,
    pub config_parameters: ConfigParameters,
}

#[derive(Clone, Debug, PartialEq)]
pub struct EvtCreateConfigV2Event(pub EvtCreateConfigV2);

impl BorshSerialize for EvtCreateConfigV2Event {
   fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        EVT_CREATE_CONFIG_V2_DISCM.serialize(writer)?;
        self.0.serialize(writer)
    }
}

impl EvtCreateConfigV2Event {
    pub fn deserialize(buf: &mut &[u8]) -> std::io::Result<Self> {
        let maybe_discm = <[u8; 8]>::deserialize(buf)?;
        if maybe_discm != EVT_CREATE_CONFIG_V2_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    EVT_CREATE_CONFIG_V2_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(EvtCreateConfigV2::deserialize(buf)?))
    }
}

pub const EVT_CREATE_DAMM_V2_MIGRATION_METADATA_DISCM: [u8; 8] = [103, 111, 132, 168, 140, 253, 150, 114];
#[derive(Clone, Debug, BorshDeserialize, BorshSerialize, PartialEq,serde::Serialize, serde::Deserialize)]
pub struct EvtCreateDammV2MigrationMetadata {
    pub virtual_pool: Pubkey,
}

#[derive(Clone, Debug, PartialEq)]
pub struct EvtCreateDammV2MigrationMetadataEvent(pub EvtCreateDammV2MigrationMetadata);

impl BorshSerialize for EvtCreateDammV2MigrationMetadataEvent {
   fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        EVT_CREATE_DAMM_V2_MIGRATION_METADATA_DISCM.serialize(writer)?;
        self.0.serialize(writer)
    }
}

impl EvtCreateDammV2MigrationMetadataEvent {
    pub fn deserialize(buf: &mut &[u8]) -> std::io::Result<Self> {
        let maybe_discm = <[u8; 8]>::deserialize(buf)?;
        if maybe_discm != EVT_CREATE_DAMM_V2_MIGRATION_METADATA_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    EVT_CREATE_DAMM_V2_MIGRATION_METADATA_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(EvtCreateDammV2MigrationMetadata::deserialize(buf)?))
    }
}

pub const EVT_CREATE_METEORA_MIGRATION_METADATA_DISCM: [u8; 8] = [99, 167, 133, 63, 214, 143, 175, 139];
#[derive(Clone, Debug, BorshDeserialize, BorshSerialize, PartialEq,serde::Serialize, serde::Deserialize)]
pub struct EvtCreateMeteoraMigrationMetadata {
    pub virtual_pool: Pubkey,
}

#[derive(Clone, Debug, PartialEq)]
pub struct EvtCreateMeteoraMigrationMetadataEvent(pub EvtCreateMeteoraMigrationMetadata);

impl BorshSerialize for EvtCreateMeteoraMigrationMetadataEvent {
   fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        EVT_CREATE_METEORA_MIGRATION_METADATA_DISCM.serialize(writer)?;
        self.0.serialize(writer)
    }
}

impl EvtCreateMeteoraMigrationMetadataEvent {
    pub fn deserialize(buf: &mut &[u8]) -> std::io::Result<Self> {
        let maybe_discm = <[u8; 8]>::deserialize(buf)?;
        if maybe_discm != EVT_CREATE_METEORA_MIGRATION_METADATA_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    EVT_CREATE_METEORA_MIGRATION_METADATA_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(EvtCreateMeteoraMigrationMetadata::deserialize(buf)?))
    }
}

pub const EVT_CREATOR_WITHDRAW_SURPLUS_DISCM: [u8; 8] = [152, 73, 21, 15, 66, 87, 53, 157];

#[derive(Clone, Debug, BorshDeserialize, BorshSerialize, PartialEq,serde::Serialize, serde::Deserialize)]
pub struct EvtCreatorWithdrawSurplus {
    pub pool: Pubkey,
    pub surplus_amount: u64,
}

#[derive(Clone, Debug, PartialEq)]
pub struct EvtCreatorWithdrawSurplusEvent(pub EvtCreatorWithdrawSurplus);

impl BorshSerialize for EvtCreatorWithdrawSurplusEvent {
   fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        EVT_CREATOR_WITHDRAW_SURPLUS_DISCM.serialize(writer)?;
        self.0.serialize(writer)
    }
}

impl EvtCreatorWithdrawSurplusEvent {
    pub fn deserialize(buf: &mut &[u8]) -> std::io::Result<Self> {
        let maybe_discm = <[u8; 8]>::deserialize(buf)?;
        if maybe_discm != EVT_CREATOR_WITHDRAW_SURPLUS_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    EVT_CREATOR_WITHDRAW_SURPLUS_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(EvtCreatorWithdrawSurplus::deserialize(buf)?))
    }
}

pub const EVT_CURVE_COMPLETE_DISCM: [u8; 8] = [229, 231, 86, 84, 156, 134, 75, 24];

#[derive(Clone, Debug, BorshDeserialize, BorshSerialize, PartialEq,serde::Serialize, serde::Deserialize)]
pub struct EvtCurveComplete {
    pub pool: Pubkey,
    pub config: Pubkey,
    pub base_reserve: u64,
    pub quote_reserve: u64,
}

#[derive(Clone, Debug, PartialEq)]
pub struct EvtCurveCompleteEvent(pub EvtCurveComplete);

impl BorshSerialize for EvtCurveCompleteEvent {
   fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        EVT_CURVE_COMPLETE_DISCM.serialize(writer)?;
        self.0.serialize(writer)
    }
}

impl EvtCurveCompleteEvent {
    pub fn deserialize(buf: &mut &[u8]) -> std::io::Result<Self> {
        let maybe_discm = <[u8; 8]>::deserialize(buf)?;
        if maybe_discm != EVT_CURVE_COMPLETE_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    EVT_CURVE_COMPLETE_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(EvtCurveComplete::deserialize(buf)?))
    }
}

pub const EVT_INITIALIZE_POOL_DISCM: [u8; 8] = [228, 50, 246, 85, 203, 66, 134, 37];
#[derive(Clone, Debug, BorshDeserialize, BorshSerialize, PartialEq,serde::Serialize, serde::Deserialize)]
pub struct EvtInitializePool {
    pub pool: Pubkey,
    pub config: Pubkey,
    pub creator: Pubkey,
    pub base_mint: Pubkey,
    pub pool_type: u8,
    pub activation_point: u64,
}

#[derive(Clone, Debug, PartialEq)]
pub struct EvtInitializePoolEvent(pub EvtInitializePool);

impl BorshSerialize for EvtInitializePoolEvent {
   fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        EVT_INITIALIZE_POOL_DISCM.serialize(writer)?;
        self.0.serialize(writer)
    }
}

impl EvtInitializePoolEvent {
    pub fn deserialize(buf: &mut &[u8]) -> std::io::Result<Self> {
        let maybe_discm = <[u8; 8]>::deserialize(buf)?;
        if maybe_discm != EVT_INITIALIZE_POOL_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    EVT_INITIALIZE_POOL_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(EvtInitializePool::deserialize(buf)?))
    }
}

pub const EVT_PARTNER_METADATA_DISCM: [u8; 8] = [200, 127, 6, 55, 13, 32, 8, 150];

#[derive(Clone, Debug, BorshDeserialize, BorshSerialize, PartialEq,serde::Serialize, serde::Deserialize)]
pub struct EvtPartnerMetadata {
    pub partner_metadata: Pubkey,
    pub fee_claimer: Pubkey,
}

#[derive(Clone, Debug, PartialEq)]
pub struct EvtPartnerMetadataEvent(pub EvtPartnerMetadata);

impl BorshSerialize for EvtPartnerMetadataEvent {
   fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        EVT_PARTNER_METADATA_DISCM.serialize(writer)?;
        self.0.serialize(writer)
    }
}

impl EvtPartnerMetadataEvent {
    pub fn deserialize(buf: &mut &[u8]) -> std::io::Result<Self> {
        let maybe_discm = <[u8; 8]>::deserialize(buf)?;
        if maybe_discm != EVT_PARTNER_METADATA_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    EVT_PARTNER_METADATA_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(EvtPartnerMetadata::deserialize(buf)?))
    }
}

pub const EVT_PARTNER_WITHDRAW_MIGRATION_FEE_DISCM: [u8; 8] = [181, 105, 127, 67, 8, 187, 120, 57];
#[derive(Clone, Debug, BorshDeserialize, BorshSerialize, PartialEq,serde::Serialize, serde::Deserialize)]
pub struct EvtPartnerWithdrawMigrationFee {
    pub pool: Pubkey,
    pub fee: u64,
}

#[derive(Clone, Debug, PartialEq)]
pub struct EvtPartnerWithdrawMigrationFeeEvent(pub EvtPartnerWithdrawMigrationFee);

impl BorshSerialize for EvtPartnerWithdrawMigrationFeeEvent {
   fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        EVT_PARTNER_WITHDRAW_MIGRATION_FEE_DISCM.serialize(writer)?;
        self.0.serialize(writer)
    }
}

impl EvtPartnerWithdrawMigrationFeeEvent {
    pub fn deserialize(buf: &mut &[u8]) -> std::io::Result<Self> {
        let maybe_discm = <[u8; 8]>::deserialize(buf)?;
        if maybe_discm != EVT_PARTNER_WITHDRAW_MIGRATION_FEE_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    EVT_PARTNER_WITHDRAW_MIGRATION_FEE_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(EvtPartnerWithdrawMigrationFee::deserialize(buf)?))
    }
}

pub const EVT_PARTNER_WITHDRAW_SURPLUS_DISCM: [u8; 8] = [195, 56, 152, 9, 232, 72, 35, 22];

#[derive(Clone, Debug, BorshDeserialize, BorshSerialize, PartialEq,serde::Serialize, serde::Deserialize)]
pub struct EvtPartnerWithdrawSurplus {
    pub pool: Pubkey,
    pub surplus_amount: u64,
}

#[derive(Clone, Debug, PartialEq)]
pub struct EvtPartnerWithdrawSurplusEvent(pub EvtPartnerWithdrawSurplus);

impl BorshSerialize for EvtPartnerWithdrawSurplusEvent {
   fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        EVT_PARTNER_WITHDRAW_SURPLUS_DISCM.serialize(writer)?;
        self.0.serialize(writer)
    }
}

impl EvtPartnerWithdrawSurplusEvent {
    pub fn deserialize(buf: &mut &[u8]) -> std::io::Result<Self> {
        let maybe_discm = <[u8; 8]>::deserialize(buf)?;
        if maybe_discm != EVT_PARTNER_WITHDRAW_SURPLUS_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    EVT_PARTNER_WITHDRAW_SURPLUS_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(EvtPartnerWithdrawSurplus::deserialize(buf)?))
    }
}

pub const EVT_PROTOCOL_WITHDRAW_SURPLUS_DISCM: [u8; 8] = [109, 111, 28, 221, 134, 195, 230, 203];

#[derive(Clone, Debug, BorshDeserialize, BorshSerialize, PartialEq,serde::Serialize, serde::Deserialize)]
pub struct EvtProtocolWithdrawSurplus {
    pub pool: Pubkey,
    pub surplus_amount: u64,
}

#[derive(Clone, Debug, PartialEq)]
pub struct EvtProtocolWithdrawSurplusEvent(pub EvtProtocolWithdrawSurplus);

impl BorshSerialize for EvtProtocolWithdrawSurplusEvent {
   fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        EVT_PROTOCOL_WITHDRAW_SURPLUS_DISCM.serialize(writer)?;
        self.0.serialize(writer)
    }
}

impl EvtProtocolWithdrawSurplusEvent {
    pub fn deserialize(buf: &mut &[u8]) -> std::io::Result<Self> {
        let maybe_discm = <[u8; 8]>::deserialize(buf)?;
        if maybe_discm != EVT_PROTOCOL_WITHDRAW_SURPLUS_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    EVT_PROTOCOL_WITHDRAW_SURPLUS_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(EvtProtocolWithdrawSurplus::deserialize(buf)?))
    }
}

pub const EVT_SWAP_DISCM: [u8; 8] =  [228, 69, 165, 46, 81, 203, 154, 29];//[27, 60, 21, 213, 138, 170, 187, 147];

#[derive(Clone, Debug, BorshDeserialize, BorshSerialize, PartialEq,serde::Serialize, serde::Deserialize)]
pub struct EvtSwap {
    pub pool: Pubkey,
    pub config: Pubkey,
    pub trade_direction: u8,
    pub has_referral: bool,
    pub params: SwapParameters,
    pub swap_result: SwapResult,
    pub amount_in: u64,
    pub current_timestamp: u64,
}

#[derive(Clone, Debug, PartialEq)]
pub struct EvtSwapEvent(pub EvtSwap);

impl BorshSerialize for EvtSwapEvent {
   fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        EVT_SWAP_DISCM.serialize(writer)?;
        self.0.serialize(writer)
    }
}

impl EvtSwapEvent {
    pub fn deserialize(buf: &mut &[u8]) -> std::io::Result<Self> {
        let maybe_discm = <[u8; 8]>::deserialize(buf)?;
        if maybe_discm != EVT_SWAP_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    EVT_SWAP_DISCM, maybe_discm
                ),
            ));
        }
        if buf.len() < 8 {
         return Err(std::io::Error::new(std::io::ErrorKind::UnexpectedEof, "missing extra header"));
         }
        let _extra: [u8; 8] = <[u8; 8]>::deserialize(buf)?;
        Ok(Self(EvtSwap::deserialize(buf)?))
    }
}

pub const EVT_SWAP2_DISCM: [u8; 8] = [228, 69, 165, 46, 81, 203, 154, 29];//[189, 66, 51, 168, 38, 80, 117, 153];
#[derive(Clone, Debug, BorshDeserialize, BorshSerialize, PartialEq,serde::Serialize, serde::Deserialize)]
pub struct EvtSwap2 {
    pub pool: Pubkey,
    pub config: Pubkey,
    pub trade_direction: u8,
    pub has_referral: bool,
    pub swap_parameters: SwapParameters2,
    pub swap_result: SwapResult2,
    pub quote_reserve_amount: u64,
    pub migration_threshold: u64,
    pub current_timestamp: u64,
}

#[derive(Clone, Debug, PartialEq)]
pub struct EvtSwap2Event(pub EvtSwap2);

impl BorshSerialize for EvtSwap2Event {
   fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        EVT_SWAP2_DISCM.serialize(writer)?;
        self.0.serialize(writer)
    }
}

impl EvtSwap2Event {
    pub fn deserialize(buf: &mut &[u8]) -> std::io::Result<Self> {
        let maybe_discm = <[u8; 8]>::deserialize(buf)?;
        if maybe_discm != EVT_SWAP2_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    EVT_SWAP2_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(EvtSwap2::deserialize(buf)?))
    }
}

pub const EVT_UPDATE_POOL_CREATOR_DISCM: [u8; 8] = [107, 225, 165, 237, 91, 158, 213, 220];

#[derive(Clone, Debug, BorshDeserialize, BorshSerialize, PartialEq,serde::Serialize, serde::Deserialize)]
pub struct EvtUpdatePoolCreator {
    pub pool: Pubkey,
    pub creator: Pubkey,
    pub new_creator: Pubkey,
}

#[derive(Clone, Debug, PartialEq)]
pub struct EvtUpdatePoolCreatorEvent(pub EvtUpdatePoolCreator);

impl BorshSerialize for EvtUpdatePoolCreatorEvent {
   fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        EVT_UPDATE_POOL_CREATOR_DISCM.serialize(writer)?;
        self.0.serialize(writer)
    }
}

impl EvtUpdatePoolCreatorEvent {
    pub fn deserialize(buf: &mut &[u8]) -> std::io::Result<Self> {
        let maybe_discm = <[u8; 8]>::deserialize(buf)?;
        if maybe_discm != EVT_UPDATE_POOL_CREATOR_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    EVT_UPDATE_POOL_CREATOR_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(EvtUpdatePoolCreator::deserialize(buf)?))
    }
}

pub const EVT_VIRTUAL_POOL_METADATA_DISCM: [u8; 8] = [188, 18, 72, 76, 195, 91, 38, 74];

#[derive(Clone, Debug, BorshDeserialize, BorshSerialize, PartialEq,serde::Serialize, serde::Deserialize)]
pub struct EvtVirtualPoolMetadata {
    pub virtual_pool_metadata: Pubkey,
    pub virtual_pool: Pubkey,
}

#[derive(Clone, Debug, PartialEq)]
pub struct EvtVirtualPoolMetadataEvent(pub EvtVirtualPoolMetadata);

impl BorshSerialize for EvtVirtualPoolMetadataEvent {
   fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        EVT_VIRTUAL_POOL_METADATA_DISCM.serialize(writer)?;
        self.0.serialize(writer)
    }
}

impl EvtVirtualPoolMetadataEvent {
    pub fn deserialize(buf: &mut &[u8]) -> std::io::Result<Self> {
        let maybe_discm = <[u8; 8]>::deserialize(buf)?;
        if maybe_discm != EVT_VIRTUAL_POOL_METADATA_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    EVT_VIRTUAL_POOL_METADATA_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(EvtVirtualPoolMetadata::deserialize(buf)?))
    }
}

pub const EVT_WITHDRAW_LEFTOVER_DISCM: [u8; 8] = [191, 189, 104, 143, 111, 156, 94, 229];

#[derive(Clone, Debug, BorshDeserialize, BorshSerialize, PartialEq,serde::Serialize, serde::Deserialize)]
pub struct EvtWithdrawLeftover {
    pub pool: Pubkey,
    pub leftover_receiver: Pubkey,
    pub leftover_amount: u64,
}

#[derive(Clone, Debug, PartialEq)]
pub struct EvtWithdrawLeftoverEvent(pub EvtWithdrawLeftover);

impl BorshSerialize for EvtWithdrawLeftoverEvent {
   fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        EVT_WITHDRAW_LEFTOVER_DISCM.serialize(writer)?;
        self.0.serialize(writer)
    }
}

impl EvtWithdrawLeftoverEvent {
    pub fn deserialize(buf: &mut &[u8]) -> std::io::Result<Self> {
        let maybe_discm = <[u8; 8]>::deserialize(buf)?;
        if maybe_discm != EVT_WITHDRAW_LEFTOVER_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    EVT_WITHDRAW_LEFTOVER_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(EvtWithdrawLeftover::deserialize(buf)?))
    }
}

pub const EVT_WITHDRAW_MIGRATION_FEE_DISCM: [u8; 8] = [26, 203, 84, 85, 161, 23, 100, 214];

#[derive(Clone, Debug, BorshDeserialize, BorshSerialize, PartialEq,serde::Serialize, serde::Deserialize)]
pub struct EvtWithdrawMigrationFee {
    pub pool: Pubkey,
    pub fee: u64,
    pub flag: u8,
}

#[derive(Clone, Debug, PartialEq)]
pub struct EvtWithdrawMigrationFeeEvent(pub EvtWithdrawMigrationFee);

impl BorshSerialize for EvtWithdrawMigrationFeeEvent {
   fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        EVT_WITHDRAW_MIGRATION_FEE_DISCM.serialize(writer)?;
        self.0.serialize(writer)
    }
}

impl EvtWithdrawMigrationFeeEvent {
    pub fn deserialize(buf: &mut &[u8]) -> std::io::Result<Self> {
        let maybe_discm = <[u8; 8]>::deserialize(buf)?;
        if maybe_discm != EVT_WITHDRAW_MIGRATION_FEE_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    EVT_WITHDRAW_MIGRATION_FEE_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(EvtWithdrawMigrationFee::deserialize(buf)?))
    }
}