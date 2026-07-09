use std::env;

#[derive(Debug, Clone)]
pub struct Settings {
    pub database_url: String,
    pub rust_log: String,
}

impl Settings {
    pub fn new() -> Self {
        dotenvy::dotenv().ok();
        Self {
            database_url: env::var("DATABASE_URL").expect("DATABASE_URL must be set"),
            rust_log: env::var("RUST_LOG").unwrap_or_else(|_| "info".into()),
        }
    }
}
