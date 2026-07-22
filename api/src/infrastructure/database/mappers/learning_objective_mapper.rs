use sea_orm::ActiveValue::NotSet;
use sea_orm::ActiveValue::Set;
use sea_orm::IntoActiveModel;

use crate::domain::entities::learning_objective::LearningObjective;
use crate::domain::entities::learning_objective::LearningObjectiveNew;
use crate::domain::entities::learning_objective::LearningObjectiveUpdate;

impl From<entity::learning_objectives::Model> for LearningObjective {
    fn from(active_model: entity::learning_objectives::Model) -> Self {
        Self {
            id: active_model.id,
            course_id: active_model.course_id,
            code: active_model.code,
            title: active_model.title,
            description: active_model.description,
            display_order: active_model.display_order,
            is_active: active_model.is_active,
            created_at: active_model.created_at,
            updated_at: active_model.updated_at,
        }
    }
}

impl IntoActiveModel<entity::learning_objectives::ActiveModel> for LearningObjectiveNew {
    fn into_active_model(self) -> entity::learning_objectives::ActiveModel {
        entity::learning_objectives::ActiveModel {
            course_id: Set(self.course_id),
            code: Set(self.code),
            title: Set(self.title),
            description: Set(self.description),
            display_order: Set(self.display_order),
            ..Default::default()
        }
    }
}

impl IntoActiveModel<entity::learning_objectives::ActiveModel> for LearningObjectiveUpdate {
    fn into_active_model(self) -> entity::learning_objectives::ActiveModel {
        entity::learning_objectives::ActiveModel {
            id: Set(self.id),
            course_id: self.course_id.map_or(NotSet, Set),
            code: self.code.map_or(NotSet, Set),
            title: self.title.map_or(NotSet, Set),
            description: self.description.map_or(NotSet, Set),
            display_order: self.display_order.map_or(NotSet, Set),
            is_active: self.is_active.map_or(NotSet, Set),
            ..Default::default()
        }
    }
}
