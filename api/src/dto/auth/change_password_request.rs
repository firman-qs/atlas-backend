use garde::Validate;
use serde::Deserialize;

#[derive(Debug, Deserialize, Validate)]
pub struct ChangePasswordRequest {
    #[garde(skip)]
    pub old_password: String,
    #[garde(length(min = 8))]
    pub new_password: String,
}
