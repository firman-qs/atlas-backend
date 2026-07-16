use entity::solo_levels;
use garde::Validate;
use sea_orm::IntoActiveModel;
use serde::Deserialize;
use utoipa::ToSchema;

#[derive(Debug, Deserialize, ToSchema, Validate)]
pub struct CreateSoloLevelRequest {
    #[garde(length(min = 1, max = 50))]
    pub code: String,
    #[garde(length(min = 1, max = 100))]
    pub name: String,
    #[garde(range(min = 1, max = 32767))]
    pub order_index: i16,
    #[garde(skip)]
    pub description: Option<String>,
}

impl IntoActiveModel<solo_levels::ActiveModel> for CreateSoloLevelRequest {
    fn into_active_model(self) -> solo_levels::ActiveModel {
        solo_levels::ActiveModel {
            code: sea_orm::ActiveValue::Set(self.code),
            name: sea_orm::ActiveValue::Set(self.name),
            order_index: sea_orm::ActiveValue::Set(self.order_index),
            description: sea_orm::ActiveValue::Set(self.description),
            ..Default::default()
        }
    }
}
