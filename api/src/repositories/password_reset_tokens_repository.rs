use entity::password_reset_tokens;
use sea_orm::{
    ActiveModelTrait, ActiveValue::Set, ColumnTrait, DatabaseConnection, DbErr, EntityTrait,
    IntoActiveModel, QueryFilter,
};
use uuid::Uuid;

use crate::models::auth::password_reset_token::PasswordResetToken;

#[derive(Debug)]
pub struct PasswordResetTokensRepository {
    db: DatabaseConnection,
}

impl PasswordResetTokensRepository {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    pub async fn create(
        &self,
        prt: PasswordResetToken,
    ) -> Result<password_reset_tokens::Model, DbErr> {
        prt.into_active_model().insert(&self.db).await
    }

    pub async fn find_prt(
        &self,
        token_hash: &str,
    ) -> Result<Option<password_reset_tokens::Model>, DbErr> {
        password_reset_tokens::Entity::find()
            .filter(password_reset_tokens::Column::TokenHash.eq(token_hash))
            .one(&self.db)
            .await
    }

    pub async fn invalidate_prt(&self, id: Uuid) -> Result<(), DbErr> {
        let prt = password_reset_tokens::Entity::find_by_id(id)
            .one(&self.db)
            .await?
            .ok_or(DbErr::RecordNotFound(format!(
                "Password reset token with id {} not found",
                id
            )))?;

        let mut prt_active_model: password_reset_tokens::ActiveModel = prt.into_active_model();
        prt_active_model.used_at = Set(Some(chrono::Utc::now().into()));
        prt_active_model.update(&self.db).await?;
        Ok(())
    }

    pub async fn delete_by_user_id(&self, user_id: Uuid) -> Result<(), DbErr> {
        password_reset_tokens::Entity::delete_many()
            .filter(password_reset_tokens::Column::UserId.eq(user_id))
            .exec(&self.db)
            .await?;
        Ok(())
    }
}
