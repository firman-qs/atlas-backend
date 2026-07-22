use sea_orm::prelude::DateTimeWithTimeZone;
use uuid::Uuid;

pub struct Concept {
    pub id: Uuid,
    pub target_solo_level_id: Uuid,
    pub code: String,
    pub name: String,
    pub is_active: bool,
    pub description: Option<String>,
    pub display_order: i32,
    pub created_at: DateTimeWithTimeZone,
    pub updated_at: DateTimeWithTimeZone,
}

pub struct ConceptNew {
    pub target_solo_level_id: Uuid,
    pub code: String,
    pub name: String,
    pub description: Option<String>,
    pub display_order: i32,
}

pub struct ConceptUpdate {
    pub id: Uuid,
    pub target_solo_level_id: Option<Uuid>,
    pub code: Option<String>,
    pub name: Option<String>,
    pub is_active: Option<bool>,
    pub description: Option<Option<String>>,
    pub display_order: Option<i32>,
}
