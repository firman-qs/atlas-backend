use std::sync::Arc;

use axum::Router;
use axum::routing::get;

use crate::application::app_state::AppState;
use crate::presentation::handlers::user_handler;

pub fn user_routes() -> Router<Arc<AppState>> {
    Router::new().route("/users/{id}", get(user_handler::get_user))
}
