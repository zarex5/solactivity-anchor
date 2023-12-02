use anchor_lang::prelude::*;

declare_id!("EvViEBWR7aNDFD3rtou1GhjdYyQ31QW36UwQ1aFqPHVh");

#[program]
// Smart contract functions
pub mod proposal {
    use super::*;

    pub fn create_proposal(
        ctx: Context<CreateProposal>,
        proposed_name: String,
        proposed_type: String,
    ) -> Result<()> {
        require!(proposed_name.len() <= 34, CustomError::NameTooLong);
        require!(proposed_type.len() <= 10, CustomError::TypeTooLong);
        let proposal = &mut ctx.accounts.proposal;
        proposal.authority = ctx.accounts.authority.key();
        proposal.program = ctx.accounts.program.key();
        proposal.proposed_name = proposed_name;
        proposal.proposed_type = proposed_type;
        msg!(
            "Created proposal by:{} for program:{}",
            proposal.authority,
            proposal.program
        );
        Ok(())
    }

    pub fn delete_proposal(_ctx: Context<DeleteProposal>) -> Result<()> {
        msg!("Deleting proposal!");
        Ok(())
    }

    pub fn create_vote(ctx: Context<CreateVote>, positive: bool) -> Result<()> {
        let vote = &mut ctx.accounts.vote;
        vote.authority = ctx.accounts.authority.key();
        vote.proposal = ctx.accounts.proposal.key();
        vote.positive = positive;
        msg!(
            "Created vote by:{} for proposal:{}",
            vote.authority,
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
        msg!("Changed vote to positive:{}", vote.positive);
        Ok(())
    }

    pub fn delete_vote(_ctx: Context<DeleteVote>) -> Result<()> {
        msg!("Deleting vote!");
        Ok(())
    }
}

// Data validators
#[derive(Accounts)]
pub struct CreateProposal<'info> {
    #[account(mut)]
    authority: Signer<'info>,
    #[account(executable)]
    program: UncheckedAccount<'info>,
    #[account(
        init,
        seeds = [authority.key().as_ref(), program.key().as_ref()],
        bump,
        payer = authority,
        space = 124
    )]
    proposal: Account<'info, Proposal>,
    system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct DeleteProposal<'info> {
    authority: Signer<'info>,
    #[account(mut, has_one = authority, close = authority)]
    proposal: Account<'info, Proposal>,
}

#[derive(Accounts)]
pub struct CreateVote<'info> {
    #[account(mut)]
    authority: Signer<'info>,
    #[account(mut)]
    proposal: Account<'info, Proposal>,
    #[account(
        init,
        seeds = [authority.key().as_ref(), proposal.key().as_ref()],
        bump,
        payer = authority,
        space = 73
    )]
    vote: Account<'info, Vote>,
    system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ChangeVote<'info> {
    authority: Signer<'info>,
    #[account(mut, has_one = authority)]
    vote: Account<'info, Vote>,
}

#[derive(Accounts)]
pub struct DeleteVote<'info> {
    authority: Signer<'info>,
    #[account(mut, has_one = authority, close = authority)]
    vote: Account<'info, Vote>,
}

// Data structures
#[account] //8 + 116 = 124
pub struct Proposal {
    authority: Pubkey,     //32
    program: Pubkey,       //32
    proposed_name: String, //4 + 34: 38
    proposed_type: String, //4 + 10: 14
}

#[account] //8 + 65 = 73
pub struct Vote {
    authority: Pubkey, //32
    proposal: Pubkey,  //32
    positive: bool,    //1
}

//Errors
#[error_code]
pub enum CustomError {
    #[msg("Name should not exceed 10 characters")]
    NameTooLong,
    #[msg("Type should not exceed 34 characters")]
    TypeTooLong,
    #[msg("You already upvoted this proposal")]
    AlreadyUpvoted,
    #[msg("You already downvoted this proposal")]
    AlreadyDownvoted,
}
