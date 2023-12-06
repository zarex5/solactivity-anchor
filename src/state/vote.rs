use crate::errors::SolactivityError;
use anchor_lang::prelude::*;

#[account]
pub struct Vote {
    pub author: Pubkey,   // 32
    pub proposal: Pubkey, // 32
    pub positive: bool,   // 1
}

impl Vote {
    pub const MAXIMUM_SIZE: usize = 32 + 32 + 1;

    pub fn setup(&mut self, author: Pubkey, proposal: Pubkey, positive: bool) -> Result<()> {
        self.author = author;
        self.proposal = proposal;
        self.positive = positive;
        
        msg!(
            "Created vote by:{} for proposal:{}",
            self.author,
            self.proposal
        );
        Ok(())
    }

    pub fn change_vote(&mut self, positive: bool) -> Result<()> {
        if self.positive == positive {
            return if positive {
                err!(SolactivityError::AlreadyUpvoted)
            } else {
                err!(SolactivityError::AlreadyDownvoted)
            };
        }
        self.positive = positive;

        msg!("Changed vote to positive:{}", self.positive);
        Ok(())
    }
}