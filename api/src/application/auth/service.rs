use std::sync::Arc;

use crate::application::app_error::AppError;
use crate::application::auth::commands::change_password::ChangePassword;
use crate::application::auth::commands::forgot_password::ForgotPassword;
use crate::application::auth::commands::impersonate::Impersonate;
use crate::application::auth::commands::login::Login;
use crate::application::auth::commands::register::Register;
use crate::application::auth::commands::reset_password::ResetPassword;
use crate::application::auth::results::login_result::LoginResult;
use crate::application::users::commands::update_user::UpdateUser;
use crate::application::users::results::user_result::UserResult;
use crate::domain::entities::password_reset_token::PasswordResetTokenNew;
use crate::domain::entities::user::UserNew;
use crate::infrastructure::database::repositories::pg_password_reset_tokens_repository::PgPasswordResetTokensRepository;
use crate::infrastructure::database::repositories::pg_users_repository::PgUserRepository;
use crate::infrastructure::jwt::jwt_manager::JwtManager;
use crate::infrastructure::security::password_manager::PasswordManager;
use crate::infrastructure::security::token_manager::TokenManager;
use crate::shared::constants::MSG_INVALID_EMAIL_OR_PASSWORD;

pub struct AuthService {
    user_repository: Arc<PgUserRepository>,
    password_reset_token_repository: Arc<PgPasswordResetTokensRepository>,
    jwt_manager: Arc<JwtManager>,
    password_manager: Arc<PasswordManager>,
    token_manager: Arc<TokenManager>,
}

impl AuthService {
    pub fn new(
        user_repository: Arc<PgUserRepository>,
        password_reset_token_repository: Arc<PgPasswordResetTokensRepository>,
        jwt_manager: Arc<JwtManager>,
        password_manager: Arc<PasswordManager>,
        token_manager: Arc<TokenManager>,
    ) -> Self {
        Self {
            user_repository,
            password_reset_token_repository,
            jwt_manager,
            password_manager,
            token_manager,
        }
    }

    /// Handle user registration by validating input, hashing the password, and
    /// creating a new user
    pub async fn register(&self, cmd: Register) -> Result<UserResult, AppError> {
        let password_hash: String = self.password_manager.hash_password(&cmd.password)?;

        let new_user = UserNew {
            email: cmd.email,
            username: cmd.username,
            full_name: cmd.full_name,
            password_hash,
            avatar_url: None,
        };

        let user = self.user_repository.create(new_user).await?;

        Ok(user.into())
    }

    /// Handle user impersonation by generating JWT tokens for the impersonated
    /// user
    pub async fn impersonate(&self, cmd: Impersonate) -> Result<LoginResult, AppError> {
        let as_user = self
            .user_repository
            .find_by_id(cmd.as_user_id)
            .await?
            .ok_or_else(|| AppError::NotFound("User to be impersonated not found".to_string()))?;

        let access_token: String = self
            .jwt_manager
            .generate_access_token(cmd.user_id, Some(as_user.id))?;

        let refresh_token: String = self
            .jwt_manager
            .generate_refresh_token(cmd.user_id, Some(as_user.id))?;

        let user = UserResult::from(as_user);

        Ok(LoginResult {
            access_token,
            refresh_token,
            user,
        })
    }

    #[tracing::instrument(skip(self, cmd), fields(email = %cmd.email))]
    /// Handle user login by verifying credentials and generating JWT tokens
    pub async fn login(&self, cmd: Login) -> Result<LoginResult, AppError> {
        let user = self
            .user_repository
            .find_by_email(&cmd.email)
            .await?
            .ok_or_else(|| {
                tracing::warn!("Login failed: email not found");
                AppError::Unauthorized(MSG_INVALID_EMAIL_OR_PASSWORD.into())
            })?;

        if !self
            .password_manager
            .verify_password(&cmd.password, &user.password_hash)?
        {
            tracing::warn!(
                user_id = %user.id,
                "Login failed: invalid password"
            );

            return Err(AppError::Unauthorized(MSG_INVALID_EMAIL_OR_PASSWORD.into()));
        }

        let access_token = self.jwt_manager.generate_access_token(user.id, None)?;
        let refresh_token = self.jwt_manager.generate_refresh_token(user.id, None)?;

        let user = UserResult::from(user);

        Ok(LoginResult {
            access_token,
            refresh_token,
            user,
        })
    }

    /// Handle forgot password request by generating a password reset token and
    /// sending an email
    pub async fn forgot_password(&self, forgot_password: ForgotPassword) -> Result<(), AppError> {
        let token = self.token_manager.generate_token();
        let token_hash = self.token_manager.hash_token(&token);

        let user = self
            .user_repository
            .find_by_email(&forgot_password.email)
            .await?
            .ok_or_else(|| AppError::NotFound("User not found".to_string()))?;

        self.password_reset_token_repository
            .delete_by_user_id(user.id)
            .await?;

        let prt_new = PasswordResetTokenNew {
            token_hash,
            user_id: user.id,
            expires_at: (chrono::Utc::now() + chrono::Duration::minutes(15)).into(),
        };

        self.password_reset_token_repository.create(prt_new).await?;

        // send email
        let reset_link = format!("http://127.0.0.1:3000/auth/reset-password?token={}", token);

        // TODO: Implement email sending functionality here. For now, we will just print
        // the reset link to the console.
        println!("Password reset link: {}", reset_link);

        Ok(())
    }

    /// Reset password using the provided token and new password
    pub async fn reset_password(&self, reset_password: ResetPassword) -> Result<(), AppError> {
        let token_hash = self.token_manager.hash_token(&reset_password.token);
        let prt = self
            .password_reset_token_repository
            .find_prt(&token_hash)
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

        self.password_reset_token_repository
            .invalidate_prt(prt.id)
            .await?;

        let password_hash = self
            .password_manager
            .hash_password(&reset_password.password)?;

        let update_user = UpdateUser::new(prt.user_id).with_password_hash(password_hash);

        self.user_repository.update(update_user.into()).await?;

        Ok(())
    }

    pub async fn change_password(&self, cmd: ChangePassword) -> Result<(), AppError> {
        todo!()
    }

    pub async fn logout(&self) -> Result<(), AppError> {
        todo!()
    }
}

// #[cfg(test)]
// mod tests
// {
//     use std::sync::Arc;
//
//     use crate::application::auth::service::AuthService;
//     use crate::infrastructure::config::settings::Settings;
//     use crate::infrastructure::database::connection::connect;
//     use crate::infrastructure::database::repositories::pg_password_reset_tokens_repository::PgPasswordResetTokensRepository;
//     use crate::infrastructure::database::repositories::pg_users_repository::PgUserRepository;
//     use crate::infrastructure::jwt::jwt_manager::JwtManager;
//     use crate::infrastructure::security::password_manager::PasswordManager;
//
//     // Helper function to create an instance of AuthService for testing
//     async fn create_auth_service() -> AuthService
//     {
//         let settings = Settings::new();
//         let db: sea_orm::DatabaseConnection = connect(&settings.database_url)
//             .await
//             .expect("Cannot connect database");
//         let user_repository = Arc::new(PgUserRepository::new(db.clone()));
//         let prt_repository =
// Arc::new(PgPasswordResetTokensRepository::new(db.clone()));         let
// password_service = Arc::new(PasswordMaanger::new(prt_repository));
//         let jwt_service = Arc::new(JwtManager::new(
//             settings.jwt_secret.clone(),
//             settings.access_token_exp_minutes,
//             settings.refresh_token_exp_days,
//         ));
//         let password_crypto = Arc::new(PasswordManager);
//         let token_crypto = Arc::new(PasswordManager);
//
//         AuthService {
//             user_repository,
//             password_service,
//             password_crypto,
//             token_crypto,
//             jwt_service,
//         }
//     }
//
//     #[tokio::test]
//     async fn test_register_with_empty_fields()
//     {
//         let auth_service = create_auth_service().await;
//
//         let register: Register = RegisterRequest {
//             email: "".to_string(),
//             username: "".to_string(),
//             full_name: "".to_string(),
//             password: "".to_string(),
//         }
//         .into();
//
//         let result = auth_service.register(register).await;
//         assert!(result.is_err());
//     }
//
//     #[tokio::test]
//     async fn test_register_with_existing_email()
//     {
//         let auth_service = create_auth_service().await;
//
//         let register: Register = RegisterRequest {
//             email: "existing_email@gmail.com".to_string(),
//             username: "12345678".to_string(),
//             full_name: "12345678".to_string(),
//             password: "12345678".to_string(),
//         }
//         .into();
//
//         let result = auth_service.register(register).await;
//         assert!(result.is_err());
//     }
//
//     #[tokio::test]
//     async fn test_register_with_existing_username()
//     {
//         let auth_service = create_auth_service().await;
//
//         let register_request: Register = RegisterRequest {
//             email: "non_existing_email@gmail.com".to_string(),
//             username: "existing".to_string(),
//             full_name: "12345678".to_string(),
//             password: "12345678".to_string(),
//         }
//         .into();
//
//         let result = auth_service.register(register_request).await;
//         assert!(result.is_err());
//     }
//
//     #[tokio::test]
//     async fn test_register_success()
//     {
//         let auth_service = create_auth_service().await;
//
//         let register: Register = RegisterRequest {
//             email: "non_existing_email@gmail.com".to_string(),
//             username: "nonexisting".to_string(),
//             full_name: "Non Existing".to_string(),
//             password: "12345678".to_string(),
//         }
//         .into();
//
//         let result = auth_service.register(register).await;
//
//         // make sure to delete the user after the test to avoid conflicts in
// future         // tests i am not implementin unique email since it will make
// the         // database populate with test data
//         assert!(result.is_ok());
//     }
//
//     #[tokio::test]
//     async fn test_login_success()
//     {
//         let auth_service = create_auth_service().await;
//
//         let login: Login = LoginRequest {
//             email: "non_existing_email@gmail.com".to_string(),
//             password: "12345678".to_string(),
//         }
//         .into();
//
//         let result = auth_service.login(login).await;
//         assert!(result.is_ok());
//     }
//
//     #[tokio::test]
//     async fn test_login_failed_password()
//     {
//         let auth_service = create_auth_service().await;
//
//         let login = LoginRequest {
//             // its already registered
//             email: "non_existing_email@gmail.com".to_string(),
//             password: "123456789".to_string(),
//         }
//         .into();
//
//         let result = auth_service.login(login).await;
//         assert!(result.is_err());
//     }
//
//     #[tokio::test]
//     async fn test_login_failed_email()
//     {
//         let auth_service = create_auth_service().await;
//         let login: Login = LoginRequest {
//             email: "non_existing_email@wrong.com".to_string(),
//             password: "12345678".to_string(),
//         }
//         .into();
//
//         let result = auth_service.login(login).await;
//         assert!(result.is_err());
//     }
// }
