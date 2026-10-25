use crate::{constants::*, extentions::*, pda::*, error::*};

use borsh::BorshSerialize;
use solana_instruction::{Instruction, AccountMeta};
use solana_pubkey::Pubkey;
use spl_associated_token_account_interface::address::get_associated_token_address_with_program_id;

pub struct LaunchInstruction;

pub struct LaunchInstructionCtx {
    /// is_signer: true
    pub user: Pubkey,
    /// is_signer: true
    /// 
    /// arbitrary account 
    pub mint: Pubkey
}

#[derive(Debug, borsh::BorshSerialize)]
pub struct LaunchInstructionArgs {
    pub token_name: String,
    pub token_symbol: String,
    pub token_uri: String
}

impl Discriminator for LaunchInstruction {
    const DISCRIMINATOR: [u8; ANCHOR_DISCRIMINATOR_SIZE] = LAUNCH_DISCRIMINATOR;
}

impl AccountCounts for LaunchInstruction {
    const ACCOUNTS: usize = LAUNCH_ACCOUNTS;
}

impl LaunchInstruction {
    pub fn new(
        ctx: LaunchInstructionCtx,
        args: LaunchInstructionArgs
    ) -> SdkResult<Instruction> {
        let LaunchInstructionCtx { user, mint } = ctx;

        let curve = derive_curve(&mint).0;
        let prize_pool = derive_prize_pool(&curve).0;
        let global_config = derive_global_config().0;
        let mint_authority = derive_mint_authority().0;
        let curve_ata = get_associated_token_address_with_program_id(
            &curve, 
            &mint, 
            &SPL_TOKEN_2022_PROGRAM_ID
        );

        // exact payload size cannot be known at compile time
        let mut ix_data: Vec<u8> = Vec::with_capacity(launch_size(
            &args.token_name, 
            &args.token_symbol, 
            &args.token_uri
        ));
        let mut ix_accounts: Vec<AccountMeta> = Vec::with_capacity(Self::TOTAL_ACCOUNTS);

        ix_data.extend(Self::DISCRIMINATOR);
        args.serialize(&mut ix_data)?;
        
        ix_accounts.extend([
            AccountMeta::new(user, true),
            AccountMeta::new(mint, true),
            AccountMeta::new(curve, false),
            AccountMeta::new(prize_pool, false),
            AccountMeta::new_readonly(global_config, false),
            AccountMeta::new_readonly(mint_authority, false),
            AccountMeta::new(curve_ata, false),
            AccountMeta::new_readonly(SPL_TOKEN_2022_PROGRAM_ID, false),
            AccountMeta::new_readonly(SPL_ATA_PROGRAM_ID, false),
            AccountMeta::new_readonly(SYSTEM_PROGRAM_ID, false)
        ]);

        Ok(Instruction::new_with_bytes(
            PROGRAM_ID, 
            &ix_data, 
            ix_accounts
        ))
    }
}