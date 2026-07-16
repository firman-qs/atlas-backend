use std::sync::Arc;

use entity::sea_orm_active_enums::AttemptStatusEnum;
use uuid::Uuid;

use crate::{
    dto::assessment_attempt::{
        assessment_attempt_response::{AssessmentAttemptListResponse, AssessmentAttemptResponse},
        start_assessment_attempt_request::StartAssessmentAttemptRequest,
        update_assessment_attempt_request::UpdateAssessmentAttemptRequest,
    },
    errors::app_error::AppError,
    models::assessment_attempt::{
        start_assessment_attempt::StartAssessmentAttempt,
        update_assessment_attempt::UpdateAssessmentAttempt,
    },
    repositories::{
        assessment_attempt_repository::AssessmentAttemptRepository,
        solo_level_repository::SoloLevelRepository,
    },
};

pub struct AssessmentAttemptService {
    repository: Arc<AssessmentAttemptRepository>,
    solo_level_repository: Arc<SoloLevelRepository>,
}

impl AssessmentAttemptService {
    pub fn new(
        repository: Arc<AssessmentAttemptRepository>,
        solo_level_repository: Arc<SoloLevelRepository>,
    ) -> Self {
        Self {
            repository,
            solo_level_repository,
        }
    }

    pub async fn start(
        &self,
        attempt: StartAssessmentAttemptRequest,
    ) -> Result<AssessmentAttemptResponse, AppError> {
        if let Some(existing_attempt) = self
            .repository
            .find_active_by_student_and_concept(attempt.student_id, attempt.concept_id)
            .await?
        {
            return Ok(existing_attempt.into());
        }

        let current = self
            .solo_level_repository
            .find_by_code("UNISTRUCTURAL")
            .await?
            .ok_or_else(|| AppError::NotFound("Missing SOLO level".into()))?;

        let target = self
            .solo_level_repository
            .find_by_code("RELATIONAL")
            .await?
            .ok_or_else(|| AppError::NotFound("Missing SOLO level".into()))?;

        let create_attempt = StartAssessmentAttempt {
            student_id: attempt.student_id,
            concept_id: attempt.concept_id,
            current_solo_level_id: current.id,
            target_solo_level_id: target.id,
        };

        let attempt = self.repository.create(create_attempt).await?;
        Ok(attempt.into())
    }

    pub async fn get_by_id(&self, id: Uuid) -> Result<AssessmentAttemptResponse, AppError> {
        let attempt = self.repository.find_by_id(id).await?;
        let attempt =
            attempt.ok_or_else(|| AppError::NotFound("Assessment attempt not found".into()))?;
        Ok(attempt.into())
    }

    pub async fn get_by_student(
        &self,
        student_id: Uuid,
    ) -> Result<AssessmentAttemptListResponse, AppError> {
        let attempts = self.repository.find_by_student(student_id).await?;
        Ok(AssessmentAttemptListResponse {
            responses: attempts.into_iter().map(|attempt| attempt.into()).collect(),
        })
    }

    pub async fn advance(
        &self,
        request: UpdateAssessmentAttemptRequest,
    ) -> Result<AssessmentAttemptResponse, AppError> {
        let attempt = self
            .repository
            .find_by_id(request.id)
            .await?
            .ok_or_else(|| AppError::NotFound("Assessment attempt not found".into()))?;

        if attempt.is_mastered {
            return Err(AppError::BadRequest(
                "Cannot advance a mastered assessment attempt".into(),
            ));
        }

        let solo_levels = self
            .solo_level_repository
            .find_by_ids([
                request.achieved_solo_level_id,
                attempt.current_solo_level_id,
                attempt.target_solo_level_id,
            ])
            .await?;

        let new_level = solo_levels
            .iter()
            .find(|s| s.id == request.achieved_solo_level_id)
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

        let update = UpdateAssessmentAttempt {
            id: attempt.id,
            current_solo_level_id: new_level.id,
            status: if mastered {
                AttemptStatusEnum::Completed
            } else {
                AttemptStatusEnum::InProgress
            },
            completed_at: if mastered {
                attempt.completed_at.or(Some(now.into()))
            } else {
                None
            },
            is_mastered: mastered,
            updated_at: now.into(),
        };

        let attempt = self.repository.update(update).await?;
        Ok(attempt.into())
    }

    pub async fn abandon(&self, attempt_id: Uuid) -> Result<AssessmentAttemptResponse, AppError> {
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

        let update = UpdateAssessmentAttempt {
            id: attempt.id,
            current_solo_level_id: attempt.current_solo_level_id,
            status: AttemptStatusEnum::Abandoned,
            completed_at: Some(chrono::Utc::now().into()),
            is_mastered: false,
            updated_at: chrono::Utc::now().into(),
        };

        let attempt = self.repository.update(update).await?;
        Ok(attempt.into())
    }
}
