pub use discriminators::*;

pub mod discriminators {
    pub use accounts::*;
    pub use events::*;
    pub use instructions::*;
    
    pub const ANCHOR_DISCRIMINATOR_SIZE: usize = 8;

    pub mod accounts {
        use super::*;
    }

    pub mod events {
        use super::*;
    }

    pub mod instructions {
        use super::*;
    }
}