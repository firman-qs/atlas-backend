use crate::application::learning::courses::commands::create_course::CreateCourse;
use crate::application::learning::courses::commands::update_course::UpdateCourse;
use crate::application::learning::courses::offering::commands::create_course_offering::CreateCourseOffering;
use crate::domain::entities::course_offering::CourseOfferingUpdate;
use crate::presentation::requests::course::create_course_request::CreateCourseRequest;
use crate::presentation::requests::course::update_course_request::UpdateCourseRequest;
use crate::presentation::requests::course_offering::create_course_offering_request::CreateCourseOfferingRequest;
use crate::presentation::requests::course_offering::update_course_offering_request::UpdateCourseOfferingRequest;

impl From<UpdateCourseRequest> for UpdateCourse {
    fn from(request: UpdateCourseRequest) -> Self {
        Self {
            id: request.id,
            code: request.code,
            title: request.title,
            description: request.description,
            is_active: request.is_active,
        }
    }
}

impl From<CreateCourseRequest> for CreateCourse {
    fn from(request: CreateCourseRequest) -> Self {
        Self {
            code: request.code,
            title: request.title,
            description: request.description,
        }
    }
}

impl From<CreateCourseOfferingRequest> for CreateCourseOffering {
    fn from(request: CreateCourseOfferingRequest) -> Self {
        Self {
            course_id: request.course_id,
            academic_term_id: request.academic_term_id,
            lecturer_id: request.lecturer_id,
            section: request.section,
            capacity: request.capacity,
            starts_at: request.starts_at,
            ends_at: request.ends_at,
        }
    }
}

impl From<UpdateCourseOfferingRequest> for CourseOfferingUpdate {
    fn from(request: UpdateCourseOfferingRequest) -> Self {
        Self {
            id: request.id,
            course_id: request.course_id,
            academic_term_id: request.academic_term_id,
            lecturer_id: request.lecturer_id,
            section: request.section,
            capacity: request.capacity,
            starts_at: request.starts_at,
            ends_at: request.ends_at,
            is_active: request.is_active,
        }
    }
}
