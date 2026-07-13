use std::sync::Arc;

use uuid::Uuid;

use crate::{
    dto::concept::{
        concept_response::ConceptResponse, create_concept_request::CreateConceptRequest,
        update_concept_request::UpdateConceptRequest,
    },
    errors::app_error::AppError,
    models::concept::create_concept::CreateConcept,
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

    pub async fn update(&self, concept: UpdateConceptRequest) -> Result<ConceptResponse, AppError> {
        Ok(self.concept_repository.update(concept.into()).await?.into())
    }

    pub async fn archive(&self, id: Uuid) -> Result<ConceptResponse, AppError> {
        Ok(self.concept_repository.archive(id).await?.into())
    }

    pub async fn unarchive(&self, id: Uuid) -> Result<ConceptResponse, AppError> {
        Ok(self.concept_repository.unarchive(id).await?.into())
    }

    pub async fn delete(&self, id: Uuid) -> Result<(), AppError> {
        self.concept_repository.delete(id).await?;
        Ok(())
    }
}
