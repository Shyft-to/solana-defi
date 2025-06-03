use solana_program::{
    decode_error::DecodeError, 
    msg, 
    program_error::{PrintProgramError, ProgramError},
};
use thiserror::Error;
use num_derive::FromPrimitive;

#[derive(Clone, Copy, Debug, Eq, Error, num_derive::FromPrimitive, PartialEq)]
pub enum PumpfunAmmSwapError {
    #[error("Fee basis points exceed the maximum allowed.")]
    FeeBasisPointsExceedsMaximum = 6000,
    
    #[error("Base amount cannot be zero.")]
    ZeroBaseAmount = 6001,
    
    #[error("Quote amount cannot be zero.")]
    ZeroQuoteAmount = 6002,
    
    #[error("Pool token liquidity is too low.")]
    TooLittlePoolTokenLiquidity = 6003,
    
    #[error("Slippage exceeded the allowed threshold.")]
    ExceededSlippage = 6004,
    
    #[error("Admin is not valid.")]
    InvalidAdmin = 6005,
    
    #[error("Unsupported base token mint.")]
    UnsupportedBaseMint = 6006,
    
    #[error("Unsupported quote token mint.")]
    UnsupportedQuoteMint = 6007,
    
    #[error("Base token mint is invalid.")]
    InvalidBaseMint = 6008,
    
    #[error("Quote token mint is invalid.")]
    InvalidQuoteMint = 6009,
    
    #[error("LP mint is invalid.")]
    InvalidLpMint = 6010,
    
    #[error("All protocol fee recipients must be non-zero.")]
    AllProtocolFeeRecipientsShouldBeNonZero = 6011,
    
    #[error("Protocol fee recipients must be sorted and unique.")]
    UnsortedNotUniqueProtocolFeeRecipients = 6012,
    
    #[error("Protocol fee recipient is invalid.")]
    InvalidProtocolFeeRecipient = 6013,
    
    #[error("Invalid pool base token account.")]
    InvalidPoolBaseTokenAccount = 6014,
    
    #[error("Invalid pool quote token account.")]
    InvalidPoolQuoteTokenAccount = 6015,
    
    #[error("Buy amount exceeds pool reserves.")]
    BuyMoreBaseAmountThanPoolReserves = 6016,
    
    #[error("Creating pool is currently disabled.")]
    DisabledCreatePool = 6017,
    
    #[error("Deposits are currently disabled.")]
    DisabledDeposit = 6018,
    
    #[error("Withdrawals are currently disabled.")]
    DisabledWithdraw = 6019,
    
    #[error("Buying is currently disabled.")]
    DisabledBuy = 6020,
    
    #[error("Selling is currently disabled.")]
    DisabledSell = 6021,
    
    #[error("Base and quote mints must be different.")]
    SameMint = 6022,
    
    #[error("Overflow error.")]
    Overflow = 6023,
    
    #[error("Truncation error.")]
    Truncation = 6024,
    
    #[error("Division by zero error.")]
    DivisionByZero = 6025,
    
    #[error("New size cannot be less than the current size.")]
    NewSizeLessThanCurrentSize = 6026,
    
    #[error("Account type not supported.")]
    AccountTypeNotSupported = 6027,
}

impl From<PumpfunAmmSwapError> for ProgramError {
    fn from(e: PumpfunAmmSwapError) -> Self {
        ProgramError::Custom(e as u32)
    }
}

impl<T> DecodeError<T> for PumpfunAmmSwapError {
    fn type_of() -> &'static str {
        "PumpfunAmmSwapError"
    }
}

impl PrintProgramError for PumpfunAmmSwapError {
    fn print<E>(&self)
    where
        E: 'static + std::error::Error + DecodeError<E> + PrintProgramError + num_traits::FromPrimitive,
    {
        msg!("{}", self.to_string());
    }
}
