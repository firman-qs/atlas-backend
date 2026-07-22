use std::sync::Arc;

use uuid::Uuid;

use crate::application::app_error::AppError;
use crate::application::learning::concepts::results::concept_result::ConceptListResult;
use crate::application::learning::question::results::question_result::QuestionListResult;
use crate::application::learning::question_concepts::commands::create_question_concept::CreateQuestionConcept;
use crate::domain::entities::question_concept::QuestionConcept;
use crate::infrastructure::database::repositories::pg_question_concept_repository::PgQuestionConceptRepository;

pub struct QuestionConceptService {
    repository: Arc<PgQuestionConceptRepository>,
}

impl QuestionConceptService {
    pub fn new(repository: Arc<PgQuestionConceptRepository>) -> Self {
        Self { repository }
    }

    pub async fn create(&self, cmd: CreateQuestionConcept) -> Result<QuestionConcept, AppError> {
        let question_concept = self.repository.create(cmd.into()).await?;

        Ok(question_concept)
    }

    pub async fn delete(&self, question_id: Uuid, concept_id: Uuid) -> Result<(), AppError> {
        self.repository.delete(question_id, concept_id).await?;
        Ok(())
    }

    pub async fn exists(&self, question_id: Uuid, concept_id: Uuid) -> Result<bool, AppError> {
        let is_exists = self.repository.exists(question_id, concept_id).await?;

        Ok(is_exists)
    }

    pub async fn get_questions_by_concept_and_solo(
        &self,
        concept_id: Uuid,
        solo_level_id: Uuid,
    ) -> Result<QuestionListResult, AppError> {
        let questions = self
            .repository
            .find_questions_by_concept_and_solo(concept_id, solo_level_id)
            .await?;

        let result = QuestionListResult {
            results: questions.into_iter().map(|q| q.into()).collect(),
        };

        Ok(result)
    }

    pub async fn get_questions_by_concept(
        &self,
        concept_id: Uuid,
    ) -> Result<QuestionListResult, AppError> {
        let questions = self.repository.find_questions(concept_id).await?;
        let result = QuestionListResult {
            results: questions.into_iter().map(|q| q.into()).collect(),
        };

        Ok(result)
    }

    pub async fn get_concepts_by_question(
        &self,
        question_id: Uuid,
    ) -> Result<ConceptListResult, AppError> {
        let concepts = self.repository.find_concepts(question_id).await?;
        let result = ConceptListResult {
            results: concepts.into_iter().map(|concept| concept.into()).collect(),
        };

        Ok(result)
    }
}
