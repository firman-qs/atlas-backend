use uuid::Uuid;

pub struct CreateLearningObjective {
    pub course_id: Uuid,
    pub code: String,
    pub title: String,
    pub description: Option<String>,
    pub display_order: i32,
}
