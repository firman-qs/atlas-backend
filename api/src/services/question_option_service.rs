use std::sync::Arc;

use uuid::Uuid;

use crate::{
    dto::question::{
        create_question_option_request::CreateQuestionOptionRequest,
        question_option_response::{QuestionOptionListResponse, QuestionOptionResponse},
        update_question_option_request::UpdateQuestionOptionRequest,
    },
    errors::app_error::AppError,
    repositories::question_option_repository::QuestionOptionRepository,
};

pub struct QuestionOptionService {
    repository: Arc<QuestionOptionRepository>,
}

impl QuestionOptionService {
    pub fn new(repository: Arc<QuestionOptionRepository>) -> Self {
        Self { repository }
    }

    pub async fn create(
        &self,
        co: CreateQuestionOptionRequest,
    ) -> Result<QuestionOptionResponse, AppError> {
        let co = self.repository.create(co).await?;
        Ok(co.into())
    }

    pub async fn get_by_id(&self, id: Uuid) -> Result<QuestionOptionResponse, AppError> {
        let co = self.repository.find_by_id(id).await?;
        let co = co.ok_or_else(|| {
            AppError::NotFound(format!("QuestionOption with id {} not found", id))
        })?;
        Ok(co.into())
    }

    pub async fn get_by_question(
        &self,
        question_id: Uuid,
    ) -> Result<QuestionOptionListResponse, AppError> {
        let cos = self.repository.find_by_question(question_id).await?;
        Ok(QuestionOptionListResponse {
            responses: cos.into_iter().map(|co| co.into()).collect(),
        })
    }

    pub async fn update(
        &self,
        co: UpdateQuestionOptionRequest,
    ) -> Result<QuestionOptionResponse, AppError> {
        let co = self.repository.update(co).await?;
        Ok(co.into())
    }

    pub async fn delete(&self, id: Uuid) -> Result<(), AppError> {
        self.repository.delete(id).await?;
        Ok(())
    }
}
