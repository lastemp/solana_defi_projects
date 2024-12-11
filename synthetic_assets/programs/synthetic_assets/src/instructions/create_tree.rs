//! CreateTree instruction handler

use mpl_bubblegum::instructions::{
    CreateTreeConfigCpi, CreateTreeConfigCpiAccounts, CreateTreeConfigInstructionArgs,
};
//use spl_account_compression::Noop;
use {
    crate::{
        error::CustomError,
        state::configs::{MplBubblegum, Noop, SPLCompression},
    },
    anchor_lang::prelude::*,
};

#[derive(Accounts)]
#[instruction(params: CreateTreeParams)]
pub struct CreateTree<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(
        seeds = [b"auth"],
        bump,
    )]
    /// CHECK: This account is checked in the instruction
    pub pda: UncheckedAccount<'info>,
    #[account(
        mut,
        seeds = [merkle_tree.key().as_ref()],
        bump,
        seeds::program = bubblegum_program.key()
    )]
    /// CHECK: This account is checked in the instruction
    pub tree_authority: UncheckedAccount<'info>,
    #[account(mut)]
    /// CHECK: This account is checked in the instruction
    pub merkle_tree: UncheckedAccount<'info>,
    pub log_wrapper: Program<'info, Noop>,
    pub compression_program: Program<'info, SPLCompression>,
    pub bubblegum_program: Program<'info, MplBubblegum>,
    pub system_program: Program<'info, System>,
}

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct CreateTreeParams {
    // max_depth - used to compute the maximum number of leaves
    // i.e Compressed NFTs that the Merkle Tree can hold
    pub max_depth: u32,
    // max_buffer_size - indicates the minimum concurrency limit of the Merkle Tree
    pub max_buffer_size: u32,
    pub public: bool,
}

pub fn create_tree(ctx: Context<CreateTree>, params: &CreateTreeParams) -> Result<()> {
    msg!("Validate inputs");

    if params.max_depth == 0 {
        return Err(CustomError::InvalidMaxDepth.into());
    }

    if params.max_buffer_size == 0 {
        return Err(CustomError::InvalidMaxBufferSize.into());
    }

    let public = Some(params.public);
    let max_depth = params.max_depth;
    let max_buffer_size = params.max_buffer_size;

    let signer_seeds: &[&[&[u8]]] = &[&[b"auth", &[ctx.bumps.pda]]];

    // instruction accounts
    let bubblegum_program = ctx.accounts.bubblegum_program.to_account_info();
    let tree_config = ctx.accounts.tree_authority.to_account_info();
    let merkle_tree = ctx.accounts.merkle_tree.to_account_info();
    let payer = ctx.accounts.payer.to_account_info();
    let tree_creator = ctx.accounts.pda.to_account_info(); // set creator as pda
    let log_wrapper = ctx.accounts.log_wrapper.to_account_info();
    let compression_program = ctx.accounts.compression_program.to_account_info();
    let system_program = ctx.accounts.system_program.to_account_info();

    let cpi_create_tree_config = CreateTreeConfigCpi::new(
        &bubblegum_program,
        CreateTreeConfigCpiAccounts {
            tree_config: &tree_config,
            merkle_tree: &merkle_tree,
            payer: &payer,
            tree_creator: &tree_creator,
            log_wrapper: &log_wrapper,
            compression_program: &compression_program,
            system_program: &system_program,
        },
        CreateTreeConfigInstructionArgs {
            max_depth,
            max_buffer_size,
            public,
        },
    );

    // performs the CPI
    let _result = cpi_create_tree_config.invoke_signed(signer_seeds);

    Ok(())
}
