use serde::Serialize;
use utoipa::ToSchema;
use uuid::Uuid;

use crate::domain::entities::course::Course;

#[derive(Debug, Serialize, ToSchema)]
pub struct CourseResult {
    pub id: Uuid,
    pub code: String,
    pub title: String,
    pub description: Option<String>,
    pub created_at: chrono::DateTime<chrono::FixedOffset>,
    pub updated_at: chrono::DateTime<chrono::FixedOffset>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct CourseListResult {
    pub results: Vec<CourseResult>,
}

impl From<Course> for CourseResult {
    fn from(course: Course) -> Self {
        Self {
            id: course.id,
            code: course.code,
            title: course.title,
            description: course.description,
            created_at: course.created_at.into(),
            updated_at: course.updated_at.into(),
        }
    }
}
