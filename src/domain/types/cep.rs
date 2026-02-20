use brasilapi::cep;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CEP(String);

impl CEP {
    pub async fn is_exists(&self) -> bool {
        cep::get_cep(&self.0).await.is_ok()
    }
}