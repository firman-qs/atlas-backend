use sea_orm::prelude::DateTimeWithTimeZone;
use uuid::Uuid;

pub struct QuestionType {
    pub id: Uuid,
    pub code: String,
    pub name: String,
    pub description: Option<String>,
    pub supports_options: bool,
    pub supports_autograde: bool,
    pub is_active: bool,
    pub created_at: DateTimeWithTimeZone,
    pub updated_at: DateTimeWithTimeZone,
}

pub struct QuestionTypeNew {
    pub code: String,
    pub name: String,
    pub description: Option<String>,
    pub supports_options: bool,
    pub supports_autograde: bool,
}

pub struct QuestionTypeUpdate {
    pub id: Uuid,
    pub code: Option<String>,
    pub name: Option<String>,
    pub description: Option<Option<String>>,
    pub supports_options: Option<bool>,
    pub supports_autograde: Option<bool>,
    pub is_active: Option<bool>,
}
