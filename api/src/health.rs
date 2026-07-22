use std::sync::Arc;

use axum::Extension;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::Html;
use axum::response::IntoResponse;

use crate::application::app_error::AppError;
use crate::application::app_state::AppState;
use crate::infrastructure::jwt::jwt_claims::JwtClaims;

pub async fn health(
    State(state): State<Arc<AppState>>,
    Extension(ext): Extension<JwtClaims>,
) -> Result<impl IntoResponse, AppError> {
    let user = state.user_service.get_by_id(ext.sub).await?;
    let user = user.full_name;
    Ok((
        StatusCode::OK,
        Html(format!(
            r#"
<style>
    body {{ background-color: #000000; margin: 0; padding: 0; color: #dddddd; font-family: monospace; }}
</style>
<div>
    <h1>Hello {u}, Your backend is Healthy :)</h1>
    <p>If you're not firmanqs, you're not supposed to be here, but anyway, Hello!</p>
</div>
"#,
            u = user
        )),
    ))
}
