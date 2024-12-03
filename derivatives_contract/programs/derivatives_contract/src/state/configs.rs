use anchor_lang::prelude::*;

#[account]
#[derive(Default, InitSpace)]
pub struct Configs {
    pub is_initialized: bool,
}
