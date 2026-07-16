use serde::Serialize;
use utoipa::ToSchema;

use crate::dto::user::user_response::UserResponse;

#[derive(Debug, Serialize, ToSchema)]
pub struct LoginResponse {
    pub access_token: String,
    pub refresh_token: String,
    pub user: UserResponse,
}
