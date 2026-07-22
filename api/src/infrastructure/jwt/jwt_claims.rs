use serde::Deserialize;
use serde::Serialize;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct JwtClaims {
    pub sub: Uuid,
    pub as_sub: Option<Uuid>,
    pub exp: usize,
    pub typ: TokenType,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub enum TokenType {
    Access,
    Refresh,
}
