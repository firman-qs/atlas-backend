use std::sync::Arc;

use sea_orm::DatabaseConnection;

use crate::{
    config::settings::Settings,
    repositories::user_repository::UserRepository,
    services::{
        auth_service::AuthService, password_service::PasswordService, user_service::UserService,
    },
};

#[derive(Debug)]
pub struct AppState {
    pub settings: Settings,
    pub db: sea_orm::DatabaseConnection,
    pub user_service: Arc<UserService>,
    pub auth_service: Arc<AuthService>,
}

impl AppState {
    pub fn new(settings: Settings, db: DatabaseConnection) -> Self {
        let user_repository = Arc::new(UserRepository::new(db.clone()));
        let user_service = UserService::new(user_repository.clone());

        let password_service = PasswordService::new();
        let auth_service = AuthService::new(user_repository, password_service);

        Self {
            settings,
            db,
            user_service: Arc::new(user_service),
            auth_service: Arc::new(auth_service),
        }
    }
}
