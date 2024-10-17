use std::fs;

use url::Url;
use reqwest::{self, Client};

use crate::{error, pqkd::Pqkd};

pub struct BuilderPqkd {
    url: Option<Url>,
    port_kme: Option<u16>,
    port_qrng: Option<u16>,
    client: Client,
} 

impl BuilderPqkd {
    pub fn with_url(url: Url) -> Self {
        Self {
            url: Some(url),
            port_kme: None,
            port_qrng: None,
            client: reqwest::Client::new(),
        }
    }

    pub fn with_port_kme(self, port: u16) -> Self {
        Self {
            url: self.url,
            port_kme: Some(port),
            port_qrng: self.port_qrng,
            client: self.client,
        }
    }

    pub fn with_port_qrng(self, port: u16) -> Self {
        Self {
            url: self.url,
            port_kme: self.port_kme,
            port_qrng: Some(port),
            client: self.client,
        }
    }

    pub fn with_tls(self, server_cert: &str, client_cert: &str, client_key: &str) -> Result<Self, error::PqkdError> {
        let cert = fs::read(client_cert)?;
        let key = fs::read(client_key)?;
        let ca_cert = fs::read(server_cert)?;
        let id = reqwest::Identity::from_pkcs8_pem(&cert, &key)?;
        let ca_cert = reqwest::Certificate::from_pem(&ca_cert)?;
        Ok(Self { 
            url: self.url,
            port_kme: self.port_kme,
            port_qrng: self.port_qrng,
            client: reqwest::Client::builder().use_native_tls().identity(id).add_root_certificate(ca_cert).build()?,
        })
    }
    
    pub fn build(self) -> Pqkd {
        Pqkd::new(
            self.url.unwrap(),
            self.port_kme.unwrap_or(8082),
            self.port_qrng.unwrap_or(8085),
            self.client,
        )
    }
}