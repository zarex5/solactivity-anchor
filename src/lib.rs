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
}

// Data validators
#[derive(Accounts)]
pub struct CreateProposal<'info> {
    #[account(mut)]
    authority: Signer<'info>,
    #[account(mut)]
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

// Data structures
#[account] //8 + 116 = 124
pub struct Proposal {
    authority: Pubkey,     //32
    program: Pubkey,       //32
    proposed_name: String, //4 + 10: 14
    proposed_type: String, //4 + 34: 38
}
