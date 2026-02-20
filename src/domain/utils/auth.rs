use std::sync::OnceLock;
use passcheck::PasswordChecker;
use argon2::{
    password_hash::{
        rand_core::OsRng,
        PasswordHash, PasswordHasher, PasswordVerifier, SaltString
    },
    Argon2
};

static PASSWORD_PERMISSIONS: OnceLock<PasswordChecker> = OnceLock::new();
pub struct PasswordManager;

impl PasswordManager {
    fn validate_password(password: &str) -> bool {
        let checker = PASSWORD_PERMISSIONS.get_or_init(|| {
            PasswordChecker::new()
                .min_length(8, None)
                .require_upper_lower(None)
                .require_number(Some("Password must include at least one digit"))
                .require_special_char(None)
        });

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

    pub fn hash_password(password: &str) -> Result<String, password_hash::Error> {
        if !Self::validate_password(password) {
            return Err(password_hash::Error::Password)
        }

        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();
        let password_hash = argon2.hash_password(password.as_bytes(), &salt)?;

        Ok(password_hash.to_string())
    }

    pub fn verify_password(password: &str, hashed_password: &str) -> bool {
        let parsed_hash = match PasswordHash::new(hashed_password) {
            Ok(parsed_hash) => parsed_hash,
            Err(_) => return false,
        };

        Argon2::default().verify_password(password.as_bytes(), &parsed_hash).is_ok()
    }
}