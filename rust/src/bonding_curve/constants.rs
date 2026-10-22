pub use discriminators::*;
pub use seeds::*;
pub use layout::*;

use solana_pubkey::{Pubkey, pubkey};

#[cfg(feature = "mainnet-beta")]
pub const PROGRAM_ID: Pubkey = pubkey!("Lbuyii1PVUvt9xToGuyUf6CQCDbdRRzzo8SsHuNYMkf");

#[cfg(feature = "devnet")]
pub const PROGRAM_ID: Pubkey = pubkey!("Lbuyii1PVUvt9xToGuyUf6CQCDbdRRzzo8SsHuNYMkf");

#[cfg(feature = "testnet")]
pub const PROGRAM_ID: Pubkey = pubkey!("Lbuyii1PVUvt9xToGuyUf6CQCDbdRRzzo8SsHuNYMkf");

pub mod discriminators {
    pub use accounts::*;
    pub use events::*;
    pub use instructions::*;

    pub const ANCHOR_DISCRIMINATOR_SIZE: usize = 8;

    pub mod accounts {
        use super::*;

        /// Anchor account discriminator for `CurveState`
        pub const CURVE_STATE_DISCRIMINATOR: [u8; ANCHOR_DISCRIMINATOR_SIZE] =
            [198, 152, 48, 255, 91, 4, 10, 197];

        /// Anchor account discriminator for `GlobalConfig`
        pub const GLOBAL_CONFIG_DISCRIMINATOR: [u8; ANCHOR_DISCRIMINATOR_SIZE] =
            [149, 8, 156, 202, 160, 252, 176, 217];

        /// Anchor account discriminator for `PoolCreatorState`
        pub const POOL_CREATOR_STATE_DISCRIMINATOR: [u8; ANCHOR_DISCRIMINATOR_SIZE] =
            [240, 22, 176, 10, 233, 85, 3, 183];

        /// Anchor account discriminator for `PrizePool`
        pub const PRIZE_POOL_DISCRIMINATOR: [u8; ANCHOR_DISCRIMINATOR_SIZE] =
            [51, 88, 38, 85, 206, 166, 162, 156];
    }

    pub mod events {
        use super::*;

        /// Anchor event discriminator for `BuyEvent`
        pub const BUY_EVENT_DISCRIMINATOR: [u8; ANCHOR_DISCRIMINATOR_SIZE] =
            [103, 244, 82, 31, 44, 245, 119, 119];

        /// Anchor event discriminator for `ClaimCreatorFeeEvent`
        pub const CLAIM_CREATOR_FEE_EVENT_DISCRIMINATOR: [u8; ANCHOR_DISCRIMINATOR_SIZE] =
            [151, 172, 14, 31, 51, 38, 77, 175];

        /// Anchor event discriminator for `ClaimPrizeEvent`
        pub const CLAIM_PRIZE_EVENT_DISCRIMINATOR: [u8; ANCHOR_DISCRIMINATOR_SIZE] =
            [121, 119, 127, 115, 209, 198, 28, 0];

        /// Anchor event discriminator for `DefineAlternativeWinnerEvent`
        pub const DEFINE_ALTERNATIVE_WINNER_EVENT_DISCRIMINATOR: [u8; ANCHOR_DISCRIMINATOR_SIZE] =
            [90, 55, 84, 22, 113, 7, 148, 151];

        /// Anchor event discriminator for `DefineWinnerEvent`
        pub const DEFINE_WINNER_EVENT_DISCRIMINATOR: [u8; ANCHOR_DISCRIMINATOR_SIZE] =
            [6, 213, 65, 173, 77, 44, 94, 245];

        /// Anchor event discriminator for `EmergencyDrainCurveEvent`
        pub const EMERGENCY_DRAIN_CURVE_EVENT_DISCRIMINATOR: [u8; ANCHOR_DISCRIMINATOR_SIZE] =
            [84, 251, 214, 87, 67, 174, 50, 88];

        /// Anchor event discriminator for `FundMigrationEvent`
        pub const FUND_MIGRATION_EVENT_DISCRIMINATOR: [u8; ANCHOR_DISCRIMINATOR_SIZE] =
            [204, 159, 33, 56, 183, 38, 153, 110];

        /// Anchor event discriminator for `GraduationEvent`
        pub const GRADUATION_EVENT_DISCRIMINATOR: [u8; ANCHOR_DISCRIMINATOR_SIZE] =
            [10, 246, 223, 127, 48, 98, 149, 55];

        /// Anchor event discriminator for `LaunchEvent`
        pub const LAUNCH_EVENT_DISCRIMINATOR: [u8; ANCHOR_DISCRIMINATOR_SIZE] =
            [27, 193, 47, 130, 115, 92, 239, 94];

        /// Anchor event discriminator for `MigrationEvent`
        pub const MIGRATION_EVENT_DISCRIMINATOR: [u8; ANCHOR_DISCRIMINATOR_SIZE] =
            [255, 202, 76, 147, 91, 231, 73, 22];

        /// Anchor event discriminator for `PrepareMigrationEvent`
        pub const PREPARE_MIGRATION_EVENT_DISCRIMINATOR: [u8; ANCHOR_DISCRIMINATOR_SIZE] =
            [22, 141, 98, 25, 142, 126, 5, 218];

        /// Anchor event discriminator for `SellEvent`
        pub const SELL_EVENT_DISCRIMINATOR: [u8; ANCHOR_DISCRIMINATOR_SIZE] =
            [62, 47, 55, 10, 165, 3, 220, 42];
    }

    pub mod instructions {
        use super::*;

        /// Anchor instruction discriminator for `buy_exact`
        pub const BUY_EXACT_DISCRIMINATOR: [u8; ANCHOR_DISCRIMINATOR_SIZE] =
            [2, 162, 8, 52, 139, 46, 160, 13];

        /// Anchor instruction discriminator for `claim_creator_fee`
        pub const CLAIM_CREATOR_FEE_DISCRIMINATOR: [u8; ANCHOR_DISCRIMINATOR_SIZE] =
            [26, 97, 138, 203, 132, 171, 141, 252];

        /// Anchor instruction discriminator for `claim_prize_idempotent`
        pub const CLAIM_PRIZE_IDEMPOTENT_DISCRIMINATOR: [u8; ANCHOR_DISCRIMINATOR_SIZE] =
            [140, 9, 103, 107, 31, 27, 249, 190];

        /// Anchor instruction discriminator for `define_winner_idempotent`
        pub const DEFINE_WINNER_IDEMPOTENT_DISCRIMINATOR: [u8; ANCHOR_DISCRIMINATOR_SIZE] =
            [188, 241, 34, 137, 125, 48, 69, 200];

        /// Anchor instruction discriminator for `emergency_drain_curve`
        pub const EMERGENCY_DRAIN_CURVE_DISCRIMINATOR: [u8; ANCHOR_DISCRIMINATOR_SIZE] =
            [158, 244, 94, 60, 201, 81, 152, 57];

        /// Anchor instruction discriminator for `fund_migration`
        pub const FUND_MIGRATION_DISCRIMINATOR: [u8; ANCHOR_DISCRIMINATOR_SIZE] =
            [128, 94, 3, 7, 224, 221, 30, 177];

        /// Anchor instruction discriminator for `initialize_global`
        pub const INITIALIZE_GLOBAL_DISCRIMINATOR: [u8; ANCHOR_DISCRIMINATOR_SIZE] =
            [47, 225, 15, 112, 86, 51, 190, 231];

        /// Anchor instruction discriminator for `launch`
        pub const LAUNCH_DISCRIMINATOR: [u8; ANCHOR_DISCRIMINATOR_SIZE] =
            [153, 241, 93, 225, 22, 69, 74, 61];

        /// Anchor instruction discriminator for `migrate`
        pub const MIGRATE_DISCRIMINATOR: [u8; ANCHOR_DISCRIMINATOR_SIZE] =
            [155, 234, 231, 146, 236, 158, 162, 30];

        /// Anchor instruction discriminator for `prepare_migration`
        pub const PREPARE_MIGRATION_DISCRIMINATOR: [u8; ANCHOR_DISCRIMINATOR_SIZE] =
            [59, 118, 102, 18, 9, 133, 23, 172];

        /// Anchor instruction discriminator for `sell_exact`
        pub const SELL_EXACT_DISCRIMINATOR: [u8; ANCHOR_DISCRIMINATOR_SIZE] =
            [168, 188, 137, 139, 68, 70, 192, 114];

        /// Anchor instruction discriminator for `update_global`
        pub const UPDATE_GLOBAL_DISCRIMINATOR: [u8; ANCHOR_DISCRIMINATOR_SIZE] =
            [90, 152, 240, 21, 199, 38, 72, 20];
    }
}

pub mod seeds {
    pub const GLOBAL_CONFIG_SEED: &str = "global_config";

    pub const CURVE_SEED: &str = "curve";

    pub const PRIZE_POOL_SEED: &str = "prize_pool";

    pub const MINT_AUTHORITY_SEED: &str = "mint_authority";

    pub const POOL_CREATOR_SEED: &str = "pool_creator";

    pub const POOL_CREATOR_STATE_SEED: &str = "pool_creator_state";

    pub const POOL_STATE_SEED: &str = "pool_state";

    pub const NFT_MINT_SEED: &str = "nft_mint";
}

pub mod layout {
    use super::*;

    pub use payloads::*;

    pub mod payloads {
        use super::*;
        
        /// Borsh layout size of `GlobalConfigArgs` (without instruction discriminator):
        ///
        /// protocol_version: u8,
        /// fees: FeeConfig (4 x u16 + 2 x Pubkey),
        /// protocol: ProtocolConfig (i64 + u64 + Pubkey),
        /// raydium: RaydiumConfig (4 x Pubkey + u64),
        /// admin: AdminConfig (8 x bool)
        pub const GLOBAL_CONFIG_ARGS_SIZE: usize =
            1 + (4 * 2 + 32 * 2) + (8 + 8 + 32) + (32 * 4 + 8) + 8;

        /// Borsh layout size of `LaunchArgs` length prefixes (3 x u32), without the
        /// instruction discriminator and the string bytes themselves
        pub const LAUNCH_ARGS_MIN_SIZE: usize = 4 + 4 + 4;

        /// DISCRIMINATOR,
        /// sol_in: u64,
        /// min_tokens_out: u64
        pub const BUY_EXACT_SIZE: usize = ANCHOR_DISCRIMINATOR_SIZE + 8 + 8;

        /// DISCRIMINATOR (no args)
        pub const CLAIM_CREATOR_FEE_SIZE: usize = ANCHOR_DISCRIMINATOR_SIZE;

        /// DISCRIMINATOR (no args)
        pub const CLAIM_PRIZE_IDEMPOTENT_SIZE: usize = ANCHOR_DISCRIMINATOR_SIZE;

        /// DISCRIMINATOR (no args)
        pub const DEFINE_WINNER_IDEMPOTENT_SIZE: usize = ANCHOR_DISCRIMINATOR_SIZE;

        /// DISCRIMINATOR (no args)
        pub const EMERGENCY_DRAIN_CURVE_SIZE: usize = ANCHOR_DISCRIMINATOR_SIZE;

        /// DISCRIMINATOR (no args)
        pub const FUND_MIGRATION_SIZE: usize = ANCHOR_DISCRIMINATOR_SIZE;

        /// DISCRIMINATOR,
        /// args: GlobalConfigArgs
        pub const INITIALIZE_GLOBAL_SIZE: usize = ANCHOR_DISCRIMINATOR_SIZE + GLOBAL_CONFIG_ARGS_SIZE;

        /// DISCRIMINATOR (no args)
        pub const MIGRATE_SIZE: usize = ANCHOR_DISCRIMINATOR_SIZE;

        /// DISCRIMINATOR (no args)
        pub const PREPARE_MIGRATION_SIZE: usize = ANCHOR_DISCRIMINATOR_SIZE;

        /// DISCRIMINATOR,
        /// tokens_in: u64,
        /// min_sol_out: u64
        pub const SELL_EXACT_SIZE: usize = ANCHOR_DISCRIMINATOR_SIZE + 8 + 8;

        /// DISCRIMINATOR,
        /// args: GlobalConfigArgs
        pub const UPDATE_GLOBAL_SIZE: usize = ANCHOR_DISCRIMINATOR_SIZE + GLOBAL_CONFIG_ARGS_SIZE;

        /// Exact instruction data size of `launch` for the given metadata:
        ///
        /// DISCRIMINATOR,
        /// args: LaunchArgs {
        ///     token_name: string,
        ///     token_symbol: string,
        ///     token_uri: string,
        /// }
        pub const fn launch_size(token_name: &str, token_symbol: &str, token_uri: &str) -> usize {
            ANCHOR_DISCRIMINATOR_SIZE
                + LAUNCH_ARGS_MIN_SIZE
                + token_name.len()
                + token_symbol.len()
                + token_uri.len()
        }
    }
}