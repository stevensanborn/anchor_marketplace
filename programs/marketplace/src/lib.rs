#![allow(unexpected_cfgs)]
use anchor_lang::prelude::*;
mod instructions;
mod state;
pub use instructions::initialize::*;
pub use instructions::list::*;
pub use instructions::purchase::*;
// use crate::instructions::Initialize;

declare_id!("CFupJ1UdaSgyHcLJDJTXmM8HYCkfCcmbJMSsMRmRfxXb");

#[program]
pub mod marketplace {
    use super::*;

    pub fn initialize_market(ctx:Context<Initialize>,name:String, fee:u16 )->Result<()>{
        ctx.accounts.initialize_market(name, fee , &ctx.bumps)?;
        Ok(())
    }
    


    pub fn listing(ctx:Context<List> , price:u64)->Result<()>{
        ctx.accounts.create_listing(price,&ctx.bumps)?;
        ctx.accounts.deposit_nft()?;
        Ok(())
    }

    
    pub fn purchase(ctx:Context<Purchase>)->Result<()>{

        ctx.accounts.purchase()?;
        ctx.accounts.close_vault()
    
    }
}