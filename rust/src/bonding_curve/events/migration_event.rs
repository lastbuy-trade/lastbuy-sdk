use borsh::{BorshDeserialize, BorshSerialize};
use solana_pubkey::Pubkey;

use crate::constants::*;
use crate::extentions::Discriminator;

/// Anchor event `MigrationEvent`
#[derive(BorshDeserialize, BorshSerialize, Debug, Clone, PartialEq)]
pub struct MigrationEvent {
    pub curve: Pubkey,
    pub mint: Pubkey,
    pub user: Pubkey,
    pub migration_funder: Pubkey,
    pub pool_creator: Pubkey,
    pub pool: Pubkey,
    pub lp_mint: Pubkey,
    pub sol_amount: u64,
    pub token_amount: u64,
    pub lp_amount: u64,
    pub locked_lp_amount: u64,
    pub timestamp: i64,
}

impl Discriminator for MigrationEvent {
    const DISCRIMINATOR: [u8; ANCHOR_DISCRIMINATOR_SIZE] = MIGRATION_EVENT_DISCRIMINATOR;
}
