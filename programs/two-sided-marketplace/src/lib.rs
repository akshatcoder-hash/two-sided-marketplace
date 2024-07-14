use anchor_lang::prelude::*;

declare_id!("AamrL6gaYNEiFhRreM1JJTb9ZH4uBeNcryuW6Lp5FFR8");

#[program]
pub mod two_sided_marketplace {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
