use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Listing{

    pub maker:Pubkey, //maker - the one is listing

    pub mint : Pubkey,  //mint of the nft

    pub price :u64, // price

    pub bump :u8, //bymp of listing 

}

