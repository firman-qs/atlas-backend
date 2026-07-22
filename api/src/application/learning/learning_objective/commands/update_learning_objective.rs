use uuid::Uuid;

pub struct UpdateLearningObjective {
    pub id: Uuid,
    pub course_id: Option<Uuid>,
    pub code: Option<String>,
    pub title: Option<String>,
    pub description: Option<Option<String>>,
    pub display_order: Option<i32>,
    pub is_active: Option<bool>,
}
