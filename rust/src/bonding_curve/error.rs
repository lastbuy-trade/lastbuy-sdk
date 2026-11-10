#[cfg(feature = "parse-events")]
#[derive(Debug, thiserror::Error)]
pub enum ParseEventError {
    #[error("Invalid event instruction data")]
    InvalidData,
    
    #[error("Invalid event instruction discriminator")]
    InvalidInstructionDiscriminator,

    #[error("Invalid event discriminator")]
    InvalidDiscriminator
}

#[derive(Debug, thiserror::Error)]
pub enum SdkError {
    #[error(transparent)]
    IO(#[from] std::io::Error),

    #[cfg(feature = "parse-events")]
    #[error(transparent)]
    ParseEvent(#[from] ParseEventError),
}

pub type SdkResult<T> = Result<T, SdkError>;