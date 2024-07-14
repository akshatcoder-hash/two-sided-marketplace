use anchor_lang::prelude::*;

declare_id!("AamrL6gaYNEiFhRreM1JJTb9ZH4uBeNcryuW6Lp5FFR8");

pub mod state;
pub mod instructions;
pub mod errors;

use instructions::*;

#[program]
pub mod two_sided_marketplace {
    use super::*;

    pub fn initialize_marketplace(ctx: Context<InitializeMarketplace>, fee_percentage: u8) -> Result<()> {
        instructions::initialize_marketplace::handler(ctx, fee_percentage)
    }

    pub fn list_service(
        ctx: Context<ListService>,
        name: String,
        description: String,
        price: u64,
        is_soulbound: bool,
    ) -> Result<()> {
        instructions::list_service::handler(ctx, name, description, price, is_soulbound)
    }

    pub fn purchase_service(ctx: Context<PurchaseService>) -> Result<()> {
        instructions::purchase_service::handler(ctx)
    }

    pub fn resell_service(ctx: Context<ResellService>, price: u64) -> Result<()> {
        instructions::resell_service::handler(ctx, price)
    }
}