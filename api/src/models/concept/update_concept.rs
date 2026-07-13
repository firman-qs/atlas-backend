use entity::concepts;
use sea_orm::IntoActiveModel;
use uuid::Uuid;

use crate::dto::concept::update_concept_request::UpdateConceptRequest;

pub struct UpdateConcept {
    pub id: Uuid,
    pub code: Option<String>,
    pub name: Option<String>,
    pub description: Option<Option<String>>,
}

impl From<UpdateConceptRequest> for UpdateConcept {
    fn from(request: UpdateConceptRequest) -> Self {
        Self {
            id: request.id,
            code: request.code,
            name: request.name,
            description: request.description,
        }
    }
}

impl IntoActiveModel<concepts::ActiveModel> for UpdateConcept {
    fn into_active_model(self) -> concepts::ActiveModel {
        let mut active_model = concepts::ActiveModel {
            id: sea_orm::ActiveValue::Set(self.id),
            ..Default::default()
        };

        if let Some(code) = self.code {
            active_model.code = sea_orm::ActiveValue::Set(code);
        }

        if let Some(name) = self.name {
            active_model.name = sea_orm::ActiveValue::Set(name);
        }

        if let Some(description) = self.description {
            active_model.description = sea_orm::ActiveValue::Set(description);
        }

        active_model.updated_at = sea_orm::ActiveValue::Set(chrono::Utc::now().into());
        active_model
    }
}
