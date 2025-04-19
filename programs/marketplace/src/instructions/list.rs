




use anchor_lang::prelude::*;

use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{Mint,TokenInterface,TokenAccount,TransferChecked,transfer_checked,}
};
use anchor_spl::metadata::{Metadata,MetadataAccount,MasterEditionAccount};

use crate::state::listing::Listing;
use crate::state::marketplace::Marketplace;


#[derive(Accounts)]
pub struct List<'info>{

    //signer
    #[account(mut)]
    pub maker:Signer<'info>,

    // //marketplace account 
    #[account(
        seeds =[b"marketplace",marketplace.name.as_bytes()],
        bump= marketplace.bump,
    )]
    pub marketplace:Account<'info,Marketplace>,

    pub maker_mint:InterfaceAccount<'info, Mint>,

    #[account(
        mut,
        associated_token::mint = maker_mint, //verify the mint is the same as the maker mint 
        associated_token::authority = maker, //verify that its the makers ata
        associated_token::token_program =token_program
    )]
    pub maker_ata: InterfaceAccount<'info, TokenAccount>,

    //valut to store the nft
    #[account(
        init ,
        payer = maker,
        associated_token::mint = maker_mint,
        associated_token::authority = listing,
        associated_token::token_program = token_program
    )]
    pub vault:InterfaceAccount<'info, TokenAccount>,


    //metadata account
     #[account(
        seeds = [b"metadata",
        metadata_program.key().as_ref(),
        maker_mint.key().as_ref(),
         ],
        seeds::program= metadata_program.key(),   //set the metadata seeds program
        //verify the collection is the same as the one in the metadata
        constraint=metadata.collection.as_ref().unwrap().key.as_ref() == collection_mint.key().as_ref(),
        //verify the collection is verified
        constraint=metadata.collection.as_ref().unwrap().verified,
        bump
    )]
    pub metadata: Account<'info,MetadataAccount>,

    // //collection mint
    pub collection_mint:Account<'info,MetadataAccount >,

    // //create a listing account 
    #[account(
        init,
        payer = maker,
        seeds = [marketplace.key().as_ref(),maker_mint.key().as_ref(),maker.key().as_ref()],
        bump,
        space = 8+ Listing::INIT_SPACE
    )]
    pub listing:Account<'info, Listing>,

    pub metadata_program:Program<'info, Metadata>,

    #[account(
        seeds = [b"metadata",
        metadata_program.key().as_ref(),
        maker_mint.key().as_ref(),
        b"edition"
        ],
        seeds::program = metadata_program.key(),
        bump,
    )]
    pub master_edition:Account<'info,MasterEditionAccount>,
    
    
    pub associated_token_program:Program<'info,AssociatedToken>,
    pub token_program:Interface<'info,TokenInterface>,
    pub system_program:Program<'info,System>

}

impl<'info> List<'info> {
    pub fn create_listing(&mut self,price:u64,bumps:&ListBumps)->Result<()>{


    //     self.listing.set_inner(Listing { 
    //         maker: self.maker.key(),
    //         mint:self.maker_mint.key(),
    //         price,
    //         bump:bumps.listing}
    //     );

        Ok(())
    }


    pub fn deposit_nft(&mut self)->Result<()>{

        let cpi_program = self.token_program.to_account_info();


        let cpi_accounts = TransferChecked{
            from:self.maker_ata.to_account_info(),
            mint: self.maker_mint.to_account_info(),
            to:self.vault.to_account_info(),
            authority:self.maker.to_account_info(),
        };

        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        transfer_checked (cpi_ctx,1, self.maker_mint.decimals);
        
        Ok(())
    }
}