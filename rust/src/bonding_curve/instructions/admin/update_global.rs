use crate::{constants::*, extentions::*, types::*, error::*, pda::*};

use borsh::BorshSerialize;
use solana_instruction::{Instruction, AccountMeta};
use solana_pubkey::Pubkey;

pub struct UpdateGlobalInstruction;

pub struct UpdateGlobalCtx {
    pub user: Pubkey
}

#[derive(borsh::BorshSerialize)]
pub struct UpdateGlobalArgs {
    pub protocol_version: u8,
    pub fees: FeeConfig,
    pub protocol: ProtocolConfig,
    pub raydium: RaydiumConfig,
    pub admin: AdminConfig,
}

impl Discriminator for UpdateGlobalInstruction {
    const DISCRIMINATOR: [u8; ANCHOR_DISCRIMINATOR_SIZE] = UPDATE_GLOBAL_DISCRIMINATOR;
}

impl UpdateGlobalInstruction {
    pub fn new(
        ctx: UpdateGlobalCtx,
        args: UpdateGlobalArgs
    ) -> SdkResult<Instruction> {
        let UpdateGlobalCtx { user } = ctx;

        let global_config = derive_global_config().0;

        let mut ix_data: Vec<u8> = Vec::with_capacity(UPDATE_GLOBAL_SIZE);
        let mut ix_accounts: Vec<AccountMeta> = Vec::with_capacity(UPDATE_GLOBAL_ACCOUNTS);
        
        ix_data.extend(UPDATE_GLOBAL_DISCRIMINATOR);
        args.serialize(&mut ix_data)?;

        ix_accounts.extend([
            AccountMeta::new(user, true),
            AccountMeta::new(global_config, false),
        ]);

        Ok(Instruction::new_with_bytes(
            PROGRAM_ID, 
            &ix_data, 
            ix_accounts
        ))
    }
}