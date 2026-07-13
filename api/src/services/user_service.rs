use std::sync::Arc;

use uuid::Uuid;

use crate::{
    common::constants::MSG_USER_NOT_FOUND, dto::user::user_response::UserResponse,
    errors::app_error::AppError, repositories::user_repository::UserRepository,
};

#[derive(Debug)]
pub struct UserService {
    user_repository: Arc<UserRepository>,
}

impl UserService {
    pub fn new(user_repository: Arc<UserRepository>) -> Self {
        Self { user_repository }
    }

    // just an example of a service method that uses the repository to find a user by email
    pub async fn get_user_by_email(&self, email: &str) -> Result<UserResponse, AppError> {
        let user = self.user_repository.find_by_email(email).await?;
        let user = user.ok_or_else(|| AppError::NotFound(MSG_USER_NOT_FOUND.into()))?;
        Ok(user.into())
    }

    pub async fn get_user_by_id(&self, id: Uuid) -> Result<UserResponse, AppError> {
        let user = self.user_repository.find_by_id(id).await?;
        let user = user.ok_or_else(|| AppError::NotFound(MSG_USER_NOT_FOUND.into()))?;
        Ok(user.into())
    }
}
