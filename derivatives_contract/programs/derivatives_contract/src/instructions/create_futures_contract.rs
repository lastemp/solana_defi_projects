//! CreateFuturesContract instruction handler

use {
    crate::{
        error::CustomError,
        state::{derivative_contract::ContractType, derivative_contract::DerivativeContract},
    },
    anchor_lang::prelude::*,
};

#[derive(Accounts)]
#[instruction(params: CreateFuturesContractParams)]
pub struct CreateFuturesContract<'info> {
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
pub struct CreateFuturesContractParams {
    pub expiry_date: i64,
    pub underlying_asset: Pubkey,
    pub price: u64,
    pub buyer: Pubkey,
    pub seller: Pubkey,
}

pub fn create_futures_contract(
    ctx: Context<CreateFuturesContract>,
    params: &CreateFuturesContractParams,
) -> Result<()> {
    let contract = &mut ctx.accounts.derivative_contract;
    contract.contract_type = ContractType::Futures;
    contract.expiry_date = params.expiry_date;
    contract.underlying_asset = params.underlying_asset;
    contract.price = params.price;
    contract.buyer = Some(params.buyer);
    contract.seller = Some(params.seller);
    Ok(())
}
