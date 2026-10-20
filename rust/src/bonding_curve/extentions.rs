use crate::constants::*;

pub trait Discriminator {
    const DISCRIMINATOR: [u8; ANCHOR_DISCRIMINATOR_SIZE]; 
}