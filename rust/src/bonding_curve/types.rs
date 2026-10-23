//! Shared types used by accounts and instruction args

use borsh::{BorshDeserialize, BorshSerialize};
use solana_pubkey::Pubkey;

/// Admin feature flags
#[derive(BorshDeserialize, BorshSerialize, Debug, Clone, PartialEq)]
pub struct AdminConfig {
    pub is_curve_swaps_disabled: bool,
    pub is_curve_launch_disabled: bool,
    pub is_prepare_migration_disabled: bool,
    pub is_migrate_disabled: bool,
    pub is_fund_migration_disabled: bool,
    pub is_claim_prize_disabled: bool,
    pub is_claim_creator_fee_disabled: bool,
    pub is_define_winner_disabled: bool,
}

/// Protocol fee configuration
#[derive(BorshDeserialize, BorshSerialize, Debug, Clone, PartialEq)]
pub struct FeeConfig {
    pub protocol_curve_fee_bps: u16,
    pub protocol_claim_creator_fee_bps: u16,
    pub creator_fee_bps: u16,
    pub last_buyer_fee_bps: u16,
    pub fee_recipient_1: Pubkey,
    pub fee_recipient_2: Pubkey,
}

/// Core protocol parameters
#[derive(BorshDeserialize, BorshSerialize, Debug, Clone, PartialEq)]
pub struct ProtocolConfig {
    pub last_buy_timer_secs: i64,
    pub migration_threshold_sol: u64,
    pub protocol_authority: Pubkey,
}

/// Raydium CPMM integration configuration
#[derive(BorshDeserialize, BorshSerialize, Debug, Clone, PartialEq)]
pub struct RaydiumConfig {
    pub raydium_cpmm_program: Pubkey,
    pub raydium_lock_program: Pubkey,
    pub raydium_amm_config: Pubkey,
    pub raydium_fee_account: Pubkey,
    pub raydium_cpmm_migration_coverage_sol: u64,
}

/// Lifecycle status of a bonding curve
#[derive(BorshDeserialize, BorshSerialize, Debug, Clone, Copy, PartialEq, Eq)]
pub enum CurveStatus {
    Active,
    Inactive,
    AlternativeWinnerDefined,
    WinnerDefined,
    Graduated,
    Migrating,
    Migrated,
}
