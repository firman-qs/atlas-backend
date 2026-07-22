use argon2::Argon2;
use argon2::PasswordHash;
use argon2::password_hash::SaltString;
use argon2::password_hash::rand_core::OsRng;

pub struct PasswordManager;

impl PasswordManager {
    pub fn new() -> Self {
        Self {}
    }

    /// Hash the password using Argon2 algorithm
    pub fn hash_password(&self, password: &str) -> anyhow::Result<String> {
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();

        let hash =
            argon2::PasswordHasher::hash_password(&argon2, password.as_bytes(), &salt)?.to_string();

        Ok(hash)
    }

    /// Verify the password against the stored hash using Argon2 algorithm
    pub fn verify_password(&self, password: &str, stored_hash: &str) -> anyhow::Result<bool> {
        let parsed_hash = PasswordHash::new(stored_hash)?;
        let argon2 = Argon2::default();

        match argon2::PasswordVerifier::verify_password(&argon2, password.as_bytes(), &parsed_hash)
        {
            Ok(()) => Ok(true),
            Err(argon2::password_hash::Error::Password) => Ok(false),
            Err(e) => Err(e.into()),
        }
    }
}
