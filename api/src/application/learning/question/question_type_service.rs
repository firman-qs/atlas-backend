use std::sync::Arc;

use uuid::Uuid;

use crate::application::app_error::AppError;
use crate::application::learning::question::results::question_type_result::QuestionTypeListResult;
use crate::application::learning::question::results::question_type_result::QuestionTypeResult;
use crate::domain::entities::question_type::QuestionTypeNew;
use crate::domain::entities::question_type::QuestionTypeUpdate;
use crate::infrastructure::database::repositories::pg_question_type_repository::PgQuestionTypeRepository;

pub struct QuestionTypeService {
    repository: Arc<PgQuestionTypeRepository>,
}

impl QuestionTypeService {
    pub fn new(repository: Arc<PgQuestionTypeRepository>) -> Self {
        Self { repository }
    }

    pub async fn create(&self, qt: QuestionTypeNew) -> Result<QuestionTypeResult, AppError> {
        let qt = self.repository.create(qt).await?;
        Ok(qt.into())
    }

    pub async fn get_by_id(&self, id: Uuid) -> Result<QuestionTypeResult, AppError> {
        let qt = self.repository.find_by_id(id).await?;
        let qt = qt
            .ok_or_else(|| AppError::NotFound(format!("Question Type with id {} not found", id)))?;
        Ok(qt.into())
    }

    pub async fn get_by_code(&self, code: &str) -> Result<QuestionTypeResult, AppError> {
        let qt = self.repository.find_by_code(code).await?;
        let qt = qt.ok_or_else(|| {
            AppError::NotFound(format!("Question Type with code {} not found", code))
        })?;
        Ok(qt.into())
    }

    pub async fn get_all(&self) -> Result<QuestionTypeListResult, AppError> {
        let qts = self.repository.find_all().await?;
        Ok(QuestionTypeListResult {
            results: qts.into_iter().map(|qt| qt.into()).collect(),
        })
    }

    pub async fn update(&self, qt: QuestionTypeUpdate) -> Result<QuestionTypeResult, AppError> {
        let qt = self.repository.update(qt).await?;
        Ok(qt.into())
    }

    pub async fn search_by_code(
        &self,
        query: &str,
        limit: u64,
    ) -> Result<QuestionTypeListResult, AppError> {
        let qts = self.repository.search_by_code(query, limit).await?;
        Ok(QuestionTypeListResult {
            results: qts.into_iter().map(|qt| qt.into()).collect(),
        })
    }

    pub async fn search_by_name(
        &self,
        query: &str,
        limit: u64,
    ) -> Result<QuestionTypeListResult, AppError> {
        let qts = self.repository.search_by_name(query, limit).await?;
        Ok(QuestionTypeListResult {
            results: qts.into_iter().map(|qt| qt.into()).collect(),
        })
    }

    pub async fn delete(&self, id: Uuid) -> Result<(), AppError> {
        self.repository.delete(id).await?;
        Ok(())
    }

    pub async fn activate(&self, id: Uuid) -> Result<QuestionTypeResult, AppError> {
        let qt = self.repository.activate(id).await?;
        Ok(qt.into())
    }

    pub async fn deactivate(&self, id: Uuid) -> Result<QuestionTypeResult, AppError> {
        let qt = self.repository.deactivate(id).await?;
        Ok(qt.into())
    }
}
