use crate::errors::SolactivityError;
use crate::constants::ADMIN_PUBKEY;
use crate::state::proposal::*;
use anchor_lang::prelude::*;

pub fn create_proposal(
    ctx: Context<CreateProposal>,
    name: String,
    group: String,
    sub_group: String,
) -> Result<()> {
    require!(name.len() <= 34, SolactivityError::NameTooLong);
    require!(group.len() <= 8, SolactivityError::GroupTooLong);
    require!(sub_group.len() <= 18, SolactivityError::SubGroupTooLong);
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

pub fn migrate_proposal(
    ctx: Context<MigrateProposal>,
    name: String,
    group: String,
    sub_group: String,
) -> Result<()> {
    require!(ctx.accounts.signer.key() == ADMIN_PUBKEY.key(), SolactivityError::NotAdmin);
    require!(name.len() <= 34, SolactivityError::NameTooLong);
    require!(group.len() <= 8, SolactivityError::GroupTooLong);
    require!(sub_group.len() <= 18, SolactivityError::SubGroupTooLong);
    let proposal = &mut ctx.accounts.proposal;
    proposal.author = ctx.accounts.author.key();
    proposal.program = ctx.accounts.program.key();
    proposal.name = name;
    proposal.group = group;
    proposal.sub_group = sub_group;
    proposal.score = 0;
    msg!(
        "Migrated proposal by:{} for program:{}",
        proposal.author,
        proposal.program
    );
    Ok(())
}

pub fn delete_proposal(ctx: Context<DeleteProposal>) -> Result<()> {
    let signer = &mut ctx.accounts.signer;
    let proposal = &mut ctx.accounts.proposal;
    if signer.key() != proposal.author.key() && signer.key() != ADMIN_PUBKEY.key() {
        return err!(SolactivityError::NotAuthorOrAdmin);
    }
    //TODO: Delete all votes associated with the proposal? (+allow proposal owner to delete votes on its proposal)
    msg!("Deleting proposal!");
    Ok(())
}

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
        space = Proposal::MAXIMUM_SIZE + 8
    )]
    proposal: Account<'info, Proposal>,
    system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct MigrateProposal<'info> {
    #[account(mut)]
    signer: Signer<'info>,
    #[account()]
    /// CHECK: Any user address is okay (we don't read or write from this account)
    author: UncheckedAccount<'info>,
    #[account(executable)]
    /// CHECK: Any program address is okay (we don't read or write from this account)
    program: UncheckedAccount<'info>,
    #[account(
        init,
        seeds = [author.key().as_ref(), program.key().as_ref()],
        bump,
        payer = signer,
        space = Proposal::MAXIMUM_SIZE + 8
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