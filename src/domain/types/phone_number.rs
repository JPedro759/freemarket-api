use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(try_from = "String", into = "String")]
pub struct PhoneNumber(String);

#[derive(Error, Debug)]
pub enum PhoneError {
    #[error("Formato de telefone inválido!")]
    InvalidFormat,
}

impl PhoneNumber {
    fn validate(number: &str) -> bool {
        match phonenumber::parse(None, &number) {
            Ok(parsed) => phonenumber::is_valid(&parsed),
            Err(_) => false,
        }
    }
}

impl TryFrom<String> for PhoneNumber {
    type Error = PhoneError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if !Self::validate(&value) {
            return Err(PhoneError::InvalidFormat)
        }

        Ok(PhoneNumber(value))
    }
}

impl From<PhoneNumber> for String {
    fn from(p: PhoneNumber) -> String {
        p.0
    }
}


