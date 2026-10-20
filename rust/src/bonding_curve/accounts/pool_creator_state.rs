use borsh::{BorshDeserialize, BorshSerialize};

use crate::constants::*;
use crate::extentions::Discriminator;

/// Raydium CPMM pool creator tracking account
///
/// PDA: `["pool_creator_state", pool_creator]`
#[derive(BorshDeserialize, BorshSerialize, Debug, Clone, PartialEq)]
pub struct PoolCreatorState {
    pub bump: u8,
    /// this flag basically indicates whether or not fees can be claimed, acts as a guard, so fees cannot be claimed during prepare_migration ix
    pub is_fee_claim_locked: bool,
}

impl Discriminator for PoolCreatorState {
    const DISCRIMINATOR: [u8; ANCHOR_DISCRIMINATOR_SIZE] = POOL_CREATOR_STATE_DISCRIMINATOR;
}
