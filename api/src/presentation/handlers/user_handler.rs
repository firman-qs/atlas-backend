use std::sync::Arc;

use axum::Json;
use axum::extract::Path;
use axum::extract::State;
use uuid::Uuid;

use crate::application::app_error::AppError;
use crate::application::app_state::AppState;
use crate::application::users::results::user_result::UserResult;
use crate::presentation::responses::api_response::ApiResponse;

pub async fn get_user(
    Path(uuid): Path<Uuid>,
    State(state): State<Arc<AppState>>,
) -> Result<Json<ApiResponse<UserResult>>, AppError> {
    let user = state.user_service.get_by_id(uuid).await?;
    Ok(Json(ApiResponse::<UserResult>::success(user)))
}

pub async fn delete(
    Path(uuid): Path<Uuid>,
    State(state): State<Arc<AppState>>,
) -> Result<Json<ApiResponse<()>>, AppError> {
    state.user_service.delete(uuid).await?;
    Ok(Json(ApiResponse::<()>::success(())))
}
