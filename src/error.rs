use thiserror::Error;

#[derive(Error, Debug)]
pub enum PqkdError {
    #[error("Failed request")]
    RequestError(#[from] reqwest::Error),
    #[error("Error json")]
    SerdeJsonError(#[from] serde_json::Error),
    #[error("invalid size for {format:?} (max size {max_size:?}, found {found:?})")]
    InvalidSize {
        format: String,
        max_size: u32,
        found: u32,
    },
    #[error("Failed read file")]
    IoError(#[from] std::io::Error),
    #[error("min number of keys = 1")]
    NumberOfKeysError,
    #[error("min key size = 64, and number must be divisible by 8")]
    SizeOfKeysError,
}