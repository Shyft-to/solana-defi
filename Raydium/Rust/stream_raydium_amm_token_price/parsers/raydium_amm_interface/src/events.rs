use borsh::{BorshDeserialize, BorshSerialize};
use serde::Serialize;


#[derive(
    Clone,
    Debug,
    BorshDeserialize,
    BorshSerialize,
    PartialEq,
    Serialize,
    serde::Deserialize,
)]
pub struct SwapBaseInLog {
    pub log_type: u8,
    pub amount_in: u64,
    pub minimum_out: u64,
    pub direction: u64,
    pub user_source: u64,
    pub pool_coin: u64,
    pub pool_pc: u64,
    pub out_amount: u64,
}

#[derive(Clone, Debug, PartialEq)]
pub struct SwapBaseInLogEvent(pub SwapBaseInLog);

impl SwapBaseInLogEvent {
    pub fn deserialize(buf: &mut &[u8]) -> std::io::Result<Self> {
        Ok(Self(SwapBaseInLog::deserialize(buf)?))
    }
}

#[derive(
    Clone,
    Debug,
    BorshDeserialize,
    BorshSerialize,
    PartialEq,
    Serialize,
    serde::Deserialize,
)]
pub struct SwapBaseOutLog {
    pub log_type: u8,
    pub max_in: u64,
    pub amount_out: u64,
    pub direction: u64,
    pub user_source: u64,
    pub pool_coin: u64,
    pub pool_pc: u64,
    pub direct_in: u64,
}

#[derive(Clone, Debug, PartialEq)]
pub struct SwapBaseOutLogEvent(pub SwapBaseOutLog);

impl SwapBaseOutLogEvent {
    pub fn deserialize(buf: &mut &[u8]) -> std::io::Result<Self> {
        Ok(Self(SwapBaseOutLog::deserialize(buf)?))
    }
}