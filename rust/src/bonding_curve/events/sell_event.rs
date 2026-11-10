use borsh::{BorshDeserialize, BorshSerialize};
use solana_pubkey::Pubkey;

use crate::constants::*;
use crate::extentions::Discriminator;

/// Anchor event `SellEvent`
#[derive(BorshDeserialize, BorshSerialize, Debug, Clone, PartialEq)]
pub struct SellEvent {
    pub trader: Pubkey,
    pub mint: Pubkey,
    pub curve: Pubkey,
    pub creator: Pubkey,
    pub tokens_in: u64,
    pub gross_sol_out: u64,
    pub net_sol_out: u64,
    pub protocol_fee: u64,
    pub creator_fee: u64,
    pub last_buyer_fee: u64,
    pub virtual_sol_reserves_after: u64,
    pub virtual_token_reserves_after: u64,
    pub timestamp: i64,
}

impl Discriminator for SellEvent {
    const DISCRIMINATOR: [u8; ANCHOR_DISCRIMINATOR_SIZE] = SELL_EVENT_DISCRIMINATOR;
}

#[cfg(feature = "parse-events")]
impl super::parse::EventParser for SellEvent {}