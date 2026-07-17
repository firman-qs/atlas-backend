use std::sync::Arc;

use axum::{Router, routing::post};

use crate::{
    config::app::AppState,
    handlers::auth_handler::{forgot_password, login, register, reset_password},
};

pub fn auth_routes() -> Router<Arc<AppState>> {
    Router::new().nest(
        "/auth",
        Router::new()
            .route("/register", post(register))
            .route("/login", post(login))
            .route("/forgot-password", post(forgot_password))
            .route("/reset-password", post(reset_password)),
    )
}
