use anchor_lang::prelude::*;
use crate::state::Marketplace;
use crate::errors::MarketplaceError;

#[derive(Accounts)]
pub struct InitializeMarketplace<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    #[account(
        init,
        payer = authority,
        space = Marketplace::LEN,
        seeds = [b"marketplace"],
        bump
    )]
    pub marketplace: Account<'info, Marketplace>,
    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<InitializeMarketplace>, fee_percentage: u8) -> Result<()> {
    require!(fee_percentage <= 100, MarketplaceError::InvalidFeePercentage);

    let marketplace = &mut ctx.accounts.marketplace;
    marketplace.authority = ctx.accounts.authority.key();
    marketplace.fee_percentage = fee_percentage;

    Ok(())
}