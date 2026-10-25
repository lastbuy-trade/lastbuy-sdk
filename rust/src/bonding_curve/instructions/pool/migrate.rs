use crate::{constants::*, extentions::*, pda::*, error::*};

use solana_instruction::{Instruction, AccountMeta};
use solana_pubkey::Pubkey;
use spl_associated_token_account_interface::address::get_associated_token_address_with_program_id;

pub struct MigrateInstruction;

pub struct MigrateInstructionCtx {
    /// is_signer: true
    pub user: Pubkey,
    /// can be distinct from `user`
    pub migration_funder: Pubkey,
    pub mint: Pubkey,
}