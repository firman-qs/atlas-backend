use std::sync::Arc;

use uuid::Uuid;

use crate::application::app_error::AppError;
use crate::application::learning::question::results::question_result::QuestionListResult;
use crate::application::learning::question::results::question_result::QuestionResult;
use crate::domain::entities::question::QuestionNew;
use crate::domain::entities::question::QuestionUpdate;
use crate::infrastructure::database::repositories::pg_question_repository::PgQuestionRepository;

pub struct QuestionService {
    repository: Arc<PgQuestionRepository>,
}

impl QuestionService {
    pub fn new(repository: Arc<PgQuestionRepository>) -> Self {
        Self { repository }
    }

    pub async fn create(&self, question: QuestionNew) -> Result<QuestionResult, AppError> {
        let question = self.repository.create(question).await?;

        Ok(question.into())
    }

    pub async fn get_by_id(&self, id: Uuid) -> Result<QuestionResult, AppError> {
        let question = self.repository.find_by_id(id).await?;
        let question = question
            .ok_or_else(|| AppError::NotFound(format!("Question with id {} not found", id)))?;

        Ok(question.into())
    }

    pub async fn get_by_creator_id(
        &self,
        creator_id: Uuid,
    ) -> Result<QuestionListResult, AppError> {
        let questions = self.repository.find_by_creator_id(creator_id).await?;

        Ok(QuestionListResult {
            results: questions.into_iter().map(|q| q.into()).collect(),
        })
    }

    pub async fn get_all(&self) -> Result<QuestionListResult, AppError> {
        let questions = self.repository.find_all().await?;

        Ok(QuestionListResult {
            results: questions.into_iter().map(|q| q.into()).collect(),
        })
    }

    pub async fn search_by_title(
        &self,
        query: &str,
        limit: u64,
    ) -> Result<QuestionListResult, AppError> {
        let questions = self.repository.search_by_title(query, limit).await?;

        Ok(QuestionListResult {
            results: questions.into_iter().map(|q| q.into()).collect(),
        })
    }

    pub async fn search_by_text(
        &self,
        query: &str,
        limit: u64,
    ) -> Result<QuestionListResult, AppError> {
        let questions = self.repository.search_by_text(query, limit).await?;

        Ok(QuestionListResult {
            results: questions.into_iter().map(|q| q.into()).collect(),
        })
    }

    pub async fn update(&self, question: QuestionUpdate) -> Result<QuestionResult, AppError> {
        let question = self.repository.update(question).await?;

        Ok(question.into())
    }

    pub async fn deactivate(&self, id: Uuid) -> Result<QuestionResult, AppError> {
        let question = self.repository.deactivate(id).await?;

        Ok(question.into())
    }

    pub async fn activate(&self, id: Uuid) -> Result<QuestionResult, AppError> {
        let question = self.repository.activate(id).await?;

        Ok(question.into())
    }
}
