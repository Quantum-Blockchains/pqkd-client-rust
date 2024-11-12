use httpmock::MockServer;
use pqkd::{blocking::BuilderPqkdClient, PqkdStatus};
use serde_json::json;

#[test]
fn test_status() {
    let kme_server = MockServer::start();
    let addr_kme_server: String = format!("http://{}", kme_server.address());
    let response = json!(
        {
            "max_key_count": 4096,
            "max_key_per_request": 64,
            "max_key_size": 4096,
            "source_KME_ID": "Test_2KME",
            "master_SAE_ID": "Test_2SAE",
            "stored_key_count": 0,
            "min_key_size": 64,
            "max_SAE_ID_count": 0,
            "key_size": 256
        }
    );

    kme_server.mock(|when, then| {
        when.method("GET").path("/api/v1/keys/Test_2SAE/status");
        then.status(200)
            .header("content-type", "application/json")
            .json_body(response.clone());
    });

    let pqkd_client = BuilderPqkdClient::with_addr(&addr_kme_server)
        .unwrap()
        .build();

    let result = pqkd_client
        .status("Test_2SAE")
        .send()
        .unwrap()
        .as_status()
        .unwrap();

    assert_eq!(
        result,
        serde_json::from_str::<PqkdStatus>(&response.to_string()).unwrap()
    );
}

#[test]
fn test_enc_key() {
    let kme_server = MockServer::start();
    let addr_kme_server: String = format!("http://{}", kme_server.address());
    let response_key_id = "17d3e519-10e9-43e6-bd7a-72b2da710dcd";
    let response_key =
        "lRXjNYtHITV4KXkdIJZN/Pv0ojAkuLGwzwumMev959w=GR6XALTLg+B5I6jP/OlVDLQR3+j8PtpevhajPYY0hkM=";
    let response_key_size = 512u16;

    kme_server.mock(|when, then| {
        when.method("POST")
            .path("/api/v1/keys/Test_2SAE/enc_keys")
            .json_body(json!({"size": response_key_size, "number": 1}));
        then.status(200)
            .header("content-type", "application/json")
            .json_body(json!({"keys": [{"key_ID": response_key_id, "key": response_key}]}));
    });

    let pqkd_client = BuilderPqkdClient::with_addr(&addr_kme_server)
        .unwrap()
        .build();

    let result = pqkd_client
        .enc_keys("Test_2SAE")
        .size(response_key_size)
        .send()
        .unwrap()
        .keys();

    assert_eq!(result[0].key(), response_key);
    assert_eq!(result[0].key_id(), response_key_id);
}

#[test]
fn test_enc_keys() {
    let kme_server = MockServer::start();
    let addr_kme_server: String = format!("http://{}", kme_server.address());

    let keys = vec![
        ("17d3e519-10e9-43e6-bd7a-72b2da710dcd", "lRXjNYtHITV4KXkdIJZN/Pv0ojAkuLGwzwumMev959w=GR6XALTLg+B5I6jP/OlVDLQR3+j8PtpevhajPYY0hkM="),
        ("8195ac8a-22b2-47ba-a54f-9c9eb75cd723", "UfjRtIkZWFmxlTX3dGQ3GdlnyQMkHSiWf7A29Wj4XsFrbq6DqGnu0nlzlBdijighv5Gwn2C7VUXpLgxaIj4v9g=="),
        ("8650d18d-5858-4830-b2f6-7641905ed936", "xgnwHNTlBoNpvtWa5JlvfVieibB5Yl6cT0fP6wzNZcvEzVjwueg07W7eY7BCd+VFoDqmZy17whqIjwKPhpy4XQ=="),
        ("ea81b590-ac56-4778-a979-9d523afdecb1", "7KCGzY4HKwLI7wcHRdTgdP4F+yZJsvAeLDDEz4IOc92XuPOE3eE6A79rqWjkFiosoKfaHSIsh2KtVz3r4f/XbA==")
    ];
    let response_key_size = 512u16;
    let number = keys.len() as u32;
    let keys_json: Vec<serde_json::Value> = keys
        .iter()
        .map(|(key_id, key)| json!({"key_ID": *key_id, "key": *key}))
        .collect();

    kme_server.mock(|when, then| {
        when.method("POST")
            .path("/api/v1/keys/Test_2SAE/enc_keys")
            .json_body(json!({"size": response_key_size, "number": number}));
        then.status(200)
            .header("content-type", "application/json")
            .json_body(json!({"keys": keys_json}));
    });

    let pqkd_client = BuilderPqkdClient::with_addr(&addr_kme_server)
        .unwrap()
        .build();

    let result = pqkd_client
        .enc_keys("Test_2SAE")
        .number(number)
        .size(response_key_size)
        .send()
        .unwrap()
        .keys();

    let result: Vec<(&str, &str)> = result.iter().map(|key| (key.key_id(), key.key())).collect();

    assert_eq!(result, keys);
}

#[test]
fn test_enc_keys_with_key_ids() {
    let kme_server = MockServer::start();
    let addr_kme_server: String = format!("http://{}", kme_server.address());

    let keys = vec![
        ("17d3e519-10e9-43e6-bd7a-72b2da710dcd", "lRXjNYtHITV4KXkdIJZN/Pv0ojAkuLGwzwumMev959w=GR6XALTLg+B5I6jP/OlVDLQR3+j8PtpevhajPYY0hkM="),
        ("8195ac8a-22b2-47ba-a54f-9c9eb75cd723", "UfjRtIkZWFmxlTX3dGQ3GdlnyQMkHSiWf7A29Wj4XsFrbq6DqGnu0nlzlBdijighv5Gwn2C7VUXpLgxaIj4v9g=="),
        ("8650d18d-5858-4830-b2f6-7641905ed936", "xgnwHNTlBoNpvtWa5JlvfVieibB5Yl6cT0fP6wzNZcvEzVjwueg07W7eY7BCd+VFoDqmZy17whqIjwKPhpy4XQ=="),
        ("ea81b590-ac56-4778-a979-9d523afdecb1", "7KCGzY4HKwLI7wcHRdTgdP4F+yZJsvAeLDDEz4IOc92XuPOE3eE6A79rqWjkFiosoKfaHSIsh2KtVz3r4f/XbA==")
    ];
    let response_key_size = 512u16;
    let keys_json: Vec<serde_json::Value> = keys
        .iter()
        .map(|(key_id, key)| json!({"key_ID": *key_id, "key": *key}))
        .collect();
    let keys_ids: Vec<&str> = keys.iter().map(|(key_id, _)| *key_id).collect();

    kme_server.mock(|when, then| {
        when.method("POST")
            .path("/api/v1/keys/Test_2SAE/enc_keys")
            .json_body(json!({"size": response_key_size, "key_IDs": keys_ids}));
        then.status(200)
            .header("content-type", "application/json")
            .json_body(json!({"keys": keys_json}));
    });

    let pqkd_client = BuilderPqkdClient::with_addr(&addr_kme_server)
        .unwrap()
        .build();

    let result = pqkd_client
        .enc_keys("Test_2SAE")
        .size(response_key_size)
        .key_ids(keys_ids)
        .send()
        .unwrap()
        .keys();

    let result: Vec<(&str, &str)> = result.iter().map(|key| (key.key_id(), key.key())).collect();

    assert_eq!(result, keys);
}

#[test]
fn test_dec_key() {
    let kme_server = MockServer::start();
    let addr_kme_server: String = format!("http://{}", kme_server.address());
    let response_key_id = "17d3e519-10e9-43e6-bd7a-72b2da710dcd";
    let response_key =
        "lRXjNYtHITV4KXkdIJZN/Pv0ojAkuLGwzwumMev959w=GR6XALTLg+B5I6jP/OlVDLQR3+j8PtpevhajPYY0hkM=";

    kme_server.mock(|when, then| {
        when.method("POST")
            .path("/api/v1/keys/Test_1SAE/dec_keys")
            .json_body(json!(
                {
                    "key_IDs": [
                        {"key_ID": response_key_id }
                    ]
                }
            ));
        then.status(200)
            .header("content-type", "application/json")
            .json_body(json!({"keys": [{"key_ID": response_key_id, "key": response_key}]}));
    });

    let pqkd_client = BuilderPqkdClient::with_addr(&addr_kme_server)
        .unwrap()
        .build();

    let result = pqkd_client
        .dec_keys("Test_1SAE")
        .key_id(&response_key_id)
        .send()
        .unwrap()
        .keys();

    assert_eq!(result[0].key(), response_key);
    assert_eq!(result[0].key_id(), response_key_id);
}

#[test]
fn test_dec_keys() {
    let kme_server = MockServer::start();
    let addr_kme_server: String = format!("http://{}", kme_server.address());
    let keys = vec![
        ("17d3e519-10e9-43e6-bd7a-72b2da710dcd", "lRXjNYtHITV4KXkdIJZN/Pv0ojAkuLGwzwumMev959w=GR6XALTLg+B5I6jP/OlVDLQR3+j8PtpevhajPYY0hkM="),
        ("8195ac8a-22b2-47ba-a54f-9c9eb75cd723", "UfjRtIkZWFmxlTX3dGQ3GdlnyQMkHSiWf7A29Wj4XsFrbq6DqGnu0nlzlBdijighv5Gwn2C7VUXpLgxaIj4v9g=="),
        ("8650d18d-5858-4830-b2f6-7641905ed936", "xgnwHNTlBoNpvtWa5JlvfVieibB5Yl6cT0fP6wzNZcvEzVjwueg07W7eY7BCd+VFoDqmZy17whqIjwKPhpy4XQ=="),
        ("ea81b590-ac56-4778-a979-9d523afdecb1", "7KCGzY4HKwLI7wcHRdTgdP4F+yZJsvAeLDDEz4IOc92XuPOE3eE6A79rqWjkFiosoKfaHSIsh2KtVz3r4f/XbA=="),
    ];
    let keys_json: Vec<serde_json::Value> = keys
        .iter()
        .map(|(key_id, key)| json!({"key_ID": *key_id, "key": *key}))
        .collect();
    let key_ids: Vec<&str> = keys.iter().map(|(key_id, _)| *key_id).collect();
    let key_ids_json: Vec<serde_json::Value> = keys
        .iter()
        .map(|(key_id, _)| json!({"key_ID": *key_id}))
        .collect();

    kme_server.mock(|when, then| {
        when.method("POST")
            .path("/api/v1/keys/Test_1SAE/dec_keys")
            .header("content-type", "application/json")
            .json_body(json!({
               "key_IDs": key_ids_json
            }));
        then.status(200)
            .header("content-type", "application/json")
            .json_body(json!({"keys": keys_json}));
    });

    let pqkd_client = BuilderPqkdClient::with_addr(&addr_kme_server)
        .unwrap()
        .build();

    let result = pqkd_client
        .dec_keys("Test_1SAE")
        .key_ids(key_ids)
        .send()
        .unwrap()
        .keys();
    let result: Vec<(&str, &str)> = result.iter().map(|key| (key.key_id(), key.key())).collect();

    assert_eq!(result, keys);
}

