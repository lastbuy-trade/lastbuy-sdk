use borsh::{BorshDeserialize, BorshSerialize};

use crate::constants::*;
use crate::extentions::Discriminator;

use super::types::{AdminConfig, FeeConfig, ProtocolConfig, RaydiumConfig};

/// Global protocol configuration account
///
/// PDA: `["global_config"]`
#[derive(BorshDeserialize, BorshSerialize, Debug, Clone, PartialEq)]
pub struct GlobalConfig {
    pub protocol_version: u8,
    pub bump: u8,
    pub fees: FeeConfig,
    pub protocol: ProtocolConfig,
    pub raydium: RaydiumConfig,
    pub admin: AdminConfig,
}

impl Discriminator for GlobalConfig {
    const DISCRIMINATOR: [u8; ANCHOR_DISCRIMINATOR_SIZE] = GLOBAL_CONFIG_DISCRIMINATOR;
}
