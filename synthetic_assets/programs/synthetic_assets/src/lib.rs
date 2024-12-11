pub mod error;
pub mod instructions;
pub mod state;

use {anchor_lang::prelude::*, instructions::*};

declare_id!("71s43VMpGrDiNtWLGYCfSfUqdtLhsr9YoL8APGhfjNU3");

#[program]
pub mod synthetic_assets {
    use super::*;

    // admin instructions
    pub fn create_tree(ctx: Context<CreateTree>, params: CreateTreeParams) -> Result<()> {
        instructions::create_tree(ctx, &params)
    }

    pub fn mint_cnft(ctx: Context<MintCNft>, params: MintCNftParams) -> Result<()> {
        instructions::mint_cnft(ctx, &params)
    }
}
