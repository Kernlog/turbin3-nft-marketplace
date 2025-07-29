use anchor_lang::prelude::*;
use anchor_spl::token_interface::{Mint, TokenInterface};

use crate::state::marketplace::Marketplace;

#[derive(Accounts)]
#[instruction(name: String)] // Instruction parameter for marketplace name

pub struct Initialize<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,

    #[account(
        init,
        payer = admin,
        seeds = [b"marketplace", name.as_str().as_bytes()], // Creates deterministic address based on marketplace name
        bump,
        space = Marketplace::INIT_SPACE, 
    )]
    pub marketplace: Account<'info, Marketplace>,

    // Treasury account for collecting platform fees
    #[account(
        seeds = [b"treasury", marketplace.key().as_ref()],
        bump
    )]
    pub treasury: SystemAccount<'info>,

    // Reward token mint for platform incentives
    #[account(
        init,
        payer = admin,
        seeds = [b"rewards", marketplace.key().as_ref()],
        bump,
        mint::decimals = 6,
        mint::authority = marketplace,
    )]
    pub rewards_mint: InterfaceAccount<'info, Mint>,

    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
}
    

impl<'info> Initialize<'info> {
    // Configures the marketplace with initial settings and parameters
    pub fn init(&mut self, name: String, fee: u16, bumps: &InitializeBumps ) -> Result <()> {
        
        self.marketplace.set_inner(Marketplace {
            admin: self.admin.key(),
            fee,
            bump: bumps.marketplace,
            treasury_bump: bumps.treasury,
            rewards_bump: bumps.rewards_mint,
            name
        });
        
        Ok(())
    }
}