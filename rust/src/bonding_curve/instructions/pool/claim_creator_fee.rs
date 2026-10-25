use crate::{constants::*, extentions::*, pda::*, error::*};

use borsh::BorshSerialize;
use solana_instruction::{Instruction, AccountMeta};
use solana_pubkey::Pubkey;
use spl_associated_token_account_interface::address::get_associated_token_address_with_program_id;

pub struct ClaimCreatorFeeInstruction;

pub struct ClaimCreatorFeeInstructionCtx {
    /// is_signer: true
    /// 
    /// must equal to `curve.migration_funder`
    pub user: Pubkey,
    pub mint: Pubkey,
    /// must be equal to `global_config.raydium.raydium_amm_config`
    pub raydium_amm_config: Pubkey,
    pub raydium_cpmm_program: Pubkey,
    pub raydium_lock_program: Pubkey,
    pub fee_recipient_1: Pubkey,
    pub fee_recipient_2: Pubkey,
}