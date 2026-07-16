use std::sync::Arc;

use garde::Validate;
use uuid::Uuid;

use crate::{
    dto::question::{
        create_question_type_request::CreateQuestionTypeRequest,
        question_type_response::{QuestionTypeListResponse, QuestionTypeResponse},
        update_question_type_request::UpdateQuestionTypeRequest,
    },
    errors::app_error::AppError,
    repositories::question_type_repository::QuestionTypeRepository,
};

pub struct QuestionTypeService {
    repository: Arc<QuestionTypeRepository>,
}

impl QuestionTypeService {
    pub fn new(repository: Arc<QuestionTypeRepository>) -> Self {
        Self { repository }
    }

    pub async fn create(
        &self,
        qt: CreateQuestionTypeRequest,
    ) -> Result<QuestionTypeResponse, AppError> {
        qt.validate()?;
        let qt = self.repository.create(qt).await?;
        Ok(qt.into())
    }

    pub async fn get_by_id(&self, id: Uuid) -> Result<QuestionTypeResponse, AppError> {
        let qt = self.repository.find_by_id(id).await?;
        let qt = qt
            .ok_or_else(|| AppError::NotFound(format!("Question Type with id {} not found", id)))?;
        Ok(qt.into())
    }

    pub async fn get_by_code(&self, code: &str) -> Result<QuestionTypeResponse, AppError> {
        let qt = self.repository.find_by_code(code).await?;
        let qt = qt.ok_or_else(|| {
            AppError::NotFound(format!("Question Type with code {} not found", code))
        })?;
        Ok(qt.into())
    }

    pub async fn get_all(&self) -> Result<QuestionTypeListResponse, AppError> {
        let qts = self.repository.find_all().await?;
        Ok(QuestionTypeListResponse {
            responses: qts.into_iter().map(|qt| qt.into()).collect(),
        })
    }

    pub async fn update(
        &self,
        qt: UpdateQuestionTypeRequest,
    ) -> Result<QuestionTypeResponse, AppError> {
        qt.validate()?;
        let qt = self.repository.update(qt).await?;
        Ok(qt.into())
    }

    pub async fn search_by_code(
        &self,
        query: &str,
        limit: u64,
    ) -> Result<QuestionTypeListResponse, AppError> {
        let qts = self.repository.search_by_code(query, limit).await?;
        Ok(QuestionTypeListResponse {
            responses: qts.into_iter().map(|qt| qt.into()).collect(),
        })
    }

    pub async fn search_by_name(
        &self,
        query: &str,
        limit: u64,
    ) -> Result<QuestionTypeListResponse, AppError> {
        let qts = self.repository.search_by_name(query, limit).await?;
        Ok(QuestionTypeListResponse {
            responses: qts.into_iter().map(|qt| qt.into()).collect(),
        })
    }

    pub async fn delete(&self, id: Uuid) -> Result<(), AppError> {
        self.repository.delete(id).await?;
        Ok(())
    }

    pub async fn activate(&self, id: Uuid) -> Result<QuestionTypeResponse, AppError> {
        let qt = self.repository.activate(id).await?;
        Ok(qt.into())
    }

    pub async fn deactivate(&self, id: Uuid) -> Result<QuestionTypeResponse, AppError> {
        let qt = self.repository.deactivate(id).await?;
        Ok(qt.into())
    }
}
