use entity::learning_objectives;
use sea_orm::IntoActiveModel;
use uuid::Uuid;

use crate::dto::learning_objective::create_lo_request::CreateLoRequest;

pub struct CreateLo {
    course_id: Uuid,
    code: String,
    title: String,
    description: Option<String>,
    display_order: i32,
}

impl From<CreateLoRequest> for CreateLo {
    fn from(request: CreateLoRequest) -> Self {
        Self {
            course_id: request.course_id,
            code: request.code,
            title: request.title,
            description: request.description,
            display_order: request.display_order,
        }
    }
}

impl IntoActiveModel<learning_objectives::ActiveModel> for CreateLo {
    fn into_active_model(self) -> learning_objectives::ActiveModel {
        learning_objectives::ActiveModel {
            course_id: sea_orm::ActiveValue::Set(self.course_id),
            code: sea_orm::ActiveValue::Set(self.code),
            title: sea_orm::ActiveValue::Set(self.title),
            description: sea_orm::ActiveValue::Set(self.description),
            display_order: sea_orm::ActiveValue::Set(self.display_order),
            ..Default::default()
        }
    }
}
