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
                authority: self.custodian_token_account.to_account_info(),
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
    // Transfer tokens from the user to the custodian
    token::transfer(ctx.accounts.into_transfer_context(), amount)?;

    // Mint wrapped tokens to the user's wrapped token account
    token::mint_to(ctx.accounts.into_mint_to_context(), amount)?;

    Ok(())
}
