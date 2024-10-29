use serde::{Serialize, Deserialize};


#[derive(Debug)]
pub enum PqkdResponse {
    Status(PqkdStatus),
    Keys(Vec<Key>),
}

impl PqkdResponse {
    pub fn as_status(self) -> Option<PqkdStatus> {
        match self {
            PqkdResponse::Status(status) => Some(status),
            _ => None,
        }
    }

    pub fn keys(self) -> Vec<Key> {
        match self {
            PqkdResponse::Keys(keys) => keys,
            _ => Vec::new(),
        }
    }
}

/// Contains the status(information received from pQKD)
/// of the connection between the pQKD device and the 
/// other pQKD device.
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct PqkdStatus {
    pub max_key_count: u32,
    pub max_key_per_request: u32,
    pub max_key_size: u32,
    #[serde(rename(deserialize = "source_KME_ID"))]
    pub source_kme_id: String,
    #[serde(rename(deserialize = "master_SAE_ID"))]
    pub master_sae_id: String,
    pub stored_key_count: u32,
    pub min_key_size: u32,
    #[serde(rename(deserialize = "max_SAE_ID_count"))]
    pub max_sae_id_count: u32,
    pub key_size: u32,
}

/// Contains the key and its ID, which generates and sends pQKD.
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Key {
    #[serde(rename(deserialize = "key_ID"))]
    key_id: String,
    key: String,
}

impl Key {
    /// Returns the id of key.
    pub fn key_id(&self) -> &str {
        &self.key_id
    }

    /// Returns the key.
    pub fn key(&self) -> &str {
        &self.key
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct Keys {
    pub(crate) keys: Vec<Key>
}