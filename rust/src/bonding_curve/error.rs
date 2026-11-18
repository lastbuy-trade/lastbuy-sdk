#[cfg(feature = "parse-events")]
#[derive(Debug, thiserror::Error)]
pub enum ParseEventError {
    #[error("Invalid event instruction data")]
    InvalidData,

    #[error("Invalid event instruction discriminator")]
    InvalidInstructionDiscriminator,

    #[error("Invalid event discriminator")]
    InvalidDiscriminator,
}

#[cfg(feature = "math")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, thiserror::Error)]
pub enum MathError {
    #[error("Arithmetic overflow or division by zero")]
    Overflow,

    #[error("Insufficient output amount")]
    InsufficientOutputAmount,
}

#[derive(Debug, thiserror::Error)]
pub enum SdkError {
    #[error(transparent)]
    IO(#[from] std::io::Error),

    #[cfg(feature = "parse-events")]
    #[error(transparent)]
    ParseEvent(#[from] ParseEventError),

    #[cfg(feature = "math")]
    #[error(transparent)]
    Math(#[from] MathError),
}

pub type SdkResult<T> = Result<T, SdkError>;
