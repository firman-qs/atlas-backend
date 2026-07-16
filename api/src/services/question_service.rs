use std::sync::Arc;

use garde::Validate;
use uuid::Uuid;

use crate::{
    dto::question::{
        create_question_request::CreateQuestionRequest,
        question_response::{QuestionListResponse, QuestionResponse},
        update_question_request::UpdateQuestionRequest,
    },
    errors::app_error::AppError,
    repositories::question_repository::QuestionRepository,
};

pub struct QuestionService {
    repository: Arc<QuestionRepository>,
}

impl QuestionService {
    pub fn new(repository: Arc<QuestionRepository>) -> Self {
        Self { repository }
    }

    pub async fn create(
        &self,
        question: CreateQuestionRequest,
    ) -> Result<QuestionResponse, AppError> {
        question.validate()?;
        let question = self.repository.create(question).await?;
        Ok(question.into())
    }

    pub async fn get_by_id(&self, id: Uuid) -> Result<QuestionResponse, AppError> {
        let question = self.repository.find_by_id(id).await?;
        let question = question
            .ok_or_else(|| AppError::NotFound(format!("Question with id {} not found", id)))?;
        Ok(question.into())
    }

    pub async fn get_by_creator_id(
        &self,
        creator_id: Uuid,
    ) -> Result<QuestionListResponse, AppError> {
        let questions = self.repository.find_by_creator_id(creator_id).await?;
        Ok(QuestionListResponse {
            responses: questions.into_iter().map(|q| q.into()).collect(),
        })
    }

    pub async fn get_all(&self) -> Result<QuestionListResponse, AppError> {
        let questions = self.repository.find_all().await?;
        Ok(QuestionListResponse {
            responses: questions.into_iter().map(|q| q.into()).collect(),
        })
    }

    pub async fn search_by_title(
        &self,
        query: &str,
        limit: u64,
    ) -> Result<QuestionListResponse, AppError> {
        let questions = self.repository.search_by_title(query, limit).await?;
        Ok(QuestionListResponse {
            responses: questions.into_iter().map(|q| q.into()).collect(),
        })
    }

    pub async fn search_by_text(
        &self,
        query: &str,
        limit: u64,
    ) -> Result<QuestionListResponse, AppError> {
        let questions = self.repository.search_by_text(query, limit).await?;
        Ok(QuestionListResponse {
            responses: questions.into_iter().map(|q| q.into()).collect(),
        })
    }

    pub async fn update(
        &self,
        question: UpdateQuestionRequest,
    ) -> Result<QuestionResponse, AppError> {
        question.validate()?;
        let question = self.repository.update(question).await?;
        Ok(question.into())
    }

    pub async fn deactivate(&self, id: Uuid) -> Result<QuestionResponse, AppError> {
        let question = self.repository.deactivate(id).await?;
        Ok(question.into())
    }

    pub async fn activate(&self, id: Uuid) -> Result<QuestionResponse, AppError> {
        let question = self.repository.activate(id).await?;
        Ok(question.into())
    }
}
