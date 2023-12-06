use crate::errors::SolactivityError;
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

    pub fn setup(&mut self, author: Pubkey, program: Pubkey, name: String, group: String, sub_group: String, score: i32) -> Result<()> {
        require!(name.len() <= 34, SolactivityError::NameTooLong);
        require!(group.len() <= 8, SolactivityError::GroupTooLong);
        require!(sub_group.len() <= 18, SolactivityError::SubGroupTooLong);

        self.author = author;
        self.program = program;
        self.name = name;
        self.group = group;
        self.sub_group = sub_group;
        self.score = score;
        
        msg!(
            "Created proposal by:{} for program:{}",
            self.author,
            self.program
        );
        Ok(())
    }

    pub fn increment_score(&mut self, nb: i32) {
        self.score += nb;
    }
}
