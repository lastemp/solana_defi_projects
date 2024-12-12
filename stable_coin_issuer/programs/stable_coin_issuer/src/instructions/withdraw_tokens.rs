use crate::{error::CustomError, state::deposit_base::DepositBase};
use anchor_lang::{prelude::*, system_program};
use anchor_spl::token::{self, Burn, Mint, MintTo, Token, TokenAccount, Transfer};

// Context for withdrawing tokens
#[derive(Accounts)]
#[instruction(params: WithdrawTokensParams)]
pub struct WithdrawTokens<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    //#[account(mut)]
    //pub custodian_token_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub wrapped_mint: Account<'info, Mint>,
    #[account(mut)]
    pub user_wrapped_token_account: Account<'info, TokenAccount>,
    //#[account(mut)]
    //pub user_token_account: Account<'info, TokenAccount>,
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
    /*
    fn into_transfer_back_context(&self) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        CpiContext::new(
            self.token_program.to_account_info(),
            Transfer {
                from: self.custodian_token_account.to_account_info(),
                to: self.user_token_account.to_account_info(),
                authority: self.user.to_account_info(),
            },
        )
    }
    */
    /*
    fn getSigner(&self) -> &[&[&[u8]]; 1] {
        let seeds = &[
            b"treasury-vault",
            self.pda_auth.to_account_info().key.as_ref(),
            &[self.deposit_account.admin_treasury_vault_bump.unwrap()],
        ];

        let signer = &[&seeds[..]];
        signer
    }

    fn into_transfer_back_context(
        &self,
    ) -> CpiContext<'_, '_, '_, 'info, system_program::Transfer<'info>> {
        /*
        let seeds = &[
            b"treasury-vault",
            self.pda_auth.to_account_info().key.as_ref(),
            &[self.deposit_account.admin_treasury_vault_bump.unwrap()],
        ];

        let signer = &[&seeds[..]];
        */
        let signer = self.getSigner();
        CpiContext::new_with_signer(
            self.system_program.to_account_info(),
            system_program::Transfer {
                from: self.treasury_vault.to_account_info(),
                to: self.user.to_account_info(),
            },
            signer,
        )
    }
    */
}

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct WithdrawTokensParams {
    pub deposit_amount: u64,
    pub stable_coin_amount: u64,
}

// Burn wrapped tokens and release original sol
pub fn withdraw_tokens(ctx: Context<WithdrawTokens>, params: &WithdrawTokensParams) -> Result<()> {
    let deposit_amount = params.deposit_amount;
    let stable_coin_amount = params.stable_coin_amount;

    msg!("Validate inputs");
    if deposit_amount == 0 {
        return Err(CustomError::InvalidAmount.into());
    }

    if stable_coin_amount == 0 {
        return Err(CustomError::InvalidAmount.into());
    }

    // Burn wrapped tokens from the user's wrapped token account
    token::burn(ctx.accounts.into_burn_context(), stable_coin_amount)?;

    // Transfer sol back from the treasury-vault to the user
    //token::transfer(ctx.accounts.into_transfer_back_context(), amount)?;
    //system_program::transfer(ctx.accounts.into_transfer_back_context(), amount)?;
    let sys_program = &ctx.accounts.system_program;
    let deposit_account = &ctx.accounts.deposit_account;
    let pda_auth = &mut ctx.accounts.pda_auth;
    let treasury_vault = &mut ctx.accounts.treasury_vault;
    let cpi_accounts = system_program::Transfer {
        from: treasury_vault.to_account_info(),
        to: ctx.accounts.user.to_account_info(),
    };

    let seeds = &[
        b"treasury-vault",
        pda_auth.to_account_info().key.as_ref(),
        &[deposit_account.admin_treasury_vault_bump.unwrap()],
    ];

    let signer = &[&seeds[..]];

    let cpi = CpiContext::new_with_signer(sys_program.to_account_info(), cpi_accounts, signer);

    system_program::transfer(cpi, deposit_amount)?;

    Ok(())
}
