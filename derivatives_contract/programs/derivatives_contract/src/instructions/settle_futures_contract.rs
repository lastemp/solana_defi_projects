//! SettleFuturesContract instruction handler

use {
    crate::{
        error::CustomError,
        state::{deposit_base::DepositBase, derivative_contract::DerivativeContract},
    },
    anchor_lang::prelude::*,
    anchor_spl::{
        associated_token::AssociatedToken,
        token::{transfer_checked, Mint, Token, TokenAccount, TransferChecked},
    },
};

#[derive(Accounts)]
#[instruction(params: SettleFuturesContractParams)]
pub struct SettleFuturesContract<'info> {
    #[account(mut,has_one = owner,
        constraint = derivative_contract.is_initialized @ CustomError::AccountNotInitialized,
    )]
    pub derivative_contract: Account<'info, DerivativeContract>,
    #[account(mut)]
    pub sender_tokens: Account<'info, TokenAccount>,
    #[account(mut)]
    pub recipient_tokens: Account<'info, TokenAccount>,
    #[account(mut)]
    pub mint_token: Account<'info, Mint>,
    #[account(mut,
        constraint = deposit_account.is_initialized @ CustomError::AccountNotInitialized
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
    pub token_program: Program<'info, Token>,
    pub associate_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct SettleFuturesContractParams {
    pub amount: u32,
    pub buyer: Pubkey,
}

pub fn settle_futures_contract(
    ctx: Context<SettleFuturesContract>,
    params: &SettleFuturesContractParams,
) -> Result<()> {
    msg!("Validate inputs");
    if params.amount == 0 {
        return Err(CustomError::InvalidAmount.into());
    }

    let derivative_contract = &mut ctx.accounts.derivative_contract;
    let sender_tokens = &ctx.accounts.sender_tokens;
    let recipient_tokens = &ctx.accounts.recipient_tokens;
    let mint_token = &ctx.accounts.mint_token;
    let deposit_account = &ctx.accounts.deposit_account;
    let pda_auth = &mut ctx.accounts.pda_auth;
    let treasury_vault = &mut ctx.accounts.treasury_vault;
    let token_program = &ctx.accounts.token_program;
    let decimals: u8 = derivative_contract.decimals;
    let _amount = params.amount;

    let buyer = match derivative_contract.buyer {
        Some(buyer) => buyer,
        None => return Err(CustomError::BuyerNotFound.into()),
    };

    if params.buyer != buyer {
        return Err(CustomError::InvalidBuyer.into());
    }

    // _buyer gets asset
    // _seller gets sol

    let base: u32 = 10;
    let exponent = derivative_contract.decimals as u32;
    // lets get the amount in decimal format
    // 10 ** 9 * 3(base 10, 9 decimals, 3 amount), // 3 amount of token to transfer (in smallest unit i.e 9 decimals)
    let result = (base).pow(exponent);
    let _amount = (_amount as u64)
        .checked_mul(result as u64)
        .ok_or(CustomError::InvalidArithmeticOperation)?;

    // Transfer funds from treasury vault to recipient
    let cpi_accounts = TransferChecked {
        from: sender_tokens.to_account_info(),
        mint: mint_token.to_account_info(),
        to: recipient_tokens.to_account_info(),
        authority: treasury_vault.to_account_info(),
    };

    let seeds = &[
        b"treasury-vault",
        pda_auth.to_account_info().key.as_ref(),
        &[deposit_account.admin_treasury_vault_bump.unwrap()],
    ];

    let signer = &[&seeds[..]];

    let cpi = CpiContext::new_with_signer(token_program.to_account_info(), cpi_accounts, signer);

    transfer_checked(cpi, _amount, decimals)?;

    Ok(())
}
