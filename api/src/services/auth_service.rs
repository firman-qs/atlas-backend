use std::sync::Arc;

use garde::Validate;

use crate::{
    common::constants::MSG_INVALID_EMAIL_OR_PASSWORD,
    dto::{
        auth::{
            change_password_request::ChangePasswordRequest, login_request::LoginRequest,
            login_response::LoginResponse, register_request::RegisterRequest,
        },
        user::user_response::UserResponse,
    },
    errors::app_error::AppError,
    models::user::create_user::CreateUser,
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

    pub async fn change_password(&self, req: ChangePasswordRequest) -> Result<(), AppError> {
        todo!()
    }

    pub async fn logout(&self) -> Result<(), AppError> {
        todo!()
    }

    pub async fn reset_password(&self) -> Result<(), AppError> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use crate::config::database::connect;
    use crate::config::settings::Settings;
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
        let user_repository = Arc::new(UserRepository::new(db));
        let password_service = Arc::new(PasswordService::new());
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
