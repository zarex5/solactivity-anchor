use anchor_lang::prelude::*;

#[account]
pub struct Proposal {
    pub author: Pubkey,    // 32
    pub program: Pubkey,   // 32
    pub name: String,      // 4 + 34
    pub group: String,     // 4 + 8
    pub sub_group: String, // 4 + 18
    pub score: i32,        // 4
}

impl Proposal {
    pub const MAXIMUM_SIZE: usize = 32 + 32 + (4 + 34) + (4 + 8) + (4 + 18) + 4;
}