use entity::{question_concepts, sea_orm_active_enums::QuestionPurposeEnum};
use serde::Serialize;
use uuid::Uuid;

#[derive(Debug, Serialize)]
pub struct QuestionConceptResponse {
    pub question_id: Uuid,
    pub concept_id: Uuid,
    pub solo_level_id: Uuid,
    pub purpose: QuestionPurposeEnum,
    pub is_primary: bool,
    pub display_order: i32,
}

pub struct QuestionConceptListResponse {
    pub responses: Vec<QuestionConceptResponse>,
}

impl From<question_concepts::Model> for QuestionConceptResponse {
    fn from(model: question_concepts::Model) -> Self {
        QuestionConceptResponse {
            question_id: model.question_id,
            concept_id: model.concept_id,
            solo_level_id: model.solo_level_id,
            purpose: model.purpose,
            is_primary: model.is_primary,
            display_order: model.display_order,
        }
    }
}
