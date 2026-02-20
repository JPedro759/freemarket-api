
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::domain::models::address::Address;
use crate::domain::types::cpf::CPF;
use crate::domain::types::email::Email;
use crate::domain::types::phone_number::PhoneNumber;

#[derive(Debug, Deserialize, Serialize)]
pub struct User {
    pub id: Uuid,
    pub name: String,
    pub profile_image_url: Option<String>,
    pub email: Email,
    pub password: String,
    pub cpf: CPF,
    pub phone_number: PhoneNumber,
    pub address: Address,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}