use axum::Router;
use axum::routing::get;
use std::sync::Arc;

use crate::config::app::AppState;
use crate::handlers::user_handler::get_user;

pub fn user_routes() -> Router<Arc<AppState>> {
    Router::new().route("/users/{id}", get(get_user))
}
