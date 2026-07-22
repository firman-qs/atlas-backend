use entity::sea_orm_active_enums::QuestionPurposeEnum;
use garde::Validate;
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
