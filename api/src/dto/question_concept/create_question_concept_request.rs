use entity::{question_concepts, sea_orm_active_enums::QuestionPurposeEnum};
use garde::Validate;
use sea_orm::{ActiveValue::Set, IntoActiveModel};
use serde::Deserialize;
use uuid::Uuid;

#[derive(Debug, Deserialize, Validate)]
pub struct CreateQuestionConceptRequest {
    #[garde(skip)]
    pub question_id: Uuid,
    #[garde(skip)]
    pub concept_id: Uuid,
    #[garde(skip)]
    pub solo_level_id: Uuid,
    #[garde(skip)]
    pub purpose: QuestionPurposeEnum,
    #[garde(skip)]
    pub is_primary: bool,
    #[garde(range(min = 1))]
    pub display_order: i32,
}

impl IntoActiveModel<question_concepts::ActiveModel> for CreateQuestionConceptRequest {
    fn into_active_model(self) -> question_concepts::ActiveModel {
        question_concepts::ActiveModel {
            question_id: Set(self.question_id),
            concept_id: Set(self.concept_id),
            solo_level_id: Set(self.solo_level_id),
            purpose: Set(self.purpose),
            is_primary: Set(self.is_primary),
            display_order: Set(self.display_order),
            ..Default::default()
        }
    }
}
