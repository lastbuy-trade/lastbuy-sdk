use crate::{constants::*, extentions::*, pda::*, error::*};

use borsh::BorshSerialize;
use solana_instruction::{Instruction, AccountMeta};
use solana_pubkey::Pubkey;
use spl_associated_token_account_interface::address::get_associated_token_address_with_program_id;

pub struct BuyExactInstruction;

pub struct BuyExactInstructionCtx {
    /// is_signer: true
    pub user: Pubkey,
    pub creator: Pubkey
}

#[derive(Debug, borsh::BorshSerialize)]
pub struct BuyExactInstructionArgs {
    pub sol_in: u64,
    pub min_tokens_out: u64
}

impl Discriminator for BuyExactInstruction {
    const DISCRIMINATOR: [u8; ANCHOR_DISCRIMINATOR_SIZE] = BUY_EXACT_DISCRIMINATOR;
}

impl AccountCounts for BuyExactInstruction {
    const ACCOUNTS: usize = BUY_EXACT_ACCOUNTS;
}