use passcheck::PasswordChecker;
use serde::{Deserialize, Serialize};

fn is_strong_password(password: &str) -> bool {
    let checker = PasswordChecker::new()
        .min_length(8, None)
        .require_upper_lower(None)
        .require_number(Some("Password must include at least one digit"))
        .require_special_char(None);

    match checker.validate(password) {
        Ok(_) => {
            println!("✅ Strong password!");
            true
        },
        Err(errors) => {
            println!("❌ Invalid password:");
            for err in errors {
                println!("  - {}", err);
            }
            false
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Password(String);

pub enum PasswordError {
    Invalid,
}

impl Password {
    pub fn to_string(&self) -> &str {
        &self.0
    }
}

impl TryFrom<String> for Password {
    type Error = PasswordError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if !is_strong_password(value.as_str()) {
            return Err(PasswordError::Invalid);
        }

        Ok(Self(value))
    }
}