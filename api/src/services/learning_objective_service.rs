use std::sync::Arc;

use crate::{
    dto::learning_objective::{
        archive_learning_objective_request::ArchiveLearningObjectiveRequest,
        create_learning_objective_request::CreateLearningObjectiveRequest,
        learning_objective_list_response::LearningObjectiveListResponse,
        learning_objective_response::LearningObjectiveResponse,
        unarchive_learning_objective_request::UnarchiveLearningObjectiveRequest,
        update_learning_objective_request::UpdateLearningObjectiveRequest,
    },
    errors::app_error::AppError,
    models::learning_objective::{
        create_learning_objective::CreateLearningObjective,
        update_learning_objective::UpdateLearningObjective,
    },
    repositories::learning_objective_repository::LearningObjectiveRepository,
};

pub struct LearningObjectiveService {
    repository: Arc<LearningObjectiveRepository>,
}

impl LearningObjectiveService {
    pub fn new(repository: Arc<LearningObjectiveRepository>) -> Self {
        Self { repository }
    }

    pub async fn create(
        &self,
        lo: CreateLearningObjectiveRequest,
    ) -> Result<LearningObjectiveResponse, AppError> {
        let lo: CreateLearningObjective = lo.into();
        let res = self.repository.create(lo).await?;
        Ok(res.into())
    }

    pub async fn get_by_id(&self, id: uuid::Uuid) -> Result<LearningObjectiveResponse, AppError> {
        let lo = self.repository.find_by_id(id).await?;
        let lo = lo.ok_or_else(|| {
            AppError::NotFound(format!("Learning Objective with id {} not found", id))
        })?;
        Ok(lo.into())
    }

    pub async fn get_by_code(&self, code: &str) -> Result<LearningObjectiveResponse, AppError> {
        let lo = self.repository.find_by_code(code).await?;
        let lo = lo.ok_or_else(|| {
            AppError::NotFound(format!("Learning Objective with code {} not found", code))
        })?;
        Ok(lo.into())
    }

    pub async fn get_all(&self) -> Result<LearningObjectiveListResponse, AppError> {
        let los = self.repository.find_all().await?;
        Ok(LearningObjectiveListResponse {
            responses: los.into_iter().map(|lo| lo.into()).collect(),
        })
    }

    pub async fn get_archived_all(&self) -> Result<LearningObjectiveListResponse, AppError> {
        let los = self.repository.find_archived_all().await?;
        Ok(LearningObjectiveListResponse {
            responses: los.into_iter().map(|lo| lo.into()).collect(),
        })
    }

    pub async fn search_by_code(
        &self,
        query: &str,
        limit: u64,
    ) -> Result<LearningObjectiveListResponse, AppError> {
        let los = self.repository.search_by_code(query, limit).await?;
        Ok(LearningObjectiveListResponse {
            responses: los.into_iter().map(|lo| lo.into()).collect(),
        })
    }

    pub async fn search_by_title(
        &self,
        query: &str,
        limit: u64,
    ) -> Result<LearningObjectiveListResponse, AppError> {
        let los = self.repository.search_by_title(query, limit).await?;
        Ok(LearningObjectiveListResponse {
            responses: los.into_iter().map(|lo| lo.into()).collect(),
        })
    }

    pub async fn update(
        &self,
        lo: UpdateLearningObjectiveRequest,
    ) -> Result<LearningObjectiveResponse, AppError> {
        let lo: UpdateLearningObjective = lo.into();
        let res = self.repository.update(lo).await?;
        Ok(res.into())
    }

    pub async fn deactivate(
        &self,
        lo: ArchiveLearningObjectiveRequest,
    ) -> Result<LearningObjectiveResponse, AppError> {
        let res = self.repository.deactivate(lo.id).await?;
        Ok(res.into())
    }

    pub async fn activate(
        &self,
        lo: UnarchiveLearningObjectiveRequest,
    ) -> Result<LearningObjectiveResponse, AppError> {
        let res = self.repository.activate(lo.id).await?;
        Ok(res.into())
    }
}
