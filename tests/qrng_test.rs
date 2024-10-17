use pqkd::qrng::{QrngFormat, QrngReturnFormat, QrngSize};

#[test]
fn format_to_string() {
    assert_eq!(QrngFormat::Hex.to_string(), "hex".to_string());
    assert_eq!(QrngFormat::Base64.to_string(), "base64".to_string());
    assert_eq!(QrngFormat::Bytes.to_string(), "bytes".to_string());
}

#[test]
fn size_to_string() {
    assert_eq!(QrngSize::Bytes(123).to_string(), "123".to_string());
    assert_eq!(QrngSize::Kilobytes(123).to_string(), "123K".to_string());
    assert_eq!(QrngSize::Megabytes(123).to_string(), "123M".to_string());
}

#[test]
fn size_to_bytes() {
    assert_eq!(QrngSize::Bytes(32).size_to_bytes(), 32);
    assert_eq!(QrngSize::Kilobytes(32).size_to_bytes(), 32768);
    assert_eq!(QrngSize::Megabytes(32).size_to_bytes(), 33554432);
}

#[test]
fn check_size() {
    assert!(QrngFormat::Bytes.check_size(&QrngSize::Bytes(1000)).is_ok());
    assert!(QrngFormat::Bytes.check_size(&QrngSize::MAX_SIZE_FOR_BYTES_FORMAT).is_ok()); 
    assert!(QrngFormat::Bytes.check_size(&QrngSize::Bytes(QrngSize::MAX_SIZE_FOR_BYTES_FORMAT.size_to_bytes() + 1)).is_err()); 
    assert!(QrngFormat::Bytes.check_size(&QrngSize::Kilobytes(1)).is_ok());
    assert!(QrngFormat::Bytes.check_size(&QrngSize::Kilobytes(16000)).is_ok());
    assert!(QrngFormat::Bytes.check_size(&QrngSize::Kilobytes(17000)).is_err());
    assert!(QrngFormat::Bytes.check_size(&QrngSize::Megabytes(1)).is_ok());
    assert!(QrngFormat::Bytes.check_size(&QrngSize::Megabytes(16)).is_ok());
    assert!(QrngFormat::Bytes.check_size(&QrngSize::Megabytes(17)).is_err());

    assert!(QrngFormat::Hex.check_size(&QrngSize::Bytes(1000)).is_ok());
    assert!(QrngFormat::Hex.check_size(&QrngSize::MAX_SIZE_FOR_STRING_FORMAT).is_ok()); 
    assert!(QrngFormat::Hex.check_size(&QrngSize::Bytes(QrngSize::MAX_SIZE_FOR_STRING_FORMAT.size_to_bytes() + 1)).is_err()); 
    assert!(QrngFormat::Hex.check_size(&QrngSize::Kilobytes(1)).is_ok());
    assert!(QrngFormat::Hex.check_size(&QrngSize::Kilobytes(256)).is_ok());
    assert!(QrngFormat::Hex.check_size(&QrngSize::Kilobytes(257)).is_err());
    assert!(QrngFormat::Hex.check_size(&QrngSize::Megabytes(1)).is_err());
    assert!(QrngFormat::Hex.check_size(&QrngSize::Megabytes(16)).is_err());
    assert!(QrngFormat::Hex.check_size(&QrngSize::Megabytes(17)).is_err());

    assert!(QrngFormat::Base64.check_size(&QrngSize::Bytes(1000)).is_ok());
    assert!(QrngFormat::Base64.check_size(&QrngSize::MAX_SIZE_FOR_STRING_FORMAT).is_ok()); 
    assert!(QrngFormat::Base64.check_size(&QrngSize::Bytes(QrngSize::MAX_SIZE_FOR_STRING_FORMAT.size_to_bytes() + 1)).is_err()); 
    assert!(QrngFormat::Base64.check_size(&QrngSize::Kilobytes(1)).is_ok());
    assert!(QrngFormat::Base64.check_size(&QrngSize::Kilobytes(256)).is_ok());
    assert!(QrngFormat::Base64.check_size(&QrngSize::Kilobytes(257)).is_err());
    assert!(QrngFormat::Base64.check_size(&QrngSize::Megabytes(1)).is_err());
    assert!(QrngFormat::Base64.check_size(&QrngSize::Megabytes(16)).is_err());
    assert!(QrngFormat::Base64.check_size(&QrngSize::Megabytes(17)).is_err());
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