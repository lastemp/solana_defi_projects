//! Init instruction handler

use {
    crate::{
        error::DexError,
        state::{deposit_base::DepositBase, pool::Pool},
    },
    anchor_lang::prelude::*,
};

#[derive(Accounts)]
#[instruction(params: InitParams)]
pub struct Init<'info> {
    // init means to create account
    // bump to use unique address for account
    #[account(
        init,
        payer = owner,
        space = 8 + Pool::INIT_SPACE,
        constraint = !liquidity_pool.is_initialized @ DexError::AccountAlreadyInitialized,
        seeds = [b"liquidity-pool", owner.key().as_ref()],
        bump
    )]
    pub liquidity_pool: Account<'info, Pool>,
    // init means to create account
    // bump to use unique address for account
    #[account(init, payer = owner, space = 8 + DepositBase::INIT_SPACE,
        constraint = !deposit_account.is_initialized @ DexError::AccountAlreadyInitialized
    )]
    pub deposit_account: Account<'info, DepositBase>,
    #[account(seeds = [b"auth", deposit_account.key().as_ref()], bump)]
    /// CHECK: no need to check this.
    pub pda_auth: UncheckedAccount<'info>,
    #[account(mut, seeds = [b"treasury-vault", pda_auth.key().as_ref()], bump)]
    pub treasury_vault: SystemAccount<'info>,
    // mut makes it changeble (mutable)
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct InitParams {
    token_a: Pubkey,
    token_b: Pubkey,
    decimals: u8, // decimals for the token mint
}

pub fn init(ctx: Context<Init>, params: &InitParams) -> Result<()> {
    let liquidity_pool = &mut ctx.accounts.liquidity_pool;
    let deposit_account = &mut ctx.accounts.deposit_account;

    // liquidity pool
    liquidity_pool.owner = *ctx.accounts.owner.key;
    liquidity_pool.token_a = params.token_a;
    liquidity_pool.token_b = params.token_b;
    liquidity_pool.reserve_a = 0;
    liquidity_pool.reserve_b = 0;
    liquidity_pool.decimals = params.decimals;
    liquidity_pool.is_initialized = true;

    // deposit account
    // * - means dereferencing
    deposit_account.owner = *ctx.accounts.owner.key;
    deposit_account.admin_auth_bump = ctx.bumps.pda_auth;
    deposit_account.admin_treasury_vault_bump = Some(ctx.bumps.treasury_vault);
    deposit_account.is_initialized = true;

    Ok(())
}
