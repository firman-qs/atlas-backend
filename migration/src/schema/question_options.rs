use sea_orm_migration::prelude::*;

#[derive(DeriveIden)]
pub enum QuestionOptions
{
    Table,
    Id,

    /// FK -> Questions.Id
    QuestionId,

    /// Displayed answer text.
    ///
    /// Examples:
    /// "Stack"
    /// "Heap"
    /// "Borrow Checker"
    /// "All of the above"
    OptionText,

    /// Whether this option is a correct answer.
    ///
    /// Examples:
    /// true
    /// false
    IsCorrect,

    /// Display order shown to students.
    ///
    /// Examples:
    /// 1
    /// 2
    /// 3
    /// 4
    DisplayOrder,
}
