use garde::Validate;
use serde::Deserialize;

#[derive(Debug, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct RegisterRequest {
    #[garde(email)]
    pub email: String,

    #[garde(length(min = 3))]
    pub username: String,

    #[garde(length(min = 8))]
    pub password: String,

    #[garde(length(min = 1, max = 255))]
    pub full_name: String,
}
