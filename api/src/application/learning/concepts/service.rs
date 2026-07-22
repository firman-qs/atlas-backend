use std::sync::Arc;

use uuid::Uuid;

use crate::application::app_error::AppError;
use crate::application::learning::concepts::commands::create_concept::CreateConcept;
use crate::application::learning::concepts::commands::update_concept::UpdateConcept;
use crate::application::learning::concepts::results::concept_result::ConceptListResult;
use crate::application::learning::concepts::results::concept_result::ConceptResult;
use crate::infrastructure::database::repositories::pg_concept_repository::PgConceptRepository;

pub struct ConceptService {
    repository: Arc<PgConceptRepository>,
}

impl ConceptService {
    pub fn new(repository: Arc<PgConceptRepository>) -> Self {
        Self { repository }
    }

    pub async fn create(&self, cmd: CreateConcept) -> Result<ConceptResult, AppError> {
        Ok(self.repository.create(cmd.into()).await?.into())
    }

    pub async fn get_by_id(&self, id: Uuid) -> Result<ConceptResult, AppError> {
        let concept = self
            .repository
            .find_by_id(id)
            .await?
            .ok_or_else(|| AppError::NotFound(format!("Concept with id {} not found", id)))?;

        Ok(concept.into())
    }

    pub async fn get_by_code(&self, code: &str) -> Result<ConceptResult, AppError> {
        let concept =
            self.repository.find_by_code(code).await?.ok_or_else(|| {
                AppError::NotFound(format!("Concept with code {} not found", code))
            })?;

        Ok(concept.into())
    }

    pub async fn search_by_code(
        &self,
        query: &str,
        limit: u64,
    ) -> Result<ConceptListResult, AppError> {
        let concepts = self.repository.search_by_code(query, limit).await?;

        Ok(ConceptListResult {
            results: concepts.into_iter().map(|c| c.into()).collect(),
        })
    }

    pub async fn search_by_name(
        &self,
        query: &str,
        limit: u64,
    ) -> Result<ConceptListResult, AppError> {
        let concepts = self.repository.search_by_name(query, limit).await?;

        Ok(ConceptListResult {
            results: concepts.into_iter().map(|c| c.into()).collect(),
        })
    }

    pub async fn update(&self, concept: UpdateConcept) -> Result<ConceptResult, AppError> {
        Ok(self.repository.update(concept.into()).await?.into())
    }

    pub async fn deactivate(&self, id: Uuid) -> Result<ConceptResult, AppError> {
        Ok(self.repository.deactivate(id).await?.into())
    }

    pub async fn activate(&self, id: Uuid) -> Result<ConceptResult, AppError> {
        Ok(self.repository.activate(id).await?.into())
    }

    pub async fn delete(&self, id: Uuid) -> Result<(), AppError> {
        self.repository.delete(id).await?;
        Ok(())
    }
}
