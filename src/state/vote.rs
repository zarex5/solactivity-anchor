use anchor_lang::prelude::*;

#[account] //8 + 65 = 73
pub struct Vote {
    pub author: Pubkey,   //32
    pub proposal: Pubkey, //32
    pub positive: bool,   //1
}
