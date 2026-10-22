use crate::{constants::*, extentions::*, pda::*};

use solana_instruction::{Instruction, AccountMeta};
use solana_pubkey::Pubkey;


pub struct EmergencyDrainCurveInstruction;

pub struct EmergencyDrainCurveInstructionCtx {
    pub user: Pubkey,
    pub mint: Pubkey,
    pub recipient: Pubkey
}

impl Discriminator for EmergencyDrainCurveInstruction {
    const DISCRIMINATOR: [u8; ANCHOR_DISCRIMINATOR_SIZE] = EMERGENCY_DRAIN_CURVE_DISCRIMINATOR;
}

impl EmergencyDrainCurveInstruction {
    pub fn new(EmergencyDrainCurveInstructionCtx { 
        user, 
        mint, 
        recipient 
    }: EmergencyDrainCurveInstructionCtx) -> Instruction {
        // let curve = 

        let mut ix_data: [u8; EMERGENCY_DRAIN_CURVE_SIZE] = [0u8; EMERGENCY_DRAIN_CURVE_SIZE]; 
        let mut ix_accounts = Vec::with_capacity(11);  // todo, use const EMERGENCY_DRAIN_CURVE_ACCOUNTS 

        ix_data[..ANCHOR_DISCRIMINATOR_SIZE].copy_from_slice(&EMERGENCY_DRAIN_CURVE_DISCRIMINATOR);
        ix_accounts.extend([
            AccountMeta::new(user, true),
            AccountMeta::new(recipient, false),
            
        ]);

        Instruction::new_with_bytes(
            PROGRAM_ID, 
            &ix_data, 
            ix_accounts
        );        

        todo!()
    }
}