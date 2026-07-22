use entity::sea_orm_active_enums::QuestionPurposeEnum;
use uuid::Uuid;

pub struct UpdateQuestionConcept {
    pub question_id: Uuid,
    pub concept_id: Uuid,
    pub solo_level_id: Option<Uuid>,
    pub purpose: Option<QuestionPurposeEnum>,
    pub is_primary: Option<bool>,
    pub display_order: Option<i32>,
}
