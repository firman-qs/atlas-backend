use std::env;

#[derive(Debug, Clone)]
pub struct Settings {
    pub database_url: String,
    pub rust_log: String,
    pub jwt_secret: String,
    pub access_token_exp_minutes: u64,
    pub refresh_token_exp_days: u64,
}

impl Settings {
    pub fn new() -> Self {
        dotenvy::dotenv().ok();
        Self {
            database_url: env::var("DATABASE_URL").expect("DATABASE_URL must be set"),
            rust_log: env::var("RUST_LOG").unwrap_or_else(|_| "info".into()),
            jwt_secret: env::var("JWT_SECRET").expect("JWT_SECRET must be set"),

            access_token_exp_minutes: env::var("ACCESS_TOKEN_EXP_MINUTES")
                .unwrap_or_else(|_| "30".into())
                .parse()
                .expect("ACCESS_TOKEN_EXPIRATION_MINUTES must be a number"),

            refresh_token_exp_days: env::var("REFRESH_TOKEN_EXP_DAYS")
                .unwrap_or_else(|_| "1".into())
                .parse()
                .expect("REFRESH_TOKEN_EXPIRATION_DAYS must be a number"),
        }
    }
}
