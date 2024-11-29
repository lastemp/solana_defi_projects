use anchor_lang::prelude::*;

#[account]
#[derive(Default, Debug, InitSpace)]
pub struct Trader {
    pub owner: Pubkey, // publickey of the trader
    pub active: bool,  // status of trader
    pub amount_out: u32,
    pub amount_out_2: u64,
}
