use uuid::Uuid;

pub struct CreateConcept {
    pub code: String,
    pub name: String,
    pub description: Option<String>,
    pub target_solo_level_id: Uuid,
    pub display_order: i32,
}
