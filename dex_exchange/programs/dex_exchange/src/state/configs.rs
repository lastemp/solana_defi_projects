//use crate::state::lottery_operator::LotteryOperator;
use anchor_lang::prelude::*;

#[account]
#[derive(Default, InitSpace)]
pub struct DexConfigs {
    //#[max_len(5)]
    //pub operators: Vec<LotteryOperator>,
    pub is_initialized: bool,
}
