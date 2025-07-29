use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    metadata::{MasterEditionAccount, Metadata, MetadataAccount},
    token_interface::{transfer_checked, TransferChecked, Mint, TokenAccount, TokenInterface},
};

use crate::state::{listing::Listing, marketplace::Marketplace};

#[derive(Accounts)]
#[instruction(name: String)]

pub struct List<'info> {
    #[account(mut)]
    pub maker: Signer<'info>,

    #[account(
        seeds = [b"marketplace", name.as_str().as_bytes()],
        bump = marketplace.bump,
    )]
    pub marketplace: Account<'info, Marketplace>,

    pub maker_mint: InterfaceAccount<'info, Mint>,

    #[account(
        mut,
        associated_token::mint = maker_mint,
        associated_token::authority = maker,
    )]
    pub maker_ata: InterfaceAccount<'info, TokenAccount>,
    
    // Creates a new listing account to track the NFT sale
    #[account(
        init,
        payer = maker,
        seeds = [marketplace.key().as_ref(), maker_mint.key().as_ref()], // Links listing to marketplace and specific NFT
        bump,
        space = 8 + Listing::INIT_SPACE,
    )]
    pub listing: Account<'info, Listing>,

    // Escrow vault that temporarily holds the NFT during listing
    #[account(
        init,
        payer = maker,
        associated_token::mint = maker_mint,
        associated_token::authority = listing,
    )]
    pub vault: InterfaceAccount<'info, TokenAccount>,

    pub collection_mint: InterfaceAccount<'info, Mint>,

    // Validates NFT metadata and collection verification
    #[account(                                                              
        seeds = [                                                           
            b"metadata",                                                    
            metadata_program.key().as_ref(),                                
            maker_mint.key().as_ref(),                                      
        ],                                                                  
        seeds::program = metadata_program.key(),                            
        bump,                                                               
        constraint = metadata.collection.as_ref().unwrap().key.as_ref() == collection_mint.key().as_ref(), // Ensures NFT belongs to specified collection
        constraint = metadata.collection.as_ref().unwrap().verified == true,                               // Verifies collection authenticity
    )]                                                                   
    pub metadata: Account<'info, MetadataAccount>,                     

    // Validates NFT's master edition for authenticity
    #[account(                                                              
        seeds = [                                                           
            b"metadata",                                                    
            metadata_program.key().as_ref(),                                
            maker_mint.key().as_ref(),                                      
            b"edition",                                                     
        ],                                                                  
        seeds::program = metadata_program.key(),                            
        bump,                                                               
    )]                                                                  
    pub master_edition: Account<'info, MasterEditionAccount>,         

    pub token_program: Interface<'info, TokenInterface>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
    pub metadata_program: Program<'info, Metadata>,
}

impl<'info> List<'info> {
    // Establishes the listing record with price and seller information
    pub fn create_listing(&mut self, price: u64, bumps: &ListBumps) -> Result<()> {

        self.listing.set_inner(Listing { 
            maker: self.maker.key(), 
            maker_mint: self.maker_mint.key(), 
            price,
            bump: bumps.listing,
        });
        Ok(())
    }

    // Transfers NFT from seller's wallet to escrow vault
    pub fn deposit_nft(&mut self) -> Result<()> {

        let cpi_program = self.token_program.to_account_info();

        let cpi_accounts = TransferChecked { 
            from: self.maker_ata.to_account_info(), 
            mint: self.maker_mint.to_account_info(), 
            to: self.vault.to_account_info(),
            authority: self.maker.to_account_info(), 
        };

        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);

        transfer_checked(cpi_ctx, self.maker_ata.amount, self.maker_mint.decimals)?; // Securely transfers NFT to escrow with decimal verification

        Ok(())
    }

}