use anchor_lang::error_code;

#[error_code]
pub enum SolactivityError {
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
