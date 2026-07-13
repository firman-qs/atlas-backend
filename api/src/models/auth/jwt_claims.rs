use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::models::auth::user_role::UserRole;

#[derive(Debug, Serialize, Deserialize)]
pub struct JwtClaims {
    pub sub: Uuid,
    pub role: UserRole,
    pub exp: usize,
}
