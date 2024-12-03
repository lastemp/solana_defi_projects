use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct DerivativeContract {
    pub owner: Pubkey, // publickey of the admin
    pub contract_type: ContractType,
    pub expiry_date: i64,
    pub underlying_asset: Pubkey,
    pub price: u64,
    pub buyer: Option<Pubkey>,
    pub seller: Option<Pubkey>,
    pub custodian: Option<Pubkey>, // New field
    pub option_type: Option<OptionType>,
    pub notional_amount: Option<u64>,
    pub fixed_rate: Option<u64>,
    pub floating_rate: Option<u64>,
    pub is_initialized: bool, // is derivative contract initialized
    pub decimals: u8,         // decimals for the token mint
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq, InitSpace)]
pub enum ContractType {
    Futures,
    Options,
    Swaps,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq, InitSpace)]
pub enum OptionType {
    Call,
    Put,
}
