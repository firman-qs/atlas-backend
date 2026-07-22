use entity::sea_orm_active_enums::QuestionPurposeEnum;
use uuid::Uuid;

pub struct CreateQuestionConcept {
    pub question_id: Uuid,
    pub concept_id: Uuid,
    pub solo_level_id: Uuid,
    pub purpose: QuestionPurposeEnum,
    pub is_primary: bool,
    pub display_order: i32,
}
