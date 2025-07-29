use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]

// Core marketplace configuration and state management
pub struct Marketplace {
    pub admin: Pubkey,                    // Platform administrator with management privileges
    pub fee: u16,                         // Transaction fee in basis points (e.g., 250 = 2.5%)
    pub bump: u8,                         // PDA bump seed for marketplace account
    pub treasury_bump: u8,                // PDA bump seed for treasury account
    pub rewards_bump: u8,                 // PDA bump seed for rewards token mint
    #[max_len(32)]
    pub name: String,                     // Marketplace display name (max 32 characters)

}