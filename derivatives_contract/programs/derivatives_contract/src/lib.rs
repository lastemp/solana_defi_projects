pub mod error;
pub mod instructions;
pub mod state;

use {anchor_lang::prelude::*, instructions::*};

declare_id!("HjQAsSTgfHJZrEgwtDTztwaphTqVjgQ148JZinYrj6VD");

#[program]
pub mod derivatives_contract {
    use super::*;

    // admin instructions
    pub fn init(ctx: Context<Init>, params: InitParams) -> Result<()> {
        instructions::init(ctx, &params)
    }

    // public instructions: add sol transfer by buyer and seller sends asset
    pub fn create_futures_contract(
        ctx: Context<CreateFuturesContract>,
        params: CreateFuturesContractParams,
    ) -> Result<()> {
        instructions::create_futures_contract(ctx, &params)
    }

    pub fn create_token(ctx: Context<CreateToken>, params: CreateTokenParams) -> Result<()> {
        instructions::create_token(ctx, &params)
    }

    pub fn transfer_token(ctx: Context<TransferToken>, params: TransferTokenParams) -> Result<()> {
        instructions::transfer_token(ctx, &params)
    }

    pub fn deposit_asset(ctx: Context<DepositAsset>, params: DepositAssetParams) -> Result<()> {
        instructions::deposit_asset(ctx, &params)
    }

    // _buyer gets asset
    // _seller gets sol
    pub fn settle_futures_contract(
        ctx: Context<SettleFuturesContract>,
        params: SettleFuturesContractParams,
    ) -> Result<()> {
        instructions::settle_futures_contract(ctx, &params)
    }
}
