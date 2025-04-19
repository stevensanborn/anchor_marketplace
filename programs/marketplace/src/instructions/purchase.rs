
use anchor_lang::prelude::*;
use anchor_spl::{
    token_interface::{transfer_checked, Mint, TokenAccount, TokenInterface, TransferChecked,CloseAccount,close_account},
};
use anchor_spl::metadata::{Metadata,MetadataAccount,MasterEditionAccount};
use crate::state::marketplace::Marketplace;
use crate::state::listing::Listing;

#[derive(Accounts)]
pub struct Purchase<'info>{

    #[account(mut)]
    pub taker:Signer<'info>,

    #[account(mut)]
    pub maker:SystemAccount<'info>,

    //marketplace
    #[account(
        seeds =[b"marketplace",marketplace.name.as_bytes()],
        bump=marketplace.bump,
    )]
    pub marketplace:Account<'info,Marketplace>,


    pub maker_mint:InterfaceAccount<'info, Mint>,
    #[account(
        mut,
        associated_token::mint = maker_mint, //verify the mint is the same as the maker mint
        associated_token::authority = listing, //verify that its the makers ata
    )]
    pub vault:InterfaceAccount<'info, TokenAccount>,

    #[account(
        mut,
        seeds=[b"treasury",marketplace.key().as_ref()],
    bump)]
    pub treasury:SystemAccount<'info>,

    #[account(
        mut,
        seeds = [b"rewards", marketplace.key().as_ref()],
        mint::authority = marketplace,
        bump= marketplace.rewards_bump
    )]
    pub rewards_mint:InterfaceAccount<'info, Mint>,


    #[account(
        mut,
        seeds = [marketplace.key().as_ref(),maker_mint.key().as_ref()],
        bump,
        close = maker, //close the listing to the maker
    )]
    pub listing:Account<'info, Listing>,



    pub token_program:Interface<'info,TokenInterface>,
    pub system_program:Program<'info,System>

}

//initialze 
impl<'info> Purchase<'info> {

    pub fn purchase(&mut self)->Result<()>{
        let seeds= &[
            &self.marketplace.key().to_bytes()[..],
            &self.maker_mint.key().to_bytes()[..],
            &self.maker.key().to_bytes()[..],
            &[self.listing.bump]
            ];
        let signer = &[&seeds[..]];

        //transfer the nft from the vault to the taker
        let cpi_accounts = TransferChecked {
            from: self.vault.to_account_info().clone(),
            to: self.taker.to_account_info().clone(),
            authority: self.listing.to_account_info().clone(),
            mint: self.maker_mint.to_account_info().clone(),
        };
        let cpi_program = self.token_program.to_account_info();
        let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts,signer);
        transfer_checked(cpi_ctx, 1, self.maker_mint.decimals)?;
        Ok(())
    }

    pub fn close_vault(&mut self)->Result<()>{

      
            let seeds= &[
                &self.marketplace.key().to_bytes()[..],
                &self.maker_mint.key().to_bytes()[..],
                &self.maker.key().to_bytes()[..],
                &[self.listing.bump]
                ];
            let signer = &[&seeds[..]];

            let cpi_program = self.token_program.to_account_info();

            let close_accounts = CloseAccount{
                account: self.vault.to_account_info(),
                destination: self.taker.to_account_info(),
                authority: self.listing.to_account_info(),
            };
            let cpi_ctx = CpiContext::new_with_signer(cpi_program, close_accounts,signer);
            //close the vault
            close_account(cpi_ctx)
    }
    
}

