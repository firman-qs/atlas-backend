use entity::courses;
use migration::Expr;
use sea_orm::{
    ActiveModelTrait,
    ActiveValue::{self, Set},
    ColumnTrait, DatabaseConnection, EntityTrait, IntoActiveModel, QueryFilter, QuerySelect,
    sea_query::extension::postgres::PgExpr,
};
use uuid::Uuid;

use crate::models::course::{create_course::CreateCourse, update_course::UpdateCourse};

#[derive(Debug)]
pub struct CourseRepository {
    db: DatabaseConnection,
}

impl CourseRepository {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    pub async fn create(&self, course: CreateCourse) -> Result<courses::Model, sea_orm::DbErr> {
        let course = courses::ActiveModel {
            code: ActiveValue::Set(course.code),
            title: ActiveValue::Set(course.title),
            description: ActiveValue::Set(course.description),
            ..Default::default()
        };

        course.insert(&self.db).await
    }

    pub async fn find_by_id(
        &self,
        id: uuid::Uuid,
    ) -> Result<Option<courses::Model>, sea_orm::DbErr> {
        courses::Entity::find_by_id(id).one(&self.db).await
    }

    pub async fn find_all(&self) -> Result<Vec<courses::Model>, sea_orm::DbErr> {
        courses::Entity::find().all(&self.db).await
    }

    pub async fn find_by_code(&self, code: &str) -> Result<Option<courses::Model>, sea_orm::DbErr> {
        courses::Entity::find()
            .filter(courses::Column::Code.eq(code))
            .one(&self.db)
            .await
    }

    pub async fn find_archived_all(&self) -> Result<Vec<courses::Model>, sea_orm::DbErr> {
        courses::Entity::find()
            .filter(courses::Column::IsActive.eq(false))
            .all(&self.db)
            .await
    }

    pub async fn search_by_code(
        &self,
        query: &str,
        limit: u64,
    ) -> Result<Vec<courses::Model>, sea_orm::DbErr> {
        courses::Entity::find()
            .filter(Expr::col(courses::Column::Code).ilike(query))
            .limit(limit)
            .all(&self.db)
            .await
    }

    pub async fn search_by_title(
        &self,
        query: &str,
        limit: u64,
    ) -> Result<Vec<courses::Model>, sea_orm::DbErr> {
        courses::Entity::find()
            .filter(Expr::col(courses::Column::Title).ilike(query))
            .limit(limit)
            .all(&self.db)
            .await
    }

    pub async fn update(&self, update: UpdateCourse) -> Result<courses::Model, sea_orm::DbErr> {
        update.into_active_model().update(&self.db).await
    }

    pub async fn deactivate(&self, id: Uuid) -> Result<courses::Model, sea_orm::DbErr> {
        self.set_active(id, false).await
    }

    pub async fn activate(&self, id: Uuid) -> Result<courses::Model, sea_orm::DbErr> {
        self.set_active(id, true).await
    }

    pub async fn delete(&self, id: Uuid) -> Result<(), sea_orm::DbErr> {
        let result = courses::Entity::delete_by_id(id).exec(&self.db).await?;
        if result.rows_affected == 0 {
            return Err(sea_orm::DbErr::RecordNotFound(format!(
                "Course with id {} not found",
                id
            )));
        }
        Ok(())
    }

    pub async fn set_active(
        &self,
        id: Uuid,
        active: bool,
    ) -> Result<courses::Model, sea_orm::DbErr> {
        courses::ActiveModel {
            id: Set(id),
            is_active: Set(active),
            updated_at: Set(chrono::Utc::now().into()),
            ..Default::default()
        }
        .update(&self.db)
        .await
    }
}
