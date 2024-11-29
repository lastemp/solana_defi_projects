use anchor_lang::prelude::*;

#[account]
#[derive(Default, Debug, InitSpace)]
pub struct LiquidityProvider {
    pub owner: Pubkey,                  // publickey of the liquidity provider
    pub active: bool,                   // status of liquidity provider
    pub reserve_a_available_funds: u32, // reserve_a available funds
    pub reserve_b_available_funds: u32, // reserve_b available funds
}
