use axum::{
    http::StatusCode,
    response::{Html, IntoResponse},
};

pub async fn health() -> impl IntoResponse {
    (
        StatusCode::OK,
        Html(
            r#"
<style>
    body {
        background-color: #000000;
        margin: 0;
        padding: 0;
        color: #dddddd;
        font-family: monospace;
    }
</style>
<div>
<h1>Your backend is Healthy :)</h1>
<p>If you're not firmanqs, you supposed not to go there, but any way, Hello!</p>
</div>
"#,
        ),
    )
}
