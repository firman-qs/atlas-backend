use garde::Validate;
use serde::Deserialize;
use uuid::Uuid;

use crate::validation::course_validation::validate_course_description;

#[derive(Debug, Deserialize, Validate)]
pub struct UpdateCourseRequest {
    #[garde(skip)]
    pub id: Uuid,

    #[garde(length(min = 3, max = 20))]
    pub code: Option<String>,

    #[garde(length(min = 1, max = 255))]
    pub title: Option<String>,

    #[garde(custom(validate_course_description))]
    pub description: Option<String>,
}
