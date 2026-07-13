use sea_orm::IntoActiveModel;

pub struct CreateCourse {
    pub code: String,
    pub title: String,
    pub description: Option<String>,
}

impl IntoActiveModel<entity::courses::ActiveModel> for CreateCourse {
    fn into_active_model(self) -> entity::courses::ActiveModel {
        entity::courses::ActiveModel {
            code: sea_orm::ActiveValue::Set(self.code),
            title: sea_orm::ActiveValue::Set(self.title),
            description: sea_orm::ActiveValue::Set(self.description),
            ..Default::default()
        }
    }
}
