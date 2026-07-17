use std::env;

#[derive(Clone, Debug)]
pub struct Settings {
    pub database_url: String,
    pub base_url: String,
    pub rust_log: String,
    pub jwt_secret: String,
    pub access_token_exp_minutes: u64,
    pub refresh_token_exp_days: u64,
}

impl Settings {
    pub fn new() -> Self {
        dotenvy::dotenv().ok();
        let secret_key = env::var("MY_SECRET_KEY").expect("MY_SECRET_KEY must be set");
        let base_url = env::var("BASE_URL").unwrap_or_else(|_| "http://localhost:3000".into());
        Self {
            base_url,
            database_url: env::var("DATABASE_URL").expect("DATABASE_URL must be set"),
            rust_log: env::var("RUST_LOG").unwrap_or_else(|_| "info".into()),
            jwt_secret: secret_key.clone(),
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
