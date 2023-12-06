use crate::errors::SolactivityError;
use crate::constants::ADMIN_PUBKEY;
use crate::state::proposal::*;
use crate::state::vote::*;
use anchor_lang::prelude::*;

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
            err!(SolactivityError::AlreadyUpvoted)
        } else {
            err!(SolactivityError::AlreadyDownvoted)
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
        return err!(SolactivityError::NotAuthorOrAdmin);
    }
    let proposal = &mut ctx.accounts.proposal;
    proposal.score += if vote.positive { -1 } else { 1 };
    msg!("Deleting vote!");
    Ok(())
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
        space = Vote::MAXIMUM_SIZE + 8
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
