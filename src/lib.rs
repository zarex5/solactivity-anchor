use anchor_lang::prelude::*;
use solana_program::{pubkey, pubkey::Pubkey};

declare_id!("acTiJkzfuF6vx8Z6GvH4JcZEWCyztU3M5L6BsQDzfNa");

const ADMIN_PUBKEY: Pubkey = pubkey!("ANcHrHbApAcPqfYPs3WegLytaQhsHt6UYAGVsDjJeuhX");

#[program]
// Smart contract functions
pub mod solactivity {
    use super::*;

    pub fn create_proposal(
        ctx: Context<CreateProposal>,
        name: String,
        group: String,
        sub_group: String,
    ) -> Result<()> {
        require!(name.len() <= 34, CustomError::NameTooLong);
        require!(group.len() <= 8, CustomError::GroupTooLong);
        require!(sub_group.len() <= 18, CustomError::SubGroupTooLong);
        let proposal = &mut ctx.accounts.proposal;
        proposal.author = ctx.accounts.author.key();
        proposal.program = ctx.accounts.program.key();
        proposal.name = name;
        proposal.group = group;
        proposal.sub_group = sub_group;
        proposal.score = 0;
        msg!(
            "Created proposal by:{} for program:{}",
            proposal.author,
            proposal.program
        );
        Ok(())
    }

    pub fn delete_proposal(ctx: Context<DeleteProposal>) -> Result<()> {
        let signer = &mut ctx.accounts.signer;
        let proposal = &mut ctx.accounts.proposal;
        if signer.key() != proposal.author.key() && signer.key() != ADMIN_PUBKEY.key() {
            return err!(CustomError::NotAuthorOrAdmin);
        }
        //TODO: Delete all votes associated with the proposal? (+allow proposal owner to delete votes on its proposal)
        msg!("Deleting proposal!");
        Ok(())
    }

    pub fn create_vote(ctx: Context<CreateVote>, positive: bool) -> Result<()> {
        let vote = &mut ctx.accounts.vote;
        vote.author = ctx.accounts.author.key();
        vote.proposal = ctx.accounts.proposal.key();
        vote.positive = positive;
        let proposal = &mut ctx.accounts.proposal;
        proposal.score += if positive { 1 } else { -1 };
        msg!(
            "Created vote by:{} for proposal:{}",
            vote.author,
            vote.proposal
        );
        Ok(())
    }

    pub fn change_vote(ctx: Context<ChangeVote>, positive: bool) -> Result<()> {
        let vote = &mut ctx.accounts.vote;
        if vote.positive == positive {
            return if positive {
                err!(CustomError::AlreadyUpvoted)
            } else {
                err!(CustomError::AlreadyDownvoted)
            };
        }
        vote.positive = positive;
        let proposal = &mut ctx.accounts.proposal;
        proposal.score += if positive { 2 } else { -2 };
        msg!("Changed vote to positive:{}", vote.positive);
        Ok(())
    }

    pub fn delete_vote(ctx: Context<DeleteVote>) -> Result<()> {
        let signer = &mut ctx.accounts.signer;
        let vote = &mut ctx.accounts.vote;
        if signer.key() != vote.author.key() && signer.key() != ADMIN_PUBKEY.key() {
            return err!(CustomError::NotAuthorOrAdmin);
        }
        let proposal = &mut ctx.accounts.proposal;
        proposal.score += if vote.positive { -1 } else { 1 };
        msg!("Deleting vote!");
        Ok(())
    }
}

// Data validators
#[derive(Accounts)]
pub struct CreateProposal<'info> {
    #[account(mut)]
    author: Signer<'info>,
    #[account(executable)]
    /// CHECK: Any program address is okay (we don't read or write from this account)
    program: UncheckedAccount<'info>,
    #[account(
        init,
        seeds = [author.key().as_ref(), program.key().as_ref()],
        bump,
        payer = author,
        space = 148
    )]
    proposal: Account<'info, Proposal>,
    system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct DeleteProposal<'info> {
    signer: Signer<'info>,
    #[account(mut, close = signer)]
    proposal: Account<'info, Proposal>,
}

#[derive(Accounts)]
pub struct CreateVote<'info> {
    #[account(mut)]
    author: Signer<'info>,
    #[account(mut)]
    proposal: Account<'info, Proposal>,
    #[account(
        init,
        seeds = [author.key().as_ref(), proposal.key().as_ref()],
        bump,
        payer = author,
        space = 73
    )]
    vote: Account<'info, Vote>,
    system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ChangeVote<'info> {
    author: Signer<'info>,
    #[account(mut)]
    proposal: Account<'info, Proposal>,
    #[account(mut, has_one = author)]
    vote: Account<'info, Vote>,
}

#[derive(Accounts)]
pub struct DeleteVote<'info> {
    signer: Signer<'info>,
    #[account(mut)]
    proposal: Account<'info, Proposal>,
    #[account(mut, close = signer)]
    vote: Account<'info, Vote>,
}

// Data structures
#[account] //8 + 140 = 148
pub struct Proposal {
    author: Pubkey,    //32
    program: Pubkey,   //32
    name: String,      //4 + 34: 38
    group: String,     //4 + 8: 12
    sub_group: String, //4 + 18: 22
    score: i32,        //4
}

#[account] //8 + 65 = 73
pub struct Vote {
    author: Pubkey,   //32
    proposal: Pubkey, //32
    positive: bool,   //1
}

//Errors
#[error_code]
pub enum CustomError {
    #[msg("Name should not exceed 34 characters")]
    NameTooLong,
    #[msg("Group should not exceed 8 characters")]
    GroupTooLong,
    #[msg("Sub Group should not exceed 18 characters")]
    SubGroupTooLong,
    #[msg("Signer already upvoted this proposal")]
    AlreadyUpvoted,
    #[msg("Signer already downvoted this proposal")]
    AlreadyDownvoted,
    #[msg("Signer must be the author or admin")]
    NotAuthorOrAdmin,
}
