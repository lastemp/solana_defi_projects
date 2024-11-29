//! Swap instruction handler

use {
    crate::{
        error::DexError,
        state::{pool::Pool, trader::Trader},
    },
    anchor_lang::prelude::*,
    anchor_spl::{
        associated_token::AssociatedToken,
        token::{transfer, Mint, Token, TokenAccount, Transfer},
    },
};

#[derive(Accounts)]
#[instruction(params: SwapParams)]
pub struct Swap<'info> {
    #[account(mut,
        constraint = liquidity_pool.is_initialized @ DexError::AccountNotInitialized
    )]
    pub liquidity_pool: Account<'info, Pool>,
    #[account(mut,has_one = owner,
        constraint = trader.active @ DexError::InvalidTraderStatus
    )]
    pub trader: Account<'info, Trader>,
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
pub struct SwapParams {
    pub amount_in: u32,
    pub token_in: Pubkey,
}

pub fn swap(ctx: Context<Swap>, params: &SwapParams) -> Result<()> {
    msg!("Validate inputs");
    if params.amount_in == 0 {
        return Err(DexError::InvalidAmount.into());
    }

    let sender = &ctx.accounts.owner;
    let sender_tokens = &ctx.accounts.sender_tokens;
    let recipient_tokens = &ctx.accounts.recipient_tokens;
    let token_program = &ctx.accounts.token_program;
    let liquidity_pool = &mut ctx.accounts.liquidity_pool;
    let trader = &mut ctx.accounts.trader;
    let token_a_reserve = liquidity_pool.reserve_a;
    let token_b_reserve = liquidity_pool.reserve_b;
    let decimals = liquidity_pool.decimals as u64;
    let amount_in = params.amount_in;
    let mut is_token_a = false;

    // Determine the token being swapped and adjust reserves accordingly
    let (reserve_in, reserve_out) = if params.token_in == liquidity_pool.token_a {
        is_token_a = true;
        (token_a_reserve, token_b_reserve)
    } else {
        (token_b_reserve, token_a_reserve)
    };

    let amount_out = (reserve_out * amount_in) / (reserve_in + amount_in); // Simplified AMM calculation

    if is_token_a {
        // Increment reserve_a with amount
        liquidity_pool.reserve_a = token_a_reserve
            .checked_add(amount_in)
            .ok_or(DexError::InvalidArithmeticOperation)?;

        // Decrement reserve_b with amount
        liquidity_pool.reserve_b = token_b_reserve
            .checked_sub(amount_out)
            .ok_or(DexError::InvalidArithmeticOperation)?;
    } else {
        // Increment reserve_b with amount
        liquidity_pool.reserve_b = token_b_reserve
            .checked_add(amount_in)
            .ok_or(DexError::InvalidArithmeticOperation)?;

        // Decrement reserve_a with amount
        liquidity_pool.reserve_a = token_a_reserve
            .checked_sub(amount_out)
            .ok_or(DexError::InvalidArithmeticOperation)?;
    }

    let base: u32 = 10;
    let exponent = liquidity_pool.decimals as u32;

    // lets get the amount in decimal format
    // 10 ** 9 * 3(base 10, 9 decimals, 3 amount), // 3 amount of token to transfer (in smallest unit i.e 9 decimals)
    let result = (base).pow(exponent);
    let _amount = (amount_out as u64)
        .checked_mul(result as u64)
        .ok_or(DexError::InvalidArithmeticOperation)?;

    // tests
    trader.amount_out = amount_out;
    trader.amount_out_2 = _amount;

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
