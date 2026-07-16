use std::sync::Arc;

use axum::{Json, extract::State};
use serde_json::Value;

use crate::{
    config::app::AppState,
    dto::{
        api_response::ApiResponse,
        auth::{
            login_request::LoginRequest, login_response::LoginResponse,
            register_request::RegisterRequest,
        },
        user::user_response::UserResponse,
    },
    errors::app_error::AppError,
};

pub async fn register(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<Value>,
) -> Result<Json<ApiResponse<UserResponse>>, AppError> {
    let request: RegisterRequest = serde_json::from_value(payload)
        .map_err(|e| AppError::BadRequest(format!("Invalid request payload: {}", e)))?;

    let auth_service = &state.auth_service;
    let register_response = auth_service.register(request).await?;
    let api_response = ApiResponse::<UserResponse>::success(register_response);

    Ok(Json(api_response))
}

pub async fn login(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<Value>,
) -> Result<Json<ApiResponse<LoginResponse>>, AppError> {
    let request: LoginRequest = serde_json::from_value(payload)
        .map_err(|e| AppError::BadRequest(format!("Invalid request payload: {}", e)))?;

    let auth_service = &state.auth_service;
    let login_response = auth_service.login(request).await?;
    let api_response = ApiResponse::<LoginResponse>::success(login_response);

    Ok(Json(api_response))
}

pub async fn change_password() {}
pub async fn reset_password() {}
