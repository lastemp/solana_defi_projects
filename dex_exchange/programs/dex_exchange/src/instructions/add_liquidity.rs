//! AddLiquidity instruction handler

use {
    crate::{
        error::DexError,
        state::{liquidity_provider::LiquidityProvider, pool::Pool},
    },
    anchor_lang::prelude::*,
    anchor_spl::{
        associated_token::AssociatedToken,
        token::{transfer, Mint, Token, TokenAccount, Transfer},
    },
};

#[derive(Accounts)]
#[instruction(params: AddLiquidityParams)]
pub struct AddLiquidity<'info> {
    #[account(mut,
        constraint = liquidity_pool.is_initialized @ DexError::AccountNotInitialized
    )]
    pub liquidity_pool: Account<'info, Pool>,
    #[account(mut,has_one = owner,
        constraint = liquidity_provider.active @ DexError::InvalidLiquidityProviderStatus
    )]
    pub liquidity_provider: Account<'info, LiquidityProvider>,
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
pub struct AddLiquidityParams {
    pub amount_a: u32,
    pub amount_b: u32,
}

pub fn add_liquidity(ctx: Context<AddLiquidity>, params: &AddLiquidityParams) -> Result<()> {
    msg!("Validate inputs");
    if params.amount_a == 0 {
        return Err(DexError::InvalidAmount.into());
    }

    if params.amount_b == 0 {
        return Err(DexError::InvalidAmount.into());
    }

    let sender = &ctx.accounts.owner;
    let sender_tokens = &ctx.accounts.sender_tokens;
    let recipient_tokens = &ctx.accounts.recipient_tokens;
    let token_program = &ctx.accounts.token_program;
    let liquidity_pool = &mut ctx.accounts.liquidity_pool;
    let liquidity_provider = &mut ctx.accounts.liquidity_provider;
    let token_a_reserve = liquidity_pool.reserve_a;
    let token_b_reserve = liquidity_pool.reserve_b;
    let reserve_a_available_funds = liquidity_provider.reserve_a_available_funds;
    let reserve_b_available_funds = liquidity_provider.reserve_b_available_funds;
    let decimals = liquidity_pool.decimals as u64;
    let amount_a = params.amount_a;
    let amount_b = params.amount_b;

    // Increment reserve_a with amount_a
    liquidity_pool.reserve_a = token_a_reserve
        .checked_add(amount_a)
        .ok_or(DexError::InvalidArithmeticOperation)?;

    // Increment reserve_b with amount_b
    liquidity_pool.reserve_b = token_b_reserve
        .checked_add(amount_b)
        .ok_or(DexError::InvalidArithmeticOperation)?;

    // Increment available_funds with new amount
    liquidity_provider.reserve_a_available_funds = reserve_a_available_funds
        .checked_add(amount_a)
        .ok_or(DexError::InvalidArithmeticOperation)?;

    // Increment available_funds with new amount
    liquidity_provider.reserve_b_available_funds = reserve_b_available_funds
        .checked_add(amount_b)
        .ok_or(DexError::InvalidArithmeticOperation)?;

    let base: u32 = 10;
    let exponent = liquidity_pool.decimals as u32;

    // lets get the amount in decimal format
    // 10 ** 9 * 3(base 10, 9 decimals, 3 amount), // 3 amount of token to transfer (in smallest unit i.e 9 decimals)
    let result = (base).pow(exponent);
    let _amount = (amount_a as u64)
        .checked_mul(result as u64)
        .ok_or(DexError::InvalidArithmeticOperation)?;

    // check if liquidity provider exists before adding
    liquidity_pool.liquidity_providers.push(*sender.key);

    transfer(
        CpiContext::new(
            token_program.to_account_info(),
            Transfer {
                from: sender_tokens.to_account_info(),
                to: recipient_tokens.to_account_info(),
                authority: sender.to_account_info(),
            },
        ),
        _amount,
    )?;

    Ok(())
}
