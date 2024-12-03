use anchor_lang::prelude::*;

#[error_code]
pub enum CustomError {
    #[msg("Invalid operator length")]
    InvalidOperatorLength,
    #[msg("Invalid lottery game name length")]
    InvalidLotteryGameNameLength,
    #[msg("Invalid value date length")]
    InvalidValueDateLength,
    #[msg("Invalid amount.")]
    InvalidAmount,
    #[msg("Available balance should match tranfer amount.")]
    MismatchedAmount,
    #[msg("Invalid numeric value.")]
    InvalidNumeric,
    #[msg("Invalid lottery ticket amount.")]
    InvalidLotteryTicketAmount,
    #[msg("Lottery game is closed.")]
    LotteryGameClosed,
    #[msg("Invalid lottery game winner.")]
    InvalidLotteryGameWinner,

    //
    #[msg("Invalid country length")]
    InvalidCountryLength,

    // Arithmetic
    #[msg("Arithmetic operation failed.")]
    InvalidArithmeticOperation,
    #[msg("Insufficient funds.")]
    InsufficientFunds,

    // liquidity provider
    #[msg("Liquidity provider has no active status.")]
    InvalidLiquidityProviderStatus,
    //#[msg("Participant(s) missing.")]
    //InvalidParticipants,

    // trader
    #[msg("Trader has no active status.")]
    InvalidTraderStatus,

    // account
    #[msg("Account is not initialized.")]
    AccountNotInitialized,
    #[msg("Account is already initialized.")]
    AccountAlreadyInitialized,
}
