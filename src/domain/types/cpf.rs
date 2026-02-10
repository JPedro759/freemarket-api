use brazilian_utils::cpf::validate;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct CPF(String);

pub enum ErrorCPF {
    Invalid,
}

impl CPF {
    pub fn to_string(&self) -> &str {
        &self.0
    }
}

impl TryFrom<String> for CPF {
    type Error = ErrorCPF;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if validate(value.as_str()) {
            Ok(Self(value))
        } else {
            Err(ErrorCPF::Invalid)
        }
    }
}