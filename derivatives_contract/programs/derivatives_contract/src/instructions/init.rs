//! Init instruction handler

use {
    crate::{
        error::CustomError,
        state::{deposit_base::DepositBase, derivative_contract::DerivativeContract},
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
        space = 8 + DerivativeContract::INIT_SPACE,
        constraint = !derivative_contract.is_initialized @ CustomError::AccountAlreadyInitialized,
        seeds = [b"derivative-contract", owner.key().as_ref()],
        bump
    )]
    pub derivative_contract: Account<'info, DerivativeContract>,
    // init means to create account
    // bump to use unique address for account
    #[account(init, payer = owner, space = 8 + DepositBase::INIT_SPACE,
        constraint = !deposit_account.is_initialized @ CustomError::AccountAlreadyInitialized
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
    decimals: u8, // decimals for the token mint
}

pub fn init(ctx: Context<Init>, params: &InitParams) -> Result<()> {
    let derivative_contract = &mut ctx.accounts.derivative_contract;
    let deposit_account = &mut ctx.accounts.deposit_account;

    // derivative contract
    derivative_contract.owner = *ctx.accounts.owner.key;
    derivative_contract.decimals = params.decimals;
    derivative_contract.is_initialized = true;

    // deposit account
    // * - means dereferencing
    deposit_account.owner = *ctx.accounts.owner.key;
    deposit_account.admin_auth_bump = ctx.bumps.pda_auth;
    deposit_account.admin_treasury_vault_bump = Some(ctx.bumps.treasury_vault);
    deposit_account.is_initialized = true;

    Ok(())
}
