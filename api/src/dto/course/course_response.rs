use entity::courses;
use sea_orm::prelude::DateTimeWithTimeZone;
use serde::Serialize;
use uuid::Uuid;

#[derive(Debug, Serialize)]
pub struct CourseResponse {
    pub id: Uuid,
    pub code: String,
    pub title: String,
    pub description: Option<String>,
    pub created_at: DateTimeWithTimeZone,
    pub updated_at: DateTimeWithTimeZone,
}

impl From<courses::Model> for CourseResponse {
    fn from(course: courses::Model) -> Self {
        Self {
            id: course.id,
            code: course.code,
            title: course.title,
            description: course.description,
            created_at: course.created_at,
            updated_at: course.updated_at,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct CourseListResponse {
    pub responses: Vec<CourseResponse>,
}
