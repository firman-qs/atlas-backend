use std::sync::Arc;

use axum::Json;
use axum::extract::Request;
use axum::extract::State;
use garde::Validate;
use serde_json::Value;

use crate::application::app_error::AppError;
use crate::application::app_state::AppState;
use crate::application::auth::commands::login::Login;
use crate::application::auth::results::jwt_refresh_result::JwtRefreshResult;
use crate::application::auth::results::login_result::LoginResult;
use crate::application::users::results::user_result::UserResult;
use crate::presentation::requests::auth::forgot_password_request::ForgotPasswordRequest;
use crate::presentation::requests::auth::login_request::LoginRequest;
use crate::presentation::requests::auth::register_request::RegisterRequest;
use crate::presentation::requests::auth::reset_password_request::ResetPasswordRequest;
use crate::presentation::responses::api_response::ApiResponse;

pub async fn register(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<Value>,
) -> Result<Json<ApiResponse<UserResult>>, AppError> {
    let request: RegisterRequest = serde_json::from_value(payload)
        .map_err(|e| AppError::BadRequest(format!("Invalid request payload: {}", e)))?;

    let auth_service = &state.auth_service;

    let result = auth_service.register(request.into()).await?;

    let response = ApiResponse::<UserResult>::success(result);

    Ok(Json(response))
}

pub async fn login(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<Value>,
) -> Result<Json<ApiResponse<LoginResult>>, AppError> {
    let request: LoginRequest = serde_json::from_value(payload)
        .map_err(|e| AppError::BadRequest(format!("Invalid request payload: {}", e)))?;

    request.validate()?;

    let login = Login {
        email: request.email.clone(),
        password: request.password.clone(),
    };

    let auth_service = &state.auth_service;
    let result = auth_service.login(login).await?;
    let response = ApiResponse::<LoginResult>::success(result);

    Ok(Json(response))
}

pub async fn forgot_password(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<Value>,
) -> Result<Json<ApiResponse<String>>, AppError> {
    let request: ForgotPasswordRequest = serde_json::from_value(payload)
        .map_err(|e| AppError::BadRequest(format!("Invalid request payload: {}", e)))?;

    let auth_service = &state.auth_service;

    auth_service.forgot_password(request.into()).await?;

    let response = ApiResponse::<String>::success(
        "If this email exists in our system, a password reset link has been sent. Please check \
         your email."
            .to_string(),
    );

    Ok(Json(response))
}

pub async fn reset_password(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<Value>,
) -> Result<Json<ApiResponse<String>>, AppError> {
    let request: ResetPasswordRequest = serde_json::from_value(payload)
        .map_err(|e| AppError::BadRequest(format!("Invalid request payload: {}", e)))?;

    let auth_service = &state.auth_service;

    auth_service.reset_password(request.into()).await?;

    let api = ApiResponse::<String>::success("Password has been reset successfully.".to_string());

    Ok(Json(api))
}

pub async fn change_password() {}

pub async fn jwt_refresh(
    State(state): State<Arc<AppState>>,
    request: Request,
) -> Result<Json<ApiResponse<JwtRefreshResult>>, AppError> {
    let token = request
        .headers()
        .get(axum::http::header::AUTHORIZATION)
        .and_then(|value| value.to_str().ok())
        .and_then(|h| h.strip_prefix("Bearer "))
        .ok_or_else(|| AppError::Unauthorized("You are not authorized".to_string()))?;

    let claims = state.jwt_manager.verify_refresh_token(token)?;
    let user_id = state.user_service.get_by_id(claims.sub).await?.id;

    let as_user_id = if let Some(as_user) = claims.as_sub {
        Some(state.user_service.get_by_id(as_user).await?.id)
    } else {
        None
    };

    let new_refresh_token = state
        .jwt_manager
        .generate_refresh_token(user_id, as_user_id)?;

    let new_access_token = state
        .jwt_manager
        .generate_access_token(user_id, as_user_id)?;

    let result = JwtRefreshResult {
        access_token: new_access_token,
        refresh_token: new_refresh_token,
    };

    let response = ApiResponse::<JwtRefreshResult>::success(result);

    Ok(Json(response))
}
