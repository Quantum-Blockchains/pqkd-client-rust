pub enum PqkdMethod {
    Status,
    EncKeys,
    DesKeys,
}

pub struct PqkdRequest {
    pub(crate) pqkd_method: PqkdMethod,
    pub(crate) sae_id: String,
    pub(crate) size: u16,
    pub(crate) number: u32,
    pub(crate) key_ids: Vec<String>,
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