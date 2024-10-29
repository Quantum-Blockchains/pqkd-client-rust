use crate::{error::PqkdError, request::PqkdRequest, PqkdResponse};

use super::pqkd::PqkdClient;

pub struct PqkdRequestBuilder {
    pqkd_client: PqkdClient,
    pqkd_request: Result<PqkdRequest, PqkdError>,
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

    pub fn send(self) -> Result<PqkdResponse, PqkdError> {
        match self.pqkd_request {
            Ok(request) => self.pqkd_client.kme_execute_request(request),
            Err(err) => Err(err),
        }
    }

}