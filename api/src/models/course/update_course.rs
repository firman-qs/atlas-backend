use uuid::Uuid;

pub struct UpdateCourse {
    pub id: Uuid,
    pub code: Option<String>,
    pub title: Option<String>,
    pub description: Option<String>,
}
