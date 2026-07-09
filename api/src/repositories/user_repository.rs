/// Repository responsible for accessing user data stored in the database.
///
/// This layer should only contain database queries. Business logic belongs in
/// the Service layer.
use entity::users::{self, ActiveModel};
use sea_orm::{
    ActiveModelTrait, ActiveValue::Set, ColumnTrait, DatabaseConnection, DbErr, EntityTrait,
    QueryFilter,
};
use uuid::Uuid;

use crate::models::user::create_user::CreateUser;

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
        let user = ActiveModel {
            email: Set(user.email),
            username: Set(user.username),
            password_hash: Set(user.password_hash),
            full_name: Set(user.full_name),
            ..Default::default()
        };

        user.insert(&self.db).await
    }
}
