use crate::{constants::*, extentions::*, pda::*};

use solana_instruction::{Instruction, AccountMeta};
use solana_pubkey::Pubkey;

pub struct ClaimPrizeIdempotentInstruction;

pub struct ClaimPrizeIdempotentInstructionCtx {
    /// is_signer: true
    pub user: Pubkey,
    pub mint: Pubkey,
}

impl Discriminator for ClaimPrizeIdempotentInstruction {
    const DISCRIMINATOR: [u8; ANCHOR_DISCRIMINATOR_SIZE] = CLAIM_PRIZE_IDEMPOTENT_DISCRIMINATOR;
}

impl AccountCounts for ClaimPrizeIdempotentInstruction {
    const ACCOUNTS: usize = CLAIM_PRIZE_IDEMPOTENT_ACCOUNTS;
}

impl ClaimPrizeIdempotentInstruction {
    pub fn new(ctx: ClaimPrizeIdempotentInstructionCtx) -> Instruction {
        let ClaimPrizeIdempotentInstructionCtx { user, mint } = ctx;

        let global_config = derive_global_config().0;
        let curve = derive_curve(&mint).0;
        let prize_pool = derive_prize_pool(&curve).0;

        let mut ix_data: [u8; CLAIM_PRIZE_IDEMPOTENT_SIZE] = [0u8; CLAIM_PRIZE_IDEMPOTENT_SIZE];
        let mut ix_accounts: Vec<AccountMeta> = Vec::with_capacity(Self::TOTAL_ACCOUNTS);

        ix_data[..ANCHOR_DISCRIMINATOR_SIZE].copy_from_slice(&Self::DISCRIMINATOR);
        ix_accounts.extend([
            AccountMeta::new(user, true),
            AccountMeta::new_readonly(global_config, false),
            AccountMeta::new(curve, false),
            AccountMeta::new_readonly(mint, false),
            AccountMeta::new(prize_pool, false),
        ]);

        Instruction::new_with_bytes(PROGRAM_ID, &ix_data, ix_accounts)
    }
}
