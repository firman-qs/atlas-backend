use entity::password_reset_tokens;
use sea_orm::{ActiveValue::Set, IntoActiveModel, prelude::DateTimeWithTimeZone};
use uuid::Uuid;

pub struct PasswordResetToken {
    pub user_id: Uuid,
    pub token_hash: String,
    pub expires_at: DateTimeWithTimeZone,
}

impl IntoActiveModel<password_reset_tokens::ActiveModel> for PasswordResetToken {
    fn into_active_model(self) -> password_reset_tokens::ActiveModel {
        password_reset_tokens::ActiveModel {
            user_id: Set(self.user_id),
            token_hash: Set(self.token_hash),
            expires_at: Set(self.expires_at),
            ..Default::default()
        }
    }
}
