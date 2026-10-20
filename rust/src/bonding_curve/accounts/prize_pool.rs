use borsh::{BorshDeserialize, BorshSerialize};
use solana_pubkey::Pubkey;

use crate::constants::*;
use crate::extentions::Discriminator;

/// Prize pool account holding SOL rewards for the winner
///
/// PDA: `["prize_pool", curve]`
#[derive(BorshDeserialize, BorshSerialize, Debug, Clone, PartialEq)]
pub struct PrizePool {
    pub bump: u8,
    pub is_claimed: bool,
    pub amount_sol: u64,
    pub curve: Pubkey,
    pub winner: Pubkey,
    pub alternative_winner: Pubkey,
}

impl Discriminator for PrizePool {
    const DISCRIMINATOR: [u8; ANCHOR_DISCRIMINATOR_SIZE] = PRIZE_POOL_DISCRIMINATOR;
}
