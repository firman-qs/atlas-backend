use chrono::Duration;
use chrono::Utc;
use jsonwebtoken::EncodingKey;
use jsonwebtoken::Header;
use uuid::Uuid;

use crate::infrastructure::jwt::jwt_claims::JwtClaims;
use crate::infrastructure::jwt::jwt_claims::TokenType;

#[derive(Debug, Clone)]
pub struct JwtManager {
    secret: String,
    access_exp_minuites: i64,
    refresh_exp_days: i64,
}

impl JwtManager {
    /// Creates a new instance of JwtService with the provided secret, access
    /// token expiration time in minutes, and refresh token expiration time in
    /// days.
    pub fn new(secret: String, access_exp_minuites: i64, refresh_exp_days: i64) -> Self {
        Self {
            secret,
            access_exp_minuites,
            refresh_exp_days,
        }
    }

    /// Generates an access token for the given user ID and optional
    /// impersonated
    pub fn generate_access_token(
        &self,
        user_id: Uuid,
        as_user_id: Option<Uuid>,
    ) -> Result<String, jsonwebtoken::errors::Error> {
        let exp_duration = Duration::minutes(self.access_exp_minuites);
        self.generate_token(user_id, as_user_id, exp_duration, TokenType::Access)
    }

    /// Generates a refresh token for the given user ID and optional
    /// impersonated
    pub fn generate_refresh_token(
        &self,
        user_id: Uuid,
        as_user_id: Option<Uuid>,
    ) -> Result<String, jsonwebtoken::errors::Error> {
        let exp_duration = Duration::days(self.refresh_exp_days);
        self.generate_token(user_id, as_user_id, exp_duration, TokenType::Refresh)
    }

    /// Verifies the provided access token and returns the claims if valid.
    pub fn verify_access_token(
        &self,
        token: &str,
    ) -> Result<JwtClaims, jsonwebtoken::errors::Error> {
        let claims = self.verify_token(token, TokenType::Access)?;

        Ok(claims)
    }

    /// Verifies the provided refresh token and returns the claims if valid.
    pub fn verify_refresh_token(
        &self,
        token: &str,
    ) -> Result<JwtClaims, jsonwebtoken::errors::Error> {
        let claims = self.verify_token(token, TokenType::Refresh)?;
        Ok(claims)
    }

    /// Generates a JWT token with the specified user ID, optional impersonated
    fn generate_token(
        &self,
        user_id: Uuid,
        as_user_id: Option<Uuid>,
        exp_duration: Duration,
        typ: TokenType,
    ) -> Result<String, jsonwebtoken::errors::Error> {
        let claims = JwtClaims {
            typ,
            sub: user_id,
            as_sub: as_user_id,
            exp: (Utc::now() + exp_duration).timestamp() as usize,
        };

        let token = jsonwebtoken::encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(self.secret.as_bytes()),
        )?;
        Ok(token)
    }

    /// Verifies the provided JWT token and returns the claims if valid. It
    /// checks the token type and expiration time.
    fn verify_token(
        &self,
        token: &str,
        typ: TokenType,
    ) -> Result<JwtClaims, jsonwebtoken::errors::Error> {
        let data = jsonwebtoken::decode::<JwtClaims>(
            token,
            &jsonwebtoken::DecodingKey::from_secret(self.secret.as_bytes()),
            &jsonwebtoken::Validation::default(),
        )?;

        let claims = data.claims;

        if claims.typ != typ {
            return Err(jsonwebtoken::errors::Error::from(
                jsonwebtoken::errors::ErrorKind::InvalidToken,
            ));
        }

        Ok(claims)
    }
}

#[cfg(test)]
mod tests {
    use chrono::DateTime;
    use uuid::Uuid;

    use super::*;
    use crate::infrastructure::config::settings::Settings;

    #[test]
    fn test_should_generate_and_verify_token() {
        let settings = Settings::new();
        let jwt_service = JwtManager::new(
            settings.jwt_secret.clone(),
            settings.access_token_exp_minutes,
            settings.refresh_token_exp_days,
        );
        let user_id = Uuid::new_v4();
        let token = jwt_service.generate_access_token(user_id, None).unwrap();
        let claims = jwt_service.verify_access_token(&token).unwrap();

        println!(".... Claims: {:?}", claims);
        println!("exp (unix): {}", claims.exp);

        let exp = DateTime::<Utc>::from_timestamp(claims.exp as i64, 0).expect("invalid timestamp");

        println!("expires at (UTC): {}", exp);
        println!("now (UTC):        {}", Utc::now());

        assert_eq!(claims.sub, user_id);
    }
}
