use anchor_lang::prelude::*;
use solana_security_txt::security_txt;
use instructions::*;

pub mod errors;
pub mod instructions;
pub mod state;

#[cfg(not(feature = "no-entrypoint"))]
security_txt! {
    // Required fields
    name: "Solactivity",
    project_url: "https://solactivity.info",
    contacts: "twitter:https://twitter.com/solactivity,link:https://forms.gle/N9tFXTGzm6nmWLYy5",
    policy: "https://solactivity.info/security",
    // Optional Fields
    preferred_languages: "en",
    //source_code: "https://github.com/example/example",
    auditors: "None"
    //acknowledgements: ""
}

declare_id!("acTiJkzfuF6vx8Z6GvH4JcZEWCyztU3M5L6BsQDzfNa");

#[program]
pub mod solactivity {
    use super::*;

    pub fn create_proposal(
        ctx: Context<CreateProposal>,
        name: String,
        group: String,
        sub_group: String,
    ) -> Result<()> {
        instructions::proposal::create_proposal(ctx, name, group, sub_group)
    }

    pub fn delete_proposal(ctx: Context<DeleteProposal>) -> Result<()> {
        instructions::proposal::delete_proposal(ctx)
    }

    pub fn create_vote(ctx: Context<CreateVote>, positive: bool) -> Result<()> {
        instructions::vote::create_vote(ctx, positive)
    }

    pub fn change_vote(ctx: Context<ChangeVote>, positive: bool) -> Result<()> {
        instructions::vote::change_vote(ctx, positive)
    }

    pub fn delete_vote(ctx: Context<DeleteVote>) -> Result<()> {
        instructions::vote::delete_vote(ctx)
    }
}
