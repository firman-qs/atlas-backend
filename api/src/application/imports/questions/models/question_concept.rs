use entity::sea_orm_active_enums::QuestionPurposeEnum;
use serde::Deserialize;
use uuid::Uuid;

use crate::domain::entities::question_concept::QuestionConceptNew;
use crate::domain::entities::solo_level::SoloLevelCode;

#[derive(Clone, Debug, Deserialize, PartialEq, Eq, Hash)]
pub struct ImportQuestionConcept
{
    pub code: String,
    pub solo_level_code: SoloLevelCode,
}

impl ImportQuestionConcept
{
    pub fn into_new(
        &self,
        question_id: Uuid,
        concept_id: Uuid,
        solo_level_id: Uuid,
        display_order: i32,
    ) -> QuestionConceptNew
    {
        QuestionConceptNew {
            question_id,
            concept_id,
            solo_level_id,
            display_order,
            is_primary: true,
            purpose: QuestionPurposeEnum::Assessment,
        }
    }
}
