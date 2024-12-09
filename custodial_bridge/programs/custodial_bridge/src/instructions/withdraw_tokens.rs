use anchor_lang::prelude::*;
use anchor_spl::token::{self, Burn, Mint, MintTo, Token, TokenAccount, Transfer};

// Context for withdrawing tokens
#[derive(Accounts)]
#[instruction(params: WithdrawTokensParams)]
pub struct WithdrawTokens<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(mut)]
    pub custodian_token_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub wrapped_mint: Account<'info, Mint>,
    #[account(mut)]
    pub user_wrapped_token_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub user_token_account: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
}

impl<'info> WithdrawTokens<'info> {
    fn into_burn_context(&self) -> CpiContext<'_, '_, '_, 'info, Burn<'info>> {
        CpiContext::new(
            self.token_program.to_account_info(),
            Burn {
                mint: self.wrapped_mint.to_account_info(),
                from: self.user_wrapped_token_account.to_account_info(),
                authority: self.user.to_account_info(),
            },
        )
    }

    fn into_transfer_back_context(&self) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        CpiContext::new(
            self.token_program.to_account_info(),
            Transfer {
                from: self.custodian_token_account.to_account_info(),
                to: self.user_token_account.to_account_info(),
                authority: self.custodian_token_account.to_account_info(),
            },
        )
    }
}

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct WithdrawTokensParams {
    pub amount: u64,
}

// Burn wrapped tokens and release original tokens
pub fn withdraw_tokens(ctx: Context<WithdrawTokens>, params: &WithdrawTokensParams) -> Result<()> {
    let amount = params.amount;
    // Burn wrapped tokens from the user's wrapped token account
    token::burn(ctx.accounts.into_burn_context(), amount)?;

    // Transfer tokens back from the custodian to the user
    token::transfer(ctx.accounts.into_transfer_back_context(), amount)?;

    Ok(())
}
