use std::sync::Arc;

use uuid::Uuid;

use crate::application::app_error::AppError;
use crate::application::learning::courses::commands::create_course::CreateCourse;
use crate::application::learning::courses::commands::update_course::UpdateCourse;
use crate::application::learning::courses::results::course_result::CourseListResult;
use crate::application::learning::courses::results::course_result::CourseResult;
use crate::infrastructure::database::repositories::pg_course_repository::PgCourseRepository;
use crate::shared::constants::MSG_COURSE_NOT_FOUND;

pub struct CourseService {
    course_repository: Arc<PgCourseRepository>,
}

impl CourseService {
    pub fn new(course_repository: Arc<PgCourseRepository>) -> Self {
        Self { course_repository }
    }

    pub async fn create(&self, course: CreateCourse) -> Result<CourseResult, AppError> {
        let new_course = CreateCourse {
            code: course.code.trim().to_owned(),
            title: course.title.trim().to_owned(),
            description: course
                .description
                .map(|d| d.trim().to_owned())
                .filter(|d| !d.is_empty()),
        };

        let course = self.course_repository.create(new_course.into()).await?;

        Ok(course.into())
    }

    pub async fn get_by_code(&self, code: &str) -> Result<CourseResult, AppError> {
        let course = self.course_repository.find_by_code(code).await?;
        let course = course.ok_or_else(|| AppError::NotFound(MSG_COURSE_NOT_FOUND.into()))?;
        Ok(course.into())
    }

    pub async fn get_by_id(&self, id: Uuid) -> Result<CourseResult, AppError> {
        let course = self.course_repository.find_by_id(id).await?;
        let course = course.ok_or_else(|| AppError::NotFound(MSG_COURSE_NOT_FOUND.into()))?;
        Ok(course.into())
    }

    pub async fn get_all(&self) -> Result<CourseListResult, AppError> {
        let courses = self.course_repository.find_all().await?;
        Ok(CourseListResult {
            results: courses.into_iter().map(|c| c.into()).collect(),
        })
    }

    pub async fn search_by_code(
        &self,
        query: &str,
        limit: u64,
    ) -> Result<CourseListResult, AppError> {
        let courses = self.course_repository.search_by_code(query, limit).await?;
        Ok(CourseListResult {
            results: courses.into_iter().map(|c| c.into()).collect(),
        })
    }

    pub async fn search_by_title(
        &self,
        query: &str,
        limit: u64,
    ) -> Result<CourseListResult, AppError> {
        let courses = self.course_repository.search_by_title(query, limit).await?;
        Ok(CourseListResult {
            results: courses.into_iter().map(|c| c.into()).collect(),
        })
    }

    pub async fn update(&self, course: UpdateCourse) -> Result<CourseResult, AppError> {
        Ok(self.course_repository.update(course.into()).await?.into())
    }

    pub async fn get_archived_all(&self) -> Result<CourseListResult, AppError> {
        let courses = self.course_repository.find_archived_all().await?;
        Ok(CourseListResult {
            results: courses.into_iter().map(|c| c.into()).collect(),
        })
    }

    pub async fn deactivate(&self, id: Uuid) -> Result<CourseResult, AppError> {
        let updated_course = self.course_repository.deactivate(id).await?;
        Ok(updated_course.into())
    }

    pub async fn activate(&self, id: Uuid) -> Result<CourseResult, AppError> {
        let updated_course = self.course_repository.activate(id).await?;
        Ok(updated_course.into())
    }

    pub async fn delete(&self, id: Uuid) -> Result<(), AppError> {
        self.course_repository.delete(id).await?;
        Ok(())
    }
}

// #[cfg(test)]
// mod tests
// {
//     use std::sync::Arc;
//
//     use crate::config::database::connect;
//     use crate::config::settings::Settings;
//     use crate::dto::course::create_course_request::CreateCourseRequest;
//     use crate::dto::course::update_course_request::UpdateCourseRequest;
//     use crate::repositories::course_repository::CourseRepository;
//     use crate::services::course_service::CourseService;
//
//     async fn create_course_service() -> CourseService
//     {
//         let settings = Settings::new();
//         let db: sea_orm::DatabaseConnection = connect(&settings.database_url)
//             .await
//             .expect("Cannot connect database");
//         let course_repository = Arc::new(CourseRepository::new(db));
//         CourseService::new(course_repository)
//     }
//
//     #[tokio::test]
//     async fn test_create_course_success()
//     {
//         let course_service = create_course_service().await;
//         let course_request = CreateCourseRequest {
//             code: "CS101".to_string(),
//             title: "Introduction to Computer Science".to_string(),
//             description: Some("An introductory course to computer
// science.".to_string()),         };
//
//         let result = course_service.create(course_request).await;
//         assert!(result.is_ok());
//
//         // cleanup: delete the created course
//         let created_course = result.unwrap();
//         let _ = course_service
//             .course_repository
//             .delete(created_course.id)
//             .await;
//     }
//
//     #[tokio::test]
//     async fn test_create_course_duplicate_code()
//     {
//         let course_service = create_course_service().await;
//
//         // first case from create.
//         let create_request_1 = CreateCourseRequest {
//             code: "CS102".to_string(),
//             title: "Introduction to Computer Science".to_string(),
//             description: Some("An introductory course to computer
// science.".to_string()),         };
//         let result_create_request_1 =
// course_service.create(create_request_1).await;         let create_request_2 =
// CreateCourseRequest {             code: "CS102".to_string(),
//             title: "Another Course".to_string(),
//             description: Some("This course has a duplicate
// code.".to_string()),         };
//         let result_create_request_2 =
// course_service.create(create_request_2).await;         assert!
// (result_create_request_2.is_err());
//
//         // second case from update.
//         let create_request_3 = CreateCourseRequest {
//             code: "CS103".to_string(),
//             title: "Introduction to Computer Science".to_string(),
//             description: Some("An introductory course to computer
// science.".to_string()),         };
//         let result_create_request_3 =
// course_service.create(create_request_3).await;         let uuid_3 =
// result_create_request_3.as_ref().unwrap().id;         let update_request =
// UpdateCourseRequest {             id: uuid_3,
//             code: Some("CS102".to_string()),
//             title: Some("Updated Course Title".to_string()),
//             description: Some(Some("Updated description.".to_string())),
//         };
//         let result_update_request =
// course_service.update(update_request).await;         assert!
// (result_update_request.is_err());
//
//         // cleanup
//         let _ = course_service
//             .course_repository
//             .delete(result_create_request_1.unwrap().id)
//             .await;
//         let _ = course_service.course_repository.delete(uuid_3).await;
//     }
//
//     #[tokio::test]
//     async fn test_create_course_empty_title()
//     {
//         let course_service = create_course_service().await;
//         let course_request = CreateCourseRequest {
//             code: "CS104".to_string(),
//             title: "".to_string(),
//             description: Some("An introductory course to computer
// science.".to_string()),         };
//
//         let result = course_service.create(course_request).await;
//         assert!(result.is_err());
//     }
//
//     #[tokio::test]
//     async fn test_get_course_by_id()
//     {
//         let course_service = create_course_service().await;
//         let course_request = CreateCourseRequest {
//             code: "CS105".to_string(),
//             title: "Introduction to Computer Science".to_string(),
//             description: Some("An introductory course to computer
// science.".to_string()),         };
//
//         let created_course =
// course_service.create(course_request).await.unwrap();         let result =
// course_service.get_by_id(created_course.id).await;         assert!(result.
// is_ok());         assert_eq!(result.unwrap().id, created_course.id);
//
//         // cleanup
//         let _ = course_service
//             .course_repository
//             .delete(created_course.id)
//             .await;
//     }
//
//     #[tokio::test]
//     async fn test_get_all_courses()
//     {
//         let course_service = create_course_service().await;
//         let course_request_1 = CreateCourseRequest {
//             code: "CS106".to_string(),
//             title: "Introduction to Computer Science".to_string(),
//             description: Some("An introductory course to computer
// science.".to_string()),         };
//         let course_request_2 = CreateCourseRequest {
//             code: "CS107".to_string(),
//             title: "Data Structures".to_string(),
//             description: Some("A course on data structures.".to_string()),
//         };
//
//         let created_course_1 =
// course_service.create(course_request_1).await.unwrap();         let
// created_course_2 = course_service.create(course_request_2).await.unwrap();
//
//         let result = course_service.get_all().await;
//         assert!(result.is_ok());
//
//         let courses = result.unwrap().responses;
//         assert!(courses.iter().any(|c| c.id == created_course_1.id));
//         assert!(courses.iter().any(|c| c.id == created_course_2.id));
//
//         // cleanup
//         let _ = course_service
//             .course_repository
//             .delete(created_course_1.id)
//             .await;
//         let _ = course_service
//             .course_repository
//             .delete(created_course_2.id)
//             .await;
//     }
//
//     #[tokio::test]
//     async fn test_update_course()
//     {
//         let course_service = create_course_service().await;
//         let course_request = CreateCourseRequest {
//             code: "CS108".to_string(),
//             title: "Introduction to Computer Science".to_string(),
//             description: Some("An introductory course to computer
// science.".to_string()),         };
//
//         let created_course =
// course_service.create(course_request).await.unwrap();
//
//         let update_request = UpdateCourseRequest {
//             id: created_course.id,
//             code: Some("CS108Updated".to_string()),
//             title: None,
//             description: Some(Some("Updated description.".to_string())),
//         };
//
//         let result = course_service.update(update_request).await;
//         assert!(result.is_ok());
//
//         let updated_course = result.unwrap();
//         assert_eq!(updated_course.code, "CS108Updated");
//         assert_eq!(updated_course.title, "Introduction to Computer Science");
//         assert_eq!(
//             updated_course.description.as_deref(),
//             Some("Updated description.")
//         );
//
//         // cleanup
//         let _ = course_service
//             .course_repository
//             .delete(created_course.id)
//             .await;
//     }
// }
