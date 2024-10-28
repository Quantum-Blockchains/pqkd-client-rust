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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn qrng_format_to_string() {
        assert_eq!(QrngFormat::Hex.to_string(), "hex".to_string());
        assert_eq!(QrngFormat::Base64.to_string(), "base64".to_string());
        assert_eq!(QrngFormat::Bytes.to_string(), "bytes".to_string());
    }

    #[test]
    fn check_size() {
        assert!(QrngFormat::Bytes.check_size(1000).is_ok());
        assert!(QrngFormat::Bytes.check_size(MAX_SIZE_FOR_BYTES_FORMAT).is_ok()); 
        assert!(QrngFormat::Bytes.check_size(MAX_SIZE_FOR_BYTES_FORMAT + 1).is_err()); 

        assert!(QrngFormat::Hex.check_size(1000).is_ok());
        assert!(QrngFormat::Hex.check_size(MAX_SIZE_FOR_STRING_FORMAT).is_ok()); 
        assert!(QrngFormat::Hex.check_size(MAX_SIZE_FOR_STRING_FORMAT + 1).is_err()); 

        assert!(QrngFormat::Base64.check_size(1000).is_ok());
        assert!(QrngFormat::Base64.check_size(MAX_SIZE_FOR_STRING_FORMAT).is_ok()); 
        assert!(QrngFormat::Base64.check_size(MAX_SIZE_FOR_STRING_FORMAT + 1).is_err()); 
    } 

    #[test]
    fn qrng_return_format_unwrap() {
        assert!(QrngReturnFormat::Hex("123".to_string()).as_hex().is_some());
        assert!(QrngReturnFormat::Base64("123".to_string()).as_hex().is_none());
        assert!(QrngReturnFormat::Bytes(vec![1,2,3]).as_hex().is_none());

        assert!(QrngReturnFormat::Hex("123".to_string()).as_bytes().is_none());
        assert!(QrngReturnFormat::Base64("123".to_string()).as_bytes().is_none());
        assert!(QrngReturnFormat::Bytes(vec![1,2,3]).as_bytes().is_some());

        assert!(QrngReturnFormat::Hex("123".to_string()).as_base64().is_none());
        assert!(QrngReturnFormat::Base64("123".to_string()).as_base64().is_some());
        assert!(QrngReturnFormat::Bytes(vec![1,2,3]).as_base64().is_none());
    }
}
 