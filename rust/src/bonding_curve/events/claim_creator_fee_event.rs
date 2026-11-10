use borsh::{BorshDeserialize, BorshSerialize};
use solana_pubkey::Pubkey;

use crate::constants::*;
use crate::extentions::Discriminator;

/// Anchor event `ClaimCreatorFeeEvent`
#[derive(BorshDeserialize, BorshSerialize, Debug, Clone, PartialEq)]
pub struct ClaimCreatorFeeEvent {
    pub user: Pubkey,
    pub pool_creator: Pubkey,
    /// token amount claimed from raydium CPMM fees
    pub gross_token_0_amount: u64,
    /// token amount claimed from raydium CPMM fees
    pub gross_token_1_amount: u64,
    /// token amount after protocol fees that receives user
    pub net_token_0_amount: u64,
    /// token amount after protocol fees that receives user
    ///
    /// fee free, equals to `gross_token_1_amount`, we dont charge fees from token_1
    pub net_token_1_amount: u64,
    pub protocol_fee: u64,
    pub timestamp: i64,
}

impl Discriminator for ClaimCreatorFeeEvent {
    const DISCRIMINATOR: [u8; ANCHOR_DISCRIMINATOR_SIZE] = CLAIM_CREATOR_FEE_EVENT_DISCRIMINATOR;
}

#[cfg(feature = "parse-events")]
impl super::parse::EventParser for ClaimCreatorFeeEvent {}