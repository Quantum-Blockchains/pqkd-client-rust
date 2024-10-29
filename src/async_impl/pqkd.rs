use url::Url;
use serde_json::{json, Value};
use crate::qrng::{QrngFormat, QrngReturnFormat};
use crate::error::PqkdError;
use crate::request::{PqkdMethod, PqkdRequest};
use crate::response::PqkdResponse;
use reqwest::Client;
use crate::{PqkdStatus, Keys, Key};
use super::request_builder::PqkdRequestBuilder;

/// Contains the necessary data for
/// communication with the pQKD device.
///
/// # Example
/// 
/// ```
/// use pqkd::BuilderPqkdClient;
/// use std::error::Error;
///
/// #[tokio::main] 
/// async fn main() -> Result<(), Box<dyn Error>> {
///     
///     //let ca_cert = std::fs::read("ca.cert")?;
///     //let client_cert = std::fs::read("client.cert")?;
///     //let client_key = std::fs::read("client.key")?;
///
/// 
///     let pqkd_client_1 = BuilderPqkdClient::with_addr("http://172.16.0.154:8082")?
///         .with_qrng_addr("http://172.16.0.154:8085")?
///      //   .with_tls(&ca_cert, &client_cert, &client_key)?
///         .build();
/// 
///     let pqkd_client_2 = BuilderPqkdClient::with_addr("http://172.16.0.155:8082")?
///         .with_qrng_addr("http://172.16.0.155:8085")?
///         //.with_tls(&ca_cert, &client_cert, &client_key)?
///         .build();
///     
///     let status1 = pqkd_client_1.status("Test_2SAE")
///         .send()
///         .await?;
///     let status2 = pqkd_client_2.status("Test_1SAE")
///         .send()
///         .await?;
/// 
///     let key1 = pqkd_client_1.enc_keys("Test_2SAE")
///         .size(1024)
///         .send()
///         .await?
///         .keys();
///     let key2 = pqkd_client_2.dec_keys("Test_1SAE")
///         .key_id(key1[0].key_id())
///         .send()
///         .await?
///         .keys();
///     
///     assert_eq!(key1[0].key(), key2[0].key());
/// 
///     let keys1 = pqkd_client_1.enc_keys("Test_2SAE")
///         .number(10)
///         .size(512)
///         .send()
///         .await?
///         .keys();
///     let keys2 = pqkd_client_2.dec_keys("Test_1SAE")
///         .key_ids(keys1.iter().map(|key| key.key_id()).collect::<Vec<&str>>())
///         .send()
///         .await?
///         .keys();
///     
///     assert_eq!(keys1.len(), keys2.len());
///     let mut iter1 = keys1.iter();
///     keys2.iter().map(|key| assert_eq!(key, iter1.next().unwrap()));
///     
/// 
///     let random_hex = pqkd_client_1.get_random_hex(64).await.unwrap();
///     let random_bytes = pqkd_client_1.get_random_bytes(512).await.unwrap();
///     let random_base64 = pqkd_client_1.get_random_base64(256).await.unwrap();
///      
///     Ok(())
/// }
/// 
#[derive(Clone)]
pub struct PqkdClient {
    kme_addr: Url,
    qrng_addr: Url,
    client: Client,
    local_target: Vec<u8>,
}

/// Build [PqkdClient](pqkd::PqkdClient) by combining an address KME server,
/// address QRNG server and request::Client with certificates.
///
/// ```
/// use pqkd::BuilderPqkdClient;
/// use std::error::Error;
/// 
/// fn build_pqkd() -> Result<(), Box<dyn Error>> {
///     
///     let ca_cert = std::fs::read("./ca.cert")?;
///     let client_cert = std::fs::read("./client.cert")?;
///     let client_key = std::fs::read("./client.keu")?;
///    
///     let pqkd = BuilderPqkdClient::with_addr("https://172.0.0.1:8082")?
///         .with_qrng_addr("http://127.0.0.1:8085")?
///         .with_tls(&ca_cert, &client_cert, &client_key)?
///         .build();
///     
///     Ok(())
/// }
/// 
/// ```
pub struct BuilderPqkdClient {
    kme_addr: Url,
    qrng_addr: Url,
    client: Client,
    local_target: Vec<u8>,
} 

impl BuilderPqkdClient { 
    /// Add a KME server address of PQKD.
    /// Returns an error if parsing of the address failed.
    /// # Examples
    ///
    /// ```
    /// use pqkd::BuilderPqkdClient;
    /// use std::error::Error;
    /// 
    /// fn build_pqkd() -> Result<(), Box<dyn Error>> {
    ///     
    ///     let pqkd_builder = BuilderPqkdClient::with_addr("http://172.0.0.1:8082")?;
    ///     
    ///     Ok(())
    /// }
    /// 
    /// ```
    pub fn with_addr(addr: &str) -> Result<Self, PqkdError> {
        // TODO addr must be "mailto:rms@example.net"!!!!! 
        let kme_addr = Url::parse(addr).map_err(|_| PqkdError::BuildPqkdError("parsing failed.".to_string()))?; 
        let mut qrng_addr = kme_addr.clone();
        let _ = qrng_addr.set_port(Some(8085)).map_err(|_| PqkdError::BuildPqkdError("".to_string()));
        Ok( Self {
            kme_addr,
            qrng_addr,
            client: reqwest::ClientBuilder::new()
                .http1_title_case_headers()
                .build()
                .unwrap(),
            local_target: Vec::new(),
        })
    }

    /// Add a QRNG server address of PQKD.
    /// If this method is not called, the default address will be used,
    /// i.e. the same as the KME server address of the pQKD only with port 8085.
    /// Returns an error if parsing of the address failed.
    /// # Examples
    /// 
    /// ```
    /// use pqkd::BuilderPqkdClient;
    /// use std::error::Error;
    /// 
    /// fn build_pqk() -> Result<(), Box<dyn Error>> {
    ///     
    ///     let pqkd_builder = BuilderPqkdClient::with_addr("https://172.0.0.1:8082")?
    ///         .with_qrng_addr("http://127.0.0.1:8085")?;
    ///     
    ///     Ok(())
    /// }
    /// ``` 
    pub fn with_qrng_addr(self, addr: &str) -> Result<Self, PqkdError> {
        let qrng_addr: Url = Url::parse(addr).map_err(|_| PqkdError::BuildPqkdError("parsing failed.".to_string()))?;
        Ok(Self {
            kme_addr: self.kme_addr,
            qrng_addr,
            client: self.client,
            local_target: self.local_target,
        })
    }
    
    /// Add a CA certificate, client certificate and client key for TLS. 
    /// This method accepts as parameters vectors of bytes certificates and key.
    /// File must be in pem format.   
    /// # Examples
    /// 
    /// ```
    /// use pqkd::BuilderPqkdClient;
    /// use std::error::Error;
    /// 
    /// fn build_pqk() -> Result<(), Box<dyn Error>> {
    ///     
    ///     let ca_cert = std::fs::read("ca.cert")?;
    ///     let client_cert = std::fs::read("client.cert")?;
    ///     let client_key = std::fs::read("client.key")?;
    ///     
    ///     let pqkd_builder = BuilderPqkdClient::with_addr("https://172.0.0.1:8082")?
    ///         .with_tls(&ca_cert, & client_cert, & client_key)?;
    ///     
    ///     Ok(())
    /// }
    /// ``` 
    pub fn with_tls(self, ca_cert: &Vec<u8>, client_cert: &Vec<u8>, client_key: &Vec<u8>) -> Result<Self, PqkdError> {
        let id = reqwest::Identity::from_pkcs8_pem(client_cert, client_key)?;
        let ca_cert = reqwest::Certificate::from_pem(ca_cert)?;
        Ok(Self { 
            kme_addr: self.kme_addr,
            qrng_addr: self.qrng_addr,
            client: reqwest::Client::builder().use_native_tls().identity(id).add_root_certificate(ca_cert).build()?,
            local_target: self.local_target,
        })
    }
    
    pub fn with_local_target(self, local_target: Vec<u8>) -> Self {
        Self { 
            kme_addr: self.kme_addr,
            qrng_addr: self.qrng_addr,
            client: self.client,
            local_target: local_target,
        }
    }

    /// Creates PqkdClient by passing it the data it contains and return it.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use pqkd::BuilderPqkdClient;
    /// use std::error::Error;
    /// 
    /// fn build_pqk() -> Result<(), Box<dyn Error>> {
    ///     
    ///     let ca_cert = std::fs::read("ca.cert")?;
    ///     let client_cert = std::fs::read("client.cert")?;
    ///     let client_key = std::fs::read("client.key")?; 
    ///     
    ///     let pqkd = BuilderPqkdClient::with_addr("https://172.0.0.1:8082")?
    ///         .with_qrng_addr("http://127.0.0.1:8085")?
    ///         .with_tls(&ca_cert, &client_cert, & client_key)?
    ///         .build();
    ///     
    ///     Ok(())
    /// }
    /// ``` 
    pub fn build(self) -> PqkdClient {
        PqkdClient::new(
            self.kme_addr,
            self.qrng_addr,
            self.client,
            self.local_target,
        )
    }
}

impl PqkdClient {
    /// Create a new ['PqkdClient'] from the given
    /// url of kme server, url of qrng server.
    pub fn new(kme_addr: Url, qrng_addr: Url, client: Client, local_target: Vec<u8>) -> Self {
        Self {
            kme_addr,   
            qrng_addr,
            client,
            local_target,
        }
    }

    // pub async fn status(&self, sae_id: &str) -> Result<PqkdStatus, PqkdError> {
    //     self._fetch_status(sae_id).await
    // }

    pub fn status(&self, sae_id: &str) -> PqkdRequestBuilder {
        PqkdRequestBuilder::new(
            self.clone(),
            PqkdRequest::new(PqkdMethod::Status, sae_id)
        )
    }

    // pub async fn enc_key(&self, sae_id: &str, size: u32) -> Result<Key, PqkdError> {
    //     let mut vec = self._fetch_enc_keys(sae_id, 1, size, None).await?;
    //     let key = vec.pop().unwrap();
    //     Ok(key)
    // }

    pub fn enc_keys(&self, sae_id: &str) -> PqkdRequestBuilder {
        PqkdRequestBuilder::new(
            self.clone(),
            PqkdRequest::new(PqkdMethod::EncKeys, sae_id)
        )
    }

    // pub async fn enc_key_with_key_id(&self, sae_id: &str, size: u32, key_id: &str) -> Result<Key, PqkdError> {
    //     let mut vec = self._fetch_enc_keys(sae_id, 1, size, Some(vec![key_id])).await?;
    //     let key = vec.pop().unwrap();
    //     Ok(key)
    // }

    // pub async fn enc_keys(&self, sae_id: &str, number: u32, size: u32) -> Result<Vec<Key>, PqkdError> {
    //     self._fetch_enc_keys(sae_id, number, size, None).await
    // } 

    // pub async fn enc_keys_with_key_ids(&self, sae_id: &str, size: u32, key_ids: Vec<&str>) -> Result<Vec<Key>, PqkdError> {
    //     self._fetch_enc_keys(sae_id, 1u32, size, Some(key_ids)).await
    // }

    // pub async fn dec_key(&self, sae_id: &str, key_id: &str) -> Result<Key, PqkdError> {
    //     let mut vec = self._fetch_dec_keys(sae_id, vec![key_id]).await?;
    //     let key = vec.pop().unwrap();
    //     Ok(key)
    // }

    pub fn dec_keys(&self, sae_id: &str) -> PqkdRequestBuilder {
        PqkdRequestBuilder::new(
            self.clone(),
            PqkdRequest::new(PqkdMethod::DesKeys, sae_id)
        )
    }

    // pub async fn dec_keys(&self, sae_id: &str, key_ids: Vec<&str>) -> Result<Vec<Key>, PqkdError> {
    //     self._fetch_dec_keys(sae_id, key_ids).await
    // }

    pub async fn get_random_hex(&self, size: u32) -> Result<String, PqkdError> {
        Ok(self._fetch_random(QrngFormat::Hex, size).await?.as_hex().unwrap())
    }

    pub async fn get_random_bytes(&self, size: u32) -> Result<Vec<u8>, PqkdError> {
        Ok(self._fetch_random(QrngFormat::Bytes, size).await?.as_bytes().unwrap())
    }

    pub async fn get_random_base64(&self, size: u32) -> Result<String, PqkdError> {
        Ok(self._fetch_random(QrngFormat::Base64, size).await?.as_base64().unwrap())
    }
    
    pub async fn get_local_target(&self) -> Vec<u8> {
        self.local_target.clone()
    }

    pub async fn get_sae_ids(&self) -> Result<Vec<String>, PqkdError> {
        todo!();
    }

    pub async fn add_target(&self) -> Result<(), PqkdError> {
        todo!();
    }

    pub async fn remove_target(&self) -> Result<(), PqkdError> {
        todo!();
    }
}

impl PqkdClient {
    pub async fn kme_execute_request(&self, pqkd_request: PqkdRequest) -> Result<PqkdResponse, PqkdError>{
        match pqkd_request.pqkd_method() {
            PqkdMethod::Status => {
                let url = self.kme_addr.join(
                    &format!("api/v1/keys/{}/status", pqkd_request.sae_id())
                ).map_err(|_| PqkdError::ErrorKmeRequest).unwrap();
                let res = self.client.get(url).send()
                    .await?
                    .error_for_status()?;

                let body = res.text().await?;
                let status: PqkdStatus = serde_json::from_str(&body).unwrap();
                Ok(PqkdResponse::Status(status))
            },
            PqkdMethod::EncKeys => {
                let url = self.kme_addr.join(
                &format!("/api/v1/keys/{}/enc_keys", pqkd_request.sae_id())
                    ).map_err(|_| PqkdError::ErrorKmeRequest).unwrap();
                let body = if pqkd_request.key_ids().len() > 0 {
                    let ids: Vec<&str> = pqkd_request.key_ids().iter().map(|id| id.as_str()).collect();
                    json!({"size": pqkd_request.size(), "key_IDs": ids})
                } else {
                    json!({"size": pqkd_request.size(), "number": pqkd_request.number()})
                };
                
                let res = self.client.post(url)
                    .body(body.to_string());
                let res = res.send()
                    .await?
                    .error_for_status()
                    .unwrap();
            
                let body = res.text().await?;
                let keys: Keys = serde_json::from_str(&body)?;
                Ok(PqkdResponse::Keys(keys.keys))
            },
            PqkdMethod::DesKeys => {
                let url = self.kme_addr.join(
                    &format!("/api/v1/keys/{}/dec_keys", pqkd_request.sae_id())
                    ).map_err(|_| PqkdError::ErrorKmeRequest).unwrap();
                let key_ids: Vec<serde_json::Value> = pqkd_request.key_ids().iter().map(|key_id| json!({"key_ID": key_id})).collect();
                let body = json!({"key_IDs": key_ids});
        
                let res = self.client.post(url)
                    .header("Content-Type", "application/json")
                    .body(body.to_string())
                    .send()
                    .await?
                    .error_for_status()
                    .unwrap();
                
                let body = res.text().await.unwrap();
                let keys: Keys = serde_json::from_str(&body).unwrap();
                Ok(PqkdResponse::Keys(keys.keys))
            }
        }
    }

    async fn _fetch_random(&self, format: QrngFormat, size: u32) -> Result<QrngReturnFormat, PqkdError> {
        format.check_size(size)?;

        let url = self.qrng_addr.join(
            &format!("qrng/{}?size={}", &format.to_string(), size)
        ).map_err(|_| PqkdError::ErrorQrngRequest).unwrap();

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
        let url = self.kme_addr.join(
            &format!("api/v1/keys/{}/status", sae_id)
        ).map_err(|_| PqkdError::ErrorKmeRequest).unwrap();

        let res = self.client.get(url).send()
            .await?
            .error_for_status()?;

        let body = res.text().await?;
        let status: PqkdStatus = serde_json::from_str(&body).unwrap();
        Ok(status)
        } 
    
    async fn _fetch_enc_keys(&self, sae_id: &str, number: u32, size: u32, key_ids: Option<Vec<&str>>) -> Result<Vec<Key>, PqkdError> { 
        if number == 0 { 
            return Err(PqkdError::NumberOfKeysError); 
        } 
        if size < 64 || size%8 != 0 || size > 4096
        { 
            return Err(PqkdError::SizeOfKeysError); 
        } 
        let url = self.kme_addr.join(
        &format!("/api/v1/keys/{}/enc_keys", sae_id)
            ).map_err(|_| PqkdError::ErrorKmeRequest).unwrap();
        let body = if let Some(ids) = key_ids {
            let ids: Vec<&str> = ids.iter().map(|id| *id).collect();
            json!({"size": size, "key_IDs": ids})
        } else {
            json!({"size": size, "number": number})
        };

        let res = self.client.post(url)
            .body(body.to_string());
        let res = res.send()
            .await?
            .error_for_status()
            .unwrap();

        let body = res.text().await?;
        let keys: Keys = serde_json::from_str(&body)?;
        Ok(keys.keys)
    }

    async fn _fetch_dec_keys(&self, sae_id: &str, key_ids: Vec<&str>) -> Result<Vec<Key>, PqkdError> {
        let url = self.kme_addr.join(
            &format!("/api/v1/keys/{}/dec_keys", sae_id)
            ).map_err(|_| PqkdError::ErrorKmeRequest).unwrap();
        let key_ids: Vec<serde_json::Value> = key_ids.iter().map(|key_id| json!({"key_ID": *key_id})).collect();
        let body = json!({"key_IDs": key_ids});
        // let body = KeyIds{
        //     key_IDs: key_ids.iter().map(|key_id| KeyId { key_ID: key_id.to_string()}).collect(),
        // };
        

        let res = self.client.post(url)
            .header("Content-Type", "application/json")
            .body(body.to_string())
            .send()
            .await?
            .error_for_status()
            .unwrap();
        
        let body = res.text().await.unwrap();
        let keys: Keys = serde_json::from_str(&body).unwrap();
        Ok(keys.keys)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_builder() {
        let _pqkd_client = BuilderPqkdClient::with_addr("http://127.0.0.1:8082")
            .unwrap()
            .with_qrng_addr("http://127.0.0.1:8085")
            .unwrap()
            .build();
    }
}