use anchor_lang::prelude::*;

#[error_code]
pub enum CustomError {
    #[msg("Invalid name length")]
    InvalidNameLength,
    #[msg("Invalid symbol length")]
    InvalidSymbolLength,
    #[msg("Invalid uri length")]
    InvalidUriLength,
    #[msg("Max depth has invalid value.")]
    InvalidMaxDepth,
    #[msg("Max buffer size has invalid value.")]
    InvalidMaxBufferSize,
}
