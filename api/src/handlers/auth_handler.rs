use std::sync::Arc;

use axum::{Json, extract::State};
use serde_json::Value;

use crate::{
    config::app::AppState,
    dto::{
        api_response::ApiResponse,
        auth::{
            forgot_password_request::ForgotPasswordRequest, login_request::LoginRequest,
            login_response::LoginResponse, register_request::RegisterRequest,
            reset_password_request::ResetPasswordRequest,
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
    let response = auth_service.login(request).await?;
    let api = ApiResponse::<LoginResponse>::success(response);

    Ok(Json(api))
}

pub async fn forgot_password(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<Value>,
) -> Result<Json<ApiResponse<String>>, AppError> {
    let request: ForgotPasswordRequest = serde_json::from_value(payload)
        .map_err(|e| AppError::BadRequest(format!("Invalid request payload: {}", e)))?;

    let auth_service = &state.auth_service;
    auth_service.forgot_password(request).await?;
    let api = ApiResponse::<String>::success("If this email exists in our system, a password reset link has been sent. Please check your email.".to_string());
    Ok(Json(api))
}

pub async fn reset_password(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<Value>,
) -> Result<Json<ApiResponse<String>>, AppError> {
    let request: ResetPasswordRequest = serde_json::from_value(payload)
        .map_err(|e| AppError::BadRequest(format!("Invalid request payload: {}", e)))?;

    let auth_service = &state.auth_service;
    auth_service.reset_password(request).await?;
    let api = ApiResponse::<String>::success("Password has been reset successfully.".to_string());
    Ok(Json(api))
}

pub async fn change_password() {}
