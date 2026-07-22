use std::sync::Arc;

use uuid::Uuid;

use crate::application::app_error::AppError;
use crate::application::learning::question::results::question_option_result::QuestionOptionListResult;
use crate::application::learning::question::results::question_option_result::QuestionOptionResult;
use crate::domain::entities::question_option::QuestionOptionNew;
use crate::domain::entities::question_option::QuestionOptionUpdate;
use crate::infrastructure::database::repositories::pg_question_option_repository::PgQuestionOptionRepository;

pub struct QuestionOptionService {
    repository: Arc<PgQuestionOptionRepository>,
}

impl QuestionOptionService {
    pub fn new(repository: Arc<PgQuestionOptionRepository>) -> Self {
        Self { repository }
    }

    pub async fn create(&self, co: QuestionOptionNew) -> Result<QuestionOptionResult, AppError> {
        let co = self.repository.create(co).await?;
        Ok(co.into())
    }

    pub async fn get_by_id(&self, id: Uuid) -> Result<QuestionOptionResult, AppError> {
        let co = self.repository.find_by_id(id).await?;
        let co = co.ok_or_else(|| {
            AppError::NotFound(format!("QuestionOption with id {} not found", id))
        })?;
        Ok(co.into())
    }

    pub async fn get_by_question(
        &self,
        question_id: Uuid,
    ) -> Result<QuestionOptionListResult, AppError> {
        let cos = self.repository.find_by_question(question_id).await?;
        Ok(QuestionOptionListResult {
            results: cos.into_iter().map(|co| co.into()).collect(),
        })
    }

    pub async fn update(&self, co: QuestionOptionUpdate) -> Result<QuestionOptionResult, AppError> {
        let co = self.repository.update(co).await?;
        Ok(co.into())
    }

    pub async fn delete(&self, id: Uuid) -> Result<(), AppError> {
        self.repository.delete(id).await?;
        Ok(())
    }
}
