use sea_orm_migration::prelude::*;

#[derive(DeriveIden)]
pub enum Questions
{
    Table,

    /// UUID
    Id,

    /// Unique question code, e.g. "Q-001"
    Code,

    /// FK -> Courses.Id
    CourseId,

    /// FK -> QuestionTypes.Id
    QuestionTypeId,

    /// FK -> Users.Id
    CreatedBy,

    /// Example:
    /// Ownership and Borrowing
    Title,

    /// Markdown / HTML / Plain text
    QuestionText,

    /// Expected solving time (minutes)
    ///
    /// Examples:
    /// 60
    /// 90
    /// 300
    EstimatedMinutes,

    /// Reference for correct answ
    ReferenceExplanation,

    /// Feedback
    Feedback,

    /// Soft delete / archive
    IsActive,

    CreatedAt,

    UpdatedAt,
}
