use garde::Validate;
use serde::Deserialize;

#[derive(Debug, Deserialize, Validate)]
pub struct LoginRequest {
    #[garde(email)]
    pub email: String,
    #[garde(length(min = 8))]
    pub password: String,
}
