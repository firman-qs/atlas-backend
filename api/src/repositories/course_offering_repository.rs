use entity::course_offerings;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, IntoActiveModel, QueryFilter,
};
use uuid::Uuid;

use crate::models::course::{
    create_course_offering::CreateCourseOffering, update_course_offering::UpdateCourseOffering,
};

pub struct CourseOfferingRepository {
    db: DatabaseConnection,
}

impl CourseOfferingRepository {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    pub async fn create(
        &self,
        offering: CreateCourseOffering,
    ) -> Result<course_offerings::Model, sea_orm::DbErr> {
        let offering: course_offerings::ActiveModel = offering.into_active_model();
        let response = offering.insert(&self.db).await?;
        Ok(response)
    }

    pub async fn find_by_id(
        &self,
        id: uuid::Uuid,
    ) -> Result<Option<course_offerings::Model>, sea_orm::DbErr> {
        course_offerings::Entity::find_by_id(id).one(&self.db).await
    }

    pub async fn find_all(&self) -> Result<Vec<course_offerings::Model>, sea_orm::DbErr> {
        course_offerings::Entity::find().all(&self.db).await
    }

    pub async fn find_by_course_id(
        &self,
        course_id: uuid::Uuid,
    ) -> Result<Vec<course_offerings::Model>, sea_orm::DbErr> {
        course_offerings::Entity::find()
            .filter(course_offerings::Column::CourseId.eq(course_id))
            .all(&self.db)
            .await
    }

    pub async fn find_by_academic_term_id(
        &self,
        academic_term_id: uuid::Uuid,
    ) -> Result<Vec<course_offerings::Model>, sea_orm::DbErr> {
        course_offerings::Entity::find()
            .filter(course_offerings::Column::AcademicTermId.eq(academic_term_id))
            .all(&self.db)
            .await
    }

    pub async fn find_by_lecturer_id(
        &self,
        lecturer_id: uuid::Uuid,
    ) -> Result<Vec<course_offerings::Model>, sea_orm::DbErr> {
        course_offerings::Entity::find()
            .filter(course_offerings::Column::LecturerId.eq(lecturer_id))
            .all(&self.db)
            .await
    }

    pub async fn find_archived_all(&self) -> Result<Vec<course_offerings::Model>, sea_orm::DbErr> {
        course_offerings::Entity::find()
            .filter(course_offerings::Column::IsActive.eq(false))
            .all(&self.db)
            .await
    }

    pub async fn update(
        &self,
        offering: UpdateCourseOffering,
    ) -> Result<course_offerings::Model, sea_orm::DbErr> {
        let offering_active = offering.into_active_model();
        let response = offering_active.update(&self.db).await?;
        Ok(response)
    }

    pub async fn archive(&self, id: uuid::Uuid) -> Result<course_offerings::Model, sea_orm::DbErr> {
        self.archive(id, true).await
    }

    pub async fn unarchive(
        &self,
        id: uuid::Uuid,
    ) -> Result<course_offerings::Model, sea_orm::DbErr> {
        self.archive(id, false).await
    }

    pub async fn delete(&self, id: uuid::Uuid) -> Result<(), sea_orm::DbErr> {
        let offering = course_offerings::Entity::find_by_id(id)
            .one(&self.db)
            .await?
            .ok_or_else(|| {
                sea_orm::DbErr::RecordNotFound("Course offering not found".to_string())
            })?;

        let offering: course_offerings::ActiveModel = offering.into();
        offering.delete(&self.db).await?;
        Ok(())
    }

    async fn archive(
        &self,
        id: Uuid,
        archive: bool,
    ) -> Result<course_offerings::Model, sea_orm::DbErr> {
        let offering = course_offerings::Entity::find_by_id(id)
            .one(&self.db)
            .await?
            .ok_or_else(|| {
                sea_orm::DbErr::RecordNotFound("Course offering not found".to_string())
            })?;

        let mut offering: course_offerings::ActiveModel = offering.into();
        offering.is_active = sea_orm::ActiveValue::Set(!archive);
        let response = offering.update(&self.db).await?;
        Ok(response)
    }
}
