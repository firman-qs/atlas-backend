use entity::solo_levels;
use garde::Validate;
use sea_orm::{ActiveValue::Set, IntoActiveModel};
use serde::Deserialize;
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug, Deserialize, ToSchema, Validate)]
pub struct UpdateSoloLevelRequest {
    #[garde(skip)]
    pub id: Uuid,
    #[garde(length(min = 1, max = 50))]
    pub code: String,
    #[garde(length(min = 1, max = 100))]
    pub name: String,
    #[garde(range(min = 1, max = 32767))]
    pub order_index: i16,
    #[garde(skip)]
    pub description: Option<Option<String>>,
}

impl IntoActiveModel<solo_levels::ActiveModel> for UpdateSoloLevelRequest {
    fn into_active_model(self) -> solo_levels::ActiveModel {
        let mut active_model = solo_levels::ActiveModel {
            id: Set(self.id),
            ..Default::default()
        };

        if let Some(code) = Some(self.code) {
            active_model.code = Set(code);
        }

        if let Some(name) = Some(self.name) {
            active_model.name = Set(name);
        }

        if let Some(order_index) = Some(self.order_index) {
            active_model.order_index = Set(order_index);
        }

        if let Some(description) = self.description {
            active_model.description = Set(description);
        }

        active_model.updated_at = Set(chrono::Utc::now().into());
        active_model
    }
}
