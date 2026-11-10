use borsh::{BorshDeserialize, BorshSerialize};
use solana_pubkey::Pubkey;

use crate::constants::*;
use crate::extentions::Discriminator;

/// Anchor event `LaunchEvent`
#[derive(BorshDeserialize, BorshSerialize, Debug, Clone, PartialEq)]
pub struct LaunchEvent {
    pub creator: Pubkey,
    pub mint: Pubkey,
    pub curve: Pubkey,
    pub timestamp: i64,
    pub token_name: String,
    pub token_symbol: String,
    pub token_uri: String,
}

impl Discriminator for LaunchEvent {
    const DISCRIMINATOR: [u8; ANCHOR_DISCRIMINATOR_SIZE] = LAUNCH_EVENT_DISCRIMINATOR;
}

#[cfg(feature = "parse-events")]
impl super::parse::EventParser for LaunchEvent {}