use entity::solo_levels;
use serde::Serialize;
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug, Serialize, ToSchema)]
pub struct SoloLevelResponse {
    pub id: Uuid,
    pub code: String,
    pub name: String,
    pub order_index: i16,
    pub description: Option<String>,
    pub is_active: bool,
    pub created_at: chrono::DateTime<chrono::FixedOffset>,
    pub updated_at: chrono::DateTime<chrono::FixedOffset>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct SoloLevelListResponse {
    pub responses: Vec<SoloLevelResponse>,
}

impl From<solo_levels::Model> for SoloLevelResponse {
    fn from(model: solo_levels::Model) -> Self {
        SoloLevelResponse {
            id: model.id,
            code: model.code,
            name: model.name,
            order_index: model.order_index,
            description: model.description,
            is_active: model.is_active,
            created_at: model.created_at,
            updated_at: model.updated_at,
        }
    }
}
