use chrono::{DateTime, Local, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::domain::types::cpf::CPF;
use crate::domain::types::email::Email;
use crate::domain::types::password::Password;

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub id: Uuid,
    pub name: String,
    pub email: Email,
    pub password: Password,
    pub profile_picture: Option<String>,
    pub cpf: CPF,
    pub created_at: DateTime<Local>,
    pub updated_at: DateTime<Local>,
}