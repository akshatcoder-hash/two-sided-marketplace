use anchor_lang::prelude::*;

#[error_code]
pub enum MarketplaceError {
    #[msg("The provided fee percentage must be between 0 and 100")]
    InvalidFeePercentage,
    #[msg("The provided price must be greater than 0")]
    InvalidPrice,
    #[msg("The service name must not be empty")]
    EmptyServiceName,
    #[msg("The service description must not be empty")]
    EmptyServiceDescription,
    #[msg("This NFT is soulbound and cannot be transferred")]
    SoulboundNonTransferable,
    #[msg("Insufficient funds to purchase the service")]
    InsufficientFunds,
    #[msg("The owner of the service listing is invalid")]
    InvalidOwner,
}