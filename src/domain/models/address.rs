use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Address {
    pub zip_code: String,
    pub street: String,
    pub neighborhood: String,
    pub number: String,
    pub city: String,
    pub state: String,
    pub country: String,
    pub complement: Option<String>,
    pub is_default: bool,
}