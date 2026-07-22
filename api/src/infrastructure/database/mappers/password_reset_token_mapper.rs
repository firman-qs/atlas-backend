use sea_orm::ActiveValue::Set;
use sea_orm::IntoActiveModel;

use crate::domain::entities::password_reset_token::PasswordResetToken;
use crate::domain::entities::password_reset_token::PasswordResetTokenNew;
use crate::domain::entities::password_reset_token::PasswordResetTokenUpdate;

impl From<entity::password_reset_tokens::Model> for PasswordResetToken {
    fn from(active_model: entity::password_reset_tokens::Model) -> Self {
        Self {
            id: active_model.id,
            user_id: active_model.user_id,
            token_hash: active_model.token_hash,
            created_at: active_model.created_at,
            expires_at: active_model.expires_at,
            used_at: active_model.used_at,
        }
    }
}

impl IntoActiveModel<entity::password_reset_tokens::ActiveModel> for PasswordResetTokenNew {
    fn into_active_model(self) -> entity::password_reset_tokens::ActiveModel {
        entity::password_reset_tokens::ActiveModel {
            user_id: Set(self.user_id),
            token_hash: Set(self.token_hash),
            expires_at: Set(self.expires_at),
            ..Default::default()
        }
    }
}

impl IntoActiveModel<entity::password_reset_tokens::ActiveModel> for PasswordResetTokenUpdate {
    fn into_active_model(self) -> entity::password_reset_tokens::ActiveModel {
        entity::password_reset_tokens::ActiveModel {
            used_at: self
                .used_at
                .map_or(sea_orm::ActiveValue::NotSet, |v| Set(v.into())),
            ..Default::default()
        }
    }
}
