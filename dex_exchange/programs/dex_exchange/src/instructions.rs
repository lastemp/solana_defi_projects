// admin instructions
pub mod init;
//pub mod register_lottery_game;

// public instructions
pub mod add_liquidity;
pub mod create_token;
pub mod register_liquidity_provider;
pub mod register_trader;
pub mod swap;
pub mod transfer_token;

// bring everything in scope
pub use {
    add_liquidity::*, create_token::*, init::*, register_liquidity_provider::*, register_trader::*,
    swap::*, transfer_token::*,
};
