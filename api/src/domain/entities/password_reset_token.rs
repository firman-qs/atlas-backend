use sea_orm::prelude::DateTimeWithTimeZone;
use uuid::Uuid;

pub struct PasswordResetToken {
    pub id: Uuid,
    pub user_id: Uuid,
    pub token_hash: String,
    pub expires_at: DateTimeWithTimeZone,
    pub used_at: Option<DateTimeWithTimeZone>,
    pub created_at: DateTimeWithTimeZone,
}

pub struct PasswordResetTokenNew {
    pub user_id: Uuid,
    pub token_hash: String,
    pub expires_at: DateTimeWithTimeZone,
}

pub struct PasswordResetTokenUpdate {
    pub used_at: Option<DateTimeWithTimeZone>,
}
