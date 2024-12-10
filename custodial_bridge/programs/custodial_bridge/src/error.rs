use anchor_lang::prelude::*;

#[error_code]
pub enum CustomError {
    #[msg("Invalid amount.")]
    InvalidAmount,
    #[msg("Account is not initialized.")]
    AccountNotInitialized,
    #[msg("Account is already initialized.")]
    AccountAlreadyInitialized,
}
