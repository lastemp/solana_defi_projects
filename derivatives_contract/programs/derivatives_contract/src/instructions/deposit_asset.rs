//! DepositAsset instruction handler

use {
    crate::{error::CustomError, state::derivative_contract::DerivativeContract},
    anchor_lang::prelude::*,
    anchor_spl::{
        associated_token::AssociatedToken,
        token::{transfer, Mint, Token, TokenAccount, Transfer},
    },
};

#[derive(Accounts)]
#[instruction(params: DepositAssetParams)]
pub struct DepositAsset<'info> {
    #[account(mut,
        constraint = derivative_contract.is_initialized @ CustomError::AccountNotInitialized
    )]
    pub derivative_contract: Account<'info, DerivativeContract>,
    #[account(mut)]
    pub sender_tokens: Account<'info, TokenAccount>,
    #[account(mut)]
    pub recipient_tokens: Account<'info, TokenAccount>,
    #[account(mut)]
    pub mint_token: Account<'info, Mint>,
    // mut makes it changeble (mutable)
    #[account(mut)]
    pub owner: Signer<'info>,
    pub token_program: Program<'info, Token>,
    pub associate_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct DepositAssetParams {
    pub amount: u32,
}

pub fn deposit_asset(ctx: Context<DepositAsset>, params: &DepositAssetParams) -> Result<()> {
    msg!("Validate inputs");
    if params.amount == 0 {
        return Err(CustomError::InvalidAmount.into());
    }

    let sender = &ctx.accounts.owner;
    let sender_tokens = &ctx.accounts.sender_tokens;
    let recipient_tokens = &ctx.accounts.recipient_tokens;
    let token_program = &ctx.accounts.token_program;
    let derivative_contract = &mut ctx.accounts.derivative_contract;
    let decimals = derivative_contract.decimals as u64;
    let amount_ = params.amount;

    let base: u32 = 10;
    let exponent = derivative_contract.decimals as u32;

    // lets get the amount in decimal format
    // 10 ** 9 * 3(base 10, 9 decimals, 3 amount), // 3 amount of token to transfer (in smallest unit i.e 9 decimals)
    let result = (base).pow(exponent);
    let amount_ = (amount_ as u64)
        .checked_mul(result as u64)
        .ok_or(CustomError::InvalidArithmeticOperation)?;

    // token
    transfer(
        CpiContext::new(
            token_program.to_account_info(),
            Transfer {
                from: sender_tokens.to_account_info(),
                to: recipient_tokens.to_account_info(),
                authority: sender.to_account_info(),
            },
        ),
        amount_,
    )?;

    Ok(())
}
