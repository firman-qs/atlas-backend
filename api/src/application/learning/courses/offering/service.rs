use std::sync::Arc;

use uuid::Uuid;

use crate::application::app_error::AppError;
use crate::application::learning::courses::offering::commands::create_course_offering::CreateCourseOffering;
use crate::application::learning::courses::offering::results::course_offering_result::CourseOfferingListResult;
use crate::application::learning::courses::offering::results::course_offering_result::CourseOfferingResult;
use crate::infrastructure::database::repositories::pg_course_offering_repository::PgCourseOfferingRepository;

pub struct CourseOfferingService {
    repository: Arc<PgCourseOfferingRepository>,
}

impl CourseOfferingService {
    pub fn new(repository: Arc<PgCourseOfferingRepository>) -> Self {
        Self { repository }
    }

    pub async fn create(
        &self,
        cmd: CreateCourseOffering,
    ) -> Result<CourseOfferingResult, AppError> {
        let response = self.repository.create(cmd.into()).await?;

        Ok(response.into())
    }

    pub async fn get_by_id(&self, id: Uuid) -> Result<CourseOfferingResult, AppError> {
        let response = self.repository.find_by_id(id).await?;
        let response = response.ok_or_else(|| {
            AppError::NotFound(format!("Course offering with id {} not found", id))
        })?;

        Ok(response.into())
    }

    pub async fn get_all(&self) -> Result<CourseOfferingListResult, AppError> {
        let response = self.repository.find_all().await?;
        Ok(CourseOfferingListResult {
            responses: response.into_iter().map(|o| o.into()).collect(),
        })
    }

    pub async fn get_by_course_id(
        &self,
        course_id: Uuid,
    ) -> Result<CourseOfferingListResult, AppError> {
        let response = self.repository.find_by_course_id(course_id).await?;
        Ok(CourseOfferingListResult {
            responses: response.into_iter().map(|o| o.into()).collect(),
        })
    }

    pub async fn get_by_academic_term_id(
        &self,
        academic_term_id: Uuid,
    ) -> Result<CourseOfferingListResult, AppError> {
        let response = self
            .repository
            .find_by_academic_term_id(academic_term_id)
            .await?;
        Ok(CourseOfferingListResult {
            responses: response.into_iter().map(|o| o.into()).collect(),
        })
    }

    pub async fn get_by_lecturer_id(
        &self,
        lecturer_id: Uuid,
    ) -> Result<CourseOfferingListResult, AppError> {
        let response = self.repository.find_by_lecturer_id(lecturer_id).await?;
        Ok(CourseOfferingListResult {
            responses: response.into_iter().map(|o| o.into()).collect(),
        })
    }

    pub async fn delete(&self, id: Uuid) -> Result<(), AppError> {
        self.repository.delete(id).await?;
        Ok(())
    }

    pub async fn deactivate(&self, id: Uuid) -> Result<CourseOfferingResult, AppError> {
        let response = self.repository.deactivate(id).await?;
        Ok(response.into())
    }

    pub async fn activate(&self, id: Uuid) -> Result<CourseOfferingResult, AppError> {
        let response = self.repository.activate(id).await?;
        Ok(response.into())
    }
}
