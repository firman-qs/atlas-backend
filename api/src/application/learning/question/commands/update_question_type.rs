use uuid::Uuid;

pub struct UpdateQuestionType {
    pub id: Uuid,
    pub code: Option<String>,
    pub name: Option<String>,
    pub description: Option<Option<String>>,
    pub supports_options: Option<bool>,
    pub supports_autograde: Option<bool>,
    pub is_active: Option<bool>,
}
