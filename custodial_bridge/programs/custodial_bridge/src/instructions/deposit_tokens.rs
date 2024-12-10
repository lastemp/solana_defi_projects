use crate::{error::CustomError, state::deposit_base::DepositBase};
use anchor_lang::prelude::*;
use anchor_spl::token::{self, Burn, Mint, MintTo, Token, TokenAccount, Transfer};

// Context for depositing tokens
#[derive(Accounts)]
#[instruction(params: DepositTokensParams)]
pub struct DepositTokens<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(mut)]
    pub user_token_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub custodian_token_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub wrapped_mint: Account<'info, Mint>,
    #[account(mut)]
    pub user_wrapped_token_account: Account<'info, TokenAccount>,
    #[account(mut,
        constraint = deposit_account.is_initialized @ CustomError::AccountNotInitialized
    )]
    pub deposit_account: Account<'info, DepositBase>,
    #[account(seeds = [b"auth", deposit_account.key().as_ref()], bump = deposit_account.admin_auth_bump)]
    /// CHECK: no need to check this.
    pub pda_auth: UncheckedAccount<'info>,
    #[account(mut, seeds = [b"mint-authority", pda_auth.key().as_ref()], bump = deposit_account.admin_treasury_vault_bump.unwrap())]
    pub mint_authority: SystemAccount<'info>,
    pub token_program: Program<'info, Token>,
}

impl<'info> DepositTokens<'info> {
    fn into_transfer_context(&self) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        CpiContext::new(
            self.token_program.to_account_info(),
            Transfer {
                from: self.user_token_account.to_account_info(),
                to: self.custodian_token_account.to_account_info(),
                authority: self.user.to_account_info(),
            },
        )
    }

    fn into_mint_to_context(&self) -> CpiContext<'_, '_, '_, 'info, MintTo<'info>> {
        CpiContext::new(
            self.token_program.to_account_info(),
            MintTo {
                mint: self.wrapped_mint.to_account_info(),
                to: self.user_wrapped_token_account.to_account_info(),
                authority: self.mint_authority.to_account_info(),
            },
        )
    }
}

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct DepositTokensParams {
    pub amount: u64,
}

// Deposit tokens and mint wrapped tokens
pub fn deposit_tokens(ctx: Context<DepositTokens>, params: &DepositTokensParams) -> Result<()> {
    let amount = params.amount;

    msg!("Validate inputs");
    if amount == 0 {
        return Err(CustomError::InvalidAmount.into());
    }

    // Transfer tokens from the user to the custodian
    token::transfer(ctx.accounts.into_transfer_context(), amount)?;

    let deposit_account = &ctx.accounts.deposit_account;
    let pda_auth = &mut ctx.accounts.pda_auth;

    let seeds = &[
        b"mint-authority",
        pda_auth.to_account_info().key.as_ref(),
        &[deposit_account.admin_treasury_vault_bump.unwrap()],
    ];

    let signer = &[&seeds[..]];

    // Mint wrapped tokens to the user's wrapped token account
    token::mint_to(
        ctx.accounts.into_mint_to_context().with_signer(signer),
        amount,
    )?;

    Ok(())
}
