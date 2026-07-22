use sea_orm::prelude::DateTimeWithTimeZone;
use uuid::Uuid;

pub struct LearningObjective {
    pub id: Uuid,
    pub course_id: Uuid,
    pub code: String,
    pub title: String,
    pub description: Option<String>,
    pub display_order: i32,
    pub is_active: bool,
    pub created_at: DateTimeWithTimeZone,
    pub updated_at: DateTimeWithTimeZone,
}

pub struct LearningObjectiveNew {
    pub course_id: Uuid,
    pub code: String,
    pub title: String,
    pub description: Option<String>,
    pub display_order: i32,
}

pub struct LearningObjectiveUpdate {
    pub id: Uuid,
    pub course_id: Option<Uuid>,
    pub code: Option<String>,
    pub title: Option<String>,
    pub description: Option<Option<String>>,
    pub display_order: Option<i32>,
    pub is_active: Option<bool>,
}
