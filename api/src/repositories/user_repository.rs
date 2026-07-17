/// Repository responsible for accessing user data stored in the database.
///
/// This layer should only contain database queries. Business logic belongs in
/// the Service layer.
use entity::users::{self};
use sea_orm::{
    ActiveModelTrait, ActiveValue::Set, ColumnTrait, DatabaseConnection, DbErr, EntityTrait,
    IntoActiveModel, QueryFilter,
};
use uuid::Uuid;

use crate::models::user::{create_user::CreateUser, update_user::UpdateUser};

#[derive(Debug)]
pub struct UserRepository {
    db: DatabaseConnection,
}

impl UserRepository {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    pub async fn find_by_id(&self, id: Uuid) -> Result<Option<users::Model>, DbErr> {
        users::Entity::find_by_id(id).one(&self.db).await
    }

    pub async fn find_all(&self) -> Result<Vec<users::Model>, DbErr> {
        users::Entity::find().all(&self.db).await
    }

    pub async fn find_by_email(&self, email: &str) -> Result<Option<users::Model>, DbErr> {
        users::Entity::find()
            .filter(users::Column::Email.eq(email))
            .one(&self.db)
            .await
    }

    pub async fn find_by_username(&self, username: &str) -> Result<Option<users::Model>, DbErr> {
        users::Entity::find()
            .filter(users::Column::Username.eq(username))
            .one(&self.db)
            .await
    }

    pub async fn create(&self, user: CreateUser) -> Result<users::Model, DbErr> {
        user.into_active_model().insert(&self.db).await
    }

    pub async fn delete(&self, id: Uuid) -> Result<(), DbErr> {
        let result = users::Entity::delete_by_id(id).exec(&self.db).await?;
        if result.rows_affected == 0 {
            return Err(DbErr::RecordNotFound(format!(
                "User with id {} not found",
                id
            )));
        }

        Ok(())
    }

    pub async fn update(&self, user: UpdateUser) -> Result<users::Model, DbErr> {
        user.into_active_model().update(&self.db).await
    }

    pub async fn activate(&self, id: Uuid) -> Result<users::Model, DbErr> {
        self.set_active(id, true).await
    }

    pub async fn deactivate(&self, id: Uuid) -> Result<users::Model, DbErr> {
        self.set_active(id, false).await
    }

    async fn set_active(&self, id: Uuid, active: bool) -> Result<users::Model, DbErr> {
        users::ActiveModel {
            id: Set(id),
            is_active: Set(active),
            updated_at: Set(chrono::Utc::now().into()),
            ..Default::default()
        }
        .update(&self.db)
        .await
    }
}
