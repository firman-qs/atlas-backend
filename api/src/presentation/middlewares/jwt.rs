use std::sync::Arc;

use axum::extract::Request;
use axum::extract::State;
use axum::middleware::Next;
use axum::response::IntoResponse;

use crate::application::app_error::AppError;
use crate::application::app_state::AppState;

/// Middleware function to verify JWT tokens in incoming requests.
pub async fn jwt_middleware(
    State(state): State<Arc<AppState>>,
    mut request: Request,
    next: Next,
) -> Result<impl IntoResponse, AppError> {
    let header = request
        .headers()
        .get(axum::http::header::AUTHORIZATION)
        .and_then(|value| value.to_str().ok());

    let token = header
        .and_then(|h| h.strip_prefix("Bearer "))
        .ok_or_else(|| AppError::Unauthorized("You are not authorized".to_string()))?;

    let jwt_service = state.jwt_manager.clone();

    let claims = jwt_service
        .verify_access_token(token)
        .map_err(|_| AppError::Unauthorized("Invalid token".to_string()))?;

    request.extensions_mut().insert(claims);

    Ok(next.run(request).await)
}
