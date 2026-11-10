use crate::{constants::*, extentions::*, pda::*};

use solana_instruction::{Instruction, AccountMeta};
use solana_pubkey::Pubkey;

pub struct FundMigrationInstruction;

pub struct FundMigrationInstructionCtx {
    /// is_signer: true
    pub user: Pubkey,
    pub mint: Pubkey,
}

impl Discriminator for FundMigrationInstruction {
    const DISCRIMINATOR: [u8; ANCHOR_DISCRIMINATOR_SIZE] = FUND_MIGRATION_DISCRIMINATOR;
}

impl AccountCounts for FundMigrationInstruction {
    const ACCOUNTS: usize = FUND_MIGRATION_ACCOUNTS;
}

impl FundMigrationInstruction {
    pub fn new(ctx: FundMigrationInstructionCtx) -> Instruction {
        let FundMigrationInstructionCtx { user, mint } = ctx;

        let curve = derive_curve(&mint).0;
        let global_config = derive_global_config().0;
        let event_authority = derive_event_authority().0;

        let mut ix_data: [u8; FUND_MIGRATION_SIZE] = [0u8; FUND_MIGRATION_SIZE];
        let mut ix_accounts: Vec<AccountMeta> = Vec::with_capacity(Self::TOTAL_ACCOUNTS);

        ix_data[..ANCHOR_DISCRIMINATOR_SIZE].copy_from_slice(&Self::DISCRIMINATOR);
        ix_accounts.extend([
            AccountMeta::new(user, true),
            AccountMeta::new(curve, false),
            AccountMeta::new_readonly(global_config, false),
            AccountMeta::new_readonly(mint, false),
            AccountMeta::new_readonly(SYSTEM_PROGRAM_ID, false),
            AccountMeta::new_readonly(event_authority, false),
            AccountMeta::new_readonly(PROGRAM_ID, false),
        ]);

        Instruction::new_with_bytes(PROGRAM_ID, &ix_data, ix_accounts)
    }
}
