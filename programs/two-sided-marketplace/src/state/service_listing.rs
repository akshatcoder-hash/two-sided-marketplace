use anchor_lang::prelude::*;

#[account]
pub struct ServiceListing {
    pub vendor: Pubkey,
    pub name: String,
    pub description: String,
    pub price: u64,
    pub is_soulbound: bool,
    pub nft_mint: Pubkey,
}

impl ServiceListing {
    pub const LEN: usize = 8 + 32 + 4 + 200 + 4 + 400 + 8 + 1 + 32; // discriminator + pubkey + string len + max name len + string len + max description len + u64 + bool + pubkey
}