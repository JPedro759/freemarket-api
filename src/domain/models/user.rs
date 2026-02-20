use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::domain::types::cep::CEP;
use crate::domain::types::cpf::CPF;
use crate::domain::types::email::Email;

#[derive(Debug, Deserialize, Serialize)]
pub struct User {
    pub id: Uuid,
    pub name: String,
    pub email: Email,
    pub password: String,
    pub cpf: CPF,
    pub zip_code: CEP,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}