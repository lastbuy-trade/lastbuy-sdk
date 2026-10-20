use borsh::{BorshDeserialize, BorshSerialize};
use solana_pubkey::Pubkey;

use crate::constants::*;
use crate::extentions::Discriminator;

/// Anchor event `GraduationEvent`
#[derive(BorshDeserialize, BorshSerialize, Debug, Clone, PartialEq)]
pub struct GraduationEvent {
    pub curve: Pubkey,
    pub mint: Pubkey,
    pub timestamp: i64,
}

impl Discriminator for GraduationEvent {
    const DISCRIMINATOR: [u8; ANCHOR_DISCRIMINATOR_SIZE] = GRADUATION_EVENT_DISCRIMINATOR;
}
