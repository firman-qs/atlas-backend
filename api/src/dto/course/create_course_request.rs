use garde::Validate;
use serde::Deserialize;

use crate::validation::course_validation::validate_course_description;

#[derive(Debug, Deserialize, Validate)]
pub struct CreateCourseRequest {
    #[garde(length(min = 3, max = 20))]
    pub code: String,

    #[garde(length(min = 1, max = 255))]
    pub title: String,

    #[garde(custom(validate_course_description))]
    pub description: Option<String>,
}
