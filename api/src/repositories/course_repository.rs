use entity::courses;
use sea_orm::{
    ActiveModelTrait,
    ActiveValue::{self, Set},
    ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter,
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

    pub async fn update(&self, update: UpdateCourse) -> Result<courses::Model, sea_orm::DbErr> {
        let course = courses::Entity::find_by_id(update.id)
            .one(&self.db)
            .await?
            .ok_or(sea_orm::DbErr::RecordNotFound(
                "Course not found".to_string(),
            ))?;

        let mut active_model: courses::ActiveModel = course.into();

        if let Some(code) = update.code {
            active_model.code = Set(code);
        }

        if let Some(title) = update.title {
            active_model.title = Set(title);
        }

        if let Some(description) = update.description {
            active_model.description = Set(Some(description));
        }

        active_model.updated_at = Set(chrono::Utc::now().into());
        active_model.update(&self.db).await
    }

    pub async fn archive(&self, id: Uuid) -> Result<courses::Model, sea_orm::DbErr> {
        courses::ActiveModel {
            id: Set(id),
            is_active: Set(false),
            updated_at: Set(chrono::Utc::now().into()),
            ..Default::default()
        }
        .update(&self.db)
        .await
    }

    pub async fn unarchive(&self, id: Uuid) -> Result<courses::Model, sea_orm::DbErr> {
        courses::ActiveModel {
            id: Set(id),
            is_active: Set(true),
            updated_at: Set(chrono::Utc::now().into()),
            ..Default::default()
        }
        .update(&self.db)
        .await
    }

    pub async fn delete(&self, id: Uuid) -> Result<(), sea_orm::DbErr> {
        let course = courses::Entity::find_by_id(id).one(&self.db).await?.ok_or(
            sea_orm::DbErr::RecordNotFound("Course not found".to_string()),
        )?;

        let active_model: courses::ActiveModel = course.into();
        active_model.delete(&self.db).await.map(|_| ())
    }
}
