use entity::users;
use serde::Serialize;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize)]
pub struct UserResponse {
    pub id: Uuid,
    pub email: String,
    pub username: String,
    pub full_name: String,
    pub avatar_url: Option<String>,
}

impl From<users::Model> for UserResponse {
    fn from(user: users::Model) -> Self {
        Self {
            id: user.id,
            email: user.email,
            username: user.username,
            full_name: user.full_name,
            avatar_url: user.avatar_url,
        }
    }
}
