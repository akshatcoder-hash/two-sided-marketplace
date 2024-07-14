use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, SetAuthority};
use anchor_spl::associated_token::AssociatedToken;
use spl_token::instruction::AuthorityType;
use crate::state::{Marketplace, ServiceListing};
use crate::errors::MarketplaceError;

#[derive(Accounts)]
pub struct PurchaseService<'info> {
    #[account(mut)]
    pub buyer: Signer<'info>,
    #[account(mut)]
    pub vendor: SystemAccount<'info>,
    #[account(mut)]
    pub service_listing: Account<'info, ServiceListing>,
    pub marketplace: Account<'info, Marketplace>,
    #[account(mut)]
    pub buyer_token_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub vendor_token_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub marketplace_token_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub mint: Account<'info, token::Mint>,
    #[account(
        mut,
        constraint = buyer_nft_token_account.owner == buyer.key() @ MarketplaceError::InvalidOwner
    )]
    pub buyer_nft_token_account: Account<'info, TokenAccount>,
    #[account(
        mut,
        associated_token::mint = mint,
        associated_token::authority = vendor
    )]
    pub vendor_nft_token_account: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

pub fn handler(ctx: Context<PurchaseService>) -> Result<()> {
    let service_listing = &mut ctx.accounts.service_listing;
    let marketplace = &ctx.accounts.marketplace;

    let price = service_listing.price;
    let fee_amount = (price * marketplace.fee_percentage as u64) / 100;
    let vendor_amount = price - fee_amount;

    // Transfer payment from buyer to vendor
    token::transfer(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            token::Transfer {
                from: ctx.accounts.buyer_token_account.to_account_info(),
                to: ctx.accounts.vendor_token_account.to_account_info(),
                authority: ctx.accounts.buyer.to_account_info(),
            },
        ),
        vendor_amount,
    )?;

    // Transfer fee to marketplace
    if fee_amount > 0 {
        token::transfer(
            CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                token::Transfer {
                    from: ctx.accounts.buyer_token_account.to_account_info(),
                    to: ctx.accounts.marketplace_token_account.to_account_info(),
                    authority: ctx.accounts.buyer.to_account_info(),
                },
            ),
            fee_amount,
        )?;
    }

    // Transfer NFT from vendor to buyer
    token::transfer(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            token::Transfer {
                from: ctx.accounts.vendor_nft_token_account.to_account_info(),
                to: ctx.accounts.buyer_nft_token_account.to_account_info(),
                authority: ctx.accounts.vendor.to_account_info(),
            },
        ),
        1,
    )?;

    // If the NFT is soulbound, freeze it
    if service_listing.is_soulbound {
        let freeze_authority = ctx.accounts.vendor.key();
        token::set_authority(
            CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                SetAuthority {
                    current_authority: ctx.accounts.vendor.to_account_info(),
                    account_or_mint: ctx.accounts.mint.to_account_info(),
                },
            ),
            AuthorityType::FreezeAccount,
            Some(freeze_authority),
        )?;

        token::set_authority(
            CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                SetAuthority {
                    current_authority: ctx.accounts.buyer.to_account_info(),
                    account_or_mint: ctx.accounts.buyer_nft_token_account.to_account_info(),
                },
            ),
            AuthorityType::CloseAccount,
            Some(freeze_authority),
        )?;
    }

    // Update service listing
    service_listing.vendor = ctx.accounts.buyer.key();

    Ok(())
}