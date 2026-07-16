use std::sync::Arc;

use uuid::Uuid;

use crate::{
    dto::{
        concept::concept_response::ConceptListResponse,
        question::question_response::QuestionListResponse,
        question_concept::create_question_concept_request::CreateQuestionConceptRequest,
    },
    errors::app_error::AppError,
    repositories::question_concept_repository::QuestionConceptRepository,
};

pub struct QuestionConceptService {
    repository: Arc<QuestionConceptRepository>,
}

impl QuestionConceptService {
    pub fn new(repository: Arc<QuestionConceptRepository>) -> Self {
        Self { repository }
    }

    pub async fn create(&self, qc: CreateQuestionConceptRequest) -> Result<(), AppError> {
        self.repository.create(qc.into()).await?;
        Ok(())
    }

    pub async fn delete(&self, question_id: Uuid, concept_id: Uuid) -> Result<(), AppError> {
        self.repository.delete(question_id, concept_id).await?;
        Ok(())
    }

    pub async fn exists(&self, question_id: Uuid, concept_id: Uuid) -> Result<bool, AppError> {
        let exists = self.repository.exists(question_id, concept_id).await?;
        Ok(exists)
    }

    pub async fn get_questions_by_concept_and_solo(
        &self,
        concept_id: Uuid,
        solo_level_id: Uuid,
    ) -> Result<QuestionListResponse, AppError> {
        let questions = self
            .repository
            .find_questions_by_concept_and_solo(concept_id, solo_level_id)
            .await?;

        Ok(QuestionListResponse {
            responses: questions
                .into_iter()
                .map(|question| question.into())
                .collect(),
        })
    }

    pub async fn get_questions_by_concept(
        &self,
        concept_id: Uuid,
    ) -> Result<QuestionListResponse, AppError> {
        let questions = self.repository.find_questions(concept_id).await?;

        Ok(QuestionListResponse {
            responses: questions
                .into_iter()
                .map(|question| question.into())
                .collect(),
        })
    }

    pub async fn get_concepts_by_question(
        &self,
        question_id: Uuid,
    ) -> Result<ConceptListResponse, AppError> {
        let concepts = self.repository.find_concepts(question_id).await?;

        Ok(ConceptListResponse {
            responses: concepts.into_iter().map(|concept| concept.into()).collect(),
        })
    }
}
