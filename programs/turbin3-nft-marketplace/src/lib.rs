use anchor_lang::prelude::*;

// Unique identifier for this NFT trading platform
declare_id!("BUbjGkkoUYWVjDWYn9Yp9qgDaspMAt3rm91J7KKZpLZv");

pub mod instructions;
use instructions::*;
pub mod state;
use state::*;

#[program]
pub mod nft_marketplace {
    use super::*;

    // Sets up a new NFT trading platform with specified configuration
    pub fn initialize(ctx: Context<Initialize>, name: String, fee: u16) -> Result<()> {
        ctx.accounts.init(name, fee, &ctx.bumps)?;
        Ok(())
    }

    // Allows users to put their NFTs up for sale on the platform
    pub fn listing(ctx: Context<List>, name: String, price: u64) -> Result<()> {
        ctx.accounts.create_listing(price, &ctx.bumps)?;
        ctx.accounts.deposit_nft()?;
        Ok(())
    }

    // Enables sellers to remove their NFTs from the marketplace
    pub fn delisting(ctx: Context<Delist>) -> Result<()> {
        ctx.accounts.withdraw_nft()?;
        Ok(())
    }

    // Handles the complete NFT purchase transaction including payment and transfer
    pub fn purchase(ctx: Context<Purchase>) -> Result<()> {
        ctx.accounts.send_sol()?;
        ctx.accounts.send_nft()?;
        ctx.accounts.close_mint_vault()
    }
}