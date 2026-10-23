use crate::constants::*;

use solana_pubkey::Pubkey;

pub fn derive_global_config() -> (Pubkey, u8) {
    Pubkey::find_program_address(
        &[
            GLOBAL_CONFIG_SEED.as_ref()
        ], 
        &PROGRAM_ID
    )
}

pub fn derive_curve(mint: &Pubkey) -> (Pubkey, u8) {
    Pubkey::find_program_address(
        &[
            CURVE_SEED.as_ref(),
            mint.as_ref()
        ], 
        &PROGRAM_ID
    )
}

pub fn derive_prize_pool(curve: &Pubkey) -> (Pubkey, u8) {
    Pubkey::find_program_address(
        &[
            PRIZE_POOL_SEED.as_ref(),
            curve.as_ref()
        ], 
        &PROGRAM_ID
    )
}

pub fn derive_mint_authority() -> (Pubkey, u8) {
    Pubkey::find_program_address(
        &[
            MINT_AUTHORITY_SEED.as_ref()
        ], 
        &PROGRAM_ID
    )
}

pub fn derive_pool_creator(user: &Pubkey, curve: &Pubkey) -> (Pubkey, u8) {
    Pubkey::find_program_address(
        &[
            POOL_CREATOR_SEED.as_ref(),
            user.as_ref(),
            curve.as_ref()
        ], 
        &PROGRAM_ID
    )
}

pub fn derive_pool_creator_state(pool_creator: &Pubkey) -> (Pubkey, u8) {
    Pubkey::find_program_address(
        &[
            POOL_CREATOR_STATE_SEED.as_ref(),
            pool_creator.as_ref()
        ], 
        &PROGRAM_ID
    )
}

pub fn derive_pool_state(curve: &Pubkey) -> (Pubkey, u8) {
    Pubkey::find_program_address(
        &[
            POOL_STATE_SEED.as_ref(),
            curve.as_ref()
        ], 
        &PROGRAM_ID
    )
}

pub fn derive_nft_mint(curve: &Pubkey) -> (Pubkey, u8) {
    Pubkey::find_program_address(
        &[
            NFT_MINT_SEED.as_ref(),
            curve.as_ref()
        ], 
        &PROGRAM_ID
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