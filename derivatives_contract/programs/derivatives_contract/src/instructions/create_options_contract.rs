//! CreateOptionsContract instruction handler

use {
    crate::{
        error::CustomError,
        state::{
            derivative_contract::ContractType, derivative_contract::DerivativeContract,
            derivative_contract::OptionType,
        },
    },
    anchor_lang::prelude::*,
};

#[derive(Accounts)]
#[instruction(params: CreateOptionsContractParams)]
pub struct CreateOptionsContract<'info> {
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
pub struct CreateOptionsContractParams {
    pub expiry_date: i64,
    pub underlying_asset: Pubkey,
    pub strike_price: u64,
    pub option_type: OptionType,
    pub buyer: Pubkey,
    pub seller: Pubkey,
}

pub fn create_options_contract(
    ctx: Context<CreateOptionsContract>,
    params: &CreateOptionsContractParams,
) -> Result<()> {
    let contract = &mut ctx.accounts.derivative_contract;
    contract.contract_type = ContractType::Options;
    contract.expiry_date = params.expiry_date;
    contract.underlying_asset = params.underlying_asset;
    contract.price = params.strike_price;
    contract.option_type = Some(params.option_type);
    contract.buyer = Some(params.buyer);
    contract.seller = Some(params.seller);
    Ok(())
}
