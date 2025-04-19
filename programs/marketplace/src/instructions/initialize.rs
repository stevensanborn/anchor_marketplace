
use anchor_lang::prelude::*;
use anchor_spl::token_interface::{Mint,TokenInterface};
use crate::state::marketplace::Marketplace;

#[derive(Accounts)]
#[instruction(name:String)]
pub struct Initialize<'info>{

    #[account(mut)]
    pub admin:Signer<'info>,

    #[account(init , payer = admin, 
        seeds =[b"marketplace",name.as_bytes()],
        space=Marketplace::INIT_SPACE,
        bump,
    )]
    pub marketplace:Account<'info,Marketplace>,

    #[account(
        seeds=[b"treasury", marketplace.key().as_ref()],
        bump)]
    pub treasury:SystemAccount<'info>,

    #[account(
        init,
        payer = admin,
        seeds = [b"rewards", marketplace.key().as_ref()],
        bump,
        mint::authority = marketplace,
        mint::decimals=6
    )]
    pub rewards_mint:InterfaceAccount<'info, Mint>, //mint for giving rewards
    pub token_program:Interface<'info,TokenInterface>, 
    pub system_program:Program<'info,System>

}

//initialze 
impl<'info> Initialize<'info> {
    // use crate::state::marketplace::Marketplace;
    pub fn initialize_market(&mut self, name: String , fee :u16, bumps: &InitializeBumps)->Result<()>{

        assert!(name.len() < 32 );

        self.marketplace.set_inner(Marketplace{
            name,
            admin:self.admin.key(),
            fee,
            bump:bumps.marketplace,
            treasury_bump:bumps.treasury,
            rewards_bump:bumps.rewards_mint
        });

        Ok(())
    }
}