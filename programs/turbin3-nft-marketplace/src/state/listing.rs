use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
// Individual NFT sale listing data structure
pub struct Listing { 
    pub maker: Pubkey,                    // Original seller who created this listing
    pub maker_mint: Pubkey,               // Unique identifier of the NFT being sold
    pub price: u64,                       // Sale price in lamports (SOL)
    pub bump: u8,                         // PDA bump seed for this listing account
}