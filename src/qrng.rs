use std::fmt::Display;

use crate::error::PqkdError;

pub const MAX_SIZE_FOR_BYTES_FORMAT: u32 = 16 * 1024 * 1024;
pub const MAX_SIZE_FOR_STRING_FORMAT: u32 = 256 * 1024;

#[derive(Clone)]
pub enum QrngFormat {
    Hex,
    Base64,
    Bytes,
}

impl Display for QrngFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            QrngFormat::Hex => write!(f, "hex"),//.to_string(),
            QrngFormat::Base64 => write!(f, "base64"),
            QrngFormat::Bytes => write!(f, "bytes"),
        }
    }
}

impl QrngFormat {
    pub fn check_size(&self, size: u32) -> Result<(), PqkdError> {
        match self {
            QrngFormat::Hex | QrngFormat::Base64=> {
                if size > MAX_SIZE_FOR_STRING_FORMAT {
                    return Err(PqkdError::InvalidSize { 
                        format: self.to_string(),
                        max_size: MAX_SIZE_FOR_STRING_FORMAT,
                        found: size
                    });
                }
            },
            QrngFormat::Bytes => {
                if size > MAX_SIZE_FOR_BYTES_FORMAT {
                    return Err(PqkdError::InvalidSize { 
                        format: self.to_string(),
                        max_size: MAX_SIZE_FOR_BYTES_FORMAT,
                        found: size
                    });
                }
            },
        }
        Ok(())
    }
}

pub enum QrngReturnFormat {
    Hex(String),
    Bytes(Vec<u8>),
    Base64(String),
}

impl QrngReturnFormat {
    pub fn as_hex(self) -> Option<String> {
        match self {
            QrngReturnFormat::Hex(val) => Some(val),
            _ => None,
        }
    }

    pub fn as_bytes(self) -> Option<Vec<u8>> {
        match self {
            QrngReturnFormat::Bytes(val) => Some(val),
            _ => None,
        }
    } 

    pub fn as_base64(self) -> Option<String> {
        match self {
            QrngReturnFormat::Base64(val) => Some(val),
            _ => None,
        }
    }
}
   