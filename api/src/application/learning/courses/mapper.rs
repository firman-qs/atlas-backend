use crate::application::learning::courses::commands::create_course::CreateCourse;
use crate::application::learning::courses::commands::update_course::UpdateCourse;
use crate::domain::entities::course::CourseNew;
use crate::domain::entities::course::CourseUpdate;

impl From<CreateCourse> for CourseNew {
    fn from(command: CreateCourse) -> Self {
        Self {
            code: command.code,
            title: command.title,
            description: command.description,
        }
    }
}

impl From<UpdateCourse> for CourseUpdate {
    fn from(command: UpdateCourse) -> Self {
        Self {
            id: command.id,
            code: command.code,
            title: command.title,
            description: command.description,
            is_active: command.is_active,
        }
    }
}
