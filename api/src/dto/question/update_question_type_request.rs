use entity::question_types;
use garde::Validate;
use sea_orm::{ActiveValue::Set, IntoActiveModel};
use serde::Deserialize;
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug, Deserialize, ToSchema, Validate)]
pub struct UpdateQuestionTypeRequest {
    #[garde(skip)]
    pub id: Uuid,
    #[garde(length(min = 1, max = 30))]
    pub code: Option<String>,
    #[garde(length(min = 1, max = 100))]
    pub name: Option<String>,
    #[garde(skip)]
    pub description: Option<Option<String>>,
    #[garde(skip)]
    pub supports_options: Option<bool>,
    #[garde(skip)]
    pub supports_autograde: Option<bool>,
}

impl IntoActiveModel<question_types::ActiveModel> for UpdateQuestionTypeRequest {
    fn into_active_model(self) -> question_types::ActiveModel {
        let mut active_model = question_types::ActiveModel {
            id: sea_orm::ActiveValue::Set(self.id),
            ..Default::default()
        };

        if let Some(code) = self.code {
            active_model.code = Set(code);
        }

        if let Some(name) = self.name {
            active_model.name = Set(name);
        }

        if let Some(description) = self.description {
            active_model.description = Set(description);
        }

        if let Some(supports_options) = self.supports_options {
            active_model.supports_options = Set(supports_options);
        }

        if let Some(supports_autograde) = self.supports_autograde {
            active_model.supports_autograde = Set(supports_autograde);
        }

        active_model.updated_at = Set(chrono::Utc::now().into());
        active_model
    }
}
