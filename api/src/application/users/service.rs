use std::sync::Arc;

use uuid::Uuid;

use crate::application::app_error::AppError;
use crate::application::users::results::user_result::UserResult;
use crate::infrastructure::database::repositories::pg_users_repository::PgUserRepository;
use crate::shared::constants::MSG_USER_NOT_FOUND;

pub struct UserService {
    user_repository: Arc<PgUserRepository>,
}

impl UserService {
    pub fn new(user_repository: Arc<PgUserRepository>) -> Self {
        Self { user_repository }
    }

    pub async fn get_by_email(&self, email: &str) -> Result<UserResult, AppError> {
        let user = self.user_repository.find_by_email(email).await?;
        let user = user.ok_or_else(|| AppError::NotFound(MSG_USER_NOT_FOUND.into()))?;

        Ok(user.into())
    }

    pub async fn get_by_id(&self, id: Uuid) -> Result<UserResult, AppError> {
        let user = self.user_repository.find_by_id(id).await?;
        let user = user.ok_or_else(|| AppError::NotFound(MSG_USER_NOT_FOUND.into()))?;

        Ok(user.into())
    }

    pub async fn get_all(&self) -> Result<Vec<UserResult>, AppError> {
        let users = self.user_repository.find_all().await?;

        Ok(users.into_iter().map(|user| user.into()).collect())
    }

    pub async fn delete(&self, id: Uuid) -> Result<(), AppError> {
        self.user_repository.delete(id).await?;

        Ok(())
    }

    pub async fn activate(&self, id: Uuid) -> Result<UserResult, AppError> {
        Ok(self.user_repository.activate(id).await?.into())
    }

    pub async fn deactivate(&self, id: Uuid) -> Result<UserResult, AppError> {
        Ok(self.user_repository.deactivate(id).await?.into())
    }
}
