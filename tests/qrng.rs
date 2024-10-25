use pqkd::qrng::{QrngFormat, QrngReturnFormat, MAX_SIZE_FOR_BYTES_FORMAT, MAX_SIZE_FOR_STRING_FORMAT};
use pqkd::BuilderPqkdClient;
use serde_json::json;
use httpmock::MockServer;

#[tokio::test]
async fn test_get_random_hex() {
    let qrng_server = MockServer::start_async().await;
    let addr_qrng_server = format!("http://{}", qrng_server.address());
    let random_hex = "5ede863536f9c2cb29e2ca26d5aef0e2dffe44c7";
    let size = 20usize;

    qrng_server.mock_async(|when, then| {
        when.method("GET")
            .path("/qrng/hex")
            .query_param("size", size.to_string());
        then.status(200)
            .header("content-type", "application/json")
            .json_body(json!({"result": random_hex, "size": size.to_string(), "format": "hex", "executeTime": 335}));
    }).await;

    let pqkd_client = BuilderPqkdClient::with_addr("http://127.0.0.1")
        .unwrap()
        .with_qrng_addr(&addr_qrng_server)
        .unwrap()
        .build();

    let result = pqkd_client.get_random_hex(size as u32)
        .await.unwrap();
    
    assert_eq!(result, random_hex);
}

#[tokio::test]
async fn test_get_random_hex_for_the_large_size() {
    let size = MAX_SIZE_FOR_STRING_FORMAT + 1;

    let pqkd_client = BuilderPqkdClient::with_addr("http://127.0.0.1:8082")
        .unwrap()
        .build();

    let result = pqkd_client.get_random_hex(size as u32)
        .await;
    
    assert!(result.is_err());
}

#[tokio::test]
async fn test_get_random_base64() {
    let qrng_server = MockServer::start_async().await;
    let addr_qrng_server = format!("http://{}", qrng_server.address());
    let random_base64 = "w7jYDPNv789HHJTJg7iwkg4AYI0=";
    let size = 20usize;

    qrng_server.mock_async(|when, then| {
        when.method("GET")
            .path("/qrng/base64")
            .query_param("size", size.to_string());
        then.status(200)
            .header("content-type", "application/json")
            .json_body(json!({"result": random_base64, "size": size.to_string(), "format": "base64", "executeTime": 335}));
    }).await;

    let pqkd_client = BuilderPqkdClient::with_addr("http://127.0.0.1")
        .unwrap()
        .with_qrng_addr(&addr_qrng_server)
        .unwrap()
        .build();

    let result = pqkd_client.get_random_base64(size as u32)
        .await.unwrap();
    
    assert_eq!(result, random_base64);
}

#[tokio::test]
async fn test_get_random_base64_for_the_large_size() {
    let size = MAX_SIZE_FOR_STRING_FORMAT + 1;

    let pqkd_client = BuilderPqkdClient::with_addr("http://127.0.0.1:8082")
        .unwrap()
        .build();

    let result = pqkd_client.get_random_base64(size as u32)
        .await;
    
    assert!(result.is_err());
}

#[tokio::test]
async fn test_get_random_bytes() {
    let qrng_server = MockServer::start_async().await;
    let addr_qrng_server = format!("http://{}", qrng_server.address());
    let binary = b"\x28\x214\x252\x207\x37\x218\x43\x144\x62\x183\x81\x251\x22\x163\x201\x90\x172\x215\x80\x51";
    let size = 20usize;

    qrng_server.mock_async(|when, then| {
        when.method("GET")
            .path("/qrng/bytes")
            .query_param("size", size.to_string());
        then.status(200)
            .body(binary);
    }).await;

    let pqkd_client = BuilderPqkdClient::with_addr("http://127.0.0.1")
        .unwrap()
        .with_qrng_addr(&addr_qrng_server)
        .unwrap()
        .build();

    let random_bytes = pqkd_client.get_random_bytes(size as u32)
        .await.unwrap();
    
    assert_eq!(random_bytes, binary.to_vec());
}

#[tokio::test]
async fn test_get_random_bytes_for_the_large_size() {
    let size = MAX_SIZE_FOR_BYTES_FORMAT + 1;

    let pqkd_client = BuilderPqkdClient::with_addr("http://127.0.0.1:8082")
        .unwrap()
        .build();

    let result = pqkd_client.get_random_bytes(size as u32)
        .await;
    
    assert!(result.is_err());
}

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