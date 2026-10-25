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

pub fn derive_pool_creator(
    migration_funder: &Pubkey, 
    curve: &Pubkey
) -> (Pubkey, u8) {
    find_this_pda(
        &[
            POOL_CREATOR_SEED.as_ref(),
            migration_funder.as_ref(),
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

/// Non-canonical Raydium pool_state: a lastbuy PDA that signs Raydium `initialize`
pub fn derive_pool_state(curve: &Pubkey) -> (Pubkey, u8) {
    find_this_pda(
        &[
            POOL_STATE_SEED.as_ref(),
            curve.as_ref()
        ]
    )
}

/// Mint of the Burn & Earn fee NFT: a lastbuy PDA that signs `lock_cp_liquidity`
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

/// `program_id` - raydium cpmm program id (differs with cluster)
pub fn derive_raydium_cpmm_lp_mint(
    program_id: &Pubkey,
    pool_state: &Pubkey
) -> (Pubkey, u8) {
    Pubkey::find_program_address(
        &[
            RAYDIUM_CPMM_POOL_LP_MINT_SEED.as_ref(),
            pool_state.as_ref()
        ], 
        program_id
    )
}

/// `program_id` - raydium cpmm program id (differs with cluster)
pub fn derive_raydium_cpmm_vault(
    program_id: &Pubkey,
    pool_state: &Pubkey,
    mint: &Pubkey
) -> (Pubkey, u8) {
    Pubkey::find_program_address(
        &[
            RAYDIUM_CPMM_POOL_VAULT_SEED.as_ref(),
            pool_state.as_ref(),
            mint.as_ref()
        ], 
        program_id
    )
}

/// `program_id` - raydium cpmm program id (differs with cluster)
pub fn derive_raydium_cpmm_observation(
    program_id: &Pubkey,
    pool_state: &Pubkey
) -> (Pubkey, u8) {
    Pubkey::find_program_address(
        &[
            RAYDIUM_CPMM_OBSERVATION_SEED.as_ref(),
            pool_state.as_ref(),
        ], 
        program_id
    )
}

/// `program_id` - raydium lock lp program id (differs with cluster)
/// 
/// `nft_mint` - can be derived from `derive_nft_mint(curve)`
pub fn derive_raydium_lock_liquidity(
    program_id: &Pubkey,
    nft_mint: &Pubkey
) -> (Pubkey, u8) {
    Pubkey::find_program_address(
        &[
            RAYDIUM_LOCK_LIQUIDITY_SEED.as_ref(),
            nft_mint.as_ref()
        ], 
        program_id
    )
}

pub fn derive_mpl_token_metadata(nft_mint: &Pubkey) -> (Pubkey, u8) {
    Pubkey::find_program_address(
        &[
            MPL_TOKEN_METADATA_SEED.as_ref(),
            METADATA_PROGRAM_ID.as_ref(),
            nft_mint.as_ref()
        ], 
        &METADATA_PROGRAM_ID
    )
}