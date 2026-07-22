use crate::application::learning::courses::offering::commands::create_course_offering::CreateCourseOffering;
use crate::application::learning::courses::offering::commands::update_course_offering::UpdateCourseOffering;
use crate::domain::entities::course_offering::CourseOfferingNew;
use crate::domain::entities::course_offering::CourseOfferingUpdate;

impl From<CreateCourseOffering> for CourseOfferingNew {
    fn from(command: CreateCourseOffering) -> Self {
        Self {
            course_id: command.course_id,
            academic_term_id: command.academic_term_id,
            lecturer_id: command.lecturer_id,
            section: command.section,
            capacity: command.capacity,
            starts_at: command.starts_at,
            ends_at: command.ends_at,
        }
    }
}

impl From<UpdateCourseOffering> for CourseOfferingUpdate {
    fn from(command: UpdateCourseOffering) -> Self {
        Self {
            id: command.id,
            course_id: command.course_id,
            academic_term_id: command.academic_term_id,
            lecturer_id: command.lecturer_id,
            section: command.section,
            capacity: command.capacity,
            starts_at: command.starts_at,
            ends_at: command.ends_at,
            is_active: command.is_active,
        }
    }
}
