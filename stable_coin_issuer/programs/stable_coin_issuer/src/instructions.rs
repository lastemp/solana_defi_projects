// admin instructions
pub mod init;

// public instructions
pub mod deposit;
pub mod withdrawal;

// bring everything in scope
pub use {deposit::*, init::*, withdrawal::*};
