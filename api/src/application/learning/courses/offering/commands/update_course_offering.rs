use uuid::Uuid;

pub struct UpdateCourseOffering {
    pub id: Uuid,
    pub course_id: Option<Uuid>,
    pub academic_term_id: Option<Uuid>,
    pub lecturer_id: Option<Option<Uuid>>,
    pub section: Option<String>,
    pub capacity: Option<i32>,
    pub starts_at: Option<Option<chrono::DateTime<chrono::FixedOffset>>>,
    pub ends_at: Option<Option<chrono::DateTime<chrono::FixedOffset>>>,
    pub is_active: Option<bool>,
}
