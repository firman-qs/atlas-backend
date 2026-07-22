use serde::Serialize;
use utoipa::ToSchema;

#[derive(Debug, Serialize, ToSchema)]
pub struct JwtRefreshResult {
    pub access_token: String,
    pub refresh_token: String,
}
