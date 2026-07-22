use std::sync::Arc;

use axum::Json;
use axum::extract::Multipart;
use axum::extract::State;

use crate::application::app_error::AppError;
use crate::application::app_state::AppState;
use crate::application::imports::curriculum::commands::import_curriculum_pkg_cmd::ImportCurriculumPkgCmd;
use crate::application::imports::curriculum::results::curriculum_pkg_summary::ImportCurriculumPkgSummary;
use crate::application::imports::questions::commands::import_questions_pkg_cmd::ImportQuestionsPkgCmd;
use crate::application::imports::questions::results::question_pkg_summary::ImportQuestionPkgSummary;
use crate::presentation::responses::api_response::ApiResponse;

pub async fn curriculum_pkg_import(
    State(state): State<Arc<AppState>>,
    multipart: Multipart,
) -> Result<Json<ApiResponse<ImportCurriculumPkgSummary>>, AppError>
{
    let contents = extract_field_as_string(multipart, "curriculum").await?;

    let cmd = ImportCurriculumPkgCmd { contents };
    let summary = state.curriculum_pkg_import_service.inspect(cmd.clone())?;
    state.curriculum_pkg_import_service.import(cmd).await?;

    tracing::info!("Curriculum package import summary: {:?}", summary);
    Ok(Json(ApiResponse::success(summary)))
}

pub async fn questions_pkg_import(
    State(state): State<Arc<AppState>>,
    multipart: Multipart,
) -> Result<Json<ApiResponse<ImportQuestionPkgSummary>>, AppError>
{
    let contents = extract_field_as_string(multipart, "questions").await?;

    let cmd = ImportQuestionsPkgCmd { contents };
    let summary = state.question_pkg_import_service.inspect(cmd.clone())?;
    state.question_pkg_import_service.import(cmd).await?;

    tracing::info!("Questions package import summary: {:?}", summary);
    Ok(Json(ApiResponse::success(summary)))
}

async fn extract_field_as_string(
    mut multipart: Multipart,
    expected_field: &str,
) -> Result<String, AppError>
{
    let mut file_bytes = None;

    while let Some(field) = multipart.next_field().await?
    {
        match field.name()
        {
            Some(name) if name == expected_field =>
            {
                if file_bytes.is_some()
                {
                    return Err(AppError::BadRequest(format!(
                        "Duplicate field '{expected_field}' provided"
                    )));
                }
                file_bytes = Some(field.bytes().await?);
            }
            Some(name) =>
            {
                return Err(AppError::BadRequest(format!(
                    "Unexpected field name: {name}"
                )));
            }
            None =>
            {
                return Err(AppError::BadRequest(
                    "Multipart field missing name".to_string(),
                ));
            }
        }
    }

    let bytes = file_bytes
        .ok_or_else(|| AppError::BadRequest(format!("Missing '{expected_field}' file")))?;

    std::str::from_utf8(&bytes)
        .map(|s| s.to_string())
        .map_err(|_| AppError::BadRequest(format!("Invalid UTF-8 in '{expected_field}' file")))
}
