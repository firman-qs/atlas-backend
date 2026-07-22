use entity::users;
use sea_orm::ActiveValue::Set;
use sea_orm::IntoActiveModel;

use crate::domain::entities::user::User;
use crate::domain::entities::user::UserNew;
use crate::domain::entities::user::UserUpdate;

impl From<users::Model> for User {
    fn from(model: users::Model) -> Self {
        Self {
            id: model.id,
            email: model.email,
            username: model.username,
            password_hash: model.password_hash,
            full_name: model.full_name,
            avatar_url: model.avatar_url,
            is_active: model.is_active,
            must_change_password: model.must_change_password,
            role: model.role,
            created_at: model.created_at,
            updated_at: model.updated_at,
        }
    }
}

impl IntoActiveModel<users::ActiveModel> for UserNew {
    fn into_active_model(self) -> users::ActiveModel {
        entity::users::ActiveModel {
            email: Set(self.email),
            username: Set(self.username),
            full_name: Set(self.full_name),
            password_hash: Set(self.password_hash),
            ..Default::default()
        }
    }
}

impl IntoActiveModel<users::ActiveModel> for UserUpdate {
    fn into_active_model(self) -> users::ActiveModel {
        let mut active_model = users::ActiveModel {
            id: Set(self.id),
            ..Default::default()
        };

        if let Some(username) = self.username {
            active_model.username = Set(username);
        }

        if let Some(password_hash) = self.password_hash {
            active_model.password_hash = Set(password_hash);
        }

        if let Some(full_name) = self.full_name {
            active_model.full_name = Set(full_name);
        }

        if let Some(avatar_url) = self.avatar_url {
            active_model.avatar_url = Set(avatar_url);
        }

        if let Some(is_active) = self.is_active {
            active_model.is_active = Set(is_active);
        }

        if let Some(must_change_password) = self.must_change_password {
            active_model.must_change_password = Set(must_change_password);
        }

        active_model.updated_at = Set(chrono::Utc::now().into());

        active_model
    }
}
