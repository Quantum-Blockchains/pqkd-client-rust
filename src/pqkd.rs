use url::Url;
use serde_json::Value;
use crate::qrng::{QrngFormat, QrngSize, QrngReturnFormat};
use crate::error::PqkdError;
use reqwest::Client;
use serde::{Deserialize, Serialize};


pub struct Pqkd {
    url: Url,
    port_kme: u16,
    port_qrng: u16,
    client: Client,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug)]
pub struct PqkdStatus {
    max_key_count: u32,
    max_key_per_request: u32,
    max_key_size: u32,
    source_KME_ID: String,
    master_SAE_ID: String,
    stored_key_count: u32,
    min_key_size: u32,
    max_SAE_ID_count: u32,
    key_size: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Keys {
    pub keys: Vec<Key>
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug)]
pub struct Key {
    pub key_ID: String,
    pub key: String,
}

impl Pqkd {

    pub fn new(url: Url, port_kme: u16, port_qrng: u16, client: Client) -> Self {
        Self {
            url,   
            port_kme,
            port_qrng,
            client,
        }
    }

    pub fn url(&self) -> &Url {
        &self.url
    }

    pub fn port_kme(&self) -> u16 {
        self.port_kme
    }

    pub fn port_qrng(&self) -> u16 {
        self.port_qrng
    }

    pub async fn get_random_hex(&self, size: QrngSize) -> Result<String, PqkdError> {
        Ok(self._fetch_random(QrngFormat::Hex, size).await?.as_hex().unwrap())
    }

    pub async fn get_random_bytes(&self, size: QrngSize) -> Result<Vec<u8>, PqkdError> {
        Ok(self._fetch_random(QrngFormat::Bytes, size).await?.as_bytes().unwrap())
    }

    pub async fn get_random_base64(&self, size: QrngSize) -> Result<String, PqkdError> {
        Ok(self._fetch_random(QrngFormat::Base64, size).await?.as_base64().unwrap())
    }
    
    pub async fn status(&self, sae_id: &str) -> Result<PqkdStatus, PqkdError> {
        self._fetch_status(sae_id).await
    }

    pub async fn enc_keys(&self, sae_id: &str, number: u32, size: u32) -> Result<Keys, PqkdError> {
        self._fetch_enc_keys(sae_id, number, size).await
    } 

    pub async fn dec_keys(&self, sae_id: &str, key_id: &str) -> Result<Keys, PqkdError> {
        self._fetch_dec_keys(sae_id, key_id).await
    }

    async fn _fetch_random(&self, format: QrngFormat, size: QrngSize) -> Result<QrngReturnFormat, PqkdError> {
        format.check_size(&size)?;
        let url = format!(
            "{}://{}:{}/qrng/{}?size={}",
            self.url.scheme(),
            self.url.host().unwrap().to_string(),
            self.port_qrng,
            format.clone().to_string(),
            size.to_string(),
        );

        let res = self.client.get(url).send()
            .await?
            .error_for_status()?;
        let res = match format {
            QrngFormat::Base64 => {
                let body = res.text().await?;
                let v: Value = serde_json::from_str(&body)?;
                QrngReturnFormat::Base64(v["result"].as_str().unwrap().to_string())
            },
            QrngFormat::Bytes => {
                let body = res.bytes().await?;
                QrngReturnFormat::Bytes(body.to_vec())
            },
            QrngFormat::Hex => {
                let body = res.text().await?;
                let v: Value = serde_json::from_str(&body)?;
                QrngReturnFormat::Hex(v["result"].as_str().unwrap().to_string())
            },
        };
        Ok(res)
    }

    async fn _fetch_status(&self, sae_id: &str) -> Result<PqkdStatus, PqkdError> {
        let url = format!(
            "{}://{}:{}/api/v1/keys/{}/status",
            self.url.scheme(),
            self.url.host().unwrap().to_string(),
            self.port_kme,
            sae_id,
        );
        let res = self.client.get(url).send()
            .await?
            .error_for_status()?;

        let body = res.text().await?;
        let status: PqkdStatus = serde_json::from_str(&body).unwrap();
        Ok(status)
        
    }
    
    async fn _fetch_enc_keys(&self, sae_id: &str, number: u32, size: u32) -> Result<Keys, PqkdError> {
        
        if number == 0 {
            return Err(PqkdError::NumberOfKeysError);
        }
        if size < 64 || size%8 != 0 {
            return Err(PqkdError::SizeOfKeysError);
        }
        let url = format!(
            "{}://{}:{}/api/v1/keys/{}/enc_keys?number={}&size={}",
            self.url.scheme(),
            self.url.host().unwrap().to_string(),
            self.port_kme,
            sae_id,
            number,
            size,
        );
        
        let res = self.client.get(url)
            .send()
            .await?
            .error_for_status()
            .unwrap();
        
        let body = res.text().await?;
        let keys: Keys = serde_json::from_str(&body)?;
        Ok(keys)
    }

    async fn _fetch_dec_keys(&self, sae_id: &str, key_id: &str) -> Result<Keys, PqkdError> {
        let url = format!(
            "{}://{}:{}/api/v1/keys/{}/dec_keys?key_ID={}",
            self.url.scheme(),
            self.url.host().unwrap().to_string(),
            self.port_kme,
            sae_id,
            key_id,
        ); 

        let res = self.client.get(url)
            .send()
            .await?
            .error_for_status()
            .unwrap();

        let body = res.text().await.unwrap();
        let keys: Keys = serde_json::from_str(&body).unwrap();
        Ok(keys)
    }

    pub fn _add_target() {
        // TODO
    }

    pub fn _get_local_target() {
        // TODO
    }
}