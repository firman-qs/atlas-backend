use garde::Validate;
use serde::Deserialize;

#[derive(Debug, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct LoginRequest {
    #[garde(email)]
    pub email: String,
    #[garde(length(min = 8))]
    pub password: String,
}
