// admin instructions
pub mod init;

// public instructions
pub mod deposit_tokens;
pub mod withdraw_tokens;

// bring everything in scope
pub use {deposit_tokens::*, init::*, withdraw_tokens::*};
