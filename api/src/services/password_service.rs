use argon2::{
    Argon2, PasswordHash, PasswordHasher, PasswordVerifier,
    password_hash::{SaltString, rand_core::OsRng},
};

/// This service is responsible for handling password-related operations, such as hashing and
/// verifying passwords.

#[derive(Debug)]
pub struct PasswordService;

impl PasswordService {
    pub fn new() -> Self {
        Self
    }

    pub fn hash_password(&self, password: &str) -> anyhow::Result<String> {
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();

        let hash = argon2
            .hash_password(password.as_bytes(), &salt)?
            .to_string();

        Ok(hash)
    }

    pub fn verify_password(&self, password: &str, stored_hash: &str) -> anyhow::Result<bool> {
        let parsed_hash = PasswordHash::new(stored_hash)?;
        let argon2 = Argon2::default();

        match argon2.verify_password(password.as_bytes(), &parsed_hash) {
            Ok(()) => Ok(true),
            Err(argon2::password_hash::Error::Password) => Ok(false),
            Err(e) => Err(e.into()),
        }
    }
}

#[test]
fn test_password_service() {
    println!("Running test_password_service...");
    let password_service = PasswordService::new();
    let password = "my_secure_password";
    let hash = password_service.hash_password(password).unwrap();
    let hash2 = password_service.hash_password(password).unwrap();

    assert_ne!(hash, hash2);
    assert!(password_service.verify_password(password, &hash).unwrap());
    assert!(password_service.verify_password(password, &hash2).unwrap());
    assert!(
        !password_service
            .verify_password("wrong_password", &hash)
            .unwrap()
    );
}
