use serde::{Deserialize, Serialize};
use regex::Regex;
use std::sync::OnceLock;
use thiserror::Error;

static EMAIL_REGEX: OnceLock<Regex> = OnceLock::new();

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(try_from = "String", into = "String")]
pub struct Email(String);

#[derive(Error, Debug)]
pub enum EmailError {
    #[error("Formato de e-mail inválido!")]
    InvalidFormat,
}

impl Email {
    fn validate(email: &str) -> bool {
        let re = EMAIL_REGEX.get_or_init(|| {
            Regex::new(r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$").unwrap()
        });
        re.is_match(email)
    }
}

// Converter String -> Email com validação
impl TryFrom<String> for Email {
    type Error = EmailError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if !Self::validate(&value) {
            return Err(EmailError::InvalidFormat)

        }

        Ok(Self(value))
    }
}

// Permite que o Serde converta Email -> String
impl From<Email> for String {
    fn from(email: Email) -> Self {
        email.0
    }
}