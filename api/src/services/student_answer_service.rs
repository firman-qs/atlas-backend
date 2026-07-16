use std::sync::Arc;

use entity::sea_orm_active_enums::AttemptStatusEnum;
use uuid::Uuid;

use crate::{
    dto::student_answer::{
        student_answer_response::{StudentAnswerListResponse, StudentAnswerResponse},
        submit_student_answer_request::SubmitStudentAnswerRequest,
    },
    errors::app_error::AppError,
    models::student_answer::submit_student_answer::SubmitStudentAnswer,
    repositories::{
        assessment_attempt_repository::AssessmentAttemptRepository,
        question_concept_repository::QuestionConceptRepository,
        question_repository::QuestionRepository,
        student_answer_repository::StudentAnswerRepository,
    },
};

pub struct StudentAnswerService {
    repository: Arc<StudentAnswerRepository>,
    assessment_attempt_repository: Arc<AssessmentAttemptRepository>,
    question_repository: Arc<QuestionRepository>,
    question_concept_repository: Arc<QuestionConceptRepository>,
}

impl StudentAnswerService {
    pub fn new(
        repository: Arc<StudentAnswerRepository>,
        assessment_attempt_repository: Arc<AssessmentAttemptRepository>,
        question_repository: Arc<QuestionRepository>,
        question_concept_repository: Arc<QuestionConceptRepository>,
    ) -> Self {
        Self {
            repository,
            assessment_attempt_repository,
            question_repository,
            question_concept_repository,
        }
    }

    pub async fn submit(
        &self,
        request: SubmitStudentAnswerRequest,
    ) -> Result<StudentAnswerResponse, AppError> {
        if self
            .repository
            .exists(request.assessment_attempt_id, request.question_id)
            .await?
        {
            return Err(AppError::Conflict(
                "Question has already been answered in this assessment attempt.".into(),
            ));
        }

        let attempt = self
            .assessment_attempt_repository
            .find_by_id(request.assessment_attempt_id)
            .await?
            .ok_or_else(|| AppError::NotFound("Assessment attempt not found.".into()))?;

        if attempt.status != AttemptStatusEnum::InProgress {
            return Err(AppError::BadRequest(
                "Assessment attempt is no longer active.".into(),
            ));
        }

        self.question_repository
            .find_by_id(request.question_id)
            .await?
            .ok_or_else(|| AppError::NotFound("Question not found.".into()))?;

        let belongs = self
            .question_concept_repository
            .exists(request.question_id, attempt.concept_id)
            .await?;

        if !belongs {
            return Err(AppError::BadRequest(
                "Question does not belong to this concept.".into(),
            ));
        }

        // Rule 4
        self.validate_answer(&request).await?;

        let model = SubmitStudentAnswer {
            assessment_attempt_id: request.assessment_attempt_id,
            question_id: request.question_id,
            answer_text: request.answer_text,
            selected_option_id: request.selected_option_id,
            answer_json: request.answer_json,
        };

        let answer = self.repository.submit(model).await?;

        Ok(answer.into())
    }

    pub async fn get_by_id(&self, id: Uuid) -> Result<StudentAnswerResponse, AppError> {
        let answer = self
            .repository
            .find_by_id(id)
            .await?
            .ok_or_else(|| AppError::NotFound("Student answer not found.".into()))?;

        Ok(answer.into())
    }

    pub async fn get_attempt_answers(
        &self,
        assessment_attempt_id: Uuid,
    ) -> Result<StudentAnswerListResponse, AppError> {
        let answers = self
            .repository
            .find_by_attempt(assessment_attempt_id)
            .await?;

        Ok(StudentAnswerListResponse {
            responses: answers.into_iter().map(Into::into).collect(),
        })
    }

    pub async fn get_question_answers(
        &self,
        question_id: Uuid,
    ) -> Result<StudentAnswerListResponse, AppError> {
        let answers = self.repository.find_by_question(question_id).await?;

        Ok(StudentAnswerListResponse {
            responses: answers.into_iter().map(Into::into).collect(),
        })
    }

    pub async fn update_evaluation(&self) -> Result<StudentAnswerResponse, AppError> {
        todo!()
    }

    async fn validate_answer(&self, request: &SubmitStudentAnswerRequest) -> Result<(), AppError> {
        let filled = [
            request.answer_text.is_some(),
            request.selected_option_id.is_some(),
            request.answer_json.is_some(),
        ]
        .into_iter()
        .filter(|b| *b)
        .count();

        if filled == 0 {
            return Err(AppError::BadRequest("No answer was submitted.".into()));
        }

        Ok(())
    }
}
