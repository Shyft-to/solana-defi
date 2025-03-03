use solana_program::{
    decode_error::DecodeError,
    msg,
    program_error::{PrintProgramError, ProgramError},
};
use thiserror::Error;
#[derive(Clone, Copy, Debug, Eq, Error, PartialEq)]
pub enum WhirlpoolError {
    #[error("Enum value could not be converted")]
    InvalidEnum = 6000,
    #[error("Invalid start tick index provided.")]
    InvalidStartTick = 6001,
    #[error("Tick-array already exists in this whirlpool")]
    TickArrayExistInPool = 6002,
    #[error("Attempt to search for a tick-array failed")]
    TickArrayIndexOutofBounds = 6003,
    #[error("Tick-spacing is not supported")]
    InvalidTickSpacing = 6004,
    #[error("Position is not empty It cannot be closed")]
    ClosePositionNotEmpty = 6005,
    #[error("Unable to divide by zero")]
    DivideByZero = 6006,
    #[error("Unable to cast number into BigInt")]
    NumberCastError = 6007,
    #[error("Unable to down cast number")]
    NumberDownCastError = 6008,
    #[error("Tick not found within tick array")]
    TickNotFound = 6009,
    #[error("Provided tick index is either out of bounds or uninitializable")]
    InvalidTickIndex = 6010,
    #[error("Provided sqrt price out of bounds")]
    SqrtPriceOutOfBounds = 6011,
    #[error("Liquidity amount must be greater than zero")]
    LiquidityZero = 6012,
    #[error("Liquidity amount must be less than i64::MAX")]
    LiquidityTooHigh = 6013,
    #[error("Liquidity overflow")]
    LiquidityOverflow = 6014,
    #[error("Liquidity underflow")]
    LiquidityUnderflow = 6015,
    #[error("Tick liquidity net underflowed or overflowed")]
    LiquidityNetError = 6016,
    #[error("Exceeded token max")]
    TokenMaxExceeded = 6017,
    #[error("Did not meet token min")]
    TokenMinSubceeded = 6018,
    #[error("Position token account has a missing or invalid delegate")]
    MissingOrInvalidDelegate = 6019,
    #[error("Position token amount must be 1")]
    InvalidPositionTokenAmount = 6020,
    #[error("Timestamp should be convertible from i64 to u64")]
    InvalidTimestampConversion = 6021,
    #[error("Timestamp should be greater than the last updated timestamp")]
    InvalidTimestamp = 6022,
    #[error("Invalid tick array sequence provided for instruction.")]
    InvalidTickArraySequence = 6023,
    #[error("Token Mint in wrong order")]
    InvalidTokenMintOrder = 6024,
    #[error("Reward not initialized")]
    RewardNotInitialized = 6025,
    #[error("Invalid reward index")]
    InvalidRewardIndex = 6026,
    #[error("Reward vault requires amount to support emissions for at least one day")]
    RewardVaultAmountInsufficient = 6027,
    #[error("Exceeded max fee rate")]
    FeeRateMaxExceeded = 6028,
    #[error("Exceeded max protocol fee rate")]
    ProtocolFeeRateMaxExceeded = 6029,
    #[error("Multiplication with shift right overflow")]
    MultiplicationShiftRightOverflow = 6030,
    #[error("Muldiv overflow")]
    MulDivOverflow = 6031,
    #[error("Invalid div_u256 input")]
    MulDivInvalidInput = 6032,
    #[error("Multiplication overflow")]
    MultiplicationOverflow = 6033,
    #[error("Provided SqrtPriceLimit not in the same direction as the swap.")]
    InvalidSqrtPriceLimitDirection = 6034,
    #[error("There are no tradable amount to swap.")]
    ZeroTradableAmount = 6035,
    #[error("Amount out below minimum threshold")]
    AmountOutBelowMinimum = 6036,
    #[error("Amount in above maximum threshold")]
    AmountInAboveMaximum = 6037,
    #[error("Invalid index for tick array sequence")]
    TickArraySequenceInvalidIndex = 6038,
    #[error("Amount calculated overflows")]
    AmountCalcOverflow = 6039,
    #[error("Amount remaining overflows")]
    AmountRemainingOverflow = 6040,
    #[error("Invalid intermediary mint")]
    InvalidIntermediaryMint = 6041,
    #[error("Duplicate two hop pool")]
    DuplicateTwoHopPool = 6042,
    #[error("Bundle index is out of bounds")]
    InvalidBundleIndex = 6043,
    #[error("Position has already been opened")]
    BundledPositionAlreadyOpened = 6044,
    #[error("Position has already been closed")]
    BundledPositionAlreadyClosed = 6045,
    #[error("Unable to delete PositionBundle with open positions")]
    PositionBundleNotDeletable = 6046,
    #[error("Token mint has unsupported attributes")]
    UnsupportedTokenMint = 6047,
    #[error("Invalid remaining accounts")]
    RemainingAccountsInvalidSlice = 6048,
    #[error("Insufficient remaining accounts")]
    RemainingAccountsInsufficient = 6049,
    #[error("Unable to call transfer hook without extra accounts")]
    NoExtraAccountsForTransferHook = 6050,
    #[error("Output and input amount mismatch")]
    IntermediateTokenAmountMismatch = 6051,
    #[error("Transfer fee calculation failed")]
    TransferFeeCalculationError = 6052,
    #[error("Same accounts type is provided more than once")]
    RemainingAccountsDuplicatedAccountsType = 6053,
    #[error("Too many supplemental tick arrays provided")]
    TooManySupplementalTickArrays = 6054,
    #[error("TickArray account for different whirlpool provided")]
    DifferentWhirlpoolTickArrayAccount = 6055,
}
impl From<WhirlpoolError> for ProgramError {
    fn from(e: WhirlpoolError) -> Self {
        ProgramError::Custom(e as u32)
    }
}
impl<T> DecodeError<T> for WhirlpoolError {
    fn type_of() -> &'static str {
        "WhirlpoolError"
    }
}
impl PrintProgramError for WhirlpoolError {
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
