use serde::{Deserialize, Serialize};
use brazilian_utils::cpf;
use thiserror::Error;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(try_from = "String", into = "String")]
pub struct CPF(String);

#[derive(Error, Debug)]
pub enum CPFError {
    #[error("Formato de cpf inválido!")]
    InvalidFormat,
}

impl CPF {
    pub fn validate(cpf: &str) -> bool {
        cpf::is_valid(&cpf)
    }
}

impl TryFrom<String> for CPF {
    type Error = CPFError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if !Self::validate(&value) {
            return Err(CPFError::InvalidFormat)
        }

        Ok(Self(value))
    }
}

impl From<CPF> for String {
    fn from(cpf: CPF) -> Self {
        cpf.0
    }
}
