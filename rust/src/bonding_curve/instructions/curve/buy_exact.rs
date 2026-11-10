use crate::{constants::*, extentions::*, pda::*, error::*};

use borsh::BorshSerialize;
use solana_instruction::{Instruction, AccountMeta};
use solana_pubkey::Pubkey;
use spl_associated_token_account_interface::address::get_associated_token_address_with_program_id;

pub struct BuyExactInstruction;

pub struct BuyExactInstructionCtx {
    /// is_signer: true
    pub user: Pubkey,
    pub creator: Pubkey,
    pub mint: Pubkey,
    pub fee_recipient_1: Pubkey,
    pub fee_recipient_2: Pubkey,
}

#[derive(Debug, borsh::BorshSerialize)]
pub struct BuyExactInstructionArgs {
    pub sol_in: u64,
    pub min_tokens_out: u64
}

impl Discriminator for BuyExactInstruction {
    const DISCRIMINATOR: [u8; ANCHOR_DISCRIMINATOR_SIZE] = BUY_EXACT_DISCRIMINATOR;
}

impl AccountCounts for BuyExactInstruction {
    const ACCOUNTS: usize = BUY_EXACT_ACCOUNTS;
    const REMAINING_ACCOUNTS: usize = BUY_EXACT_REMAINING_ACCOUNTS;
}

impl BuyExactInstruction {
    pub fn new(
        ctx: BuyExactInstructionCtx,
        args: BuyExactInstructionArgs
    ) -> SdkResult<Instruction> {
        let BuyExactInstructionCtx { 
            user, 
            creator,
            mint,
            fee_recipient_1,
            fee_recipient_2 
        } = ctx;

        let curve = derive_curve(&mint).0;
        let prize_pool = derive_prize_pool(&curve).0;
        let global_config = derive_global_config().0;
        let curve_ata = get_associated_token_address_with_program_id(
            &curve, 
            &mint, 
            &SPL_TOKEN_2022_PROGRAM_ID
        );
        let user_ata = get_associated_token_address_with_program_id(
            &user, 
            &mint, 
            &SPL_TOKEN_2022_PROGRAM_ID
        );
        let event_authority = derive_event_authority().0;

        let mut ix_data: Vec<u8> = Vec::with_capacity(BUY_EXACT_SIZE);
        let mut ix_accounts: Vec<AccountMeta> = Vec::with_capacity(Self::TOTAL_ACCOUNTS);

        ix_data.extend(Self::DISCRIMINATOR);
        args.serialize(&mut ix_data)?;
        
        ix_accounts.extend([
            AccountMeta::new(user, true),
            AccountMeta::new(curve, false),
            AccountMeta::new_readonly(global_config, false),
            AccountMeta::new(creator, false),
            AccountMeta::new(prize_pool, false),
            AccountMeta::new_readonly(mint, false),
            AccountMeta::new(curve_ata, false),
            AccountMeta::new(user_ata, false),
            AccountMeta::new_readonly(SPL_TOKEN_2022_PROGRAM_ID, false),
            AccountMeta::new_readonly(SYSTEM_PROGRAM_ID, false),
            AccountMeta::new_readonly(event_authority, false),
            AccountMeta::new_readonly(PROGRAM_ID, false),
            // remaining accounts
            AccountMeta::new(fee_recipient_1, false),
            AccountMeta::new(fee_recipient_2, false),
        ]);

        Ok(Instruction::new_with_bytes(
            PROGRAM_ID, 
            &ix_data, 
            ix_accounts
        ))
    }
}