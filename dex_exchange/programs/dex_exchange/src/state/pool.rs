use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Pool {
    pub owner: Pubkey, // publickey of the pool admin
    pub token_a: Pubkey,
    pub token_b: Pubkey,
    pub reserve_a: u32,
    pub reserve_b: u32,
    pub is_initialized: bool, // is pool initialized
    #[max_len(10)]
    pub liquidity_providers: Vec<Pubkey>, // list of the liquidity providers
    pub decimals: u8,         // decimals for the token mint
}
