use uuid::Uuid;

pub struct CreateCourseOffering {
    pub course_id: Uuid,
    pub academic_term_id: Uuid,
    pub lecturer_id: Option<Uuid>,
    pub section: String,
    pub capacity: i32,
    pub starts_at: Option<chrono::DateTime<chrono::FixedOffset>>,
    pub ends_at: Option<chrono::DateTime<chrono::FixedOffset>>,
}
