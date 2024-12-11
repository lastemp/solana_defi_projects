//! MintCNft instruction handler

use mpl_bubblegum::instructions::{
    MintToCollectionV1Cpi, MintToCollectionV1CpiAccounts, MintToCollectionV1InstructionArgs,
};
use mpl_bubblegum::types::{Collection, Creator, MetadataArgs, TokenProgramVersion, TokenStandard};
//use spl_account_compression::Noop;
use {
    crate::{
        error::CustomError,
        state::configs::{MplBubblegum, MplTokenMetadata, Noop, SPLCompression},
    },
    anchor_lang::prelude::*,
};

#[derive(Accounts)]
#[instruction(params: MintCNftParams)]
pub struct MintCNft<'info> {
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
    /// CHECK: This account is checked in the instruction
    #[account(
        seeds = [b"collection-cpi"],
        seeds::program = bubblegum_program.key(),
        bump,
    )]
    pub bubblegum_signer: UncheckedAccount<'info>,
    /// CHECK: This account is checked in the instruction
    pub collection_mint: UncheckedAccount<'info>,
    #[account(mut)]
    /// CHECK: This account is checked in the instruction
    pub collection_metadata: UncheckedAccount<'info>,
    /// CHECK: This account is checked in the instruction
    pub edition_account: UncheckedAccount<'info>,
    pub log_wrapper: Program<'info, Noop>,
    pub compression_program: Program<'info, SPLCompression>,
    pub bubblegum_program: Program<'info, MplBubblegum>,
    pub token_metadata_program: Program<'info, MplTokenMetadata>,
    pub system_program: Program<'info, System>,
}

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct MintCNftParams {
    pub name: String,
    pub symbol: String,
    pub uri: String,
}

const NAME_LENGTH: usize = 20;
const SYMBOL_LENGTH: usize = 10;
const URI_LENGTH: usize = 200;

pub fn mint_cnft(ctx: Context<MintCNft>, params: &MintCNftParams) -> Result<()> {
    msg!("Validate inputs");

    if params.name.as_bytes().len() > 0 && params.name.as_bytes().len() <= NAME_LENGTH {
    } else {
        return Err(CustomError::InvalidNameLength.into());
    }

    if params.symbol.as_bytes().len() > 0 && params.symbol.as_bytes().len() <= SYMBOL_LENGTH {
    } else {
        return Err(CustomError::InvalidSymbolLength.into());
    }

    if params.uri.as_bytes().len() > 0 && params.uri.as_bytes().len() <= URI_LENGTH {
    } else {
        return Err(CustomError::InvalidUriLength.into());
    }

    let _name = &params.name;
    let _symbol = &params.symbol;
    let _uri = &params.uri;

    let signer_seeds: &[&[&[u8]]] = &[&[b"auth", &[ctx.bumps.pda]]];

    // instruction accounts
    let bubblegum_program = ctx.accounts.bubblegum_program.to_account_info();
    let tree_config = ctx.accounts.tree_authority.to_account_info();
    let merkle_tree = ctx.accounts.merkle_tree.to_account_info();
    let collection_mint = ctx.accounts.collection_mint.to_account_info();
    let collection_metadata = ctx.accounts.collection_metadata.to_account_info();
    let edition_account = ctx.accounts.edition_account.to_account_info();
    let bubblegum_signer = ctx.accounts.bubblegum_signer.to_account_info();
    let payer = ctx.accounts.payer.to_account_info();
    let pda = ctx.accounts.pda.to_account_info(); // set creator as pda
    let log_wrapper = ctx.accounts.log_wrapper.to_account_info();
    let compression_program = ctx.accounts.compression_program.to_account_info();
    let token_metadata_program = ctx.accounts.token_metadata_program.to_account_info();
    let system_program = ctx.accounts.system_program.to_account_info();

    let metadata = MetadataArgs {
        name: _name.to_string(),
        symbol: _symbol.to_string(),
        uri: _uri.to_string(),
        seller_fee_basis_points: 0,
        primary_sale_happened: true,
        is_mutable: true,
        edition_nonce: None,
        token_standard: Some(TokenStandard::NonFungible),
        collection: Some(Collection {
            key: ctx.accounts.collection_mint.key(),
            verified: false,
        }),
        uses: None,
        token_program_version: TokenProgramVersion::Original,
        creators: vec![Creator {
            address: ctx.accounts.pda.key(), // set creator as pda
            verified: true,
            share: 100,
        }],
    };

    let cpi_create_tree_config = MintToCollectionV1Cpi::new(
        &bubblegum_program,
        MintToCollectionV1CpiAccounts {
            tree_config: &tree_config,
            leaf_owner: &payer,
            leaf_delegate: &payer,
            merkle_tree: &merkle_tree,
            payer: &payer,
            tree_creator_or_delegate: &pda, // tree delegate is pda, required as a signer
            collection_authority: &pda, // collection authority is pda (nft metadata update authority)
            collection_authority_record_pda: Some(&bubblegum_program),
            collection_mint: &collection_mint, // collection nft mint account
            collection_metadata: &collection_metadata, // collection nft metadata account
            collection_edition: &edition_account, // collection nft master edition account
            bubblegum_signer: &bubblegum_signer,
            log_wrapper: &log_wrapper,
            compression_program: &compression_program,
            token_metadata_program: &token_metadata_program,
            system_program: &system_program,
        },
        MintToCollectionV1InstructionArgs { metadata },
    );

    // performs the CPI
    let _result = cpi_create_tree_config.invoke_signed(signer_seeds);

    Ok(())
}
