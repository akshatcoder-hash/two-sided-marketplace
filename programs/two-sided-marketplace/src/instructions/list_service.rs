use anchor_lang::prelude::*;
use anchor_spl::{
    token::{Mint, Token},
    metadata::{
        create_metadata_accounts_v3,
        mpl_token_metadata::types::DataV2,
        Metadata,
    },
};
use crate::state::ServiceListing;
use crate::errors::MarketplaceError;

#[derive(Accounts)]
pub struct ListService<'info> {
    #[account(mut)]
    pub vendor: Signer<'info>,
    #[account(
        init,
        payer = vendor,
        space = ServiceListing::LEN
    )]
    pub service_listing: Account<'info, ServiceListing>,
    #[account(
        init,
        payer = vendor,
        mint::decimals = 0,
        mint::authority = vendor,
        mint::freeze_authority = vendor,
    )]
    pub nft_mint: Account<'info, Mint>,
    /// CHECK: This is safe as it's checked in the CPI to token metadata program
    #[account(mut)]
    pub metadata: UncheckedAccount<'info>,
    pub token_program: Program<'info, Token>,
    pub token_metadata_program: Program<'info, Metadata>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

pub fn handler(
    ctx: Context<ListService>,
    name: String,
    description: String,
    price: u64,
    is_soulbound: bool,
) -> Result<()> {
    require!(!name.is_empty(), MarketplaceError::EmptyServiceName);
    require!(!description.is_empty(), MarketplaceError::EmptyServiceDescription);
    require!(price > 0, MarketplaceError::InvalidPrice);

    let service_listing = &mut ctx.accounts.service_listing;
    service_listing.vendor = ctx.accounts.vendor.key();
    service_listing.name = name.clone();
    service_listing.description = description.clone();
    service_listing.price = price;
    service_listing.is_soulbound = is_soulbound;
    service_listing.nft_mint = ctx.accounts.nft_mint.key();

    // Generate a simple on-chain URI
    let uri = format!("data:application/json,{{\"name\":\"{}\",\"description\":\"{}\",\"price\":{},\"is_soulbound\":{}}}", 
    name, description, price, is_soulbound);

    // Create metadata for the NFT
    create_metadata_accounts_v3(
        CpiContext::new(
            ctx.accounts.token_metadata_program.to_account_info(),
            anchor_spl::metadata::CreateMetadataAccountsV3 {
                metadata: ctx.accounts.metadata.to_account_info(),
                mint: ctx.accounts.nft_mint.to_account_info(),
                mint_authority: ctx.accounts.vendor.to_account_info(),
                payer: ctx.accounts.vendor.to_account_info(),
                update_authority: ctx.accounts.vendor.to_account_info(),
                system_program: ctx.accounts.system_program.to_account_info(),
                rent: ctx.accounts.rent.to_account_info(),
            },
        ),
        DataV2 {
            name,
            symbol: "SVC".to_string(),
            uri,
            seller_fee_basis_points: 0,
            creators: None,
            collection: None,
            uses: None,
        },
        true,
        true,
        None,
    )?;

    Ok(())
}