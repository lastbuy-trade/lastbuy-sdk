use borsh::{BorshDeserialize, BorshSerialize};
use solana_pubkey::Pubkey;

use crate::constants::*;
use crate::extentions::Discriminator;

/// Anchor event `FundMigrationEvent`
#[derive(BorshDeserialize, BorshSerialize, Debug, Clone, PartialEq)]
pub struct FundMigrationEvent {
    pub funder: Pubkey,
    pub curve: Pubkey,
    pub mint: Pubkey,
    pub timestamp: i64,
    pub funding_sol: u64,
}

impl Discriminator for FundMigrationEvent {
    const DISCRIMINATOR: [u8; ANCHOR_DISCRIMINATOR_SIZE] = FUND_MIGRATION_EVENT_DISCRIMINATOR;
}

#[cfg(feature = "parse-events")]
impl super::parse::EventParser for FundMigrationEvent {}