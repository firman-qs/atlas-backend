use uuid::Uuid;

pub struct UpdateSoloLevel {
    pub id: Uuid,
    pub code: Option<String>,
    pub name: Option<String>,
    pub is_active: Option<bool>,
    pub order_index: Option<i16>,
    pub description: Option<Option<String>>,
}
