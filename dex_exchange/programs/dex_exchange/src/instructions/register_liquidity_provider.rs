//! RegisterLiquidityProvider instruction handler

use {
    crate::{error::DexError, state::liquidity_provider::LiquidityProvider},
    anchor_lang::prelude::*,
};

#[derive(Accounts)]
pub struct RegisterLiquidityProvider<'info> {
    // init means to create account
    // bump to use unique address for account
    #[account(
        init,
        payer = owner,
        space = 8 + LiquidityProvider::INIT_SPACE,
        constraint = !liquidity_provider.active @ DexError::AccountAlreadyInitialized,
        seeds = [b"liquidity-provider", owner.key().as_ref()],
        bump
    )]
    pub liquidity_provider: Account<'info, LiquidityProvider>,
    // mut makes it changeble (mutable)
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}

pub fn register_liquidity_provider(ctx: Context<RegisterLiquidityProvider>) -> Result<()> {
    let liquidity_provider = &mut ctx.accounts.liquidity_provider;

    // * - means dereferencing
    liquidity_provider.owner = *ctx.accounts.owner.key;
    liquidity_provider.active = true;
    liquidity_provider.reserve_a_available_funds = 0; // reserve_a available funds
    liquidity_provider.reserve_b_available_funds = 0; // reserve_b available funds

    Ok(())
}
