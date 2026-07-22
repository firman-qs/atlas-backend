use sea_orm::prelude::DateTimeWithTimeZone;
use uuid::Uuid;

pub struct Course {
    pub id: Uuid,
    pub code: String,
    pub title: String,
    pub description: Option<String>,
    pub is_active: bool,
    pub created_at: DateTimeWithTimeZone,
    pub updated_at: DateTimeWithTimeZone,
}

pub struct CourseNew {
    pub code: String,
    pub title: String,
    pub description: Option<String>,
}

pub struct CourseUpdate {
    pub id: Uuid,
    pub code: Option<String>,
    pub title: Option<String>,
    pub description: Option<Option<String>>,
    pub is_active: Option<bool>,
}
