use crate::{constants::*, extentions::*, pda::*};

use solana_instruction::{Instruction, AccountMeta};
use solana_pubkey::Pubkey;
use spl_associated_token_account_interface::address::get_associated_token_address_with_program_id;

/// # IMPORTANT
/// 
/// * ### only V1 transactions can handle these instructions, solana core must be >= 4.0.0
///
/// * ### must be called in pair with [`super::MigrateInstruction`]
///
/// * ### order matters. example:
///  
/// instruction-1: PrepareMigrationInstruction
/// 
/// instruction-2: MigrateInstruction
pub struct PrepareMigrationInstruction;

pub struct PrepareMigrationInstructionCtx {
    /// is_signer: true
    /// 
    /// Can be any user either migration watcher or external bot
    pub user: Pubkey,
    /// - Can be distinct from `user`
    /// - Can be fetched from `curve` PDA
    pub migration_funder: Pubkey,
    /// curve creator
    /// 
    /// - Can be fetched from `curve` PDA
    pub creator: Pubkey,
    pub mint: Pubkey,
    pub fee_recipient_1: Pubkey,
    pub fee_recipient_2: Pubkey,
}

impl Discriminator for PrepareMigrationInstruction {
    const DISCRIMINATOR: [u8; ANCHOR_DISCRIMINATOR_SIZE] = PREPARE_MIGRATION_DISCRIMINATOR;
}

impl AccountCounts for PrepareMigrationInstruction {
    const ACCOUNTS: usize = PREPARE_MIGRATION_ACCOUNTS;
    const REMAINING_ACCOUNTS: usize = PREPARE_MIGRATION_REMAINING_ACCOUNTS;
}

impl PrepareMigrationInstruction {
    pub fn new(ctx: PrepareMigrationInstructionCtx) -> Instruction {
        let PrepareMigrationInstructionCtx {
            user,
            creator,
            mint,
            migration_funder,
            fee_recipient_1,
            fee_recipient_2,
        } = ctx;

        let curve = derive_curve(&mint).0;
        let pool_creator = derive_pool_creator(&migration_funder, &curve).0;
        let pool_creator_state = derive_pool_creator_state(&pool_creator).0;
        let curve_ata = get_associated_token_address_with_program_id(
            &curve,
            &mint,
            &SPL_TOKEN_2022_PROGRAM_ID,
        );
        let global_config = derive_global_config().0;
        let prize_pool = derive_prize_pool(&curve).0;
        let pool_creator_token_account_0 = get_associated_token_address_with_program_id(
            &pool_creator,
            &WSOL_ID,
            &SPL_TOKEN_PROGRAM_ID,
        );

        let mut ix_data: [u8; PREPARE_MIGRATION_SIZE] = [0u8; PREPARE_MIGRATION_SIZE];
        let mut ix_accounts: Vec<AccountMeta> = Vec::with_capacity(Self::TOTAL_ACCOUNTS);

        ix_data[..ANCHOR_DISCRIMINATOR_SIZE].copy_from_slice(&Self::DISCRIMINATOR);
        ix_accounts.extend([
            AccountMeta::new(user, true),
            AccountMeta::new(pool_creator, false),
            AccountMeta::new(pool_creator_state, false),
            AccountMeta::new(creator, false),
            AccountMeta::new(curve, false),
            AccountMeta::new(curve_ata, false),
            AccountMeta::new_readonly(global_config, false),
            AccountMeta::new(prize_pool, false),
            AccountMeta::new(pool_creator_token_account_0, false),
            AccountMeta::new_readonly(WSOL_ID, false),
            AccountMeta::new_readonly(mint, false),
            AccountMeta::new_readonly(SPL_TOKEN_PROGRAM_ID, false),
            AccountMeta::new_readonly(SPL_TOKEN_2022_PROGRAM_ID, false),
            AccountMeta::new_readonly(SYSTEM_PROGRAM_ID, false),
            // remaining accounts
            AccountMeta::new(fee_recipient_1, false),
            AccountMeta::new(fee_recipient_2, false),
        ]);

        Instruction::new_with_bytes(PROGRAM_ID, &ix_data, ix_accounts)
    }
}
