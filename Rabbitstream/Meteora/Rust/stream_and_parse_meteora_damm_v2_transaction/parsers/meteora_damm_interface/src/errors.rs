use solana_program::{
    decode_error::DecodeError, msg,
    program_error::{PrintProgramError, ProgramError},
};
use thiserror::Error;
use num_derive::FromPrimitive;

#[derive(Clone, Copy, Debug, Eq, Error, FromPrimitive, PartialEq)]
pub enum YourProgramError {
    #[error("Math operation overflow")]
    MathOverflow = 6000,

    #[error("Invalid fee setup")]
    InvalidFee = 6001,

    #[error("Exceeded slippage tolerance")]
    ExceededSlippage = 6002,

    #[error("Pool disabled")]
    PoolDisabled = 6003,

    #[error("Exceeded max fee bps")]
    ExceedMaxFeeBps = 6004,

    #[error("Invalid admin")]
    InvalidAdmin = 6005,

    #[error("Amount is zero")]
    AmountIsZero = 6006,

    #[error("Type cast error")]
    TypeCastFailed = 6007,

    #[error("Unable to modify activation point")]
    UnableToModifyActivationPoint = 6008,

    #[error("Invalid authority to create the pool")]
    InvalidAuthorityToCreateThePool = 6009,

    #[error("Invalid activation type")]
    InvalidActivationType = 6010,

    #[error("Invalid activation point")]
    InvalidActivationPoint = 6011,

    #[error("Quote token must be SOL,USDC")]
    InvalidQuoteMint = 6012,

    #[error("Invalid fee curve")]
    InvalidFeeCurve = 6013,

    #[error("Invalid Price Range")]
    InvalidPriceRange = 6014,

    #[error("Trade is over price range")]
    PriceRangeViolation = 6015,

    #[error("Invalid parameters")]
    InvalidParameters = 6016,

    #[error("Invalid collect fee mode")]
    InvalidCollectFeeMode = 6017,

    #[error("Invalid input")]
    InvalidInput = 6018,

    #[error("Cannot create token badge on supported mint")]
    CannotCreateTokenBadgeOnSupportedMint = 6019,

    #[error("Invalid token badge")]
    InvalidTokenBadge = 6020,

    #[error("Invalid minimum liquidity")]
    InvalidMinimumLiquidity = 6021,

    #[error("Invalid vesting information")]
    InvalidVestingInfo = 6022,

    #[error("Insufficient liquidity")]
    InsufficientLiquidity = 6023,

    #[error("Invalid vesting account")]
    InvalidVestingAccount = 6024,

    #[error("Invalid pool status")]
    InvalidPoolStatus = 6025,

    #[error("Unsupported native mint token2022")]
    UnsupportNativeMintToken2022 = 6026,

    #[error("Invalid reward index")]
    InvalidRewardIndex = 6027,

    #[error("Invalid reward duration")]
    InvalidRewardDuration = 6028,

    #[error("Reward already initialized")]
    RewardInitialized = 6029,

    #[error("Reward not initialized")]
    RewardUninitialized = 6030,

    #[error("Invalid reward vault")]
    InvalidRewardVault = 6031,

    #[error("Must withdraw ineligible reward")]
    MustWithdrawnIneligibleReward = 6032,

    #[error("Reward duration is the same")]
    IdenticalRewardDuration = 6033,

    #[error("Reward campaign in progress")]
    RewardCampaignInProgress = 6034,

    #[error("Identical funder")]
    IdenticalFunder = 6035,

    #[error("Invalid funder")]
    InvalidFunder = 6036,

    #[error("Reward not ended")]
    RewardNotEnded = 6037,

    #[error("Fee inverse is incorrect")]
    FeeInverseIsIncorrect = 6038,

    #[error("Position is not empty")]
    PositionIsNotEmpty = 6039,

    #[error("Invalid pool creator authority")]
    InvalidPoolCreatorAuthority = 6040,

    #[error("Invalid config type")]
    InvalidConfigType = 6041,
}

// Conversion to ProgramError
impl From<YourProgramError> for ProgramError {
    fn from(e: YourProgramError) -> Self {
        ProgramError::Custom(e as u32)
    }
}

// For decoding errors
impl<T> DecodeError<T> for YourProgramError {
    fn type_of() -> &'static str {
        "YourProgramError"
    }
}

// Optional: for logging/debugging on-chain
impl PrintProgramError for YourProgramError {
    fn print<E>(&self)
    where
        E: 'static + std::error::Error + DecodeError<E> + PrintProgramError + num_traits::FromPrimitive,
    {
        msg!(&self.to_string());
    }
}
