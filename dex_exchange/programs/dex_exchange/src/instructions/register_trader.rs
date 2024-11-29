//! RegisterTrader instruction handler

use {
    crate::{error::DexError, state::trader::Trader},
    anchor_lang::prelude::*,
};

#[derive(Accounts)]
pub struct RegisterTrader<'info> {
    // init means to create account
    // bump to use unique address for account
    #[account(
        init,
        payer = owner,
        space = 8 + Trader::INIT_SPACE,
        constraint = !trader.active @ DexError::AccountAlreadyInitialized,
        seeds = [b"trader", owner.key().as_ref()],
        bump
    )]
    pub trader: Account<'info, Trader>,
    // mut makes it changeble (mutable)
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}

pub fn register_trader(ctx: Context<RegisterTrader>) -> Result<()> {
    let trader = &mut ctx.accounts.trader;

    // * - means dereferencing
    trader.owner = *ctx.accounts.owner.key;
    trader.active = true;

    Ok(())
}
