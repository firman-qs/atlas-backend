use entity::password_reset_tokens;
use sea_orm::ActiveModelTrait;
use sea_orm::ActiveValue::Set;
use sea_orm::ColumnTrait;
use sea_orm::DatabaseConnection;
use sea_orm::EntityTrait;
use sea_orm::IntoActiveModel;
use sea_orm::QueryFilter;
use uuid::Uuid;

use crate::domain::entities::password_reset_token::PasswordResetToken;
use crate::domain::entities::password_reset_token::PasswordResetTokenNew;
use crate::domain::errors::repository_error::RepositoryError;

#[derive(Debug)]
pub struct PgPasswordResetTokensRepository {
    db: DatabaseConnection,
}

impl PgPasswordResetTokensRepository {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    pub async fn create(
        &self,
        new_prt: PasswordResetTokenNew,
    ) -> Result<PasswordResetToken, RepositoryError> {
        let model = new_prt.into_active_model().insert(&self.db).await?;
        Ok(PasswordResetToken::from(model))
    }

    pub async fn find_prt(
        &self,
        token_hash: &str,
    ) -> Result<Option<PasswordResetToken>, RepositoryError> {
        let model = password_reset_tokens::Entity::find()
            .filter(password_reset_tokens::Column::TokenHash.eq(token_hash))
            .one(&self.db)
            .await?;

        Ok(model.map(PasswordResetToken::from))
    }

    pub async fn invalidate_prt(&self, id: Uuid) -> Result<(), RepositoryError> {
        let prt = password_reset_tokens::Entity::find_by_id(id)
            .one(&self.db)
            .await?
            .ok_or(RepositoryError::NotFound)?;

        let mut prt_active_model: password_reset_tokens::ActiveModel = prt.into_active_model();
        prt_active_model.used_at = Set(Some(chrono::Utc::now().into()));
        prt_active_model.update(&self.db).await?;

        Ok(())
    }

    pub async fn delete_by_user_id(&self, user_id: Uuid) -> Result<(), RepositoryError> {
        password_reset_tokens::Entity::delete_many()
            .filter(password_reset_tokens::Column::UserId.eq(user_id))
            .exec(&self.db)
            .await?;

        Ok(())
    }
}
