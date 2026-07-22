use entity::course_offerings;
use sea_orm::ActiveModelTrait;
use sea_orm::ColumnTrait;
use sea_orm::DatabaseConnection;
use sea_orm::EntityTrait;
use sea_orm::IntoActiveModel;
use sea_orm::QueryFilter;
use uuid::Uuid;

use crate::domain::entities::course_offering::CourseOffering;
use crate::domain::entities::course_offering::CourseOfferingNew;
use crate::domain::entities::course_offering::CourseOfferingUpdate;
use crate::domain::errors::repository_error::RepositoryError;

pub struct PgCourseOfferingRepository {
    db: DatabaseConnection,
}

impl PgCourseOfferingRepository {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    pub async fn set_active(
        &self,
        id: Uuid,
        active: bool,
    ) -> Result<CourseOffering, RepositoryError> {
        let model = course_offerings::ActiveModel {
            id: sea_orm::ActiveValue::Set(id),
            is_active: sea_orm::ActiveValue::Set(active),
            updated_at: sea_orm::ActiveValue::Set(chrono::Utc::now().into()),
            ..Default::default()
        }
        .update(&self.db)
        .await?;

        Ok(CourseOffering::from(model))
    }

    pub async fn create(
        &self,
        offering: CourseOfferingNew,
    ) -> Result<CourseOffering, RepositoryError> {
        let model = offering.into_active_model().insert(&self.db).await?;

        Ok(CourseOffering::from(model))
    }

    pub async fn find_by_id(&self, id: Uuid) -> Result<Option<CourseOffering>, RepositoryError> {
        let model = course_offerings::Entity::find_by_id(id)
            .one(&self.db)
            .await?;

        Ok(model.map(CourseOffering::from))
    }

    pub async fn find_all(&self) -> Result<Vec<CourseOffering>, RepositoryError> {
        let models = course_offerings::Entity::find().all(&self.db).await?;

        Ok(models.into_iter().map(CourseOffering::from).collect())
    }

    pub async fn find_by_course_id(
        &self,
        course_id: Uuid,
    ) -> Result<Vec<CourseOffering>, RepositoryError> {
        let models = course_offerings::Entity::find()
            .filter(course_offerings::Column::CourseId.eq(course_id))
            .all(&self.db)
            .await?;

        Ok(models.into_iter().map(CourseOffering::from).collect())
    }

    pub async fn find_by_academic_term_id(
        &self,
        academic_term_id: Uuid,
    ) -> Result<Vec<CourseOffering>, RepositoryError> {
        let models = course_offerings::Entity::find()
            .filter(course_offerings::Column::AcademicTermId.eq(academic_term_id))
            .all(&self.db)
            .await?;

        Ok(models.into_iter().map(CourseOffering::from).collect())
    }

    pub async fn find_by_lecturer_id(
        &self,
        lecturer_id: Uuid,
    ) -> Result<Vec<CourseOffering>, RepositoryError> {
        let models = course_offerings::Entity::find()
            .filter(course_offerings::Column::LecturerId.eq(lecturer_id))
            .all(&self.db)
            .await?;

        Ok(models.into_iter().map(CourseOffering::from).collect())
    }

    pub async fn find_archived_all(&self) -> Result<Vec<CourseOffering>, RepositoryError> {
        let models = course_offerings::Entity::find()
            .filter(course_offerings::Column::IsActive.eq(false))
            .all(&self.db)
            .await?;

        Ok(models.into_iter().map(CourseOffering::from).collect())
    }

    pub async fn update(
        &self,
        offering: CourseOfferingUpdate,
    ) -> Result<CourseOffering, RepositoryError> {
        let model = offering.into_active_model().update(&self.db).await?;

        Ok(CourseOffering::from(model))
    }

    pub async fn deactivate(&self, id: Uuid) -> Result<CourseOffering, RepositoryError> {
        self.set_active(id, false).await
    }

    pub async fn activate(&self, id: Uuid) -> Result<CourseOffering, RepositoryError> {
        self.set_active(id, true).await
    }

    pub async fn delete(&self, id: Uuid) -> Result<(), RepositoryError> {
        let result = course_offerings::Entity::delete_by_id(id)
            .exec(&self.db)
            .await?;

        if result.rows_affected == 0 {
            return Err(RepositoryError::NotFound);
        }

        Ok(())
    }
}
