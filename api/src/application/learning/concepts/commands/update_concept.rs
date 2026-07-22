use uuid::Uuid;

pub struct UpdateConcept {
    pub id: Uuid,
    pub code: Option<String>,
    pub name: Option<String>,
    pub description: Option<Option<String>>,
    pub is_active: Option<bool>,
    pub target_solo_level_id: Option<Uuid>,
    pub display_order: Option<i32>,
}
