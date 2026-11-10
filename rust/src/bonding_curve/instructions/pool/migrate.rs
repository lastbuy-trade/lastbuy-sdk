use crate::{constants::*, extentions::*, pda::*};

use solana_instruction::{Instruction, AccountMeta};
use solana_pubkey::Pubkey;
use spl_associated_token_account_interface::address::get_associated_token_address_with_program_id;

/// # IMPORTANT
/// 
/// * ### only V1 transactions can handle these instructions, solana core must be >= 4.0.0
///
/// * ### must be called in pair with [`super::PrepareMigrationInstruction`]
///
/// * ### order matters. example:
///  
/// instruction-1: PrepareMigrationInstruction
/// 
/// instruction-2: MigrateInstruction
pub struct MigrateInstruction;

pub struct MigrateInstructionCtx {
    /// is_signer: true
    /// 
    /// Can be any user either migration watcher or external bot
    pub user: Pubkey,
    /// - Can be distinct from `user`
    /// - Can be fetched from `curve` PDA
    pub migration_funder: Pubkey,
    pub mint: Pubkey,
    /// - Can be fetched from `global_config` PDA, not recommended to provide as a const
    pub raydium_amm_config: Pubkey,
    /// - Can be fetched from `global_config` PDA or provided as a const
    pub raydium_fee_account: Pubkey,
    /// - Can be fetched from `global_config` PDA or provided as a const
    pub raydium_cpmm_program: Pubkey,
    /// - Can be fetched from `global_config` PDA or provided as a const
    pub raydium_lock_program: Pubkey
}

impl Discriminator for MigrateInstruction {
    const DISCRIMINATOR: [u8; ANCHOR_DISCRIMINATOR_SIZE] = MIGRATE_DISCRIMINATOR;
}

impl AccountCounts for MigrateInstruction {
    const ACCOUNTS: usize = MIGRATE_ACCOUNTS;
    const REMAINING_ACCOUNTS: usize = MIGRATE_REMAINING_ACCOUNTS;
}

impl MigrateInstruction {
    pub fn new(
        ctx: MigrateInstructionCtx
    ) -> Instruction {
        let MigrateInstructionCtx { 
            user, 
            migration_funder,
            mint, 
            raydium_amm_config,
            raydium_fee_account, 
            raydium_cpmm_program, 
            raydium_lock_program 
        } = ctx;

        let curve = derive_curve(&mint).0;
        let curve_ata = get_associated_token_address_with_program_id(
            &curve, 
            &mint, 
            &SPL_TOKEN_2022_PROGRAM_ID
        );
        let nft_mint = derive_nft_mint(&curve).0;
        let pool_creator = derive_pool_creator(&migration_funder, &curve).0;
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
        let nft_token_account = get_associated_token_address_with_program_id(
            &pool_creator, 
            &nft_mint, 
            &SPL_TOKEN_PROGRAM_ID
        );
        let nft_metadata_account = derive_mpl_token_metadata(&nft_mint).0; 
        let global_config = derive_global_config().0;
        let pool_state = derive_pool_state(&curve).0;
        
        // remaining accounts
        let raydium_cpmm_authority = derive_raydium_cpmm_authority(&raydium_cpmm_program).0;
        let raydium_lock_authority = derive_raydium_lock_authority(&raydium_lock_program).0;
        let raydium_lp_mint = derive_raydium_cpmm_lp_mint(&raydium_cpmm_program, &pool_state).0;
        let raydium_pool_creator_lp_token_account = get_associated_token_address_with_program_id(
            &pool_creator, 
            &raydium_lp_mint, 
            &SPL_TOKEN_PROGRAM_ID
        );
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
        let raydium_observation_state = derive_raydium_cpmm_observation(
            &raydium_cpmm_program, 
            &pool_state
        ).0;
        let raydium_locked_liquidity = derive_raydium_lock_liquidity(&raydium_lock_program, &nft_mint).0;
        let raydium_locked_lp_vault = get_associated_token_address_with_program_id(
            &raydium_lock_authority, 
            &raydium_lp_mint, 
            &SPL_TOKEN_PROGRAM_ID
        );
        let event_authority = derive_event_authority().0;

        let mut ix_data: [u8; MIGRATE_SIZE] = [0u8; MIGRATE_SIZE]; 
        let mut ix_accounts: Vec<AccountMeta> = Vec::with_capacity(Self::TOTAL_ACCOUNTS);

        ix_data[..ANCHOR_DISCRIMINATOR_SIZE].copy_from_slice(&Self::DISCRIMINATOR);
        ix_accounts.extend([
            AccountMeta::new(user, true),
            AccountMeta::new(nft_mint, false),
            AccountMeta::new(pool_creator, false),
            AccountMeta::new(pool_creator_state, false),
            AccountMeta::new(curve, false),
            AccountMeta::new_readonly(global_config, false),
            AccountMeta::new(pool_creator_token_account_0, false),
            AccountMeta::new(pool_creator_token_account_1, false),
            AccountMeta::new(curve_ata, false),
            AccountMeta::new_readonly(WSOL_ID, false),
            AccountMeta::new_readonly(mint, false),
            AccountMeta::new(pool_state, false),
            AccountMeta::new_readonly(raydium_amm_config, false),
            AccountMeta::new(raydium_fee_account, false),
            AccountMeta::new_readonly(SPL_TOKEN_PROGRAM_ID, false),
            AccountMeta::new_readonly(SPL_TOKEN_2022_PROGRAM_ID, false),
            AccountMeta::new_readonly(SPL_ATA_PROGRAM_ID, false),
            AccountMeta::new_readonly(raydium_cpmm_program, false),
            AccountMeta::new_readonly(raydium_lock_program, false),
            AccountMeta::new_readonly(METADATA_PROGRAM_ID, false),
            AccountMeta::new_readonly(SYSTEM_PROGRAM_ID, false),
            AccountMeta::new_readonly(RENT_SYSVAR_ID, false),
            AccountMeta::new_readonly(event_authority, false),
            AccountMeta::new_readonly(PROGRAM_ID, false),
            // remaining accounts
            AccountMeta::new_readonly(raydium_cpmm_authority, false),
            AccountMeta::new_readonly(raydium_lock_authority, false),
            AccountMeta::new(raydium_lp_mint, false),
            AccountMeta::new(raydium_pool_creator_lp_token_account, false),
            AccountMeta::new(raydium_pool_vault_token_account_0, false),
            AccountMeta::new(raydium_pool_vault_token_account_1, false),
            AccountMeta::new(raydium_observation_state, false),
            AccountMeta::new(nft_token_account, false),
            AccountMeta::new(raydium_locked_liquidity, false),
            AccountMeta::new(raydium_locked_lp_vault, false),
            AccountMeta::new(nft_metadata_account, false)
        ]);

        Instruction::new_with_bytes(
            PROGRAM_ID, 
            &ix_data, 
            ix_accounts
        )
    }
}