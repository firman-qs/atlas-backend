use entity::concepts;
use sea_orm::IntoActiveModel;

use crate::dto::concept::create_concept_request::CreateConceptRequest;

pub struct CreateConcept {
    pub code: String,
    pub name: String,
    pub description: Option<String>,
}

impl From<CreateConceptRequest> for CreateConcept {
    fn from(request: CreateConceptRequest) -> Self {
        Self {
            code: request.code,
            name: request.name,
            description: request.description,
        }
    }
}

impl IntoActiveModel<concepts::ActiveModel> for CreateConcept {
    fn into_active_model(self) -> concepts::ActiveModel {
        concepts::ActiveModel {
            code: sea_orm::ActiveValue::Set(self.code),
            name: sea_orm::ActiveValue::Set(self.name),
            description: sea_orm::ActiveValue::Set(self.description),
            ..Default::default()
        }
    }
}
