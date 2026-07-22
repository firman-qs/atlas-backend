use entity::courses;
use migration::Expr;
use sea_orm::ActiveModelTrait;
use sea_orm::ActiveValue::Set;
use sea_orm::ColumnTrait;
use sea_orm::ConnectionTrait;
use sea_orm::DatabaseConnection;
use sea_orm::EntityTrait;
use sea_orm::IntoActiveModel;
use sea_orm::QueryFilter;
use sea_orm::QuerySelect;
use sea_orm::sea_query::extension::postgres::PgExpr;
use uuid::Uuid;

use crate::domain::entities::course::Course;
use crate::domain::entities::course::CourseNew;
use crate::domain::entities::course::CourseUpdate;
use crate::domain::errors::repository_error::RepositoryError;

#[derive(Debug)]
pub struct PgCourseRepository<C = DatabaseConnection>
where
    C: ConnectionTrait,
{
    db: C,
}

impl<C: ConnectionTrait> PgCourseRepository<C> {
    pub fn new(db: C) -> Self {
        Self { db }
    }

    pub async fn set_active(&self, id: Uuid, active: bool) -> Result<Course, RepositoryError> {
        let model = courses::ActiveModel {
            id: Set(id),
            is_active: Set(active),
            updated_at: Set(chrono::Utc::now().into()),
            ..Default::default()
        }
        .update(&self.db)
        .await?;

        Ok(Course::from(model))
    }

    pub async fn create(&self, course: CourseNew) -> Result<Course, RepositoryError> {
        let model = course.into_active_model().insert(&self.db).await?;

        Ok(Course::from(model))
    }

    pub async fn find_by_id(&self, id: Uuid) -> Result<Option<Course>, RepositoryError> {
        let model = courses::Entity::find_by_id(id).one(&self.db).await?;

        Ok(model.map(Course::from))
    }

    pub async fn find_all(&self) -> Result<Vec<Course>, RepositoryError> {
        let model = courses::Entity::find().all(&self.db).await?;

        Ok(model.into_iter().map(Course::from).collect())
    }

    pub async fn find_by_code(&self, code: &str) -> Result<Option<Course>, RepositoryError> {
        let model = courses::Entity::find()
            .filter(courses::Column::Code.eq(code))
            .one(&self.db)
            .await?;

        Ok(model.map(Course::from))
    }

    pub async fn find_archived_all(&self) -> Result<Vec<Course>, RepositoryError> {
        let model = courses::Entity::find()
            .filter(courses::Column::IsActive.eq(false))
            .all(&self.db)
            .await?;

        Ok(model.into_iter().map(Course::from).collect())
    }

    pub async fn search_by_code(
        &self,
        query: &str,
        limit: u64,
    ) -> Result<Vec<Course>, RepositoryError> {
        let model = courses::Entity::find()
            .filter(Expr::col(courses::Column::Code).ilike(query))
            .limit(limit)
            .all(&self.db)
            .await?;

        Ok(model.into_iter().map(Course::from).collect())
    }

    pub async fn search_by_title(
        &self,
        query: &str,
        limit: u64,
    ) -> Result<Vec<Course>, RepositoryError> {
        let model = courses::Entity::find()
            .filter(Expr::col(courses::Column::Title).ilike(query))
            .limit(limit)
            .all(&self.db)
            .await?;

        Ok(model.into_iter().map(Course::from).collect())
    }

    pub async fn update(&self, update: CourseUpdate) -> Result<Course, RepositoryError> {
        let model = update.into_active_model().update(&self.db).await?;

        Ok(Course::from(model))
    }

    pub async fn deactivate(&self, id: Uuid) -> Result<Course, RepositoryError> {
        self.set_active(id, false).await
    }

    pub async fn activate(&self, id: Uuid) -> Result<Course, RepositoryError> {
        self.set_active(id, true).await
    }

    pub async fn delete(&self, id: Uuid) -> Result<(), RepositoryError> {
        let result = courses::Entity::delete_by_id(id).exec(&self.db).await?;

        if result.rows_affected == 0 {
            return Err(RepositoryError::NotFound);
        }

        Ok(())
    }
}
