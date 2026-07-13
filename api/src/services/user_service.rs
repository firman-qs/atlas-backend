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

    pub async fn get_by_email(&self, email: &str) -> Result<UserResponse, AppError> {
        let user = self.user_repository.find_by_email(email).await?;
        let user = user.ok_or_else(|| AppError::NotFound(MSG_USER_NOT_FOUND.into()))?;
        Ok(user.into())
    }

    pub async fn get_by_id(&self, id: Uuid) -> Result<UserResponse, AppError> {
        let user = self.user_repository.find_by_id(id).await?;
        let user = user.ok_or_else(|| AppError::NotFound(MSG_USER_NOT_FOUND.into()))?;
        Ok(user.into())
    }

    pub async fn get_all(&self) -> Result<Vec<UserResponse>, AppError> {
        let users = self.user_repository.find_all().await?;
        Ok(users.into_iter().map(|user| user.into()).collect())
    }

    pub async fn delete(&self, id: Uuid) -> Result<(), AppError> {
        self.user_repository.delete(id).await?;
        Ok(())
    }

    pub async fn activate(&self, id: Uuid) -> Result<UserResponse, AppError> {
        Ok(self.user_repository.activate(id).await?.into())
    }

    pub async fn deactivate(&self, id: Uuid) -> Result<UserResponse, AppError> {
        Ok(self.user_repository.deactivate(id).await?.into())
    }
}
