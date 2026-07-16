use entity::{question_concepts, sea_orm_active_enums::QuestionPurposeEnum};
use garde::Validate;
use sea_orm::{ActiveValue::Set, IntoActiveModel};
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

impl IntoActiveModel<question_concepts::ActiveModel> for UpdateQuestionConceptRequest {
    fn into_active_model(self) -> question_concepts::ActiveModel {
        let mut active_model = question_concepts::ActiveModel {
            question_id: Set(self.question_id),
            concept_id: Set(self.concept_id),
            ..Default::default()
        };

        if let Some(solo_level_id) = self.solo_level_id {
            active_model.solo_level_id = Set(solo_level_id);
        }

        if let Some(purpose) = self.purpose {
            active_model.purpose = Set(purpose);
        }

        if let Some(is_primary) = self.is_primary {
            active_model.is_primary = Set(is_primary);
        }

        if let Some(display_order) = self.display_order {
            active_model.display_order = Set(display_order);
        }

        active_model
    }
}
