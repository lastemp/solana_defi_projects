// admin instructions
pub mod init;

// public instructions
pub mod create_futures_contract;
pub mod create_token;
pub mod deposit_asset;
pub mod settle_futures_contract;
pub mod transfer_token;

// bring everything in scope
pub use {
    create_futures_contract::*, create_token::*, deposit_asset::*, init::*,
    settle_futures_contract::*, transfer_token::*,
};
