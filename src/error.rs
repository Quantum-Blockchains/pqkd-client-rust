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
    #[error("key size min = 64, max = 4096, and number must be divisible by 8")]
    SizeOfKeysError,
    #[error("Pqkd builder error: {0}")]
    BuildPqkdError(String),
    #[error("Failed request to QRNG server.")]
    ErrorQrngRequest,
    #[error("Failed request to KME server.")]
    ErrorKmeRequest,
}