use crate::{constants::*, extentions::*, pda::*};

use solana_instruction::{Instruction, AccountMeta};
use solana_pubkey::Pubkey;
use spl_associated_token_account_interface::address::get_associated_token_address_with_program_id;

pub struct EmergencyDrainCurveInstruction;

pub struct EmergencyDrainCurveInstructionCtx {
    pub user: Pubkey,
    pub mint: Pubkey,
    pub recipient: Pubkey,
    /// if `curve.migration_funder` is not None, this is required
    pub migration_funder: Option<Pubkey>
}

impl Discriminator for EmergencyDrainCurveInstruction {
    const DISCRIMINATOR: [u8; ANCHOR_DISCRIMINATOR_SIZE] = EMERGENCY_DRAIN_CURVE_DISCRIMINATOR;
}

impl AccountCounts for EmergencyDrainCurveInstruction {
    const ACCOUNTS: usize = EMERGENCY_DRAIN_CURVE_ACCOUNTS;
    const REMAINING_ACCOUNTS: usize = EMERGENCY_DRAIN_CURVE_REMAINING_ACCOUNTS;
}

impl EmergencyDrainCurveInstruction {
    pub fn new(
        ctx: EmergencyDrainCurveInstructionCtx
    ) -> Instruction {
        let EmergencyDrainCurveInstructionCtx { 
            user, 
            mint, 
            recipient,
            migration_funder 
        } = ctx;

        let curve = derive_curve(&mint).0;
        let global_config = derive_global_config().0;

        let recipient_ata = get_associated_token_address_with_program_id(
            &recipient, 
            &mint, 
            &SPL_TOKEN_2022_PROGRAM_ID
        );
        let curve_ata = get_associated_token_address_with_program_id(
            &curve, 
            &mint, 
            &SPL_TOKEN_2022_PROGRAM_ID
        );

        let mut ix_data: [u8; EMERGENCY_DRAIN_CURVE_SIZE] = [0u8; EMERGENCY_DRAIN_CURVE_SIZE]; 
        let mut ix_accounts = Vec::with_capacity(Self::TOTAL_ACCOUNTS); 

        ix_data[..ANCHOR_DISCRIMINATOR_SIZE].copy_from_slice(&Self::DISCRIMINATOR);
        ix_accounts.extend([
            AccountMeta::new(user, true),
            AccountMeta::new(recipient, false),
            AccountMeta::new(curve, false),
            AccountMeta::new_readonly(mint, false),
            AccountMeta::new(recipient_ata, false),
            AccountMeta::new(curve_ata, false),
            AccountMeta::new_readonly(global_config, false),
            AccountMeta::new_readonly(SPL_TOKEN_2022_PROGRAM_ID, false),
            AccountMeta::new_readonly(SPL_ATA_PROGRAM_ID, false),
            AccountMeta::new_readonly(SYSTEM_PROGRAM_ID, false),
        ]);

        if let Some(migration_funder) = migration_funder {
            ix_accounts.push(AccountMeta::new(migration_funder, false));
        }

        Instruction::new_with_bytes(
            PROGRAM_ID, 
            &ix_data, 
            ix_accounts
        )
    }
}