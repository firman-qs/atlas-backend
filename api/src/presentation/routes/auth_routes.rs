use std::sync::Arc;

use axum::Router;
use axum::routing::post;

use crate::application::app_state::AppState;
use crate::presentation::handlers::auth_handler;

pub fn public_auth_routes() -> Router<Arc<AppState>> {
    Router::new().nest(
        "/auth",
        Router::new()
            .route("/register", post(auth_handler::register))
            .route("/login", post(auth_handler::login))
            .route("/forgot-password", post(auth_handler::forgot_password))
            .route("/reset-password", post(auth_handler::reset_password))
            .route("/refresh-token", post(auth_handler::jwt_refresh)),
    )
}

pub fn protected_auth_routes() -> Router<Arc<AppState>> {
    Router::new().nest("/auth", Router::new())
}
