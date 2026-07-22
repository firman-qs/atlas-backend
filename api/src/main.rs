use std::sync::Arc;

use axum::Router;
use axum::routing::get;
use sea_orm::DatabaseConnection;
use tokio::net::TcpListener;
use toml::Table;

use crate::application::app_state::AppState;
use crate::health::health;
use crate::infrastructure::config::settings::Settings;
use crate::infrastructure::database::connection::connect;
use crate::presentation::middlewares::jwt::jwt_middleware;
use crate::presentation::routes::admin_routes::admin_routes;
use crate::presentation::routes::auth_routes::protected_auth_routes;
use crate::presentation::routes::auth_routes::public_auth_routes;
use crate::presentation::routes::user_route::user_routes;

mod application;
mod domain;
mod health;
mod infrastructure;
mod presentation;
mod shared;

#[tokio::main]
async fn main() {
    // Experiment =============================================================

    let value = "test = 'something'".parse::<Table>().unwrap();
    println!("Parsed TOML: {:?}", value);

    // ========================================================================
    let settings = Settings::new();

    let db: DatabaseConnection = connect(&settings.database_url)
        .await
        .expect("Cannot connect database");

    let app_state = Arc::new(AppState::new(settings, db.clone()));

    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    let app = Router::new()
        .route("/health", get(health))
        .merge(user_routes())
        .merge(admin_routes())
        .merge(protected_auth_routes())
        .route_layer(axum::middleware::from_fn_with_state(
            app_state.clone(),
            jwt_middleware,
        ))
        .merge(public_auth_routes())
        .with_state(app_state);

    let listener: TcpListener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    tracing::info!("Server running on {}", listener.local_addr().unwrap());

    axum::serve(listener, app).await.unwrap();
}
