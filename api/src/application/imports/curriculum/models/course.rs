use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct ImportCourse {
    pub code: String,
    pub title: String,
    pub description: Option<String>,
}
