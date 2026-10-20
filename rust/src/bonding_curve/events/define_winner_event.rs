use borsh::{BorshDeserialize, BorshSerialize};
use solana_pubkey::Pubkey;

use crate::constants::*;
use crate::extentions::Discriminator;

/// Anchor event `DefineWinnerEvent`
#[derive(BorshDeserialize, BorshSerialize, Debug, Clone, PartialEq)]
pub struct DefineWinnerEvent {
    /// last_buyer with `now_ts >= last_buy_ts + last_buy_timer_secs`
    pub winner: Pubkey,
    pub curve: Pubkey,
    pub mint: Pubkey,
    pub timestamp: i64,
}

impl Discriminator for DefineWinnerEvent {
    const DISCRIMINATOR: [u8; ANCHOR_DISCRIMINATOR_SIZE] = DEFINE_WINNER_EVENT_DISCRIMINATOR;
}
