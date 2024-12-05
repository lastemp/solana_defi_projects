//! DepositFunds instruction handler

use {
    crate::{
        error::CustomError,
        state::{deposit_base::DepositBase, derivative_contract::DerivativeContract},
    },
    anchor_lang::{prelude::*, system_program},
    anchor_spl::{
        associated_token::AssociatedToken,
        token::{transfer, Mint, Token, TokenAccount, Transfer},
    },
};

#[derive(Accounts)]
#[instruction(params: DepositFundsParams)]
pub struct DepositFunds<'info> {
    #[account(mut,
        constraint = derivative_contract.is_initialized @ CustomError::AccountNotInitialized
    )]
    pub derivative_contract: Account<'info, DerivativeContract>,
    #[account(mut,
        constraint = deposit_account.is_initialized @ CustomError::AccountNotInitialized
    )]
    pub deposit_account: Account<'info, DepositBase>,
    #[account(seeds = [b"auth", deposit_account.key().as_ref()], bump = deposit_account.admin_auth_bump)]
    /// CHECK: no need to check this.
    pub pda_auth: UncheckedAccount<'info>,
    #[account(mut, seeds = [b"treasury-vault", pda_auth.key().as_ref()], bump = deposit_account.admin_treasury_vault_bump.unwrap())]
    pub treasury_vault: SystemAccount<'info>,
    // mut makes it changeble (mutable)
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct DepositFundsParams {
    pub amount: u64,
}

pub fn deposit_funds(ctx: Context<DepositFunds>, params: &DepositFundsParams) -> Result<()> {
    msg!("Validate inputs");
    if params.amount == 0 {
        return Err(CustomError::InvalidAmount.into());
    }

    let sender = &ctx.accounts.owner;
    let derivative_contract = &mut ctx.accounts.derivative_contract;
    let amount_ = params.amount;
    let sys_program = &ctx.accounts.system_program;

    let buyer = match derivative_contract.buyer {
        Some(buyer) => buyer,
        None => return Err(CustomError::BuyerNotFound.into()),
    };

    if sender.key() != buyer {
        return Err(CustomError::InvalidBuyer.into());
    }

    let cpi_accounts = system_program::Transfer {
        from: sender.to_account_info(),
        to: ctx.accounts.treasury_vault.to_account_info(),
    };

    let cpi = CpiContext::new(sys_program.to_account_info(), cpi_accounts);

    system_program::transfer(cpi, amount_)?;

    Ok(())
}
