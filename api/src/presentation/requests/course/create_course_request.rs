use garde::Validate;
use serde::Deserialize;

#[derive(Debug, Deserialize, Validate)]
pub struct CreateCourseRequest {
    #[garde(length(min = 3, max = 20))]
    pub code: String,

    #[garde(length(min = 1, max = 255))]
    pub title: String,

    #[garde(skip)]
    pub description: Option<String>,
}
