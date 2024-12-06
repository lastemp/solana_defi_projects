//! CreateSwapContract instruction handler

use {
    crate::{
        error::CustomError,
        state::{derivative_contract::ContractType, derivative_contract::DerivativeContract},
    },
    anchor_lang::prelude::*,
};

#[derive(Accounts)]
#[instruction(params: CreateSwapContractParams)]
pub struct CreateSwapContract<'info> {
    #[account(mut,has_one = owner,
        constraint = derivative_contract.is_initialized @ CustomError::AccountNotInitialized
    )]
    pub derivative_contract: Account<'info, DerivativeContract>,
    // mut makes it changeble (mutable)
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct CreateSwapContractParams {
    pub notional_amount: u64,
    pub fixed_rate: u64,
    pub floating_rate: u64,
    pub buyer: Pubkey,
    pub seller: Pubkey,
}

pub fn create_swap_contract(
    ctx: Context<CreateSwapContract>,
    params: &CreateSwapContractParams,
) -> Result<()> {
    let contract = &mut ctx.accounts.derivative_contract;
    contract.contract_type = ContractType::Swaps;
    contract.notional_amount = Some(params.notional_amount);
    contract.fixed_rate = Some(params.fixed_rate);
    contract.floating_rate = Some(params.floating_rate);
    contract.buyer = Some(params.buyer);
    contract.seller = Some(params.seller);
    Ok(())
}
