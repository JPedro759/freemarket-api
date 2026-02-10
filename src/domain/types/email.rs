use std::sync::OnceLock;
use regex::Regex;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Email(String);

#[derive(Debug)]
pub enum EmailError {
    Invalid,
}

static EMAIL_REGEX: OnceLock<Regex> = OnceLock::new();

impl Email {
    pub fn to_string(&self) -> &str {
        &self.0
    }
}

impl TryFrom<String> for Email {
    type Error = EmailError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let re = EMAIL_REGEX.get_or_init(|| {
            Regex::new(r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$").unwrap()
        });

        if re.is_match(value.as_str()) {
            Ok(Self(value))
        } else {
            Err(EmailError::Invalid)
        }
    }
}