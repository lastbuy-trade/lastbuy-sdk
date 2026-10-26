use crate::{constants::*, extentions::*, pda::*};

use solana_instruction::{Instruction, AccountMeta};
use solana_pubkey::Pubkey;

pub struct DefineWinnerIdempotentInstruction;

pub struct DefineWinnerIdempotentInstructionCtx {
    /// is_signer: true
    pub user: Pubkey,
    pub mint: Pubkey,
}

impl Discriminator for DefineWinnerIdempotentInstruction {
    const DISCRIMINATOR: [u8; ANCHOR_DISCRIMINATOR_SIZE] = DEFINE_WINNER_IDEMPOTENT_DISCRIMINATOR;
}

impl AccountCounts for DefineWinnerIdempotentInstruction {
    const ACCOUNTS: usize = DEFINE_WINNER_IDEMPOTENT_ACCOUNTS;
}

impl DefineWinnerIdempotentInstruction {
    pub fn new(ctx: DefineWinnerIdempotentInstructionCtx) -> Instruction {
        let DefineWinnerIdempotentInstructionCtx { user, mint } = ctx;

        let global_config = derive_global_config().0;
        let prize_pool = derive_prize_pool(&derive_curve(&mint).0).0;
        let curve = derive_curve(&mint).0;

        let mut ix_data: [u8; DEFINE_WINNER_IDEMPOTENT_SIZE] = [0u8; DEFINE_WINNER_IDEMPOTENT_SIZE];
        let mut ix_accounts: Vec<AccountMeta> = Vec::with_capacity(Self::TOTAL_ACCOUNTS);

        ix_data[..ANCHOR_DISCRIMINATOR_SIZE].copy_from_slice(&Self::DISCRIMINATOR);
        ix_accounts.extend([
            AccountMeta::new_readonly(user, true),
            AccountMeta::new_readonly(global_config, false),
            AccountMeta::new(prize_pool, false),
            AccountMeta::new(curve, false),
            AccountMeta::new_readonly(mint, false),
        ]);

        Instruction::new_with_bytes(PROGRAM_ID, &ix_data, ix_accounts)
    }
}
