use sea_orm::ActiveValue::NotSet;
use sea_orm::ActiveValue::Set;
use sea_orm::IntoActiveModel;

use crate::domain::entities::assessment_attempt::AssessmentAttempt;
use crate::domain::entities::assessment_attempt::AssessmentAttemptNew;
use crate::domain::entities::assessment_attempt::AssessmentAttemptUpdate;

impl From<entity::assessment_attempts::Model> for AssessmentAttempt {
    fn from(model: entity::assessment_attempts::Model) -> Self {
        AssessmentAttempt {
            id: model.id,
            student_id: model.student_id,
            concept_id: model.concept_id,
            started_at: model.started_at,
            completed_at: model.completed_at,
            status: model.status,
            current_solo_level_id: model.current_solo_level_id,
            target_solo_level_id: model.target_solo_level_id,
            is_mastered: model.is_mastered,
            created_at: model.created_at,
            updated_at: model.updated_at,
        }
    }
}

impl IntoActiveModel<entity::assessment_attempts::ActiveModel> for AssessmentAttemptNew {
    fn into_active_model(self) -> entity::assessment_attempts::ActiveModel {
        entity::assessment_attempts::ActiveModel {
            student_id: sea_orm::ActiveValue::Set(self.student_id),
            concept_id: sea_orm::ActiveValue::Set(self.concept_id),
            started_at: sea_orm::ActiveValue::Set(self.started_at),
            current_solo_level_id: sea_orm::ActiveValue::Set(self.current_solo_level_id),
            target_solo_level_id: sea_orm::ActiveValue::Set(self.target_solo_level_id),
            ..Default::default()
        }
    }
}

impl IntoActiveModel<entity::assessment_attempts::ActiveModel> for AssessmentAttemptUpdate {
    fn into_active_model(self) -> entity::assessment_attempts::ActiveModel {
        entity::assessment_attempts::ActiveModel {
            id: Set(self.id),
            completed_at: self.completed_at.map_or(NotSet, Set),
            status: self.status.map_or(NotSet, Set),
            current_solo_level_id: self.current_solo_level_id.map_or(NotSet, Set),
            is_mastered: self.is_mastered.map_or(NotSet, Set),
            ..Default::default()
        }
    }
}
