use borsh::{BorshDeserialize, BorshSerialize};
use solana_pubkey::Pubkey;

use crate::constants::*;
use crate::extentions::Discriminator;

use super::types::CurveStatus;

/// Bonding curve state account
///
/// PDA: `["curve", mint]`
#[derive(BorshDeserialize, BorshSerialize, Debug, Clone, PartialEq)]
pub struct CurveState {
    pub bump: u8,
    pub status: CurveStatus,
    /// till this key is default, curve cannot migrate to Raydium CPMM
    pub migration_funder: Pubkey,
    /// fees are automatically distributed to token creator
    pub creator: Pubkey,
    pub mint: Pubkey,
    pub last_buyer: Pubkey,
    pub last_buy_sol: u64,
    pub last_buy_ts: i64,
    /// `max_hold_duration_secs` and `max_duration_holder` are used to track the holder with the max duration
    /// in order to define alternative winner if canonical winner is undefined
    pub max_hold_duration_secs: i64,
    /// `max_hold_duration_secs` and `max_duration_holder` are used to track the holder with the max duration
    /// in order to define alternative winner if canonical winner is undefined
    pub max_duration_holder: Pubkey,
    pub real_reserves_sol: u64,
    pub real_reserves_tokens: u64,
    pub virtual_reserves_sol: u64,
    pub virtual_reserves_tokens: u64,
    pub last_buy_timer_secs_snapshot: i64,
    pub migration_threshold_sol_snapshot: u64,
}

impl Discriminator for CurveState {
    const DISCRIMINATOR: [u8; ANCHOR_DISCRIMINATOR_SIZE] = CURVE_STATE_DISCRIMINATOR;
}
