use chrono::{Duration, Utc};
use jsonwebtoken::{EncodingKey, Header};
use uuid::Uuid;

use crate::models::auth::{jwt_claims::JwtClaims, user_role::UserRole};

#[derive(Debug)]
pub struct JwtService {
    secret: String,
    access_exp_minuites: u64,
    refresh_exp_days: u64,
}

impl JwtService {
    pub fn new(secret: String, access_exp_minuites: u64, refresh_exp_days: u64) -> Self {
        Self {
            secret,
            access_exp_minuites,
            refresh_exp_days,
        }
    }

    pub fn generate_access_token(
        &self,
        user_id: Uuid,
    ) -> Result<String, jsonwebtoken::errors::Error> {
        let role = UserRole::Student;
        let exp_duration = Duration::minutes(30);

        self.generate_token(user_id, role, exp_duration)
    }

    pub fn generate_refresh_token(
        &self,
        user_id: Uuid,
    ) -> Result<String, jsonwebtoken::errors::Error> {
        let role = UserRole::Student;
        let exp_duration = Duration::minutes(180);

        self.generate_token(user_id, role, exp_duration)
    }

    pub fn verify_token(&self, token: &str) -> Result<JwtClaims, jsonwebtoken::errors::Error> {
        let data = jsonwebtoken::decode::<JwtClaims>(
            token,
            &jsonwebtoken::DecodingKey::from_secret(self.secret.as_bytes()),
            &jsonwebtoken::Validation::default(),
        )?;

        Ok(data.claims)
    }

    fn generate_token(
        &self,
        user_id: Uuid,
        role: UserRole,
        exp_duration: Duration,
    ) -> Result<String, jsonwebtoken::errors::Error> {
        let claims = JwtClaims {
            role,
            sub: user_id,
            exp: (Utc::now() + exp_duration).timestamp() as usize,
        };

        let token = jsonwebtoken::encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(self.secret.as_bytes()),
        )?;
        Ok(token)
    }
}

#[cfg(test)]
mod tests {
    use crate::config::settings::Settings;

    use super::*;
    use chrono::DateTime;
    use uuid::Uuid;

    #[test]
    fn test_should_generate_and_verify_token() {
        let settings = Settings::new();
        let jwt_service = JwtService::new(
            settings.jwt_secret.clone(),
            settings.access_token_exp_minutes,
            settings.refresh_token_exp_days,
        );
        let user_id = Uuid::new_v4();
        let token = jwt_service.generate_access_token(user_id).unwrap();
        let claims = jwt_service.verify_token(&token).unwrap();

        println!(".... Claims: {:?}", claims);
        println!("exp (unix): {}", claims.exp);

        let exp = DateTime::<Utc>::from_timestamp(claims.exp as i64, 0).expect("invalid timestamp");

        println!("expires at (UTC): {}", exp);
        println!("now (UTC):        {}", Utc::now());

        assert_eq!(claims.sub, user_id);
        assert_eq!(claims.role, UserRole::Student);
    }
}
