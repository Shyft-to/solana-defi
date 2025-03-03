use solana_program::{
    decode_error::DecodeError, msg, program_error::{PrintProgramError, ProgramError},
};
use thiserror::Error;
#[derive(Clone, Copy, Debug, Eq, Error, num_derive::FromPrimitive, PartialEq)]
pub enum MeteoraPoolsError {
    #[error("Math operation overflow")]
    MathOverflow = 6000,
    #[error("Invalid fee setup")]
    InvalidFee = 6001,
    #[error("Invalid invariant d")]
    InvalidInvariant = 6002,
    #[error("Fee calculation failure")]
    FeeCalculationFailure = 6003,
    #[error("Exceeded slippage tolerance")]
    ExceededSlippage = 6004,
    #[error("Invalid curve calculation")]
    InvalidCalculation = 6005,
    #[error("Given pool token amount results in zero trading tokens")]
    ZeroTradingTokens = 6006,
    #[error("Math conversion overflow")]
    ConversionError = 6007,
    #[error(
        "LP mint authority must be 'A' vault lp, without freeze authority, and 0 supply"
    )]
    FaultyLpMint = 6008,
    #[error("Token mint mismatched")]
    MismatchedTokenMint = 6009,
    #[error("LP mint mismatched")]
    MismatchedLpMint = 6010,
    #[error("Invalid lp token owner")]
    MismatchedOwner = 6011,
    #[error("Invalid vault account")]
    InvalidVaultAccount = 6012,
    #[error("Invalid vault lp account")]
    InvalidVaultLpAccount = 6013,
    #[error("Invalid pool lp mint account")]
    InvalidPoolLpMintAccount = 6014,
    #[error("Pool disabled")]
    PoolDisabled = 6015,
    #[error("Invalid admin account")]
    InvalidAdminAccount = 6016,
    #[error("Invalid protocol fee account")]
    InvalidProtocolFeeAccount = 6017,
    #[error("Same admin account")]
    SameAdminAccount = 6018,
    #[error("Identical user source and destination token account")]
    IdenticalSourceDestination = 6019,
    #[error("Apy calculation error")]
    ApyCalculationError = 6020,
    #[error("Insufficient virtual price snapshot")]
    InsufficientSnapshot = 6021,
    #[error("Current curve is non-updatable")]
    NonUpdatableCurve = 6022,
    #[error("New curve is mismatched with old curve")]
    MisMatchedCurve = 6023,
    #[error("Amplification is invalid")]
    InvalidAmplification = 6024,
    #[error("Operation is not supported")]
    UnsupportedOperation = 6025,
    #[error("Exceed max amplification changes")]
    ExceedMaxAChanges = 6026,
    #[error("Invalid remaining accounts length")]
    InvalidRemainingAccountsLen = 6027,
    #[error("Invalid remaining account")]
    InvalidRemainingAccounts = 6028,
    #[error("Token mint B doesn't matches depeg type token mint")]
    MismatchedDepegMint = 6029,
    #[error("Invalid APY account")]
    InvalidApyAccount = 6030,
    #[error("Invalid token multiplier")]
    InvalidTokenMultiplier = 6031,
    #[error("Invalid depeg information")]
    InvalidDepegInformation = 6032,
    #[error("Update time constraint violated")]
    UpdateTimeConstraint = 6033,
    #[error("Exceeded max fee bps")]
    ExceedMaxFeeBps = 6034,
    #[error("Invalid admin")]
    InvalidAdmin = 6035,
    #[error("Pool is not permissioned")]
    PoolIsNotPermissioned = 6036,
    #[error("Invalid deposit amount")]
    InvalidDepositAmount = 6037,
    #[error("Invalid fee owner")]
    InvalidFeeOwner = 6038,
    #[error("Pool is not depleted")]
    NonDepletedPool = 6039,
    #[error("Token amount is not 1:1")]
    AmountNotPeg = 6040,
    #[error("Amount is zero")]
    AmountIsZero = 6041,
    #[error("Type cast error")]
    TypeCastFailed = 6042,
    #[error("Amount is not enough")]
    AmountIsNotEnough = 6043,
    #[error("Invalid activation duration")]
    InvalidActivationDuration = 6044,
    #[error("Pool is not launch pool")]
    PoolIsNotLaunchPool = 6045,
    #[error("Unable to modify activation point")]
    UnableToModifyActivationPoint = 6046,
    #[error("Invalid authority to create the pool")]
    InvalidAuthorityToCreateThePool = 6047,
    #[error("Invalid activation type")]
    InvalidActivationType = 6048,
    #[error("Invalid activation point")]
    InvalidActivationPoint = 6049,
    #[error("Pre activation swap window started")]
    PreActivationSwapStarted = 6050,
    #[error("Invalid pool type")]
    InvalidPoolType = 6051,
    #[error("Quote token must be SOL,USDC")]
    InvalidQuoteMint = 6052,
}
impl From<MeteoraPoolsError> for ProgramError {
    fn from(e: MeteoraPoolsError) -> Self {
        ProgramError::Custom(e as u32)
    }
}
impl<T> DecodeError<T> for MeteoraPoolsError {
    fn type_of() -> &'static str {
        "MeteoraPoolsError"
    }
}
impl PrintProgramError for MeteoraPoolsError {
    fn print<E>(&self)
    where
        E: 'static + std::error::Error + DecodeError<E> + PrintProgramError
            + num_traits::FromPrimitive,
    {
        msg!(& self.to_string());
    }
}
