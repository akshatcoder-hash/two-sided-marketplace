use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Mint, SetAuthority};
use spl_token::instruction::AuthorityType;
use crate::state::ServiceListing;
use crate::errors::MarketplaceError;

#[derive(Accounts)]
pub struct ResellService<'info> {
    #[account(mut)]
    pub seller: Signer<'info>,
    #[account(
        mut,
        constraint = service_listing.vendor == seller.key() @ MarketplaceError::SoulboundNonTransferable
    )]
    pub service_listing: Account<'info, ServiceListing>,
    #[account(mut)]
    pub mint: Account<'info, Mint>,
    #[account(
        mut,
        associated_token::mint = mint,
        associated_token::authority = seller
    )]
    pub seller_token_account: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
}

pub fn handler(ctx: Context<ResellService>, price: u64) -> Result<()> {
    require!(price > 0, MarketplaceError::InvalidPrice);

    let service_listing = &mut ctx.accounts.service_listing;

    require!(!service_listing.is_soulbound, MarketplaceError::SoulboundNonTransferable);

    // Update the price
    service_listing.price = price;

    // Unfreeze the token account if it was frozen
    let unfreeze_token_account = SetAuthority {
        current_authority: ctx.accounts.seller.to_account_info(),
        account_or_mint: ctx.accounts.seller_token_account.to_account_info(),
    };
    let cpi_ctx = CpiContext::new(
        ctx.accounts.token_program.to_account_info(),
        unfreeze_token_account,
    );
    token::set_authority(cpi_ctx, AuthorityType::CloseAccount, None)?;

    // Remove freeze authority from the mint
    let unfreeze_mint = SetAuthority {
        current_authority: ctx.accounts.seller.to_account_info(),
        account_or_mint: ctx.accounts.mint.to_account_info(),
    };
    let cpi_ctx = CpiContext::new(
        ctx.accounts.token_program.to_account_info(),
        unfreeze_mint,
    );
    token::set_authority(cpi_ctx, AuthorityType::FreezeAccount, None)?;

    Ok(())
}