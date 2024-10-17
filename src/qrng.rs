use std::fmt::Display;

use crate::error::PqkdError;


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
    pub fn check_size(&self, size: &QrngSize) -> Result<(), PqkdError> {
        match self {
            QrngFormat::Hex | QrngFormat::Base64=> {
                if size.size_to_bytes() > QrngSize::MAX_SIZE_FOR_STRING_FORMAT.size_to_bytes() {
                    return Err(PqkdError::InvalidSize { 
                        format: self.to_string(),
                        max_size: QrngSize::MAX_SIZE_FOR_STRING_FORMAT.size_to_bytes(),
                        found: size.size_to_bytes()
                    });
                }
            },
            QrngFormat::Bytes => {
                if size.size_to_bytes() > QrngSize::MAX_SIZE_FOR_BYTES_FORMAT.size_to_bytes() {
                    return Err(PqkdError::InvalidSize { 
                        format: self.to_string(),
                        max_size: QrngSize::MAX_SIZE_FOR_BYTES_FORMAT.size_to_bytes(),
                        found: size.size_to_bytes()
                    });
                }
            },
        }
        Ok(())
    }
}

pub enum QrngSize {
    Bytes(u32),
    Kilobytes(u32),
    Megabytes(u32),
}

impl Display for QrngSize {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            QrngSize::Bytes(size) => write!(f, "{size}"),
            QrngSize::Kilobytes(size) => write!(f, "{size}K"),
            QrngSize::Megabytes(size) => write!(f, "{size}M"),
        }
    }
}

impl QrngSize {
    pub const MAX_SIZE_FOR_BYTES_FORMAT: QrngSize = QrngSize::Bytes(16 * 1024 * 1024);
    pub const MAX_SIZE_FOR_STRING_FORMAT: QrngSize = QrngSize::Bytes(256 * 1024);

    pub fn size_to_bytes(&self) -> u32 {
        match *self {
            QrngSize::Bytes(val) => val,
            QrngSize::Kilobytes(val) => val * 1024,
            QrngSize::Megabytes(val) => val * 1024 * 1024,
        }
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
   