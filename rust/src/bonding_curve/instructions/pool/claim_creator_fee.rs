use crate::{constants::*, extentions::*, pda::*};

use solana_instruction::{Instruction, AccountMeta};
use solana_pubkey::Pubkey;
use spl_associated_token_account_interface::address::get_associated_token_address_with_program_id;

pub struct ClaimCreatorFeeInstruction;

pub struct ClaimCreatorFeeInstructionCtx {
    /// is_signer: true
    /// 
    /// must equal to `curve.migration_funder`
    pub user: Pubkey,
    pub mint: Pubkey,
    /// must be equal to `global_config.raydium.raydium_amm_config`
    pub raydium_amm_config: Pubkey,
    pub raydium_cpmm_program: Pubkey,
    pub raydium_lock_program: Pubkey,
    pub fee_recipient_1: Pubkey,
    pub fee_recipient_2: Pubkey,
}

impl Discriminator for ClaimCreatorFeeInstruction {
    const DISCRIMINATOR: [u8; ANCHOR_DISCRIMINATOR_SIZE] = CLAIM_CREATOR_FEE_DISCRIMINATOR;
}

impl AccountCounts for ClaimCreatorFeeInstruction {
    const ACCOUNTS: usize = CLAIM_CREATOR_FEE_ACCOUNTS;
    const REMAINING_ACCOUNTS: usize = CLAIM_CREATOR_FEE_REMAINING_ACCOUNTS;
}

impl ClaimCreatorFeeInstruction {
    pub fn new(
        ctx: ClaimCreatorFeeInstructionCtx
    ) -> Instruction {
        let ClaimCreatorFeeInstructionCtx { 
            user, 
            mint, 
            raydium_amm_config, 
            raydium_cpmm_program, 
            raydium_lock_program, 
            fee_recipient_1, 
            fee_recipient_2 
        } = ctx;
        
        let curve = derive_curve(&mint).0;
        let nft_mint = derive_nft_mint(&curve).0;
        // user is the migration_funder
        let pool_creator = derive_pool_creator(&user, &curve).0;
        let pool_creator_state = derive_pool_creator_state(&pool_creator).0;
        let pool_creator_token_account_0 = get_associated_token_address_with_program_id(
            &pool_creator, 
            &WSOL_ID, 
            &SPL_TOKEN_PROGRAM_ID
        );
        let pool_creator_token_account_1 = get_associated_token_address_with_program_id(
            &pool_creator, 
            &mint, 
            &SPL_TOKEN_2022_PROGRAM_ID
        );
        let user_token_account_1 = get_associated_token_address_with_program_id(
            &user, 
            &mint, 
            &SPL_TOKEN_2022_PROGRAM_ID
        );
        let nft_token_account = get_associated_token_address_with_program_id(
            &pool_creator, 
            &nft_mint, 
            &SPL_TOKEN_PROGRAM_ID
        );
        let global_config = derive_global_config().0;
        let pool_state = derive_pool_state(&curve).0;
        
        // remaining accounts
        let raydium_cpmm_authority = derive_raydium_cpmm_authority(&raydium_cpmm_program).0;
        let raydium_lock_authority = derive_raydium_lock_authority(&raydium_lock_program).0;
        let raydium_lp_mint = derive_raydium_cpmm_lp_mint(&raydium_cpmm_program, &pool_state).0;
        let raydium_pool_vault_token_account_0 = derive_raydium_cpmm_vault(
            &raydium_cpmm_program, 
            &pool_state, 
            &WSOL_ID
        ).0;
        let raydium_pool_vault_token_account_1 = derive_raydium_cpmm_vault(
            &raydium_cpmm_program, 
            &pool_state, 
            &mint
        ).0;
        let raydium_locked_liquidity = derive_raydium_lock_liquidity(&raydium_lock_program, &nft_mint).0;
        let raydium_locked_lp_vault = get_associated_token_address_with_program_id(
            &raydium_lock_authority, 
            &raydium_lp_mint, 
            &SPL_TOKEN_PROGRAM_ID
        );
        let event_authority = derive_event_authority().0; 

        let mut ix_data: [u8; CLAIM_CREATOR_FEE_SIZE] = [0u8; CLAIM_CREATOR_FEE_SIZE]; 
        let mut ix_accounts: Vec<AccountMeta> = Vec::with_capacity(Self::TOTAL_ACCOUNTS);

        ix_data[..ANCHOR_DISCRIMINATOR_SIZE].copy_from_slice(&Self::DISCRIMINATOR);
        ix_accounts.extend([
            AccountMeta::new(user, true),
            AccountMeta::new_readonly(nft_mint, false),
            AccountMeta::new(pool_creator, false),
            AccountMeta::new_readonly(pool_creator_state, false),
            AccountMeta::new(pool_creator_token_account_0, false),
            AccountMeta::new(pool_creator_token_account_1, false),
            AccountMeta::new(user_token_account_1, false),
            AccountMeta::new(nft_token_account, false),
            AccountMeta::new_readonly(curve, false),
            AccountMeta::new_readonly(global_config, false),
            AccountMeta::new_readonly(WSOL_ID, false),
            AccountMeta::new_readonly(mint, false),
            AccountMeta::new(pool_state, false),
            AccountMeta::new_readonly(raydium_amm_config, false),
            AccountMeta::new_readonly(SPL_TOKEN_PROGRAM_ID, false),
            AccountMeta::new_readonly(SPL_TOKEN_2022_PROGRAM_ID, false),
            AccountMeta::new_readonly(SPL_ATA_PROGRAM_ID, false),
            AccountMeta::new_readonly(raydium_cpmm_program, false),
            AccountMeta::new_readonly(raydium_lock_program, false),
            AccountMeta::new_readonly(MEMO_PROGRAM_ID, false),
            AccountMeta::new_readonly(SYSTEM_PROGRAM_ID, false),
            AccountMeta::new_readonly(event_authority, false),
            AccountMeta::new_readonly(PROGRAM_ID, false),
            // remaining accounts
            AccountMeta::new_readonly(raydium_cpmm_authority, false),
            AccountMeta::new_readonly(raydium_lock_authority, false),
            AccountMeta::new(raydium_lp_mint, false),
            AccountMeta::new(raydium_pool_vault_token_account_0, false),
            AccountMeta::new(raydium_pool_vault_token_account_1, false),
            AccountMeta::new(raydium_locked_liquidity, false),
            AccountMeta::new(raydium_locked_lp_vault, false),
            AccountMeta::new(fee_recipient_1, false),
            AccountMeta::new(fee_recipient_2, false),
        ]);

        Instruction::new_with_bytes(
            PROGRAM_ID, 
            &ix_data, 
            ix_accounts
        )
    }
}