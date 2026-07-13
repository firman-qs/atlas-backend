use entity::courses;
use sea_orm::IntoActiveModel;
use uuid::Uuid;

use crate::dto::course::update_course_request::UpdateCourseRequest;

pub struct UpdateCourse {
    pub id: Uuid,
    pub code: Option<String>,
    pub title: Option<String>,
    pub description: Option<Option<String>>,
}

impl From<UpdateCourseRequest> for UpdateCourse {
    fn from(request: UpdateCourseRequest) -> Self {
        Self {
            id: request.id,
            code: request.code,
            title: request.title,
            description: request.description,
        }
    }
}

impl IntoActiveModel<courses::ActiveModel> for UpdateCourse {
    fn into_active_model(self) -> courses::ActiveModel {
        let mut active_model = courses::ActiveModel {
            id: sea_orm::ActiveValue::Set(self.id),
            ..Default::default()
        };

        if let Some(code) = self.code {
            active_model.code = sea_orm::ActiveValue::Set(code);
        }

        if let Some(title) = self.title {
            active_model.title = sea_orm::ActiveValue::Set(title);
        }

        if let Some(description) = self.description {
            active_model.description = sea_orm::ActiveValue::Set(description);
        }

        active_model.updated_at = sea_orm::ActiveValue::Set(chrono::Utc::now().into());
        active_model
    }
}
