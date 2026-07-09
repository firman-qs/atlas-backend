use std::sync::Arc;

use crate::{
    dto::{
        auth::{
            change_password_request::ChangePasswordRequest, login_request::LoginRequest,
            login_response::LoginResponse, register_request::RegisterRequest,
        },
        user::user_response::UserResponse,
    },
    errors::app_error::AppError,
    repositories::user_repository::UserRepository,
    services::password_service::PasswordService,
};

#[derive(Debug)]
pub struct AuthService {
    user_repository: Arc<UserRepository>,
    password_service: PasswordService,
}

impl AuthService {
    pub fn new(user_repository: Arc<UserRepository>, password_service: PasswordService) -> Self {
        Self {
            user_repository,
            password_service,
        }
    }

    pub async fn register(&self, req: RegisterRequest) -> Result<UserResponse, AppError> {
        todo!()
    }

    pub async fn login(&self, req: LoginRequest) -> Result<LoginResponse, AppError> {
        todo!()
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
