use entity::sea_orm_active_enums::QuestionPurposeEnum;
use serde::Serialize;
use utoipa::ToSchema;
use uuid::Uuid;

use crate::domain::entities::question_concept::QuestionConcept;

#[derive(Debug, Serialize, ToSchema)]
pub struct QuestionConceptResult
{
    pub question_id: Uuid,
    pub concept_id: Uuid,
    pub solo_level_id: Uuid,
    pub purpose: QuestionPurposeEnum,
    pub is_primary: bool,
    pub display_order: i32,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct QuestionConceptListResult
{
    pub responses: Vec<QuestionConceptResult>,
}

impl From<QuestionConcept> for QuestionConceptResult
{
    fn from(question_concept: QuestionConcept) -> Self
    {
        Self {
            question_id: question_concept.question_id,
            concept_id: question_concept.concept_id,
            solo_level_id: question_concept.solo_level_id,
            purpose: question_concept.purpose,
            is_primary: question_concept.is_primary,
            display_order: question_concept.display_order,
        }
    }
}
