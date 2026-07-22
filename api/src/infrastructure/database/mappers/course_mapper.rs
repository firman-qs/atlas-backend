use sea_orm::ActiveValue::NotSet;
use sea_orm::ActiveValue::Set;
use sea_orm::IntoActiveModel;

use crate::domain::entities::course::Course;
use crate::domain::entities::course::CourseNew;
use crate::domain::entities::course::CourseUpdate;

impl From<entity::courses::Model> for Course {
    fn from(model: entity::courses::Model) -> Self {
        Course {
            id: model.id,
            code: model.code,
            title: model.title,
            description: model.description,
            is_active: model.is_active,
            created_at: model.created_at,
            updated_at: model.updated_at,
        }
    }
}

impl IntoActiveModel<entity::courses::ActiveModel> for CourseNew {
    fn into_active_model(self) -> entity::courses::ActiveModel {
        entity::courses::ActiveModel {
            code: sea_orm::ActiveValue::Set(self.code),
            title: sea_orm::ActiveValue::Set(self.title),
            description: sea_orm::ActiveValue::Set(self.description),
            ..Default::default()
        }
    }
}

impl IntoActiveModel<entity::courses::ActiveModel> for CourseUpdate {
    fn into_active_model(self) -> entity::courses::ActiveModel {
        entity::courses::ActiveModel {
            id: sea_orm::ActiveValue::Set(self.id),
            code: self.code.map_or(NotSet, Set),
            title: self.title.map_or(NotSet, Set),
            description: self.description.map_or(NotSet, Set),
            is_active: self.is_active.map_or(NotSet, Set),
            ..Default::default()
        }
    }
}
