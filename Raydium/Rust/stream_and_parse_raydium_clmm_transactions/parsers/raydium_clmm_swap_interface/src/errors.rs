use solana_program::{
    decode_error::DecodeError,
    msg,
    program_error::{PrintProgramError, ProgramError},
};
use thiserror::Error;
use num_derive::FromPrimitive;

#[derive(Clone, Copy, Debug, Eq, Error, FromPrimitive, PartialEq)]
pub enum RaydiumClmmSwapError {
    #[error("LOK")]
    LOK = 6000,
    
    #[error("Not approved")]
    NotApproved = 6001,
    
    #[error("invalid update amm config flag")]
    InvalidUpdateConfigFlag = 6002,
    
    #[error("Account lack")]
    AccountLack = 6003,
    
    #[error("Remove liquidity, collect fees owed and reward then you can close position account")]
    ClosePositionErr = 6004,
    
    #[error("Minting amount should be greater than 0")]
    ZeroMintAmount = 6005,
    
    #[error("Tick out of range")]
    InvaildTickIndex = 6006,
    
    #[error("The lower tick must be below the upper tick")]
    TickInvaildOrder = 6007,
    
    #[error("The tick must be greater, or equal to the minimum tick (-443636)")]
    TickLowerOverflow = 6008,
    
    #[error("The tick must be lesser than, or equal to the maximum tick (443636)")]
    TickUpperOverflow = 6009,
}

impl From<RaydiumClmmSwapError> for ProgramError {
    fn from(e: RaydiumClmmSwapError) -> Self {
        ProgramError::Custom(e as u32)
    }
}

impl<T> DecodeError<T> for RaydiumClmmSwapError {
    fn type_of() -> &'static str {
        "RaydiumClmmSwapError"
    }
}

impl PrintProgramError for RaydiumClmmSwapError {
    fn print<E>(&self)
    where
        E: 'static
            + std::error::Error
            + DecodeError<E>
            + PrintProgramError
            + num_traits::FromPrimitive,
    {
        msg!(&self.to_string());
    }
}
