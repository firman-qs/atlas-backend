use sea_orm_migration::prelude::*;

#[derive(DeriveIden)]
pub enum LearningObjectives
{
    Table,

    /// UUID
    Id,

    /// FK -> Courses.Id
    CourseId,

    /// Examples:
    /// LO-1
    /// CPMK-1
    Code,

    /// Example:
    /// Understand ownership and borrowing
    Title,

    Description,

    /// Display order in syllabus
    ///
    /// 0
    /// 1
    /// 2
    DisplayOrder,

    IsActive,

    CreatedAt,

    UpdatedAt,
}
