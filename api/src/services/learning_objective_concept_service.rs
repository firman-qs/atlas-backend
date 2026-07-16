use std::sync::Arc;

use crate::{
    dto::{
        concept::concept_response::ConceptListResponse,
        learning_objective::learning_objective_list_response::LearningObjectiveListResponse,
        learning_objective_concept::{
            create_learning_objective_concept_request::CreateLearningObjectiveConceptRequest,
            delete_learning_objective_concept_request::DeleteLearningObjectiveConceptRequest,
        },
    },
    errors::app_error::AppError,
    repositories::learning_objective_concept_repository::LearningObjectiveConceptRepository,
};

pub struct LearningObjectiveConceptService {
    repository: Arc<LearningObjectiveConceptRepository>,
}

impl LearningObjectiveConceptService {
    pub fn new(repository: Arc<LearningObjectiveConceptRepository>) -> Self {
        Self { repository }
    }

    pub async fn create(
        &self,
        lo_concept: CreateLearningObjectiveConceptRequest,
    ) -> Result<(), AppError> {
        self.repository.create(lo_concept.into()).await?;
        Ok(())
    }

    pub async fn delete(
        &self,
        lo_concept: DeleteLearningObjectiveConceptRequest,
    ) -> Result<(), AppError> {
        self.repository.delete(lo_concept.into()).await?;
        Ok(())
    }

    pub async fn get_concepts_by_lo(
        &self,
        lo_id: uuid::Uuid,
    ) -> Result<ConceptListResponse, AppError> {
        let concepts = self.repository.find_concepts(lo_id).await?;
        Ok(ConceptListResponse {
            responses: concepts.into_iter().map(|concept| concept.into()).collect(),
        })
    }

    pub async fn get_lo_by_concept(
        &self,
        concept_id: uuid::Uuid,
    ) -> Result<LearningObjectiveListResponse, AppError> {
        let los = self.repository.find_learning_objectives(concept_id).await?;
        Ok(LearningObjectiveListResponse {
            responses: los.into_iter().map(|lo| lo.into()).collect(),
        })
    }
}
