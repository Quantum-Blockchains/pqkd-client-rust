use crate::{error::PqkdError, pqkd::{Key, PqkdClient, PqkdStatus}};

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

pub enum PqkdMethod {
    Status,
    EncKeys,
    DesKeys,
}

pub struct PqkdRequest {
    pqkd_method: PqkdMethod,
    sae_id: String,
    size: u16,
    number: u32,
    key_ids: Vec<String>,
}

impl PqkdRequest {
    pub fn size(&self) -> u16 {
        self.size
    }
    
    pub fn size_mut(&mut self) -> &mut u16 {
        &mut self.size
    }
    
    pub fn set_size(&mut self, size: u16) {
        self.size = size;
    }
    
    pub fn set_number(&mut self, number: u32) {
        self.number = number;
    }
    
    pub fn key_ids_mut(&mut self) -> &mut Vec<String> {
        &mut self.key_ids
    }
    
    pub fn pqkd_method(&self) -> &PqkdMethod {
        &self.pqkd_method
    }
    
    pub fn sae_id(&self) -> &str {
        &self.sae_id
    }
    
    pub fn key_ids(&self) -> &[String] {
        &self.key_ids
    }
    
    pub fn number(&self) -> u32 {
        self.number
    }
}

pub struct PqkdRequestBuilder {
    pqkd_client: PqkdClient,
    pqkd_request: Result<PqkdRequest, PqkdError>,
}

impl PqkdRequest {
    /// Constructs a new pqkd request
    pub fn new(pqkd_method: PqkdMethod, sae_id: &str) -> Self {
        PqkdRequest {
            pqkd_method,
            sae_id: String::from(sae_id),
            size: 512,
            number: 1u32,
            key_ids: Vec::new(),
        }
    }
}

impl PqkdRequestBuilder {
    pub fn new (pqkd_client: PqkdClient, pqkd_request: PqkdRequest) -> Self {
        PqkdRequestBuilder {
            pqkd_client,
            pqkd_request: Ok(pqkd_request),
        }
    }

    /// Add size of key to this Pqkd Request
    pub fn size(mut self, size: u16) -> PqkdRequestBuilder {
        let mut error = None;
        if let Ok(ref mut pqkd_request) = self.pqkd_request {
            if size < 64 || size%8 != 0 || size > 4096 { 
                error = Some(PqkdError::SizeOfKeysError);
            }
            else {
                pqkd_request.set_size(size);
            }
        }
        if let Some(err) = error {
            self.pqkd_request = Err(err);
        }
        self
    }

    /// Add size of key to this Pqkd Request
    pub fn number(mut self, number: u32) -> PqkdRequestBuilder {
        let mut error = None;
        if let Ok(ref mut pqkd_request) = self.pqkd_request {
            if number == 0 { 
                error = Some(PqkdError::NumberOfKeysError);
            }
            else {
                pqkd_request.set_number(number);
            }
        }
        if let Some(err) = error {
            self.pqkd_request = Err(err);
        }
        self
    }

    /// Add key_id to this Pqkd Request
    pub fn key_id(mut self, key_id: &str) -> PqkdRequestBuilder {
        if let Ok(ref mut pqkd_request) = self.pqkd_request {
            pqkd_request.key_ids_mut().push(String::from(key_id));
        }
        self
    }

    /// Add key_id to this Pqkd Request
    pub fn key_ids(mut self, key_ids: Vec<&str>) -> PqkdRequestBuilder {
        if let Ok(ref mut pqkd_request) = self.pqkd_request {
            for key_id in key_ids {
                pqkd_request.key_ids_mut().push(String::from(key_id));
            }
        }
        self
    }

    pub fn build(self) -> Result<PqkdRequest, PqkdError> {
        self.pqkd_request
    }

    pub async fn send(self) -> Result<PqkdResponse, PqkdError> {
        match self.pqkd_request {
            Ok(request) => self.pqkd_client.kme_execute_request(request).await,
            Err(err) => Err(err),
        }
    }

}