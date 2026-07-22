use entity::sea_orm_active_enums::QuestionPurposeEnum;
use uuid::Uuid;

pub struct QuestionConcept
{
    pub question_id: Uuid,
    pub concept_id: Uuid,
    pub solo_level_id: Uuid,
    pub purpose: QuestionPurposeEnum,
    pub is_primary: bool,
    pub display_order: i32,
}

pub type QuestionConceptNew = QuestionConcept;

pub struct QuestionConceptUpdate
{
    pub solo_level_id: Option<Uuid>,
    pub purpose: Option<QuestionPurposeEnum>,
    pub is_primary: Option<bool>,
    pub display_order: Option<i32>,
}
