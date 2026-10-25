use crate::constants::*;

use solana_pubkey::Pubkey;

/// Helper function, derives addresses using [`crate::constants::PROGRAM_ID`].
pub fn find_this_pda(seeds: &[&[u8]]) -> (Pubkey, u8) {
    Pubkey::find_program_address(
        seeds, 
        &PROGRAM_ID
    )
}

pub fn derive_global_config() -> (Pubkey, u8) {
    find_this_pda(&[GLOBAL_CONFIG_SEED.as_ref()])
}

pub fn derive_curve(mint: &Pubkey) -> (Pubkey, u8) {
    find_this_pda(
        &[
            CURVE_SEED.as_ref(),
            mint.as_ref()
        ]
    )
}

pub fn derive_prize_pool(curve: &Pubkey) -> (Pubkey, u8) {
    find_this_pda(
        &[
            PRIZE_POOL_SEED.as_ref(),
            curve.as_ref()
        ]
    )
}

pub fn derive_mint_authority() -> (Pubkey, u8) {
    find_this_pda(&[MINT_AUTHORITY_SEED.as_ref()])
}

pub fn derive_pool_creator(user: &Pubkey, curve: &Pubkey) -> (Pubkey, u8) {
    find_this_pda(
        &[
            POOL_CREATOR_SEED.as_ref(),
            user.as_ref(),
            curve.as_ref()
        ]
    )
}

pub fn derive_pool_creator_state(pool_creator: &Pubkey) -> (Pubkey, u8) {
    find_this_pda(
        &[
            POOL_CREATOR_STATE_SEED.as_ref(),
            pool_creator.as_ref()
        ]
    )
}

pub fn derive_pool_state(curve: &Pubkey) -> (Pubkey, u8) {
    find_this_pda(
        &[
            POOL_STATE_SEED.as_ref(),
            curve.as_ref()
        ]
    )
}

pub fn derive_nft_mint(curve: &Pubkey) -> (Pubkey, u8) {
    find_this_pda(
        &[
            NFT_MINT_SEED.as_ref(),
            curve.as_ref()
        ]
    )
}

pub fn derive_program_data() -> (Pubkey, u8) {
    Pubkey::find_program_address(
        &[
            PROGRAM_ID.as_ref()
        ], 
        &UPGRADEABLE_LOADER_PROGRAM_ID
    )
}

/// `program_id` - raydium cpmm program id (differs with cluster)
pub fn derive_raydium_cpmm_authority(program_id: &Pubkey) -> (Pubkey, u8) {
    Pubkey::find_program_address(
        &[
            RAYDIUM_CPMM_AUTH_SEED.as_ref()
        ], 
        program_id
    )
}

/// `program_id` - raydium lock lp program id (differs with cluster)
pub fn derive_raydium_lock_authority(program_id: &Pubkey) -> (Pubkey, u8) {
    Pubkey::find_program_address(
        &[
            RAYDIUM_LOCK_AUTH_SEED.as_ref()
        ], 
        program_id
    )
}