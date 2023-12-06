use crate::errors::SolactivityError;
use anchor_lang::prelude::*;

#[account]
pub struct Proposal {
    author: Pubkey,    // 32
    program: Pubkey,   // 32
    name: String,      // 4 + 34
    group: String,     // 4 + 8
    sub_group: String, // 4 + 18
    score: i32,        // 4
}

impl Proposal {
    pub const MAXIMUM_SIZE: usize = 32 + 32 + (4 + 34) + (4 + 8) + (4 + 18) + 4;

    pub fn author(&self) -> &Pubkey {
        &self.author
    }

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
