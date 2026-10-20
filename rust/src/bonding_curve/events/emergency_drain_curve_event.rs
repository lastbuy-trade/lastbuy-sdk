use borsh::{BorshDeserialize, BorshSerialize};
use solana_pubkey::Pubkey;

use crate::constants::*;
use crate::extentions::Discriminator;

/// Anchor event `EmergencyDrainCurveEvent`
#[derive(BorshDeserialize, BorshSerialize, Debug, Clone, PartialEq)]
pub struct EmergencyDrainCurveEvent {
    /// protocol authority
    pub user: Pubkey,
    pub recipient: Pubkey,
    /// migration funder
    pub funder: Option<Pubkey>,
    pub curve: Pubkey,
    pub mint: Pubkey,
    pub timestamp: i64,
}

impl Discriminator for EmergencyDrainCurveEvent {
    const DISCRIMINATOR: [u8; ANCHOR_DISCRIMINATOR_SIZE] = EMERGENCY_DRAIN_CURVE_EVENT_DISCRIMINATOR;
}
