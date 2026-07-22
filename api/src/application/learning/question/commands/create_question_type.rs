pub struct CreateQuestionType {
    pub code: String,
    pub name: String,
    pub description: Option<String>,
    pub supports_options: bool,
    pub supports_autograde: bool,
}
