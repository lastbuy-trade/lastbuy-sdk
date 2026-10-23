use crate::{constants::*, extentions::*};

use solana_instruction::{Instruction, AccountMeta};
use solana_pubkey::Pubkey;

pub struct InitializeGlobalInstruction;

pub struct InitializeGlobalInstructionCtx {
    pub user: Pubkey
}

pub struct InitializeGlobalInstructionArgs {
    
}

impl Discriminator for InitializeGlobalInstruction {
    const DISCRIMINATOR: [u8; ANCHOR_DISCRIMINATOR_SIZE] = INITIALIZE_GLOBAL_DISCRIMINATOR;
}

impl InitializeGlobalInstruction {
    pub fn new(
        ctx: InitializeGlobalInstructionCtx, 
        args: InitializeGlobalInstructionArgs
    ) -> Instruction {
        todo!()
    }
}