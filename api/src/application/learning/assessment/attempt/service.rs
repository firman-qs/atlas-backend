use std::sync::Arc;

use entity::sea_orm_active_enums::AttemptStatusEnum;
use uuid::Uuid;

use crate::application::app_error::AppError;
use crate::application::learning::assessment::attempt::commands::start_assessment_attempt::StartAssessmentAttempt;
use crate::application::learning::assessment::attempt::results::assessment_attempt_result::AssessmentAttemptListResult;
use crate::application::learning::assessment::attempt::results::assessment_attempt_result::AssessmentAttemptResult;
use crate::domain::entities::assessment_attempt::AssessmentAttemptNew;
use crate::domain::entities::assessment_attempt::AssessmentAttemptUpdate;
use crate::domain::entities::solo_level::SoloLevelCode;
use crate::infrastructure::database::repositories::pg_assessment_attempts_repository::PgAssessmentAttemptRepository;
use crate::infrastructure::database::repositories::pg_solo_level_repository::PgSoloLevelRepository;

pub struct AssessmentAttemptService {
    repository: Arc<PgAssessmentAttemptRepository>,
    solo_level_repository: Arc<PgSoloLevelRepository>,
}

impl AssessmentAttemptService {
    pub fn new(
        repository: Arc<PgAssessmentAttemptRepository>,
        solo_level_repository: Arc<PgSoloLevelRepository>,
    ) -> Self {
        Self {
            repository,
            solo_level_repository,
        }
    }

    pub async fn start(
        &self,
        cmd: StartAssessmentAttempt,
    ) -> Result<AssessmentAttemptResult, AppError> {
        if let Some(existing_attempt) = self
            .repository
            .find_active_by_student_and_concept(cmd.student_id, cmd.concept_id)
            .await?
        {
            return Ok(existing_attempt.into());
        }

        let current = self
            .solo_level_repository
            .find_by_code(&SoloLevelCode::Unistructural)
            .await?
            .ok_or_else(|| AppError::NotFound("Missing SOLO level".into()))?;

        let target = self
            .solo_level_repository
            .find_by_code(&SoloLevelCode::Relational)
            .await?
            .ok_or_else(|| AppError::NotFound("Missing SOLO level".into()))?;

        let create_attempt = AssessmentAttemptNew {
            student_id: cmd.student_id,
            concept_id: cmd.concept_id,
            current_solo_level_id: current.id,
            target_solo_level_id: target.id,
            started_at: chrono::Utc::now().into(),
        };

        let attempt = self.repository.start(create_attempt).await?;
        Ok(attempt.into())
    }

    pub async fn get_by_id(&self, id: Uuid) -> Result<AssessmentAttemptResult, AppError> {
        let attempt = self.repository.find_by_id(id).await?;
        let attempt =
            attempt.ok_or_else(|| AppError::NotFound("Assessment attempt not found".into()))?;
        Ok(attempt.into())
    }

    pub async fn get_by_student(
        &self,
        student_id: Uuid,
    ) -> Result<AssessmentAttemptListResult, AppError> {
        let attempts = self.repository.find_by_student(student_id).await?;
        Ok(AssessmentAttemptListResult {
            results: attempts.into_iter().map(|attempt| attempt.into()).collect(),
        })
    }

    pub async fn advance(
        &self,
        cmd: AssessmentAttemptUpdate,
    ) -> Result<AssessmentAttemptResult, AppError> {
        let attempt = self
            .repository
            .find_by_id(cmd.id)
            .await?
            .ok_or_else(|| AppError::NotFound("Assessment attempt not found".into()))?;

        if attempt.is_mastered {
            return Err(AppError::BadRequest(
                "Cannot advance a mastered assessment attempt".into(),
            ));
        }

        let current_solo_update_id = if let Some(current_solo_id) = cmd.current_solo_level_id {
            current_solo_id
        } else {
            attempt.current_solo_level_id
        };

        let solo_levels = self
            .solo_level_repository
            .find_by_ids(vec![
                current_solo_update_id,
                attempt.current_solo_level_id,
                attempt.target_solo_level_id,
            ])
            .await?;

        let new_level = solo_levels
            .iter()
            .find(|s| s.id == current_solo_update_id)
            .ok_or_else(|| AppError::NotFound("SOLO level not found".into()))?;

        let current_level = solo_levels
            .iter()
            .find(|s| s.id == attempt.current_solo_level_id)
            .ok_or_else(|| AppError::NotFound("SOLO level not found".into()))?;

        let target_level = solo_levels
            .iter()
            .find(|s| s.id == attempt.target_solo_level_id)
            .ok_or_else(|| AppError::NotFound("SOLO level not found".into()))?;

        if new_level.order_index < current_level.order_index {
            return Err(AppError::BadRequest("SOLO level cannot decrease.".into()));
        }

        let mastered = new_level.order_index >= target_level.order_index;
        let now = chrono::Utc::now();

        let update = AssessmentAttemptUpdate {
            id: attempt.id,
            current_solo_level_id: Some(new_level.id),
            status: if mastered {
                Some(AttemptStatusEnum::Completed)
            } else {
                Some(AttemptStatusEnum::InProgress)
            },
            completed_at: if mastered {
                Some(attempt.completed_at.or(Some(now.into())))
            } else {
                None
            },
            is_mastered: Some(mastered),
        };

        let attempt = self.repository.update(update).await?;
        Ok(attempt.into())
    }

    pub async fn abandon(&self, attempt_id: Uuid) -> Result<AssessmentAttemptResult, AppError> {
        let attempt = self
            .repository
            .find_by_id(attempt_id)
            .await?
            .ok_or_else(|| AppError::NotFound("Assessment attempt not found".into()))?;

        if attempt.is_mastered {
            return Err(AppError::BadRequest(
                "Cannot abandon a completed assessment attempt".into(),
            ));
        }

        if attempt.status == AttemptStatusEnum::Abandoned {
            return Err(AppError::BadRequest(
                "Assessment attempt already abandoned.".into(),
            ));
        }

        let update = AssessmentAttemptUpdate {
            id: attempt.id,
            current_solo_level_id: Some(attempt.current_solo_level_id),
            status: Some(AttemptStatusEnum::Abandoned),
            completed_at: Some(Some(chrono::Utc::now().into())),
            is_mastered: None,
        };

        let attempt = self.repository.update(update).await?;
        Ok(attempt.into())
    }
}
