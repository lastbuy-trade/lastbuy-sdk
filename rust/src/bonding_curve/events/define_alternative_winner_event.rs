use borsh::{BorshDeserialize, BorshSerialize};
use solana_pubkey::Pubkey;

use crate::constants::*;
use crate::extentions::Discriminator;

/// Anchor event `DefineAlternativeWinnerEvent`
#[derive(BorshDeserialize, BorshSerialize, Debug, Clone, PartialEq)]
pub struct DefineAlternativeWinnerEvent {
    /// `curve.max_duration_holder`
    pub winner: Pubkey,
    pub curve: Pubkey,
    pub mint: Pubkey,
    pub hold_duration_secs: i64,
    pub timestamp: i64,
}

impl Discriminator for DefineAlternativeWinnerEvent {
    const DISCRIMINATOR: [u8; ANCHOR_DISCRIMINATOR_SIZE] = DEFINE_ALTERNATIVE_WINNER_EVENT_DISCRIMINATOR;
}

#[cfg(feature = "parse-events")]
impl super::parse::EventParser for DefineAlternativeWinnerEvent {}