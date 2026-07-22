use axum::extract::Request;
use axum::middleware::Next;
use axum::response::IntoResponse;

use crate::application::app_error::AppError;

pub async fn auth(
    // State(state): State<Arc<AppState>>,
    request: Request,
    next: Next,
) -> Result<impl IntoResponse, AppError> {
    Ok(next.run(request).await)
}
