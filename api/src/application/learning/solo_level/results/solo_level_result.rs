use serde::Serialize;
use utoipa::ToSchema;
use uuid::Uuid;

use crate::domain::entities::solo_level::SoloLevel;

#[derive(Debug, Serialize, ToSchema)]
pub struct SoloLevelResult {
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
pub struct SoloLevelListResult {
    pub responses: Vec<SoloLevelResult>,
}

impl From<SoloLevel> for SoloLevelResult {
    fn from(solo_level: SoloLevel) -> Self {
        Self {
            id: solo_level.id,
            code: solo_level.code,
            name: solo_level.name,
            order_index: solo_level.order_index,
            description: solo_level.description,
            is_active: solo_level.is_active,
            created_at: solo_level.created_at,
            updated_at: solo_level.updated_at,
        }
    }
}
