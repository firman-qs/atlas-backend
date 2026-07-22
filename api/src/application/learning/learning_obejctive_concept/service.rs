use crate::{
    application::{
        app_error::AppError,
        learning::{
            concepts::results::concept_result::ConceptListResult,
            learning_obejctive_concept::commands::{
                create_learning_objective_concept::CreateLearningObjectiveConcept,
                delete_learning_objective_concept::DeleteLearningObjectiveConcept,
            },
            learning_objective::results::learning_objective_result::LearningObjectiveListResult,
        },
    },
    domain::entities::learning_objective_concept::LearningObjectiveConcept,
    infrastructure::database::repositories::pg_learning_objective_concept_repository::PgLearningObjectiveConceptRepository,
};
use std::sync::Arc;
use uuid::Uuid;

pub struct LearningObjectiveConceptService {
    repository: Arc<PgLearningObjectiveConceptRepository>,
}

impl LearningObjectiveConceptService {
    pub fn new(repository: Arc<PgLearningObjectiveConceptRepository>) -> Self {
        Self { repository }
    }

    pub async fn create(
        &self,
        cmd: CreateLearningObjectiveConcept,
    ) -> Result<LearningObjectiveConcept, AppError> {
        let loc = self.repository.create(cmd.into()).await?;
        Ok(loc)
    }

    pub async fn delete(&self, cmd: DeleteLearningObjectiveConcept) -> Result<(), AppError> {
        self.repository
            .delete(cmd.learning_objective_id, cmd.concept_id)
            .await?;
        Ok(())
    }

    pub async fn get_concepts_by_learning_objective(
        &self,
        lo_id: Uuid,
    ) -> Result<ConceptListResult, AppError> {
        let concepts = self.repository.find_concepts(lo_id).await?;

        Ok(ConceptListResult {
            results: concepts.into_iter().map(|concept| concept.into()).collect(),
        })
    }

    pub async fn get_learning_objective_by_concept(
        &self,
        concept_id: Uuid,
    ) -> Result<LearningObjectiveListResult, AppError> {
        let los = self.repository.find_learning_objectives(concept_id).await?;

        Ok(LearningObjectiveListResult {
            results: los.into_iter().map(|lo| lo.into()).collect(),
        })
    }
}
