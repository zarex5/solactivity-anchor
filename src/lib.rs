use anchor_lang::prelude::*;
use solana_security_txt::security_txt;
use instructions::*;

pub mod errors;
pub mod instructions;
pub mod state;
pub mod constants;

#[cfg(not(feature = "no-entrypoint"))]
security_txt! {
    name: "Solactivity",
    project_url: "https://solactivity.info",
    contacts: "link:https://twitter.com/solactivity",
    policy: "https://solactivity.info/security",
    preferred_languages: "en",
    auditors: "None"
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

    pub fn migrate_proposal(
        ctx: Context<MigrateProposal>,
        name: String,
        group: String,
        sub_group: String,
        score: i32,
    ) -> Result<()> {
        instructions::proposal::migrate_proposal(ctx, name, group, sub_group, score)
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
