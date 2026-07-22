use serde::Serialize;
use utoipa::ToSchema;
use uuid::Uuid;

use crate::domain::entities::user::User;

#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct UserResult {
    pub id: Uuid,
    pub email: String,
    pub username: String,
    pub full_name: String,
    pub avatar_url: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct UserListResult {
    pub responses: Vec<UserResult>,
}

impl From<User> for UserResult {
    fn from(user: User) -> Self {
        Self {
            id: user.id,
            email: user.email,
            username: user.username,
            full_name: user.full_name,
            avatar_url: user.avatar_url,
        }
    }
}
