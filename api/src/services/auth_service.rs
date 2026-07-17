use std::sync::Arc;

use argon2::password_hash::rand_core::{self, RngCore};
use garde::Validate;
use sha2::Digest;

use crate::{
    common::constants::MSG_INVALID_EMAIL_OR_PASSWORD,
    dto::{
        auth::{
            change_password_request::ChangePasswordRequest,
            forgot_password_request::ForgotPasswordRequest, login_request::LoginRequest,
            login_response::LoginResponse, register_request::RegisterRequest,
            reset_password_request::ResetPasswordRequest,
        },
        user::user_response::UserResponse,
    },
    errors::app_error::AppError,
    models::{
        auth::password_reset_token::PasswordResetToken,
        user::{create_user::CreateUser, update_user::UpdateUser},
    },
    repositories::user_repository::UserRepository,
    services::{jwt_service::JwtService, password_service::PasswordService},
};

#[derive(Debug)]
pub struct AuthService {
    user_repository: Arc<UserRepository>,
    password_service: Arc<PasswordService>,
    jwt_service: Arc<JwtService>,
}

impl AuthService {
    pub fn new(
        user_repository: Arc<UserRepository>,
        password_service: Arc<PasswordService>,
        jwt_service: Arc<JwtService>,
    ) -> Self {
        Self {
            user_repository,
            password_service,
            jwt_service,
        }
    }

    pub async fn register(&self, req: RegisterRequest) -> Result<UserResponse, AppError> {
        req.validate()?;

        let password_hash = self.password_service.hash_password(&req.password)?;

        let new_user = CreateUser {
            email: req.email,
            username: req.username,
            full_name: req.full_name,
            password_hash,
        };

        let user = self.user_repository.create(new_user).await?;
        Ok(user.into())
    }

    /// Handle user login by verifying credentials and generating JWT tokens
    pub async fn login(&self, req: LoginRequest) -> Result<LoginResponse, AppError> {
        req.validate()?;

        let user = self
            .user_repository
            .find_by_email(&req.email)
            .await?
            .ok_or_else(|| AppError::Unauthorized(MSG_INVALID_EMAIL_OR_PASSWORD.into()))?;

        if !self
            .password_service
            .verify_password(&req.password, &user.password_hash)?
        {
            return Err(AppError::Unauthorized(MSG_INVALID_EMAIL_OR_PASSWORD.into()));
        }

        let access_token = self.jwt_service.generate_access_token(user.id)?;
        let refresh_token = self.jwt_service.generate_refresh_token(user.id)?;

        Ok(LoginResponse {
            user: user.into(),
            access_token,
            refresh_token,
        })
    }

    /// Handle forgot password request by generating a password reset token and sending an email
    pub async fn forgot_password(&self, req: ForgotPasswordRequest) -> Result<(), AppError> {
        let token = self.password_service.create_token();
        let token_hash = self.password_service.hash_token(&token);

        let user = self
            .user_repository
            .find_by_email(&req.email)
            .await?
            .ok_or_else(|| AppError::NotFound("User not found".to_string()))?;

        self.password_service.delete_prt(user.id).await?;

        let prt = PasswordResetToken {
            token_hash,
            user_id: user.id,
            expires_at: (chrono::Utc::now() + chrono::Duration::minutes(15)).into(),
        };

        self.password_service.store_prt(prt).await?;

        // send email
        let reset_link = format!("http://127.0.0.1:3000/auth/reset-password?token={}", token);

        // TODO: Implement email sending functionality here. For now, we will just print the reset
        // link to the console.
        println!("Password reset link: {}", reset_link);

        Ok(())
    }

    /// Reset password using the provided token and new password
    pub async fn reset_password(&self, req: ResetPasswordRequest) -> Result<(), AppError> {
        let token_hash = self.password_service.hash_token(&req.token);
        let prt = self
            .password_service
            .get_prt(&token_hash)
            .await?
            .ok_or_else(|| AppError::NotFound("Invalid password reset token".to_string()))?;

        if prt.expires_at < chrono::Utc::now() {
            return Err(AppError::BadRequest(
                "Password reset token has expired".to_string(),
            ));
        }

        if prt.used_at.is_some() {
            return Err(AppError::BadRequest(
                "Password reset token has already been used".to_string(),
            ));
        }

        self.password_service.invalidate_prt(prt.id).await?;

        let password_hash = self.password_service.hash_password(&req.password)?;
        let updated_user = UpdateUser::new(prt.user_id).with_password_hash(password_hash);
        self.user_repository.update(updated_user).await?;
        Ok(())
    }

    pub async fn change_password(&self, req: ChangePasswordRequest) -> Result<(), AppError> {
        todo!()
    }

    pub async fn logout(&self) -> Result<(), AppError> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use crate::config::database::connect;
    use crate::config::settings::Settings;
    use crate::repositories::password_reset_tokens_repository::PasswordResetTokensRepository;
    use crate::repositories::user_repository::UserRepository;
    use crate::services::auth_service::AuthService;
    use crate::services::jwt_service::JwtService;
    use crate::services::password_service::PasswordService;

    // Helper function to create an instance of AuthService for testing
    async fn create_auth_service() -> AuthService {
        let settings = Settings::new();
        let db: sea_orm::DatabaseConnection = connect(&settings.database_url)
            .await
            .expect("Cannot connect database");
        let user_repository = Arc::new(UserRepository::new(db.clone()));
        let prt_repository = Arc::new(PasswordResetTokensRepository::new(db.clone()));
        let password_service = Arc::new(PasswordService::new(prt_repository));
        let jwt_service = Arc::new(JwtService::new(
            settings.jwt_secret.clone(),
            settings.access_token_exp_minutes,
            settings.refresh_token_exp_days,
        ));
        AuthService::new(user_repository, password_service, jwt_service)
    }

    #[tokio::test]
    async fn test_register_with_empty_fields() {
        let auth_service = create_auth_service().await;

        let register_request = crate::dto::auth::register_request::RegisterRequest {
            email: "".to_string(),
            username: "".to_string(),
            full_name: "".to_string(),
            password: "".to_string(),
        };

        let result = auth_service.register(register_request).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_register_with_existing_email() {
        let auth_service = create_auth_service().await;

        let register_request = crate::dto::auth::register_request::RegisterRequest {
            email: "existing_email@gmail.com".to_string(),
            username: "12345678".to_string(),
            full_name: "12345678".to_string(),
            password: "12345678".to_string(),
        };

        let result = auth_service.register(register_request).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_register_with_existing_username() {
        let auth_service = create_auth_service().await;

        let register_request = crate::dto::auth::register_request::RegisterRequest {
            email: "non_existing_email@gmail.com".to_string(),
            username: "existing".to_string(),
            full_name: "12345678".to_string(),
            password: "12345678".to_string(),
        };

        let result = auth_service.register(register_request).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_register_success() {
        let auth_service = create_auth_service().await;

        let register_request = crate::dto::auth::register_request::RegisterRequest {
            email: "non_existing_email@gmail.com".to_string(),
            username: "nonexisting".to_string(),
            full_name: "Non Existing".to_string(),
            password: "12345678".to_string(),
        };

        let result = auth_service.register(register_request).await;

        // make sure to delete the user after the test to avoid conflicts in future tests i am not
        // implementin unique email since it will make the database populate with test data
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_login_success() {
        let auth_service = create_auth_service().await;

        let login_request = crate::dto::auth::login_request::LoginRequest {
            email: "non_existing_email@gmail.com".to_string(),
            password: "12345678".to_string(),
        };

        let result = auth_service.login(login_request).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_login_failed_password() {
        let auth_service = create_auth_service().await;

        let login_request = crate::dto::auth::login_request::LoginRequest {
            // its already registered
            email: "non_existing_email@gmail.com".to_string(),
            password: "123456789".to_string(),
        };

        let result = auth_service.login(login_request).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_login_failed_email() {
        let auth_service = create_auth_service().await;
        let login_request = crate::dto::auth::login_request::LoginRequest {
            email: "non_existing_email@wrong.com".to_string(),
            password: "12345678".to_string(),
        };

        let result = auth_service.login(login_request).await;
        assert!(result.is_err());
    }
}
