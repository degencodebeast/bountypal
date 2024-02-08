use anchor_lang::prelude::*;

use crate::constants::*;

#[account]
pub struct BountyHunter {
    /// Name of user
    pub display_name: String,

    /// Authority of user
    pub authority: Pubkey,

    // /// Bounty Hunter Token Account
    // pub bounty_hunter_token_account: Pubkey,

    /// Bounty hunter Id on the platform
    pub id: u64,

    /// Bio of user
    pub bio: String,

    /// id of platform (github/gitlab)
    pub platform_id: String,

    /// Reputation of user
    pub reputation: i64,

    /// Number of completed bounties
    pub completed_bounties: u64,

    /// Bump
    pub bump: u8,
}

impl BountyHunter {
    pub const LEN: usize = DISCRIMINATOR_LENGTH  // 8-byte discriminator
        + NAME_LENGTH                            // Name
        + PUBKEY_LENGTH                          // Authority
        // + PUBKEY_LENGTH                       //  Bounty hunter Token Account
        + DATA_LENGTH                            // id of bounty hunter on the platform
        + BIO_LENGTH                             // Bio of user
        + NAME_LENGTH                            // platform id
        + DATA_LENGTH                            // Reputation
        + DATA_LENGTH                            // Number of completed bounties
        + BOOL_LENGTH; // PDA Bump
}
