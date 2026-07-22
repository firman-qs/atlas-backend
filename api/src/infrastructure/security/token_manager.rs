use argon2::password_hash::rand_core::OsRng;
use argon2::password_hash::rand_core::RngCore;
use sha2::Digest;
use sha2::Sha256;

pub struct TokenManager;

impl TokenManager {
    pub fn new() -> Self {
        Self {}
    }

    pub fn generate_token(&self) -> String {
        let mut bytes = [0u8; 32];
        OsRng.fill_bytes(&mut bytes);
        let token = hex::encode(bytes);
        token
    }

    pub fn hash_token(&self, token: &str) -> String {
        let hash = Sha256::digest(token.as_bytes());
        let token_hash = hex::encode(hash);
        token_hash
    }
}
