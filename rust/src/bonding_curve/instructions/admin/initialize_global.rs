use crate::{constants::*, extentions::*, types::*, error::*, pda::*};

use borsh::BorshSerialize;
use solana_instruction::{Instruction, AccountMeta};
use solana_pubkey::Pubkey;

pub struct InitializeGlobalInstruction;

pub struct InitializeGlobalInstructionCtx {
    pub user: Pubkey
}

#[derive(borsh::BorshSerialize)]
pub struct InitializeGlobalInstructionArgs {
    pub protocol_version: u8,
    pub fees: FeeConfig,
    pub protocol: ProtocolConfig,
    pub raydium: RaydiumConfig,
    pub admin: AdminConfig,
}

impl Discriminator for InitializeGlobalInstruction {
    const DISCRIMINATOR: [u8; ANCHOR_DISCRIMINATOR_SIZE] = INITIALIZE_GLOBAL_DISCRIMINATOR;
}

impl InitializeGlobalInstruction {
    pub fn new(
        ctx: InitializeGlobalInstructionCtx, 
        args: InitializeGlobalInstructionArgs
    ) -> SdkResult<Instruction> {
        let InitializeGlobalInstructionCtx { user } = ctx;

        let global_config = derive_global_config().0;
        let program_data = derive_program_data().0;
        
        let mut ix_data: Vec<u8> = Vec::with_capacity(INITIALIZE_GLOBAL_SIZE);
        let mut ix_accounts: Vec<AccountMeta> = Vec::with_capacity(INITIALIZE_GLOBAL_ACCOUNTS);

        args.serialize(&mut ix_data)?;
        ix_accounts.extend([
            AccountMeta::new(user, true),
            AccountMeta::new(global_config, false),
            AccountMeta::new_readonly(PROGRAM_ID, false),
            AccountMeta::new_readonly(program_data, false),
            AccountMeta::new_readonly(SYSTEM_PROGRAM_ID, false)
        ]);

        Ok(Instruction::new_with_bytes(
            PROGRAM_ID, 
            &ix_data, 
            ix_accounts
        ))
    }
}