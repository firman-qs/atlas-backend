use sea_orm::prelude::DateTimeWithTimeZone;
use uuid::Uuid;

pub struct CourseOffering {
    pub id: Uuid,
    pub course_id: Uuid,
    pub academic_term_id: Uuid,
    pub section: String,
    pub lecturer_id: Option<Uuid>,
    pub capacity: i32,
    pub starts_at: Option<DateTimeWithTimeZone>,
    pub ends_at: Option<DateTimeWithTimeZone>,
    pub is_active: bool,
    pub created_at: DateTimeWithTimeZone,
    pub updated_at: DateTimeWithTimeZone,
}

pub struct CourseOfferingNew {
    pub course_id: Uuid,
    pub academic_term_id: Uuid,
    pub section: String,
    pub lecturer_id: Option<Uuid>,
    pub capacity: i32,
    pub starts_at: Option<DateTimeWithTimeZone>,
    pub ends_at: Option<DateTimeWithTimeZone>,
}

pub struct CourseOfferingUpdate {
    pub id: Uuid,
    pub course_id: Option<Uuid>,
    pub academic_term_id: Option<Uuid>,
    pub section: Option<String>,
    pub lecturer_id: Option<Option<Uuid>>,
    pub capacity: Option<i32>,
    pub starts_at: Option<Option<DateTimeWithTimeZone>>,
    pub ends_at: Option<Option<DateTimeWithTimeZone>>,
    pub is_active: Option<bool>,
}
