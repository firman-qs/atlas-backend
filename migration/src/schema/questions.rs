use sea_orm_migration::prelude::*;

#[derive(DeriveIden)]
pub enum Questions {
    Table,

    /// UUID
    Id,

    /// FK -> Concepts.Id
    ConceptId,

    /// FK -> QuestionTypes.Id
    QuestionTypeId,

    /// FK -> Users.Id
    CreatedBy,

    /// Example:
    /// Ownership and Borrowing
    Title,

    /// Markdown / HTML / Plain text
    QuestionText,

    /// SOLO Taxonomy level
    SoloLevel,

    /// Item difficulty
    ///
    /// Examples:
    /// 0.00
    /// 1.25
    /// 2.50
    Difficulty,

    /// Expected solving time (seconds)
    ///
    /// Examples:
    /// 60
    /// 90
    /// 300
    EstimatedTime,

    /// Reference for correct answ
    ReferenceExplanation,

    /// Feedback
    Feedback,

    /// Soft delete / archive
    IsActive,

    CreatedAt,

    UpdatedAt,
}
