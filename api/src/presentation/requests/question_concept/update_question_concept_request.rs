use entity::sea_orm_active_enums::QuestionPurposeEnum;
use garde::Validate;
use serde::Deserialize;
use uuid::Uuid;

#[derive(Debug, Deserialize, Validate)]
pub struct UpdateQuestionConceptRequest {
    #[garde(skip)]
    pub question_id: Uuid,
    #[garde(skip)]
    pub concept_id: Uuid,
    #[garde(skip)]
    pub solo_level_id: Option<Uuid>,
    #[garde(skip)]
    pub purpose: Option<QuestionPurposeEnum>,
    #[garde(skip)]
    pub is_primary: Option<bool>,
    #[garde(range(min = 1))]
    pub display_order: Option<i32>,
}
