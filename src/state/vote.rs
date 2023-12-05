use anchor_lang::prelude::*;

#[account]
pub struct Vote {
    pub author: Pubkey,   // 32
    pub proposal: Pubkey, // 32
    pub positive: bool,   // 1
}

impl Vote {
    pub const MAXIMUM_SIZE: usize = 32 + 32 + 1;
}