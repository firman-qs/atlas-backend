use entity::learning_objectives;
use sea_orm::IntoActiveModel;
use uuid::Uuid;

use crate::dto::learning_objective::{
    self, update_learning_objective_request::UpdateLearningObjectiveRequest,
};

pub struct UpdateLo {
    pub id: Uuid,
    pub code: Option<String>,
    pub title: Option<String>,
    pub description: Option<Option<String>>,
    pub display_order: Option<i32>,
}

impl From<UpdateLearningObjectiveRequest> for UpdateLo {
    fn from(request: UpdateLearningObjectiveRequest) -> Self {
        Self {
            id: request.id,
            code: request.code,
            title: request.title,
            description: request.description,
            display_order: request.display_order,
        }
    }
}

impl IntoActiveModel<learning_objectives::ActiveModel> for UpdateLo {
    fn into_active_model(self) -> learning_objectives::ActiveModel {
        let mut active_model = learning_objectives::ActiveModel {
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

        if let Some(display_order) = self.display_order {
            active_model.display_order = sea_orm::ActiveValue::Set(display_order);
        }

        active_model.updated_at = sea_orm::ActiveValue::Set(chrono::Utc::now().into());
        active_model
    }
}
