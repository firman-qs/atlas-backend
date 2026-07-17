use std::sync::Arc;

use argon2::{
    Argon2, PasswordHash, PasswordHasher, PasswordVerifier,
    password_hash::{
        SaltString,
        rand_core::{OsRng, RngCore},
    },
};
use entity::password_reset_tokens;
use sha2::{Digest, Sha256};
use uuid::Uuid;

use crate::{
    errors::app_error::AppError, models::auth::password_reset_token::PasswordResetToken,
    repositories::password_reset_tokens_repository::PasswordResetTokensRepository,
};

/// This service is responsible for handling password-related operations, such as hashing and
/// verifying passwords.

#[derive(Debug)]
pub struct PasswordService {
    password_reset_token_repository: Arc<PasswordResetTokensRepository>,
}

impl PasswordService {
    pub fn new(password_reset_token_repository: Arc<PasswordResetTokensRepository>) -> Self {
        Self {
            password_reset_token_repository,
        }
    }

    /// Generate a random password reset token
    pub fn create_token(&self) -> String {
        let mut bytes = [0u8; 32];
        OsRng.fill_bytes(&mut bytes);
        let token = hex::encode(bytes);
        token
    }

    /// Hash the password reset token using SHA-256
    pub fn hash_token(&self, token: &str) -> String {
        let hash = Sha256::digest(token.as_bytes());
        let token_hash = hex::encode(hash);
        token_hash
    }

    /// Store password reset token in the database with hashed token
    pub async fn store_prt(&self, prt: PasswordResetToken) -> Result<(), AppError> {
        self.password_reset_token_repository.create(prt).await?;
        Ok(())
    }

    /// Retrieve password reset token from the database by token_hash
    pub async fn get_prt(
        &self,
        token_hash: &str,
    ) -> Result<Option<password_reset_tokens::Model>, AppError> {
        let prt = self
            .password_reset_token_repository
            .find_prt(token_hash)
            .await?;
        Ok(prt)
    }

    /// Invalidate id-th password reset token in the database by setting the used_at field to the
    /// current timestamp
    pub async fn invalidate_prt(&self, id: Uuid) -> Result<(), AppError> {
        self.password_reset_token_repository
            .invalidate_prt(id)
            .await?;
        Ok(())
    }

    /// Delete password reset token from the database by user_id
    pub async fn delete_prt(&self, user_id: Uuid) -> Result<(), AppError> {
        self.password_reset_token_repository
            .delete_by_user_id(user_id)
            .await?;
        Ok(())
    }

    /// Hash the password using Argon2 algorithm
    pub fn hash_password(&self, password: &str) -> anyhow::Result<String> {
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();

        let hash = argon2
            .hash_password(password.as_bytes(), &salt)?
            .to_string();

        Ok(hash)
    }

    /// Verify the password against the stored hash using Argon2 algorithm
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

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use crate::{
        config::{database::connect, settings::Settings},
        repositories::password_reset_tokens_repository::PasswordResetTokensRepository,
        services::password_service::PasswordService,
    };

    #[tokio::test]
    async fn test_password_service() {
        println!("Running test_password_service...");
        let settings = Settings::new();
        let db: sea_orm::DatabaseConnection = connect(&settings.database_url)
            .await
            .expect("Cannot connect database");
        let prt_repository = Arc::new(PasswordResetTokensRepository::new(db));
        let password_service = PasswordService::new(prt_repository);
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
}
