use thiserror::Error;
use solana_client::client_error::ClientError;
use solana_program::program_error::ProgramError;
use solana_sdk::pubkey::ParsePubkeyError;
use crate::models::Network;

#[derive(Error, Debug)]
pub enum PumpFunError {
    #[error("RPC error: {0}")]
    RpcError(String),
    
    #[error("Network mismatch: expected {expected:?}, got {actual:?}")]
    NetworkMismatch { expected: Network, actual: Network },
    
    #[error("Transaction timeout")]
    TransactionTimeout,
    
    #[error("Invalid account data: {0}")]
    InvalidAccountData(String),
    
    #[error("Account not found: {0}")]
    AccountNotFound(String),
    
    #[error("Invalid public key: {0}")]
    InvalidPubkey(String),
    
    #[error("Insufficient balance: need {need} SOL, have {have} SOL")]
    InsufficientBalance { need: f64, have: f64 },
    
    #[error("Insufficient liquidity")]
    InsufficientLiquidity,
    
    #[error("Pool is complete")]
    PoolComplete,
    
    #[error("Transaction failed: {0}")]
    TransactionFailed(String),
    
    #[error("Slippage exceeded: expected {expected}, got {actual}")]
    SlippageExceeded { expected: u64, actual: u64 },
    
    #[error("Arithmetic overflow")]
    ArithmeticOverflow,
    
    #[error("Serialization error: {0}")]
    SerializationError(String),
    
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
    
    #[error("Custom error: {0}")]
    Custom(String),
}

impl From<ClientError> for PumpFunError {
    fn from(err: ClientError) -> Self {
        PumpFunError::RpcError(err.to_string())
    }
}

impl From<ParsePubkeyError> for PumpFunError {
    fn from(err: ParsePubkeyError) -> Self {
        PumpFunError::InvalidPubkey(err.to_string())
    }
}

impl From<ProgramError> for PumpFunError {
    fn from(err: ProgramError) -> Self {
        PumpFunError::SerializationError(err.to_string())
    }
}

impl From<reqwest::Error> for PumpFunError {
    fn from(err: reqwest::Error) -> Self {
        PumpFunError::Custom(format!("HTTP error: {}", err))
    }
}

impl From<std::array::TryFromSliceError> for PumpFunError {
    fn from(err: std::array::TryFromSliceError) -> Self {
        PumpFunError::InvalidAccountData(format!("Failed to parse slice: {}", err))
    }
}

impl From<std::string::FromUtf8Error> for PumpFunError {
    fn from(err: std::string::FromUtf8Error) -> Self {
        PumpFunError::SerializationError(format!("UTF-8 error: {}", err))
    }
}

impl From<serde_json::Error> for PumpFunError {
    fn from(err: serde_json::Error) -> Self {
        PumpFunError::SerializationError(format!("JSON error: {}", err))
    }
}

impl From<Box<dyn std::error::Error>> for PumpFunError {
    fn from(err: Box<dyn std::error::Error>) -> Self {
        PumpFunError::Custom(err.to_string())
    }
}

// Helper type alias
pub type Result<T> = std::result::Result<T, PumpFunError>;