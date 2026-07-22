/// Repository responsible for accessing user data stored in the database.
///
/// This layer should only contain database queries. Business logic belongs in
/// the Service layer.
use entity::users::{self};
use migration::Expr;
use sea_orm::ActiveModelTrait;
use sea_orm::ActiveValue::Set;
use sea_orm::ColumnTrait;
use sea_orm::DatabaseTransaction;
use sea_orm::DbErr;
use sea_orm::EntityTrait;
use sea_orm::IntoActiveModel;
use sea_orm::QueryFilter;
use uuid::Uuid;

use crate::domain::entities::user::User;
use crate::domain::entities::user::UserNew;
use crate::domain::entities::user::UserUpdate;
use crate::domain::errors::repository_error::RepositoryError;

#[derive(Debug)]
pub struct TxUserRepository<'a>
{
    db: &'a DatabaseTransaction,
}

impl<'a> TxUserRepository<'a>
{
    pub fn new(db: &'a DatabaseTransaction) -> Self
    {
        Self { db }
    }

    pub async fn find_by_id(&self, id: Uuid) -> Result<Option<User>, RepositoryError>
    {
        let model = users::Entity::find_by_id(id).one(self.db).await?;

        Ok(model.map(User::from))
    }

    pub async fn find_all(&self) -> Result<Vec<User>, RepositoryError>
    {
        let models = users::Entity::find().all(self.db).await?;

        Ok(models.into_iter().map(User::from).collect())
    }

    pub async fn find_by_email(&self, email: &str) -> Result<Option<User>, RepositoryError>
    {
        let model = users::Entity::find()
            .filter(users::Column::Email.eq(email))
            .one(self.db)
            .await?;

        Ok(model.map(User::from))
    }

    pub async fn find_by_username(&self, username: &str) -> Result<Option<User>, RepositoryError>
    {
        let model = users::Entity::find()
            .filter(users::Column::Username.eq(username))
            .one(self.db)
            .await?;

        Ok(model.map(User::from))
    }

    pub async fn search_by_full_name(&self, full_name: &str) -> Result<Vec<User>, RepositoryError>
    {
        let model = users::Entity::find()
            .filter(Expr::col(users::Column::FullName).like(format!("%{}%", full_name)))
            .all(self.db)
            .await?;

        Ok(model.into_iter().map(User::from).collect())
    }

    pub async fn create(&self, user: UserNew) -> Result<User, RepositoryError>
    {
        let model = user.into_active_model().insert(self.db).await?;
        Ok(User::from(model))
    }

    pub async fn delete(&self, id: Uuid) -> Result<(), RepositoryError>
    {
        let result = users::Entity::delete_by_id(id).exec(self.db).await?;

        if result.rows_affected == 0
        {
            return Err(RepositoryError::NotFound);
        }

        Ok(())
    }

    pub async fn update(&self, user: UserUpdate) -> Result<User, RepositoryError>
    {
        let model = user.into_active_model().update(self.db).await?;
        Ok(User::from(model))
    }

    pub async fn activate(&self, id: Uuid) -> Result<User, RepositoryError>
    {
        let model = self.set_active(id, true).await?;
        Ok(User::from(model))
    }

    pub async fn deactivate(&self, id: Uuid) -> Result<User, RepositoryError>
    {
        let model = self.set_active(id, false).await?;
        Ok(User::from(model))
    }

    pub async fn set_active(&self, id: Uuid, active: bool) -> Result<User, DbErr>
    {
        let model = users::ActiveModel {
            id: Set(id),
            is_active: Set(active),
            updated_at: Set(chrono::Utc::now().into()),
            ..Default::default()
        }
        .update(self.db)
        .await?;

        Ok(User::from(model))
    }
}
