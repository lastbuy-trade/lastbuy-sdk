use crate::constants::*;

pub trait Discriminator {
    const DISCRIMINATOR: [u8; ANCHOR_DISCRIMINATOR_SIZE]; 
}

pub trait AccountCounts {
    const ACCOUNTS: usize;
    const REMAINING_ACCOUNTS: usize = 0;

    const TOTAL_ACCOUNTS: usize = Self::ACCOUNTS + Self::REMAINING_ACCOUNTS;
}