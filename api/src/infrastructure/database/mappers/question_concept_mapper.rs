use sea_orm::ActiveValue::NotSet;
use sea_orm::ActiveValue::Set;
use sea_orm::IntoActiveModel;

use crate::domain::entities::question_concept::QuestionConcept;
use crate::domain::entities::question_concept::QuestionConceptUpdate;

impl From<entity::question_concepts::Model> for QuestionConcept {
    fn from(active_model: entity::question_concepts::Model) -> Self {
        Self {
            question_id: active_model.question_id,
            concept_id: active_model.concept_id,
            solo_level_id: active_model.solo_level_id,
            purpose: active_model.purpose,
            is_primary: active_model.is_primary,
            display_order: active_model.display_order,
        }
    }
}

impl IntoActiveModel<entity::question_concepts::ActiveModel> for QuestionConcept {
    fn into_active_model(self) -> entity::question_concepts::ActiveModel {
        entity::question_concepts::ActiveModel {
            question_id: Set(self.question_id),
            concept_id: Set(self.concept_id),
            solo_level_id: Set(self.solo_level_id),
            purpose: Set(self.purpose),
            is_primary: Set(self.is_primary),
            display_order: Set(self.display_order),
        }
    }
}

impl IntoActiveModel<entity::question_concepts::ActiveModel> for QuestionConceptUpdate {
    fn into_active_model(self) -> entity::question_concepts::ActiveModel {
        entity::question_concepts::ActiveModel {
            question_id: NotSet,
            concept_id: NotSet,
            solo_level_id: self.solo_level_id.map_or(NotSet, |v| Set(v)),
            purpose: self.purpose.map_or(NotSet, |v| Set(v)),
            is_primary: self.is_primary.map_or(NotSet, |v| Set(v)),
            display_order: self.display_order.map_or(NotSet, |v| Set(v)),
        }
    }
}
