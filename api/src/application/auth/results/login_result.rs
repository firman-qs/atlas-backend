use serde::Serialize;
use utoipa::ToSchema;

use crate::application::users::results::user_result::UserResult;

#[derive(Debug, Serialize, ToSchema)]
pub struct LoginResult {
    pub access_token: String,
    pub refresh_token: String,
    pub user: UserResult,
}
