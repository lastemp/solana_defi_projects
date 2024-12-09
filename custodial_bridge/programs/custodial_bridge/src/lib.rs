//use anchor_lang::prelude::*;
//pub mod error;
pub mod instructions;
//pub mod state;

use {anchor_lang::prelude::*, instructions::*};

declare_id!("4ZGK14JeopQVp3xkxYgDC9DLTSdufFz87rjxP7wGkmCj");

#[program]
pub mod custodial_bridge {
    use super::*;

    // admin instructions
    /*
    pub fn init(ctx: Context<Init>, params: InitParams) -> Result<()> {
        instructions::init(ctx, &params)
    }
    */

    // public instructions
    pub fn deposit_tokens(ctx: Context<DepositTokens>, params: DepositTokensParams) -> Result<()> {
        instructions::deposit_tokens(ctx, &params)
    }

    pub fn withdraw_tokens(
        ctx: Context<WithdrawTokens>,
        params: WithdrawTokensParams,
    ) -> Result<()> {
        instructions::withdraw_tokens(ctx, &params)
    }
}
