use borsh::{BorshDeserialize, BorshSerialize};
use solana_pubkey::Pubkey;

use crate::constants::*;
use crate::extentions::Discriminator;

/// Anchor event `ClaimPrizeEvent`
#[derive(BorshDeserialize, BorshSerialize, Debug, Clone, PartialEq)]
pub struct ClaimPrizeEvent {
    pub winner: Pubkey,
    pub curve: Pubkey,
    pub mint: Pubkey,
    pub amount_sol: u64,
    pub timestamp: i64,
}

impl Discriminator for ClaimPrizeEvent {
    const DISCRIMINATOR: [u8; ANCHOR_DISCRIMINATOR_SIZE] = CLAIM_PRIZE_EVENT_DISCRIMINATOR;
}

#[cfg(feature = "parse-events")]
impl super::parse::EventParser for ClaimPrizeEvent {}