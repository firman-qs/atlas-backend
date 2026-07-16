use std::sync::Arc;

use axum::{Router, routing::post};

use crate::{
    config::app::AppState,
    handlers::auth_handler::{login, register},
};

pub fn auth_routes() -> Router<Arc<AppState>> {
    Router::new()
        .route("/register", post(register))
        .route("/login", post(login))
}
