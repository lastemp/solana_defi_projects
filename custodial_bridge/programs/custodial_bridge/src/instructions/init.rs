//! Init instruction handler

use {
    crate::{error::CustomError, state::deposit_base::DepositBase},
    anchor_lang::prelude::*,
};

#[derive(Accounts)]
pub struct Init<'info> {
    // init means to create account
    // bump to use unique address for account
    #[account(init, payer = owner, space = 8 + DepositBase::INIT_SPACE,
        constraint = !deposit_account.is_initialized @ CustomError::AccountAlreadyInitialized
    )]
    pub deposit_account: Account<'info, DepositBase>,
    #[account(seeds = [b"auth", deposit_account.key().as_ref()], bump)]
    /// CHECK: no need to check this.
    pub pda_auth: UncheckedAccount<'info>,
    #[account(mut, seeds = [b"mint-authority", pda_auth.key().as_ref()], bump)]
    pub mint_authority: SystemAccount<'info>,
    // mut makes it changeble (mutable)
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}

pub fn init(ctx: Context<Init>) -> Result<()> {
    let deposit_account = &mut ctx.accounts.deposit_account;

    // deposit account
    // * - means dereferencing
    deposit_account.owner = *ctx.accounts.owner.key;
    deposit_account.admin_auth_bump = ctx.bumps.pda_auth;
    deposit_account.admin_treasury_vault_bump = Some(ctx.bumps.mint_authority);
    deposit_account.is_initialized = true;

    Ok(())
}
