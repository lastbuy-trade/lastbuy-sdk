use crate::{constants::*, extentions::*};

use solana_instruction::Instruction;
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
    pub fn new(ctx: EmergencyDrainCurveInstructionCtx) -> Instruction {
        todo!()
    }
}