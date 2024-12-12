use crate::{error::CustomError, state::deposit_base::DepositBase};
use anchor_lang::{prelude::*, system_program};
use anchor_spl::token::{self, Burn, Mint, MintTo, Token, TokenAccount, Transfer};

// Context for depositing tokens
#[derive(Accounts)]
#[instruction(params: DepositTokensParams)]
pub struct DepositTokens<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    //#[account(mut)]
    //pub user_token_account: Account<'info, TokenAccount>,
    //#[account(mut)]
    //pub custodian_token_account: Account<'info, TokenAccount>,
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
    #[account(mut, seeds = [b"treasury-vault", pda_auth.key().as_ref()], bump = deposit_account.admin_treasury_vault_bump.unwrap())]
    pub treasury_vault: SystemAccount<'info>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

impl<'info> DepositTokens<'info> {
    /*
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
    */
    fn into_transfer_context(
        &self,
    ) -> CpiContext<'_, '_, '_, 'info, system_program::Transfer<'info>> {
        CpiContext::new(
            self.system_program.to_account_info(),
            system_program::Transfer {
                from: self.user.to_account_info(),
                to: self.treasury_vault.to_account_info(),
            },
        )
    }

    fn into_mint_to_context(&self) -> CpiContext<'_, '_, '_, 'info, MintTo<'info>> {
        CpiContext::new(
            self.token_program.to_account_info(),
            MintTo {
                mint: self.wrapped_mint.to_account_info(),
                to: self.user_wrapped_token_account.to_account_info(),
                authority: self.treasury_vault.to_account_info(),
            },
        )
    }
}

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct DepositTokensParams {
    pub deposit_amount: u64,
    pub stable_coin_amount: u64,
}

// Deposit sol and mint wrapped tokens
pub fn deposit_tokens(ctx: Context<DepositTokens>, params: &DepositTokensParams) -> Result<()> {
    let deposit_amount = params.deposit_amount;
    let stable_coin_amount = params.stable_coin_amount;

    msg!("Validate inputs");
    if deposit_amount == 0 {
        return Err(CustomError::InvalidAmount.into());
    }

    if stable_coin_amount == 0 {
        return Err(CustomError::InvalidAmount.into());
    }

    // Transfer sol from the user to the treasury-vault
    //token::transfer(ctx.accounts.into_transfer_context(), amount)?;
    system_program::transfer(ctx.accounts.into_transfer_context(), deposit_amount)?;

    let deposit_account = &ctx.accounts.deposit_account;
    let pda_auth = &mut ctx.accounts.pda_auth;

    let seeds = &[
        b"treasury-vault",
        pda_auth.to_account_info().key.as_ref(),
        &[deposit_account.admin_treasury_vault_bump.unwrap()],
    ];

    let signer = &[&seeds[..]];

    // Mint wrapped tokens to the user's wrapped token account
    token::mint_to(
        ctx.accounts.into_mint_to_context().with_signer(signer),
        stable_coin_amount,
    )?;

    Ok(())
}
