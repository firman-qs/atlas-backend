use crate::application::users::commands::update_user::UpdateUser;
use crate::domain::entities::user::UserUpdate;

impl From<UpdateUser> for UserUpdate {
    fn from(command: UpdateUser) -> Self {
        Self {
            id: command.id,
            username: command.username,
            password_hash: command.password_hash,
            full_name: command.full_name,
            avatar_url: command.avatar_url,
            is_active: command.is_active,
            must_change_password: command.must_change_password,
        }
    }
}
