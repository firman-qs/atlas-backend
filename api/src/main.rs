use crate::{
    config::{app::AppState, database::connect, settings::Settings},
    routes::{auth::auth_routes, user::user_routes},
};
use axum::{Router, routing::get};
use sea_orm::DatabaseConnection;
use std::sync::Arc;
use tokio::net::TcpListener;

mod common;
mod config;
mod dto;
mod errors;
mod handlers;
mod models;
mod repositories;
mod routes;
mod services;
mod validation;

#[tokio::main]
async fn main() {
    let settings = Settings::new();

    let db: DatabaseConnection = connect(&settings.database_url)
        .await
        .expect("Cannot connect database");

    let app_state = Arc::new(AppState::new(settings, db.clone()));

    tracing_subscriber::fmt::init();

    let app = Router::new()
        .route("/health", get(routes::health::health))
        .merge(user_routes())
        .merge(auth_routes())
        .with_state(app_state);

    let listener: TcpListener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    tracing::info!("Server is listeing to {}", listener.local_addr().unwrap());

    axum::serve(listener, app).await.unwrap();
}
