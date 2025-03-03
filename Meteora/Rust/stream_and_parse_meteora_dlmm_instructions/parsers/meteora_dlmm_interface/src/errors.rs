use solana_program::{
    decode_error::DecodeError, msg, program_error::{PrintProgramError, ProgramError},
};
use thiserror::Error;
#[derive(Clone, Copy, Debug, Eq, Error, num_derive::FromPrimitive, PartialEq)]
pub enum MeteoraDlmmError {
    #[error("Invalid start bin index")]
    InvalidStartBinIndex = 6000,
    #[error("Invalid bin id")]
    InvalidBinId = 6001,
    #[error("Invalid input data")]
    InvalidInput = 6002,
    #[error("Exceeded amount slippage tolerance")]
    ExceededAmountSlippageTolerance = 6003,
    #[error("Exceeded bin slippage tolerance")]
    ExceededBinSlippageTolerance = 6004,
    #[error("Composition factor flawed")]
    CompositionFactorFlawed = 6005,
    #[error("Non preset bin step")]
    NonPresetBinStep = 6006,
    #[error("Zero liquidity")]
    ZeroLiquidity = 6007,
    #[error("Invalid position")]
    InvalidPosition = 6008,
    #[error("Bin array not found")]
    BinArrayNotFound = 6009,
    #[error("Invalid token mint")]
    InvalidTokenMint = 6010,
    #[error("Invalid account for single deposit")]
    InvalidAccountForSingleDeposit = 6011,
    #[error("Pair insufficient liquidity")]
    PairInsufficientLiquidity = 6012,
    #[error("Invalid fee owner")]
    InvalidFeeOwner = 6013,
    #[error("Invalid fee withdraw amount")]
    InvalidFeeWithdrawAmount = 6014,
    #[error("Invalid admin")]
    InvalidAdmin = 6015,
    #[error("Identical fee owner")]
    IdenticalFeeOwner = 6016,
    #[error("Invalid basis point")]
    InvalidBps = 6017,
    #[error("Math operation overflow")]
    MathOverflow = 6018,
    #[error("Type cast error")]
    TypeCastFailed = 6019,
    #[error("Invalid reward index")]
    InvalidRewardIndex = 6020,
    #[error("Invalid reward duration")]
    InvalidRewardDuration = 6021,
    #[error("Reward already initialized")]
    RewardInitialized = 6022,
    #[error("Reward not initialized")]
    RewardUninitialized = 6023,
    #[error("Identical funder")]
    IdenticalFunder = 6024,
    #[error("Reward campaign in progress")]
    RewardCampaignInProgress = 6025,
    #[error("Reward duration is the same")]
    IdenticalRewardDuration = 6026,
    #[error("Invalid bin array")]
    InvalidBinArray = 6027,
    #[error("Bin arrays must be continuous")]
    NonContinuousBinArrays = 6028,
    #[error("Invalid reward vault")]
    InvalidRewardVault = 6029,
    #[error("Position is not empty")]
    NonEmptyPosition = 6030,
    #[error("Unauthorized access")]
    UnauthorizedAccess = 6031,
    #[error("Invalid fee parameter")]
    InvalidFeeParameter = 6032,
    #[error("Missing oracle account")]
    MissingOracle = 6033,
    #[error("Insufficient observation sample")]
    InsufficientSample = 6034,
    #[error("Invalid lookup timestamp")]
    InvalidLookupTimestamp = 6035,
    #[error("Bitmap extension account is not provided")]
    BitmapExtensionAccountIsNotProvided = 6036,
    #[error("Cannot find non-zero liquidity binArrayId")]
    CannotFindNonZeroLiquidityBinArrayId = 6037,
    #[error("Bin id out of bound")]
    BinIdOutOfBound = 6038,
    #[error("Insufficient amount in for minimum out")]
    InsufficientOutAmount = 6039,
    #[error("Invalid position width")]
    InvalidPositionWidth = 6040,
    #[error("Excessive fee update")]
    ExcessiveFeeUpdate = 6041,
    #[error("Pool disabled")]
    PoolDisabled = 6042,
    #[error("Invalid pool type")]
    InvalidPoolType = 6043,
    #[error("Whitelist for wallet is full")]
    ExceedMaxWhitelist = 6044,
    #[error("Invalid index")]
    InvalidIndex = 6045,
    #[error("Reward not ended")]
    RewardNotEnded = 6046,
    #[error("Must withdraw ineligible reward")]
    MustWithdrawnIneligibleReward = 6047,
    #[error("Unauthorized address")]
    UnauthorizedAddress = 6048,
    #[error("Cannot update because operators are the same")]
    OperatorsAreTheSame = 6049,
    #[error("Withdraw to wrong token account")]
    WithdrawToWrongTokenAccount = 6050,
    #[error("Wrong rent receiver")]
    WrongRentReceiver = 6051,
    #[error("Already activated")]
    AlreadyPassActivationPoint = 6052,
    #[error("Swapped amount is exceeded max swapped amount")]
    ExceedMaxSwappedAmount = 6053,
    #[error("Invalid strategy parameters")]
    InvalidStrategyParameters = 6054,
    #[error("Liquidity locked")]
    LiquidityLocked = 6055,
    #[error("Bin range is not empty")]
    BinRangeIsNotEmpty = 6056,
    #[error("Amount out is not matched with exact amount out")]
    NotExactAmountOut = 6057,
    #[error("Invalid activation type")]
    InvalidActivationType = 6058,
    #[error("Invalid activation duration")]
    InvalidActivationDuration = 6059,
    #[error("Missing token amount as token launch owner proof")]
    MissingTokenAmountAsTokenLaunchProof = 6060,
    #[error("Quote token must be SOL or USDC")]
    InvalidQuoteToken = 6061,
    #[error("Invalid bin step")]
    InvalidBinStep = 6062,
    #[error("Invalid base fee")]
    InvalidBaseFee = 6063,
    #[error("Invalid pre-activation duration")]
    InvalidPreActivationDuration = 6064,
    #[error("Already pass pre-activation swap point")]
    AlreadyPassPreActivationSwapPoint = 6065,
}
impl From<MeteoraDlmmError> for ProgramError {
    fn from(e: MeteoraDlmmError) -> Self {
        ProgramError::Custom(e as u32)
    }
}
impl<T> DecodeError<T> for MeteoraDlmmError {
    fn type_of() -> &'static str {
        "MeteoraDlmmError"
    }
}
impl PrintProgramError for MeteoraDlmmError {
    fn print<E>(&self)
    where
        E: 'static + std::error::Error + DecodeError<E> + PrintProgramError
            + num_traits::FromPrimitive,
    {
        msg!(& self.to_string());
    }
}
