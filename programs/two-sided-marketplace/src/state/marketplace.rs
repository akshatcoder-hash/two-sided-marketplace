use anchor_lang::prelude::*;

#[account]
pub struct Marketplace {
    pub authority: Pubkey,
    pub fee_percentage: u8,
}

impl Marketplace {
    pub const LEN: usize = 8 + 32 + 1; // discriminator + pubkey + u8
}