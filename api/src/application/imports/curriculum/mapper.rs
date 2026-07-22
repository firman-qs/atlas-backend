use crate::application::imports::curriculum::models::course::ImportCourse;
use crate::domain::entities::course::CourseNew;

impl From<ImportCourse> for CourseNew {
    fn from(import_course: ImportCourse) -> Self {
        CourseNew {
            code: import_course.code,
            title: import_course.title,
            description: import_course.description,
        }
    }
}
