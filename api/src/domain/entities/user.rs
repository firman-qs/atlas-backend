use entity::sea_orm_active_enums::UserRoleEnum;
use sea_orm::prelude::DateTimeWithTimeZone;
use uuid::Uuid;

pub struct User {
    pub id: Uuid,
    pub email: String,
    pub username: String,
    pub password_hash: String,
    pub full_name: String,
    pub avatar_url: Option<String>,
    pub is_active: bool,
    pub created_at: DateTimeWithTimeZone,
    pub updated_at: DateTimeWithTimeZone,
    pub must_change_password: bool,
    pub role: UserRoleEnum,
}

pub struct UserNew {
    pub email: String,
    pub username: String,
    pub password_hash: String,
    pub full_name: String,
    pub avatar_url: Option<String>,
}

pub struct UserUpdate {
    pub id: Uuid,
    pub username: Option<String>,
    pub password_hash: Option<String>,
    pub full_name: Option<String>,
    pub avatar_url: Option<Option<String>>,
    pub is_active: Option<bool>,
    pub must_change_password: Option<bool>,
}
