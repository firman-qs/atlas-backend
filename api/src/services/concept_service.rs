use std::sync::Arc;

use uuid::Uuid;

use crate::{
    dto::concept::{
        concept_response::ConceptResponse, create_concept_request::CreateConceptRequest,
        update_concept_request::UpdateConceptRequest,
    },
    errors::app_error::AppError,
    repositories::concept_repository::ConceptRepository,
};

pub struct ConceptService {
    concept_repository: Arc<ConceptRepository>,
}

impl ConceptService {
    pub fn new(concept_repository: Arc<ConceptRepository>) -> Self {
        Self { concept_repository }
    }

    pub async fn create(&self, concept: CreateConceptRequest) -> Result<ConceptResponse, AppError> {
        Ok(self.concept_repository.create(concept.into()).await?.into())
    }

    pub async fn get_by_id(&self, id: Uuid) -> Result<ConceptResponse, AppError> {
        let concept = self
            .concept_repository
            .find_by_id(id)
            .await?
            .ok_or_else(|| AppError::NotFound(format!("Concept with id {} not found", id)))?;

        Ok(concept.into())
    }

    pub async fn get_by_code(&self, code: &str) -> Result<ConceptResponse, AppError> {
        let concept = self
            .concept_repository
            .find_by_code(code)
            .await?
            .ok_or_else(|| AppError::NotFound(format!("Concept with code {} not found", code)))?;

        Ok(concept.into())
    }

    pub async fn search_by_code(
        &self,
        query: &str,
        limit: u64,
    ) -> Result<Vec<ConceptResponse>, AppError> {
        let concepts = self.concept_repository.search_by_code(query, limit).await?;
        Ok(concepts.into_iter().map(|c| c.into()).collect())
    }

    pub async fn search_by_name(
        &self,
        query: &str,
        limit: u64,
    ) -> Result<Vec<ConceptResponse>, AppError> {
        let concepts = self.concept_repository.search_by_name(query, limit).await?;
        Ok(concepts.into_iter().map(|c| c.into()).collect())
    }

    pub async fn update(&self, concept: UpdateConceptRequest) -> Result<ConceptResponse, AppError> {
        Ok(self.concept_repository.update(concept.into()).await?.into())
    }

    pub async fn deactivate(&self, id: Uuid) -> Result<ConceptResponse, AppError> {
        Ok(self.concept_repository.deactivate(id).await?.into())
    }

    pub async fn activate(&self, id: Uuid) -> Result<ConceptResponse, AppError> {
        Ok(self.concept_repository.activate(id).await?.into())
    }

    pub async fn delete(&self, id: Uuid) -> Result<(), AppError> {
        self.concept_repository.delete(id).await?;
        Ok(())
    }
}
