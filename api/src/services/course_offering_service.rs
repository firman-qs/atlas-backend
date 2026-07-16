use std::sync::Arc;

use garde::Validate;
use uuid::Uuid;

use crate::{
    dto::course::{
        course_offering_response::{CourseOfferingListResponse, CourseOfferingResponse},
        create_course_offering_request::CreateCourseOfferingRequest,
    },
    errors::app_error::AppError,
    models::course::create_course_offering::CreateCourseOffering,
    repositories::course_offering_repository::CourseOfferingRepository,
};

pub struct CourseOfferingService {
    repository: Arc<CourseOfferingRepository>,
}

impl CourseOfferingService {
    pub fn new(repository: Arc<CourseOfferingRepository>) -> Self {
        Self { repository }
    }

    pub async fn create(
        &self,
        offering: CreateCourseOfferingRequest,
    ) -> Result<CourseOfferingResponse, AppError> {
        offering.validate()?;
        let offering: CreateCourseOffering = offering.into();
        let response = self.repository.create(offering).await?;
        Ok(response.into())
    }

    pub async fn get_by_id(&self, id: Uuid) -> Result<CourseOfferingResponse, AppError> {
        let response = self.repository.find_by_id(id).await?;
        let response = response.ok_or_else(|| {
            AppError::NotFound(format!("Course offering with id {} not found", id))
        })?;
        Ok(response.into())
    }

    pub async fn get_all(&self) -> Result<CourseOfferingListResponse, AppError> {
        let response = self.repository.find_all().await?;
        Ok(CourseOfferingListResponse {
            responses: response.into_iter().map(|o| o.into()).collect(),
        })
    }

    pub async fn get_by_course_id(
        &self,
        course_id: Uuid,
    ) -> Result<CourseOfferingListResponse, AppError> {
        let response = self.repository.find_by_course_id(course_id).await?;
        Ok(CourseOfferingListResponse {
            responses: response.into_iter().map(|o| o.into()).collect(),
        })
    }

    pub async fn get_by_academic_term_id(
        &self,
        academic_term_id: Uuid,
    ) -> Result<CourseOfferingListResponse, AppError> {
        let response = self
            .repository
            .find_by_academic_term_id(academic_term_id)
            .await?;
        Ok(CourseOfferingListResponse {
            responses: response.into_iter().map(|o| o.into()).collect(),
        })
    }

    pub async fn get_by_lecturer_id(
        &self,
        lecturer_id: Uuid,
    ) -> Result<CourseOfferingListResponse, AppError> {
        let response = self.repository.find_by_lecturer_id(lecturer_id).await?;
        Ok(CourseOfferingListResponse {
            responses: response.into_iter().map(|o| o.into()).collect(),
        })
    }

    pub async fn delete(&self, id: Uuid) -> Result<(), AppError> {
        self.repository.delete(id).await?;
        Ok(())
    }

    pub async fn deactivate(&self, id: Uuid) -> Result<CourseOfferingResponse, AppError> {
        let response = self.repository.deactivate(id).await?;
        Ok(response.into())
    }

    pub async fn activate(&self, id: Uuid) -> Result<CourseOfferingResponse, AppError> {
        let response = self.repository.activate(id).await?;
        Ok(response.into())
    }
}
