use std::sync::Arc;

use sea_orm::DatabaseConnection;

use crate::{
    config::settings::Settings,
    repositories::user_repository::UserRepository,
    services::{
        auth_service::AuthService, jwt_service::JwtService, password_service::PasswordService,
        user_service::UserService,
    },
};

#[derive(Debug)]
pub struct AppState {
    pub settings: Settings,
    pub db: sea_orm::DatabaseConnection,
    pub user_service: Arc<UserService>,
    pub auth_service: Arc<AuthService>,
    pub password_service: Arc<PasswordService>,
    pub jwt_service: Arc<JwtService>,
}

impl AppState {
    pub fn new(settings: Settings, db: DatabaseConnection) -> Self {
        let user_repository = Arc::new(UserRepository::new(db.clone()));
        let user_service = Arc::new(UserService::new(user_repository.clone()));
        let password_service = Arc::new(PasswordService::new());

        let jwt_service = Arc::new(JwtService::new(
            settings.jwt_secret.clone(),
            settings.access_token_exp_minutes,
            settings.refresh_token_exp_days,
        ));

        let auth_service = Arc::new(AuthService::new(
            user_repository,
            password_service.clone(),
            jwt_service.clone(),
        ));

        Self {
            settings,
            db,
            password_service,
            user_service,
            auth_service,
            jwt_service,
        }
    }
}
