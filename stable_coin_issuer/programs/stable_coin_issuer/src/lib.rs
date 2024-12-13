pub mod error;
pub mod instructions;
pub mod state;

use {anchor_lang::prelude::*, instructions::*};

declare_id!("2HNQZmuxm5zFZu64rzBjPj4DKCRCVwcc7cTvP8KgQ1wq");

#[program]
pub mod stable_coin_issuer {
    use super::*;

    // admin instructions
    pub fn init(ctx: Context<Init>) -> Result<()> {
        instructions::init(ctx)
    }

    // public instructions
    pub fn deposit(ctx: Context<Deposit>, params: DepositParams) -> Result<()> {
        instructions::deposit(ctx, &params)
    }

    pub fn withdraw(ctx: Context<Withdrawal>, params: WithdrawalParams) -> Result<()> {
        instructions::withdraw(ctx, &params)
    }
}

#[derive(Accounts)]
pub struct Initialize {}
