use axum::{
    http::StatusCode,
    response::{Html, IntoResponse},
};

pub async fn health() -> impl IntoResponse {
    (
        StatusCode::OK,
        Html(
            r#"
<h1>Your backend is Healthy :)</h1>
<p>If you're not firmanqs, you supposed not to go there, but any way, Hello!</p>
"#,
        ),
    )
}
