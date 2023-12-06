use crate::errors::SolactivityError;
use crate::constants::ADMIN_PUBKEY;
use crate::state::proposal::*;
use crate::state::vote::*;
use anchor_lang::prelude::*;

pub fn create_vote(ctx: Context<CreateVote>, positive: bool) -> Result<()> {
    let proposal = &mut ctx.accounts.proposal;
    proposal.increment_score(if positive { 1 } else { -1 });
    let vote = &mut ctx.accounts.vote;
    vote.setup(ctx.accounts.author.key(), ctx.accounts.proposal.key(), positive)
}

pub fn change_vote(ctx: Context<ChangeVote>, positive: bool) -> Result<()> {
    let proposal = &mut ctx.accounts.proposal;
    proposal.increment_score(if positive { 2 } else { -2 });
    let vote = &mut ctx.accounts.vote;
    vote.change_vote(positive)
}

pub fn delete_vote(ctx: Context<DeleteVote>) -> Result<()> {
    let signer = &mut ctx.accounts.signer;
    let vote = &mut ctx.accounts.vote;
    if signer.key() != vote.author().key() && signer.key() != ADMIN_PUBKEY.key() {
        return err!(SolactivityError::NotAuthorOrAdmin)
    }
    let proposal = &mut ctx.accounts.proposal;
    proposal.increment_score(if *vote.positive() { -1 } else { 1 });
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
