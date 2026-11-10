use borsh::{BorshDeserialize, BorshSerialize};
use solana_pubkey::Pubkey;

use crate::constants::*;
use crate::extentions::Discriminator;

/// Anchor event `PrepareMigrationEvent`
#[derive(BorshDeserialize, BorshSerialize, Debug, Clone, PartialEq)]
pub struct PrepareMigrationEvent {
    pub curve: Pubkey,
    pub mint: Pubkey,
    pub user: Pubkey,
    pub migration_funder: Pubkey,
    pub pool_creator: Pubkey,
    pub timestamp: i64,
}

impl Discriminator for PrepareMigrationEvent {
    const DISCRIMINATOR: [u8; ANCHOR_DISCRIMINATOR_SIZE] = PREPARE_MIGRATION_EVENT_DISCRIMINATOR;
}

#[cfg(feature = "parse-events")]
impl super::parse::EventParser for PrepareMigrationEvent {}