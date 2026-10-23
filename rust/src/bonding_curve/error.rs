#[derive(Debug, thiserror::Error)]
pub enum SdkError {
    #[error(transparent)]
    IO(#[from] std::io::Error),
}

pub type SdkResult<T> = Result<T, SdkError>;