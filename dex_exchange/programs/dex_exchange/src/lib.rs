pub mod error;
pub mod instructions;
pub mod state;

use {anchor_lang::prelude::*, instructions::*};

declare_id!("9EqVh33gZ9Q2rG5kSyWWrMAvKgrXakoPXavPZucqQ3P8");

#[program]
pub mod dex_exchange {
    use super::*;

    // admin instructions
    pub fn init(ctx: Context<Init>, params: InitParams) -> Result<()> {
        instructions::init(ctx, &params)
    }

    // public instructions
    pub fn register_liquidity_provider(ctx: Context<RegisterLiquidityProvider>) -> Result<()> {
        instructions::register_liquidity_provider(ctx)
    }

    pub fn register_trader(ctx: Context<RegisterTrader>) -> Result<()> {
        instructions::register_trader(ctx)
    }

    pub fn add_liquidity(ctx: Context<AddLiquidity>, params: AddLiquidityParams) -> Result<()> {
        instructions::add_liquidity(ctx, &params)
    }

    pub fn swap(ctx: Context<Swap>, params: SwapParams) -> Result<()> {
        instructions::swap(ctx, &params)
    }

    pub fn create_token(ctx: Context<CreateToken>, params: CreateTokenParams) -> Result<()> {
        instructions::create_token(ctx, &params)
    }

    pub fn transfer_token(ctx: Context<TransferToken>, params: TransferTokenParams) -> Result<()> {
        instructions::transfer_token(ctx, &params)
    }
}
