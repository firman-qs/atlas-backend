use std::sync::Arc;

use axum::Router;
use axum::routing::post;

use crate::application::app_state::AppState;
use crate::presentation::handlers::admin_handler;

pub fn admin_routes() -> Router<Arc<AppState>> {
    Router::new().nest(
        "/admin",
        Router::new()
            // adress: http://localhost:3000/admin/question-pkg/import
            .route(
                "/curriculum-pkg/import",
                post(admin_handler::curriculum_pkg_import),
            )
            .route(
                "/questions-pkg/import",
                post(admin_handler::questions_pkg_import),
            ),
    )
}
