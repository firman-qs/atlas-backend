use std::sync::Arc;

use axum::Json;
use axum::extract::{Path, State};
use uuid::Uuid;

use crate::config::app::AppState;
use crate::dto::api_response::ApiResponse;
use crate::dto::user::user_response::UserResponse;
use crate::errors::app_error::AppError;

pub async fn get_user(
    Path(uuid): Path<Uuid>,
    State(state): State<Arc<AppState>>,
) -> Result<Json<ApiResponse<UserResponse>>, AppError> {
    let user = state.user_service.get_user_by_id(uuid).await?;
    Ok(Json(ApiResponse::<UserResponse>::success(user)))
}
