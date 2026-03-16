use solana_program::{
    decode_error::DecodeError, msg, program_error::{PrintProgramError, ProgramError},
};
use thiserror::Error;
#[derive(Clone, Copy, Debug, Eq, Error, num_derive::FromPrimitive, PartialEq)]
pub enum RaydiumAmmError {
    #[error("AlreadyInUse")]
    AlreadyInUse = 0,
    #[error("InvalidProgramAddress")]
    InvalidProgramAddress = 1,
    #[error("ExpectedMint")]
    ExpectedMint = 2,
    #[error("ExpectedAccount")]
    ExpectedAccount = 3,
    #[error("InvalidCoinVault")]
    InvalidCoinVault = 4,
    #[error("InvalidPCVault")]
    InvalidPcVault = 5,
    #[error("InvalidTokenLP")]
    InvalidTokenLp = 6,
    #[error("InvalidDestTokenCoin")]
    InvalidDestTokenCoin = 7,
    #[error("InvalidDestTokenPC")]
    InvalidDestTokenPc = 8,
    #[error("InvalidPoolMint")]
    InvalidPoolMint = 9,
    #[error("InvalidOpenOrders")]
    InvalidOpenOrders = 10,
    #[error("InvalidSerumMarket")]
    InvalidSerumMarket = 11,
    #[error("InvalidSerumProgram")]
    InvalidSerumProgram = 12,
    #[error("InvalidTargetOrders")]
    InvalidTargetOrders = 13,
    #[error("InvalidWithdrawQueue")]
    InvalidWithdrawQueue = 14,
    #[error("InvalidTempLp")]
    InvalidTempLp = 15,
    #[error("InvalidCoinMint")]
    InvalidCoinMint = 16,
    #[error("InvalidPCMint")]
    InvalidPcMint = 17,
    #[error("InvalidOwner")]
    InvalidOwner = 18,
    #[error("InvalidSupply")]
    InvalidSupply = 19,
    #[error("InvalidDelegate")]
    InvalidDelegate = 20,
    #[error("Invalid Sign Account")]
    InvalidSignAccount = 21,
    #[error("InvalidStatus")]
    InvalidStatus = 22,
    #[error("Invalid instruction")]
    InvalidInstruction = 23,
    #[error("Wrong accounts number")]
    WrongAccountsNumber = 24,
    #[error("Withdraw_transfer is busy")]
    WithdrawTransferBusy = 25,
    #[error("WithdrawQueue is full")]
    WithdrawQueueFull = 26,
    #[error("WithdrawQueue is empty")]
    WithdrawQueueEmpty = 27,
    #[error("Params Set is invalid")]
    InvalidParamsSet = 28,
    #[error("InvalidInput")]
    InvalidInput = 29,
    #[error("instruction exceeds desired slippage limit")]
    ExceededSlippage = 30,
    #[error("CalculationExRateFailure")]
    CalculationExRateFailure = 31,
    #[error("Checked_Sub Overflow")]
    CheckedSubOverflow = 32,
    #[error("Checked_Add Overflow")]
    CheckedAddOverflow = 33,
    #[error("Checked_Mul Overflow")]
    CheckedMulOverflow = 34,
    #[error("Checked_Div Overflow")]
    CheckedDivOverflow = 35,
    #[error("Empty Funds")]
    CheckedEmptyFunds = 36,
    #[error("Calc pnl error")]
    CalcPnlError = 37,
    #[error("InvalidSplTokenProgram")]
    InvalidSplTokenProgram = 38,
    #[error("Take Pnl error")]
    TakePnlError = 39,
    #[error("Insufficient funds")]
    InsufficientFunds = 40,
    #[error("Conversion to u64 failed with an overflow or underflow")]
    ConversionFailure = 41,
    #[error("user token input does not match amm")]
    InvalidUserToken = 42,
    #[error("InvalidSrmMint")]
    InvalidSrmMint = 43,
    #[error("InvalidSrmToken")]
    InvalidSrmToken = 44,
    #[error("TooManyOpenOrders")]
    TooManyOpenOrders = 45,
    #[error("OrderAtSlotIsPlaced")]
    OrderAtSlotIsPlaced = 46,
    #[error("InvalidSysProgramAddress")]
    InvalidSysProgramAddress = 47,
    #[error("The provided fee does not match the program owner's constraints")]
    InvalidFee = 48,
    #[error("Repeat create amm about market")]
    RepeatCreateAmm = 49,
    #[error("Not allow Zero LP")]
    NotAllowZeroLp = 50,
    #[error("Token account has a close authority")]
    InvalidCloseAuthority = 51,
    #[error("Pool token mint has a freeze authority")]
    InvalidFreezeAuthority = 52,
    #[error("InvalidReferPCMint")]
    InvalidReferPcMint = 53,
    #[error("InvalidConfigAccount")]
    InvalidConfigAccount = 54,
    #[error("Repeat create staking config account")]
    RepeatCreateConfigAccount = 55,
    #[error("Unknown Amm Error")]
    UnknownAmmError = 56,
}
impl From<RaydiumAmmError> for ProgramError {
    fn from(e: RaydiumAmmError) -> Self {
        ProgramError::Custom(e as u32)
    }
}
impl<T> DecodeError<T> for RaydiumAmmError {
    fn type_of() -> &'static str {
        "RaydiumAmmError"
    }
}
impl PrintProgramError for RaydiumAmmError {
    fn print<E>(&self)
    where
        E: 'static + std::error::Error + DecodeError<E> + PrintProgramError
            + num_traits::FromPrimitive,
    {
        msg!(& self.to_string());
    }
}
