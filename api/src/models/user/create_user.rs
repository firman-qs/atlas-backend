use sea_orm::IntoActiveModel;

pub struct CreateUser {
    pub email: String,
    pub username: String,
    pub full_name: String,
    pub password_hash: String,
}

impl IntoActiveModel<entity::users::ActiveModel> for CreateUser {
    fn into_active_model(self) -> entity::users::ActiveModel {
        entity::users::ActiveModel {
            email: sea_orm::ActiveValue::Set(self.email),
            username: sea_orm::ActiveValue::Set(self.username),
            full_name: sea_orm::ActiveValue::Set(self.full_name),
            password_hash: sea_orm::ActiveValue::Set(self.password_hash),
            ..Default::default()
        }
    }
}
